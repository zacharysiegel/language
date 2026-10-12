import SwiftUI

struct OverviewSection: View {
    let courseId: String
    let unit: UnitOutlineRecord
    let lesson: UnitLessonRecord

    @Environment(ProgressStore.self) private var progressStore
    @State private var addedMessage: String?

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceLg) {
            VStack(alignment: .leading, spacing: .spaceSm) {
                Text("You will learn to").font(.titleText)

                ForEach(Array(unit.objectives.enumerated()), id: \.offset) { _, objective in
                    HStack(alignment: .firstTextBaseline, spacing: .spaceSm) {
                        Text("–").font(.bodyText)
                        Text(objective).font(.bodyText).fixedSize(horizontal: false, vertical: true)
                    }
                }

                Button("Add the \(unit.vocabulary.count) words to my review deck") {
                    let queued: QueuedCardsRecord? = progressStore.queueUnitVocabulary(courseId: courseId, unitId: unit.id, vocabularyCount: unit.vocabulary.count)

                    if let queued {
                        addedMessage = "\(queued.addedCount) new word(s) added and \(queued.madeDueCount) word(s) queued for review again."
                    }
                }
                .buttonStyle(SquareButtonStyle())
                .accessibilityIdentifier("add-words")

                if let addedMessage {
                    Text(addedMessage).font(.captionText).foregroundStyle(Color.inkMuted)
                }
            }

            VStack(alignment: .leading, spacing: .spaceSm) {
                Text(lesson.culture.title).font(.titleText)
                Paragraphs(text: lesson.culture.text)
            }
        }
    }
}

struct VocabularySection: View {
    let items: [VocabularyItemRecord]

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            HairlineRule()

            ForEach(items, id: \.cardId) { item in
                VocabularyRow(item: item)
                HairlineRule()
            }
        }
    }
}

struct VocabularyRow: View {
    let item: VocabularyItemRecord

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceXs) {
            HStack(alignment: .center, spacing: .spaceSm) {
                PlayButton(clip: AudioClip(clipKey: item.clipKey, text: item.target))
                Text(item.target).font(.targetText).bold()

                if let partOfSpeech = item.partOfSpeech {
                    TagView(text: partOfSpeech)
                }

                Spacer(minLength: 0)
            }

            Text(item.source).font(.bodyText)

            if let literal = item.sourceLiteral {
                Text("literally: \(literal)").font(.captionText).italic().foregroundStyle(Color.inkMuted)
            }

            HStack(alignment: .top, spacing: .spaceSm) {
                PlayButton(clip: AudioClip(clipKey: item.example.clipKey, text: item.example.target))

                VStack(alignment: .leading, spacing: 2) {
                    Text(item.example.target).font(.bodyText).fixedSize(horizontal: false, vertical: true)
                    Text(item.example.source).font(.captionText).foregroundStyle(Color.inkMuted).fixedSize(horizontal: false, vertical: true)
                }
            }
            .padding(.top, .spaceXs)

            if let note = item.note {
                Text(note).font(.captionText).foregroundStyle(Color.inkMuted).fixedSize(horizontal: false, vertical: true)
            }
        }
        .padding(.vertical, .spaceSm + .spaceXs)
    }
}

struct DialoguesSection: View {
    let unit: UnitOutlineRecord
    let dialogues: [DialogueRecord]

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceMd) {
            TranslationToggle()

            ForEach(Array(dialogues.enumerated()), id: \.element.id) { dialogueIndex, dialogue in
                let voiceChoice: VoiceChoiceRecord = Self.voiceChoice(dialogue)
                let itemNumber: Int = dialogueIndex + 1

                VStack(alignment: .leading, spacing: .spaceSm) {
                    UnitItemLabel(text: unit.itemLabel(itemNumber: itemNumber))
                    Text(dialogue.title).font(.titleText)
                    Text(dialogue.context).font(.captionText).foregroundStyle(Color.inkMuted).fixedSize(horizontal: false, vertical: true)
                    PlayAllButton(clips: dialogue.lines.map { line in AudioClip(clipKey: line.phrase.clipKey, text: line.phrase.target, voiceChoice: voiceChoice) })

                    ForEach(Array(dialogue.lines.enumerated()), id: \.offset) { _, line in
                        PhraseRow(pair: line.phrase, speaker: line.speaker, voiceChoice: voiceChoice)
                    }
                }
                .panel()
                .indexRailTarget(entryNumber: itemNumber)
            }
        }
    }

    private static func voiceChoice(_ dialogue: DialogueRecord) -> VoiceChoiceRecord {
        .wholeDialogue(lineTexts: dialogue.lines.map { line in line.phrase.target })
    }
}

struct GrammarSection: View {
    let unit: UnitOutlineRecord
    let notes: [GrammarNoteRecord]

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceMd) {
            TranslationToggle()

            ForEach(Array(notes.enumerated()), id: \.offset) { noteIndex, note in
                let itemNumber: Int = noteIndex + 1

                VStack(alignment: .leading, spacing: .spaceSm) {
                    UnitItemLabel(text: unit.itemLabel(itemNumber: itemNumber))
                    Text(note.title).font(.titleText)
                    Paragraphs(text: note.explanation)

                    if let table = note.table {
                        GrammarTableView(table: table)
                    }

                    ForEach(Array(note.examples.enumerated()), id: \.offset) { _, example in
                        PhraseRow(pair: example)
                    }
                }
                .panel()
                .indexRailTarget(entryNumber: itemNumber)
            }
        }
    }
}

/// The `<level>.<unit>.<box>` label at the head of a box of a unit tab.
struct UnitItemLabel: View {
    let text: String

    var body: some View {
        MicroLabel(text: text)
            .monospacedDigit()
            .accessibilityIdentifier("unit-item-label")
    }
}

extension UnitOutlineRecord {
    func itemLabel(itemNumber: Int) -> String {
        unitItemLabel(levelCode: levelCode, unitOrder: order, itemNumber: UInt32(itemNumber))
    }
}

struct ReadingsSection: View {
    let unit: UnitOutlineRecord
    let readings: [ReadingRecord]

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceMd) {
            TranslationToggle()

            ForEach(Array(readings.enumerated()), id: \.element.id) { readingIndex, reading in
                let itemNumber: Int = readingIndex + 1

                ReadingPanel(reading: reading, itemLabel: unit.itemLabel(itemNumber: itemNumber))
                    .indexRailTarget(entryNumber: itemNumber)
            }
        }
    }
}

struct ReadingPanel: View {
    let reading: ReadingRecord
    let itemLabel: String

    @State private var questionsOpen: Bool = false

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceSm) {
            UnitItemLabel(text: itemLabel)
            Text(reading.title).font(.titleText)
            PlayAllButton(clips: reading.paragraphs.map { paragraph in AudioClip(clipKey: paragraph.clipKey, text: paragraph.target) })

            ForEach(Array(reading.paragraphs.enumerated()), id: \.offset) { _, paragraph in
                PhraseRow(pair: paragraph)
            }

            if questionsOpen {
                ExerciseSession(
                    title: "\(reading.title): questions",
                    exercises: reading.questions,
                    onComplete: { _ in },
                    onExit: { questionsOpen = false },
                )
            } else {
                Button("Answer the comprehension questions") {
                    questionsOpen = true
                }
                .buttonStyle(SquareButtonStyle(isPrimary: true))
                .accessibilityIdentifier("comprehension-questions")
            }
        }
        .panel()
    }
}
