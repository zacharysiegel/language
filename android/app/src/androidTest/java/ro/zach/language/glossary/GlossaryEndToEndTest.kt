package ro.zach.language.glossary

import androidx.compose.ui.geometry.Rect
import androidx.compose.ui.semantics.SemanticsNode
import androidx.compose.ui.test.assert
import androidx.compose.ui.test.assertIsSelected
import androidx.compose.ui.test.hasContentDescription
import androidx.compose.ui.test.junit4.ComposeTestRule
import androidx.compose.ui.test.junit4.v2.createEmptyComposeRule
import androidx.compose.ui.test.onAllNodesWithText
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performTouchInput
import androidx.compose.ui.test.swipeUp
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.json.JSONArray
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import ro.zach.language.testsupport.CourseAppDriver
import ro.zach.language.testsupport.CourseData

private const val JUMPED_LEVEL_CODE: String = "B1"
private const val JUMPED_UNIT_ID: String = "b1-05-imperfect-subjunctive-conditionals"
private const val GLOSSARY_LIST_TAG: String = "glossary-entries"
private const val RAIL_CHANGE_TIMEOUT_MILLISECONDS: Long = 5_000
/* Slow enough to scroll without a long fling. */
private const val RAIL_SHOWING_SWIPE_DURATION_MILLISECONDS: Long = 600
/* Longer than the rail's fade in, shorter than its idle hide delay. */
private const val RAIL_SHOWN_ALLOWANCE_MILLISECONDS: Long = 400
private const val JUMP_SETTLE_MILLISECONDS: Long = 200
/* The top quarter of the list counts as its top. */
private const val NEAR_TOP_PROPORTION: Float = 0.25f

@RunWith(AndroidJUnit4::class)
class GlossaryEndToEndTest {
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
    fun glossaryLevelRail_jumpsToTheFirstWordOfALevel() {
        val firstLevelUnitId: String = readFirstUnitId(JUMPED_LEVEL_CODE)
        val firstLevelUnitNumber: Int = readUnitNumber(firstLevelUnitId)
        val firstLevelWord: String = readFirstVocabularyTarget(firstLevelUnitId)
        val levelEntryTag: String = "glossary-level-index-$JUMPED_LEVEL_CODE"
        val firstLevelUnitEntryTag: String = "glossary-unit-index-$firstLevelUnitNumber"

        openTheGlossary()

        driver.waitForTag(levelEntryTag, RAIL_CHANGE_TIMEOUT_MILLISECONDS)
        composeRule.onNodeWithTag(levelEntryTag).assert(hasContentDescription("Go to level $JUMPED_LEVEL_CODE"))

        composeRule.onNodeWithTag("glossary-level-filter-$JUMPED_LEVEL_CODE").performClick()
        driver.waitForTag(firstLevelUnitEntryTag, RAIL_CHANGE_TIMEOUT_MILLISECONDS)
        waitUntilGone(levelEntryTag)

        composeRule.onNodeWithTag("glossary-level-filter-all").performClick()
        driver.waitForTag(levelEntryTag, RAIL_CHANGE_TIMEOUT_MILLISECONDS)
        waitUntilGone(firstLevelUnitEntryTag)

        withTheIndexRailShown {
            composeRule.onNodeWithTag(levelEntryTag).performClick()
        }

        driver.saveScreenshot("22-glossary-level-index-jump")
        assertTrue(isNearTopOfTheList(firstLevelWord))
        composeRule.onNodeWithTag(levelEntryTag).assertIsSelected()
    }

