import XCTest
@testable import Language

final class GlossaryRailTests: XCTestCase {
    func testDividingByLevels_listsEachLevelAtItsFirstShownEntry() {
        let shownEntries: [GlossaryEntry] = [
            Self.entry(cardId: "a1-first", unitId: "a1-01", levelCode: "A1", order: 1),
            Self.entry(cardId: "a1-second", unitId: "a1-02", levelCode: "A1", order: 2),
            Self.entry(cardId: "a2-first", unitId: "a2-01", levelCode: "A2", order: 15),
        ]

        let glossaryRail: GlossaryRail = GlossaryRail.dividing(shownEntries, by: .levels)

        XCTAssertEqual(glossaryRail.entries, [
            IndexRailEntry(visibleText: "A1", accessibilityLabel: "Go to level A1"),
            IndexRailEntry(visibleText: "A2", accessibilityLabel: "Go to level A2"),
        ])
        XCTAssertEqual(glossaryRail.runs.map { run in run.id }, ["a1-first", "a2-first"])
        XCTAssertEqual(glossaryRail.runs.map { run in run.entries.count }, [2, 1])
        XCTAssertEqual(glossaryRail.runs.map { run in run.indexRailEntryNumber }, [1, 2])
    }

    func testDividingByUnits_listsEachUnitByItsNumberAtItsFirstShownEntry() {
        let shownEntries: [GlossaryEntry] = [
            Self.entry(cardId: "unit-25-first", unitId: "a2-11", levelCode: "A2", order: 25),
            Self.entry(cardId: "unit-25-second", unitId: "a2-11", levelCode: "A2", order: 25),
            Self.entry(cardId: "unit-26-first", unitId: "a2-12", levelCode: "A2", order: 26),
        ]

        let glossaryRail: GlossaryRail = GlossaryRail.dividing(shownEntries, by: .units)

        XCTAssertEqual(glossaryRail.entries, [
            IndexRailEntry(visibleText: "25", accessibilityLabel: "Go to unit A2 25"),
            IndexRailEntry(visibleText: "26", accessibilityLabel: "Go to unit A2 26"),
        ])
        XCTAssertEqual(glossaryRail.runs.map { run in run.id }, ["unit-25-first", "unit-26-first"])
        XCTAssertEqual(glossaryRail.runs.map { run in run.entries.count }, [2, 1])
        XCTAssertEqual(glossaryRail.runs.map { run in run.indexRailEntryNumber }, [1, 2])
    }

    func testDividingByUnits_numbersOnlyTheFirstRunOfAUnitShownTwice() {
        let shownEntries: [GlossaryEntry] = [
            Self.entry(cardId: "unit-1-first", unitId: "a1-01", levelCode: "A1", order: 1),
            Self.entry(cardId: "unit-2-first", unitId: "a1-02", levelCode: "A1", order: 2),
            Self.entry(cardId: "unit-1-second", unitId: "a1-01", levelCode: "A1", order: 1),
        ]

        let glossaryRail: GlossaryRail = GlossaryRail.dividing(shownEntries, by: .units)

        XCTAssertEqual(glossaryRail.entries.map { railEntry in railEntry.visibleText }, ["1", "2"])
        XCTAssertEqual(glossaryRail.runs.map { run in run.indexRailEntryNumber }, [1, 2, nil])
    }

    func testDividing_noShownEntriesHasNoRunsOrRailEntries() {
        let glossaryRail: GlossaryRail = GlossaryRail.dividing([], by: .units)

        XCTAssertTrue(glossaryRail.runs.isEmpty)
        XCTAssertTrue(glossaryRail.entries.isEmpty)
    }

    func testForLevelFilter_listsLevelsForAllLevelsAndUnitsForOneLevel() {
        XCTAssertEqual(GlossaryRailKind.forLevelFilter(nil), .levels)
        XCTAssertEqual(GlossaryRailKind.forLevelFilter("B1"), .units)
    }

    private static func entry(cardId: String, unitId: String, levelCode: String, order: UInt32) -> GlossaryEntry {
        let item: VocabularyItemRecord = VocabularyItemRecord(
            cardId: cardId,
            target: cardId,
            source: cardId,
            sourceLiteral: nil,
            partOfSpeech: nil,
            note: nil,
            clipKey: nil,
            example: PhrasePairRecord(target: "", source: "", sourceLiteral: nil, clipKey: nil),
        )
        let unit: UnitOutlineRecord = UnitOutlineRecord(id: unitId, levelCode: levelCode, order: order, title: unitId, objectives: [], vocabulary: [item])

        return GlossaryEntry(id: cardId, unit: unit, item: item, searchKeys: [])
    }
}
