package ro.zach.language.glossary

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test
import ro.zach.language.core.PhrasePairRecord
import ro.zach.language.core.UnitOutlineRecord
import ro.zach.language.core.VocabularyItemRecord
import ro.zach.language.design.IndexRailEntry

class GlossaryRailTest {
    @Test
    fun dividingByLevels_listsEachLevelAtItsFirstShownEntry() {
        val shownEntries: List<GlossaryEntry> = listOf(
            entry(cardId = "a1-first", unitId = "a1-01", levelCode = "A1", order = 1u),
            entry(cardId = "a1-second", unitId = "a1-02", levelCode = "A1", order = 2u),
            entry(cardId = "a2-first", unitId = "a2-01", levelCode = "A2", order = 15u),
        )

        val glossaryRail: GlossaryRail = GlossaryRail.dividing(shownEntries, GlossaryRailKind.Levels)

        assertEquals(
            listOf(
                IndexRailEntry(visibleText = "A1", accessibilityLabel = "Go to level A1"),
                IndexRailEntry(visibleText = "A2", accessibilityLabel = "Go to level A2"),
            ),
            glossaryRail.entries,
        )
        assertEquals(listOf("a1-first", "a2-first"), firstEntryIds(glossaryRail))
        assertEquals(listOf(2, 1), glossaryRail.runs.map { run: GlossaryRun -> run.entries.size })
        assertEquals(listOf(1, 2), glossaryRail.runs.map { run: GlossaryRun -> run.indexRailEntryNumber })
    }

    @Test
    fun dividingByUnits_listsEachUnitByItsNumberAtItsFirstShownEntry() {
        val shownEntries: List<GlossaryEntry> = listOf(
            entry(cardId = "unit-25-first", unitId = "a2-11", levelCode = "A2", order = 25u),
            entry(cardId = "unit-25-second", unitId = "a2-11", levelCode = "A2", order = 25u),
            entry(cardId = "unit-26-first", unitId = "a2-12", levelCode = "A2", order = 26u),
        )

        val glossaryRail: GlossaryRail = GlossaryRail.dividing(shownEntries, GlossaryRailKind.Units)

        assertEquals(
            listOf(
                IndexRailEntry(visibleText = "25", accessibilityLabel = "Go to unit A2 25"),
                IndexRailEntry(visibleText = "26", accessibilityLabel = "Go to unit A2 26"),
            ),
            glossaryRail.entries,
        )
        assertEquals(listOf("unit-25-first", "unit-26-first"), firstEntryIds(glossaryRail))
        assertEquals(listOf(2, 1), glossaryRail.runs.map { run: GlossaryRun -> run.entries.size })
        assertEquals(listOf(1, 2), glossaryRail.runs.map { run: GlossaryRun -> run.indexRailEntryNumber })
    }

    @Test
    fun dividingByUnits_numbersOnlyTheFirstRunOfAUnitShownTwice() {
        val shownEntries: List<GlossaryEntry> = listOf(
            entry(cardId = "unit-1-first", unitId = "a1-01", levelCode = "A1", order = 1u),
            entry(cardId = "unit-2-first", unitId = "a1-02", levelCode = "A1", order = 2u),
            entry(cardId = "unit-1-second", unitId = "a1-01", levelCode = "A1", order = 1u),
        )

        val glossaryRail: GlossaryRail = GlossaryRail.dividing(shownEntries, GlossaryRailKind.Units)

        assertEquals(listOf("1", "2"), glossaryRail.entries.map { railEntry: IndexRailEntry -> railEntry.visibleText })
        assertEquals(listOf(1, 2, null), glossaryRail.runs.map { run: GlossaryRun -> run.indexRailEntryNumber })
        assertEquals(mapOf(1 to 0, 2 to 1), glossaryRail.firstEntryIndexByEntryNumber())
    }

    @Test
    fun dividing_noShownEntriesHasNoRunsOrRailEntries() {
        val glossaryRail: GlossaryRail = GlossaryRail.dividing(emptyList(), GlossaryRailKind.Units)

        assertTrue(glossaryRail.runs.isEmpty())
        assertTrue(glossaryRail.entries.isEmpty())
    }

    @Test
    fun forLevelFilter_listsLevelsForAllLevelsAndUnitsForOneLevel() {
        assertEquals(GlossaryRailKind.Levels, GlossaryRailKind.forLevelFilter(null))
        assertEquals(GlossaryRailKind.Units, GlossaryRailKind.forLevelFilter("B1"))
    }

    @Test
    fun firstEntryIndexByEntryNumber_isWhereEachRailEntrysFirstRunStarts() {
        val shownEntries: List<GlossaryEntry> = listOf(
            entry(cardId = "a1-first", unitId = "a1-01", levelCode = "A1", order = 1u),
            entry(cardId = "a1-second", unitId = "a1-02", levelCode = "A1", order = 2u),
            entry(cardId = "a2-first", unitId = "a2-01", levelCode = "A2", order = 15u),
        )

        val glossaryRail: GlossaryRail = GlossaryRail.dividing(shownEntries, GlossaryRailKind.Units)

        assertEquals(mapOf(1 to 0, 2 to 1, 3 to 2), glossaryRail.firstEntryIndexByEntryNumber())
    }

    @Test
    fun matchingGlossaryEntries_keepsTheShownLevelsEntriesContainingTheQuery() {
        val entries: List<GlossaryEntry> = listOf(
            entry(cardId = "a1-ola", unitId = "a1-01", levelCode = "A1", order = 1u, searchKeys = listOf("ola", "hello")),
            entry(cardId = "a2-sonho", unitId = "a2-01", levelCode = "A2", order = 15u, searchKeys = listOf("o sonho", "dream")),
            entry(cardId = "a2-ola", unitId = "a2-02", levelCode = "A2", order = 16u, searchKeys = listOf("ola de novo", "hello again")),
        )

        assertEquals(listOf("a1-ola", "a2-sonho", "a2-ola"), entryIds(matchingGlossaryEntries(entries, query = "", levelFilter = null)))
        assertEquals(listOf("a1-ola", "a2-ola"), entryIds(matchingGlossaryEntries(entries, query = "hello", levelFilter = null)))
        assertEquals(listOf("a2-ola"), entryIds(matchingGlossaryEntries(entries, query = "ola", levelFilter = "A2")))
        assertEquals(emptyList<String>(), entryIds(matchingGlossaryEntries(entries, query = "dream", levelFilter = "A1")))
    }

    private fun firstEntryIds(glossaryRail: GlossaryRail): List<String> {
        return glossaryRail.runs.map { run: GlossaryRun -> run.entries.first().id }
    }

    private fun entryIds(entries: List<GlossaryEntry>): List<String> {
        return entries.map { entry: GlossaryEntry -> entry.id }
    }

    private fun entry(cardId: String, unitId: String, levelCode: String, order: UInt, searchKeys: List<String> = emptyList()): GlossaryEntry {
        val item: VocabularyItemRecord = VocabularyItemRecord(
            cardId = cardId,
            target = cardId,
            source = cardId,
            sourceLiteral = null,
            partOfSpeech = null,
            note = null,
            clipKey = null,
            example = PhrasePairRecord(target = "", source = "", sourceLiteral = null, clipKey = null),
        )
        val unit: UnitOutlineRecord = UnitOutlineRecord(id = unitId, levelCode = levelCode, order = order, title = unitId, objectives = emptyList(), vocabulary = listOf(item))

        return GlossaryEntry(id = cardId, unit = unit, item = item, searchKeys = searchKeys)
    }
}
