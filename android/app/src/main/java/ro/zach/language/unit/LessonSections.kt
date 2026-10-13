package ro.zach.language.unit

import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import ro.zach.language.core.DialogueRecord
import ro.zach.language.core.GrammarNoteRecord
import ro.zach.language.core.ReadingRecord
import ro.zach.language.core.UnitLessonRecord
import ro.zach.language.core.UnitOutlineRecord
import ro.zach.language.core.VocabularyItemRecord
import ro.zach.language.design.ColumnScrollTargets
import ro.zach.language.design.PageHeader

@Composable
fun OverviewSection(courseId: String, unit: UnitOutlineRecord, lesson: UnitLessonRecord, modifier: Modifier = Modifier) {
    PageHeader(caption = null, title = "Overview", modifier = modifier)
}

@Composable
fun VocabularySection(items: List<VocabularyItemRecord>, modifier: Modifier = Modifier) {
    PageHeader(caption = null, title = "Vocabulary", modifier = modifier)
}

/** Each dialogue is the target of index rail entry n, through `Modifier.indexRailTarget(railTargets, n)`. */
@Composable
fun DialoguesSection(unit: UnitOutlineRecord, dialogues: List<DialogueRecord>, railTargets: ColumnScrollTargets, modifier: Modifier = Modifier) {
    PageHeader(caption = null, title = "Dialogues", modifier = modifier)
}

/** Each note is the target of index rail entry n, through `Modifier.indexRailTarget(railTargets, n)`. */
@Composable
fun GrammarSection(unit: UnitOutlineRecord, notes: List<GrammarNoteRecord>, railTargets: ColumnScrollTargets, modifier: Modifier = Modifier) {
    PageHeader(caption = null, title = "Principles", modifier = modifier)
}

/** Each reading is the target of index rail entry n, through `Modifier.indexRailTarget(railTargets, n)`. */
@Composable
fun ReadingsSection(unit: UnitOutlineRecord, readings: List<ReadingRecord>, railTargets: ColumnScrollTargets, modifier: Modifier = Modifier) {
    PageHeader(caption = null, title = "Reading", modifier = modifier)
}
