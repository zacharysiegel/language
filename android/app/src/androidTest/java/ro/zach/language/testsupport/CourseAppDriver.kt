package ro.zach.language.testsupport

import android.content.Context
import android.content.Intent
import android.graphics.Bitmap
import androidx.compose.ui.semantics.SemanticsProperties
import androidx.compose.ui.test.SemanticsMatcher
import androidx.compose.ui.test.assert
import androidx.compose.ui.test.assertTextEquals
import androidx.compose.ui.test.junit4.ComposeTestRule
import androidx.compose.ui.test.onAllNodesWithTag
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performScrollTo
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import ro.zach.language.app.MainActivity
import ro.zach.language.coursedata.CoreEnvironment
import ro.zach.language.design.PAGE_HEADER_TITLE_TEST_TAG
import java.io.File
import java.io.FileOutputStream
import java.util.UUID

private const val FIRST_SCREEN_TIMEOUT_MILLISECONDS: Long = 20_000
private const val UNIT_OPEN_TIMEOUT_MILLISECONDS: Long = 15_000
private const val SCREENSHOT_DIRECTORY_NAME: String = "language-android-screenshots"
private const val SCREENSHOT_QUALITY: Int = 100

/** Drives the app as a learner would, each test with a progress file of its own. */
class CourseAppDriver(private val composeRule: ComposeTestRule) {
    private var scenario: ActivityScenario<MainActivity>? = null
    private var launchExtras: Map<String, String> = mapOf(CoreEnvironment.PROGRESS_FILE_EXTRA to newProgressFileName())

    fun launch() {
        scenario = ActivityScenario.launch(launchIntent())
    }

    fun relaunchKeepingProgress() {
        close()
        launch()
        chooseTheCourseIfTheCourseListOpens()
    }

    /** Starts the app again with a cache of its own and neither the origin nor the static repository reachable. */
    fun relaunchWithUnreachableRepository() {
        close()
        launchExtras = mapOf(
            CoreEnvironment.PROGRESS_FILE_EXTRA to newProgressFileName(),
            CoreEnvironment.CACHE_DIRECTORY_EXTRA to "ui-test-cache-${UUID.randomUUID()}",
            CoreEnvironment.ORIGIN_EXTRA to UnreachableOrigins.WAITING_ORIGIN,
            CoreEnvironment.REPOSITORY_BASE_URL_EXTRA to UnreachableOrigins.WAITING_ORIGIN,
        )
        launch()
        chooseTheCourseIfTheCourseListOpens()
    }

    fun close() {
        scenario?.close()
        scenario = null
    }

    /** Opens the course under test when the app starts on the course list. */
    fun chooseTheCourseIfTheCourseListOpens() {
        val courseChoiceTag: String = "course-${CourseData.COURSE_ID}"

        composeRule.waitUntil(FIRST_SCREEN_TIMEOUT_MILLISECONDS) {
            hasNode(courseChoiceTag) || hasNode("courses")
        }

        if (hasNode(courseChoiceTag)) {
            composeRule.onNodeWithTag(courseChoiceTag).performClick()
        }
    }

    fun openUnit(unitId: String) {
        val unitTag: String = "unit-$unitId"

        waitForTag(unitTag, FIRST_SCREEN_TIMEOUT_MILLISECONDS)
        composeRule.onNodeWithTag(unitTag).performScrollTo().performClick()
        waitForTag("tab-overview", UNIT_OPEN_TIMEOUT_MILLISECONDS)
    }

    fun waitForTag(testTag: String, timeoutMilliseconds: Long = FIRST_SCREEN_TIMEOUT_MILLISECONDS) {
        composeRule.waitUntil(timeoutMilliseconds) { hasNode(testTag) }
    }

    /** Asserts that the page shown names itself `title` in a heading. */
    fun assertPageHeading(title: String) {
        composeRule.onNodeWithTag(PAGE_HEADER_TITLE_TEST_TAG)
            .assertTextEquals(title)
            .assert(SemanticsMatcher.keyIsDefined(SemanticsProperties.Heading))
    }

    fun hasNode(testTag: String): Boolean {
        return composeRule.onAllNodesWithTag(testTag).fetchSemanticsNodes().isNotEmpty()
    }

    /** Saves the screen as `<name>.png` in the app's external files, which `adb pull` reads without root. */
    fun saveScreenshot(name: String) {
        composeRule.waitForIdle()

        val targetContext: Context = InstrumentationRegistry.getInstrumentation().targetContext
        val directory: File = File(targetContext.getExternalFilesDir(null), SCREENSHOT_DIRECTORY_NAME)
        val screenshot: Bitmap = InstrumentationRegistry.getInstrumentation().uiAutomation.takeScreenshot()
            ?: return

        directory.mkdirs()
        FileOutputStream(File(directory, "$name.png")).use { output -> screenshot.compress(Bitmap.CompressFormat.PNG, SCREENSHOT_QUALITY, output) }
    }

    private fun launchIntent(): Intent {
        val targetContext: Context = InstrumentationRegistry.getInstrumentation().targetContext
        val intent: Intent = Intent(targetContext, MainActivity::class.java)

        for ((name, value) in launchExtras) {
            intent.putExtra(name, value)
        }

        return intent
    }

    private fun newProgressFileName(): String {
        return "ui-test-progress-${UUID.randomUUID()}.json"
    }
}
