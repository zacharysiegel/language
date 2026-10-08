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

    func unitProgress(unitId: String) -> UnitProgressRecord {
        _ = changeCount
        return book.unitProgress(unitId: unitId)
    }

    func completedUnitCount() -> Int {
        _ = changeCount
        return Int(book.completedUnitCount())
    }

    func dueCardCount() -> Int {
        _ = changeCount
        return Int(book.dueCardCount())
    }

    func cardCount() -> Int {
        _ = changeCount
        return Int(book.cardCount())
    }

    func dueCardIds(limit: Int) -> [String] {
        _ = changeCount
        return book.dueCardIds(limit: UInt32(limit))
    }

    func countByBox() -> [Int] {
        _ = changeCount
        return book.countByBox().map { count in Int(count) }
    }

    func streakDays() -> Int {
        _ = changeCount
        return Int(book.streakDays())
    }

    func lastError() -> String? {
        errorMessage
    }

    func recordPracticeSession(unitId: String, mode: PracticeModeRecord, correctCount: Int, totalCount: Int, vocabularyCount: Int) -> PracticeOutcomeRecord? {
        var outcome: PracticeOutcomeRecord?

        perform {
            outcome = try book.recordPracticeSession(
                unitId: unitId,
                mode: mode,
                correctCount: UInt32(correctCount),
                totalCount: UInt32(totalCount),
                vocabularyCount: UInt32(vocabularyCount),
            )
        }

        return outcome
    }

    func queueUnitVocabulary(unitId: String, vocabularyCount: Int) -> QueuedCardsRecord? {
        var queued: QueuedCardsRecord?

        perform { queued = try book.queueUnitVocabulary(unitId: unitId, vocabularyCount: UInt32(vocabularyCount)) }

        return queued
    }

    func gradeCard(cardId: String, remembered: Bool) {
        perform { try book.gradeCard(cardId: cardId, remembered: remembered) }
    }

    func exportJson() -> String? {
        try? book.exportJson()
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
            errorMessage = String(describing: error)
        }

        changeCount += 1
    }
}
