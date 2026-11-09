import SwiftUI

/// Runs open responses one at a time with the self-check, keeping no score, then lists the learner's answers.
struct OpenResponseSession: View {
    let title: String
    let exercises: [ExerciseRecord]
    let onExit: () -> Void

    @State private var currentIndex: Int = 0
    @State private var answerTexts: [String] = []
    @State private var currentAnswerText: String?

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceMd) {
            if currentIndex < exercises.count {
                questionView(exercise: exercises[currentIndex])
            } else {
                summaryView()
            }
        }
        .panel()
    }

    private func questionView(exercise: ExerciseRecord) -> some View {
        VStack(alignment: .leading, spacing: .spaceMd) {
            SessionQuestionHeader(
                title: title,
                questionIndex: currentIndex,
                questionCount: exercises.count,
                exitKind: .quit,
                instruction: exerciseInstruction(exercise: exercise),
                onExit: onExit,
            )

            if case .openResponse(let promptSource, let contextSource, let exampleAnswers, let explanation, let clipKey) = exercise {
                OpenResponseQuestion(promptSource: promptSource, contextSource: contextSource, onCheck: check)
                    .id(currentIndex)

                if let currentAnswerText {
                    OpenResponseSelfCheck(
                        answerText: currentAnswerText,
                        exampleAnswers: exampleAnswers,
                        explanation: explanation,
                        clipKey: clipKey,
                        onContinue: advance,
                    )
                }
            }
        }
    }

    private func summaryView() -> some View {
        VStack(alignment: .leading, spacing: .spaceSm) {
            Text(title)
                .font(.titleText)
                .accessibilityIdentifier("session-summary")

            MicroLabel(text: "Your answers")

            ForEach(Array(zip(exercises, answerTexts).enumerated()), id: \.offset) { _, entry in
                AnswerListItem(promptText: exercisePromptText(exercise: entry.0), answerText: entry.1, answerColor: Color.inkMuted)
            }

            Button("Done") { onExit() }
                .buttonStyle(SquareButtonStyle(isPrimary: true))
                .accessibilityIdentifier("session-done")
        }
    }

    private func check(_ answerText: String) {
        guard currentAnswerText == nil else {
            return
        }

        answerTexts.append(answerText)
        currentAnswerText = answerText
    }

    private func advance() {
        guard currentAnswerText != nil else {
            return
        }

        currentAnswerText = nil
        currentIndex += 1
    }
}

/// A task the learner answers in their own words; it reports the trimmed answer once through `onCheck`.
struct OpenResponseQuestion: View {
    static let reservedLineCount: Int = 4

    let promptSource: String
    let contextSource: String?
    let onCheck: (String) -> Void

    @State private var writtenText: String = ""
    @State private var answered: Bool = false
    @FocusState private var focused: Bool

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceMd) {
            Text(promptSource).font(.targetText).fixedSize(horizontal: false, vertical: true)

            if let contextSource {
                Text(contextSource)
                    .font(.captionText)
                    .foregroundStyle(Color.inkMuted)
                    .fixedSize(horizontal: false, vertical: true)
            }

            /* Return starts a new line in a vertical text field. Spell checking follows autocorrection while its type is left at the default. */
            TextField("Type your answer", text: $writtenText, axis: .vertical)
                .lineLimit(Self.reservedLineCount...)
                .font(.targetText)
                .textInputAutocapitalization(.never)
                .autocorrectionDisabled()
                .writingToolsBehavior(.disabled)
                .focused($focused)
                .disabled(answered)
                .padding(.spaceSm)
                .overlay(Rectangle().stroke(Color.ink, lineWidth: .hairline))
                .accessibilityIdentifier("open-response-field")

            Button("Check") { check() }
                .buttonStyle(SquareButtonStyle(isPrimary: true))
                .disabled(answered || trimmedText().isEmpty)
                .accessibilityIdentifier("check")
        }
        .onAppear { focused = true }
    }

    private func trimmedText() -> String {
        writtenText.trimmingCharacters(in: .whitespacesAndNewlines)
    }

    private func check() {
        let answerText: String = trimmedText()

        guard !answered, !answerText.isEmpty else {
            return
        }

        answered = true
        focused = false
        onCheck(answerText)
    }
}

/// The feedback on an open response that no model graded: the learner's answer beside the example answers.
struct OpenResponseSelfCheck: View {
    static let heading: String = "Compare with the examples"
    static let notice: String = "Automatic grading is not available on this device. In supported web browsers and native desktop applications, an AI model checks your answer and points out each mistake."

    let answerText: String
    let exampleAnswers: [String]
    let explanation: String?
    let clipKey: String?
    let onContinue: () -> Void

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceSm) {
            VStack(alignment: .leading, spacing: .spaceXs) {
                Text(Self.heading)
                    .font(.titleText)
                    .foregroundStyle(Color.ink)
                    .accessibilityIdentifier("feedback-heading")

                Text(Self.notice)
                    .font(.captionText)
                    .foregroundStyle(Color.inkMuted)
                    .fixedSize(horizontal: false, vertical: true)
                    .accessibilityIdentifier("self-check-notice")
            }
            .accessibilityElement(children: .contain)

            MicroLabel(text: "Your answer")

            Text(answerText)
                .font(.bodyText)
                .fixedSize(horizontal: false, vertical: true)
                .accessibilityIdentifier("written-answer")

            MicroLabel(text: "Example answers")

            ForEach(Array(exampleAnswers.enumerated()), id: \.offset) { exampleIndex, exampleAnswer in
                HStack(spacing: .spaceSm) {
                    Text(exampleAnswer)
                        .font(.bodyText)
                        .fixedSize(horizontal: false, vertical: true)
                        .accessibilityIdentifier("example-answer")

                    // The exercise's clip speaks the first example answer.
                    if exampleIndex == 0 {
                        PlayButton(clip: AudioClip(clipKey: clipKey, text: exampleAnswer), label: "Hear it")
                    }
                }
            }

            if let explanation {
                Text(explanation).font(.captionText).foregroundStyle(Color.inkMuted).fixedSize(horizontal: false, vertical: true)
            }

            Button("Continue") { onContinue() }
                .buttonStyle(SquareButtonStyle(isPrimary: true))
                .accessibilityIdentifier("continue-exercise")
        }
        .padding(.spaceSm + .spaceXs)
        .frame(maxWidth: .infinity, alignment: .leading)
        .overlay(Rectangle().stroke(Color.ink, lineWidth: .hairline))
    }
}
