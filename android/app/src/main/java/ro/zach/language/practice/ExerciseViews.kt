package ro.zach.language.practice

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.foundation.border
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.drawBehind
import androidx.compose.ui.focus.FocusManager
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.graphics.PathEffect
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardCapitalization
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import ro.zach.language.core.AnswerStatusRecord
import ro.zach.language.core.ExerciseRecord
import ro.zach.language.core.PhrasePairRecord
import ro.zach.language.core.checkTypedAnswer
import ro.zach.language.core.exerciseSpokenText
import ro.zach.language.core.fillBlankMarker
import ro.zach.language.design.ChoiceButton
import ro.zach.language.design.ChoiceMarkKind
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType
import ro.zach.language.design.PendingGlyph
import ro.zach.language.design.SquareButton
import ro.zach.language.design.SquareTextButton
import ro.zach.language.design.pendingAccessibility
import ro.zach.language.progressdata.LocalProgressStore
import ro.zach.language.progressdata.ProgressStore
import kotlin.random.Random

private const val MATCH_WRONG_FLASH_MILLISECONDS: Long = 500
private const val ORDER_WORDS_SHUFFLE_ATTEMPTS: Int = 20
private const val BLANK_SHOWN: String = "_____"
private const val TRANSLATION_ANSWER_SEPARATOR: String = "  |  "
private const val MATCH_PAIR_SEPARATOR: String = "; "
private const val DEFAULT_MATCH_PROMPT: String = "Match each phrase with its translation"

/** The interactive part of one exercise; it reports the learner's answer once through `onSubmit`. */
@Composable
fun ExerciseBody(exercise: ExerciseRecord, onSubmit: (ExerciseResult) -> Unit) {
    when (exercise) {
        is ExerciseRecord.MultipleChoice -> {
            val answerIndex: Int = exercise.answerIndex.toInt()

            Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd)) {
                LanguageText(text = exercise.prompt, style = LanguageType.targetText)
                ChoiceList(choices = exercise.choices, answerIndex = answerIndex, answerText = exercise.choices[answerIndex], onSubmit = onSubmit)
            }
        }
        is ExerciseRecord.ListeningChoice -> {
            val answerIndex: Int = exercise.answerIndex.toInt()

            Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd)) {
                ListeningControls(clip = AudioClip(clipKey = exercise.clipKey, text = exercise.audioTarget))
                ChoiceList(
                    choices = exercise.choices,
                    answerIndex = answerIndex,
                    answerText = "${exercise.audioTarget} = ${exercise.choices[answerIndex]}",
                    onSubmit = onSubmit,
                )
            }
        }
        is ExerciseRecord.FillBlank -> FillBlankView(
            exercise = exercise,
            completedSentence = exerciseSpokenText(exercise) ?: exercise.promptTarget,
            onSubmit = onSubmit,
        )
        is ExerciseRecord.TranslateToTarget -> TypedAnswerView(
            acceptedAnswers = exercise.answers,
            answerText = exercise.answers.joinToString(TRANSLATION_ANSWER_SEPARATOR),
            onSubmit = onSubmit,
        ) {
            LanguageText(text = exercise.promptSource, style = LanguageType.targetText)
        }
        is ExerciseRecord.Dictation -> TypedAnswerView(
            acceptedAnswers = listOf(exercise.audioTarget),
            answerText = "${exercise.audioTarget} = ${exercise.hintSource}",
            onSubmit = onSubmit,
        ) {
            ListeningControls(clip = AudioClip(clipKey = exercise.clipKey, text = exercise.audioTarget))
        }
        is ExerciseRecord.OrderWords -> OrderWordsView(exercise = exercise, onSubmit = onSubmit)
        is ExerciseRecord.Match -> MatchView(prompt = exercise.prompt ?: DEFAULT_MATCH_PROMPT, pairs = exercise.pairs, onSubmit = onSubmit)
    }
}

/** Play and slow-play buttons that also play the clip once when the exercise appears. */
@Composable
private fun ListeningControls(clip: AudioClip) {
    val audioPlayer: AudioPlayer = LocalAudioPlayer.current
    val progressStore: ProgressStore = LocalProgressStore.current

    LaunchedEffect(clip) {
        audioPlayer.play(clip, progressStore.settings().audioEngine, false, PlaybackControlKind.NormalSpeed)
    }

    Row(horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm), verticalAlignment = Alignment.CenterVertically) {
        PlayButton(clip = clip, label = "Play")
        PlayButton(clip = clip, slow = true)
    }
}

