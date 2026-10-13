package ro.zach.language.unit

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import ro.zach.language.core.ExerciseRecord
import ro.zach.language.core.PausedPracticeRecord
import ro.zach.language.core.PausedPracticeStateRecord
import ro.zach.language.core.PracticeModeRecord
import ro.zach.language.core.PracticeOutcomeRecord
import ro.zach.language.core.SessionScoreRecord
import ro.zach.language.core.UnitLessonRecord
import ro.zach.language.core.UnitOutlineRecord
import ro.zach.language.core.UnitProgressRecord
import ro.zach.language.core.UnitStatusRecord
import ro.zach.language.core.passPercent
import ro.zach.language.core.quickPracticeExerciseCount
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType
import ro.zach.language.design.SquareTextButton
import ro.zach.language.practice.ExerciseResult
import ro.zach.language.practice.ExerciseSession
import ro.zach.language.practice.ExerciseSessionExitKind
import ro.zach.language.progressdata.LocalProgressStore
import ro.zach.language.progressdata.ProgressStore
import kotlin.math.min
import kotlin.math.roundToInt

const val CHANGED_PRACTICE_NOTE: String = "The practice changed since you paused, so it starts over."

/** A unit's best score and its full and quick practice sessions. */
@Composable
fun PracticeSection(courseId: String, unit: UnitOutlineRecord, lesson: UnitLessonRecord, modifier: Modifier = Modifier) {
    val progressStore: ProgressStore = LocalProgressStore.current
    var activeMode: PracticeModeRecord? by rememberSaveable { mutableStateOf(null) }
    /* Indexes into the lesson's exercises, in the order the session asks them. */
    var sessionExerciseIndexes: List<Int> by rememberSaveable { mutableStateOf(emptyList()) }
    var answeredResults: List<ExerciseResult> by rememberSaveable { mutableStateOf(emptyList()) }
    var noticeMessage: String? by rememberSaveable { mutableStateOf(null) }
    val open: (PracticeModeRecord, List<Int>, List<ExerciseResult>) -> Unit = { mode: PracticeModeRecord, exerciseIndexes: List<Int>, results: List<ExerciseResult> ->
        sessionExerciseIndexes = exerciseIndexes
        answeredResults = results
        noticeMessage = null
        activeMode = mode
        progressStore.startUnit(courseId, unit.id)
    }
    val mode: PracticeModeRecord? = activeMode

    if (mode != null) {
        ExerciseSession(
            title = "Practice",
            exercises = sessionExerciseIndexes.mapNotNull { exerciseIndex -> lesson.exercises.getOrNull(exerciseIndex) },
            answeredResults = answeredResults,
            exitKind = exitKind(mode),
            onAnswer = { result -> recordAnswer(progressStore, courseId, unit.id, mode, result) },
            onComplete = { score ->
                val outcome: PracticeOutcomeRecord? = recordOutcome(progressStore, courseId, unit, mode, score)

                if (outcome != null && outcome.wordsAddedCount > 0u) {
                    noticeMessage = "${outcome.wordsAddedCount} word(s) from this unit were added to your review deck."
                }
            },
            onExit = { activeMode = null },
            modifier = modifier,
        )

        return
    }

    LaunchedEffect(Unit) {
        val dropOutcome: PausedPracticeDropOutcome = dropThePausedPracticeIfTheExercisesChanged(progressStore, courseId, unit.id, lesson.exercises)

        if (dropOutcome == PausedPracticeDropOutcome.Dropped) {
            noticeMessage = CHANGED_PRACTICE_NOTE
        }
    }

    Column(modifier = modifier, verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
        LanguageText(text = bestSummary(progressStore.unitProgress(courseId, unit.id)), style = LanguageType.bodyText)
        LanguageText(text = "Pass mark: ${passPercent()}%.", style = LanguageType.captionText, color = LanguageColors.inkMuted)

        noticeMessage?.let { message: String ->
            LanguageText(text = message, style = LanguageType.captionText, color = LanguageColors.inkMuted, modifier = Modifier.testTag("practice-notice"))
        }

        PracticeStartButtons(
            pausedPracticeState = progressStore.pausedPracticeState(courseId, unit.id, lesson.exercises),
            exerciseCount = lesson.exercises.size,
            onStartFull = {
                val pausedPractice: PausedPracticeRecord? = progressStore.startFullPractice(courseId, unit.id, lesson.exercises)

                if (pausedPractice != null) {
                    open(PracticeModeRecord.FULL, exerciseIndexes(pausedPractice), emptyList())
                }
            },
            onResume = { pausedPractice: PausedPracticeRecord ->
                val savedResults: List<ExerciseResult> = pausedPractice.results.map { savedResult -> ExerciseResult.fromSaved(savedResult) }

                open(PracticeModeRecord.FULL, exerciseIndexes(pausedPractice), savedResults)
            },
            onStartQuick = { open(PracticeModeRecord.QUICK, quickPracticeExerciseIndexes(lesson.exercises), emptyList()) },
        )
    }
}

