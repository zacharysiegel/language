package ro.zach.language.glossary

import ro.zach.language.core.CourseOutlineRecord
import ro.zach.language.core.UnitOutlineRecord
import ro.zach.language.core.VocabularyItemRecord
import ro.zach.language.core.searchKey

/** A vocabulary item with its unit and the keys it is searched by. */
data class GlossaryEntry(
    val id: String,
    val unit: UnitOutlineRecord,
    val item: VocabularyItemRecord,
    val searchKeys: List<String>,
) {
    fun unitNumberText(): String {
        return "${unit.order}"
    }

    fun unitLabel(): String {
        return "${unit.levelCode} ${unitNumberText()}"
    }

    companion object {
        /** Every vocabulary item of the course, in unit order. */
        fun indexing(outline: CourseOutlineRecord): List<GlossaryEntry> {
            return outline.units.flatMap { unit: UnitOutlineRecord ->
                unit.vocabulary.map { item: VocabularyItemRecord ->
                    GlossaryEntry(id = item.cardId, unit = unit, item = item, searchKeys = listOf(searchKey(item.target), searchKey(item.source)))
                }
            }
        }
    }
}

/** The entries of the shown level, `null` for all levels, with a search key containing `query`, already folded with `searchKey`. */
fun matchingGlossaryEntries(entries: List<GlossaryEntry>, query: String, levelFilter: String?): List<GlossaryEntry> {
    return entries.filter { entry: GlossaryEntry ->
        val levelMatches: Boolean = levelFilter == null || entry.unit.levelCode == levelFilter
        val textMatches: Boolean = query.isEmpty() || entry.searchKeys.any { key: String -> key.contains(query) }

        levelMatches && textMatches
    }
}
