package ro.zach.language.progresspage

import androidx.compose.ui.test.assert
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.ComposeTestRule
import androidx.compose.ui.test.junit4.v2.createEmptyComposeRule
import androidx.compose.ui.test.onAllNodesWithText
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performScrollTo
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.json.JSONArray
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import ro.zach.language.app.RootTabKind
import ro.zach.language.practice.ExerciseSolver
import ro.zach.language.testsupport.CourseAppDriver
import ro.zach.language.testsupport.CourseData
import ro.zach.language.unit.UnitPageDriver
import ro.zach.language.unit.UnitTabKind

private const val PRACTICED_LEVEL_CODE: String = "A1"
private const val PAGE_OPEN_TIMEOUT_MILLISECONDS: Long = 5_000
private const val UNIT_OPEN_TIMEOUT_MILLISECONDS: Long = 15_000

@RunWith(AndroidJUnit4::class)
class ProgressPageEndToEndTest {
    @get:Rule
    val composeRule: ComposeTestRule = createEmptyComposeRule()

    private val driver: CourseAppDriver = CourseAppDriver(composeRule)
    private val unitPageDriver: UnitPageDriver = UnitPageDriver(composeRule)
    private val solver: ExerciseSolver = ExerciseSolver(composeRule)

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
    fun progressPage_showsCompletionPerLevelAndQuizResults() {
        val exercises: JSONArray = CourseData.readQuizExercises(CourseData.PRACTICED_UNIT_ID)
        val levelUnitCount: Int = readLevelUnitCount(PRACTICED_LEVEL_CODE)
        val quizResultTag: String = "quiz-result-${CourseData.PRACTICED_UNIT_ID}"

        openTheProgressPage()
        driver.waitForTag("quiz-results-empty", PAGE_OPEN_TIMEOUT_MILLISECONDS)
        assertLevelCompletion(completedUnitCount = 0, unitCount = levelUnitCount)

        composeRule.onNodeWithTag(RootTabKind.Course.testTag()).performClick()
        driver.openUnit(CourseData.PRACTICED_UNIT_ID)
        unitPageDriver.openTab(UnitTabKind.Practice)
        tap("quiz")
        solver.answerCorrectly(exercises, 0 until exercises.length())
        tap("session-done")

        openTheProgressPage()
        driver.waitForTag(quizResultTag, PAGE_OPEN_TIMEOUT_MILLISECONDS)
        composeRule.onNodeWithTag(quizResultTag).assert(hasText("Best score: 100% over 1 attempt"))
        assertFalse(driver.hasNode("quiz-results-empty"))
        assertLevelCompletion(completedUnitCount = 1, unitCount = levelUnitCount)
        driver.saveScreenshot("28-progress-recorded")

        composeRule.onNodeWithTag(quizResultTag).performScrollTo().performClick()
        driver.waitForTag("tab-overview", UNIT_OPEN_TIMEOUT_MILLISECONDS)
        driver.assertPageHeading(CourseData.readUnit(CourseData.PRACTICED_UNIT_ID).getString("title"))

        driver.relaunchKeepingProgress()
        driver.waitForTag("continue")
        assertTrue(composeRule.onAllNodesWithText("units completed", substring = true).fetchSemanticsNodes().isEmpty())
        driver.saveScreenshot("29-course-progress-recorded")
    }

    private fun openTheProgressPage() {
        composeRule.onNodeWithTag(RootTabKind.Progress.testTag()).performClick()
        composeRule.waitUntil(PAGE_OPEN_TIMEOUT_MILLISECONDS) { composeRule.onAllNodesWithText("Review deck").fetchSemanticsNodes().isNotEmpty() }
    }

    private fun tap(testTag: String) {
        driver.waitForTag(testTag, PAGE_OPEN_TIMEOUT_MILLISECONDS)
        composeRule.onNodeWithTag(testTag).performScrollTo().performClick()
    }

    private fun assertLevelCompletion(completedUnitCount: Int, unitCount: Int) {
        val levelCompletionTag: String = "level-completion-$PRACTICED_LEVEL_CODE"

        driver.waitForTag(levelCompletionTag, PAGE_OPEN_TIMEOUT_MILLISECONDS)
        composeRule.onNodeWithTag(levelCompletionTag).assert(hasText("$completedUnitCount of $unitCount units completed", substring = true))
    }

    private fun readLevelUnitCount(levelCode: String): Int {
        val curriculumUnits: JSONArray = CourseData.readCurriculum().getJSONArray("units")

        return (0 until curriculumUnits.length()).count { index ->
            val curriculumUnit: JSONObject = curriculumUnits.getJSONObject(index)

            curriculumUnit.getString("level") == levelCode
        }
    }
}
