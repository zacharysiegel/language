package ro.zach.language.design

import androidx.compose.foundation.ScrollState
import androidx.compose.foundation.gestures.stopScroll
import androidx.compose.foundation.interaction.DragInteraction
import androidx.compose.foundation.interaction.Interaction
import androidx.compose.foundation.lazy.LazyListItemInfo
import androidx.compose.foundation.lazy.LazyListLayoutInfo
import androidx.compose.foundation.lazy.LazyListState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.runtime.snapshotFlow
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.layout.LayoutCoordinates
import androidx.compose.ui.layout.onGloballyPositioned
import androidx.compose.ui.node.GlobalPositionAwareModifierNode
import androidx.compose.ui.node.ModifierNodeElement
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.unit.Dp
import kotlin.math.roundToInt

const val FIRST_INDEX_RAIL_ENTRY_NUMBER: Int = 1

/* The line, below the top of the visible page, that a target's top passes when the learner reaches it. */
private val readingLineOffset: Dp = LanguageSpacing.spaceLg

/**
 * What an index rail follows of its page's scrolling. Positions are along the page in one unit per page: content
 * offsets for a scrolling column, item indexes for a lazy list.
 */
class IndexRailTracker {
    private var shownEntryNumber: Int by mutableIntStateOf(FIRST_INDEX_RAIL_ENTRY_NUMBER)
    private val targets: MutableMap<Int, IndexRailTarget> = HashMap()
    private var readingLinePosition: Float = 0f
    /* Held until the learner scrolls, since a target near the end cannot scroll up to the top. */
    private var jumpedEntryNumber: Int? = null

    /** The entry the learner is reading: the one jumped to, or else the last whose target has passed the reading line. */
    fun currentEntryNumber(): Int {
        return shownEntryNumber
    }

    fun targetPosition(entryNumber: Int): Float? {
        return targets[entryNumber]?.position
    }

    /** Records where `owner`'s target of `entryNumber` is; a later owner of the same entry replaces it. */
    fun recordTargetPosition(entryNumber: Int, position: Float, owner: Any) {
        targets[entryNumber] = IndexRailTarget(position = position, owner = owner)
        updateShownEntryNumber()
    }

    /** Forgets the target of `entryNumber` when `owner` still holds it. */
    fun removeTarget(entryNumber: Int, owner: Any) {
        if (targets[entryNumber]?.owner === owner) {
            targets.remove(entryNumber)
            updateShownEntryNumber()
        }
    }

    /** Replaces every target. */
    fun recordTargetPositions(positions: Map<Int, Float>) {
        targets.clear()

        for ((entryNumber, position) in positions) {
            targets[entryNumber] = IndexRailTarget(position = position, owner = this)
        }

        updateShownEntryNumber()
    }

    fun recordReadingLinePosition(position: Float) {
        readingLinePosition = position
        updateShownEntryNumber()
    }

    fun recordLearnerScroll() {
        jumpedEntryNumber = null
        updateShownEntryNumber()
    }

    /** Records that the targets were replaced, so an entry number may name a different target. */
    fun recordContentChange() {
        jumpedEntryNumber = null
        updateShownEntryNumber()
    }

    fun recordJump(entryNumber: Int) {
        jumpedEntryNumber = entryNumber
        updateShownEntryNumber()
    }

    private fun updateShownEntryNumber() {
        val entryNumber: Int = jumpedEntryNumber ?: scrolledEntryNumber()

        if (entryNumber != shownEntryNumber) {
            shownEntryNumber = entryNumber
        }
    }

    private fun scrolledEntryNumber(): Int {
        val passedEntryNumbers: List<Int> = targets
            .filterValues { target -> target.position <= readingLinePosition }
            .keys
            .toList()

        return passedEntryNumbers.maxOrNull() ?: FIRST_INDEX_RAIL_ENTRY_NUMBER
    }
}

private class IndexRailTarget(val position: Float, val owner: Any)

/** The page an index rail jumps along. */
interface IndexRailScrollTarget {
    fun isScrollInProgress(): Boolean

    /** Stops any fling, then scrolls the entry's target to the top without animation. */
    suspend fun jumpTo(entryNumber: Int)
}

/** The targets of a `verticalScroll` column, each marked with `Modifier.indexRailTarget`. */
class ColumnScrollTargets(val scrollState: ScrollState, val tracker: IndexRailTracker) : IndexRailScrollTarget {
    private var contentCoordinates: LayoutCoordinates? = null
    private val targetCoordinates: MutableMap<Any, ColumnTargetCoordinates> = HashMap()

    override fun isScrollInProgress(): Boolean {
        return scrollState.isScrollInProgress
    }

    override suspend fun jumpTo(entryNumber: Int) {
        tracker.recordJump(entryNumber)
        scrollState.stopScroll()

        val position: Float = tracker.targetPosition(entryNumber)
            ?: return

        scrollState.scrollTo(position.roundToInt())
    }

    internal fun recordContentCoordinates(coordinates: LayoutCoordinates) {
        contentCoordinates = coordinates

        for ((owner, target) in targetCoordinates) {
            recordTargetPosition(owner, target)
        }
    }

    internal fun recordTargetCoordinates(owner: Any, entryNumber: Int, coordinates: LayoutCoordinates) {
        val target: ColumnTargetCoordinates = ColumnTargetCoordinates(entryNumber = entryNumber, coordinates = coordinates)

        targetCoordinates[owner] = target
        recordTargetPosition(owner, target)
    }

    internal fun removeTarget(owner: Any, entryNumber: Int) {
        targetCoordinates.remove(owner)
        tracker.removeTarget(entryNumber, owner)
    }

