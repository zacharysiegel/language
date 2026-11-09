package ro.zach.language.practice

import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.key
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusManager
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.isTraversalGroup
import androidx.compose.ui.semantics.semantics
import ro.zach.language.audio.AudioClip
import ro.zach.language.audio.AudioPlayer
import ro.zach.language.audio.LocalAudioPlayer
import ro.zach.language.audio.PlayButton
import ro.zach.language.core.ExerciseRecord
import ro.zach.language.core.exerciseInstruction
import ro.zach.language.core.exercisePromptText
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType
import ro.zach.language.design.MicroLabel
import ro.zach.language.design.SquareTextButton
import ro.zach.language.design.SquareTextField
import ro.zach.language.design.panel

private const val OPEN_RESPONSE_LINE_COUNT: Int = 4
private const val SELF_CHECK_HEADING: String = "Compare with the examples"
private const val SELF_CHECK_NOTICE: String = "Automatic grading is not available on this device. In supported web browsers on desktop computers, an AI model checks your answer and points out each mistake."

/** Open responses the learner checks against their example answers; it keeps no score and records nothing. */
@Composable
fun OpenResponseSession(title: String, exercises: List<ExerciseRecord.OpenResponse>, onExit: () -> Unit, modifier: Modifier = Modifier) {
    /* The answers written so far, in the order the exercises are asked. */
    var writtenAnswers: List<String> by rememberSaveable { mutableStateOf(emptyList()) }
    var currentPosition: Int by rememberSaveable { mutableIntStateOf(0) }
    val currentExercise: ExerciseRecord.OpenResponse? = exercises.getOrNull(currentPosition)
    val audioPlayer: AudioPlayer = LocalAudioPlayer.current
    val exit: () -> Unit = {
        audioPlayer.stop()
        onExit()
    }

    Column(modifier = modifier.panel(), verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd)) {
        if (currentExercise != null) {
            val writtenAnswer: String? = writtenAnswers.getOrNull(currentPosition)

            Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd)) {
                SessionHeader(title = title, position = currentPosition, total = exercises.size, exitKind = ExerciseSessionExitKind.Quit, onExit = exit)
                MicroLabel(text = exerciseInstruction(currentExercise))

                key(currentPosition) {
                    OpenResponseQuestion(
                        exercise = currentExercise,
                        isAnswered = writtenAnswer != null,
                        onCheck = { answerText: String ->
                            if (writtenAnswers.size == currentPosition) {
                                writtenAnswers = writtenAnswers + answerText
                            }
                        },
                    )
                }

                if (writtenAnswer != null) {
                    SelfCheckFeedback(exercise = currentExercise, writtenAnswer = writtenAnswer, onContinue = { currentPosition += 1 })
                }
            }
        } else {
            YourAnswersSummary(title = title, exercises = exercises, writtenAnswers = writtenAnswers, onDone = onExit)
        }
    }
}

@Composable
private fun OpenResponseQuestion(exercise: ExerciseRecord.OpenResponse, isAnswered: Boolean, onCheck: (String) -> Unit) {
    var typedText: String by rememberSaveable { mutableStateOf("") }
    val focusRequester: FocusRequester = remember { FocusRequester() }
    val focusManager: FocusManager = LocalFocusManager.current
    val check: () -> Unit = {
        val answerText: String = typedText.trim()

        if (!isAnswered && answerText.isNotEmpty()) {
            focusManager.clearFocus()
            onCheck(answerText)
        }
    }

    LaunchedEffect(focusRequester) {
        if (!isAnswered) {
            focusRequester.requestFocus()
        }
    }

    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd)) {
        LanguageText(text = exercise.promptSource, style = LanguageType.targetText)

        exercise.contextSource?.let { contextSource: String ->
            LanguageText(text = contextSource, style = LanguageType.captionText, color = LanguageColors.inkMuted)
        }

        SquareTextField(
            value = typedText,
            onValueChange = { text -> typedText = text },
            placeholder = "Type your answer",
            singleLine = false,
            minLines = OPEN_RESPONSE_LINE_COUNT,
            enabled = !isAnswered,
            modifier = Modifier
                .focusRequester(focusRequester)
                .testTag("open-response-field"),
        )
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
private fun SelfCheckFeedback(exercise: ExerciseRecord.OpenResponse, writtenAnswer: String, onContinue: () -> Unit) {
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .border(LanguageSpacing.hairline, LanguageColors.ink)
            .padding(LanguageSpacing.spaceSm + LanguageSpacing.spaceXs),
        verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm),
    ) {
        SelfCheckHeading()
        MicroLabel(text = "Your answer")
        LanguageText(text = writtenAnswer, style = LanguageType.bodyText, modifier = Modifier.testTag("written-answer"))
        MicroLabel(text = "Example answers")
        ExampleAnswers(exercise = exercise)

        exercise.explanation?.let { explanation: String ->
            LanguageText(text = explanation, style = LanguageType.captionText, color = LanguageColors.inkMuted)
        }

        SquareTextButton(text = "Continue", onClick = onContinue, isPrimary = true, modifier = Modifier.testTag("continue-exercise"))
    }
}

@Composable
private fun SelfCheckHeading() {
    Column(modifier = Modifier.semantics { isTraversalGroup = true }, verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs)) {
        LanguageText(text = SELF_CHECK_HEADING, style = LanguageType.titleText, modifier = Modifier.testTag("feedback-heading"))
        LanguageText(text = SELF_CHECK_NOTICE, style = LanguageType.captionText, color = LanguageColors.inkMuted, modifier = Modifier.testTag("self-check-notice"))
    }
}

@Composable
private fun ExampleAnswers(exercise: ExerciseRecord.OpenResponse) {
    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs)) {
        exercise.exampleAnswers.forEachIndexed { exampleIndex: Int, exampleAnswer: String ->
            Row(horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm), verticalAlignment = Alignment.CenterVertically) {
                LanguageText(text = exampleAnswer, style = LanguageType.bodyText, modifier = Modifier.weight(1f, fill = false).testTag("example-answer"))

                /* The exercise's clip speaks the first example answer. */
                if (exampleIndex == 0) {
                    PlayButton(clip = AudioClip(clipKey = exercise.clipKey, text = exampleAnswer), label = "Hear it")
                }
            }
        }
    }
}

@Composable
private fun YourAnswersSummary(title: String, exercises: List<ExerciseRecord.OpenResponse>, writtenAnswers: List<String>, onDone: () -> Unit) {
    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
        LanguageText(text = title, style = LanguageType.titleText, modifier = Modifier.testTag("session-summary"))
        MicroLabel(text = "Your answers", modifier = Modifier.testTag("your-answers"))

        for ((exercise, writtenAnswer) in exercises.zip(writtenAnswers)) {
            Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.phraseLineGap)) {
                LanguageText(text = exercisePromptText(exercise), style = LanguageType.bodyText)
                LanguageText(text = writtenAnswer, style = LanguageType.captionText, color = LanguageColors.inkMuted, modifier = Modifier.testTag("your-answer"))
            }
        }

        SquareTextButton(text = "Done", onClick = onDone, isPrimary = true, modifier = Modifier.testTag("session-done"))
    }
}
