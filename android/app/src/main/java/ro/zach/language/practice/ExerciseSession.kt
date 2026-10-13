package ro.zach.language.practice

import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.key
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.testTag
import ro.zach.language.core.AnswerStatusRecord
import ro.zach.language.core.ExerciseRecord
import ro.zach.language.core.SessionScoreRecord
import ro.zach.language.core.exerciseInstruction
import ro.zach.language.core.exercisePromptText
import ro.zach.language.core.exerciseSpokenText
import ro.zach.language.core.passPercent
import ro.zach.language.core.sessionScore
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType
import ro.zach.language.design.MicroLabel
import ro.zach.language.design.SquareProgressBar
import ro.zach.language.design.SquareTextButton
import ro.zach.language.design.TextLink
import ro.zach.language.design.panel
import ro.zach.language.progressdata.LocalProgressStore
import ro.zach.language.progressdata.ProgressStore

/** What leaving a session before its summary does to it. */
enum class ExerciseSessionExitKind {
    /** Discards the session. */
    Quit,

    /** Keeps the session to be resumed. */
    Pause;

    fun label(): String {
        return when (this) {
            Quit -> "Quit"
            Pause -> "Pause"
        }
    }

    fun testTag(): String {
        return when (this) {
            Quit -> "quit"
            Pause -> "pause"
        }
    }
}

/**
 * Runs exercises one at a time with feedback, then a summary that offers to retry the missed ones.
 * `answeredResults` are the results of the first exercises, in order; the session continues after them.
 * `exitKind` applies until the summary, since the retry of missed exercises can only be quit, and `onAnswer` is not called in that retry.
 */
@Composable
fun ExerciseSession(
    title: String,
    exercises: List<ExerciseRecord>,
    answeredResults: List<ExerciseResult>,
    exitKind: ExerciseSessionExitKind,
    onAnswer: (ExerciseResult) -> Unit,
    onComplete: (SessionScoreRecord) -> Unit,
    onExit: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val sessionState: ExerciseSessionState = rememberSaveable(saver = ExerciseSessionState.saver(exercises.size)) {
        ExerciseSessionState.start(exerciseCount = exercises.size, answeredResults = answeredResults)
    }
    val currentExercise: ExerciseRecord? = sessionState.currentExerciseIndex()?.let { exerciseIndex -> exercises.getOrNull(exerciseIndex) }
    val audioPlayer: AudioPlayer = LocalAudioPlayer.current
    val progressStore: ProgressStore = LocalProgressStore.current
    val submit: (ExerciseResult) -> Unit = { result: ExerciseResult ->
        val outcome: ExerciseSubmissionOutcome = sessionState.submit(result)

        if (outcome == ExerciseSubmissionOutcome.Recorded) {
            if (!sessionState.isRetry()) {
                onAnswer(result)
            }

            currentExercise?.let { exercise: ExerciseRecord -> speakTheAnswer(exercise, audioPlayer, progressStore) }
        }
    }
    val exit: () -> Unit = {
        audioPlayer.stop()
        onExit()
    }

    Column(modifier = modifier.panel(), verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd)) {
        if (currentExercise != null) {
            QuestionView(title = title, exercise = currentExercise, sessionState = sessionState, exitKind = exitKind, onSubmit = submit, onExit = exit)
        } else {
            SummaryView(title = title, exercises = exercises, sessionState = sessionState, onComplete = onComplete, onExit = onExit)
        }
    }
}

@Composable
private fun QuestionView(
    title: String,
    exercise: ExerciseRecord,
    sessionState: ExerciseSessionState,
    exitKind: ExerciseSessionExitKind,
    onSubmit: (ExerciseResult) -> Unit,
    onExit: () -> Unit,
) {
    val shownExitKind: ExerciseSessionExitKind = if (sessionState.isRetry()) ExerciseSessionExitKind.Quit else exitKind
    val shownTitle: String = if (sessionState.isRetry()) "$title, retry of missed" else title
    val total: Int = sessionState.askedExerciseIndexes().size
    val currentResult: ExerciseResult? = sessionState.currentResult()

    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd)) {
        Row(modifier = Modifier.fillMaxWidth(), verticalAlignment = Alignment.CenterVertically) {
            LanguageText(
                text = "$shownTitle: question ${sessionState.currentPosition() + 1} of $total",
                style = LanguageType.captionText,
                color = LanguageColors.inkMuted,
                modifier = Modifier.weight(1f).testTag("question-position"),
            )
            TextLink(text = shownExitKind.label(), onClick = onExit, modifier = Modifier.testTag(shownExitKind.testTag()))
        }

        SquareProgressBar(value = sessionState.currentPosition(), total = total)
        MicroLabel(text = exerciseInstruction(exercise))

        key(sessionState.isRetry(), sessionState.currentPosition()) {
            ExerciseBody(exercise = exercise, onSubmit = onSubmit)
        }

        if (currentResult != null) {
            FeedbackView(exercise = exercise, result = currentResult, onContinue = sessionState::advance)
        }
    }
}

