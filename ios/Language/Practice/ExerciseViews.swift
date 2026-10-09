import SwiftUI

/// The interactive part of one exercise; it reports the learner's answer once through `onSubmit`.
struct ExerciseBody: View {
    let exercise: ExerciseRecord
    let onSubmit: (ExerciseResult) -> Void

    var body: some View {
        switch exercise {
        case .multipleChoice(let prompt, let choices, let answerIndex, _):
            VStack(alignment: .leading, spacing: .spaceMd) {
                Text(prompt).font(.targetText).fixedSize(horizontal: false, vertical: true)
                ChoiceList(choices: choices, answerIndex: Int(answerIndex), answerText: choices[Int(answerIndex)], onSubmit: onSubmit)
            }
        case .listeningChoice(let audioTarget, let choices, let answerIndex, _, let clipKey):
            VStack(alignment: .leading, spacing: .spaceMd) {
                ListeningControls(clip: AudioClip(clipKey: clipKey, text: audioTarget))
                ChoiceList(choices: choices, answerIndex: Int(answerIndex), answerText: "\(audioTarget) = \(choices[Int(answerIndex)])", onSubmit: onSubmit)
            }
        case .fillBlank(let promptTarget, let answers, let hintSource, _, _):
            FillBlankView(promptTarget: promptTarget, answers: answers, hintSource: hintSource, completedSentence: exerciseSpokenText(exercise: exercise) ?? promptTarget, onSubmit: onSubmit)
        case .translateToTarget(let promptSource, let answers, _, _):
            TypedAnswerView(
                prompt: { Text(promptSource).font(.targetText) },
                acceptedAnswers: answers,
                answerText: answers.joined(separator: "  |  "),
                onSubmit: onSubmit,
            )
        case .dictation(let audioTarget, let hintSource, _, let clipKey):
            TypedAnswerView(
                prompt: { ListeningControls(clip: AudioClip(clipKey: clipKey, text: audioTarget)) },
                acceptedAnswers: [audioTarget],
                answerText: "\(audioTarget) = \(hintSource)",
                onSubmit: onSubmit,
            )
        case .orderWords(let promptSource, let words, let distractors, _, _):
            OrderWordsView(promptSource: promptSource, words: words, distractors: distractors, onSubmit: onSubmit)
        case .match(let prompt, let pairs, _):
            MatchView(prompt: prompt ?? "Match each phrase with its translation", pairs: pairs, onSubmit: onSubmit)
        }
    }
}

/// Play and slow-play buttons that also play the clip once when the exercise appears.
struct ListeningControls: View {
    let clip: AudioClip

    @Environment(AudioPlayer.self) private var audioPlayer
    @Environment(ProgressStore.self) private var progressStore

    var body: some View {
        HStack(spacing: .spaceSm) {
            PlayButton(clip: clip, label: "Play")
            PlayButton(clip: clip, slow: true)
        }
        .onAppear {
            audioPlayer.play(clip, engine: progressStore.settings().audioEngine, slow: false, control: .normalSpeed)
        }
    }
}

struct ChoiceList: View {
    let choices: [String]
    let answerIndex: Int
    let answerText: String
    let onSubmit: (ExerciseResult) -> Void

    @State private var displayOrder: [Int]
    @State private var selectedIndex: Int?

    init(choices: [String], answerIndex: Int, answerText: String, onSubmit: @escaping (ExerciseResult) -> Void) {
        self.choices = choices
        self.answerIndex = answerIndex
        self.answerText = answerText
        self.onSubmit = onSubmit
        _displayOrder = State(initialValue: Array(choices.indices).shuffled())
    }

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceSm) {
            ForEach(displayOrder, id: \.self) { choiceIndex in
                Button {
                    guard selectedIndex == nil else {
                        return
                    }

                    selectedIndex = choiceIndex
                    onSubmit(ExerciseResult.fromChoice(chosenCorrectly: choiceIndex == answerIndex, answerText: answerText))
                } label: {
                    Text(choices[choiceIndex])
                }
                .buttonStyle(ChoiceButtonStyle(mark: mark(for: choiceIndex)))
                .disabled(selectedIndex != nil)
                .accessibilityIdentifier("choice")
            }
        }
    }

    private func mark(for choiceIndex: Int) -> ChoiceMarkKind {
        guard let selectedIndex else {
            return .unmarked
        }

        if choiceIndex == answerIndex {
            return .right
        }

        return choiceIndex == selectedIndex ? .wrong : .unmarked
    }
}

/// A text field checked against accepted answers, ignoring case, punctuation and spacing, and noting accents.
struct TypedAnswerView<Prompt: View>: View {
    @ViewBuilder let prompt: () -> Prompt
    let acceptedAnswers: [String]
    let answerText: String
    let onSubmit: (ExerciseResult) -> Void

