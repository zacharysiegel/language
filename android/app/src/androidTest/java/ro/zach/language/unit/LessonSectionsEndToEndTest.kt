package ro.zach.language.unit

import androidx.compose.ui.semantics.SemanticsProperties
import androidx.compose.ui.semantics.getOrNull
import androidx.compose.ui.test.SemanticsMatcher
import androidx.compose.ui.test.SemanticsNodeInteraction
import androidx.compose.ui.test.SemanticsNodeInteractionCollection
import androidx.compose.ui.test.assert
import androidx.compose.ui.test.assertContentDescriptionEquals
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.assertIsNotDisplayed
import androidx.compose.ui.test.assertTextEquals
import androidx.compose.ui.test.hasTestTag
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.ComposeTestRule
import androidx.compose.ui.test.junit4.v2.createEmptyComposeRule
import androidx.compose.ui.test.onAllNodesWithTag
import androidx.compose.ui.test.onFirst
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performScrollTo
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
import ro.zach.language.audio.PLAY_BUTTON_PENDING_ACCESSIBILITY_VALUE
import ro.zach.language.testsupport.CourseAppDriver
import ro.zach.language.testsupport.CourseData

/* The unit captioned "A1 · 1". */
private const val FIRST_UNIT_ID: String = "a1-02-sounds-alphabet"
private const val FIRST_UNIT_LABEL_PREFIX: String = "A1.1."
private const val JUMPED_NOTE_NUMBER: Int = 4
private const val MOMENTUM_JUMPED_NOTE_NUMBER: Int = 2
/* Frames for the fling to be under way before the press, and for the jump after it. */
private const val MOMENTUM_ADVANCED_FRAME_COUNT: Int = 3
/* Longer than a fast fling's deceleration. */
private const val MOMENTUM_SETTLE_MILLISECONDS: Long = 3_000
/* The app's idle hide delay and fade, with room to spare. */
private const val INDEX_RAIL_HIDE_ALLOWANCE_MILLISECONDS: Long = 3_000
private const val RAIL_DRAG_HOLD_MILLISECONDS: Long = 100
private const val RAIL_DRAG_STEP_COUNT: Int = 12
private const val GROUPED_TABLE_UNIT_ID: String = "a1-00-how-portuguese-works"
private const val GROUPED_TABLE_NOTE_TITLE: String = "Gender, number and agreement"
private const val STACKED_TABLE_UNIT_ID: String = "a1-09-shopping-colors-prices"
private const val STACKED_TABLE_NOTE_TITLE: String = "Demonstratives: este, esse, aquele"
private const val SECTION_TIMEOUT_MILLISECONDS: Long = 5_000
private const val UNIT_ITEM_LABEL_TEST_TAG: String = "unit-item-label"
private const val GROUPED_TABLE_HEADING_TEST_TAG: String = "grouped-table-heading"
/* The pending indication delay, with room for a slow emulator. */
private const val PENDING_SHOWN_TIMEOUT_MILLISECONDS: Long = 5_000

@RunWith(AndroidJUnit4::class)
class LessonSectionsEndToEndTest {
    @get:Rule
    val composeRule: ComposeTestRule = createEmptyComposeRule()

    private val driver: CourseAppDriver = CourseAppDriver(composeRule)
    private val unitPage: UnitPageDriver = UnitPageDriver(composeRule)

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
    fun principles_notesAreLabelledAndTheIndexRailJumpsToThem() {
        val noteCount: Int = CourseData.readUnit(FIRST_UNIT_ID).getJSONArray("grammar").length()

        openThePrinciples(FIRST_UNIT_ID)

        val itemLabels: SemanticsNodeInteractionCollection = composeRule.onAllNodesWithTag(UNIT_ITEM_LABEL_TEST_TAG)

        assertEquals(noteCount, itemLabels.fetchSemanticsNodes().size)
        itemLabels[0].assertTextEquals("${FIRST_UNIT_LABEL_PREFIX}1")
        itemLabels[1].assertTextEquals("${FIRST_UNIT_LABEL_PREFIX}2")
        unitPage.assertSelected(unitPage.railEntryTag(1))

        val jumpedLabel: SemanticsNodeInteraction = itemLabels[JUMPED_NOTE_NUMBER - 1]

        jumpedLabel.assertIsNotDisplayed()
        composeRule.onNodeWithTag(unitPage.railEntryTag(JUMPED_NOTE_NUMBER))
            .assertContentDescriptionEquals("Go to note $FIRST_UNIT_LABEL_PREFIX$JUMPED_NOTE_NUMBER")

        unitPage.withTheIndexRailShown {
            unitPage.pressRailEntry(JUMPED_NOTE_NUMBER)
        }

        unitPage.assertNearTopOfThePage(jumpedLabel)
        unitPage.assertSelected(unitPage.railEntryTag(JUMPED_NOTE_NUMBER))
        driver.saveScreenshot("17-unit-item-index-jump")

        unitPage.withTheIndexRailShown {
            unitPage.dragAlongTheRail(lastEntryNumber = noteCount, holdMilliseconds = RAIL_DRAG_HOLD_MILLISECONDS, stepCount = RAIL_DRAG_STEP_COUNT)
        }

        itemLabels[noteCount - 1].assertIsDisplayed()
        unitPage.assertSelected(unitPage.railEntryTag(noteCount))
        driver.saveScreenshot("18-unit-item-index-drag")
    }

