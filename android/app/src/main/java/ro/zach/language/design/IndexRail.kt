package ro.zach.language.design

import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.animation.core.tween
import androidx.compose.foundation.background
import androidx.compose.foundation.focusable
import androidx.compose.foundation.gestures.awaitEachGesture
import androidx.compose.foundation.gestures.awaitFirstDown
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxWithConstraints
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.State
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.rememberUpdatedState
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.alpha
import androidx.compose.ui.focus.FocusState
import androidx.compose.ui.focus.onFocusChanged
import androidx.compose.ui.hapticfeedback.HapticFeedback
import androidx.compose.ui.hapticfeedback.HapticFeedbackType
import androidx.compose.ui.input.pointer.PointerInputChange
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.platform.LocalHapticFeedback
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.CustomAccessibilityAction
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.customActions
import androidx.compose.ui.semantics.onClick
import androidx.compose.ui.semantics.selected
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.min
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import ro.zach.language.core.unitItemIndexMinimumItemCount

/* About as long as the system scroll indicator stays after scrolling stops. */
private const val INDEX_RAIL_IDLE_HIDE_DELAY_MILLISECONDS: Long = 1500
private const val INDEX_RAIL_FADE_DURATION_MILLISECONDS: Int = 200
private const val NEXT_ENTRY_ACTION_LABEL: String = "Next entry"
private const val PREVIOUS_ENTRY_ACTION_LABEL: String = "Previous entry"

/** An entry of an index rail, numbered from 1 in rail order. */
data class IndexRailEntry(
    /** Short enough to fit the rail's width; unique within the rail. */
    val visibleText: String,
    val accessibilityLabel: String,
)

object IndexRailLimits {
    val minimumEntryCount: Int = unitItemIndexMinimumItemCount().toInt()
}

/** The entry number, from 1, under a press `y` from the rail's top. */
fun indexRailEntryNumberAt(y: Float, entryHeight: Float, entryCount: Int): Int {
    val entryIndex: Int = kotlin.math.floor(y / entryHeight).toInt()

    return entryIndex.coerceIn(0, entryCount - 1) + FIRST_INDEX_RAIL_ENTRY_NUMBER
}

/**
 * A column of short entries over the page's trailing edge; pressing or dragging along it jumps to an entry's target.
 * It shows nothing with fewer than `IndexRailLimits.minimumEntryCount` entries. It is shown while the page scrolls or
 * the rail is in use, and hidden after an idle delay; hidden, it takes no presses.
 */
@Composable
fun IndexRail(
    entries: List<IndexRailEntry>,
    railAccessibilityLabel: String,
    /** Each entry's test tag is this followed by `-` and its visible text. */
    railTestTag: String,
    tracker: IndexRailTracker,
    scrollTarget: IndexRailScrollTarget,
    modifier: Modifier = Modifier,
) {
    if (entries.size < IndexRailLimits.minimumEntryCount) {
        return
    }

    var isShown: Boolean by remember { mutableStateOf(false) }
    var draggedEntryNumber: Int? by remember { mutableStateOf(null) }
    var focusedEntryNumber: Int? by remember { mutableStateOf(null) }
    val isInUse: Boolean = scrollTarget.isScrollInProgress() || draggedEntryNumber != null || focusedEntryNumber != null
    val shownAlpha: Float by animateFloatAsState(
        targetValue = if (isShown) 1f else 0f,
        animationSpec = tween(durationMillis = INDEX_RAIL_FADE_DURATION_MILLISECONDS),
        label = "index rail",
    )
    val scope: CoroutineScope = rememberCoroutineScope()
    val hapticFeedback: HapticFeedback = LocalHapticFeedback.current
    val jump: (Int) -> Unit = { entryNumber: Int -> scope.launch { scrollTarget.jumpTo(entryNumber) } }

    LaunchedEffect(isInUse) {
        if (isInUse) {
            isShown = true
        } else {
            delay(INDEX_RAIL_IDLE_HIDE_DELAY_MILLISECONDS)
            isShown = false
        }
    }

    BoxWithConstraints(
        modifier = modifier
            .fillMaxHeight()
            .padding(end = LanguageSpacing.indexRailEdgeInset)
            .width(LanguageSpacing.indexRailWidth)
            .alpha(shownAlpha)
            .semantics { contentDescription = railAccessibilityLabel }
            .testTag(railTestTag),
        contentAlignment = Alignment.Center,
    ) {
        val entryHeight: Dp = min(LanguageSpacing.indexRailEntryMaximumHeight, maxHeight / entries.size)
        val entryHeightPixels: Float = with(LocalDensity.current) { entryHeight.toPx() }
        val currentJump: State<(Int) -> Unit> = rememberUpdatedState(jump)
        val pressHandling: Modifier = if (isShown) {
            Modifier.pointerInput(entries.size, entryHeightPixels) {
                awaitEachGesture {
                    val down: PointerInputChange = awaitFirstDown(requireUnconsumed = false)

                    down.consume()
                    draggedEntryNumber = trackDrag(
                        firstY = down.position.y,
                        entryHeight = entryHeightPixels,
                        entryCount = entries.size,
                        onEntry = { entryNumber: Int ->
                            hapticFeedback.performHapticFeedback(HapticFeedbackType.SegmentTick)
                            draggedEntryNumber = entryNumber
                            currentJump.value(entryNumber)
                        },
                    )
                }
            }
        } else {
            Modifier
        }

        Column(modifier = Modifier.shadowedBox(shadowOffset = LanguageSpacing.smallBoxShadowOffset).then(pressHandling)) {
            entries.forEachIndexed { entryIndex: Int, entry: IndexRailEntry ->
                val entryNumber: Int = entryIndex + FIRST_INDEX_RAIL_ENTRY_NUMBER

                IndexRailEntryView(
                    entry = entry,
                    entryNumber = entryNumber,
                    entryCount = entries.size,
                    isCurrent = entryNumber == tracker.currentEntryNumber(),
                    height = entryHeight,
                    testTag = "$railTestTag-${entry.visibleText}",
                    onJump = jump,
                    onFocusChange = { isFocused: Boolean ->
                        if (isFocused) {
                            focusedEntryNumber = entryNumber
                        } else if (focusedEntryNumber == entryNumber) {
                            focusedEntryNumber = null
                        }
                    },
                )
            }
        }
    }
}

