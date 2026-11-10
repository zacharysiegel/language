import XCTest

/// A unit's open responses on the Practice tab, on the test course, whose units each have one.
final class OpenResponseUITests: LanguageUITestCase {
    static let selfCheckHeading: String = "Compare with the examples"
    static let selfCheckNotice: String = "Automatic grading is not available on this device. In supported web browsers and native desktop applications, an AI model checks your answer and points out each mistake."
    /* The test course's unit whose open response gives the facts of a scene. */
    static let sceneUnitId: String = "a1-02-feelings"
    static let writtenAnswerLines: [String] = ["O Paulo e alto.", "Hoje ele esta cansado."]

    override func courseIdUnderTest() -> String {
        Self.testCourseId
    }

    override func embeddedBundleDirectoryOverride() -> String? {
        Self.localRepositoryDirectory
    }

    func testTheOpenResponseOptionCountsTheUnitsTasks() throws {
        let openResponses: [[String: Any]] = try Self.readOpenResponses(Self.testCourseQuizUnitId)

        openThePracticeTab(Self.testCourseQuizUnitId)

        let option: XCUIElement = app.buttons["open-response"]

        XCTAssertTrue(option.waitForExistence(timeout: 5))
        XCTAssertEqual(openResponses.count, 1)
        XCTAssertEqual(option.label, "Open response (1 task)")
    }

    func testCheckingAnOpenResponseShowsTheSelfCheck() throws {
        let openResponse: [String: Any] = try XCTUnwrap(Self.readOpenResponses(Self.sceneUnitId).first)
        let exampleAnswers: [String] = openResponse["example_answers"] as? [String] ?? []
        let writtenAnswer: String = Self.writtenAnswerLines.joined(separator: "\n")

        openThePracticeTab(Self.sceneUnitId)
        startTheOpenResponses()
        XCTAssertTrue(app.staticTexts[try XCTUnwrap(openResponse["prompt_source"] as? String)].waitForExistence(timeout: 5))
        XCTAssertTrue(app.staticTexts[try XCTUnwrap(openResponse["context_source"] as? String)].exists)

        writeAnswer(writtenAnswer)

        let heading: XCUIElement = app.staticTexts["feedback-heading"]

        XCTAssertTrue(heading.waitForExistence(timeout: 5))
        XCTAssertEqual(heading.label, Self.selfCheckHeading)

        let notice: XCUIElement = app.staticTexts["self-check-notice"]
        let writtenAnswerElement: XCUIElement = app.staticTexts["written-answer"]

        XCTAssertEqual(notice.label, Self.selfCheckNotice)
        XCTAssertGreaterThanOrEqual(notice.frame.minY, heading.frame.maxY)
        XCTAssertLessThanOrEqual(notice.frame.maxY, writtenAnswerElement.frame.minY)
        XCTAssertEqual(writtenAnswerElement.label, writtenAnswer)
        XCTAssertEqual(app.staticTexts.matching(identifier: "example-answer").allElementsBoundByIndex.map { exampleElement in exampleElement.label }, exampleAnswers)
        XCTAssertTrue(app.buttons["Play audio: \(exampleAnswers[0])"].exists)
        saveScreenshot("25-open-response-self-check")
    }

    func testTheOpenResponsePracticeEndsWithTheLearnersAnswersAndLeavesTheQuizUntaken() throws {
        let openResponses: [[String: Any]] = try Self.readOpenResponses(Self.testCourseQuizUnitId)
        let writtenAnswer: String = Self.writtenAnswerLines[0]

        openThePracticeTab(Self.testCourseQuizUnitId)
        startTheOpenResponses()

        for _ in openResponses {
            writeAnswer(writtenAnswer)
            scrollUntilHittable(app.buttons["continue-exercise"])
            app.buttons["continue-exercise"].tap()
        }

        let summary: XCUIElement = app.staticTexts["session-summary"]

        XCTAssertTrue(summary.waitForExistence(timeout: 5))
        XCTAssertEqual(summary.label, "Open response")
        XCTAssertTrue(app.staticTexts["YOUR ANSWERS"].exists)

        for openResponse in openResponses {
            XCTAssertTrue(app.staticTexts[try XCTUnwrap(openResponse["prompt_source"] as? String)].exists)
        }

        XCTAssertEqual(app.staticTexts.matching(NSPredicate(format: "label == %@", writtenAnswer)).count, openResponses.count)

        app.buttons["session-done"].tap()
        XCTAssertTrue(app.buttons["open-response"].waitForExistence(timeout: 5))
        XCTAssertTrue(app.staticTexts["You have not taken the quiz yet."].exists)
        XCTAssertFalse(app.buttons["resume-quiz"].exists)
    }

    func testAnOpenResponseShowsItsSavedAnswerAfterTheAppStartsAgain() throws {
        let writtenAnswer: String = Self.writtenAnswerLines[0]

        openThePracticeTab(Self.testCourseQuizUnitId)
        startTheOpenResponses()
        XCTAssertFalse(app.staticTexts["saved-answer"].exists)
        writeAnswer(writtenAnswer)
        scrollUntilHittable(app.buttons["continue-exercise"])
        app.buttons["continue-exercise"].tap()
        XCTAssertTrue(app.buttons["session-done"].waitForExistence(timeout: 5))
        app.buttons["session-done"].tap()

        relaunchKeepingProgress()
        openThePracticeTab(Self.testCourseQuizUnitId)
        startTheOpenResponses()

        let savedAnswer: XCUIElement = app.staticTexts["saved-answer"]

        XCTAssertTrue(savedAnswer.waitForExistence(timeout: 5))
        XCTAssertEqual(savedAnswer.label, writtenAnswer)
        XCTAssertFalse(app.staticTexts["saved-grade-heading"].exists)
        saveScreenshot("27-open-response-saved-answer")
    }

    private func startTheOpenResponses() {
        let option: XCUIElement = app.buttons["open-response"]

        XCTAssertTrue(option.waitForExistence(timeout: 5))
        scrollUntilHittable(option)
        option.tap()
        XCTAssertTrue(app.buttons["quit"].waitForExistence(timeout: 5))
    }

    private func writeAnswer(_ text: String) {
        let field: XCUIElement = app.textFields["open-response-field"]

        XCTAssertTrue(field.waitForExistence(timeout: 5))
        field.tap()
        field.typeText(text)
        app.buttons["check"].tap()
    }

    private static func readOpenResponses(_ unitId: String) throws -> [[String: Any]] {
        let exercises: [[String: Any]] = try readUnit(unitId, courseId: testCourseId)["exercises"] as? [[String: Any]] ?? []

        return exercises.filter { exercise in exercise["type"] as? String == "open_response" }
    }
}
