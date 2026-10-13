package ro.zach.language.unit

import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Rect
import androidx.compose.ui.semantics.SemanticsNode
import androidx.compose.ui.semantics.SemanticsProperties
import androidx.compose.ui.semantics.getOrNull
import androidx.compose.ui.test.SemanticsNodeInteraction
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.click
import androidx.compose.ui.test.junit4.ComposeTestRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onRoot
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performScrollTo
import androidx.compose.ui.test.performTouchInput
import androidx.compose.ui.test.swipeUp
import androidx.compose.ui.unit.Density
import androidx.test.platform.app.InstrumentationRegistry
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue

private const val SELECTION_TIMEOUT_MILLISECONDS: Long = 5_000
private const val SLOW_SWIPE_DURATION_MILLISECONDS: Long = 600
private const val FAST_SWIPE_DURATION_MILLISECONDS: Long = 80
/* Enough frames for a scroll's start to show the rail and give it its presses. */
private const val RAIL_SHOWING_FRAME_COUNT: Int = 4
private const val RAIL_SETTLING_FRAME_COUNT: Int = 4
private const val LAYOUT_TOLERANCE_DP: Float = 1f
/* The app's offset of the rail's shadow. */
private const val INDEX_RAIL_SHADOW_OFFSET_DP: Float = 2f
private const val UNIT_ITEM_INDEX_TEST_TAG: String = "unit-item-index"

/** Drives the unit page's tabs and index rail, and checks where its content lies. */
class UnitPageDriver(private val composeRule: ComposeTestRule) {
    private val density: Density = Density(InstrumentationRegistry.getInstrumentation().targetContext)

    fun openTab(tab: UnitTabKind) {
        composeRule.onNodeWithTag(tabTag(tab)).performScrollTo().performClick()
        composeRule.waitForIdle()
    }

    fun tabTag(tab: UnitTabKind): String {
        return "tab-${tab.rawValue()}"
    }

    fun railEntryTag(entryNumber: Int): String {
        return "$UNIT_ITEM_INDEX_TEST_TAG-$entryNumber"
    }

    fun isSelected(testTag: String): Boolean {
        val node: SemanticsNode = composeRule.onNodeWithTag(testTag).fetchSemanticsNode()

        return node.config.getOrNull(SemanticsProperties.Selected) == true
    }

    fun assertSelected(testTag: String) {
        composeRule.waitUntil(SELECTION_TIMEOUT_MILLISECONDS) { isSelected(testTag) }
    }

    fun boundsInRoot(testTag: String): Rect {
        return composeRule.onNodeWithTag(testTag).fetchSemanticsNode().boundsInRoot
    }

    fun rootBounds(): Rect {
        return composeRule.onRoot().fetchSemanticsNode().boundsInRoot
    }

    /** Fails unless the node shows within the top quarter of the page's viewport. */
    fun assertNearTopOfThePage(node: SemanticsNodeInteraction) {
        val pageBounds: Rect = boundsInRoot(UNIT_PAGE_TEST_TAG)
        val nodeTop: Float = node.fetchSemanticsNode().boundsInRoot.top

        node.assertIsDisplayed()
        assertTrue(nodeTop >= pageBounds.top - layoutTolerance())
        assertTrue(nodeTop < pageBounds.top + pageBounds.height / 4)
    }

    /** Fails when the page's content column keeps further from the trailing edge than from the leading edge. */
    fun assertTheContentIsInsetEquallyFromBothEdges() {
        val screenBounds: Rect = rootBounds()
        val tabRowBounds: Rect = boundsInRoot("unit-tabs")

        assertFalse(tabRowBounds.isEmpty)
        assertEquals(tabRowBounds.left - screenBounds.left, screenBounds.right - tabRowBounds.right, layoutTolerance())
    }

    /** Fails when the rail's shadow reaches the screen's trailing edge. */
    fun assertTheIndexRailShadowClearsTheTrailingEdge() {
        val railTrailingClearance: Float = rootBounds().right - boundsInRoot(railEntryTag(1)).right
        assertTrue(railTrailingClearance > pixels(INDEX_RAIL_SHADOW_OFFSET_DP) + layoutTolerance())
    }

    /** Fails when a press where the rail's entry for `entryNumber` is drawn jumps to that box. */
    fun assertTheIndexRailLetsTouchesThrough(entryNumber: Int) {
        val entryCenter: Offset = boundsInRoot(railEntryTag(entryNumber)).center

        assertFalse(isSelected(railEntryTag(entryNumber)))
        composeRule.onRoot().performTouchInput { click(entryCenter) }
        composeRule.waitForIdle()
        assertFalse(isSelected(railEntryTag(entryNumber)))
    }

    /** Scrolls the page a little, which shows the index rail, and runs `railActions` before it can hide again. */
    fun withTheIndexRailShown(railActions: () -> Unit) {
        withoutWaitingForThePageToSettle {
            swipeThePageUp(SLOW_SWIPE_DURATION_MILLISECONDS)
            advanceFrames(RAIL_SHOWING_FRAME_COUNT)
            railActions()
            advanceFrames(RAIL_SETTLING_FRAME_COUNT)
        }
    }

    /** Runs `actions` with the clock advanced only by their own events and frames, so a fling is still moving. */
    fun withoutWaitingForThePageToSettle(actions: () -> Unit) {
        composeRule.mainClock.autoAdvance = false

        try {
            actions()
        } finally {
            composeRule.mainClock.autoAdvance = true
        }

        composeRule.waitForIdle()
    }

    fun swipeThePageUpSlowly() {
        swipeThePageUp(SLOW_SWIPE_DURATION_MILLISECONDS)
    }

    fun flingThePageUp() {
        swipeThePageUp(FAST_SWIPE_DURATION_MILLISECONDS)
    }

    fun advanceFrames(frameCount: Int) {
        repeat(frameCount) {
            composeRule.mainClock.advanceTimeByFrame()
        }
    }

    fun pressRailEntry(entryNumber: Int) {
        composeRule.onNodeWithTag(railEntryTag(entryNumber)).performTouchInput { click() }
    }

    /** Presses the rail's first entry, then drags along the rail to its entry `lastEntryNumber`. */
    fun dragAlongTheRail(lastEntryNumber: Int, holdMilliseconds: Long, stepCount: Int) {
        val start: Offset = boundsInRoot(railEntryTag(1)).center
        val end: Offset = boundsInRoot(railEntryTag(lastEntryNumber)).center

        composeRule.onRoot().performTouchInput {
            down(start)
            advanceEventTime(holdMilliseconds)

            for (step in 1..stepCount) {
                moveTo(start + (end - start) * (step.toFloat() / stepCount))
            }

            up()
        }
    }

    private fun swipeThePageUp(durationMilliseconds: Long) {
        composeRule.onNodeWithTag(UNIT_PAGE_TEST_TAG).performTouchInput {
            swipeUp(startY = centerY + height / 4f, endY = centerY - height / 4f, durationMillis = durationMilliseconds)
        }
    }

    private fun layoutTolerance(): Float {
        return pixels(LAYOUT_TOLERANCE_DP)
    }

    private fun pixels(dp: Float): Float {
        return dp * density.density
    }
}
