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

                Text("Opening the unit starts it. Pass its full practice with at least \(passPercent())% to complete it.")
                    .font(.captionText)
                    .foregroundStyle(Color.inkMuted)
                    .fixedSize(horizontal: false, vertical: true)

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
    let dialogues: [DialogueRecord]

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceMd) {
            TranslationToggle()

            ForEach(dialogues, id: \.id) { dialogue in
                VStack(alignment: .leading, spacing: .spaceSm) {
                    Text(dialogue.title).font(.titleText)
                    Text(dialogue.context).font(.captionText).foregroundStyle(Color.inkMuted).fixedSize(horizontal: false, vertical: true)
                    PlayAllButton(clips: dialogue.lines.map { line in AudioClip(clipKey: line.phrase.clipKey, text: line.phrase.target) })

                    ForEach(Array(dialogue.lines.enumerated()), id: \.offset) { _, line in
                        PhraseRow(pair: line.phrase, speaker: line.speaker)
                    }
                }
                .panel()
            }
        }
    }
}

struct GrammarSection: View {
    let notes: [GrammarNoteRecord]

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceMd) {
            TranslationToggle()

            ForEach(Array(notes.enumerated()), id: \.offset) { _, note in
                VStack(alignment: .leading, spacing: .spaceSm) {
                    Text(note.title).font(.titleText)
                    Paragraphs(text: note.explanation)

                    if let table = note.table {
                        ParadigmTable(table: table)
                    }

                    ForEach(Array(note.examples.enumerated()), id: \.offset) { _, example in
                        PhraseRow(pair: example)
                    }
                }
                .panel()
            }
        }
    }
}

/// A grammar table drawn as a hairline grid, scrolling sideways when wider than the screen.
struct ParadigmTable: View {
    let table: GrammarTableRecord

    var body: some View {
        ScrollView(.horizontal, showsIndicators: false) {
            Grid(horizontalSpacing: 0, verticalSpacing: 0) {
                GridRow {
                    ForEach(Array(table.headers.enumerated()), id: \.offset) { _, header in
                        cell(Text(header.uppercased()).font(.labelText).tracking(0.4).foregroundStyle(Color.inkMuted))
                    }
                }

                ForEach(Array(table.rows.enumerated()), id: \.offset) { _, row in
                    GridRow {
                        ForEach(Array(row.enumerated()), id: \.offset) { _, value in
                            cell(Text(value).font(.bodyText))
                        }
                    }
                }
            }
            .overlay(Rectangle().stroke(Color.ink, lineWidth: .hairline))
        }
    }

    private func cell(_ content: some View) -> some View {
        content
            .padding(.horizontal, .spaceSm)
            .padding(.vertical, .spaceXs + 2)
            .frame(minWidth: 80, maxWidth: .infinity, alignment: .leading)
            .overlay(Rectangle().stroke(Color.rule, lineWidth: .hairline))
    }
}

struct ReadingsSection: View {
    let readings: [ReadingRecord]

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceMd) {
            TranslationToggle()

            ForEach(readings, id: \.id) { reading in
                ReadingPanel(reading: reading)
            }
        }
    }
}

struct ReadingPanel: View {
    let reading: ReadingRecord

    @State private var questionsOpen: Bool = false

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceSm) {
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
