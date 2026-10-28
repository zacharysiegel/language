import XCTest

/// The review deck.
final class ReviewUITests: LanguageUITestCase {
    func testReviewAddsWordsAndGradesACard() throws {
        openUnit(Self.practicedUnitId)
        app.buttons["add-words"].tap()

        app.tabBars.buttons["Review"].tap()
        XCTAssertTrue(app.buttons["start-review"].waitForExistence(timeout: 5))
        saveScreenshot("07-review-setup")
        app.buttons["start-review"].tap()

        XCTAssertTrue(app.buttons["show-answer"].waitForExistence(timeout: 5))
        app.buttons["show-answer"].tap()
        saveScreenshot("08-review-card")
        app.buttons["remembered"].tap()

        XCTAssertTrue(app.staticTexts["flashcard-prompt"].waitForExistence(timeout: 5) || app.staticTexts["review-complete"].exists)
    }
}