@Composable
private fun ChoiceList(choices: List<String>, answerIndex: Int, answerText: String, onSubmit: (ExerciseResult) -> Unit) {
    val displayOrder: List<Int> = rememberSaveable { choices.indices.shuffled() }
    var selectedIndex: Int? by rememberSaveable { mutableStateOf(null) }

    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
        for (choiceIndex in displayOrder) {
            ChoiceButton(
                mark = choiceMark(choiceIndex = choiceIndex, answerIndex = answerIndex, selectedIndex = selectedIndex),
                onClick = {
                    if (selectedIndex == null) {
                        selectedIndex = choiceIndex
                        onSubmit(ExerciseResult.fromChoice(chosenCorrectly = choiceIndex == answerIndex, answerText = answerText))
                    }
                },
                enabled = selectedIndex == null,
                modifier = Modifier.testTag("choice"),
            ) { contentColor ->
                LanguageText(text = choices[choiceIndex], style = LanguageType.bodyText, color = contentColor)
            }
        }
    }
}

private fun choiceMark(choiceIndex: Int, answerIndex: Int, selectedIndex: Int?): ChoiceMarkKind {
    if (selectedIndex == null) {
        return ChoiceMarkKind.Unmarked
    }

    if (choiceIndex == answerIndex) {
        return ChoiceMarkKind.Right
    }

    return if (choiceIndex == selectedIndex) ChoiceMarkKind.Wrong else ChoiceMarkKind.Unmarked
}

/** A text field checked against accepted answers, ignoring case, punctuation and spacing, and noting accents. */
@Composable
private fun TypedAnswerView(
    acceptedAnswers: List<String>,
    answerText: String,
    onSubmit: (ExerciseResult) -> Unit,
    prompt: @Composable () -> Unit,
) {
    var typedText: String by rememberSaveable { mutableStateOf("") }
    var isAnswered: Boolean by rememberSaveable { mutableStateOf(false) }
    val focusRequester: FocusRequester = remember { FocusRequester() }
    val focusManager: FocusManager = LocalFocusManager.current
    val check: () -> Unit = {
        if (!isAnswered && typedText.isNotBlank()) {
            isAnswered = true
            focusManager.clearFocus()

            val status: AnswerStatusRecord = checkTypedAnswer(typedText, acceptedAnswers)

            onSubmit(ExerciseResult(status = status, answerText = answerText))
        }
    }

    LaunchedEffect(focusRequester) {
        if (!isAnswered) {
            focusRequester.requestFocus()
        }
    }

    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd)) {
        prompt()
        AnswerField(text = typedText, onTextChange = { text -> typedText = text }, isAnswered = isAnswered, focusRequester = focusRequester, onDone = check)
        SquareTextButton(
            text = "Check",
            onClick = check,
            isPrimary = true,
            enabled = !isAnswered && typedText.isNotBlank(),
            modifier = Modifier.testTag("check"),
        )
    }
}

@Composable
private fun AnswerField(text: String, onTextChange: (String) -> Unit, isAnswered: Boolean, focusRequester: FocusRequester, onDone: () -> Unit) {
    BasicTextField(
        value = text,
        onValueChange = onTextChange,
        enabled = !isAnswered,
        singleLine = true,
        textStyle = LanguageType.targetText.copy(color = LanguageColors.ink),
        cursorBrush = SolidColor(LanguageColors.ink),
        keyboardOptions = KeyboardOptions(capitalization = KeyboardCapitalization.None, autoCorrectEnabled = false, imeAction = ImeAction.Done),
        keyboardActions = KeyboardActions(onDone = { onDone() }),
        modifier = Modifier
            .fillMaxWidth()
            .focusRequester(focusRequester)
            .testTag("answer-field"),
        decorationBox = { innerTextField: @Composable () -> Unit ->
            Box(modifier = Modifier.border(LanguageSpacing.hairline, LanguageColors.ink).padding(LanguageSpacing.spaceSm)) {
                if (text.isEmpty()) {
                    LanguageText(text = "Type your answer", style = LanguageType.targetText, color = LanguageColors.inkMuted, maxLines = 1)
                }

                innerTextField()
            }
        },
    )
}

@Composable
private fun FillBlankView(exercise: ExerciseRecord.FillBlank, completedSentence: String, onSubmit: (ExerciseResult) -> Unit) {
    TypedAnswerView(acceptedAnswers = exercise.answers, answerText = completedSentence, onSubmit = onSubmit) {
        Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs)) {
            LanguageText(text = exercise.hintSource, style = LanguageType.captionText, color = LanguageColors.inkMuted)
            LanguageText(text = exercise.promptTarget.replace(fillBlankMarker(), BLANK_SHOWN), style = LanguageType.targetText)
        }
    }
}

