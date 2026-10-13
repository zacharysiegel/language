package ro.zach.language.progressdata

import androidx.compose.runtime.ProvidableCompositionLocal
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableLongStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.compose.runtime.staticCompositionLocalOf
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import ro.zach.language.core.ExerciseRecord
import ro.zach.language.core.PausedPracticeRecord
import ro.zach.language.core.PausedPracticeStateRecord
import ro.zach.language.core.PracticeModeRecord
import ro.zach.language.core.PracticeOutcomeRecord
import ro.zach.language.core.ProgressBook
import ro.zach.language.core.QueuedCardsRecord
import ro.zach.language.core.SavedExerciseResultRecord
import ro.zach.language.core.SettingsRecord
import ro.zach.language.core.UnitProgressRecord
import ro.zach.language.core.UnitStatusRecord
import ro.zach.language.coursedata.CoreEnvironment
import ro.zach.language.diagnostics.CoreErrorReport

/* The device's clock can change while the store waits for the next card to come due. */
private const val REVIEW_CLOCK_MAXIMUM_WAIT_MILLISECONDS: Long = 3_600_000

val LocalProgressStore: ProvidableCompositionLocal<ProgressStore> = staticCompositionLocalOf { error("no ProgressStore provided") }

/** Whether a change to the progress was saved. */
enum class ProgressChangeOutcome {
    Saved,
    Failed,
}

/**
 * The learner's progress. Every read registers with the snapshot system, so composables recompose after any change.
 * Call from the main thread.
 */
class ProgressStore(environment: CoreEnvironment, private val scope: CoroutineScope) {
    private val book: ProgressBook = ProgressBook.open(environment.progressPath)
    private var changeCount: Int by mutableIntStateOf(0)
    private var errorMessage: String? by mutableStateOf(null)
    /* The time every due read is taken at. */
    private var reviewClockMilliseconds: Long by mutableLongStateOf(0)
    private var reviewClockAdvance: Job? = null

    init {
        advanceReviewClock()
    }

    fun settings(): SettingsRecord {
        readChangeCount()
        return book.settings()
    }

    fun updateSettings(change: (SettingsRecord) -> SettingsRecord): ProgressChangeOutcome {
        val settings: SettingsRecord = change(book.settings())

        return perform { book.updateSettings(settings) }
    }

    fun unitProgress(courseId: String, unitId: String): UnitProgressRecord {
        readChangeCount()
        return book.unitProgress(courseId, unitId)
    }

    fun completedUnitCount(courseId: String): Int {
        readChangeCount()
        return book.completedUnitCount(courseId).toInt()
    }

    fun dueCardCount(courseId: String): Int {
        readChangeCount()
        return book.dueCardCount(courseId, reviewClockMilliseconds).toInt()
    }

    fun cardCount(courseId: String): Int {
        readChangeCount()
        return book.cardCount(courseId).toInt()
    }

    fun dueCardIds(courseId: String, limit: Int): List<String> {
        readChangeCount()
        return book.dueCardIds(courseId, limit.toUInt(), reviewClockMilliseconds)
    }

    fun countByBox(courseId: String): List<Int> {
        readChangeCount()
        return book.countByBox(courseId).map { count -> count.toInt() }
    }

    /** Moves the time due reads are taken at to the present, and again when the next card comes due. */
    fun advanceReviewClock() {
        val now: Long = System.currentTimeMillis()

        reviewClockMilliseconds = now
        scheduleReviewClockAdvance(now)
    }

    /** The course the learner opened last. */
    fun lastCourseId(): String? {
        return settings().lastCourseId
    }

    fun rememberCourse(courseId: String) {
        if (settings().lastCourseId == courseId) {
            return
        }

        updateSettings { settings -> settings.copy(lastCourseId = courseId) }
    }

    fun lastError(): String? {
        return errorMessage
    }

    fun recordPracticeSession(
        courseId: String,
        unitId: String,
        mode: PracticeModeRecord,
        correctCount: Int,
        totalCount: Int,
        vocabularyCount: Int,
    ): PracticeOutcomeRecord? {
        var outcome: PracticeOutcomeRecord? = null

        perform {
            outcome = book.recordPracticeSession(courseId, unitId, mode, correctCount.toUInt(), totalCount.toUInt(), vocabularyCount.toUInt())
        }

        return outcome
    }

