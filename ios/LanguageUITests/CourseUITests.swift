import XCTest

/// Drives the app as a learner would, against the course data in the repository.
final class CourseUITests: XCTestCase {
    static let repositoryRoot: URL = URL(fileURLWithPath: #filePath)
        .deletingLastPathComponent()
        .deletingLastPathComponent()
        .deletingLastPathComponent()
    static let screenshotDirectory: String = "/tmp/language-ios-screenshots"
    static let courseId: String = "pt_br.en.foundations"
    static let practicedUnitId: String = "a1-01-greetings-introductions"
    static let readingUnitId: String = "b1-01-present-perfect-participles"

    private var app: XCUIApplication!

    override func setUp() {
        continueAfterFailure = false
        app = XCUIApplication()
        app.launchArguments = ["-LanguageProgressFile", "ui-test-progress-\(UUID().uuidString).json"]
        app.launch()
        try? FileManager.default.createDirectory(atPath: Self.screenshotDirectory, withIntermediateDirectories: true)
        chooseTheCourseIfTheCourseListOpens()
    }

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

    func testFullPracticeOfAUnitScoresEveryAnswerCorrect() throws {
        let unit: [String: Any] = try Self.readUnit(Self.practicedUnitId)
        let exercises: [[String: Any]] = unit["exercises"] as? [[String: Any]] ?? []

        openUnit(Self.practicedUnitId)
        app.buttons["tab-practice"].tap()
        app.buttons["full-practice"].tap()

        for (index, exercise) in exercises.enumerated() {
            try solve(exercise)
            assertCorrect(exercise: exercise, index: index)

            if index == 0 {
                saveScreenshot("04-practice-feedback")
            }

            app.buttons["continue-exercise"].tap()
        }

        let summary: XCUIElement = app.staticTexts["session-summary"]

        XCTAssertTrue(summary.waitForExistence(timeout: 5))
        XCTAssertTrue(summary.label.contains("\(exercises.count) of \(exercises.count) correct (100%)"), summary.label)
        saveScreenshot("05-practice-summary")
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

    func testGlossaryAndProgressAndSettingsOpen() throws {
        XCTAssertTrue(app.buttons["unit-\(Self.practicedUnitId)"].waitForExistence(timeout: 20))

        app.tabBars.buttons["Glossary"].tap()
        XCTAssertTrue(app.staticTexts["Glossary"].waitForExistence(timeout: 5))
        saveScreenshot("09-glossary")

        app.tabBars.buttons["Progress"].tap()
        XCTAssertTrue(app.staticTexts["Review deck"].waitForExistence(timeout: 5))
        saveScreenshot("10-progress")

        app.buttons["settings"].firstMatch.tap()
        XCTAssertTrue(app.buttons["voice-kokoro"].waitForExistence(timeout: 5))
        saveScreenshot("11-settings")
    }

    func testNavigationShowsTheCourseTitleAndOpensTheCourseList() throws {
        let courseButton: XCUIElement = app.buttons["courses"].firstMatch

        XCTAssertTrue(courseButton.waitForExistence(timeout: 20))
        XCTAssertEqual(courseButton.label, try Self.readCourseTitle())

        courseButton.tap()
        XCTAssertTrue(app.buttons["course-\(Self.courseId)"].waitForExistence(timeout: 5))
    }

    func testUnitStatusIsSetByStartTheStatusControlAndPractice() throws {
        openUnit(Self.practicedUnitId)
        assertSelected(app.buttons["unit-status-not-started"])
        saveScreenshot("12-unit-status")

        app.buttons["start-unit"].tap()
        assertSelected(app.buttons["unit-status-in-progress"])
        XCTAssertFalse(app.buttons["start-unit"].exists)

        app.buttons["unit-status-completed"].tap()
        assertSelected(app.buttons["unit-status-completed"])

        app.buttons["unit-status-not-started"].tap()
        assertSelected(app.buttons["unit-status-not-started"])
        XCTAssertTrue(app.buttons["start-unit"].waitForExistence(timeout: 5))

        app.buttons["tab-practice"].tap()
        app.buttons["quick-practice"].tap()
        XCTAssertTrue(app.buttons["quit"].waitForExistence(timeout: 5))
        app.buttons["quit"].tap()

        app.buttons["tab-overview"].tap()
        assertSelected(app.buttons["unit-status-in-progress"])
    }

    // MARK: - Choosing the course

    /// Opens the course under test when the app starts on the course list.
    private func chooseTheCourseIfTheCourseListOpens() {
        let courseChoiceIdentifier: String = "course-\(Self.courseId)"
        let firstScreenIdentifiers: [String] = [courseChoiceIdentifier, "courses"]
        let firstScreenElement: XCUIElement = app.buttons.matching(NSPredicate(format: "identifier IN %@", firstScreenIdentifiers)).firstMatch

        XCTAssertTrue(firstScreenElement.waitForExistence(timeout: 20))

        let courseChoice: XCUIElement = app.buttons[courseChoiceIdentifier]

        if courseChoice.exists {
            courseChoice.tap()
        }
    }

    // MARK: - Driving exercises

    private func openUnit(_ unitId: String) {
        let unitButton: XCUIElement = app.buttons["unit-\(unitId)"]

        XCTAssertTrue(unitButton.waitForExistence(timeout: 20))
        scrollUntilHittable(unitButton)
        unitButton.tap()
        XCTAssertTrue(app.buttons["tab-overview"].waitForExistence(timeout: 15))
    }

    private func solve(_ exercise: [String: Any]) throws {
        let kind: String = exercise["type"] as? String ?? ""

        switch kind {
        case "multiple_choice", "listening_choice":
            let choices: [String] = exercise["choices"] as? [String] ?? []
            let answer: Int = exercise["answer_index"] as? Int ?? 0
            tapEnabledButton(identifier: "choice", label: choices[answer])
        case "fill_blank":
            typeAnswer((exercise["answers"] as? [String] ?? []).first ?? "")
        case "translate_to_target":
            typeAnswer((exercise["answers"] as? [String] ?? []).first ?? "")
        case "dictation":
            typeAnswer(exercise["audio_target"] as? String ?? "")
        case "order_words":
            for word in exercise["words"] as? [String] ?? [] {
                tapEnabledButton(identifier: "bank-token", label: word)
            }

            app.buttons["check"].tap()
        case "match":
            for pair in exercise["pairs"] as? [[String: Any]] ?? [] {
                tapEnabledButton(identifier: "match-left", label: pair["target"] as? String ?? "")
                tapEnabledButton(identifier: "match-right", label: pair["source"] as? String ?? "")
            }
        default:
            XCTFail("unknown exercise kind \(kind)")
        }
    }

    private func typeAnswer(_ text: String) {
        let field: XCUIElement = app.textFields["answer-field"]

        XCTAssertTrue(field.waitForExistence(timeout: 5))
        field.tap()
        field.typeText(text)
        app.buttons["check"].tap()
    }

    private func tapEnabledButton(identifier: String, label: String) {
        let candidates: XCUIElementQuery = app.buttons.matching(NSPredicate(format: "identifier == %@ AND label == %@ AND enabled == true", identifier, label))
        let button: XCUIElement = candidates.firstMatch

        XCTAssertTrue(button.waitForExistence(timeout: 5), "no \(identifier) button labelled \(label)")
        scrollUntilHittable(button)
        button.tap()
    }

    private func assertCorrect(exercise: [String: Any], index: Int) {
        let heading: XCUIElement = app.staticTexts["feedback-heading"]

        XCTAssertTrue(heading.waitForExistence(timeout: 5), "no feedback for exercise \(index)")
        XCTAssertEqual(heading.label, "Correct", "exercise \(index): \(exercise)")
        scrollUntilHittable(app.buttons["continue-exercise"])
    }

    private func assertSelected(_ element: XCUIElement) {
        let selected: XCTNSPredicateExpectation = XCTNSPredicateExpectation(predicate: NSPredicate(format: "isSelected == true"), object: element)
        let waitResult: XCTWaiter.Result = XCTWaiter().wait(for: [selected], timeout: 5)

        XCTAssertEqual(waitResult, .completed)
    }

    private func scrollUntilHittable(_ element: XCUIElement) {
        var attempts: Int = 0

        while !element.isHittable && attempts < 12 {
            app.swipeUp(velocity: .slow)
            attempts += 1
        }
    }

    private func saveScreenshot(_ name: String) {
        let data: Data = XCUIScreen.main.screenshot().pngRepresentation

        try? data.write(to: URL(fileURLWithPath: "\(Self.screenshotDirectory)/\(name).png"))
    }

    private static func readUnit(_ unitId: String) throws -> [String: Any] {
        let path: URL = repositoryRoot.appendingPathComponent("data/courses/\(courseId)/units/\(unitId).json")
        let data: Data = try Data(contentsOf: path)

        return try JSONSerialization.jsonObject(with: data) as? [String: Any] ?? [:]
    }

    private static func readCourseTitle() throws -> String {
        let path: URL = repositoryRoot.appendingPathComponent("data/courses/\(courseId)/curriculum.json")
        let data: Data = try Data(contentsOf: path)
        let curriculum: [String: Any] = try JSONSerialization.jsonObject(with: data) as? [String: Any] ?? [:]

        return curriculum["title"] as? String ?? ""
    }
}