/* A drag of no distance is a press, so one gesture serves both. Returns `null` once the finger lifts. */
private suspend fun androidx.compose.ui.input.pointer.AwaitPointerEventScope.trackDrag(
    firstY: Float,
    entryHeight: Float,
    entryCount: Int,
    onEntry: (Int) -> Unit,
): Int? {
    var lastEntryNumber: Int = indexRailEntryNumberAt(firstY, entryHeight, entryCount)

    onEntry(lastEntryNumber)

    while (true) {
        val change: PointerInputChange = awaitPointerEvent().changes.firstOrNull()
            ?: return null

        if (!change.pressed) {
            return null
        }

        change.consume()

        val entryNumber: Int = indexRailEntryNumberAt(change.position.y, entryHeight, entryCount)

        if (entryNumber != lastEntryNumber) {
            lastEntryNumber = entryNumber
            onEntry(entryNumber)
        }
    }
}

@Composable
private fun IndexRailEntryView(
    entry: IndexRailEntry,
    entryNumber: Int,
    entryCount: Int,
    isCurrent: Boolean,
    height: Dp,
    testTag: String,
    onJump: (Int) -> Unit,
    onFocusChange: (Boolean) -> Unit,
) {
    Box(
        modifier = Modifier
            .width(LanguageSpacing.indexRailWidth)
            .height(height)
            .background(if (isCurrent) LanguageColors.accentTint else LanguageColors.paper)
            .onFocusChanged { focusState: FocusState -> onFocusChange(focusState.isFocused) }
            .focusable()
            .semantics {
                contentDescription = entry.accessibilityLabel
                selected = isCurrent
                onClick {
                    onJump(entryNumber)
                    true
                }
                customActions = listOf(
                    CustomAccessibilityAction(NEXT_ENTRY_ACTION_LABEL) {
                        onJump(minOf(entryNumber + 1, entryCount))
                        true
                    },
                    CustomAccessibilityAction(PREVIOUS_ENTRY_ACTION_LABEL) {
                        onJump(maxOf(entryNumber - 1, FIRST_INDEX_RAIL_ENTRY_NUMBER))
                        true
                    },
                )
            }
            .testTag(testTag),
        contentAlignment = Alignment.Center,
    ) {
        LanguageText(
            text = entry.visibleText,
            style = LanguageType.indexEntryText.copy(fontWeight = if (isCurrent) FontWeight.Bold else FontWeight.Normal),
            color = if (isCurrent) LanguageColors.ink else LanguageColors.inkMuted,
            maxLines = 1,
        )
    }
}