    @State private var typedText: String = ""
    @State private var answered: Bool = false
    @FocusState private var focused: Bool

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceMd) {
            prompt()
            AnswerField(text: $typedText, answered: answered, focused: $focused, onSubmit: check)

            Button("Check") { check() }
                .buttonStyle(SquareButtonStyle(isPrimary: true))
                .disabled(answered || typedText.trimmingCharacters(in: .whitespaces).isEmpty)
                .accessibilityIdentifier("check")
        }
        .onAppear { focused = true }
    }

    private func check() {
        guard !answered, !typedText.trimmingCharacters(in: .whitespaces).isEmpty else {
            return
        }

        answered = true
        focused = false
        onSubmit(ExerciseResult(status: checkTypedAnswer(typedText: typedText, acceptedAnswers: acceptedAnswers), answerText: answerText))
    }
}

struct AnswerField: View {
    @Binding var text: String
    let answered: Bool
    var focused: FocusState<Bool>.Binding
    let onSubmit: () -> Void

    var body: some View {
        TextField("Type your answer", text: $text)
            .font(.targetText)
            .textInputAutocapitalization(.never)
            .autocorrectionDisabled()
            .submitLabel(.done)
            .focused(focused)
            .onSubmit(onSubmit)
            .disabled(answered)
            .padding(.spaceSm)
            .overlay(Rectangle().stroke(Color.ink, lineWidth: .hairline))
            .accessibilityIdentifier("answer-field")
    }
}

struct FillBlankView: View {
    let promptTarget: String
    let answers: [String]
    let hintSource: String
    let completedSentence: String
    let onSubmit: (ExerciseResult) -> Void

    var body: some View {
        TypedAnswerView(
            prompt: {
                VStack(alignment: .leading, spacing: .spaceXs) {
                    Text(hintSource).font(.captionText).foregroundStyle(Color.inkMuted)
                    Text(promptTarget.replacingOccurrences(of: fillBlankMarker(), with: "_____"))
                        .font(.targetText)
                        .fixedSize(horizontal: false, vertical: true)
                }
            },
            acceptedAnswers: answers,
            answerText: completedSentence,
            onSubmit: onSubmit,
        )
    }
}

struct OrderWordsView: View {
    let promptSource: String
    let sentence: String
    let onSubmit: (ExerciseResult) -> Void

    @State private var tokens: [String]
    @State private var placedTokenIds: [Int] = []
    @State private var answered: Bool = false

    init(promptSource: String, words: [String], distractors: [String], onSubmit: @escaping (ExerciseResult) -> Void) {
        self.promptSource = promptSource
        self.sentence = words.joined(separator: " ")
        self.onSubmit = onSubmit
        _tokens = State(initialValue: OrderWordsView.shuffledAvoidingIdentity(words + distractors))
    }

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceMd) {
            Text(promptSource).font(.targetText).fixedSize(horizontal: false, vertical: true)

            FlowLayout {
                ForEach(placedTokenIds, id: \.self) { tokenId in
                    tokenButton(tokens[tokenId]) {
                        placedTokenIds.removeAll { placedId in placedId == tokenId }
                    }
                    .disabled(answered)
                }
            }
            .frame(maxWidth: .infinity, minHeight: 44, alignment: .topLeading)
            .padding(.spaceSm)
            .overlay(Rectangle().stroke(Color.ink, style: StrokeStyle(lineWidth: .hairline, dash: [3, 3])))
            .accessibilityIdentifier("answer-area")

            FlowLayout {
                ForEach(Array(tokens.indices), id: \.self) { tokenId in
                    tokenButton(tokens[tokenId]) {
                        placedTokenIds.append(tokenId)
                    }
                    .disabled(answered || placedTokenIds.contains(tokenId))
                    .accessibilityIdentifier("bank-token")
                }
            }

            Button("Check") { check() }
                .buttonStyle(SquareButtonStyle(isPrimary: true))
                .disabled(answered || placedTokenIds.isEmpty)
                .accessibilityIdentifier("check")
        }
    }

    private func tokenButton(_ text: String, action: @escaping () -> Void) -> some View {
        Button(action: action) {
            Text(text).font(.targetText)
        }
        .buttonStyle(SquareButtonStyle(isCompact: true))
    }

    private func check() {
        guard !answered else {
            return
        }

        answered = true

        let attempt: String = placedTokenIds.map { tokenId in tokens[tokenId] }.joined(separator: " ")

        onSubmit(ExerciseResult.fromChoice(chosenCorrectly: attempt == sentence, answerText: sentence))
    }

    /* A shuffle that leaves the words in sentence order would give the answer away. */
    private static func shuffledAvoidingIdentity(_ items: [String]) -> [String] {
        guard items.count > 1 else {
            return items
        }

        for _ in 0..<20 {
            let candidate: [String] = items.shuffled()

            if candidate != items {
                return candidate
            }
        }

        return items.reversed()
    }
}