    @Test
    fun glossaryUnitRail_ofOneLevelJumpsToTheFirstWordOfAUnit() {
        val unitNumber: Int = readUnitNumber(JUMPED_UNIT_ID)
        val firstUnitWord: String = readFirstVocabularyTarget(JUMPED_UNIT_ID)
        val unitEntryTag: String = "glossary-unit-index-$unitNumber"

        openTheGlossary()
        composeRule.onNodeWithTag("glossary-level-filter-$JUMPED_LEVEL_CODE").performClick()

        driver.waitForTag(unitEntryTag, RAIL_CHANGE_TIMEOUT_MILLISECONDS)
        composeRule.onNodeWithTag(unitEntryTag).assert(hasContentDescription("Go to unit $JUMPED_LEVEL_CODE $unitNumber"))
        assertFalse(isNearTopOfTheList(firstUnitWord))

        withTheIndexRailShown {
            composeRule.onNodeWithTag(unitEntryTag).performClick()
        }

        driver.saveScreenshot("23-glossary-unit-index-jump")
        assertTrue(isNearTopOfTheList(firstUnitWord))
        composeRule.onNodeWithTag(unitEntryTag).assertIsSelected()
    }

    private fun openTheGlossary() {
        driver.waitForTag("unit-${CourseData.PRACTICED_UNIT_ID}")
        composeRule.onNodeWithTag("root-tab-glossary").performClick()
        driver.waitForTag(GLOSSARY_LIST_TAG)
    }

    /* The rail shows only while the list scrolls, so the frames are advanced by hand until `railActions` has run. */
    private fun withTheIndexRailShown(railActions: () -> Unit) {
        composeRule.mainClock.autoAdvance = false

        try {
            composeRule.onNodeWithTag(GLOSSARY_LIST_TAG).performTouchInput { swipeUp(durationMillis = RAIL_SHOWING_SWIPE_DURATION_MILLISECONDS) }
            composeRule.mainClock.advanceTimeBy(RAIL_SHOWN_ALLOWANCE_MILLISECONDS)
            railActions()
            composeRule.mainClock.advanceTimeBy(JUMP_SETTLE_MILLISECONDS)
        } finally {
            composeRule.mainClock.autoAdvance = true
        }

        composeRule.waitForIdle()
    }

    private fun waitUntilGone(testTag: String) {
        composeRule.waitUntil(RAIL_CHANGE_TIMEOUT_MILLISECONDS) { !driver.hasNode(testTag) }
    }

    private fun isNearTopOfTheList(text: String): Boolean {
        val listBounds: Rect = composeRule.onNodeWithTag(GLOSSARY_LIST_TAG).fetchSemanticsNode().boundsInRoot
        val nearTopLimit: Float = listBounds.top + listBounds.height * NEAR_TOP_PROPORTION
        val textNodes: List<SemanticsNode> = composeRule.onAllNodesWithText(text).fetchSemanticsNodes()

        return textNodes.any { node: SemanticsNode -> node.boundsInRoot.top >= listBounds.top && node.boundsInRoot.top < nearTopLimit }
    }

    /** The first unit of the level in curriculum order. */
    private fun readFirstUnitId(levelCode: String): String {
        val curriculumUnits: List<JSONObject> = readCurriculumUnits()
        val levelUnit: JSONObject = curriculumUnits.first { curriculumUnit: JSONObject -> curriculumUnit.getString("level") == levelCode }

        return levelUnit.getString("id")
    }

    /** The number the glossary shows in the unit's label. */
    private fun readUnitNumber(unitId: String): Int {
        val curriculumUnits: List<JSONObject> = readCurriculumUnits()
        val curriculumUnit: JSONObject = curriculumUnits.first { candidateUnit: JSONObject -> candidateUnit.getString("id") == unitId }

        return curriculumUnit.getInt("order")
    }

    private fun readFirstVocabularyTarget(unitId: String): String {
        val vocabulary: JSONArray = CourseData.readUnit(unitId).getJSONArray("vocabulary")

        return vocabulary.getJSONObject(0).getString("target")
    }

    private fun readCurriculumUnits(): List<JSONObject> {
        val units: JSONArray = CourseData.readCurriculum().getJSONArray("units")

        return (0 until units.length()).map { unitIndex: Int -> units.getJSONObject(unitIndex) }
    }
}
