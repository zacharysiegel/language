import SwiftUI

struct PracticeSection: View {
    let courseId: String
    let unit: UnitOutlineRecord
    let lesson: UnitLessonRecord

    @Environment(ProgressStore.self) private var progressStore
    @State private var activeMode: PracticeModeRecord?
    @State private var sessionExercises: [ExerciseRecord] = []
    @State private var outcomeMessage: String?

    var body: some View {
        if let mode = activeMode {
            ExerciseSession(
                title: "Practice",
                exercises: sessionExercises,
                onComplete: { score in record(mode: mode, score: score) },
                onExit: { activeMode = nil },
            )
        } else {
            VStack(alignment: .leading, spacing: .spaceSm) {
                Text(bestSummary()).font(.bodyText)
                Text("Pass mark: \(passPercent())%.").font(.captionText).foregroundStyle(Color.inkMuted)

                if let outcomeMessage {
                    Text(outcomeMessage).font(.captionText).foregroundStyle(Color.inkMuted)
                }

                Button("Full practice (\(lesson.exercises.count) exercises)") { start(.full) }
                    .buttonStyle(SquareButtonStyle(isPrimary: true))
                    .accessibilityIdentifier("full-practice")

                Button("Quick practice (\(quickPracticeExerciseCount()) random exercises)") { start(.quick) }
                    .buttonStyle(SquareButtonStyle())
                    .accessibilityIdentifier("quick-practice")
            }
        }
    }

    private func start(_ mode: PracticeModeRecord) {
        switch mode {
        case .full:
            sessionExercises = lesson.exercises
        case .quick:
            sessionExercises = Array(lesson.exercises.shuffled().prefix(Int(quickPracticeExerciseCount())))
        }

        outcomeMessage = nil
        activeMode = mode
        progressStore.startUnit(courseId: courseId, unitId: unit.id)
    }

    private func record(mode: PracticeModeRecord, score: SessionScoreRecord) {
        let outcome: PracticeOutcomeRecord? = progressStore.recordPracticeSession(
            courseId: courseId,
            unitId: unit.id,
            mode: mode,
            correctCount: Int(score.correctCount),
            totalCount: Int(score.totalCount),
            vocabularyCount: unit.vocabulary.count,
        )

        if let outcome, outcome.wordsAddedCount > 0 {
            outcomeMessage = "\(outcome.wordsAddedCount) word(s) from this unit were added to your review deck."
        }
    }

    private func bestSummary() -> String {
        let unitProgress: UnitProgressRecord = progressStore.unitProgress(courseId: courseId, unitId: unit.id)

        if unitProgress.attemptCount == 0 {
            return "You have not attempted this unit yet."
        }

        let percent: Int = Int((unitProgress.bestProportion * 100).rounded())
        let completion: String = unitProgress.status == .completed ? " Unit completed." : ""

        return "Best score \(percent)% over \(unitProgress.attemptCount) attempt(s).\(completion)"
    }
}
