import XCTest

/// Opening a unit, its tabs and its status.
final class UnitUITests: LanguageUITestCase {
    func testHomeOpensAUnitAndEveryTab() throws {
        let unitButton: XCUIElement = app.buttons["unit-\(Self.practicedUnitId)"]

        XCTAssertTrue(unitButton.waitForExistence(timeout: 20))
        saveScreenshot("01-home")
        unitButton.tap()

        XCTAssertTrue(app.buttons["tab-overview"].waitForExistence(timeout: 10))
        saveScreenshot("02-unit-overview")

        for tab in ["grammar", "dialogues", "vocabulary", "practice"] {
            app.buttons["tab-\(tab)"].tap()
            XCTAssertTrue(app.buttons["tab-\(tab)"].waitForExistence(timeout: 5))
            saveScreenshot("03-unit-\(tab)")
        }
    }

    func testReadingQuestionsCanBeAnswered() throws {
        let unit: [String: Any] = try Self.readUnit(Self.readingUnitId)
        let readings: [[String: Any]] = unit["readings"] as? [[String: Any]] ?? []
        let questions: [[String: Any]] = readings.first?["questions"] as? [[String: Any]] ?? []

        openUnit(Self.readingUnitId)
        app.buttons["tab-readings"].tap()
        saveScreenshot("06-reading")

        let openQuestions: XCUIElement = app.buttons["comprehension-questions"].firstMatch
        scrollUntilHittable(openQuestions)
        openQuestions.tap()

        for (index, question) in questions.enumerated() {
            try solve(question)
            assertCorrect(exercise: question, index: index)
            app.buttons["continue-exercise"].tap()
        }

        XCTAssertTrue(app.staticTexts["session-summary"].waitForExistence(timeout: 5))
    }

    func testNavigationShowsTheCourseTitleAndOpensTheCourseList() throws {
        let courseButton: XCUIElement = app.buttons["courses"].firstMatch

        XCTAssertTrue(courseButton.waitForExistence(timeout: 20))
        XCTAssertEqual(courseButton.label, try Self.readCourseTitle())

        courseButton.tap()
        XCTAssertTrue(app.buttons["course-\(Self.courseId)"].waitForExistence(timeout: 5))
    }

    func testUnitStatusIsSetByTheStatusControlAndPractice() throws {
        openUnit(Self.practicedUnitId)
        assertSelected(app.buttons["unit-status-not-started"])
        saveScreenshot("12-unit-status")

        app.buttons["unit-status-in-progress"].tap()
        assertSelected(app.buttons["unit-status-in-progress"])

        app.buttons["unit-status-completed"].tap()
        assertSelected(app.buttons["unit-status-completed"])

        app.buttons["unit-status-not-started"].tap()
        assertSelected(app.buttons["unit-status-not-started"])

        app.buttons["tab-practice"].tap()
        app.buttons["quick-practice"].tap()
        XCTAssertTrue(app.buttons["quit"].waitForExistence(timeout: 5))
        app.buttons["quit"].tap()

        app.buttons["tab-overview"].tap()
        assertSelected(app.buttons["unit-status-in-progress"])
    }

    func testUnitStatusHelpFloatsOverThePageAndCloses() throws {
        openUnit(Self.practicedUnitId)

        let helpButton: XCUIElement = app.buttons["unit-status-help"]
        let tip: XCUIElement = app.descendants(matching: .any)["unit-status-help-tip"]
        let tabFrameBeforeOpening: CGRect = app.buttons["tab-overview"].frame

        helpButton.tap()
        XCTAssertTrue(tip.waitForExistence(timeout: 5))
        XCTAssertEqual(app.buttons["tab-overview"].frame, tabFrameBeforeOpening)
        saveScreenshot("13-unit-status-help")

        app.buttons["unit-status-help-close"].tap()
        assertGone(tip, timeout: 5)

        helpButton.tap()
        XCTAssertTrue(tip.waitForExistence(timeout: 5))
        app.buttons["add-words"].coordinate(withNormalizedOffset: CGVector(dx: 0.5, dy: 0.5)).tap()
        assertGone(tip, timeout: 5)
        XCTAssertEqual(app.staticTexts.matching(NSPredicate(format: "label CONTAINS %@", "added")).count, 0)

        helpButton.tap()
        XCTAssertTrue(tip.waitForExistence(timeout: 5))
        assertGone(tip, timeout: 12)
    }
}