struct MatchView: View {
    static let wrongFlashDuration: Duration = .milliseconds(500)

    let prompt: String
    let pairs: [PhrasePairRecord]
    let onSubmit: (ExerciseResult) -> Void

    @Environment(AudioPlayer.self) private var audioPlayer
    @Environment(ProgressStore.self) private var progressStore

    @State private var leftOrder: [Int]
    @State private var rightOrder: [Int]
    @State private var selectedLeft: Int?
    @State private var selectedRight: Int?
    @State private var matchedPairIndexes: [Int] = []
    @State private var flashingWrong: [Int] = []
    @State private var mistakeCount: Int = 0

    init(prompt: String, pairs: [PhrasePairRecord], onSubmit: @escaping (ExerciseResult) -> Void) {
        self.prompt = prompt
        self.pairs = pairs
        self.onSubmit = onSubmit
        _leftOrder = State(initialValue: Array(pairs.indices).shuffled())
        _rightOrder = State(initialValue: Array(pairs.indices).shuffled())
    }

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceMd) {
            Text(prompt).font(.bodyText)

            HStack(alignment: .top, spacing: .spaceSm) {
                VStack(spacing: .spaceSm) {
                    ForEach(leftOrder, id: \.self) { pairIndex in
                        let isPending: Bool = audioPlayer.isIndicatedPending(clip: Self.targetClip(pairs[pairIndex]), control: .normalSpeed)

                        Button {
                            selectedLeft = pairIndex
                            playTarget(pairIndex)
                            resolveSelection()
                        } label: {
                            Text(pairs[pairIndex].target)
                        }
                        .buttonStyle(ChoiceButtonStyle(mark: leftMark(pairIndex)))
                        .overlay(alignment: .topTrailing) {
                            if isPending {
                                PendingGlyph(font: PlayButton.glyphFont)
                                    .foregroundStyle(Color.ink)
                                    .padding(.top, .spaceXs / 2)
                                    .padding(.trailing, .spaceXs)
                            }
                        }
                        .disabled(matchedPairIndexes.contains(pairIndex))
                        .pendingAccessibility(isIndicated: isPending, value: PlayButton.pendingAccessibilityValue)
                        .accessibilityIdentifier("match-left")
                    }
                }

                VStack(spacing: .spaceSm) {
                    ForEach(rightOrder, id: \.self) { pairIndex in
                        Button {
                            selectedRight = pairIndex
                            resolveSelection()
                        } label: {
                            Text(pairs[pairIndex].source)
                        }
                        .buttonStyle(ChoiceButtonStyle(mark: rightMark(pairIndex)))
                        .disabled(matchedPairIndexes.contains(pairIndex))
                        .accessibilityIdentifier("match-right")
                    }
                }
            }
        }
    }

    private func leftMark(_ pairIndex: Int) -> ChoiceMarkKind {
        if matchedPairIndexes.contains(pairIndex) {
            return .right
        }

        if selectedLeft == pairIndex {
            return .selected
        }

        return flashingWrong.contains(pairIndex) ? .wrong : .unmarked
    }

    private func rightMark(_ pairIndex: Int) -> ChoiceMarkKind {
        if matchedPairIndexes.contains(pairIndex) {
            return .right
        }

        return selectedRight == pairIndex ? .selected : .unmarked
    }

    private static func targetClip(_ pair: PhrasePairRecord) -> AudioClip {
        AudioClip(clipKey: pair.clipKey, text: pair.target)
    }

    private func playTarget(_ pairIndex: Int) {
        let pair: PhrasePairRecord = pairs[pairIndex]

        audioPlayer.play(Self.targetClip(pair), engine: progressStore.settings().audioEngine, slow: false, control: .normalSpeed)
    }

    private func resolveSelection() {
        guard let leftIndex = selectedLeft, let rightIndex = selectedRight else {
            return
        }

        selectedLeft = nil
        selectedRight = nil

        guard leftIndex == rightIndex else {
            mistakeCount += 1
            flashingWrong = [leftIndex, rightIndex]

            Task {
                try? await Task.sleep(for: Self.wrongFlashDuration)
                flashingWrong = []
            }

            return
        }

        matchedPairIndexes.append(leftIndex)

        if matchedPairIndexes.count == pairs.count {
            let answerText: String = pairs.map { pair in "\(pair.target) = \(pair.source)" }.joined(separator: "; ")

            onSubmit(ExerciseResult.fromChoice(chosenCorrectly: mistakeCount == 0, answerText: answerText))
        }
    }
}
