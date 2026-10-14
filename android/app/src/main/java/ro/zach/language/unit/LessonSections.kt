package ro.zach.language.unit

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.key
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.text.font.FontWeight
import ro.zach.language.audio.AudioClip
import ro.zach.language.audio.PlayAllButton
import ro.zach.language.audio.PlayButton
import ro.zach.language.core.DialogueRecord
import ro.zach.language.core.ExerciseRecord
import ro.zach.language.core.GrammarNoteRecord
import ro.zach.language.core.GrammarTableRecord
import ro.zach.language.core.PhrasePairRecord
import ro.zach.language.core.QueuedCardsRecord
import ro.zach.language.core.ReadingRecord
import ro.zach.language.core.UnitLessonRecord
import ro.zach.language.core.UnitOutlineRecord
import ro.zach.language.core.VocabularyItemRecord
import ro.zach.language.core.VoiceChoiceRecord
import ro.zach.language.design.ColumnScrollTargets
import ro.zach.language.design.HairlineRule
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType
import ro.zach.language.design.Paragraphs
import ro.zach.language.design.SquareTextButton
import ro.zach.language.design.Tag
import ro.zach.language.design.indexRailTarget
import ro.zach.language.design.panel
import ro.zach.language.practice.ExerciseSession
import ro.zach.language.practice.ExerciseSessionExitKind
import ro.zach.language.progressdata.LocalProgressStore
import ro.zach.language.progressdata.ProgressStore

@Composable
fun OverviewSection(courseId: String, unit: UnitOutlineRecord, lesson: UnitLessonRecord, modifier: Modifier = Modifier) {
    Column(modifier = modifier.fillMaxWidth(), verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceLg)) {
        Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
            LanguageText(text = "You will learn to", style = LanguageType.titleText)

            for (objective in unit.objectives) {
                ObjectiveLine(objective = objective)
            }

            AddWordsToReviewDeck(courseId = courseId, unit = unit)
        }

        Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
            LanguageText(text = lesson.culture.title, style = LanguageType.titleText)
            Paragraphs(text = lesson.culture.text)
        }
    }
}

@Composable
private fun ObjectiveLine(objective: String) {
    Row(horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
        LanguageText(text = "–", style = LanguageType.bodyText, modifier = Modifier.alignByBaseline())
        LanguageText(text = objective, style = LanguageType.bodyText, modifier = Modifier.weight(1f).alignByBaseline())
    }
}

@Composable
private fun AddWordsToReviewDeck(courseId: String, unit: UnitOutlineRecord) {
    val progressStore: ProgressStore = LocalProgressStore.current
    var addedMessage: String? by rememberSaveable { mutableStateOf(null) }
    val vocabularyCount: Int = unit.vocabulary.size
    val shownAddedMessage: String? = addedMessage

    SquareTextButton(
        text = "Add the $vocabularyCount words to my review deck",
        onClick = {
            val queued: QueuedCardsRecord? = progressStore.queueUnitVocabulary(courseId, unit.id, vocabularyCount)

            if (queued != null) {
                addedMessage = "${queued.addedCount} new word(s) added and ${queued.madeDueCount} word(s) queued for review again."
            }
        },
        modifier = Modifier.testTag("add-words"),
    )

    if (shownAddedMessage != null) {
        LanguageText(text = shownAddedMessage, style = LanguageType.captionText, color = LanguageColors.inkMuted)
    }
}

@Composable
fun VocabularySection(items: List<VocabularyItemRecord>, modifier: Modifier = Modifier) {
    Column(modifier = modifier.fillMaxWidth()) {
        HairlineRule()

        for (item in items) {
            key(item.cardId) {
                VocabularyRow(item = item)
                HairlineRule()
            }
        }
    }
}

@Composable
fun VocabularyRow(item: VocabularyItemRecord, modifier: Modifier = Modifier) {
    val partOfSpeech: String? = item.partOfSpeech
    val literal: String? = item.sourceLiteral
    val note: String? = item.note

    Column(
        modifier = modifier.fillMaxWidth().padding(vertical = LanguageSpacing.spaceSm + LanguageSpacing.spaceXs),
        verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs),
    ) {
        Row(horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm), verticalAlignment = Alignment.CenterVertically) {
            PlayButton(clip = AudioClip(clipKey = item.clipKey, text = item.target))
            LanguageText(text = item.target, style = LanguageType.targetText.copy(fontWeight = FontWeight.Bold))

            if (partOfSpeech != null) {
                Tag(text = partOfSpeech)
            }
        }

        LanguageText(text = item.source, style = LanguageType.bodyText)

        if (literal != null) {
            LiteralGloss(literal = literal)
        }

        VocabularyExample(example = item.example, modifier = Modifier.padding(top = LanguageSpacing.spaceXs))

        if (note != null) {
            LanguageText(text = note, style = LanguageType.captionText, color = LanguageColors.inkMuted)
        }
    }
}

/** A vocabulary item's example sentence, playable, over its translation. */
@Composable
fun VocabularyExample(example: PhrasePairRecord, modifier: Modifier = Modifier) {
    Row(
        modifier = modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm),
        verticalAlignment = Alignment.Top,
    ) {
        PlayButton(clip = AudioClip(clipKey = example.clipKey, text = example.target))

        Column(modifier = Modifier.weight(1f), verticalArrangement = Arrangement.spacedBy(LanguageSpacing.phraseLineGap)) {
            LanguageText(text = example.target, style = LanguageType.bodyText)
            LanguageText(text = example.source, style = LanguageType.captionText, color = LanguageColors.inkMuted)
        }
    }
}

