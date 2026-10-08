import SwiftUI

/// The learner's answer to one exercise and the answer to show back.
struct ExerciseResult: Equatable {
    let status: AnswerStatusRecord
    let answerText: String

    func isCorrect() -> Bool {
        status != .wrong
    }

    static func fromChoice(chosenCorrectly: Bool, answerText: String) -> ExerciseResult {
        ExerciseResult(status: chosenCorrectly ? .correct : .wrong, answerText: answerText)
    }
}

extension ExerciseRecord {
    func clipKey() -> String? {
        switch self {
        case .fillBlank(_, _, _, _, let clipKey),
             .translateToPortuguese(_, _, _, let clipKey),
             .orderWords(_, _, _, _, let clipKey),
             .listeningChoice(_, _, _, _, let clipKey),
             .dictation(_, _, _, let clipKey):
            return clipKey
        case .multipleChoice, .match:
            return nil
        }
    }

    func explanation() -> String? {
        switch self {
        case .multipleChoice(_, _, _, let explanation),
             .fillBlank(_, _, _, let explanation, _),
             .translateToPortuguese(_, _, let explanation, _),
             .orderWords(_, _, _, let explanation, _),
             .listeningChoice(_, _, _, let explanation, _),
             .dictation(_, _, let explanation, _),
             .match(_, _, let explanation):
            return explanation
        }
    }

    /* Kinds whose answer is a Portuguese sentence the learner did not just hear, played once the answer is checked. */
    func speaksAfterAnswer() -> Bool {
        switch self {
        case .fillBlank, .translateToPortuguese, .orderWords:
            return true
        case .multipleChoice, .listeningChoice, .dictation, .match:
            return false
        }
    }
}

/// Runs exercises one at a time with feedback, then a summary that offers to retry the missed ones.
struct ExerciseSession: View {
    let title: String
    let exercises: [ExerciseRecord]
    let onComplete: (SessionScoreRecord) -> Void
    let onExit: () -> Void

    @Environment(AudioPlayer.self) private var audioPlayer
    @Environment(CourseStore.self) private var courseStore
    @Environment(ProgressStore.self) private var progressStore

    @State private var activeExercises: [ExerciseRecord]?
    @State private var currentIndex: Int = 0
    @State private var results: [ExerciseResult] = []
    @State private var currentResult: ExerciseResult?
    @State private var isRetry: Bool = false
    @State private var completionReported: Bool = false

    var body: some View {
        let sessionExercises: [ExerciseRecord] = activeExercises ?? exercises

        VStack(alignment: .leading, spacing: .spaceMd) {
            if currentIndex < sessionExercises.count {
                questionView(exercise: sessionExercises[currentIndex], total: sessionExercises.count)
            } else {
                summaryView(sessionExercises: sessionExercises)
            }
        }
        .panel()
    }

    private func questionView(exercise: ExerciseRecord, total: Int) -> some View {
        VStack(alignment: .leading, spacing: .spaceMd) {
            HStack {
                Text("\(isRetry ? "\(title), retry of missed" : title): question \(currentIndex + 1) of \(total)")
                    .font(.captionText)
                    .foregroundStyle(Color.inkMuted)

                Spacer()

                Button("Quit") {
                    audioPlayer.stop()
                    onExit()
                }
                .font(.captionText)
                .foregroundStyle(Color.accentLink)
                .accessibilityIdentifier("quit")
            }

            SquareProgressBar(value: currentIndex, total: total)
            MicroLabel(text: exerciseInstruction(exercise: exercise))

            ExerciseBody(exercise: exercise, onSubmit: submit)
                .id("\(isRetry)-\(currentIndex)")

            if let currentResult {
                feedbackView(exercise: exercise, result: currentResult)
            }
        }
    }

