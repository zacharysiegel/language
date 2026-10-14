package ro.zach.language.audio

import android.os.Looper
import android.os.SystemClock
import android.os.Trace
import android.util.Log
import ro.zach.language.diagnostics.LogTag
import java.util.Locale
import java.util.concurrent.atomic.AtomicInteger

private const val NANOSECONDS_PER_MILLISECOND: Double = 1_000_000.0

/** A step on the way from a play request to sound. */
enum class PlaybackStepKind {
    RecordingLookup,
    FocusRequest,
    PlayerCreation,
    PlayerPreparation,
    PlayStart,
    SystemVoiceLookup,
    SystemVoiceSpeakStart;

    fun stepName(): String {
        return when (this) {
            RecordingLookup -> "recording_lookup"
            FocusRequest -> "focus_request"
            PlayerCreation -> "player_creation"
            PlayerPreparation -> "player_preparation"
            PlayStart -> "play_start"
            SystemVoiceLookup -> "system_voice_lookup"
            SystemVoiceSpeakStart -> "system_voice_speak_start"
        }
    }
}

/** A step being timed, from `PlaybackTiming.begin` until it is passed to `PlaybackTiming.end`. */
class PlaybackStepInterval(
    val step: PlaybackStepKind,
    val traceCookie: Int,
    val startElapsedNanoseconds: Long,
    val startedOnMainThread: Boolean,
)

/** Times playback steps as trace sections for Perfetto, and as a debug line per step for logcat. */
object PlaybackTiming {
    private val nextTraceCookie: AtomicInteger = AtomicInteger(0)

    fun begin(step: PlaybackStepKind): PlaybackStepInterval {
        val traceCookie: Int = nextTraceCookie.incrementAndGet()

        Trace.beginAsyncSection(step.stepName(), traceCookie)

        return PlaybackStepInterval(
            step = step,
            traceCookie = traceCookie,
            startElapsedNanoseconds = SystemClock.elapsedRealtimeNanos(),
            startedOnMainThread = Looper.myLooper() == Looper.getMainLooper(),
        )
    }

    fun end(interval: PlaybackStepInterval) {
        val elapsedNanoseconds: Long = SystemClock.elapsedRealtimeNanos() - interval.startElapsedNanoseconds
        val durationMilliseconds: String = String.format(Locale.ROOT, "%.2f", elapsedNanoseconds / NANOSECONDS_PER_MILLISECOND)

        Trace.endAsyncSection(interval.step.stepName(), interval.traceCookie)
        Log.d(LogTag.AUDIO, "playback step finished; [step=${interval.step.stepName()} duration_ms=$durationMilliseconds main_thread=${interval.startedOnMainThread}]")
    }
}
