package ro.zach.language.unit

import androidx.compose.ui.test.assert
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.ComposeTestRule
import androidx.compose.ui.test.junit4.v2.createEmptyComposeRule
import androidx.compose.ui.test.onAllNodesWithTag
import androidx.compose.ui.test.onFirst
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performScrollTo
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.json.JSONArray
import org.junit.After
import org.junit.Assert.assertFalse
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import ro.zach.language.practice.ExerciseSolver
import ro.zach.language.testsupport.CourseAppDriver
import ro.zach.language.testsupport.CourseData

private const val PAUSED_PRACTICE_ANSWER_COUNT: Int = 3
private const val READING_UNIT_ID: String = "b1-01-present-perfect-participles"
private const val PRACTICE_TIMEOUT_MILLISECONDS: Long = 5_000

@RunWith(AndroidJUnit4::class)
class PracticeSectionEndToEndTest {
    @get:Rule
    val composeRule: ComposeTestRule = createEmptyComposeRule()

    private val driver: CourseAppDriver = CourseAppDriver(composeRule)
    private val unitPageDriver: UnitPageDriver = UnitPageDriver(composeRule)
    private val solver: ExerciseSolver = ExerciseSolver(composeRule)
    private val exercises: JSONArray = CourseData.readUnit(CourseData.PRACTICED_UNIT_ID).getJSONArray("exercises")

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
    fun pausedQuiz_resumesAfterTheAppIsRelaunched() {
        val questionCount: Int = exercises.length()

        openThePracticeTab()
        tap("quiz")
        solver.answerCorrectly(exercises, 0 until PAUSED_PRACTICE_ANSWER_COUNT)
        pauseThePractice()
        assertResumeOffered(questionNumber = PAUSED_PRACTICE_ANSWER_COUNT + 1, questionCount = questionCount)

        driver.relaunchKeepingProgress()
        openThePracticeTab()
        assertResumeOffered(questionNumber = PAUSED_PRACTICE_ANSWER_COUNT + 1, questionCount = questionCount)
        driver.saveScreenshot("24-practice-paused")

        tap("resume-quiz")
        assertQuestionPosition(questionNumber = PAUSED_PRACTICE_ANSWER_COUNT + 1, questionCount = questionCount)
        solver.answerCorrectly(exercises, PAUSED_PRACTICE_ANSWER_COUNT until questionCount)
        assertTheSummaryScoresEveryAnswerCorrect()
        driver.saveScreenshot("05-practice-summary")

        tap("session-done")
        driver.waitForTag("quiz", PRACTICE_TIMEOUT_MILLISECONDS)
        assertFalse(driver.hasNode("resume-quiz"))
    }

    @Test
    fun startOver_replacesThePausedQuiz() {
        val questionCount: Int = exercises.length()

        openThePracticeTab()
        tap("quiz")
        solver.answerCorrectly(exercises, 0 until 1)
        pauseThePractice()
        assertResumeOffered(questionNumber = 2, questionCount = questionCount)

        tap("start-over")
        assertQuestionPosition(questionNumber = 1, questionCount = questionCount)
        pauseThePractice()
        assertResumeOffered(questionNumber = 1, questionCount = questionCount)
    }

    @Test
    fun quickPractice_isQuitAndLeavesThePausedQuiz() {
        openThePracticeTab()
        tap("quiz")
        solver.answerCorrectly(exercises, 0 until 1)
        pauseThePractice()

        tap("quick-practice")
        driver.waitForTag("quit", PRACTICE_TIMEOUT_MILLISECONDS)
        assertFalse(driver.hasNode("pause"))
        tap("quit")

        assertResumeOffered(questionNumber = 2, questionCount = exercises.length())
    }

    @Test
    fun readingQuestions_canBeAnswered() {
        val readings: JSONArray = CourseData.readUnit(READING_UNIT_ID).getJSONArray("readings")
        val questions: JSONArray = readings.getJSONObject(0).getJSONArray("questions")

        driver.openUnit(READING_UNIT_ID)
        unitPageDriver.openTab(UnitTabKind.Readings)
        driver.saveScreenshot("06-reading")
        composeRule.onAllNodesWithTag("comprehension-questions").onFirst().performScrollTo().performClick()

        solver.answerCorrectly(questions, 0 until questions.length())

        driver.waitForTag("session-summary", PRACTICE_TIMEOUT_MILLISECONDS)
    }

    private fun openThePracticeTab() {
        driver.openUnit(CourseData.PRACTICED_UNIT_ID)
        unitPageDriver.openTab(UnitTabKind.Practice)
    }

    private fun tap(testTag: String) {
        driver.waitForTag(testTag, PRACTICE_TIMEOUT_MILLISECONDS)
        composeRule.onNodeWithTag(testTag).performScrollTo().performClick()
    }

    private fun pauseThePractice() {
        driver.waitForTag("pause", PRACTICE_TIMEOUT_MILLISECONDS)
        assertFalse(driver.hasNode("quit"))
        tap("pause")
    }

    private fun assertResumeOffered(questionNumber: Int, questionCount: Int) {
        driver.waitForTag("resume-quiz", PRACTICE_TIMEOUT_MILLISECONDS)
        composeRule.onNodeWithTag("resume-quiz").assert(hasText("question $questionNumber of $questionCount", substring = true))
        driver.waitForTag("start-over", PRACTICE_TIMEOUT_MILLISECONDS)
        assertFalse(driver.hasNode("quiz"))
    }

    private fun assertQuestionPosition(questionNumber: Int, questionCount: Int) {
        driver.waitForTag("question-position", PRACTICE_TIMEOUT_MILLISECONDS)
        composeRule.onNodeWithTag("question-position").assert(hasText("question $questionNumber of $questionCount", substring = true))
    }

    private fun assertTheSummaryScoresEveryAnswerCorrect() {
        val questionCount: Int = exercises.length()

        driver.waitForTag("session-summary", PRACTICE_TIMEOUT_MILLISECONDS)
        composeRule.onNodeWithTag("session-summary").assert(hasText("$questionCount of $questionCount correct (100%)", substring = true))
    }
}