@Composable
private fun PracticeStartButtons(
    pausedPracticeState: PausedPracticeStateRecord,
    exerciseCount: Int,
    onStartFull: () -> Unit,
    onResume: (PausedPracticeRecord) -> Unit,
    onStartQuick: () -> Unit,
) {
    when (pausedPracticeState) {
        is PausedPracticeStateRecord.Resumable -> {
            val pausedPractice: PausedPracticeRecord = pausedPracticeState.pausedPractice

            SquareTextButton(
                text = "Resume full practice (question ${resumedQuestionNumber(pausedPractice)} of ${pausedPractice.questionCount})",
                onClick = { onResume(pausedPractice) },
                isPrimary = true,
                modifier = Modifier.testTag("resume-full-practice"),
            )
            SquareTextButton(text = "Start over", onClick = onStartFull, modifier = Modifier.testTag("start-over"))
        }
        PausedPracticeStateRecord.NotPaused, PausedPracticeStateRecord.ExercisesChanged -> SquareTextButton(
            text = "Full practice ($exerciseCount exercises)",
            onClick = onStartFull,
            isPrimary = true,
            modifier = Modifier.testTag("full-practice"),
        )
    }

    SquareTextButton(
        text = "Quick practice (${quickPracticeExerciseCount()} random exercises)",
        onClick = onStartQuick,
        modifier = Modifier.testTag("quick-practice"),
    )
}

private enum class PausedPracticeDropOutcome {
    Dropped,
    Kept,
}

private fun dropThePausedPracticeIfTheExercisesChanged(
    progressStore: ProgressStore,
    courseId: String,
    unitId: String,
    exercises: List<ExerciseRecord>,
): PausedPracticeDropOutcome {
    val pausedPracticeState: PausedPracticeStateRecord = progressStore.pausedPracticeState(courseId, unitId, exercises)

    if (pausedPracticeState != PausedPracticeStateRecord.ExercisesChanged) {
        return PausedPracticeDropOutcome.Kept
    }

    progressStore.clearPausedPractice(courseId, unitId)

    return PausedPracticeDropOutcome.Dropped
}

/* A practice left on its last feedback has every question answered and resumes at its summary. */
private fun resumedQuestionNumber(pausedPractice: PausedPracticeRecord): UInt {
    return min(pausedPractice.nextQuestionIndex + 1u, pausedPractice.questionCount)
}

private fun exerciseIndexes(pausedPractice: PausedPracticeRecord): List<Int> {
    return pausedPractice.exerciseOrder.map { exerciseIndex -> exerciseIndex.toInt() }
}

private fun quickPracticeExerciseIndexes(exercises: List<ExerciseRecord>): List<Int> {
    return exercises.indices.shuffled().take(quickPracticeExerciseCount().toInt())
}

private fun exitKind(mode: PracticeModeRecord): ExerciseSessionExitKind {
    return when (mode) {
        PracticeModeRecord.FULL -> ExerciseSessionExitKind.Pause
        PracticeModeRecord.QUICK -> ExerciseSessionExitKind.Quit
    }
}

private fun recordAnswer(progressStore: ProgressStore, courseId: String, unitId: String, mode: PracticeModeRecord, result: ExerciseResult) {
    when (mode) {
        PracticeModeRecord.FULL -> progressStore.recordFullPracticeResult(courseId, unitId, result.savedResult())
        PracticeModeRecord.QUICK -> Unit
    }
}

private fun recordOutcome(progressStore: ProgressStore, courseId: String, unit: UnitOutlineRecord, mode: PracticeModeRecord, score: SessionScoreRecord): PracticeOutcomeRecord? {
    return when (mode) {
        PracticeModeRecord.FULL -> progressStore.finishFullPractice(courseId, unit.id, unit.vocabulary.size)
        PracticeModeRecord.QUICK -> progressStore.recordPracticeSession(
            courseId = courseId,
            unitId = unit.id,
            mode = PracticeModeRecord.QUICK,
            correctCount = score.correctCount.toInt(),
            totalCount = score.totalCount.toInt(),
            vocabularyCount = unit.vocabulary.size,
        )
    }
}

private fun bestSummary(unitProgress: UnitProgressRecord): String {
    if (unitProgress.attemptCount == 0u) {
        return "You have not attempted this unit yet."
    }

    val percent: Int = (unitProgress.bestProportion * 100).roundToInt()
    val completion: String = if (unitProgress.status == UnitStatusRecord.COMPLETED) " Unit completed." else ""

    return "Best score $percent% over ${unitProgress.attemptCount} attempt(s).$completion"
}
