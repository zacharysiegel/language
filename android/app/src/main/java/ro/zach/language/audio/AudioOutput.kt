package ro.zach.language.audio

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.media.AudioAttributes
import android.media.AudioFocusRequest
import android.media.AudioManager
import android.net.Uri
import android.os.Handler
import android.os.HandlerThread
import android.speech.tts.TextToSpeech
import android.speech.tts.UtteranceProgressListener
import android.util.Log
import androidx.core.content.ContextCompat
import androidx.media3.common.C
import androidx.media3.common.MediaItem
import androidx.media3.common.PlaybackException
import androidx.media3.common.PlaybackParameters
import androidx.media3.common.Player
import androidx.media3.exoplayer.ExoPlayer
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.CompletableDeferred
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.Deferred
import kotlinx.coroutines.android.asCoroutineDispatcher
import kotlinx.coroutines.ensureActive
import kotlinx.coroutines.withContext
import ro.zach.language.diagnostics.LogTag
import java.io.File
import java.util.Locale
import java.util.concurrent.ConcurrentHashMap

private const val AUDIO_THREAD_NAME: String = "language-audio"
/* Long enough that the clips of a sequence keep the focus between them. */
private const val FOCUS_RELEASE_DELAY_MILLISECONDS: Long = 1_000
private const val UTTERANCE_ID_PREFIX: String = "utterance-"

/** How a recording or utterance ended. */
enum class PlaybackOutcome {
    Finished,
    Failed,
    Stopped,
    Interrupted,
}

/** Whether the output has a sound it started and has not yet ended. */
enum class AudioOutputActivityKind {
    Idle,
    Playing,
}

/** The app's audio focus as the output last requested or observed it. */
private enum class AudioFocusActivationKind {
    Inactive,
    Active,
    Interrupted,
}

/** Whether the noisy-output receiver is registered. */
private enum class AudioOutputPreparationKind {
    Unprepared,
    Prepared,
}

/** The sound the output started last and has not yet ended; `completion` resolves to its first reported outcome. */
private sealed interface CurrentPlayback {
    data object Nothing : CurrentPlayback

    class Recording(val completion: CompletableDeferred<PlaybackOutcome>, val onSoundStarted: () -> Unit) : CurrentPlayback

    class Speech(val utteranceId: String, val completion: CompletableDeferred<PlaybackOutcome>, val onSoundStarted: () -> Unit) : CurrentPlayback
}

/**
 * Owns the audio focus, the recording player and the system voice. Every call runs on one audio thread, which is the
 * player's application looper, so none of their work reaches the main thread.
 */
class AudioOutput(context: Context) {
    companion object {
        val systemVoiceLocale: Locale = Locale.forLanguageTag("pt-BR")
    }