@Composable
private fun OrderWordsView(exercise: ExerciseRecord.OrderWords, onSubmit: (ExerciseResult) -> Unit) {
    val sentence: String = exercise.words.joinToString(" ")
    val tokens: List<String> = rememberSaveable { shuffledAvoidingIdentity(exercise.words + exercise.distractors) }
    /* Indexes into `tokens`, in the order placed. */
    var placedTokenIds: List<Int> by rememberSaveable { mutableStateOf(emptyList()) }
    var isAnswered: Boolean by rememberSaveable { mutableStateOf(false) }
    val check: () -> Unit = {
        if (!isAnswered) {
            isAnswered = true

            val attempt: String = placedTokenIds.joinToString(" ") { tokenId -> tokens[tokenId] }

            onSubmit(ExerciseResult.fromChoice(chosenCorrectly = attempt == sentence, answerText = sentence))
        }
    }

    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd)) {
        LanguageText(text = exercise.promptSource, style = LanguageType.targetText)

        FlowRow(
            modifier = Modifier
                .fillMaxWidth()
                .defaultMinSize(minHeight = LanguageSpacing.answerAreaMinimumHeight)
                .dashedHairlineBorder()
                .padding(LanguageSpacing.spaceSm)
                .testTag("answer-area"),
            horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm),
            verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm),
        ) {
            for (tokenId in placedTokenIds) {
                TokenButton(text = tokens[tokenId], enabled = !isAnswered, onClick = { placedTokenIds = placedTokenIds - tokenId })
            }
        }

        FlowRow(
            horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm),
            verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm),
        ) {
            for (tokenId in tokens.indices) {
                TokenButton(
                    text = tokens[tokenId],
                    enabled = !isAnswered && tokenId !in placedTokenIds,
                    onClick = { placedTokenIds = placedTokenIds + tokenId },
                    modifier = Modifier.testTag("bank-token"),
                )
            }
        }

        SquareTextButton(
            text = "Check",
            onClick = check,
            isPrimary = true,
            enabled = !isAnswered && placedTokenIds.isNotEmpty(),
            modifier = Modifier.testTag("check"),
        )
    }
}

@Composable
private fun TokenButton(text: String, enabled: Boolean, onClick: () -> Unit, modifier: Modifier = Modifier) {
    SquareButton(onClick = onClick, isCompact = true, enabled = enabled, modifier = modifier) { contentColor ->
        LanguageText(text = text, style = LanguageType.targetText, color = contentColor)
    }
}

private fun Modifier.dashedHairlineBorder(): Modifier {
    return drawBehind {
        val dashLength: Float = LanguageSpacing.answerAreaDashLength.toPx()
        val stroke: Stroke = Stroke(width = LanguageSpacing.hairline.toPx(), pathEffect = PathEffect.dashPathEffect(floatArrayOf(dashLength, dashLength)))

        drawRect(color = LanguageColors.ink, style = stroke)
    }
}

/* A shuffle that leaves the words in sentence order would give the answer away. */
internal fun shuffledAvoidingIdentity(items: List<String>, random: Random = Random.Default): List<String> {
    if (items.size <= 1) {
        return items
    }

    repeat(ORDER_WORDS_SHUFFLE_ATTEMPTS) {
        val candidate: List<String> = items.shuffled(random)

        if (candidate != items) {
            return candidate
        }
    }

    return items.reversed()
}