    fun pausedPracticeState(courseId: String, unitId: String, exercises: List<ExerciseRecord>): PausedPracticeStateRecord {
        readChangeCount()
        return book.pausedPracticeState(courseId, unitId, exercises)
    }

    /** Replaces any paused practice of the unit. */
    fun startFullPractice(courseId: String, unitId: String, exercises: List<ExerciseRecord>): PausedPracticeRecord? {
        var pausedPractice: PausedPracticeRecord? = null

        perform { pausedPractice = book.startFullPractice(courseId, unitId, exercises) }

        return pausedPractice
    }

    fun recordFullPracticeResult(courseId: String, unitId: String, result: SavedExerciseResultRecord): ProgressChangeOutcome {
        return perform { book.recordFullPracticeResult(courseId, unitId, result) }
    }

    fun clearPausedPractice(courseId: String, unitId: String): ProgressChangeOutcome {
        return perform { book.clearPausedPractice(courseId, unitId) }
    }

    /** Records the unit's paused practice as a full practice session; `null` while it has unanswered questions or when the unit has none. */
    fun finishFullPractice(courseId: String, unitId: String, vocabularyCount: Int): PracticeOutcomeRecord? {
        var outcome: PracticeOutcomeRecord? = null

        perform { outcome = book.finishFullPractice(courseId, unitId, vocabularyCount.toUInt()) }

        return outcome
    }

    /** Moves a not started unit to in progress and leaves any other status alone. */
    fun startUnit(courseId: String, unitId: String) {
        if (unitProgress(courseId, unitId).status != UnitStatusRecord.NOT_STARTED) {
            return
        }

        perform { book.startUnit(courseId, unitId) }
    }

    /** Keeps the best score and the attempt count. */
    fun setUnitStatus(courseId: String, unitId: String, status: UnitStatusRecord): ProgressChangeOutcome {
        return perform { book.setUnitStatus(courseId, unitId, status) }
    }

    fun queueUnitVocabulary(courseId: String, unitId: String, vocabularyCount: Int): QueuedCardsRecord? {
        var queued: QueuedCardsRecord? = null

        perform { queued = book.queueUnitVocabulary(courseId, unitId, vocabularyCount.toUInt()) }

        return queued
    }

    fun gradeCard(courseId: String, cardId: String, remembered: Boolean): ProgressChangeOutcome {
        return perform { book.gradeCard(courseId, cardId, remembered) }
    }

    fun exportJson(): String? {
        try {
            return book.exportJson()
        } catch (error: Exception) {
            errorMessage = CoreErrorReport.record(error, "exporting progress")
            return null
        }
    }

    fun importJson(text: String): ProgressChangeOutcome {
        return perform { book.importJson(text) }
    }

    fun reset(): ProgressChangeOutcome {
        return perform { book.reset() }
    }

    /** Releases the core's progress book; the store does nothing after. */
    fun close() {
        reviewClockAdvance?.cancel()
        book.close()
    }

    /* Registers the caller with the snapshot system, so it recomposes after any change. */
    private fun readChangeCount() {
        changeCount
    }

    private fun perform(change: () -> Unit): ProgressChangeOutcome {
        val outcome: ProgressChangeOutcome = try {
            change()
            errorMessage = null
            ProgressChangeOutcome.Saved
        } catch (error: Exception) {
            errorMessage = CoreErrorReport.record(error, "saving progress")
            ProgressChangeOutcome.Failed
        }

        changeCount += 1
        advanceReviewClock()

        return outcome
    }

    private fun scheduleReviewClockAdvance(now: Long) {
        val waitMilliseconds: Long = reviewClockWaitMilliseconds(now)

        reviewClockAdvance?.cancel()
        reviewClockAdvance = scope.launch {
            delay(waitMilliseconds)
            advanceReviewClock()
        }
    }

    private fun reviewClockWaitMilliseconds(now: Long): Long {
        val nextDueMilliseconds: Long? = book.nextDueMillisecondsAfter(now)
        val untilNextDue: Long = if (nextDueMilliseconds == null) REVIEW_CLOCK_MAXIMUM_WAIT_MILLISECONDS else nextDueMilliseconds - now

        return minOf(untilNextDue, REVIEW_CLOCK_MAXIMUM_WAIT_MILLISECONDS)
    }
}
