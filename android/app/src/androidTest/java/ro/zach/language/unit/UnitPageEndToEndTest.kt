package ro.zach.language.unit

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
class UnitPageEndToEndTest {
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
    fun unitPage_opensEveryTab() {
        driver.openUnit(CourseData.PRACTICED_UNIT_ID)
        driver.saveScreenshot("02-unit-overview")

        for (tab in listOf(UnitTabKind.Grammar, UnitTabKind.Dialogues, UnitTabKind.Vocabulary, UnitTabKind.Practice)) {
            val tabTag: String = "tab-${tab.rawValue()}"

            composeRule.onNodeWithTag(tabTag).performClick()
            driver.waitForTag(tabTag)
            driver.saveScreenshot("03-unit-${tab.rawValue()}")
        }
    }
}
