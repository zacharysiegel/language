package ro.zach.language.audio

import android.media.AudioManager
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class AudioInterruptionEventTest {
    @Test
    fun fromFocusChange_transientLossesBeginAnInterruption() {
        assertEquals(AudioInterruptionEvent.Began, AudioInterruptionEvent.fromFocusChange(AudioManager.AUDIOFOCUS_LOSS_TRANSIENT))
        assertEquals(AudioInterruptionEvent.Began, AudioInterruptionEvent.fromFocusChange(AudioManager.AUDIOFOCUS_LOSS_TRANSIENT_CAN_DUCK))
    }

    @Test
    fun fromFocusChange_gainEndsAnInterruptionToResume() {
        assertEquals(AudioInterruptionEvent.EndedShouldResume, AudioInterruptionEvent.fromFocusChange(AudioManager.AUDIOFOCUS_GAIN))
    }

    @Test
    fun fromFocusChange_permanentLossEndsWithoutResuming() {
        assertEquals(AudioInterruptionEvent.EndedShouldNotResume, AudioInterruptionEvent.fromFocusChange(AudioManager.AUDIOFOCUS_LOSS))
    }

    @Test
    fun fromFocusChange_otherGainsAreIgnored() {
        assertNull(AudioInterruptionEvent.fromFocusChange(AudioManager.AUDIOFOCUS_GAIN_TRANSIENT))
        assertNull(AudioInterruptionEvent.fromFocusChange(AudioManager.AUDIOFOCUS_NONE))
    }
}
