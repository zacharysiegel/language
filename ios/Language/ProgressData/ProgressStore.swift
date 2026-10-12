import Foundation
import Observation

/// The learner's progress; every read registers with Observation, so views refresh after any change.
@MainActor
@Observable
final class ProgressStore {
    /* The device's clock can change while the store waits for the next card to come due. */
    static let reviewClockMaximumWait: Duration = .seconds(3600)

    private let book: ProgressBook
    private var changeCount: Int = 0
    private var errorMessage: String?
    /// The time every due read is taken at.
    private var reviewClockMilliseconds: Int64 = 0
    @ObservationIgnored private var reviewClockAdvance: Task<Void, Never>?

    init(environment: CoreEnvironment) {
        book = ProgressBook.open(path: environment.progressPath)
        advanceReviewClock()
    }

    func settings() -> SettingsRecord {
        _ = changeCount
        return book.settings()
    }

    func updateSettings(_ change: (inout SettingsRecord) -> Void) {
        var settings: SettingsRecord = book.settings()

        change(&settings)
        perform { try book.updateSettings(settings: settings) }
    }

    func unitProgress(courseId: String, unitId: String) -> UnitProgressRecord {
        _ = changeCount
        return book.unitProgress(courseId: courseId, unitId: unitId)
    }

    func completedUnitCount(courseId: String) -> Int {
        _ = changeCount
        return Int(book.completedUnitCount(courseId: courseId))
    }

    func dueCardCount(courseId: String) -> Int {
        _ = changeCount
        return Int(book.dueCardCount(courseId: courseId, nowMilliseconds: reviewClockMilliseconds))
    }

    func cardCount(courseId: String) -> Int {
        _ = changeCount
        return Int(book.cardCount(courseId: courseId))
    }

    func dueCardIds(courseId: String, limit: Int) -> [String] {
        _ = changeCount
        return book.dueCardIds(courseId: courseId, limit: UInt32(limit), nowMilliseconds: reviewClockMilliseconds)
    }

    func countByBox(courseId: String) -> [Int] {
        _ = changeCount
        return book.countByBox(courseId: courseId).map { count in Int(count) }
    }

    /// Moves the time due reads are taken at to the present, and again when the next card comes due.
    func advanceReviewClock() {
        let now: Int64 = Self.currentMilliseconds()

        reviewClockMilliseconds = now
        scheduleReviewClockAdvance(now: now)
    }

    /// The course the learner opened last.
    func lastCourseId() -> String? {
        settings().lastCourseId
    }

    func rememberCourse(courseId: String) {
        guard settings().lastCourseId != courseId else {
            return
        }

        updateSettings { settings in settings.lastCourseId = courseId }
    }

    func lastError() -> String? {
        errorMessage
    }

    func recordPracticeSession(courseId: String, unitId: String, mode: PracticeModeRecord, correctCount: Int, totalCount: Int, vocabularyCount: Int) -> PracticeOutcomeRecord? {
        var outcome: PracticeOutcomeRecord?

        perform {
            outcome = try book.recordPracticeSession(
                courseId: courseId,
                unitId: unitId,
                mode: mode,
                correctCount: UInt32(correctCount),
                totalCount: UInt32(totalCount),
                vocabularyCount: UInt32(vocabularyCount),
            )
        }

        return outcome
    }

    /// Moves a not started unit to in progress and leaves any other status alone.
    func startUnit(courseId: String, unitId: String) {
        guard unitProgress(courseId: courseId, unitId: unitId).status == .notStarted else {
            return
        }

        perform { try book.startUnit(courseId: courseId, unitId: unitId) }
    }

    /// Keeps the best score and the attempt count.
    func setUnitStatus(courseId: String, unitId: String, status: UnitStatusRecord) {
        perform { try book.setUnitStatus(courseId: courseId, unitId: unitId, status: status) }
    }

    func queueUnitVocabulary(courseId: String, unitId: String, vocabularyCount: Int) -> QueuedCardsRecord? {
        var queued: QueuedCardsRecord?

        perform { queued = try book.queueUnitVocabulary(courseId: courseId, unitId: unitId, vocabularyCount: UInt32(vocabularyCount)) }

        return queued
    }

    func gradeCard(courseId: String, cardId: String, remembered: Bool) {
        perform { try book.gradeCard(courseId: courseId, cardId: cardId, remembered: remembered) }
    }

    func exportJson() -> String? {
        do {
            return try book.exportJson()
        } catch {
            errorMessage = CoreErrorReport.record(error, while: "exporting progress")
            return nil
        }
    }

    func importJson(_ text: String) -> Bool {
        var imported: Bool = false

        perform {
            try book.importJson(text: text)
            imported = true
        }

        return imported
    }

    func reset() {
        perform { try book.reset() }
    }

    private func perform(_ change: () throws -> Void) {
        do {
            try change()
            errorMessage = nil
        } catch {
            errorMessage = CoreErrorReport.record(error, while: "saving progress")
        }

        changeCount += 1
        advanceReviewClock()
    }

    private func scheduleReviewClockAdvance(now: Int64) {
        let wait: Duration = reviewClockWait(now: now)

        reviewClockAdvance?.cancel()
        reviewClockAdvance = Task { [weak self] in
            do {
                try await Task.sleep(for: wait)
            } catch {
                return
            }

            self?.advanceReviewClock()
        }
    }

    private func reviewClockWait(now: Int64) -> Duration {
        let nextDueMilliseconds: Int64? = book.nextDueMillisecondsAfter(nowMilliseconds: now)
        let untilNextDue: Duration = nextDueMilliseconds.map { nextDue in .milliseconds(nextDue - now) } ?? Self.reviewClockMaximumWait

        return min(untilNextDue, Self.reviewClockMaximumWait)
    }

    private static func currentMilliseconds() -> Int64 {
        Int64((Date().timeIntervalSince1970 * 1000).rounded(.down))
    }
}
