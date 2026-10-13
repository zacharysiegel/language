package ro.zach.language.practice

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test
import ro.zach.language.core.AnswerStatusRecord

private const val EXERCISE_COUNT: Int = 4

class ExerciseSessionStateTest {
    private val right: ExerciseResult = ExerciseResult(status = AnswerStatusRecord.CORRECT, answerText = "right")
    private val accented: ExerciseResult = ExerciseResult(status = AnswerStatusRecord.ACCENT_MISTAKE, answerText = "accented")
    private val wrong: ExerciseResult = ExerciseResult(status = AnswerStatusRecord.WRONG, answerText = "wrong")

    private fun answer(state: ExerciseSessionState, results: List<ExerciseResult>) {
        for (result in results) {
            state.submit(result)
            state.advance()
        }
    }

    @Test
    fun start_continuesAfterTheAnsweredResults() {
        val state: ExerciseSessionState = ExerciseSessionState.start(exerciseCount = EXERCISE_COUNT, answeredResults = listOf(right, wrong))

        assertEquals(2, state.currentPosition())
        assertEquals(2, state.currentExerciseIndex())
        assertEquals(listOf(right, wrong), state.results())
    }

    @Test
    fun submit_takesOneAnswerPerQuestionUntilTheLearnerContinues() {
        val state: ExerciseSessionState = ExerciseSessionState.start(exerciseCount = EXERCISE_COUNT, answeredResults = emptyList())

        assertEquals(ExerciseSubmissionOutcome.Recorded, state.submit(wrong))
        assertEquals(ExerciseSubmissionOutcome.AlreadyAnswered, state.submit(right))
        assertEquals(wrong, state.currentResult())

        state.advance()

        assertNull(state.currentResult())
        assertEquals(1, state.currentPosition())
        assertEquals(listOf(wrong), state.results())
    }

    @Test
    fun takeCompletion_reportsTheFirstPassOnceItsLastAnswerIsIn() {
        val state: ExerciseSessionState = ExerciseSessionState.start(exerciseCount = EXERCISE_COUNT, answeredResults = emptyList())

        answer(state, listOf(right, accented, wrong))
        assertNull(state.takeCompletion())

        answer(state, listOf(right))

        assertEquals(ExerciseSessionCompletion(correctCount = 3, totalCount = EXERCISE_COUNT), state.takeCompletion())
        assertNull(state.takeCompletion())
    }

    @Test
    fun retryMissed_asksOnlyTheMissedExercisesAndReportsNoCompletion() {
        val state: ExerciseSessionState = ExerciseSessionState.start(exerciseCount = EXERCISE_COUNT, answeredResults = emptyList())

        answer(state, listOf(wrong, right, wrong, accented))
        state.takeCompletion()
        state.retryMissed()

        assertTrue(state.isRetry())
        assertEquals(listOf(0, 2), state.askedExerciseIndexes())
        assertEquals(0, state.currentExerciseIndex())

        answer(state, listOf(right, wrong))

        assertNull(state.currentExerciseIndex())
        assertNull(state.takeCompletion())
        assertEquals(listOf(2), state.missedExerciseIndexes())
    }
}
