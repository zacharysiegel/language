package ro.zach.language.app

import androidx.compose.ui.test.assertTextEquals
import androidx.compose.ui.test.junit4.ComposeTestRule
import androidx.compose.ui.test.junit4.v2.createEmptyComposeRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.After
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import ro.zach.language.testsupport.CourseAppDriver
import ro.zach.language.testsupport.CourseData

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
        driver.saveScreenshot("01-home")

        composeRule.onNodeWithTag("courses").performClick()
        driver.waitForTag("course-${CourseData.COURSE_ID}")
    }
}
