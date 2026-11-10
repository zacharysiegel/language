import SwiftUI

/// Runs open responses one at a time with the self-check, saving each answer as its task's latest, then lists the learner's answers.
struct OpenResponseSession: View {
    let title: String
    let courseId: String
    let unit: UnitOutlineRecord
    let exercises: [ExerciseRecord]
    let onExit: () -> Void

    @Environment(ProgressStore.self) private var progressStore
    @Environment(CourseStore.self) private var courseStore
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
                OpenResponseQuestion(promptSource: promptSource, contextSource: contextSource, onCheck: { answerText in check(exercise: exercise, answerText: answerText) })
                    .id(currentIndex)

                if currentAnswerText == nil, let savedOpenResponse = progressStore.savedOpenResponse(courseId: courseId, unitId: unit.id, exercise: exercise) {
                    SavedOpenResponseAnswer(savedOpenResponse: savedOpenResponse, targetLanguageName: targetLanguageName())
                }

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

    private func targetLanguageName() -> String {
        courseStore.outline(courseId: courseId)?.targetLanguageName ?? ""
    }

    private func check(exercise: ExerciseRecord, answerText: String) {
        guard currentAnswerText == nil else {
            return
        }

        progressStore.saveOpenResponseAnswer(courseId: courseId, unit: unit, exercise: exercise, answerText: answerText)
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

/// The learner's saved answer to the task, shown while they write a new one, with any grade an AI model gave it.
struct SavedOpenResponseAnswer: View {
    let savedOpenResponse: SavedOpenResponseRecord
    let targetLanguageName: String

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceSm) {
            MicroLabel(text: "Your last answer")

            Text(savedOpenResponse.answerText)
                .font(.bodyText)
                .fixedSize(horizontal: false, vertical: true)
                .accessibilityIdentifier("saved-answer")

            if let grade = savedOpenResponse.grade {
                SavedOpenResponseGrade(grade: grade, targetLanguageName: targetLanguageName)
            }
        }
        .padding(.spaceSm + .spaceXs)
        .frame(maxWidth: .infinity, alignment: .leading)
        .overlay(Rectangle().stroke(Color.rule, lineWidth: .hairline))
    }
}

/// An AI model's grade of an answer: the mistake count, the two checks and each mistake with its correction.
struct SavedOpenResponseGrade: View {
    static let aiModelTag: String = "AI model"

    let grade: SavedOpenResponseGradeRecord
    let targetLanguageName: String

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceSm) {
            HStack(spacing: .spaceSm) {
                MicroLabel(text: Self.mistakeCountHeading(findingCount: grade.findings.count)).accessibilityIdentifier("saved-grade-heading")
                TagView(text: Self.aiModelTag)
            }

            checkRow(label: "Addresses the task", value: grade.answersTheQuestion)
            checkRow(label: "Written in \(targetLanguageName)", value: grade.writtenInTargetLanguage)

            ForEach(Array(grade.findings.enumerated()), id: \.offset) { _, finding in
                VStack(alignment: .leading, spacing: .spaceXs) {
                    HStack(spacing: .spaceSm) {
                        Text(finding.quote).font(.bodyText).foregroundStyle(Color.accentActive).strikethrough()
                        Text("→").font(.bodyText).accessibilityHidden(true)
                        Text(finding.correction).font(.bodyText).fontWeight(.semibold)
                    }
                    .accessibilityElement(children: .combine)

                    Text(finding.explanation).font(.captionText).foregroundStyle(Color.inkMuted).fixedSize(horizontal: false, vertical: true)
                }
            }
        }
    }

    static func mistakeCountHeading(findingCount: Int) -> String {
        switch findingCount {
        case 0:
            return "No mistakes found"
        case 1:
            return "1 mistake found"
        default:
            return "\(findingCount) mistakes found"
        }
    }

    private func checkRow(label: String, value: Bool) -> some View {
        HStack(spacing: .spaceMd) {
            MicroLabel(text: label)
            Text(value ? "Yes" : "No").font(.bodyText)
        }
    }
}
