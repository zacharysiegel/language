import SwiftUI

/// A session open on the Practice tab.
private enum PracticeSessionKind: Equatable {
    case quiz
    case quickPractice
    case openResponse

    func title() -> String {
        switch self {
        case .quiz:
            return "Quiz"
        case .quickPractice:
            return "Practice"
        case .openResponse:
            return "Open response"
        }
    }

    /// `nil` for a session that records nothing.
    func practiceMode() -> PracticeModeRecord? {
        switch self {
        case .quiz:
            return .full
        case .quickPractice:
            return .quick
        case .openResponse:
            return nil
        }
    }
}

struct PracticeSection: View {
    static let changedPracticeNote: String = "The practice changed since you paused, so it starts over."

    let courseId: String
    let unit: UnitOutlineRecord
    let lesson: UnitLessonRecord

    @Environment(ProgressStore.self) private var progressStore
    @State private var activeSession: PracticeSessionKind?
    @State private var sessionExercises: [ExerciseRecord] = []
    @State private var answeredResults: [ExerciseResult] = []
    @State private var noticeMessage: String?

    var body: some View {
        if let activeSession {
            sessionView(sessionKind: activeSession)
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
    private func sessionView(sessionKind: PracticeSessionKind) -> some View {
        if let mode = sessionKind.practiceMode() {
            ExerciseSession(
                title: sessionKind.title(),
                exercises: sessionExercises,
                answeredResults: answeredResults,
                exitKind: exitKind(mode: mode),
                onAnswer: { result in recordAnswer(mode: mode, result: result) },
                onComplete: { score in record(mode: mode, score: score) },
                onExit: { activeSession = nil },
            )
        } else {
            OpenResponseSession(
                title: sessionKind.title(),
                courseId: courseId,
                unit: unit,
                exercises: unitOpenResponseExercises(),
                onExit: { activeSession = nil },
            )
        }
    }

    @ViewBuilder
    private func startButtons() -> some View {
        let pausedPracticeState: PausedPracticeStateRecord = progressStore.pausedPracticeState(courseId: courseId, unitId: unit.id, exercises: unitQuizExercises())

        switch pausedPracticeState {
        case .resumable(let pausedPractice):
            Button("Resume quiz (question \(resumedQuestionNumber(pausedPractice: pausedPractice)) of \(pausedPractice.questionCount))") {
                resume(pausedPractice: pausedPractice)
            }
            .buttonStyle(SquareButtonStyle(isPrimary: true))
            .accessibilityIdentifier("resume-quiz")

            Button("Start over") { start(.full) }
                .buttonStyle(SquareButtonStyle())
                .accessibilityIdentifier("start-over")
        case .notPaused, .exercisesChanged:
            Button(quizLabel(exerciseCount: unitQuizExercises().count)) { start(.full) }
                .buttonStyle(SquareButtonStyle(isPrimary: true))
                .accessibilityIdentifier("quiz")
        }

        Button("Quick practice (\(quickPracticeExerciseCount()) random exercises)") { start(.quick) }
            .buttonStyle(SquareButtonStyle())
            .accessibilityIdentifier("quick-practice")

        let openResponseTaskCount: Int = unitOpenResponseExercises().count

        if openResponseTaskCount > 0 {
            Button(openResponseLabel(taskCount: openResponseTaskCount)) { startOpenResponse() }
                .buttonStyle(SquareButtonStyle())
                .accessibilityIdentifier("open-response")
        }
    }

    private func quizLabel(exerciseCount: Int) -> String {
        "Quiz (\(countedNounPhrase(count: UInt32(exerciseCount), singularNoun: "exercise", pluralNoun: "exercises")))"
    }

    private func openResponseLabel(taskCount: Int) -> String {
        "Open response (\(countedNounPhrase(count: UInt32(taskCount), singularNoun: "task", pluralNoun: "tasks")))"
    }

    /* A practice left on its last feedback has every question answered and resumes at its summary. */
    private func resumedQuestionNumber(pausedPractice: PausedPracticeRecord) -> UInt32 {
        min(pausedPractice.nextQuestionIndex + 1, pausedPractice.questionCount)
    }

    private func dropThePausedPracticeIfTheExercisesChanged() {
        let pausedPracticeState: PausedPracticeStateRecord = progressStore.pausedPracticeState(courseId: courseId, unitId: unit.id, exercises: unitQuizExercises())

        guard pausedPracticeState == .exercisesChanged else {
            return
        }

        progressStore.clearPausedPractice(courseId: courseId, unitId: unit.id)
        noticeMessage = Self.changedPracticeNote
    }

    private func start(_ mode: PracticeModeRecord) {
        switch mode {
        case .full:
            let pausedPractice: PausedPracticeRecord? = progressStore.startFullPractice(courseId: courseId, unitId: unit.id, exercises: unitQuizExercises())

            guard let pausedPractice else {
                return
            }

            open(sessionKind: .quiz, exercises: exercisesInOrder(pausedPractice: pausedPractice), answeredResults: [])
        case .quick:
            open(sessionKind: .quickPractice, exercises: quickPracticeExercises(unitExercises: lesson.exercises), answeredResults: [])
        }
    }

    private func resume(pausedPractice: PausedPracticeRecord) {
        let savedResults: [ExerciseResult] = pausedPractice.results.map { savedResult in ExerciseResult.fromSaved(savedResult) }

        open(sessionKind: .quiz, exercises: exercisesInOrder(pausedPractice: pausedPractice), answeredResults: savedResults)
    }

    private func open(sessionKind: PracticeSessionKind, exercises: [ExerciseRecord], answeredResults: [ExerciseResult]) {
        sessionExercises = exercises
        self.answeredResults = answeredResults
        noticeMessage = nil
        activeSession = sessionKind
        progressStore.startUnit(courseId: courseId, unitId: unit.id)
    }

    private func startOpenResponse() {
        noticeMessage = nil
        activeSession = .openResponse
    }

    private func unitQuizExercises() -> [ExerciseRecord] {
        quizExercises(unitExercises: lesson.exercises)
    }

    private func unitOpenResponseExercises() -> [ExerciseRecord] {
        openResponseExercises(unitExercises: lesson.exercises)
    }

    private func exercisesInOrder(pausedPractice: PausedPracticeRecord) -> [ExerciseRecord] {
        let quizExercises: [ExerciseRecord] = unitQuizExercises()

        return pausedPractice.exerciseOrder.map { exerciseIndex in quizExercises[Int(exerciseIndex)] }
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
            noticeMessage = "Added \(countedNounPhrase(count: outcome.wordsAddedCount, singularNoun: "word", pluralNoun: "words")) from this unit to your review deck."
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
            return "You have not taken the quiz yet."
        }

        let completion: String = unitProgress.status == .completed ? " Unit completed." : ""

        return "Quiz: \(unitProgress.bestScorePercent)% over \(countedNounPhrase(count: unitProgress.attemptCount, singularNoun: "attempt", pluralNoun: "attempts")).\(completion)"
    }
}
