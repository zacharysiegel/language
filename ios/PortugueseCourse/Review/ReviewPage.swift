import SwiftUI

/// A card to show in a review session, and which side faces the learner first.
struct ReviewItem: Equatable {
    let cardId: String
    let promptIsPortuguese: Bool
}

/// Every vocabulary item of the course by its review card id, with the unit it belongs to.
struct VocabularyIndex {
    let entriesByCardId: [String: (UnitOutlineRecord, VocabularyItemRecord)]

    init(outline: CourseOutlineRecord) {
        var entriesByCardId: [String: (UnitOutlineRecord, VocabularyItemRecord)] = [:]

        for unit in outline.units {
            for item in unit.vocabulary {
                entriesByCardId[item.cardId] = (unit, item)
            }
        }

        self.entriesByCardId = entriesByCardId
    }
}

struct ReviewPage: View {
    static let sessionCardLimit: Int = 20

    @Environment(CourseStore.self) private var courseStore
    @Environment(ProgressStore.self) private var progressStore
    @State private var sessionItems: [ReviewItem]?

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: .spaceMd) {
                if let sessionItems, let outline = courseStore.outline() {
                    ReviewSession(index: VocabularyIndex(outline: outline), items: sessionItems, onExit: { self.sessionItems = nil })
                } else {
                    setupView()
                }
            }
            .padding(.spaceMd)
            .readableWidth()
        }
        .navigationTitle("Review")
        .navigationBarTitleDisplayMode(.inline)
    }

    private func setupView() -> some View {
        let dueCount: Int = progressStore.dueCardCount()
        let laterCount: Int = max(progressStore.cardCount() - dueCount, 0)

        return VStack(alignment: .leading, spacing: .spaceMd) {
            PageHeader(caption: nil, title: "Review")

            VStack(alignment: .leading, spacing: .spaceXs) {
                Text("\(dueCount)").font(.figureText).accessibilityIdentifier("due-count")
                Text("words ready to review now").font(.bodyText)
                Text("\(laterCount) scheduled for later").font(.dataText).foregroundStyle(Color.inkMuted)
            }

            Text("A word is ready when its next review time has come. Words you remember come back after a longer wait each time; words you forget come back in a minute. The number on the Review tab is the count of words ready now.")
                .font(.captionText)
                .foregroundStyle(Color.inkMuted)
                .fixedSize(horizontal: false, vertical: true)

            directionPicker()

            if dueCount == 0 {
                Text("No words are ready. Add a unit's words to your deck from its overview, or pass its practice; words you have already reviewed return when their wait is over.")
                    .font(.captionText)
                    .foregroundStyle(Color.inkMuted)
                    .fixedSize(horizontal: false, vertical: true)
            } else {
                Button("Start review") { sessionItems = buildSessionItems() }
                    .buttonStyle(SquareButtonStyle(isPrimary: true))
                    .accessibilityIdentifier("start-review")
            }
        }
    }

    private func directionPicker() -> some View {
        VStack(alignment: .leading, spacing: .spaceXs) {
            MicroLabel(text: "Direction")

            Picker("Direction", selection: Binding(
                get: { progressStore.settings().reviewDirection },
                set: { direction in progressStore.updateSettings { settings in settings.reviewDirection = direction } },
            )) {
                Text("Portuguese to English").tag(ReviewDirectionRecord.portugueseToEnglish)
                Text("English to Portuguese").tag(ReviewDirectionRecord.englishToPortuguese)
                Text("Mixed").tag(ReviewDirectionRecord.mixed)
            }
            .pickerStyle(.menu)
            .tint(Color.ink)
            .overlay(Rectangle().stroke(Color.ink, lineWidth: .hairline))
        }
    }

    private func buildSessionItems() -> [ReviewItem] {
        let direction: ReviewDirectionRecord = progressStore.settings().reviewDirection

        return progressStore.dueCardIds(limit: Self.sessionCardLimit).map { cardId in
            let promptIsPortuguese: Bool

            switch direction {
            case .portugueseToEnglish:
                promptIsPortuguese = true
            case .englishToPortuguese:
                promptIsPortuguese = false
            case .mixed:
                promptIsPortuguese = Bool.random()
            }

            return ReviewItem(cardId: cardId, promptIsPortuguese: promptIsPortuguese)
        }
    }
}

struct ReviewSession: View {
    let index: VocabularyIndex
    let items: [ReviewItem]
    let onExit: () -> Void

