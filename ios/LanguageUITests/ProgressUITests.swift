import XCTest

/// The progress page's completion per level and quiz results, on the test course so that its quiz stays short.
final class ProgressUITests: LanguageUITestCase {
    static let testCourseLevelCode: String = "A1"

    override func courseIdUnderTest() -> String {
        Self.testCourseId
    }

    override func embeddedBundleDirectoryOverride() -> String? {
        Self.localRepositoryDirectory
    }

    @MainActor
    func testTheProgressPageShowsCompletionPerLevelAndQuizResults() throws {
        let exercises: [[String: Any]] = try Self.readQuizExercises(Self.testCourseQuizUnitId, courseId: Self.testCourseId)
        let quizUnitTitle: String = try XCTUnwrap(Self.readUnit(Self.testCourseQuizUnitId, courseId: Self.testCourseId)["title"] as? String)
        let unitCount: Int = try Self.readCurriculumUnits(courseId: Self.testCourseId).filter { curriculumUnit in curriculumUnit["level"] as? String == Self.testCourseLevelCode }.count

        openTheProgressPage()
        XCTAssertTrue(app.staticTexts["quiz-results-empty"].waitForExistence(timeout: 5))
        assertLevelCompletion(completedUnitCount: 0, unitCount: unitCount)
        XCTAssertFalse(app.scrollViews.staticTexts["Progress"].exists)

        app.tabBars.buttons["Course"].tap()
        openThePracticeTab(Self.testCourseQuizUnitId)
        app.buttons["quiz"].tap()
        try answerCorrectly(exercises: exercises, indexes: 0..<exercises.count)
        app.buttons["session-done"].tap()

        openTheProgressPage()

        let quizResult: XCUIElement = app.buttons["quiz-result-\(Self.testCourseQuizUnitId)"]

        XCTAssertTrue(quizResult.waitForExistence(timeout: 5))
        XCTAssertTrue(quizResult.label.contains("Best score: 100% over 1 attempt(s)"), quizResult.label)
        XCTAssertFalse(app.staticTexts["quiz-results-empty"].exists)
        assertLevelCompletion(completedUnitCount: 1, unitCount: unitCount)
        saveScreenshot("28-progress-recorded")

        quizResult.tap()
        XCTAssertTrue(app.buttons["tab-overview"].waitForExistence(timeout: 15))
        XCTAssertTrue(app.navigationBars[quizUnitTitle].exists)

        relaunchKeepingProgress()
        XCTAssertTrue(app.buttons["continue"].waitForExistence(timeout: 20))
        XCTAssertFalse(app.staticTexts.containing(NSPredicate(format: "label CONTAINS %@", "units completed")).firstMatch.exists)
        saveScreenshot("29-course-progress-recorded")
    }

    @MainActor
    private func openTheProgressPage() {
        app.tabBars.buttons["Progress"].tap()
        XCTAssertTrue(app.staticTexts["Review deck"].waitForExistence(timeout: 5))
    }

    @MainActor
    private func assertLevelCompletion(completedUnitCount: Int, unitCount: Int) {
        let levelCompletion: XCUIElement = app.descendants(matching: .any)["level-completion-\(Self.testCourseLevelCode)"]

        XCTAssertTrue(levelCompletion.waitForExistence(timeout: 5))
        XCTAssertTrue(levelCompletion.label.contains("\(completedUnitCount) of \(unitCount) units completed"), levelCompletion.label)
    }
}
