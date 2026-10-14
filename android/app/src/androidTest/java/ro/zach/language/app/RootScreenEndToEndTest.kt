package ro.zach.language.app

import androidx.compose.ui.semantics.SemanticsProperties
import androidx.compose.ui.test.assertTextEquals
import androidx.compose.ui.test.junit4.ComposeTestRule
import androidx.compose.ui.test.junit4.v2.createEmptyComposeRule
import androidx.compose.ui.test.onAllNodesWithText
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.compose.ui.text.AnnotatedString
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.After
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import ro.zach.language.testsupport.CourseAppDriver
import ro.zach.language.testsupport.CourseData

private const val PAGE_OPEN_TIMEOUT_MILLISECONDS: Long = 5_000
private const val PUBLICATION_LABEL_TIMEOUT_MILLISECONDS: Long = 10_000

@RunWith(AndroidJUnit4::class)
class RootScreenEndToEndTest {
    @get:Rule
    val composeRule: ComposeTestRule = createEmptyComposeRule()

    private val driver: CourseAppDriver = CourseAppDriver(composeRule)

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
    fun navigation_showsTheCourseTitleAndOpensTheCourseList() {
        driver.waitForTag("courses")
        composeRule.onNodeWithTag("courses").assertTextEquals(CourseData.readCourseTitle())
        driver.assertPageHeading(CourseData.readCourseTitle())
        driver.saveScreenshot("01-home")

        composeRule.onNodeWithTag("courses").performClick()
        driver.waitForTag("course-${CourseData.COURSE_ID}")
    }

    @Test
    fun glossaryProgressAndSettings_open() {
        driver.waitForTag("unit-${CourseData.PRACTICED_UNIT_ID}")

        composeRule.onNodeWithTag(RootTabKind.Glossary.testTag()).performClick()
        driver.waitForTag("glossary-entries", PAGE_OPEN_TIMEOUT_MILLISECONDS)
        driver.saveScreenshot("09-glossary")

        composeRule.onNodeWithTag(RootTabKind.Progress.testTag()).performClick()
        composeRule.waitUntil(PAGE_OPEN_TIMEOUT_MILLISECONDS) { composeRule.onAllNodesWithText("Review deck").fetchSemanticsNodes().isNotEmpty() }
        driver.saveScreenshot("10-progress")

        composeRule.onNodeWithTag("settings").performClick()
        driver.waitForTag("voice-kokoro", PAGE_OPEN_TIMEOUT_MILLISECONDS)
        driver.waitForTag("publication-label", PUBLICATION_LABEL_TIMEOUT_MILLISECONDS)

        val publicationLabel: String = composeRule.onNodeWithTag("publication-label").fetchSemanticsNode().config[SemanticsProperties.Text]
            .joinToString(separator = "") { text: AnnotatedString -> text.text }

        assertTrue(publicationLabel.startsWith("Published as "))
        driver.saveScreenshot("11-settings")
    }
}
