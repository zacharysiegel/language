package ro.zach.language.app

import android.os.Bundle
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

private const val SELECTED_TAB_KEY: String = "selected-tab"
private const val OVERLAY_KEY: String = "overlay"
private const val NEXT_ENTRY_ID_KEY: String = "next-entry-id"
private const val STACK_KEY_PREFIX: String = "stack-"
private const val STACK_COURSE_IDS_KEY: String = "course-ids"
private const val STACK_UNIT_IDS_KEY: String = "unit-ids"
private const val STACK_TABS_KEY: String = "tabs"
private const val STACK_ENTRY_IDS_KEY: String = "entry-ids"

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

/** An opened unit in a tab's stack; `entryId` is unique among the entries of one navigation state and of every state restored from it. */
data class UnitStackEntry(val destination: UnitDestination, val entryId: Long)

/** What going back closed. */
sealed interface BackOutcome {
    data object ClosedOverlay : BackOutcome

    data class LeftUnit(val entry: UnitStackEntry) : BackOutcome

    data object NothingToClose : BackOutcome
}

/** The selected tab, each tab's own stack of opened units, and the overlay shown over them. */
class RootNavigationState private constructor(selectedTab: RootTabKind, overlay: RootOverlayKind, unitStacks: Map<RootTabKind, List<UnitStackEntry>>, nextEntryId: Long) {
    private var selectedTab: RootTabKind by mutableStateOf(selectedTab)
    private var overlay: RootOverlayKind by mutableStateOf(overlay)
    private val unitStacks: Map<RootTabKind, SnapshotStateList<UnitStackEntry>> = RootTabKind.entries.associateWith { tab ->
        mutableStateListOf<UnitStackEntry>().apply { addAll(unitStacks[tab].orEmpty()) }
    }
    private var nextEntryId: Long = nextEntryId

    companion object {
        /** The state `savedState()` wrote, or the Course tab with nothing opened when there is none. */
        fun restoring(savedState: Bundle?): RootNavigationState {
            if (savedState == null) {
                return RootNavigationState(RootTabKind.Course, RootOverlayKind.None, emptyMap(), nextEntryId = 0)
            }

            val selectedTab: RootTabKind = enumByName(RootTabKind.entries, savedState.getString(SELECTED_TAB_KEY)) ?: RootTabKind.Course
            val overlay: RootOverlayKind = enumByName(RootOverlayKind.entries, savedState.getString(OVERLAY_KEY)) ?: RootOverlayKind.None
            val unitStacks: Map<RootTabKind, List<UnitStackEntry>> = RootTabKind.entries.associateWith { tab ->
                readUnitStack(savedState.getBundle(stackKey(tab)))
            }

            return RootNavigationState(selectedTab, overlay, unitStacks, savedState.getLong(NEXT_ENTRY_ID_KEY))
        }
    }

    /** Everything `restoring` needs to rebuild this state after the process is ended. */
    fun savedState(): Bundle {
        val savedState: Bundle = Bundle()

        savedState.putString(SELECTED_TAB_KEY, selectedTab.name)
        savedState.putString(OVERLAY_KEY, overlay.name)
        savedState.putLong(NEXT_ENTRY_ID_KEY, nextEntryId)

        for ((tab, stack) in unitStacks) {
            savedState.putBundle(stackKey(tab), writeUnitStack(stack))
        }

        return savedState
    }

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

private fun stackKey(tab: RootTabKind): String {
    return "$STACK_KEY_PREFIX${tab.name}"
}

private fun writeUnitStack(stack: List<UnitStackEntry>): Bundle {
    val savedStack: Bundle = Bundle()

    savedStack.putStringArray(STACK_COURSE_IDS_KEY, stack.map { entry -> entry.destination.courseId }.toTypedArray())
    savedStack.putStringArray(STACK_UNIT_IDS_KEY, stack.map { entry -> entry.destination.unitId }.toTypedArray())
    savedStack.putStringArray(STACK_TABS_KEY, stack.map { entry -> entry.destination.tab.name }.toTypedArray())
    savedStack.putLongArray(STACK_ENTRY_IDS_KEY, stack.map { entry -> entry.entryId }.toLongArray())

    return savedStack
}

/* An entry whose tab this build no longer has is dropped. */
private fun readUnitStack(savedStack: Bundle?): List<UnitStackEntry> {
    val courseIds: Array<String> = savedStack?.getStringArray(STACK_COURSE_IDS_KEY) ?: emptyArray()
    val unitIds: Array<String> = savedStack?.getStringArray(STACK_UNIT_IDS_KEY) ?: emptyArray()
    val tabNames: Array<String> = savedStack?.getStringArray(STACK_TABS_KEY) ?: emptyArray()
    val entryIds: LongArray = savedStack?.getLongArray(STACK_ENTRY_IDS_KEY) ?: LongArray(0)
    val entryCount: Int = minOf(courseIds.size, unitIds.size, tabNames.size, entryIds.size)

    return (0 until entryCount).mapNotNull { index: Int ->
        val tab: UnitTabKind? = enumByName(UnitTabKind.entries, tabNames[index])

        if (tab == null) {
            null
        } else {
            UnitStackEntry(destination = UnitDestination(courseId = courseIds[index], unitId = unitIds[index], tab = tab), entryId = entryIds[index])
        }
    }
}

private fun <T : Enum<T>> enumByName(entries: List<T>, name: String?): T? {
    return entries.firstOrNull { entry -> entry.name == name }
}
