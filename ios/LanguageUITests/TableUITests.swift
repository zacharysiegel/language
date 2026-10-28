import XCTest

/// How a principle's table lays out at compact width.
final class TableUITests: LanguageUITestCase {
    @MainActor
    func testPrinciplesTablesAreGroupedByTheirFirstColumnAtCompactWidth() throws {
        let (noteNumber, table): (Int, [String: Any]) = try Self.readGrammarTable(unitId: Self.groupedTableUnitId, noteTitle: Self.groupedTableNoteTitle)
        let headers: [String] = table["headers"] as? [String] ?? []
        let firstRow: [String] = (table["rows"] as? [[String]] ?? []).first ?? []

        openPrinciplesNote(unitId: Self.groupedTableUnitId, noteNumber: noteNumber)

        let firstHeading: XCUIElement = app.staticTexts.matching(NSPredicate(format: "identifier == %@ AND label == %@", "grouped-table-heading", firstRow[0])).firstMatch

        XCTAssertTrue(firstHeading.waitForExistence(timeout: 5))
        scrollUntilHittable(firstHeading)
        XCTAssertTrue(groupedTableKeyLabel(headers[0]).exists)
        XCTAssertTrue(groupedTableKeyLabel(headers[1]).exists)
        XCTAssertTrue(groupedTableRow(headers: Array(headers.dropFirst()), cells: Array(firstRow.dropFirst())).isHittable)
        saveScreenshot("19-grouped-table")
    }

    @MainActor
    func testPrinciplesTablesWithoutARepeatedFirstColumnAreStackedAtCompactWidth() throws {
        let (noteNumber, table): (Int, [String: Any]) = try Self.readGrammarTable(unitId: Self.stackedTableUnitId, noteTitle: Self.stackedTableNoteTitle)
        let headers: [String] = table["headers"] as? [String] ?? []
        let firstRow: [String] = (table["rows"] as? [[String]] ?? []).first ?? []

        openPrinciplesNote(unitId: Self.stackedTableUnitId, noteNumber: noteNumber)

        let firstTableRow: XCUIElement = groupedTableRow(headers: headers, cells: firstRow)

        XCTAssertTrue(firstTableRow.waitForExistence(timeout: 5))
        scrollUntilHittable(firstTableRow)
        XCTAssertEqual(app.staticTexts.matching(identifier: "grouped-table-heading").count, 0)
        XCTAssertTrue(groupedTableKeyLabel(headers[0]).exists)
        saveScreenshot("20-stacked-table")
    }
}