    private val applicationContext: Context = context.applicationContext
    private val audioThread: HandlerThread = HandlerThread(AUDIO_THREAD_NAME)
    private val audioHandler: Handler
    private val audioDispatcher: CoroutineDispatcher
    private val audioManager: AudioManager = applicationContext.getSystemService(AudioManager::class.java)
    private val platformAudioAttributes: AudioAttributes = AudioAttributes.Builder()
        .setUsage(AudioAttributes.USAGE_MEDIA)
        .setContentType(AudioAttributes.CONTENT_TYPE_SPEECH)
        .build()
    private val focusRequest: AudioFocusRequest
    private val releaseFocus: Runnable = Runnable { releaseFocusWhenIdle() }
    private val noisyOutputReceiver: BroadcastReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context, intent: Intent) {
            stopCurrentPlayback(PlaybackOutcome.Interrupted)
        }
    }
    /* Read by the system voice's callbacks, which arrive on a binder thread. */
    private val speechUtterances: ConcurrentHashMap<String, CurrentPlayback.Speech> = ConcurrentHashMap()

    /* Touched only on the audio thread. */
    private var preparation: AudioOutputPreparationKind = AudioOutputPreparationKind.Unprepared
    private var focusActivation: AudioFocusActivationKind = AudioFocusActivationKind.Inactive
    private var player: ExoPlayer? = null
    private var systemVoiceCreation: Deferred<TextToSpeech?>? = null
    private var readySystemVoice: TextToSpeech? = null
    private var currentPlayback: CurrentPlayback = CurrentPlayback.Nothing
    private var utteranceCount: Long = 0

    init {
        audioThread.start()
        audioHandler = Handler(audioThread.looper)
        audioDispatcher = audioHandler.asCoroutineDispatcher()
        focusRequest = AudioFocusRequest.Builder(AudioManager.AUDIOFOCUS_GAIN_TRANSIENT)
            .setAudioAttributes(platformAudioAttributes)
            .setWillPauseWhenDucked(true)
            .setOnAudioFocusChangeListener({ focusChange: Int -> observeInterruptionOnAudioThread(AudioInterruptionEvent.fromFocusChange(focusChange)) }, audioHandler)
            .build()
    }

    /** Creates the recording player and loads the system voice, without taking the audio focus. */
    suspend fun prepareWithoutActivating() {
        withContext(audioDispatcher) {
            prepareIfUnprepared()
            playerCreatingIfAbsent()
            systemVoiceCreatingIfAbsent()
        }
    }

    /** Returns once the recording has ended, failed, been stopped or been interrupted; `onSoundStarted` runs when it starts playing. */
    suspend fun playRecording(path: String, rate: Float, onSoundStarted: () -> Unit): PlaybackOutcome {
        return withContext(audioDispatcher) {
            prepareIfUnprepared()
            stopCurrentPlayback(PlaybackOutcome.Stopped)

            val recordingPlayer: ExoPlayer = playerCreatingIfAbsent()

            ensureActive()

            val focusOutcome: AudioFocusActivationKind = requestFocusUnlessActive()

            if (focusOutcome != AudioFocusActivationKind.Active) {
                return@withContext PlaybackOutcome.Interrupted
            }

            val recording: CurrentPlayback.Recording = CurrentPlayback.Recording(completion = CompletableDeferred(), onSoundStarted = onSoundStarted)

            currentPlayback = recording
            preparePlayer(recordingPlayer, path, rate)
            startPlaying(recordingPlayer)
            awaitOutcome(recording, recording.completion)
        }
    }

    /** Returns once the utterance has ended, failed, been stopped or been interrupted; `onSoundStarted` runs when the voice starts speaking it. */
    suspend fun speak(text: String, rate: Float, onSoundStarted: () -> Unit): PlaybackOutcome {
        return withContext(audioDispatcher) {
            prepareIfUnprepared()
            stopCurrentPlayback(PlaybackOutcome.Stopped)

            val systemVoice: TextToSpeech = systemVoiceCreatingIfAbsent()
                ?: return@withContext PlaybackOutcome.Failed

            ensureActive()

            val focusOutcome: AudioFocusActivationKind = requestFocusUnlessActive()

            if (focusOutcome != AudioFocusActivationKind.Active) {
                return@withContext PlaybackOutcome.Interrupted
            }

            utteranceCount += 1

            val speech: CurrentPlayback.Speech = CurrentPlayback.Speech(
                utteranceId = "$UTTERANCE_ID_PREFIX$utteranceCount",
                completion = CompletableDeferred(),
                onSoundStarted = onSoundStarted,
            )

            speechUtterances[speech.utteranceId] = speech
            currentPlayback = speech
            startSpeaking(systemVoice, text, rate, speech)
            awaitOutcome(speech, speech.completion)
        }
    }

    suspend fun activity(): AudioOutputActivityKind {
        return withContext(audioDispatcher) {
            when (currentPlayback) {
                CurrentPlayback.Nothing -> AudioOutputActivityKind.Idle
                is CurrentPlayback.Recording, is CurrentPlayback.Speech -> AudioOutputActivityKind.Playing
            }
        }
    }

    suspend fun observeInterruption(interruption: AudioInterruptionEvent?) {
        withContext(audioDispatcher) {
            observeInterruptionOnAudioThread(interruption)
        }
    }

    /** Stops any sound and releases the player, the system voice and the audio thread; the output plays nothing after. */
    fun close() {
        audioHandler.post {
            stopCurrentPlayback(PlaybackOutcome.Stopped)
            audioHandler.removeCallbacks(releaseFocus)
            abandonFocus()
            releasePlayerAndSystemVoice()
            unregisterNoisyOutputReceiver()
            audioThread.quitSafely()
        }
    }

    private fun prepareIfUnprepared() {
        if (preparation == AudioOutputPreparationKind.Prepared) {
            return
        }

        preparation = AudioOutputPreparationKind.Prepared
        ContextCompat.registerReceiver(
            applicationContext,
            noisyOutputReceiver,
            IntentFilter(AudioManager.ACTION_AUDIO_BECOMING_NOISY),
            null,
            audioHandler,
            ContextCompat.RECEIVER_NOT_EXPORTED,
        )
    }

    private fun unregisterNoisyOutputReceiver() {
        if (preparation == AudioOutputPreparationKind.Unprepared) {
            return
        }

        preparation = AudioOutputPreparationKind.Unprepared
        applicationContext.unregisterReceiver(noisyOutputReceiver)
    }

    private fun playerCreatingIfAbsent(): ExoPlayer {
        val existingPlayer: ExoPlayer? = player

        if (existingPlayer != null) {
            return existingPlayer
        }

        val creation: PlaybackStepInterval = PlaybackTiming.begin(PlaybackStepKind.PlayerCreation)
        val playerAudioAttributes: androidx.media3.common.AudioAttributes = androidx.media3.common.AudioAttributes.Builder()
            .setUsage(C.USAGE_MEDIA)
            .setContentType(C.AUDIO_CONTENT_TYPE_SPEECH)
            .build()
        /* The output requests and abandons the focus itself, for the recordings and the system voice alike. */
        val createdPlayer: ExoPlayer = ExoPlayer.Builder(applicationContext)
            .setLooper(audioThread.looper)
            .setAudioAttributes(playerAudioAttributes, false)
            .build()

        createdPlayer.addListener(RecordingPlayerListener())
        player = createdPlayer
        PlaybackTiming.end(creation)

        return createdPlayer
    }

    /* Concurrent callers share one creation; one that failed is retried by the next caller. */
    private suspend fun systemVoiceCreatingIfAbsent(): TextToSpeech? {
        val creation: Deferred<TextToSpeech?> = systemVoiceCreation ?: startSystemVoiceCreation()
        val systemVoice: TextToSpeech? = creation.await()

        if (systemVoice == null && systemVoiceCreation === creation) {
            systemVoiceCreation = null
        }

        return systemVoice
    }

    private fun startSystemVoiceCreation(): Deferred<TextToSpeech?> {
        val creation: CompletableDeferred<TextToSpeech?> = CompletableDeferred()
        val lookup: PlaybackStepInterval = PlaybackTiming.begin(PlaybackStepKind.SystemVoiceLookup)
        lateinit var systemVoice: TextToSpeech

        systemVoiceCreation = creation
        /* Posted, since the engine can report its start from inside the constructor, before `systemVoice` is assigned. */
        systemVoice = TextToSpeech(applicationContext) { status: Int ->
            val posted: Boolean = audioHandler.post { finishSystemVoiceCreation(systemVoice, status, creation, lookup) }

            if (!posted) {
                systemVoice.shutdown()
            }
        }

        return creation
    }

    private fun finishSystemVoiceCreation(systemVoice: TextToSpeech, status: Int, creation: CompletableDeferred<TextToSpeech?>, lookup: PlaybackStepInterval) {
        PlaybackTiming.end(lookup)

        if (status != TextToSpeech.SUCCESS) {
            Log.e(LogTag.AUDIO, "the system voice could not start; [status=$status]")
            systemVoice.shutdown()
            creation.complete(null)
            return
        }

        /* The output was closed while the voice started. */
        if (systemVoiceCreation !== creation) {
            systemVoice.shutdown()
            creation.complete(null)
            return
        }

        val languageAvailability: Int = systemVoice.setLanguage(systemVoiceLocale)

        if (languageAvailability < TextToSpeech.LANG_AVAILABLE) {
            Log.w(LogTag.AUDIO, "no system voice speaks the language, so the default voice speaks; [language=${systemVoiceLocale.toLanguageTag()} availability=$languageAvailability]")
        }

        systemVoice.setAudioAttributes(platformAudioAttributes)
        systemVoice.setOnUtteranceProgressListener(SpeechProgressListener())
        readySystemVoice = systemVoice
        creation.complete(systemVoice)
    }

    private fun releasePlayerAndSystemVoice() {
        player?.release()
        player = null
        readySystemVoice?.shutdown()
        readySystemVoice = null
        systemVoiceCreation = null
    }

    private fun preparePlayer(recordingPlayer: ExoPlayer, path: String, rate: Float) {
        val preparation: PlaybackStepInterval = PlaybackTiming.begin(PlaybackStepKind.PlayerPreparation)

        recordingPlayer.setMediaItem(MediaItem.fromUri(Uri.fromFile(File(path))))
        recordingPlayer.playbackParameters = PlaybackParameters(rate)
        recordingPlayer.prepare()
        PlaybackTiming.end(preparation)
    }

    private fun startPlaying(recordingPlayer: ExoPlayer) {
        val playStart: PlaybackStepInterval = PlaybackTiming.begin(PlaybackStepKind.PlayStart)

        recordingPlayer.play()
        PlaybackTiming.end(playStart)
    }

    private fun startSpeaking(systemVoice: TextToSpeech, text: String, rate: Float, speech: CurrentPlayback.Speech) {
        val speakStart: PlaybackStepInterval = PlaybackTiming.begin(PlaybackStepKind.SystemVoiceSpeakStart)

        systemVoice.setSpeechRate(rate)

        val queued: Int = systemVoice.speak(text, TextToSpeech.QUEUE_FLUSH, null, speech.utteranceId)

        PlaybackTiming.end(speakStart)

        if (queued != TextToSpeech.SUCCESS) {
            speech.completion.complete(PlaybackOutcome.Failed)
        }
    }

    /* A cancelled caller stops its sound; one already replaced by a newer play leaves the newer one alone. */
    private suspend fun awaitOutcome(playback: CurrentPlayback, completion: CompletableDeferred<PlaybackOutcome>): PlaybackOutcome {
        try {
            return completion.await()
        } catch (cancelled: CancellationException) {
            completion.complete(PlaybackOutcome.Stopped)
            throw cancelled
        } finally {
            endPlayback(playback)
        }
    }

    private fun endPlayback(playback: CurrentPlayback) {
        if (currentPlayback !== playback) {
            return
        }

        stopCurrentPlayback(PlaybackOutcome.Stopped)
    }

    /* The outcome is decided before the player or voice is stopped, so their own stop report does not decide it. */
    private fun stopCurrentPlayback(outcome: PlaybackOutcome) {
        when (val playback: CurrentPlayback = currentPlayback) {
            CurrentPlayback.Nothing -> return
            is CurrentPlayback.Recording -> {
                playback.completion.complete(outcome)
                player?.stop()
                player?.clearMediaItems()
            }
            is CurrentPlayback.Speech -> {
                playback.completion.complete(outcome)
                speechUtterances.remove(playback.utteranceId)
                readySystemVoice?.stop()
            }
        }

        currentPlayback = CurrentPlayback.Nothing
        audioHandler.removeCallbacks(releaseFocus)
        audioHandler.postDelayed(releaseFocus, FOCUS_RELEASE_DELAY_MILLISECONDS)
    }

    private fun requestFocusUnlessActive(): AudioFocusActivationKind {
        audioHandler.removeCallbacks(releaseFocus)

        if (focusActivation == AudioFocusActivationKind.Active) {
            return focusActivation
        }

        val request: PlaybackStepInterval = PlaybackTiming.begin(PlaybackStepKind.FocusRequest)
        val requestResult: Int = audioManager.requestAudioFocus(focusRequest)

        PlaybackTiming.end(request)

        if (requestResult == AudioManager.AUDIOFOCUS_REQUEST_GRANTED) {
            focusActivation = AudioFocusActivationKind.Active
        } else {
            Log.w(LogTag.AUDIO, "the audio focus was refused; [result=$requestResult]")
            focusActivation = AudioFocusActivationKind.Inactive
        }

        return focusActivation
    }

    private fun releaseFocusWhenIdle() {
        if (currentPlayback != CurrentPlayback.Nothing) {
            return
        }

        abandonFocus()
    }

    private fun abandonFocus() {
        if (focusActivation == AudioFocusActivationKind.Inactive) {
            return
        }

        audioManager.abandonAudioFocusRequest(focusRequest)
        focusActivation = AudioFocusActivationKind.Inactive
    }

    /* Android, unlike iOS, can take the focus for good without a transient loss before it. */
    private fun observeInterruptionOnAudioThread(interruption: AudioInterruptionEvent?) {
        when (interruption) {
            AudioInterruptionEvent.Began -> if (focusActivation == AudioFocusActivationKind.Active) {
                focusActivation = AudioFocusActivationKind.Interrupted
                pauseForInterruption()
            }
            AudioInterruptionEvent.EndedShouldResume -> if (focusActivation == AudioFocusActivationKind.Interrupted) {
                focusActivation = AudioFocusActivationKind.Active
                resumeAfterInterruption()
            }
            AudioInterruptionEvent.EndedShouldNotResume -> if (focusActivation != AudioFocusActivationKind.Inactive) {
                stopCurrentPlayback(PlaybackOutcome.Interrupted)
                abandonFocus()
            }
            null -> Unit
        }
    }

    /* The system voice cannot pause, so an utterance ends. */
    private fun pauseForInterruption() {
        when (currentPlayback) {
            CurrentPlayback.Nothing -> Unit
            is CurrentPlayback.Recording -> player?.pause()
            is CurrentPlayback.Speech -> stopCurrentPlayback(PlaybackOutcome.Interrupted)
        }
    }

    private fun resumeAfterInterruption() {
        when (currentPlayback) {
            CurrentPlayback.Nothing, is CurrentPlayback.Speech -> Unit
            is CurrentPlayback.Recording -> player?.play()
        }
    }

    /** Finishes the current recording when the player ends it, on the audio thread. */
    private inner class RecordingPlayerListener : Player.Listener {
        override fun onPlaybackStateChanged(playbackState: Int) {
            if (playbackState == Player.STATE_ENDED) {
                currentRecording()?.completion?.complete(PlaybackOutcome.Finished)
            }
        }

        override fun onPlayerError(error: PlaybackException) {
            Log.e(LogTag.AUDIO, "a recording could not be played; [error_code=${error.errorCodeName}]", error)
            currentRecording()?.completion?.complete(PlaybackOutcome.Failed)
        }

        override fun onIsPlayingChanged(isPlaying: Boolean) {
            if (isPlaying) {
                currentRecording()?.onSoundStarted?.invoke()
            }
        }

        private fun currentRecording(): CurrentPlayback.Recording? {
            return currentPlayback as? CurrentPlayback.Recording
        }
    }

    /** Reports each utterance's start and end, on a binder thread; an utterance already ended is no longer found. */
    private inner class SpeechProgressListener : UtteranceProgressListener() {
        override fun onStart(utteranceId: String) {
            speechUtterances[utteranceId]?.onSoundStarted?.invoke()
        }

        override fun onDone(utteranceId: String) {
            speechUtterances[utteranceId]?.completion?.complete(PlaybackOutcome.Finished)
        }

        override fun onStop(utteranceId: String, interrupted: Boolean) {
            speechUtterances[utteranceId]?.completion?.complete(PlaybackOutcome.Stopped)
        }

        @Deprecated("Replaced by onError(String, Int)")
        override fun onError(utteranceId: String) {
            speechUtterances[utteranceId]?.completion?.complete(PlaybackOutcome.Failed)
        }

        override fun onError(utteranceId: String, errorCode: Int) {
            speechUtterances[utteranceId]?.completion?.complete(PlaybackOutcome.Failed)
        }
    }
}