    @Test
    fun indexRail_isShownOnlyWhileScrollingAndTakesNoWidth() {
        openThePrinciples(FIRST_UNIT_ID)
        unitPage.assertTheIndexRailLetsTouchesThrough(JUMPED_NOTE_NUMBER)
        driver.saveScreenshot("16-unit-item-index-at-rest")

        unitPage.assertTheContentIsInsetEquallyFromBothEdges()

        unitPage.withoutWaitingForThePageToSettle {
            unitPage.swipeThePageUpSlowly()
            unitPage.assertTheIndexRailShadowClearsTheTrailingEdge()
            driver.saveScreenshot("16-unit-item-index-while-scrolling")
        }

        composeRule.mainClock.advanceTimeBy(INDEX_RAIL_HIDE_ALLOWANCE_MILLISECONDS)
        unitPage.assertTheIndexRailLetsTouchesThrough(JUMPED_NOTE_NUMBER)
    }

    @Test
    fun indexRail_jumpsDuringMomentumScrolling() {
        openThePrinciples(FIRST_UNIT_ID)

        val jumpedLabel: SemanticsNodeInteraction = composeRule.onAllNodesWithTag(UNIT_ITEM_LABEL_TEST_TAG)[MOMENTUM_JUMPED_NOTE_NUMBER - 1]

        unitPage.withoutWaitingForThePageToSettle {
            unitPage.flingThePageUp()
            unitPage.advanceFrames(MOMENTUM_ADVANCED_FRAME_COUNT)
            unitPage.pressRailEntry(MOMENTUM_JUMPED_NOTE_NUMBER)
            unitPage.advanceFrames(MOMENTUM_ADVANCED_FRAME_COUNT)
        }

        unitPage.assertSelected(unitPage.railEntryTag(MOMENTUM_JUMPED_NOTE_NUMBER))
        unitPage.assertNearTopOfThePage(jumpedLabel)
        composeRule.mainClock.advanceTimeBy(MOMENTUM_SETTLE_MILLISECONDS)
        unitPage.assertNearTopOfThePage(jumpedLabel)
        driver.saveScreenshot("21-unit-item-index-momentum-jump")
    }

    @Test
    fun principlesTables_areGroupedByTheirFirstColumnAtCompactWidth() {
        val (noteNumber: Int, table: JSONObject) = readGrammarTable(GROUPED_TABLE_UNIT_ID, GROUPED_TABLE_NOTE_TITLE)
        val headers: List<String> = stringList(table.getJSONArray("headers"))
        val firstRow: List<String> = stringList(table.getJSONArray("rows").getJSONArray(0))

        openPrinciplesNote(GROUPED_TABLE_UNIT_ID, noteNumber)

        val firstHeading: SemanticsNodeInteraction = composeRule.onAllNodes(hasTestTag(GROUPED_TABLE_HEADING_TEST_TAG) and hasText(firstRow[0])).onFirst()

        composeRule.waitUntil(SECTION_TIMEOUT_MILLISECONDS) { firstHeadingExists(firstRow[0]) }
        firstHeading.performScrollTo().assertIsDisplayed()
        assertTrue(hasGroupedTableKeyLabel(headers[0]))
        assertTrue(hasGroupedTableKeyLabel(headers[1]))
        groupedTableRow(headers = headers.drop(1), cells = firstRow.drop(1)).performScrollTo().assertIsDisplayed()
        driver.saveScreenshot("19-grouped-table")
    }

    @Test
    fun principlesTables_withoutARepeatedFirstColumnAreStacked() {
        val (noteNumber: Int, table: JSONObject) = readGrammarTable(STACKED_TABLE_UNIT_ID, STACKED_TABLE_NOTE_TITLE)
        val headers: List<String> = stringList(table.getJSONArray("headers"))
        val firstRow: List<String> = stringList(table.getJSONArray("rows").getJSONArray(0))

        openPrinciplesNote(STACKED_TABLE_UNIT_ID, noteNumber)

        groupedTableRow(headers = headers, cells = firstRow).performScrollTo().assertIsDisplayed()
        assertTrue(composeRule.onAllNodesWithTag(GROUPED_TABLE_HEADING_TEST_TAG).fetchSemanticsNodes().isEmpty())
        assertTrue(hasGroupedTableKeyLabel(headers[0]))
        driver.saveScreenshot("20-stacked-table")
    }

