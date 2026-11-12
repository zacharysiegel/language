package ro.zach.language.testsupport

import android.view.View
import android.view.Window
import androidx.compose.ui.test.junit4.ComposeTestRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.core.view.ViewCompat
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.test.platform.app.InstrumentationRegistry
import androidx.test.runner.lifecycle.ActivityLifecycleMonitorRegistry
import androidx.test.runner.lifecycle.Stage
import ro.zach.language.app.SAFE_DRAWING_AREA_TEST_TAG
import kotlin.math.abs

private const val SOFT_KEYBOARD_TIMEOUT_MILLISECONDS: Long = 5_000
private const val LAYOUT_TOLERANCE_PIXELS: Float = 1f

/** Puts the soft keyboard, which the window opens and closes on a schedule of its own, in a state that no longer moves the app's layout. */
class SoftKeyboardDriver(private val composeRule: ComposeTestRule) {
    /** Closes the soft keyboard, including one still opening, and waits until the app's layout fills the space it left. */
    fun close() {
        InstrumentationRegistry.getInstrumentation().runOnMainSync {
            val window: Window = resumedWindow()
                ?: return@runOnMainSync

            WindowCompat.getInsetsController(window, window.decorView).hide(WindowInsetsCompat.Type.ime())
        }

        composeRule.waitUntil(SOFT_KEYBOARD_TIMEOUT_MILLISECONDS) { isClosedWithTheLayoutFillingTheWindow() }
    }

    private fun isClosedWithTheLayoutFillingTheWindow(): Boolean {
        val expectedSafeDrawingAreaBottom: Float = readSafeDrawingAreaBottomWithTheKeyboardClosed()
            ?: return false
        val safeDrawingAreaBottom: Float = composeRule.onNodeWithTag(SAFE_DRAWING_AREA_TEST_TAG).fetchSemanticsNode().boundsInWindow.bottom

        return abs(safeDrawingAreaBottom - expectedSafeDrawingAreaBottom) < LAYOUT_TOLERANCE_PIXELS
    }

    /** The bottom, in window pixels, of the area the window's insets leave the app, or null while the keyboard is open. */
    private fun readSafeDrawingAreaBottomWithTheKeyboardClosed(): Float? {
        var safeDrawingAreaBottom: Float? = null

        InstrumentationRegistry.getInstrumentation().runOnMainSync {
            val decorView: View = resumedWindow()?.decorView
                ?: return@runOnMainSync
            val insets: WindowInsetsCompat = ViewCompat.getRootWindowInsets(decorView)
                ?: return@runOnMainSync

            if (!insets.isVisible(WindowInsetsCompat.Type.ime())) {
                val safeDrawingTypes: Int = WindowInsetsCompat.Type.systemBars() or WindowInsetsCompat.Type.displayCutout()

                safeDrawingAreaBottom = (decorView.height - insets.getInsets(safeDrawingTypes).bottom).toFloat()
            }
        }

        return safeDrawingAreaBottom
    }

    private fun resumedWindow(): Window? {
        return ActivityLifecycleMonitorRegistry.getInstance().getActivitiesInStage(Stage.RESUMED).firstOrNull()?.window
    }
}
