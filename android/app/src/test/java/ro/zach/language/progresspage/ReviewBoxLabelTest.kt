package ro.zach.language.progresspage

import org.junit.Assert.assertEquals
import org.junit.Test

class ReviewBoxLabelTest {
    @Test
    fun reviewBoxLabel_namesTheFirstBoxNewOrForgottenAndTheOthersByTheirInterval() {
        assertEquals("New or forgotten", reviewBoxLabel(0))
        assertEquals("Next review after 3 day(s)", reviewBoxLabel(3))
    }
}