    @Test
    fun play_waitingForItsRecordingShowsPendingUntilStopped() {
        driver.relaunchWithUnreachableRepository()
        driver.openUnit(CourseData.PRACTICED_UNIT_ID)
        unitPage.openTab(UnitTabKind.Dialogues)
        driver.waitForTag("play", SECTION_TIMEOUT_MILLISECONDS)

        val playButton: SemanticsNodeInteraction = composeRule.onAllNodesWithTag("play").onFirst()
        val playAllButton: SemanticsNodeInteraction = composeRule.onAllNodesWithTag("play-all").onFirst()
        val stopButton: SemanticsNodeInteraction = composeRule.onAllNodesWithTag("stop").onFirst()

        playButton.performScrollTo().assert(hasStateDescription(""))
        playButton.performClick()
        waitForStateDescription(playButton, PLAY_BUTTON_PENDING_ACCESSIBILITY_VALUE)
        driver.saveScreenshot("14-play-pending")
        stopButton.performScrollTo().performClick()
        waitForStateDescription(playButton, "")

        playAllButton.performScrollTo().performClick()
        waitForStateDescription(playAllButton, PLAY_BUTTON_PENDING_ACCESSIBILITY_VALUE)
        playButton.assert(hasStateDescription(""))
        driver.saveScreenshot("15-play-all-pending")
        stopButton.performClick()
        waitForStateDescription(playAllButton, "")
    }

    private fun hasStateDescription(stateDescription: String): SemanticsMatcher {
        return SemanticsMatcher.expectValue(SemanticsProperties.StateDescription, stateDescription)
    }

    private fun waitForStateDescription(node: SemanticsNodeInteraction, stateDescription: String) {
        composeRule.waitUntil(PENDING_SHOWN_TIMEOUT_MILLISECONDS) {
            node.fetchSemanticsNode().config.getOrNull(SemanticsProperties.StateDescription) == stateDescription
        }
    }

    private fun openThePrinciples(unitId: String) {
        driver.openUnit(unitId)
        unitPage.openTab(UnitTabKind.Grammar)
        driver.waitForTag(UNIT_ITEM_LABEL_TEST_TAG, SECTION_TIMEOUT_MILLISECONDS)
    }

    private fun openPrinciplesNote(unitId: String, noteNumber: Int) {
        openThePrinciples(unitId)

        unitPage.withTheIndexRailShown {
            unitPage.pressRailEntry(noteNumber)
        }
    }

    private fun firstHeadingExists(text: String): Boolean {
        return composeRule.onAllNodes(hasTestTag(GROUPED_TABLE_HEADING_TEST_TAG) and hasText(text)).fetchSemanticsNodes().isNotEmpty()
    }

    private fun hasGroupedTableKeyLabel(header: String): Boolean {
        return composeRule.onAllNodes(hasTestTag("grouped-table-key") and hasText(header.uppercase())).fetchSemanticsNodes().isNotEmpty()
    }

    /** The row of a grouped or stacked table whose spoken text starts with its lead and ends with its last cell, each after its header. */
    private fun groupedTableRow(headers: List<String>, cells: List<String>): SemanticsNodeInteraction {
        val leadPair: String = "${headers.first()}: ${cells.first()}"
        val lastPair: String = "${headers.last()}: ${cells.last()}"
        val matchesTheRow: SemanticsMatcher = SemanticsMatcher("grouped table row starting \"$leadPair\" and holding \"$lastPair\"") { node ->
            val spokenText: String = node.config.getOrNull(SemanticsProperties.ContentDescription)?.firstOrNull() ?: ""

            spokenText.startsWith(leadPair) && spokenText.contains(lastPair)
        }

        return composeRule.onAllNodes(hasTestTag("grouped-table-row") and matchesTheRow).onFirst()
    }

    /** The note titled `noteTitle`, numbered from 1, and its table. */
    private fun readGrammarTable(unitId: String, noteTitle: String): Pair<Int, JSONObject> {
        val notes: JSONArray = CourseData.readUnit(unitId).getJSONArray("grammar")
        val noteIndex: Int = (0 until notes.length()).first { index -> notes.getJSONObject(index).getString("title") == noteTitle }

        return Pair(noteIndex + 1, notes.getJSONObject(noteIndex).getJSONObject("table"))
    }

    private fun stringList(array: JSONArray): List<String> {
        return (0 until array.length()).map { index -> array.getString(index) }
    }
}
