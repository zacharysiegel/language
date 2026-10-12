import SwiftUI

struct PracticeSection: View {
    static let changedPracticeNote: String = "The practice changed since you paused, so it starts over."

    let courseId: String
    let unit: UnitOutlineRecord
    let lesson: UnitLessonRecord

    @Environment(ProgressStore.self) private var progressStore
    @State private var activeMode: PracticeModeRecord?
    @State private var sessionExercises: [ExerciseRecord] = []
    @State private var answeredResults: [ExerciseResult] = []
    @State private var noticeMessage: String?

    var body: some View {
        if let mode = activeMode {
            ExerciseSession(
                title: "Practice",
                exercises: sessionExercises,
                answeredResults: answeredResults,
                exitKind: exitKind(mode: mode),
                onAnswer: { result in recordAnswer(mode: mode, result: result) },
                onComplete: { score in record(mode: mode, score: score) },
                onExit: { activeMode = nil },
            )
        } else {
            VStack(alignment: .leading, spacing: .spaceSm) {
                Text(bestSummary()).font(.bodyText)
                Text("Pass mark: \(passPercent())%.").font(.captionText).foregroundStyle(Color.inkMuted)

                if let noticeMessage {
                    Text(noticeMessage)
                        .font(.captionText)
                        .foregroundStyle(Color.inkMuted)
                        .accessibilityIdentifier("practice-notice")
                }

                startButtons()
            }
            .onAppear { dropThePausedPracticeIfTheExercisesChanged() }
        }
    }

    @ViewBuilder
    private func startButtons() -> some View {
        let pausedPracticeState: PausedPracticeStateRecord = progressStore.pausedPracticeState(courseId: courseId, unitId: unit.id, exercises: lesson.exercises)

        switch pausedPracticeState {
        case .resumable(let pausedPractice):
            Button("Resume full practice (question \(resumedQuestionNumber(pausedPractice: pausedPractice)) of \(pausedPractice.questionCount))") {
                resume(pausedPractice: pausedPractice)
            }
            .buttonStyle(SquareButtonStyle(isPrimary: true))
            .accessibilityIdentifier("resume-full-practice")

            Button("Start over") { start(.full) }
                .buttonStyle(SquareButtonStyle())
                .accessibilityIdentifier("start-over")
        case .notPaused, .exercisesChanged:
            Button("Full practice (\(lesson.exercises.count) exercises)") { start(.full) }
                .buttonStyle(SquareButtonStyle(isPrimary: true))
                .accessibilityIdentifier("full-practice")
        }

        Button("Quick practice (\(quickPracticeExerciseCount()) random exercises)") { start(.quick) }
            .buttonStyle(SquareButtonStyle())
            .accessibilityIdentifier("quick-practice")
    }

    /* A practice left on its last feedback has every question answered and resumes at its summary. */
    private func resumedQuestionNumber(pausedPractice: PausedPracticeRecord) -> UInt32 {
        min(pausedPractice.nextQuestionIndex + 1, pausedPractice.questionCount)
    }

    private func dropThePausedPracticeIfTheExercisesChanged() {
        let pausedPracticeState: PausedPracticeStateRecord = progressStore.pausedPracticeState(courseId: courseId, unitId: unit.id, exercises: lesson.exercises)

        guard pausedPracticeState == .exercisesChanged else {
            return
        }

        progressStore.clearPausedPractice(courseId: courseId, unitId: unit.id)
        noticeMessage = Self.changedPracticeNote
    }

    private func start(_ mode: PracticeModeRecord) {
        switch mode {
        case .full:
            let pausedPractice: PausedPracticeRecord? = progressStore.startFullPractice(courseId: courseId, unitId: unit.id, exercises: lesson.exercises)

            guard let pausedPractice else {
                return
            }

            open(mode: .full, exercises: exercisesInOrder(pausedPractice: pausedPractice), answeredResults: [])
        case .quick:
            open(mode: .quick, exercises: Array(lesson.exercises.shuffled().prefix(Int(quickPracticeExerciseCount()))), answeredResults: [])
        }
    }

    private func resume(pausedPractice: PausedPracticeRecord) {
        let savedResults: [ExerciseResult] = pausedPractice.results.map { savedResult in ExerciseResult.fromSaved(savedResult) }

        open(mode: .full, exercises: exercisesInOrder(pausedPractice: pausedPractice), answeredResults: savedResults)
    }

    private func open(mode: PracticeModeRecord, exercises: [ExerciseRecord], answeredResults: [ExerciseResult]) {
        sessionExercises = exercises
        self.answeredResults = answeredResults
        noticeMessage = nil
        activeMode = mode
        progressStore.startUnit(courseId: courseId, unitId: unit.id)
    }

    private func exercisesInOrder(pausedPractice: PausedPracticeRecord) -> [ExerciseRecord] {
        pausedPractice.exerciseOrder.map { exerciseIndex in lesson.exercises[Int(exerciseIndex)] }
    }

    private func exitKind(mode: PracticeModeRecord) -> ExerciseSessionExitKind {
        switch mode {
        case .full:
            return .pause
        case .quick:
            return .quit
        }
    }

    private func recordAnswer(mode: PracticeModeRecord, result: ExerciseResult) {
        switch mode {
        case .full:
            progressStore.recordFullPracticeResult(courseId: courseId, unitId: unit.id, result: result.savedResult())
        case .quick:
            break
        }
    }

    private func record(mode: PracticeModeRecord, score: SessionScoreRecord) {
        let outcome: PracticeOutcomeRecord? = recordedOutcome(mode: mode, score: score)

        if let outcome, outcome.wordsAddedCount > 0 {
            noticeMessage = "\(outcome.wordsAddedCount) word(s) from this unit were added to your review deck."
        }
    }

    private func recordedOutcome(mode: PracticeModeRecord, score: SessionScoreRecord) -> PracticeOutcomeRecord? {
        switch mode {
        case .full:
            return progressStore.finishFullPractice(courseId: courseId, unitId: unit.id, vocabularyCount: unit.vocabulary.count)
        case .quick:
            return progressStore.recordPracticeSession(
                courseId: courseId,
                unitId: unit.id,
                mode: .quick,
                correctCount: Int(score.correctCount),
                totalCount: Int(score.totalCount),
                vocabularyCount: unit.vocabulary.count,
            )
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
