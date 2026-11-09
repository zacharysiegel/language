package ro.zach.language.practice

import androidx.compose.ui.geometry.Rect
import androidx.compose.ui.semantics.SemanticsProperties
import androidx.compose.ui.semantics.getOrNull
import androidx.compose.ui.test.assert
import androidx.compose.ui.test.assertTextEquals
import androidx.compose.ui.test.hasContentDescription
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.ComposeTestRule
import androidx.compose.ui.test.junit4.v2.createEmptyComposeRule
import androidx.compose.ui.test.onAllNodesWithTag
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performScrollTo
import androidx.compose.ui.test.performTextInput
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.json.JSONArray
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import ro.zach.language.testsupport.CourseAppDriver
import ro.zach.language.testsupport.CourseData
import ro.zach.language.unit.UnitPageDriver
import ro.zach.language.unit.UnitTabKind

/* A unit with more than one open response, so the summary lists several answers. */
private const val OPEN_RESPONSE_UNIT_ID: String = "a1-03-ser-estar-feelings"
private const val UNIT_WITHOUT_OPEN_RESPONSES_ID: String = "a1-02-sounds-alphabet"
private const val SELF_CHECK_NOTICE: String = "Automatic grading is not available on this device. In supported web browsers and native desktop applications, an AI model checks your answer and points out each mistake."
private const val NO_QUIZ_TAKEN_SUMMARY: String = "You have not taken the quiz yet."
private const val OPEN_RESPONSE_TIMEOUT_MILLISECONDS: Long = 5_000

@RunWith(AndroidJUnit4::class)
class OpenResponseEndToEndTest {
    @get:Rule
    val composeRule: ComposeTestRule = createEmptyComposeRule()

    private val driver: CourseAppDriver = CourseAppDriver(composeRule)
    private val unitPageDriver: UnitPageDriver = UnitPageDriver(composeRule)
    private val openResponseExercises: JSONArray = CourseData.readOpenResponseExercises(OPEN_RESPONSE_UNIT_ID)

    @Before
    fun launch() {
        driver.launch()
        driver.chooseTheCourseIfTheCourseListOpens()
    }

    @After
    fun close() {
        driver.close()
    }

    @Test
    fun practiceTab_offersTheOpenResponsesWithTheirTaskCount() {
        openThePracticeTab(OPEN_RESPONSE_UNIT_ID)

        driver.waitForTag("open-response", OPEN_RESPONSE_TIMEOUT_MILLISECONDS)
        composeRule.onNodeWithTag("open-response").assert(hasText("Open response (${openResponseExercises.length()} tasks)"))
    }

    @Test
    fun practiceTab_offersNoOpenResponseInAUnitWithoutThem() {
        openThePracticeTab(UNIT_WITHOUT_OPEN_RESPONSES_ID)

        driver.waitForTag("quick-practice", OPEN_RESPONSE_TIMEOUT_MILLISECONDS)
        assertFalse(driver.hasNode("open-response"))
    }

    @Test
    fun openResponses_showTheExampleAnswersAfterCheckThenListYourAnswersAndRecordNothing() {
        val writtenAnswers: List<String> = (1..openResponseExercises.length()).map { taskNumber -> "Resposta $taskNumber.\nEu estou cansado hoje." }

        openThePracticeTab(OPEN_RESPONSE_UNIT_ID)
        tap("open-response")

        for (exerciseIndex in 0 until openResponseExercises.length()) {
            val exercise: JSONObject = openResponseExercises.getJSONObject(exerciseIndex)

            assertTheTaskIsShown(exercise)
            writeAndCheck(writtenAnswers[exerciseIndex])
            assertTheSelfCheck(exercise, writtenAnswers[exerciseIndex])

            if (exerciseIndex == 0) {
                driver.saveScreenshot("25-open-response-self-check")
            }

            tap("continue-exercise")
        }

        driver.waitForTag("your-answers", OPEN_RESPONSE_TIMEOUT_MILLISECONDS)
        composeRule.onNodeWithTag("session-summary").assertTextEquals("Open response")
        assertEquals(writtenAnswers, texts("your-answer"))

        for (exerciseIndex in 0 until openResponseExercises.length()) {
            assertTrue(isTextShown(openResponseExercises.getJSONObject(exerciseIndex).getString("prompt_source")))
        }

        driver.saveScreenshot("26-open-response-your-answers")

        tap("session-done")
        driver.waitForTag("open-response", OPEN_RESPONSE_TIMEOUT_MILLISECONDS)
        assertTrue(isTextShown(NO_QUIZ_TAKEN_SUMMARY))
        assertFalse(driver.hasNode("resume-quiz"))
    }