@Composable
private fun FeedbackView(exercise: ExerciseRecord, result: ExerciseResult, onContinue: () -> Unit) {
    val explanation: String? = exercise.explanation()
    val spokenText: String? = exerciseSpokenText(exercise)
    val markColor: Color = if (result.isCorrect()) LanguageColors.goodText else LanguageColors.accentActive

    Column(
        modifier = Modifier
            .fillMaxWidth()
            .border(LanguageSpacing.hairline, markColor)
            .padding(LanguageSpacing.spaceSm + LanguageSpacing.spaceXs),
        verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm),
    ) {
        LanguageText(text = feedbackHeading(result.status), style = LanguageType.titleText, color = markColor, modifier = Modifier.testTag("feedback-heading"))

        if (result.status != AnswerStatusRecord.CORRECT) {
            LanguageText(text = "Answer: ${result.answerText}", style = LanguageType.bodyText)
        }

        if (explanation != null) {
            LanguageText(text = explanation, style = LanguageType.captionText, color = LanguageColors.inkMuted)
        }

        Row(horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm), verticalAlignment = Alignment.CenterVertically) {
            if (spokenText != null) {
                PlayButton(clip = AudioClip(clipKey = exercise.clipKey(), text = spokenText), label = "Hear it")
            }

            SquareTextButton(text = "Continue", onClick = onContinue, isPrimary = true, modifier = Modifier.testTag("continue-exercise"))
        }
    }
}

@Composable
private fun SummaryView(
    title: String,
    exercises: List<ExerciseRecord>,
    sessionState: ExerciseSessionState,
    onComplete: (SessionScoreRecord) -> Unit,
    onExit: () -> Unit,
) {
    val score: SessionScoreRecord = sessionScore(sessionState.correctCount().toUInt(), sessionState.askedExerciseIndexes().size.toUInt())
    val missedEntries: List<Pair<ExerciseRecord, ExerciseResult>> = missedEntries(exercises, sessionState)

    LaunchedEffect(sessionState) {
        val completion: ExerciseSessionCompletion = sessionState.takeCompletion()
            ?: return@LaunchedEffect

        onComplete(sessionScore(completion.correctCount.toUInt(), completion.totalCount.toUInt()))
    }

    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
        LanguageText(
            text = "$title: ${score.correctCount} of ${score.totalCount} correct (${score.percent}%)",
            style = LanguageType.titleText,
            modifier = Modifier.testTag("session-summary"),
        )
        LanguageText(
            text = if (score.passed) "Passed." else "Not passed yet. ${passPercent()}% is needed.",
            style = LanguageType.bodyText,
            color = if (score.passed) LanguageColors.goodText else LanguageColors.accentActive,
        )

        if (missedEntries.isNotEmpty()) {
            MicroLabel(text = "To review")

            for ((exercise, result) in missedEntries) {
                Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.phraseLineGap)) {
                    LanguageText(text = exercisePromptText(exercise), style = LanguageType.bodyText)
                    LanguageText(text = result.answerText, style = LanguageType.captionText, color = LanguageColors.goodText)
                }
            }
        }

        Row(horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
            if (missedEntries.isNotEmpty()) {
                SquareTextButton(text = "Retry missed", onClick = sessionState::retryMissed, isPrimary = true, modifier = Modifier.testTag("retry-missed"))
            }

            SquareTextButton(text = "Done", onClick = onExit, modifier = Modifier.testTag("session-done"))
        }
    }
}

private fun speakTheAnswer(exercise: ExerciseRecord, audioPlayer: AudioPlayer, progressStore: ProgressStore) {
    val spokenText: String? = exerciseSpokenText(exercise)

    if (!exercise.speaksAfterAnswer() || spokenText == null) {
        return
    }

    audioPlayer.play(AudioClip(clipKey = exercise.clipKey(), text = spokenText), progressStore.settings().audioEngine, false, PlaybackControlKind.NormalSpeed)
}

private fun missedEntries(exercises: List<ExerciseRecord>, sessionState: ExerciseSessionState): List<Pair<ExerciseRecord, ExerciseResult>> {
    val askedExercises: List<ExerciseRecord?> = sessionState.askedExerciseIndexes().map { exerciseIndex -> exercises.getOrNull(exerciseIndex) }

    return askedExercises.zip(sessionState.results())
        .filter { (_, result) -> !result.isCorrect() }
        .mapNotNull { (exercise, result) -> exercise?.let { askedExercise -> askedExercise to result } }
}

private fun feedbackHeading(status: AnswerStatusRecord): String {
    return when (status) {
        AnswerStatusRecord.CORRECT -> "Correct"
        AnswerStatusRecord.ACCENT_MISTAKE -> "Correct, but check the accents"
        AnswerStatusRecord.WRONG -> "Not quite"
    }
}
