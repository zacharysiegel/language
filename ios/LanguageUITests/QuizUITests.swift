import XCTest

/// The quiz of a unit, on the test course so that it has few questions.
final class QuizUITests: LanguageUITestCase {
    override func courseIdUnderTest() -> String {
        Self.testCourseId
    }

    override func embeddedBundleDirectoryOverride() -> String? {
        Self.localRepositoryDirectory
    }

    func testQuizOfAUnitScoresEveryAnswerCorrect() throws {
        let unit: [String: Any] = try Self.readUnit(Self.testCourseQuizUnitId, courseId: Self.testCourseId)
        let exercises: [[String: Any]] = unit["exercises"] as? [[String: Any]] ?? []

        openUnit(Self.testCourseQuizUnitId)
        app.buttons["tab-practice"].tap()
        app.buttons["quiz"].tap()

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
}
