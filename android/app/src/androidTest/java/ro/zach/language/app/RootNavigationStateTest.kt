package ro.zach.language.app

import android.os.Bundle
import android.os.Parcel
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Test
import org.junit.runner.RunWith
import ro.zach.language.testsupport.CourseData
import ro.zach.language.unit.UnitTabKind

private const val FIRST_UNIT_ID: String = "first-unit"
private const val SECOND_UNIT_ID: String = "second-unit"

/* Bundle round trips run on the device, where Bundle is not a stub. */
@RunWith(AndroidJUnit4::class)
class RootNavigationStateTest {
    @Test
    fun restoring_rebuildsTheTabsStacksAndOverlay() {
        val navigation: RootNavigationState = RootNavigationState.restoring(null)

        navigation.openUnit(UnitDestination(courseId = CourseData.COURSE_ID, unitId = FIRST_UNIT_ID, tab = UnitTabKind.Practice))
        navigation.select(RootTabKind.Glossary)
        navigation.openUnit(UnitDestination(courseId = CourseData.COURSE_ID, unitId = SECOND_UNIT_ID, tab = UnitTabKind.Vocabulary))
        navigation.present(RootOverlayKind.Settings)

        val restored: RootNavigationState = RootNavigationState.restoring(throughParcel(navigation.savedState()))

        assertEquals(RootTabKind.Glossary, restored.selectedTab())
        assertEquals(RootOverlayKind.Settings, restored.overlay())

        for (tab in RootTabKind.entries) {
            assertEquals(navigation.unitStack(tab).toList(), restored.unitStack(tab).toList())
        }
    }

    @Test
    fun restoring_neverReusesARestoredEntryId() {
        val navigation: RootNavigationState = RootNavigationState.restoring(null)

        navigation.openUnit(UnitDestination(courseId = CourseData.COURSE_ID, unitId = FIRST_UNIT_ID, tab = UnitTabKind.Practice))

        val restored: RootNavigationState = RootNavigationState.restoring(throughParcel(navigation.savedState()))
        val restoredEntryIds: List<Long> = restored.unitStack(RootTabKind.Course).map { entry -> entry.entryId }

        restored.goBack()
        restored.openUnit(UnitDestination(courseId = CourseData.COURSE_ID, unitId = SECOND_UNIT_ID, tab = UnitTabKind.Overview))

        val openedEntry: UnitStackEntry = restored.unitStack(RootTabKind.Course).last()

        assertFalse(openedEntry.entryId in restoredEntryIds)
    }

    @Test
    fun restoring_withoutSavedStateStartsOnTheCourseTab() {
        val navigation: RootNavigationState = RootNavigationState.restoring(null)

        assertEquals(RootTabKind.Course, navigation.selectedTab())
        assertEquals(RootOverlayKind.None, navigation.overlay())
        assertFalse(navigation.canGoBack())
    }

    /** Writes the bundle to a parcel and reads it back, as the system does when the process is ended. */
    private fun throughParcel(savedState: Bundle): Bundle {
        val parcel: Parcel = Parcel.obtain()

        try {
            parcel.writeBundle(savedState)
            parcel.setDataPosition(0)

            return parcel.readBundle(javaClass.classLoader) ?: error("the parcel held no bundle")
        } finally {
            parcel.recycle()
        }
    }

}
