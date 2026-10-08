import Foundation
import Observation

/// The learner's progress; every read registers with Observation, so views refresh after any change.
@MainActor
@Observable
final class ProgressStore {
    private let book: ProgressBook
    private var changeCount: Int = 0
    private var errorMessage: String?

    init(environment: CoreEnvironment) {
        book = ProgressBook.open(path: environment.progressPath)
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
        return Int(book.dueCardCount(courseId: courseId))
    }

    func cardCount(courseId: String) -> Int {
        _ = changeCount
        return Int(book.cardCount(courseId: courseId))
    }

    func dueCardIds(courseId: String, limit: Int) -> [String] {
        _ = changeCount
        return book.dueCardIds(courseId: courseId, limit: UInt32(limit))
    }

    func countByBox(courseId: String) -> [Int] {
        _ = changeCount
        return book.countByBox(courseId: courseId).map { count in Int(count) }
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
    }
}