    private fun openThePracticeTab(unitId: String) {
        driver.openUnit(unitId)
        unitPageDriver.openTab(UnitTabKind.Practice)
    }

    private fun assertTheTaskIsShown(exercise: JSONObject) {
        val promptSource: String = exercise.getString("prompt_source")

        composeRule.waitUntil(OPEN_RESPONSE_TIMEOUT_MILLISECONDS) { isTextShown(promptSource) }

        if (exercise.has("context_source")) {
            assertTrue(isTextShown(exercise.getString("context_source")))
        }
    }

    private fun writeAndCheck(answerText: String) {
        driver.waitForTag("open-response-field", OPEN_RESPONSE_TIMEOUT_MILLISECONDS)
        composeRule.onNodeWithTag("open-response-field").performScrollTo().performTextInput(answerText)
        tap("check")
    }

    private fun assertTheSelfCheck(exercise: JSONObject, writtenAnswer: String) {
        val exampleAnswers: JSONArray = exercise.getJSONArray("example_answers")

        driver.waitForTag("feedback-heading", OPEN_RESPONSE_TIMEOUT_MILLISECONDS)
        composeRule.onNodeWithTag("feedback-heading").assertTextEquals("Compare with the examples")
        composeRule.onNodeWithTag("self-check-notice").assertTextEquals(SELF_CHECK_NOTICE)
        assertTheNoticeIsBetweenTheHeadingAndTheWrittenAnswer()
        composeRule.onNodeWithTag("written-answer").assertTextEquals(writtenAnswer)
        assertEquals((0 until exampleAnswers.length()).map { answerIndex -> exampleAnswers.getString(answerIndex) }, texts("example-answer"))
        composeRule.onNode(hasContentDescription("Play audio: ${exampleAnswers.getString(0)}")).assertExists()
    }

    private fun assertTheNoticeIsBetweenTheHeadingAndTheWrittenAnswer() {
        val headingBounds: Rect = boundsInRoot("feedback-heading")
        val noticeBounds: Rect = boundsInRoot("self-check-notice")
        val writtenAnswerBounds: Rect = boundsInRoot("written-answer")

        assertTrue(noticeBounds.top >= headingBounds.bottom)
        assertTrue(noticeBounds.bottom <= writtenAnswerBounds.top)
    }

    private fun tap(testTag: String) {
        driver.waitForTag(testTag, OPEN_RESPONSE_TIMEOUT_MILLISECONDS)
        composeRule.onNodeWithTag(testTag).performScrollTo().performClick()
    }

    private fun isTextShown(text: String): Boolean {
        return composeRule.onAllNodes(hasText(text)).fetchSemanticsNodes().isNotEmpty()
    }

    private fun boundsInRoot(testTag: String): Rect {
        return composeRule.onNodeWithTag(testTag).fetchSemanticsNode().boundsInRoot
    }

    private fun texts(testTag: String): List<String> {
        return composeRule.onAllNodesWithTag(testTag).fetchSemanticsNodes().map { node ->
            node.config.getOrNull(SemanticsProperties.Text).orEmpty().joinToString("") { text -> text.text }
        }
    }
}
