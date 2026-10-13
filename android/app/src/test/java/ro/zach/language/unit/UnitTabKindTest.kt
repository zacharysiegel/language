package ro.zach.language.unit

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test
import ro.zach.language.core.CultureNoteRecord
import ro.zach.language.core.ReadingRecord
import ro.zach.language.core.UnitLessonRecord

class UnitTabKindTest {
    private fun lessonWithReadings(readings: List<ReadingRecord>): UnitLessonRecord {
        return UnitLessonRecord(
            dialogues = emptyList(),
            grammar = emptyList(),
            culture = CultureNoteRecord(title = "Culture", text = "Text"),
            readings = readings,
            exercises = emptyList(),
        )
    }

    @Test
    fun availableTabs_leaveOutReadingForAUnitWithoutReadings() {
        val lesson: UnitLessonRecord = lessonWithReadings(emptyList())

        assertEquals(
            listOf(UnitTabKind.Overview, UnitTabKind.Grammar, UnitTabKind.Dialogues, UnitTabKind.Vocabulary, UnitTabKind.Practice),
            UnitTabKind.availableTabs(lesson),
        )
    }

    @Test
    fun availableTabs_includeReadingForAUnitWithReadings() {
        val reading: ReadingRecord = ReadingRecord(id = "reading-1", title = "Reading", paragraphs = emptyList(), questions = emptyList())
        val lesson: UnitLessonRecord = lessonWithReadings(listOf(reading, reading.copy(id = "reading-2")))

        assertEquals(UnitTabKind.entries.toList(), UnitTabKind.availableTabs(lesson))
        assertEquals(2, UnitTabKind.Readings.indexedItemCount(lesson))
    }

    @Test
    fun indexedItemNoun_namesTheBoxesOfTheTabsMadeOfBoxes() {
        assertEquals("note", UnitTabKind.Grammar.indexedItemNoun())
        assertEquals("dialogue", UnitTabKind.Dialogues.indexedItemNoun())
        assertEquals("reading", UnitTabKind.Readings.indexedItemNoun())
        assertNull(UnitTabKind.Overview.indexedItemNoun())
        assertNull(UnitTabKind.Vocabulary.indexedItemNoun())
        assertNull(UnitTabKind.Practice.indexedItemNoun())
    }
}
