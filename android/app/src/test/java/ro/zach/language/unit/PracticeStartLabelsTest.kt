package ro.zach.language.unit

import org.junit.Assert.assertEquals
import org.junit.Test

class PracticeStartLabelsTest {
    @Test
    fun quizLabel_countsTheExercises() {
        assertEquals("Quiz (1 exercise)", quizLabel(1))
        assertEquals("Quiz (26 exercises)", quizLabel(26))
    }

    @Test
    fun openResponseLabel_countsTheTasks() {
        assertEquals("Open response (1 task)", openResponseLabel(1))
        assertEquals("Open response (2 tasks)", openResponseLabel(2))
    }
}
