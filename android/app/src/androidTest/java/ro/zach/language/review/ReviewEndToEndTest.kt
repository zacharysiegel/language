package ro.zach.language.review

import androidx.compose.ui.semantics.SemanticsProperties
import androidx.compose.ui.test.assert
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.ComposeTestRule
import androidx.compose.ui.test.junit4.v2.createEmptyComposeRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performScrollTo
import androidx.compose.ui.text.AnnotatedString
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.json.JSONArray
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import ro.zach.language.app.RootTabKind
import ro.zach.language.testsupport.CourseAppDriver
import ro.zach.language.testsupport.CourseData

private const val REVIEW_TIMEOUT_MILLISECONDS: Long = 5_000

@RunWith(AndroidJUnit4::class)
class ReviewEndToEndTest {
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
    fun review_addsWordsAndGradesACard() {
        addThePracticedUnitsWords()
        openTheReviewTab()
        driver.saveScreenshot("07-review-setup")
        tap("start-review")

        tap("show-answer")
        driver.saveScreenshot("08-review-card")
        tap("remembered")

        composeRule.waitUntil(REVIEW_TIMEOUT_MILLISECONDS) { driver.hasNode("flashcard-prompt") || driver.hasNode("review-complete") }
    }

    @Test
    fun review_dueCountAgreesWithTheReviewTabBadge() {
        addThePracticedUnitsWords()
        openTheReviewTab()

        val dueCountBeforeGrading: Int = dueCount()

        assertTrue(dueCountBeforeGrading > 0)
        assertTheReviewTabBadgeShows(dueCountBeforeGrading)

        tap("start-review")
        tap("show-answer")
        tap("remembered")
        tap("quit")

        driver.waitForTag("due-count", REVIEW_TIMEOUT_MILLISECONDS)
        assertEquals(dueCountBeforeGrading - 1, dueCount())
        assertTheReviewTabBadgeShows(dueCountBeforeGrading - 1)
    }

    @Test
    fun review_sourceToTargetDirectionPromptsWithTheTranslation() {
        val vocabularySources: Set<String> = readPracticedUnitsVocabulary("source")

        addThePracticedUnitsWords()
        openTheReviewTab()
        tap("review-direction")
        tapInTheDropDownMenu("direction-source_to_target")
        tap("start-review")

        driver.waitForTag("flashcard-prompt", REVIEW_TIMEOUT_MILLISECONDS)
        assertTrue(nodeText("flashcard-prompt") in vocabularySources)
    }

    private fun addThePracticedUnitsWords() {
        driver.openUnit(CourseData.PRACTICED_UNIT_ID)
        tap("add-words")
    }

    private fun openTheReviewTab() {
        composeRule.onNodeWithTag(RootTabKind.Review.testTag()).performClick()
        driver.waitForTag("due-count", REVIEW_TIMEOUT_MILLISECONDS)
    }

    private fun tap(testTag: String) {
        driver.waitForTag(testTag, REVIEW_TIMEOUT_MILLISECONDS)
        composeRule.onNodeWithTag(testTag).performScrollTo().performClick()
    }

    /* The menu draws in a popup of its own, which does not scroll. */
    private fun tapInTheDropDownMenu(testTag: String) {
        driver.waitForTag(testTag, REVIEW_TIMEOUT_MILLISECONDS)
        composeRule.onNodeWithTag(testTag).performClick()
    }

    private fun dueCount(): Int {
        return nodeText("due-count").toInt()
    }

    private fun assertTheReviewTabBadgeShows(count: Int) {
        composeRule.onNodeWithTag(RootTabKind.Review.testTag()).assert(hasText("$count"))
    }

    private fun nodeText(testTag: String): String {
        return composeRule.onNodeWithTag(testTag).fetchSemanticsNode().config[SemanticsProperties.Text]
            .joinToString(separator = "") { text: AnnotatedString -> text.text }
    }

    private fun readPracticedUnitsVocabulary(field: String): Set<String> {
        val vocabulary: JSONArray = CourseData.readUnit(CourseData.PRACTICED_UNIT_ID).getJSONArray("vocabulary")

        return (0 until vocabulary.length()).map { itemIndex: Int ->
            val item: JSONObject = vocabulary.getJSONObject(itemIndex)

            item.getString(field)
        }.toSet()
    }
}
