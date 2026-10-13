package ro.zach.language.unit

import org.junit.Assert.assertEquals
import org.junit.Test
import ro.zach.language.core.GroupedTableFieldRecord
import ro.zach.language.core.GroupedTableRowRecord

class GroupedTableRowTest {
    @Test
    fun groupedTableRowSpokenText_namesEveryCellAfterItsHeaderLeadFirst() {
        val row: GroupedTableRowRecord = GroupedTableRowRecord(
            lead = GroupedTableFieldRecord(header = "Person", value = "eu"),
            fields = listOf(
                GroupedTableFieldRecord(header = "Ser", value = "sou"),
                GroupedTableFieldRecord(header = "Estar", value = "estou"),
            ),
        )

        assertEquals("Person: eu. Ser: sou. Estar: estou.", groupedTableRowSpokenText(row))
    }

    @Test
    fun groupedTableRowSpokenText_keepsTheEndOfACellThatAlreadyEndsASentence() {
        val row: GroupedTableRowRecord = GroupedTableRowRecord(
            lead = GroupedTableFieldRecord(header = "Question", value = "Tudo bem?"),
            fields = listOf(
                GroupedTableFieldRecord(header = "Answer", value = "Tudo bem!"),
                GroupedTableFieldRecord(header = "Note", value = "Informal."),
            ),
        )

        assertEquals("Question: Tudo bem? Answer: Tudo bem! Note: Informal.", groupedTableRowSpokenText(row))
    }

    @Test
    fun groupedTableRowSpokenText_ofARowWithOnlyALeadIsOneSentence() {
        val row: GroupedTableRowRecord = GroupedTableRowRecord(
            lead = GroupedTableFieldRecord(header = "Word", value = "casa"),
            fields = emptyList(),
        )

        assertEquals("Word: casa.", groupedTableRowSpokenText(row))
    }
}
