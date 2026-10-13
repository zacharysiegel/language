package ro.zach.language.practice

import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.Saver
import androidx.compose.runtime.saveable.listSaver
import androidx.compose.runtime.setValue
import ro.zach.language.core.AnswerStatusRecord
import ro.zach.language.core.SavedExerciseResultRecord
import java.io.Serializable

/** The learner's answer to one exercise and the answer to show back. */
data class ExerciseResult(val status: AnswerStatusRecord, val answerText: String) : Serializable {
    fun isCorrect(): Boolean {
        return status != AnswerStatusRecord.WRONG
    }

    fun savedResult(): SavedExerciseResultRecord {
        return SavedExerciseResultRecord(status = status, answerText = answerText)
    }

    companion object {
        fun fromChoice(chosenCorrectly: Boolean, answerText: String): ExerciseResult {
            val status: AnswerStatusRecord = if (chosenCorrectly) AnswerStatusRecord.CORRECT else AnswerStatusRecord.WRONG

            return ExerciseResult(status = status, answerText = answerText)
        }

        fun fromSaved(savedResult: SavedExerciseResultRecord): ExerciseResult {
            return ExerciseResult(status = savedResult.status, answerText = savedResult.answerText)
        }
    }
}

/** Whether an answer was taken as the current question's. */
enum class ExerciseSubmissionOutcome {
    Recorded,
    AlreadyAnswered,
}

/** The counts of a first pass through a session, once its last question is answered. */
data class ExerciseSessionCompletion(val correctCount: Int, val totalCount: Int)

/** Which exercises a session asks, how far it has got and the answers so far; positions index the asked exercises. */
class ExerciseSessionState private constructor(
    private val exerciseCount: Int,
    retryExerciseIndexes: List<Int>?,
    currentPosition: Int,
    results: List<ExerciseResult>,
    currentResult: ExerciseResult?,
    isCompletionReported: Boolean,
) {
    /* Indexes into the session's exercises; `null` on the first pass, which asks every exercise in order. */
    private var retryExerciseIndexes: List<Int>? by mutableStateOf(retryExerciseIndexes)
    private var currentPosition: Int by mutableIntStateOf(currentPosition)
    private var results: List<ExerciseResult> by mutableStateOf(results)
    private var currentResult: ExerciseResult? by mutableStateOf(currentResult)
    private var isCompletionReported: Boolean by mutableStateOf(isCompletionReported)

    /** Indexes into the session's exercises, in the order they are asked. */
    fun askedExerciseIndexes(): List<Int> {
        return retryExerciseIndexes ?: (0 until exerciseCount).toList()
    }

    fun isRetry(): Boolean {
        return retryExerciseIndexes != null
    }

    fun currentPosition(): Int {
        return currentPosition
    }

    /** The answer to the current question, shown as feedback until the learner continues. */
    fun currentResult(): ExerciseResult? {
        return currentResult
    }

    /** The index into the session's exercises of the question being asked; `null` once every one is answered. */
    fun currentExerciseIndex(): Int? {
        return askedExerciseIndexes().getOrNull(currentPosition)
    }

    fun results(): List<ExerciseResult> {
        return results
    }

    fun correctCount(): Int {
        return results.count { result -> result.isCorrect() }
    }

    /** The asked exercises answered wrongly, by their index into the session's exercises. */
    fun missedExerciseIndexes(): List<Int> {
        return askedExerciseIndexes().zip(results)
            .filter { (_, result) -> !result.isCorrect() }
            .map { (exerciseIndex, _) -> exerciseIndex }
    }

    fun submit(result: ExerciseResult): ExerciseSubmissionOutcome {
        if (currentResult != null || currentExerciseIndex() == null) {
            return ExerciseSubmissionOutcome.AlreadyAnswered
        }

        results = results + result
        currentResult = result

        return ExerciseSubmissionOutcome.Recorded
    }

    fun advance() {
        if (currentResult == null) {
            return
        }

        currentResult = null
        currentPosition += 1
    }

    /** The first pass's counts, the first time this is called after its last answer; `null` otherwise and in a retry. */
    fun takeCompletion(): ExerciseSessionCompletion? {
        if (isRetry() || isCompletionReported || currentExerciseIndex() != null) {
            return null
        }

        isCompletionReported = true

        return ExerciseSessionCompletion(correctCount = correctCount(), totalCount = askedExerciseIndexes().size)
    }

    /** Asks the missed exercises again; their answers are not saved and the retry reports no completion. */
    fun retryMissed() {
        val missedExerciseIndexes: List<Int> = missedExerciseIndexes()

        retryExerciseIndexes = missedExerciseIndexes
        results = emptyList()
        currentResult = null
        currentPosition = 0
    }

    companion object {
        /** Continues after `answeredResults`, the answers to the first exercises in order. */
        fun start(exerciseCount: Int, answeredResults: List<ExerciseResult>): ExerciseSessionState {
            return ExerciseSessionState(
                exerciseCount = exerciseCount,
                retryExerciseIndexes = null,
                currentPosition = answeredResults.size,
                results = answeredResults,
                currentResult = null,
                isCompletionReported = false,
            )
        }

        fun saver(exerciseCount: Int): Saver<ExerciseSessionState, Any> {
            return listSaver(
                save = { state: ExerciseSessionState ->
                    listOf(
                        state.retryExerciseIndexes?.let { indexes -> ArrayList(indexes) },
                        state.currentPosition,
                        ArrayList(state.results),
                        state.currentResult,
                        state.isCompletionReported,
                    )
                },
                restore = { values: List<Any?> ->
                    @Suppress("UNCHECKED_CAST")
                    ExerciseSessionState(
                        exerciseCount = exerciseCount,
                        retryExerciseIndexes = values[0] as List<Int>?,
                        currentPosition = values[1] as Int,
                        results = values[2] as List<ExerciseResult>,
                        currentResult = values[3] as ExerciseResult?,
                        isCompletionReported = values[4] as Boolean,
                    )
                },
            )
        }
    }
}
