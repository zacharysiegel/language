package ro.zach.language.design

import androidx.compose.foundation.layout.Box
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.alpha
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.LiveRegionMode
import androidx.compose.ui.semantics.hideFromAccessibility
import androidx.compose.ui.semantics.liveRegion
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.stateDescription
import androidx.compose.ui.unit.Dp
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import ro.zach.language.core.pendingIndicationDelayMilliseconds

const val PENDING_STATUS_TEST_TAG: String = "pending-status"

/** How long an operation the learner started runs before its pending state is shown. */
object PendingIndication {
    val delayMilliseconds: Long = pendingIndicationDelayMilliseconds().toLong()

    /** Returns once the delay has passed; cancelling the caller ends the wait. */
    suspend fun waitForDelay() {
        delay(delayMilliseconds)
    }
}

/** Shows `PendingGlyph` in place of its content while `isIndicated`, keeping the content's size. */
@Composable
fun PendingGlyphSwap(
    isIndicated: Boolean,
    glyphColor: Color,
    modifier: Modifier = Modifier,
    content: @Composable () -> Unit,
) {
    Box(modifier = modifier, contentAlignment = Alignment.Center) {
        Box(modifier = Modifier.alpha(if (isIndicated) 0f else 1f)) {
            content()
        }

        if (isIndicated) {
            PendingGlyph(color = glyphColor)
        }
    }
}

/** The static glyph a control shows while what it started is still running. */
@Composable
fun PendingGlyph(color: Color, modifier: Modifier = Modifier, size: Dp = LanguageSpacing.spaceMd) {
    Glyph(glyph = GlyphKind.Pending, color = color, size = size, modifier = modifier)
}

/** Gives a pending control its state description, announced politely once it is indicated. */
fun Modifier.pendingAccessibility(isIndicated: Boolean, value: String): Modifier {
    return semantics {
        stateDescription = if (isIndicated) value else ""
        liveRegion = LiveRegionMode.Polite
    }
}

/** Where a control-started operation is between its start and its end. */
enum class PendingActivityPhase {
    Idle,
    Running,
    Indicated,
}

/** One operation a control starts at a time, indicated once it outlasts `PendingIndication.delayMilliseconds`. */
class PendingActivity(private val scope: CoroutineScope) {
    private var phase: PendingActivityPhase by mutableStateOf(PendingActivityPhase.Idle)

    fun isIndicated(): Boolean {
        return phase == PendingActivityPhase.Indicated
    }

    /** Ignored while an earlier operation is still running. */
    fun run(operation: suspend () -> Unit) {
        if (phase != PendingActivityPhase.Idle) {
            return
        }

        phase = PendingActivityPhase.Running

        val indication: Job = scope.launch {
            PendingIndication.waitForDelay()

            if (phase == PendingActivityPhase.Running) {
                phase = PendingActivityPhase.Indicated
            }
        }

        scope.launch {
            try {
                operation()
            } finally {
                indication.cancel()
                phase = PendingActivityPhase.Idle
            }
        }
    }
}

/** A `PendingActivity` whose operations end when the caller leaves the composition. */
@Composable
fun rememberPendingActivity(): PendingActivity {
    val scope: CoroutineScope = rememberCoroutineScope()

    return remember(scope) { PendingActivity(scope) }
}

/** A loading message which appears only once the load has outlasted `PendingIndication.delayMilliseconds`. */
@Composable
fun PendingStatusMessage(text: String, modifier: Modifier = Modifier) {
    var isShown: Boolean by remember { mutableStateOf(false) }

    LaunchedEffect(Unit) {
        PendingIndication.waitForDelay()
        isShown = true
    }

    StatusMessage(
        text = text,
        modifier = modifier
            .alpha(if (isShown) 1f else 0f)
            .testTag(PENDING_STATUS_TEST_TAG)
            .semantics {
                liveRegion = LiveRegionMode.Polite

                if (!isShown) {
                    hideFromAccessibility()
                }
            },
    )
}
