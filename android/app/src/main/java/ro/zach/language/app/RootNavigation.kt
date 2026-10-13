package ro.zach.language.app

import androidx.compose.runtime.ProvidableCompositionLocal
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.compose.runtime.snapshots.SnapshotStateList
import androidx.compose.runtime.staticCompositionLocalOf
import ro.zach.language.design.GlyphKind
import ro.zach.language.unit.UnitTabKind

val LocalRootNavigation: ProvidableCompositionLocal<RootNavigationState> = staticCompositionLocalOf { error("no RootNavigationState provided") }

enum class RootTabKind {
    Course,
    Review,
    Glossary,
    Progress;

    fun label(): String {
        return when (this) {
            Course -> "Course"
            Review -> "Review"
            Glossary -> "Glossary"
            Progress -> "Progress"
        }
    }

    fun glyph(): GlyphKind {
        return when (this) {
            Course -> GlyphKind.Course
            Review -> GlyphKind.Review
            Glossary -> GlyphKind.Glossary
            Progress -> GlyphKind.Progress
        }
    }

    fun testTag(): String {
        return "root-tab-${name.lowercase()}"
    }
}

/** A unit of a course, and the tab to open it on. */
data class UnitDestination(val courseId: String, val unitId: String, val tab: UnitTabKind)

/** A full-screen layer over the tabs, with its own Done. */
enum class RootOverlayKind {
    None,
    Settings,
    Courses,
}

/** An opened unit in a tab's stack; `entryId` is unique among every entry opened this launch. */
data class UnitStackEntry(val destination: UnitDestination, val entryId: Long)

/** What going back closed. */
sealed interface BackOutcome {
    data object ClosedOverlay : BackOutcome

    data class LeftUnit(val entry: UnitStackEntry) : BackOutcome

    data object NothingToClose : BackOutcome
}

/** The selected tab, each tab's own stack of opened units, and the overlay shown over them. */
class RootNavigationState {
    private var selectedTab: RootTabKind by mutableStateOf(RootTabKind.Course)
    private var overlay: RootOverlayKind by mutableStateOf(RootOverlayKind.None)
    private val unitStacks: Map<RootTabKind, SnapshotStateList<UnitStackEntry>> = RootTabKind.entries.associateWith { mutableStateListOf() }
    private var nextEntryId: Long = 0

    fun selectedTab(): RootTabKind {
        return selectedTab
    }

    fun select(tab: RootTabKind) {
        selectedTab = tab
    }

    fun overlay(): RootOverlayKind {
        return overlay
    }

    fun present(overlay: RootOverlayKind) {
        this.overlay = overlay
    }

    fun dismissOverlay() {
        overlay = RootOverlayKind.None
    }

    fun unitStack(tab: RootTabKind): List<UnitStackEntry> {
        return unitStacks.getValue(tab)
    }

    /** Opens the unit on top of the selected tab's stack. */
    fun openUnit(destination: UnitDestination) {
        unitStacks.getValue(selectedTab).add(UnitStackEntry(destination = destination, entryId = nextEntryId))
        nextEntryId += 1
    }

    /** Closes the overlay, else the selected tab's top unit. */
    fun goBack(): BackOutcome {
        if (overlay != RootOverlayKind.None) {
            overlay = RootOverlayKind.None
            return BackOutcome.ClosedOverlay
        }

        val stack: SnapshotStateList<UnitStackEntry> = unitStacks.getValue(selectedTab)

        if (stack.isEmpty()) {
            return BackOutcome.NothingToClose
        }

        return BackOutcome.LeftUnit(stack.removeAt(stack.lastIndex))
    }

    fun canGoBack(): Boolean {
        return overlay != RootOverlayKind.None || unitStacks.getValue(selectedTab).isNotEmpty()
    }
}