    private fun recordTargetPosition(owner: Any, target: ColumnTargetCoordinates) {
        val content: LayoutCoordinates = contentCoordinates
            ?: return

        if (!content.isAttached || !target.coordinates.isAttached) {
            return
        }

        val contentTop: Float = content.localPositionOf(target.coordinates, Offset.Zero).y

        tracker.recordTargetPosition(target.entryNumber, contentTop, owner)
    }
}

private class ColumnTargetCoordinates(val entryNumber: Int, val coordinates: LayoutCoordinates)

@Composable
fun rememberColumnScrollTargets(scrollState: ScrollState): ColumnScrollTargets {
    return remember(scrollState) { ColumnScrollTargets(scrollState = scrollState, tracker = IndexRailTracker()) }
}

/** Marks this view, the direct child of the `verticalScroll`, as the content holding `targets`. */
fun Modifier.indexRailContent(targets: ColumnScrollTargets): Modifier {
    return onGloballyPositioned { coordinates: LayoutCoordinates -> targets.recordContentCoordinates(coordinates) }
}

/** Makes this view the target of its index rail entry. */
fun Modifier.indexRailTarget(targets: ColumnScrollTargets, entryNumber: Int): Modifier {
    return this then IndexRailTargetElement(targets = targets, entryNumber = entryNumber)
}

private class IndexRailTargetElement(val targets: ColumnScrollTargets, val entryNumber: Int) : ModifierNodeElement<IndexRailTargetNode>() {
    override fun create(): IndexRailTargetNode {
        return IndexRailTargetNode(targets = targets, entryNumber = entryNumber)
    }

    override fun update(node: IndexRailTargetNode) {
        node.update(targets = targets, entryNumber = entryNumber)
    }

    override fun equals(other: Any?): Boolean {
        return other is IndexRailTargetElement && other.targets === targets && other.entryNumber == entryNumber
    }

    override fun hashCode(): Int {
        return 31 * System.identityHashCode(targets) + entryNumber
    }
}

private class IndexRailTargetNode(private var targets: ColumnScrollTargets, private var entryNumber: Int) : Modifier.Node(), GlobalPositionAwareModifierNode {
    fun update(targets: ColumnScrollTargets, entryNumber: Int) {
        this.targets.removeTarget(owner = this, entryNumber = this.entryNumber)
        this.targets = targets
        this.entryNumber = entryNumber
    }

    override fun onGloballyPositioned(coordinates: LayoutCoordinates) {
        targets.recordTargetCoordinates(owner = this, entryNumber = entryNumber, coordinates = coordinates)
    }

    override fun onDetach() {
        targets.removeTarget(owner = this, entryNumber = entryNumber)
    }
}

/** Reports a scrolling column's reading line and the learner's drags to its tracker. */
@Composable
fun ColumnIndexRailTracking(targets: ColumnScrollTargets) {
    val readingLineOffsetPixels: Float = with(LocalDensity.current) { readingLineOffset.toPx() }

    LaunchedEffect(targets, readingLineOffsetPixels) {
        snapshotFlow { targets.scrollState.value }.collect { scrolledOffset: Int ->
            targets.tracker.recordReadingLinePosition(scrolledOffset + readingLineOffsetPixels)
        }
    }

    LaunchedEffect(targets) {
        targets.scrollState.interactionSource.interactions.collect { interaction: Interaction ->
            if (interaction is DragInteraction.Start) {
                targets.tracker.recordLearnerScroll()
            }
        }
    }
}

/** The targets of a `LazyColumn`, each the index of the item an entry jumps to. */
class LazyListScrollTargets(val listState: LazyListState, val tracker: IndexRailTracker) : IndexRailScrollTarget {
    override fun isScrollInProgress(): Boolean {
        return listState.isScrollInProgress
    }

    override suspend fun jumpTo(entryNumber: Int) {
        tracker.recordJump(entryNumber)
        listState.stopScroll()

        val itemIndex: Float = tracker.targetPosition(entryNumber)
            ?: return

        listState.scrollToItem(itemIndex.roundToInt())
    }
}

@Composable
fun rememberLazyListScrollTargets(listState: LazyListState): LazyListScrollTargets {
    return remember(listState) { LazyListScrollTargets(listState = listState, tracker = IndexRailTracker()) }
}

/** Reports a lazy list's targets, the item at its reading line and the learner's drags to its tracker. */
@Composable
fun LazyListIndexRailTracking(targets: LazyListScrollTargets, itemIndexByEntryNumber: Map<Int, Int>) {
    val readingLineOffsetPixels: Float = with(LocalDensity.current) { readingLineOffset.toPx() }

    LaunchedEffect(targets, itemIndexByEntryNumber) {
        val positions: Map<Int, Float> = itemIndexByEntryNumber.mapValues { (_, itemIndex) -> itemIndex.toFloat() }

        targets.tracker.recordTargetPositions(positions)
    }

    LaunchedEffect(targets, readingLineOffsetPixels) {
        snapshotFlow { readingLineItemIndex(targets.listState.layoutInfo, targets.listState.firstVisibleItemIndex, readingLineOffsetPixels) }
            .collect { itemIndex: Int -> targets.tracker.recordReadingLinePosition(itemIndex.toFloat()) }
    }

    LaunchedEffect(targets) {
        targets.listState.interactionSource.interactions.collect { interaction: Interaction ->
            if (interaction is DragInteraction.Start) {
                targets.tracker.recordLearnerScroll()
            }
        }
    }
}

private fun readingLineItemIndex(layoutInfo: LazyListLayoutInfo, firstVisibleItemIndex: Int, readingLineOffsetPixels: Float): Int {
    val itemAtReadingLine: LazyListItemInfo? = layoutInfo.visibleItemsInfo.lastOrNull { item -> item.offset <= readingLineOffsetPixels }

    return itemAtReadingLine?.index ?: firstVisibleItemIndex
}
