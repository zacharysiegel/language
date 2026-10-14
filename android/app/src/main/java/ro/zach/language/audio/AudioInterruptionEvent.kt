package ro.zach.language.audio

import android.media.AudioManager

/** An audio focus change, read as the start or end of an interruption of the output's sound. */
enum class AudioInterruptionEvent {
    Began,
    EndedShouldResume,
    EndedShouldNotResume;

    companion object {
        /** `null` for a change that neither starts nor ends an interruption. */
        fun fromFocusChange(focusChange: Int): AudioInterruptionEvent? {
            return when (focusChange) {
                /* Speech is paused rather than spoken under another sound. */
                AudioManager.AUDIOFOCUS_LOSS_TRANSIENT, AudioManager.AUDIOFOCUS_LOSS_TRANSIENT_CAN_DUCK -> Began
                AudioManager.AUDIOFOCUS_GAIN -> EndedShouldResume
                AudioManager.AUDIOFOCUS_LOSS -> EndedShouldNotResume
                else -> null
            }
        }
    }
}
