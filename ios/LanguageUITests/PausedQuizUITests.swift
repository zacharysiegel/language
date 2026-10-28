import XCTest

/// Pausing, resuming and restarting a unit's quiz, on the test course so that it has few questions.
final class PausedQuizUITests: LanguageUITestCase {
    override func courseIdUnderTest() -> String {
        Self.testCourseId
    }

    override func embeddedBundleDirectoryOverride() -> String? {
        Self.localRepositoryDirectory
    }

    func testAPausedQuizResumesAfterTheAppIsRelaunched() throws {
        let exercises: [[String: Any]] = try Self.readExercises(Self.testCourseQuizUnitId, courseId: Self.testCourseId)

        openThePracticeTab(Self.testCourseQuizUnitId)
        app.buttons["quiz"].tap()
        try answerCorrectly(exercises: exercises, indexes: 0..<Self.pausedPracticeAnswerCount)
        pauseThePractice()
        assertResumeOffered(questionNumber: Self.pausedPracticeAnswerCount + 1, questionCount: exercises.count)

        relaunchKeepingProgress()
        openThePracticeTab(Self.testCourseQuizUnitId)
        assertResumeOffered(questionNumber: Self.pausedPracticeAnswerCount + 1, questionCount: exercises.count)
        saveScreenshot("24-practice-paused")

        app.buttons["resume-quiz"].tap()
        assertQuestionPosition(questionNumber: Self.pausedPracticeAnswerCount + 1, questionCount: exercises.count)
        try answerCorrectly(exercises: exercises, indexes: Self.pausedPracticeAnswerCount..<exercises.count)

        let summary: XCUIElement = app.staticTexts["session-summary"]

        XCTAssertTrue(summary.waitForExistence(timeout: 5))
        XCTAssertTrue(summary.label.contains("\(exercises.count) of \(exercises.count) correct (100%)"))

        app.buttons["session-done"].tap()
        XCTAssertTrue(app.buttons["quiz"].waitForExistence(timeout: 5))
        XCTAssertFalse(app.buttons["resume-quiz"].exists)
    }

    func testStartOverReplacesThePausedQuiz() throws {
        let exercises: [[String: Any]] = try Self.readExercises(Self.testCourseQuizUnitId, courseId: Self.testCourseId)

        openThePracticeTab(Self.testCourseQuizUnitId)
        app.buttons["quiz"].tap()
        try answerCorrectly(exercises: exercises, indexes: 0..<1)
        pauseThePractice()
        assertResumeOffered(questionNumber: 2, questionCount: exercises.count)

        app.buttons["start-over"].tap()
        assertQuestionPosition(questionNumber: 1, questionCount: exercises.count)
        pauseThePractice()
        assertResumeOffered(questionNumber: 1, questionCount: exercises.count)
    }

    func testQuickPracticeIsQuitAndLeavesThePausedQuiz() throws {
        let exercises: [[String: Any]] = try Self.readExercises(Self.testCourseQuizUnitId, courseId: Self.testCourseId)

        openThePracticeTab(Self.testCourseQuizUnitId)
        app.buttons["quiz"].tap()
        try answerCorrectly(exercises: exercises, indexes: 0..<1)
        pauseThePractice()

        app.buttons["quick-practice"].tap()
        XCTAssertTrue(app.buttons["quit"].waitForExistence(timeout: 5))
        XCTAssertFalse(app.buttons["pause"].exists)
        app.buttons["quit"].tap()

        assertResumeOffered(questionNumber: 2, questionCount: exercises.count)
    }
}