    @Environment(ProgressStore.self) private var progressStore
    @Environment(AudioPlayer.self) private var audioPlayer
    @State private var currentIndex: Int = 0
    @State private var revealed: Bool = false
    @State private var rememberedCount: Int = 0

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceMd) {
            if currentIndex < items.count {
                cardView(item: items[currentIndex])
                    .id(currentIndex)
            } else {
                Text("Session complete").font(.titleText).accessibilityIdentifier("review-complete")
                Text("You remembered \(rememberedCount) of \(items.count).").font(.bodyText)
                Button("Done") { onExit() }
                    .buttonStyle(SquareButtonStyle(isPrimary: true))
            }
        }
    }

    @ViewBuilder
    private func cardView(item: ReviewItem) -> some View {
        if let entry = index.entriesByCardId[item.cardId] {
            ReviewCardView(
                unit: entry.0,
                item: entry.1,
                promptIsPortuguese: item.promptIsPortuguese,
                position: currentIndex,
                total: items.count,
                revealed: $revealed,
                onGrade: grade,
                onExit: onExit,
            )
        } else {
            Text("This card refers to a word that is no longer in the course.").font(.bodyText).foregroundStyle(Color.accentActive)
            Button("Skip") {
                revealed = false
                currentIndex += 1
            }
            .buttonStyle(SquareButtonStyle())
        }
    }

    private func grade(remembered: Bool) {
        progressStore.gradeCard(cardId: items[currentIndex].cardId, remembered: remembered)

        if remembered {
            rememberedCount += 1
        }

        audioPlayer.stop()
        revealed = false
        currentIndex += 1
    }
}

struct ReviewCardView: View {
    let unit: UnitOutlineRecord
    let item: VocabularyItemRecord
    let promptIsPortuguese: Bool
    let position: Int
    let total: Int
    @Binding var revealed: Bool
    let onGrade: (Bool) -> Void
    let onExit: () -> Void

    @Environment(AudioPlayer.self) private var audioPlayer
    @Environment(CourseStore.self) private var courseStore
    @Environment(ProgressStore.self) private var progressStore

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceMd) {
            HStack {
                Text("Card \(position + 1) of \(total)").font(.captionText).foregroundStyle(Color.inkMuted)
                Spacer()
                Button("Quit") { onExit() }.font(.captionText).foregroundStyle(Color.accentLink)
            }

            SquareProgressBar(value: position, total: total)

            VStack(alignment: .leading, spacing: .spaceMd) {
                MicroLabel(text: unit.title)
                Text(promptIsPortuguese ? item.pt : item.en)
                    .font(.headingText)
                    .accessibilityIdentifier("flashcard-prompt")

                if revealed {
                    answerView()
                } else {
                    Button("Show answer") { revealed = true }
                        .buttonStyle(SquareButtonStyle(isPrimary: true))
                        .accessibilityIdentifier("show-answer")
                }
            }
            .panel()
        }
        .onAppear { playIfPortugueseShows() }
        .onChange(of: revealed) { playIfPortugueseShows() }
    }

    private func answerView() -> some View {
        VStack(alignment: .leading, spacing: .spaceSm) {
            HStack(spacing: .spaceSm) {
                PlayButton(clip: AudioClip(clipKey: item.clipKey, text: item.pt))
                Text(promptIsPortuguese ? item.en : item.pt).font(.titleText)
            }

            if let literal = item.literal {
                Text("literally: \(literal)").font(.captionText).italic().foregroundStyle(Color.inkMuted)
            }

            HStack(alignment: .top, spacing: .spaceSm) {
                PlayButton(clip: AudioClip(clipKey: item.example.clipKey, text: item.example.pt))

                VStack(alignment: .leading, spacing: 2) {
                    Text(item.example.pt).font(.bodyText)
                    Text(item.example.en).font(.captionText).foregroundStyle(Color.inkMuted)
                }
                .fixedSize(horizontal: false, vertical: true)
            }

            if let note = item.note {
                Text(note).font(.captionText).foregroundStyle(Color.inkMuted).fixedSize(horizontal: false, vertical: true)
            }

            HStack(spacing: .spaceSm) {
                Button("Forgot") { onGrade(false) }
                    .buttonStyle(SquareButtonStyle())
                    .accessibilityIdentifier("forgot")

                Button("Remembered") { onGrade(true) }
                    .buttonStyle(SquareButtonStyle(isPrimary: true))
                    .accessibilityIdentifier("remembered")
            }
        }
    }

    private func playIfPortugueseShows() {
        guard promptIsPortuguese || revealed else {
            return
        }

        audioPlayer.play(AudioClip(clipKey: item.clipKey, text: item.pt), version: courseStore.shownVersion(), engine: progressStore.settings().audioEngine, slow: false)
    }
}
