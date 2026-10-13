package ro.zach.language.design

import org.junit.Assert.assertEquals
import org.junit.Test

class IndexRailTrackerTest {
    private val targetOwner: Any = Any()

    private fun trackerWithTargetsAt(vararg positions: Float): IndexRailTracker {
        val tracker: IndexRailTracker = IndexRailTracker()

        positions.forEachIndexed { index: Int, position: Float ->
            tracker.recordTargetPosition(entryNumber = index + 1, position = position, owner = targetOwner)
        }

        return tracker
    }

    @Test
    fun currentEntryNumber_isTheLastEntryWhoseTargetPassedTheReadingLine() {
        val tracker: IndexRailTracker = trackerWithTargetsAt(0f, 100f, 200f)

        tracker.recordReadingLinePosition(150f)
        assertEquals(2, tracker.currentEntryNumber())

        tracker.recordReadingLinePosition(200f)
        assertEquals(3, tracker.currentEntryNumber())

        tracker.recordReadingLinePosition(-10f)
        assertEquals(1, tracker.currentEntryNumber())
    }

    @Test
    fun currentEntryNumber_holdsAJumpedEntryUntilTheLearnerScrolls() {
        val tracker: IndexRailTracker = trackerWithTargetsAt(0f, 100f, 200f)

        tracker.recordJump(3)
        tracker.recordReadingLinePosition(120f)
        assertEquals(3, tracker.currentEntryNumber())

        tracker.recordLearnerScroll()
        assertEquals(2, tracker.currentEntryNumber())
    }

    @Test
    fun recordContentChange_releasesAJumpedEntry() {
        val tracker: IndexRailTracker = trackerWithTargetsAt(0f, 100f)

        tracker.recordJump(2)
        tracker.recordContentChange()
        assertEquals(1, tracker.currentEntryNumber())
    }

    @Test
    fun removeTarget_leavesATargetALaterOwnerRecorded() {
        val tracker: IndexRailTracker = trackerWithTargetsAt(0f, 100f)
        val laterOwner: Any = Any()

        tracker.recordTargetPosition(entryNumber = 2, position = 300f, owner = laterOwner)
        tracker.removeTarget(entryNumber = 2, owner = targetOwner)
        assertEquals(300f, tracker.targetPosition(2))

        tracker.removeTarget(entryNumber = 2, owner = laterOwner)
        assertEquals(null, tracker.targetPosition(2))
    }

    @Test
    fun indexRailEntryNumberAt_isTheEntryUnderThePressClampedToTheRail() {
        assertEquals(1, indexRailEntryNumberAt(y = 0f, entryHeight = 20f, entryCount = 5))
        assertEquals(3, indexRailEntryNumberAt(y = 50f, entryHeight = 20f, entryCount = 5))
        assertEquals(1, indexRailEntryNumberAt(y = -5f, entryHeight = 20f, entryCount = 5))
        assertEquals(5, indexRailEntryNumberAt(y = 400f, entryHeight = 20f, entryCount = 5))
    }
}
