import SwiftUI

/// A card to show in a review session, and which side faces the learner first.
struct ReviewItem: Equatable {
    let cardId: String
    let promptIsTarget: Bool
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

    let outline: CourseOutlineRecord

    @Environment(ProgressStore.self) private var progressStore
    @State private var sessionItems: [ReviewItem]?

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: .spaceMd) {
                if let sessionItems {
                    ReviewSession(courseId: outline.id, index: VocabularyIndex(outline: outline), items: sessionItems, onExit: { self.sessionItems = nil })
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
        let dueCount: Int = progressStore.dueCardCount(courseId: outline.id)
        let laterCount: Int = max(progressStore.cardCount(courseId: outline.id) - dueCount, 0)

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
                Text("\(outline.targetLanguageName) to \(outline.sourceLanguageName)").tag(ReviewDirectionRecord.targetToSource)
                Text("\(outline.sourceLanguageName) to \(outline.targetLanguageName)").tag(ReviewDirectionRecord.sourceToTarget)
                Text("Mixed").tag(ReviewDirectionRecord.mixed)
            }
            .pickerStyle(.menu)
            .tint(Color.ink)
            .overlay(Rectangle().stroke(Color.ink, lineWidth: .hairline))
        }
    }

    private func buildSessionItems() -> [ReviewItem] {
        let direction: ReviewDirectionRecord = progressStore.settings().reviewDirection

        return progressStore.dueCardIds(courseId: outline.id, limit: Self.sessionCardLimit).map { cardId in
            let promptIsTarget: Bool

            switch direction {
            case .targetToSource:
                promptIsTarget = true
            case .sourceToTarget:
                promptIsTarget = false
            case .mixed:
                promptIsTarget = Bool.random()
            }

            return ReviewItem(cardId: cardId, promptIsTarget: promptIsTarget)
        }
    }
}

struct ReviewSession: View {
    let courseId: String
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
                promptIsTarget: item.promptIsTarget,
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
        progressStore.gradeCard(courseId: courseId, cardId: items[currentIndex].cardId, remembered: remembered)

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
    let promptIsTarget: Bool
    let position: Int
    let total: Int
    @Binding var revealed: Bool
    let onGrade: (Bool) -> Void
    let onExit: () -> Void

    @Environment(AudioPlayer.self) private var audioPlayer
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
                Text(promptIsTarget ? item.target : item.source)
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
        .onAppear { playIfTargetShows() }
        .onChange(of: revealed) { playIfTargetShows() }
    }

    private func answerView() -> some View {
        VStack(alignment: .leading, spacing: .spaceSm) {
            HStack(spacing: .spaceSm) {
                PlayButton(clip: AudioClip(clipKey: item.clipKey, text: item.target))
                Text(promptIsTarget ? item.source : item.target).font(.titleText)
            }

            if let literal = item.sourceLiteral {
                Text("literally: \(literal)").font(.captionText).italic().foregroundStyle(Color.inkMuted)
            }

            HStack(alignment: .top, spacing: .spaceSm) {
                PlayButton(clip: AudioClip(clipKey: item.example.clipKey, text: item.example.target))

                VStack(alignment: .leading, spacing: 2) {
                    Text(item.example.target).font(.bodyText)
                    Text(item.example.source).font(.captionText).foregroundStyle(Color.inkMuted)
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

    private func playIfTargetShows() {
        guard promptIsTarget || revealed else {
            return
        }

        audioPlayer.play(AudioClip(clipKey: item.clipKey, text: item.target), engine: progressStore.settings().audioEngine, slow: false, control: .normalSpeed)
    }
}
