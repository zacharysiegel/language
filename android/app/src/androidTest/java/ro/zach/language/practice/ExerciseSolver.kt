package ro.zach.language.practice

import androidx.compose.ui.test.SemanticsMatcher
import androidx.compose.ui.test.assertTextEquals
import androidx.compose.ui.test.hasTestTag
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.isEnabled
import androidx.compose.ui.test.junit4.ComposeTestRule
import androidx.compose.ui.test.onAllNodesWithTag
import androidx.compose.ui.test.onFirst
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performScrollTo
import androidx.compose.ui.test.performTextInput
import org.json.JSONArray
import org.json.JSONObject

private const val EXERCISE_TIMEOUT_MILLISECONDS: Long = 5_000

/** Answers exercises from their course JSON as a learner who knows every answer would. */
class ExerciseSolver(private val composeRule: ComposeTestRule) {
    fun solve(exercise: JSONObject) {
        when (val kind: String = exercise.getString("type")) {
            "multiple_choice", "listening_choice" -> {
                val choices: JSONArray = exercise.getJSONArray("choices")

                tapEnabled(testTag = "choice", label = choices.getString(exercise.getInt("answer_index")))
            }
            "fill_blank", "translate_to_target" -> typeAnswer(exercise.getJSONArray("answers").getString(0))
            "dictation" -> typeAnswer(exercise.getString("audio_target"))
            "order_words" -> {
                val words: JSONArray = exercise.getJSONArray("words")

                for (wordIndex in 0 until words.length()) {
                    tapEnabled(testTag = "bank-token", label = words.getString(wordIndex))
                }

                composeRule.onNodeWithTag("check").performScrollTo().performClick()
            }
            "match" -> {
                val pairs: JSONArray = exercise.getJSONArray("pairs")

                for (pairIndex in 0 until pairs.length()) {
                    val pair: JSONObject = pairs.getJSONObject(pairIndex)

                    tapEnabled(testTag = "match-left", label = pair.getString("target"))
                    tapEnabled(testTag = "match-right", label = pair.getString("source"))
                }
            }
            else -> throw AssertionError("unknown exercise kind $kind")
        }
    }

    fun assertCorrect() {
        waitForTag("feedback-heading")
        composeRule.onNodeWithTag("feedback-heading").assertTextEquals("Correct")
    }

    fun continueToTheNextQuestion() {
        composeRule.onNodeWithTag("continue-exercise").performScrollTo().performClick()
    }

    /** Solves each exercise at `indexes`, checks it was graded correct and continues. */
    fun answerCorrectly(exercises: JSONArray, indexes: IntRange) {
        for (index in indexes) {
            solve(exercises.getJSONObject(index))
            assertCorrect()
            continueToTheNextQuestion()
        }
    }

    private fun typeAnswer(text: String) {
        waitForTag("answer-field")
        composeRule.onNodeWithTag("answer-field").performScrollTo().performTextInput(text)
        composeRule.onNodeWithTag("check").performScrollTo().performClick()
    }

    private fun tapEnabled(testTag: String, label: String) {
        val matcher: SemanticsMatcher = hasTestTag(testTag) and hasText(label) and isEnabled()

        composeRule.waitUntil(EXERCISE_TIMEOUT_MILLISECONDS) { composeRule.onAllNodes(matcher).fetchSemanticsNodes().isNotEmpty() }
        composeRule.onAllNodes(matcher).onFirst().performScrollTo().performClick()
    }

    private fun waitForTag(testTag: String) {
        composeRule.waitUntil(EXERCISE_TIMEOUT_MILLISECONDS) { composeRule.onAllNodesWithTag(testTag).fetchSemanticsNodes().isNotEmpty() }
    }
}
