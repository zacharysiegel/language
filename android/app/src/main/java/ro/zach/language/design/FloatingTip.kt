package ro.zach.language.design

import androidx.compose.foundation.gestures.awaitEachGesture
import androidx.compose.foundation.gestures.awaitFirstDown
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxScope
import androidx.compose.foundation.layout.offset
import androidx.compose.foundation.layout.width
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.State
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberUpdatedState
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Rect
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.layout.LayoutCoordinates
import androidx.compose.ui.layout.onGloballyPositioned
import androidx.compose.ui.layout.positionInRoot
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.unit.Density
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.IntOffset
import androidx.compose.ui.unit.IntSize
import androidx.compose.ui.unit.toSize
import kotlinx.coroutines.delay
import kotlin.math.max
import kotlin.math.min
import kotlin.math.roundToInt

/** Where a page's floating tip opens: below the one control marked as its anchor. */
class FloatingTipState {
    internal var anchorBoundsInRoot: Rect? by mutableStateOf(null)
    internal var hostPositionInRoot: Offset by mutableStateOf(Offset.Zero)
    internal var hostSize: IntSize by mutableStateOf(IntSize.Zero)
}

@Composable
fun rememberFloatingTipState(): FloatingTipState {
    return remember { FloatingTipState() }
}

/** Marks this control as the one the page's floating tip opens below. */
fun Modifier.floatingTipAnchor(state: FloatingTipState): Modifier {
    return onGloballyPositioned { coordinates: LayoutCoordinates ->
        state.anchorBoundsInRoot = Rect(coordinates.positionInRoot(), coordinates.size.toSize())
    }
}

/**
 * Draws `tip` below the anchor, above `content` and without moving it, while `isShown`. A press anywhere outside the
 * tip dismisses it without reaching the content, as does `autoCloseMilliseconds` passing.
 */
@Composable
fun FloatingTipHost(
    state: FloatingTipState,
    isShown: Boolean,
    autoCloseMilliseconds: Long,
    onDismiss: () -> Unit,
    tip: @Composable () -> Unit,
    modifier: Modifier = Modifier,
    content: @Composable BoxScope.() -> Unit,
) {
    val currentOnDismiss: State<() -> Unit> = rememberUpdatedState(onDismiss)
    val anchorBounds: Rect? = state.anchorBoundsInRoot

    LaunchedEffect(isShown) {
        if (isShown) {
            delay(autoCloseMilliseconds)
            currentOnDismiss.value()
        }
    }

    Box(
        modifier = modifier.onGloballyPositioned { coordinates: LayoutCoordinates ->
            state.hostPositionInRoot = coordinates.positionInRoot()
            state.hostSize = coordinates.size
        },
    ) {
        content()

        if (isShown && anchorBounds != null) {
            Box(
                modifier = Modifier
                    .matchParentSize()
                    .pointerInput(Unit) {
                        awaitEachGesture {
                            awaitFirstDown(requireUnconsumed = false).consume()
                            currentOnDismiss.value()
                        }
                    },
            )

            FloatingTipLayer(state = state, anchorBounds = anchorBounds, tip = tip)
        }
    }
}

@Composable
private fun FloatingTipLayer(state: FloatingTipState, anchorBounds: Rect, tip: @Composable () -> Unit) {
    val density: Density = LocalDensity.current
    val placement: FloatingTipPlacement = floatingTipPlacement(
        density = density,
        anchorBounds = anchorBounds.translate(-state.hostPositionInRoot),
        layerWidth = state.hostSize.width.toFloat(),
    )

    Box(
        modifier = Modifier
            .offset { IntOffset(placement.leading.roundToInt(), placement.top.roundToInt()) }
            .width(placement.width)
            /* The tip takes its own presses, so they never reach the dismissing layer under it. */
            .pointerInput(Unit) {
                awaitEachGesture {
                    awaitFirstDown(requireUnconsumed = false)
                }
            },
    ) {
        tip()
    }
}

private class FloatingTipPlacement(val leading: Float, val top: Float, val width: Dp)

/* Clamped to the layer less `spaceMd` either side and the shadow on the trailing side. */
private fun floatingTipPlacement(density: Density, anchorBounds: Rect, layerWidth: Float): FloatingTipPlacement {
    with(density) {
        val edgeInset: Float = LanguageSpacing.spaceMd.toPx()
        val shadowOffset: Float = LanguageSpacing.boxShadowOffset.toPx()
        val tipWidth: Float = min(LanguageSpacing.floatingTipMaximumWidth.toPx(), layerWidth - 2 * edgeInset - shadowOffset).coerceAtLeast(0f)
        val rightmostTipLeading: Float = layerWidth - edgeInset - shadowOffset - tipWidth
        val tipLeading: Float = max(edgeInset, min(anchorBounds.left, rightmostTipLeading))
        val tipTop: Float = anchorBounds.bottom + LanguageSpacing.spaceXs.toPx()

        return FloatingTipPlacement(leading = tipLeading, top = tipTop, width = tipWidth.toDp())
    }
}
