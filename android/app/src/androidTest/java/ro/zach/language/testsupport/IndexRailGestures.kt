package ro.zach.language.testsupport

import androidx.compose.ui.test.junit4.ComposeTestRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performTouchInput
import androidx.compose.ui.test.swipeUp

/* Enough frames for a scroll's start to show the rail and give it its presses. */
private const val RAIL_SHOWING_FRAME_COUNT: Int = 4
private const val RAIL_SETTLING_FRAME_COUNT: Int = 4

/** Scrolls a page that has an index rail and acts on the rail while it shows. */
class IndexRailGestures(private val composeRule: ComposeTestRule) {
    companion object {
        /* Slow enough to scroll without a long fling. */
        const val SLOW_SWIPE_DURATION_MILLISECONDS: Long = 600
    }

    /** Scrolls the node tagged `scrollingNodeTag` a little, which shows its index rail, and runs `railActions` before the rail can hide again. */
    fun withTheIndexRailShown(scrollingNodeTag: String, railActions: () -> Unit) {
        withoutWaitingToSettle {
            swipeNodeUp(scrollingNodeTag, SLOW_SWIPE_DURATION_MILLISECONDS)
            advanceFrames(RAIL_SHOWING_FRAME_COUNT)
            railActions()
            advanceFrames(RAIL_SETTLING_FRAME_COUNT)
        }
    }

    /** Runs `actions` with the clock advanced only by their own events and frames, so a fling is still moving. */
    fun withoutWaitingToSettle(actions: () -> Unit) {
        composeRule.mainClock.autoAdvance = false

        try {
            actions()
        } finally {
            composeRule.mainClock.autoAdvance = true
        }

        composeRule.waitForIdle()
    }

    fun advanceFrames(frameCount: Int) {
        repeat(frameCount) {
            composeRule.mainClock.advanceTimeByFrame()
        }
    }

    /** Swipes up across the middle half of the node tagged `scrollingNodeTag`. */
    fun swipeNodeUp(scrollingNodeTag: String, durationMilliseconds: Long) {
        composeRule.onNodeWithTag(scrollingNodeTag).performTouchInput {
            swipeUp(startY = centerY + height / 4f, endY = centerY - height / 4f, durationMillis = durationMilliseconds)
        }
    }

}