/** Each dialogue is the target of index rail entry n, through `Modifier.indexRailTarget(railTargets, n)`. */
@Composable
fun DialoguesSection(unit: UnitOutlineRecord, dialogues: List<DialogueRecord>, railTargets: ColumnScrollTargets, modifier: Modifier = Modifier) {
    Column(modifier = modifier.fillMaxWidth(), verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd)) {
        TranslationToggle()

        dialogues.forEachIndexed { dialogueIndex: Int, dialogue: DialogueRecord ->
            val itemNumber: Int = dialogueIndex + 1

            key(dialogue.id) {
                DialoguePanel(
                    dialogue = dialogue,
                    itemLabel = unit.itemLabel(itemNumber),
                    modifier = Modifier.indexRailTarget(railTargets, itemNumber),
                )
            }
        }
    }
}

@Composable
private fun DialoguePanel(dialogue: DialogueRecord, itemLabel: String, modifier: Modifier) {
    val voiceChoice: VoiceChoiceRecord = dialogueVoiceChoice(dialogue)
    val clips: List<AudioClip> = dialogue.lines.map { line -> AudioClip(clipKey = line.phrase.clipKey, text = line.phrase.target, voiceChoice = voiceChoice) }

    Column(modifier = modifier.panel(), verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
        UnitItemLabel(text = itemLabel)
        LanguageText(text = dialogue.title, style = LanguageType.titleText)
        LanguageText(text = dialogue.context, style = LanguageType.captionText, color = LanguageColors.inkMuted)
        PlayAllButton(clips = clips)

        for (line in dialogue.lines) {
            PhraseRow(pair = line.phrase, speaker = line.speaker, voiceChoice = voiceChoice)
        }
    }
}

/** The voice choice of each of the dialogue's lines, which plays the whole dialogue from one recording set. */
fun dialogueVoiceChoice(dialogue: DialogueRecord): VoiceChoiceRecord {
    return VoiceChoiceRecord.WholeDialogue(lineTexts = dialogue.lines.map { line -> line.phrase.target })
}

/** Each note is the target of index rail entry n, through `Modifier.indexRailTarget(railTargets, n)`. */
@Composable
fun GrammarSection(unit: UnitOutlineRecord, notes: List<GrammarNoteRecord>, railTargets: ColumnScrollTargets, modifier: Modifier = Modifier) {
    Column(modifier = modifier.fillMaxWidth(), verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd)) {
        TranslationToggle()

        notes.forEachIndexed { noteIndex: Int, note: GrammarNoteRecord ->
            val itemNumber: Int = noteIndex + 1

            GrammarNotePanel(
                note = note,
                itemLabel = unit.itemLabel(itemNumber),
                modifier = Modifier.indexRailTarget(railTargets, itemNumber),
            )
        }
    }
}

@Composable
private fun GrammarNotePanel(note: GrammarNoteRecord, itemLabel: String, modifier: Modifier) {
    val table: GrammarTableRecord? = note.table

    Column(modifier = modifier.panel(), verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
        UnitItemLabel(text = itemLabel)
        LanguageText(text = note.title, style = LanguageType.titleText)
        Paragraphs(text = note.explanation)

        if (table != null) {
            GrammarTableView(table = table)
        }

        for (example in note.examples) {
            PhraseRow(pair = example)
        }
    }
}

/** Each reading is the target of index rail entry n, through `Modifier.indexRailTarget(railTargets, n)`. */
@Composable
fun ReadingsSection(unit: UnitOutlineRecord, readings: List<ReadingRecord>, railTargets: ColumnScrollTargets, modifier: Modifier = Modifier) {
    Column(modifier = modifier.fillMaxWidth(), verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd)) {
        TranslationToggle()

        readings.forEachIndexed { readingIndex: Int, reading: ReadingRecord ->
            val itemNumber: Int = readingIndex + 1

            key(reading.id) {
                ReadingPanel(
                    reading = reading,
                    itemLabel = unit.itemLabel(itemNumber),
                    modifier = Modifier.indexRailTarget(railTargets, itemNumber),
                )
            }
        }
    }
}

@Composable
fun ReadingPanel(reading: ReadingRecord, itemLabel: String, modifier: Modifier = Modifier) {
    var areQuestionsOpen: Boolean by rememberSaveable(reading.id) { mutableStateOf(false) }
    val clips: List<AudioClip> = reading.paragraphs.map { paragraph -> AudioClip(clipKey = paragraph.clipKey, text = paragraph.target) }

    Column(modifier = modifier.panel(), verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
        UnitItemLabel(text = itemLabel)
        LanguageText(text = reading.title, style = LanguageType.titleText)
        PlayAllButton(clips = clips)

        for (paragraph in reading.paragraphs) {
            PhraseRow(pair = paragraph)
        }

        if (areQuestionsOpen) {
            ReadingQuestionsSession(
                title = "${reading.title}: questions",
                questions = reading.questions,
                onExit = { areQuestionsOpen = false },
            )
        } else {
            SquareTextButton(
                text = "Answer the comprehension questions",
                onClick = { areQuestionsOpen = true },
                isPrimary = true,
                modifier = Modifier.testTag("comprehension-questions"),
            )
        }
    }
}

/** A reading's comprehension questions as an exercise session that can only be quit. */
@Composable
private fun ReadingQuestionsSession(title: String, questions: List<ExerciseRecord>, onExit: () -> Unit, modifier: Modifier = Modifier) {
    ExerciseSession(
        title = title,
        exercises = questions,
        answeredResults = emptyList(),
        exitKind = ExerciseSessionExitKind.Quit,
        onAnswer = {},
        onComplete = {},
        onExit = onExit,
        modifier = modifier,
    )
}
