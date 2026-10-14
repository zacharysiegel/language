package ro.zach.language.settings

import androidx.compose.ui.semantics.SemanticsNode
import androidx.compose.ui.semantics.SemanticsProperties
import androidx.compose.ui.semantics.getOrNull
import androidx.compose.ui.test.SemanticsMatcher
import androidx.compose.ui.test.assert
import androidx.compose.ui.test.junit4.ComposeTestRule
import androidx.compose.ui.test.junit4.v2.createEmptyComposeRule
import androidx.compose.ui.test.onAllNodesWithTag
import androidx.compose.ui.test.onAllNodesWithText
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performScrollTo
import androidx.compose.ui.text.AnnotatedString
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.After
import org.junit.Assert.assertFalse
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import ro.zach.language.coursedata.CONTENT_UP_TO_DATE_MESSAGE
import ro.zach.language.coursedata.SERVER_UNREACHABLE_MESSAGE
import ro.zach.language.testsupport.CourseAppDriver
import ro.zach.language.testsupport.CourseData
import ro.zach.language.testsupport.UnreachableOrigins

private const val SETTINGS_OPEN_TIMEOUT_MILLISECONDS: Long = 10_000
/* The pending indication delay, with room for a slow emulator. */
private const val PENDING_SHOWN_TIMEOUT_MILLISECONDS: Long = 5_000
private const val UPDATE_CHECK_TIMEOUT_MILLISECONDS: Long = 15_000

@RunWith(AndroidJUnit4::class)
class SettingsEndToEndTest {
    @get:Rule
    val composeRule: ComposeTestRule = createEmptyComposeRule()

    private val driver: CourseAppDriver = CourseAppDriver(composeRule)

    @Before
    fun launch() {
        driver.launch()
        driver.chooseTheCourseIfTheCourseListOpens()
    }

    @After
    fun close() {
        driver.close()
    }

    @Test
    fun updateCheck_showsItsPendingStateWhileTheRepositoryCannotBeReached() {
        driver.relaunchWithUnreachableRepository()
        openTheSettings()
        composeRule.waitUntil(SETTINGS_OPEN_TIMEOUT_MILLISECONDS) { hasText("The copy included in the app") }

        checkForUpdatedContent()
        composeRule.waitUntil(PENDING_SHOWN_TIMEOUT_MILLISECONDS) { hasText(UPDATE_CHECKING_MESSAGE) }
        composeRule.onNodeWithTag("check-for-updated-content").assert(SemanticsMatcher.expectValue(SemanticsProperties.StateDescription, "Checking"))
        driver.saveScreenshot("24-settings-update-check-pending")
    }

    @Test
    fun updateCheck_saysTheServerCouldNotBeReachedWhenTheRepositoryRefuses() {
        driver.relaunchWithUnreachableRepository(UnreachableOrigins.REFUSING_ORIGIN)
        openTheSettings()

        checkForUpdatedContent()
        composeRule.waitUntil(UPDATE_CHECK_TIMEOUT_MILLISECONDS) { updateCheckMessage() == SERVER_UNREACHABLE_MESSAGE }
    }

    /* The first check may download content published after the cached copy; the second finds it current. */
    @Test
    fun updateCheck_saysTheCourseContentIsUpToDate() {
        openTheSettings()
        assertFalse(driver.hasNode(UPDATE_CHECK_MESSAGE_TEST_TAG))

        checkForUpdatedContent()
        driver.waitForTag(UPDATE_CHECK_MESSAGE_TEST_TAG, UPDATE_CHECK_TIMEOUT_MILLISECONDS)

        checkForUpdatedContent()
        composeRule.waitUntil(UPDATE_CHECK_TIMEOUT_MILLISECONDS) { updateCheckMessage() == CONTENT_UP_TO_DATE_MESSAGE }
        driver.saveScreenshot("25-settings-update-check-up-to-date")
    }

    private fun openTheSettings() {
        driver.waitForTag("unit-${CourseData.PRACTICED_UNIT_ID}")
        composeRule.onNodeWithTag("settings").performClick()
        driver.waitForTag("publication-label", SETTINGS_OPEN_TIMEOUT_MILLISECONDS)
    }

    private fun checkForUpdatedContent() {
        composeRule.onNodeWithTag("check-for-updated-content").performScrollTo().performClick()
    }

    private fun updateCheckMessage(): String? {
        val messageNodes: List<SemanticsNode> = composeRule.onAllNodesWithTag(UPDATE_CHECK_MESSAGE_TEST_TAG).fetchSemanticsNodes()
        val texts: List<AnnotatedString> = messageNodes.firstOrNull()?.config?.getOrNull(SemanticsProperties.Text)
            ?: return null

        return texts.joinToString(separator = "") { text: AnnotatedString -> text.text }
    }

    private fun hasText(text: String): Boolean {
        return composeRule.onAllNodesWithText(text).fetchSemanticsNodes().isNotEmpty()
    }
}