@Composable
private fun MatchView(prompt: String, pairs: List<PhrasePairRecord>, onSubmit: (ExerciseResult) -> Unit) {
    val audioPlayer: AudioPlayer = LocalAudioPlayer.current
    val progressStore: ProgressStore = LocalProgressStore.current
    val scope: CoroutineScope = rememberCoroutineScope()
    val leftOrder: List<Int> = rememberSaveable { pairs.indices.shuffled() }
    val rightOrder: List<Int> = rememberSaveable { pairs.indices.shuffled() }
    var selectedLeft: Int? by rememberSaveable { mutableStateOf(null) }
    var selectedRight: Int? by rememberSaveable { mutableStateOf(null) }
    var matchedPairIndexes: List<Int> by rememberSaveable { mutableStateOf(emptyList()) }
    var flashingWrong: List<Int> by remember { mutableStateOf(emptyList()) }
    var mistakeCount: Int by rememberSaveable { mutableIntStateOf(0) }
    val resolveSelection: () -> Unit = resolve@{
        val leftIndex: Int = selectedLeft
            ?: return@resolve
        val rightIndex: Int = selectedRight
            ?: return@resolve

        selectedLeft = null
        selectedRight = null

        if (leftIndex != rightIndex) {
            mistakeCount += 1
            flashingWrong = listOf(leftIndex, rightIndex)
            scope.launch {
                delay(MATCH_WRONG_FLASH_MILLISECONDS)
                flashingWrong = emptyList()
            }

            return@resolve
        }

        matchedPairIndexes = matchedPairIndexes + leftIndex

        if (matchedPairIndexes.size == pairs.size) {
            val answerText: String = pairs.joinToString(MATCH_PAIR_SEPARATOR) { pair -> "${pair.target} = ${pair.source}" }

            onSubmit(ExerciseResult.fromChoice(chosenCorrectly = mistakeCount == 0, answerText = answerText))
        }
    }

    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd)) {
        LanguageText(text = prompt, style = LanguageType.bodyText)

        Row(horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm), verticalAlignment = Alignment.Top) {
            Column(modifier = Modifier.weight(1f), verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
                for (pairIndex in leftOrder) {
                    val pair: PhrasePairRecord = pairs[pairIndex]
                    val clip: AudioClip = AudioClip(clipKey = pair.clipKey, text = pair.target)

                    MatchTargetChoice(
                        clip = clip,
                        mark = matchLeftMark(pairIndex = pairIndex, matchedPairIndexes = matchedPairIndexes, selectedLeft = selectedLeft, flashingWrong = flashingWrong),
                        enabled = pairIndex !in matchedPairIndexes,
                        onClick = {
                            selectedLeft = pairIndex
                            audioPlayer.play(clip, progressStore.settings().audioEngine, false, PlaybackControlKind.NormalSpeed)
                            resolveSelection()
                        },
                    )
                }
            }

            Column(modifier = Modifier.weight(1f), verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
                for (pairIndex in rightOrder) {
                    ChoiceButton(
                        mark = matchRightMark(pairIndex = pairIndex, matchedPairIndexes = matchedPairIndexes, selectedRight = selectedRight),
                        onClick = {
                            selectedRight = pairIndex
                            resolveSelection()
                        },
                        enabled = pairIndex !in matchedPairIndexes,
                        modifier = Modifier.testTag("match-right"),
                    ) { contentColor ->
                        LanguageText(text = pairs[pairIndex].source, style = LanguageType.bodyText, color = contentColor)
                    }
                }
            }
        }
    }
}

/** A target phrase to match, showing the pending glyph while its recording is awaited. */
@Composable
private fun MatchTargetChoice(clip: AudioClip, mark: ChoiceMarkKind, enabled: Boolean, onClick: () -> Unit) {
    val isPending: Boolean = LocalAudioPlayer.current.isIndicatedPending(clip, PlaybackControlKind.NormalSpeed)

    Box {
        ChoiceButton(
            mark = mark,
            onClick = onClick,
            enabled = enabled,
            modifier = Modifier
                .pendingAccessibility(isIndicated = isPending, value = PLAY_BUTTON_PENDING_ACCESSIBILITY_VALUE)
                .testTag("match-left"),
        ) { contentColor ->
            LanguageText(text = clip.text, style = LanguageType.bodyText, color = contentColor)
        }

        if (isPending) {
            PendingGlyph(
                color = LanguageColors.ink,
                size = LanguageSpacing.spaceSm + LanguageSpacing.spaceXs,
                modifier = Modifier
                    .align(Alignment.TopEnd)
                    .padding(top = LanguageSpacing.spaceXs / 2, end = LanguageSpacing.spaceXs),
            )
        }
    }
}

private fun matchLeftMark(pairIndex: Int, matchedPairIndexes: List<Int>, selectedLeft: Int?, flashingWrong: List<Int>): ChoiceMarkKind {
    if (pairIndex in matchedPairIndexes) {
        return ChoiceMarkKind.Right
    }

    if (selectedLeft == pairIndex) {
        return ChoiceMarkKind.Selected
    }

    return if (pairIndex in flashingWrong) ChoiceMarkKind.Wrong else ChoiceMarkKind.Unmarked
}

private fun matchRightMark(pairIndex: Int, matchedPairIndexes: List<Int>, selectedRight: Int?): ChoiceMarkKind {
    if (pairIndex in matchedPairIndexes) {
        return ChoiceMarkKind.Right
    }

    return if (selectedRight == pairIndex) ChoiceMarkKind.Selected else ChoiceMarkKind.Unmarked
}
