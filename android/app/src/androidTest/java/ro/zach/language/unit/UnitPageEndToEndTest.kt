package ro.zach.language.unit

import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.ComposeTestRule
import androidx.compose.ui.test.junit4.v2.createEmptyComposeRule
import androidx.compose.ui.test.onAllNodesWithTag
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performScrollTo
import androidx.compose.ui.geometry.Rect
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import ro.zach.language.testsupport.CourseAppDriver
import ro.zach.language.testsupport.CourseData

private const val TIP_TIMEOUT_MILLISECONDS: Long = 5_000
/* Longer than the tip's auto close, with room for a slow emulator. */
private const val TIP_AUTO_CLOSE_ALLOWANCE_MILLISECONDS: Long = 12_000
private const val CLIP_PLAYBACK_ALLOWANCE_MILLISECONDS: Long = 3_000
private const val STATUS_TIP_TEST_TAG: String = "unit-status-help-tip"
private val playedButtonIndexes: List<Int> = listOf(0, 0, 1, 2)

@RunWith(AndroidJUnit4::class)
class UnitPageEndToEndTest {
    @get:Rule
    val composeRule: ComposeTestRule = createEmptyComposeRule()

    private val driver: CourseAppDriver = CourseAppDriver(composeRule)
    private val unitPage: UnitPageDriver = UnitPageDriver(composeRule)

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
        driver.assertPageHeading(CourseData.readUnit(CourseData.PRACTICED_UNIT_ID).getString("title"))
        driver.saveScreenshot("02-unit-overview")

        for (tab in listOf(UnitTabKind.Grammar, UnitTabKind.Dialogues, UnitTabKind.Vocabulary, UnitTabKind.Practice)) {
            unitPage.openTab(tab)
            unitPage.assertSelected(unitPage.tabTag(tab))
            driver.saveScreenshot("03-unit-${tab.rawValue()}")
        }
    }

    @Test
    fun unitStatus_isSetByTheStatusControlAndPractice() {
        driver.openUnit(CourseData.PRACTICED_UNIT_ID)
        unitPage.assertSelected("unit-status-not-started")
        driver.saveScreenshot("12-unit-status")

        for (status in listOf("in-progress", "completed", "not-started")) {
            composeRule.onNodeWithTag("unit-status-$status").performClick()
            unitPage.assertSelected("unit-status-$status")
        }

        unitPage.openTab(UnitTabKind.Practice)
        composeRule.onNodeWithTag("quick-practice").performScrollTo().performClick()
        driver.waitForTag("quit", TIP_TIMEOUT_MILLISECONDS)
        composeRule.onNodeWithTag("quit").performScrollTo().performClick()

        unitPage.openTab(UnitTabKind.Overview)
        unitPage.assertSelected("unit-status-in-progress")
    }

    @Test
    fun unitStatusHelp_floatsOverThePageAndCloses() {
        driver.openUnit(CourseData.PRACTICED_UNIT_ID)

        val tabBoundsBeforeOpening: Rect = unitPage.boundsInRoot(unitPage.tabTag(UnitTabKind.Overview))

        openTheStatusHelp()
        assertEquals(tabBoundsBeforeOpening, unitPage.boundsInRoot(unitPage.tabTag(UnitTabKind.Overview)))
        driver.saveScreenshot("13-unit-status-help")

        composeRule.onNodeWithTag("unit-status-help-close").performClick()
        waitForTheStatusHelpToClose(TIP_TIMEOUT_MILLISECONDS)

        openTheStatusHelp()
        composeRule.onNodeWithTag("add-words").performClick()
        waitForTheStatusHelpToClose(TIP_TIMEOUT_MILLISECONDS)
        assertTrue(composeRule.onAllNodes(hasText("added", substring = true)).fetchSemanticsNodes().isEmpty())

        openTheStatusHelp()
        composeRule.mainClock.advanceTimeBy(UNIT_STATUS_HELP_AUTO_CLOSE_MILLISECONDS)
        waitForTheStatusHelpToClose(TIP_AUTO_CLOSE_ALLOWANCE_MILLISECONDS)
    }

    @Test
    fun playingClips_leavesThePageUsable() {
        driver.openUnit(CourseData.PRACTICED_UNIT_ID)
        unitPage.openTab(UnitTabKind.Vocabulary)
        driver.waitForTag("play", TIP_TIMEOUT_MILLISECONDS)

        for (buttonIndex in playedButtonIndexes) {
            composeRule.onAllNodesWithTag("play")[buttonIndex].performScrollTo().performClick()
            Thread.sleep(CLIP_PLAYBACK_ALLOWANCE_MILLISECONDS)
        }

        unitPage.openTab(UnitTabKind.Overview)
        driver.waitForTag("unit-status-not-started", TIP_TIMEOUT_MILLISECONDS)
    }

    private fun openTheStatusHelp() {
        composeRule.onNodeWithTag("unit-status-help").performClick()
        driver.waitForTag(STATUS_TIP_TEST_TAG, TIP_TIMEOUT_MILLISECONDS)
    }

    private fun waitForTheStatusHelpToClose(timeoutMilliseconds: Long) {
        composeRule.waitUntil(timeoutMilliseconds) { !driver.hasNode(STATUS_TIP_TEST_TAG) }
    }
}