    private func feedbackView(exercise: ExerciseRecord, result: ExerciseResult) -> some View {
        let spokenText: String? = exerciseSpokenText(exercise: exercise)

        return VStack(alignment: .leading, spacing: .spaceSm) {
            Text(heading(for: result.status))
                .font(.titleText)
                .foregroundStyle(result.isCorrect() ? Color.goodText : Color.accentActive)
                .accessibilityIdentifier("feedback-heading")

            if result.status != .correct {
                Text("Answer: \(result.answerText)").font(.bodyText).fixedSize(horizontal: false, vertical: true)
            }

            if let explanation = exercise.explanation() {
                Text(explanation).font(.captionText).foregroundStyle(Color.inkMuted).fixedSize(horizontal: false, vertical: true)
            }

            HStack(spacing: .spaceSm) {
                if let spokenText {
                    PlayButton(clip: AudioClip(clipKey: exercise.clipKey(), text: spokenText), label: "Hear it")
                }

                Button("Continue") { advance() }
                    .buttonStyle(SquareButtonStyle(isPrimary: true))
                    .accessibilityIdentifier("continue-exercise")
            }
        }
        .padding(.spaceSm + .spaceXs)
        .frame(maxWidth: .infinity, alignment: .leading)
        .overlay(Rectangle().stroke(result.isCorrect() ? Color.goodText : Color.accentActive, lineWidth: .hairline))
    }

    private func summaryView(sessionExercises: [ExerciseRecord]) -> some View {
        let score: SessionScoreRecord = sessionScore(correctCount: UInt32(results.filter { result in result.isCorrect() }.count), totalCount: UInt32(sessionExercises.count))
        let missed: [(ExerciseRecord, ExerciseResult)] = zip(sessionExercises, results).filter { _, result in !result.isCorrect() }

        return VStack(alignment: .leading, spacing: .spaceSm) {
            Text("\(title): \(score.correctCount) of \(score.totalCount) correct (\(score.percent)%)")
                .font(.titleText)
                .accessibilityIdentifier("session-summary")

            Text(score.passed ? "Passed." : "Not passed yet. \(passPercent())% is needed.")
                .font(.bodyText)
                .foregroundStyle(score.passed ? Color.goodText : Color.accentActive)

            if !missed.isEmpty {
                MicroLabel(text: "To review")

                ForEach(Array(missed.enumerated()), id: \.offset) { _, entry in
                    VStack(alignment: .leading, spacing: 2) {
                        Text(exercisePromptText(exercise: entry.0)).font(.bodyText)
                        Text(entry.1.answerText).font(.captionText).foregroundStyle(Color.goodText)
                    }
                    .fixedSize(horizontal: false, vertical: true)
                }
            }

            HStack(spacing: .spaceSm) {
                if !missed.isEmpty {
                    Button("Retry missed") { retryMissed(sessionExercises: sessionExercises) }
                        .buttonStyle(SquareButtonStyle(isPrimary: true))
                }

                Button("Done") { onExit() }
                    .buttonStyle(SquareButtonStyle())
                    .accessibilityIdentifier("session-done")
            }
        }
        .onAppear { reportCompletion(sessionExercises: sessionExercises) }
    }

    private func heading(for status: AnswerStatusRecord) -> String {
        switch status {
        case .correct:
            return "Correct"
        case .accentMistake:
            return "Correct, but check the accents"
        case .wrong:
            return "Not quite"
        }
    }

    private func submit(_ result: ExerciseResult) {
        guard currentResult == nil else {
            return
        }

        let exercise: ExerciseRecord = (activeExercises ?? exercises)[currentIndex]

        results.append(result)
        currentResult = result

        if exercise.speaksAfterAnswer(), let spokenText = exerciseSpokenText(exercise: exercise) {
            audioPlayer.play(
                AudioClip(clipKey: exercise.clipKey(), text: spokenText),
                version: courseStore.shownVersion(),
                engine: progressStore.settings().audioEngine,
                slow: false,
            )
        }
    }

    private func advance() {
        guard currentResult != nil else {
            return
        }

        currentResult = nil
        currentIndex += 1
    }

    private func reportCompletion(sessionExercises: [ExerciseRecord]) {
        guard !isRetry, !completionReported else {
            return
        }

        completionReported = true
        onComplete(sessionScore(correctCount: UInt32(results.filter { result in result.isCorrect() }.count), totalCount: UInt32(sessionExercises.count)))
    }

    private func retryMissed(sessionExercises: [ExerciseRecord]) {
        let missedExercises: [ExerciseRecord] = zip(sessionExercises, results).filter { _, result in !result.isCorrect() }.map { exercise, _ in exercise }

        activeExercises = missedExercises
        results = []
        currentResult = nil
        currentIndex = 0
        isRetry = true
    }
}
