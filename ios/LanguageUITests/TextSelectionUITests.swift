import XCTest

/// Selecting and copying the text a lesson shows.
final class TextSelectionUITests: LanguageUITestCase {
    /* Element queries reject a long identifier string, so the paragraph is found by its start. */
    static let paragraphMatchLength: Int = 60

    @MainActor
    func testAPrinciplesParagraphCanBeSelectedAndCopied() throws {
        let notes: [[String: Any]] = try Self.readUnit(Self.groupedTableUnitId)["grammar"] as? [[String: Any]] ?? []
        let explanation: String = try XCTUnwrap(notes.first?["explanation"] as? String)
        let firstParagraph: String = try XCTUnwrap(explanation.components(separatedBy: "\n\n").first)

        openPrinciplesNote(unitId: Self.groupedTableUnitId, noteNumber: 1)

        let paragraphStart: String = String(firstParagraph.prefix(Self.paragraphMatchLength))
        let paragraph: XCUIElement = app.textViews.matching(NSPredicate(format: "label BEGINSWITH %@", paragraphStart)).firstMatch

        XCTAssertTrue(paragraph.waitForExistence(timeout: 5))
        scrollUntilHittable(paragraph)
        paragraph.press(forDuration: 1.2)

        XCTAssertTrue(app.menuItems["Copy"].waitForExistence(timeout: 5))
        saveScreenshot("22-text-selection")
    }

}
