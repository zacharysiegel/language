import SwiftUI

struct ProgressPage: View {
    let outline: CourseOutlineRecord

    @Environment(ProgressStore.self) private var progressStore

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: .spaceLg) {
                levelsSection()
                quizResultsSection()
                deckSection()
                ProgressTransfer()
            }
            .padding(.spaceMd)
            .readableWidth()
        }
        .navigationTitle("Progress")
        .navigationBarTitleDisplayMode(.inline)
    }

    private func sectionTitle(_ text: String) -> some View {
        Text(text).font(.titleText).padding(.bottom, .spaceSm)
    }

    private func levelsSection() -> some View {
        VStack(alignment: .leading, spacing: 0) {
            sectionTitle("Levels")
            HairlineRule()

            ForEach(progressStore.levelCompletions(outline: outline), id: \.levelCode) { levelCompletion in
                VStack(alignment: .leading, spacing: .spaceXs) {
                    Text("\(levelCompletion.levelCode) · \(levelCompletion.levelTitle)").font(.bodyText)
                    Text("\(levelCompletion.completedUnitCount) of \(levelCompletion.unitCount) units completed")
                        .font(.dataText)
                        .foregroundStyle(Color.inkMuted)
                }
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding(.vertical, .spaceSm)
                .accessibilityElement(children: .combine)
                .accessibilityIdentifier("level-completion-\(levelCompletion.levelCode)")

                HairlineRule()
            }
        }
    }

    private func quizResultsSection() -> some View {
        let quizResults: [QuizResultRecord] = progressStore.quizResults(outline: outline)

        return VStack(alignment: .leading, spacing: 0) {
            sectionTitle("Quiz results")
            HairlineRule()

            if quizResults.isEmpty {
                Text("You have not taken a quiz yet.")
                    .font(.bodyText)
                    .foregroundStyle(Color.inkMuted)
                    .padding(.vertical, .spaceSm)
                    .accessibilityIdentifier("quiz-results-empty")

                HairlineRule()
            }

            ForEach(quizResults, id: \.unitId) { quizResult in
                NavigationLink(value: UnitDestination(courseId: outline.id, unitId: quizResult.unitId, tab: .overview)) {
                    quizResultRow(quizResult)
                }
                .buttonStyle(.plain)
                .accessibilityIdentifier("quiz-result-\(quizResult.unitId)")

                HairlineRule()
            }
        }
    }

    private func quizResultRow(_ quizResult: QuizResultRecord) -> some View {
        HStack(spacing: .spaceSm) {
            VStack(alignment: .leading, spacing: .spaceXs) {
                Text(quizResult.unitTitle)
                    .font(.bodyText)
                    .foregroundStyle(Color.accentLink)
                    .multilineTextAlignment(.leading)

                Text("Best score: \(quizResult.bestScorePercent)% over \(countedNounPhrase(count: quizResult.attemptCount, singularNoun: "attempt", pluralNoun: "attempts"))")
                    .font(.dataText)
                    .foregroundStyle(Color.inkMuted)
            }

            Spacer()

            Image(systemName: "chevron.right")
                .font(.system(size: 11))
                .foregroundStyle(Color.rule)
        }
        .padding(.vertical, .spaceSm)
        .contentShape(Rectangle())
    }

    private func deckSection() -> some View {
        let intervals: [UInt32] = reviewIntervalDays()
        let counts: [Int] = progressStore.countByBox(courseId: outline.id)

        return VStack(alignment: .leading, spacing: 0) {
            sectionTitle("Review deck")
            HairlineRule()

            ForEach(Array(zip(intervals, counts).enumerated()), id: \.offset) { _, entry in
                HStack {
                    Text(reviewBoxLabel(intervalDays: entry.0)).font(.bodyText)
                    Spacer()
                    Text("\(entry.1)").font(.dataText)
                }
                .padding(.vertical, .spaceSm)

                HairlineRule()
            }
        }
    }
}

/// Copies progress out as text, restores it from text, or erases it.
struct ProgressTransfer: View {
    @Environment(ProgressStore.self) private var progressStore
    @State private var transferText: String = ""
    @State private var message: String?
    @State private var resetRequested: Bool = false

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceSm) {
            Text("Back up or move your progress").font(.titleText)
            Text("Progress is saved on this device only. Export it to keep a backup, or paste a backup to restore it.")
                .font(.captionText)
                .foregroundStyle(Color.inkMuted)
                .fixedSize(horizontal: false, vertical: true)

            HStack(spacing: .spaceSm) {
                Button("Export") {
                    transferText = progressStore.exportJson() ?? ""
                    UIPasteboard.general.string = transferText
                    message = "Progress exported and copied."
                }
                .buttonStyle(SquareButtonStyle(isCompact: true))

                Button("Import from the text box") {
                    let imported: Bool = progressStore.importJson(transferText)
                    message = imported ? "Progress imported." : "Could not import: \(progressStore.lastError() ?? "unreadable text")"
                }
                .buttonStyle(SquareButtonStyle(isCompact: true))
            }

            TextEditor(text: $transferText)
                .font(.system(size: 11, design: .monospaced))
                .frame(height: 140)
                .scrollContentBackground(.hidden)
                .overlay(Rectangle().stroke(Color.ink, lineWidth: .hairline))
                .accessibilityIdentifier("transfer-text")

            if let message {
                Text(message).font(.captionText).foregroundStyle(Color.inkMuted)
            }

            Text("Reset").font(.titleText).padding(.top, .spaceMd)

            Button(resetRequested ? "Tap again to erase all progress" : "Reset all progress") {
                guard resetRequested else {
                    resetRequested = true
                    return
                }

                progressStore.reset()
                resetRequested = false
                message = "All progress was reset."
            }
            .buttonStyle(SquareButtonStyle())
        }
    }
}
