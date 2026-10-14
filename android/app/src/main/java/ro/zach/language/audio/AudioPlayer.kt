package ro.zach.language.audio

import androidx.compose.runtime.ProvidableCompositionLocal
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.compose.runtime.staticCompositionLocalOf
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import ro.zach.language.core.AudioEngineRecord
import ro.zach.language.core.CourseLibrary
import ro.zach.language.core.VoiceChoiceRecord
import ro.zach.language.design.PendingIndication
import ro.zach.language.diagnostics.CoreErrorReport

val LocalAudioPlayer: ProvidableCompositionLocal<AudioPlayer> = staticCompositionLocalOf { error("no AudioPlayer provided") }

/** Something to play: a clip's recording, or its text spoken by the system voice when it has none. */
data class AudioClip(
    val clipKey: String?,
    val text: String,
    val voiceChoice: VoiceChoiceRecord = VoiceChoiceRecord.OwnText,
)

/** Which of a clip's play controls a request is shown on. */
enum class PlaybackControlKind {
    NormalSpeed,
    SlowSpeed,
}

/** A play request as the control which shows it pending identifies it. */
sealed interface PlaybackPendingTarget {
    data class Clip(val clipKey: String?, val text: String, val control: PlaybackControlKind) : PlaybackPendingTarget

    data class Sequence(val clips: List<AudioClip>) : PlaybackPendingTarget
}

/** A play request whose sound has not started; `serial` tells it apart from later requests for the same target. */
private class PendingPlaybackRequest(val target: PlaybackPendingTarget, val serial: Int)

/** Plays clips one at a time; starting a clip or a sequence stops whatever was playing. Call from the main thread. */
class AudioPlayer(private val library: CourseLibrary?, private val output: AudioOutput, private val scope: CoroutineScope) {
    companion object {
        const val NORMAL_RATE: Float = 1.0f
        const val SLOW_RATE: Float = 0.7f

        private fun pendingTarget(clip: AudioClip, control: PlaybackControlKind): PlaybackPendingTarget {
            return PlaybackPendingTarget.Clip(clipKey = clip.clipKey, text = clip.text, control = control)
        }
    }

    private var playbackGeneration: Int = 0
    private var playbackJob: Job? = null
    /* Set once the pending request has outlasted `PendingIndication.delayMilliseconds`. */
    private var indicatedPendingTarget: PlaybackPendingTarget? by mutableStateOf(null)
    private var pendingRequest: PendingPlaybackRequest? = null
    private var pendingRequestSerial: Int = 0
    private var pendingIndicationJob: Job? = null

    init {
        scope.launch { output.prepareWithoutActivating() }
    }

    fun isIndicatedPending(clip: AudioClip, control: PlaybackControlKind): Boolean {
        return indicatedPendingTarget == pendingTarget(clip, control)
    }

    fun isIndicatedPending(sequence: List<AudioClip>): Boolean {
        return indicatedPendingTarget == PlaybackPendingTarget.Sequence(sequence)
    }

    /** Whether a request is waiting for its sound, indicated or not yet. */
    fun hasPendingRequest(): Boolean {
        return pendingRequest != null
    }

    fun stop() {
        playbackGeneration += 1
        playbackJob?.cancel()
        playbackJob = null
        endAnyPendingRequest()
    }

    fun play(clip: AudioClip, engine: AudioEngineRecord, slow: Boolean, control: PlaybackControlKind) {
        stop()

        val generation: Int = playbackGeneration
        val pendingSerial: Int = beginPendingRequest(pendingTarget(clip, control))

        playbackJob = scope.launch {
            playWithoutStopping(clip, engine, slow, generation, pendingSerial)
            endPendingRequest(pendingSerial)
        }
    }

    fun playSequence(clips: List<AudioClip>, engine: AudioEngineRecord, slow: Boolean) {
        stop()

        val generation: Int = playbackGeneration

        playbackJob = scope.launch {
            for (clip in clips) {
                if (playbackGeneration != generation) {
                    return@launch
                }

                val pendingSerial: Int = beginPendingRequest(PlaybackPendingTarget.Sequence(clips))
                val clipOutcome: PlaybackOutcome = playWithoutStopping(clip, engine, slow, generation, pendingSerial)

                endPendingRequest(pendingSerial)

                if (clipOutcome == PlaybackOutcome.Interrupted) {
                    return@launch
                }
            }
        }
    }

    /** Stops any sound and releases the output; the player plays nothing after. */
    fun close() {
        stop()
        output.close()
    }

    private fun beginPendingRequest(target: PlaybackPendingTarget): Int {
        endAnyPendingRequest()
        pendingRequestSerial += 1

        val serial: Int = pendingRequestSerial

        pendingRequest = PendingPlaybackRequest(target, serial)
        pendingIndicationJob = scope.launch {
            PendingIndication.waitForDelay()

            if (pendingRequest?.serial == serial) {
                indicatedPendingTarget = target
            }
        }

        return serial
    }

    /* A request already replaced by a newer one leaves the newer one's pending state alone. */
    private fun endPendingRequest(serial: Int) {
        if (pendingRequest?.serial != serial) {
            return
        }

        endAnyPendingRequest()
    }

    private fun endAnyPendingRequest() {
        pendingIndicationJob?.cancel()
        pendingIndicationJob = null
        pendingRequest = null

        /* Every play control reads this, so it is written only when it changes. */
        if (indicatedPendingTarget != null) {
            indicatedPendingTarget = null
        }
    }

    /* The output reports the start from its own threads. */
    private fun soundStartReport(pendingSerial: Int): () -> Unit {
        return {
            scope.launch { endPendingRequest(pendingSerial) }
        }
    }

    private suspend fun playWithoutStopping(clip: AudioClip, engine: AudioEngineRecord, slow: Boolean, generation: Int, pendingSerial: Int): PlaybackOutcome {
        val recordingPath: String? = findRecording(clip, engine)
        val rate: Float = if (slow) SLOW_RATE else NORMAL_RATE
        val onSoundStarted: () -> Unit = soundStartReport(pendingSerial)

        if (playbackGeneration != generation) {
            return PlaybackOutcome.Stopped
        }

        if (recordingPath == null) {
            return output.speak(clip.text, rate, onSoundStarted)
        }

        val recordingOutcome: PlaybackOutcome = output.playRecording(recordingPath, rate, onSoundStarted)

        if (recordingOutcome != PlaybackOutcome.Failed || playbackGeneration != generation) {
            return recordingOutcome
        }

        return output.speak(clip.text, rate, onSoundStarted)
    }

    /* Off the main thread: the lookup can read and verify a downloaded recording. */
    private suspend fun findRecording(clip: AudioClip, engine: AudioEngineRecord): String? {
        val clipKey: String = clip.clipKey
            ?: return null
        val courseLibrary: CourseLibrary = library
            ?: return null

        return withContext(Dispatchers.IO) {
            val lookup: PlaybackStepInterval = PlaybackTiming.begin(PlaybackStepKind.RecordingLookup)

            try {
                courseLibrary.recordingPath(clipKey, engine, clip.text, clip.voiceChoice)
            } catch (cancelled: CancellationException) {
                throw cancelled
            } catch (error: Exception) {
                CoreErrorReport.record(error, "finding the recording of clip $clipKey")
                null
            } finally {
                PlaybackTiming.end(lookup)
            }
        }
    }
}
