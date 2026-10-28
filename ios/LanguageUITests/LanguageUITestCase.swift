import XCTest

/// Starts the app afresh for each test and holds what the feature test classes share.
class LanguageUITestCase: XCTestCase {
    static let repositoryRoot: URL = URL(fileURLWithPath: #filePath)
        .deletingLastPathComponent()
        .deletingLastPathComponent()
        .deletingLastPathComponent()
    static let screenshotDirectory: String = "/tmp/language-ios-screenshots"
    static let courseId: String = "pt_br.en.foundations"
    static let practicedUnitId: String = "a1-01-greetings-introductions"
    static let readingUnitId: String = "b1-01-present-perfect-participles"
    /* The unit captioned "A1 · 1". */
    static let firstUnitId: String = "a1-02-sounds-alphabet"
    static let firstUnitLabelPrefix: String = "A1.1."
    static let jumpedNoteNumber: Int = 4
    static let momentumJumpedNoteNumber: Int = 2
    /* Longer than a fast fling's deceleration. */
    static let momentumSettleDuration: TimeInterval = 3
    /* XCUIApplicationInteractionOptionSkipPreEventQuiescence | XCUIApplicationInteractionOptionSkipPostEventQuiescence, private to XCUIAutomation. */
    static let skipQuiescenceInteractionOptions: UInt = 0b11
    /* The app's idle hide delay and fade, with room for a slow simulator. */
    static let indexRailHideAllowance: TimeInterval = 3
    static let layoutTolerance: CGFloat = 1
    /* The app's offset of the rail's shadow. */
    static let indexRailShadowOffset: CGFloat = 2
    static let groupedTableUnitId: String = "a1-00-how-portuguese-works"
    static let groupedTableNoteTitle: String = "Gender, number and agreement"
    static let stackedTableUnitId: String = "a1-09-shopping-colors-prices"
    static let stackedTableNoteTitle: String = "Demonstratives: este, esse, aquele"
    static let playedButtonIndexes: [Int] = [0, 0, 1, 2]
    static let clipPlaybackAllowance: TimeInterval = 3
    /* A TEST-NET-1 address (RFC 5737), never routed, so a connection to it waits rather than failing. */
    static let unreachableOrigin: String = "http://192.0.2.1:3002"
    static let pendingRecordingValue: String = "Loading the recording"
    static let jumpedGlossaryLevelCode: String = "B1"
    static let jumpedGlossaryUnitId: String = "b1-05-imperfect-subjunctive-conditionals"
    static let pausedPracticeAnswerCount: Int = 3
    static let testCourseId: String = "pt_br.en.test_course"
    /* The test course's first unit, whose quiz has the fewest questions a unit can have. */
    static let testCourseQuizUnitId: String = "a1-01-greetings"
    /* The local repository, which holds the test course beside the course under test elsewhere. */
    static let localRepositoryDirectory: String = repositoryRoot.appendingPathComponent("build/repository").path

    var app: XCUIApplication!

    /// The course the test opens when the app starts on the course list.
    func courseIdUnderTest() -> String {
        Self.courseId
    }

    /// A directory to read the embedded bundle from instead of the app's own; `nil` keeps the app's own.
    func embeddedBundleDirectoryOverride() -> String? {
        nil
    }

    override func setUpWithError() throws {
        continueAfterFailure = false
        app = XCUIApplication()
        app.launchArguments = try launchArguments()
        app.launch()
        try? FileManager.default.createDirectory(atPath: Self.screenshotDirectory, withIntermediateDirectories: true)
        chooseTheCourseIfTheCourseListOpens()
    }

    func launchArguments() throws -> [String] {
        var arguments: [String] = ["-LanguageProgressFile", "ui-test-progress-\(UUID().uuidString).json", "-LanguageSilentPlayback", "YES"]

        if let embeddedBundleDirectory = embeddedBundleDirectoryOverride() {
            let courseIndexPath: String = "\(embeddedBundleDirectory)/course-index.json"

            guard FileManager.default.fileExists(atPath: courseIndexPath)
            else {
                throw TestSetupError.missingEmbeddedBundle(courseIndexPath)
            }

            arguments += ["-LanguageEmbeddedBundleDirectory", embeddedBundleDirectory]
        }

        return arguments
    }

    // MARK: - Choosing the course

    /// Starts the app again with the same progress file.
    func relaunchKeepingProgress() {
        app.terminate()
        app.launch()
        chooseTheCourseIfTheCourseListOpens()
    }

    /// Starts the app again with a cache of its own and neither the origin nor the static repository reachable.
    func relaunchWithUnreachableRepository() {
        app.terminate()
        app.launchArguments = [
            "-LanguageProgressFile", "ui-test-progress-\(UUID().uuidString).json",
            "-LanguageCacheDirectory", "ui-test-cache-\(UUID().uuidString)",
            "-LanguageOrigin", Self.unreachableOrigin,
            "-LanguageRepositoryBaseUrl", Self.unreachableOrigin,
            "-LanguageSilentPlayback", "YES",
        ]
        app.launch()
        chooseTheCourseIfTheCourseListOpens()
    }

    /// Opens the course under test when the app starts on the course list.
    func chooseTheCourseIfTheCourseListOpens() {
        let courseChoiceIdentifier: String = "course-\(courseIdUnderTest())"
        let firstScreenIdentifiers: [String] = [courseChoiceIdentifier, "courses"]
        let firstScreenElement: XCUIElement = app.buttons.matching(NSPredicate(format: "identifier IN %@", firstScreenIdentifiers)).firstMatch

        XCTAssertTrue(firstScreenElement.waitForExistence(timeout: 20))

        let courseChoice: XCUIElement = app.buttons[courseChoiceIdentifier]

        if courseChoice.exists {
            courseChoice.tap()
        }
    }

    // MARK: - Driving exercises

    func openUnit(_ unitId: String) {
        let unitButton: XCUIElement = app.buttons["unit-\(unitId)"]

        XCTAssertTrue(unitButton.waitForExistence(timeout: 20))
        scrollUntilHittable(unitButton)
        unitButton.tap()
        XCTAssertTrue(app.buttons["tab-overview"].waitForExistence(timeout: 15))
    }

    func openThePracticeTab(_ unitId: String) {
        openUnit(unitId)
        app.buttons["tab-practice"].tap()
    }

    func answerCorrectly(exercises: [[String: Any]], indexes: Range<Int>) throws {
        for index in indexes {
            try solve(exercises[index])
            assertCorrect(exercise: exercises[index], index: index)
            app.buttons["continue-exercise"].tap()
        }
    }

    func pauseThePractice() {
        let pauseButton: XCUIElement = app.buttons["pause"]

        XCTAssertTrue(pauseButton.waitForExistence(timeout: 5))
        XCTAssertFalse(app.buttons["quit"].exists)
        pauseButton.tap()
    }

    func assertResumeOffered(questionNumber: Int, questionCount: Int) {
        let resumeButton: XCUIElement = app.buttons["resume-quiz"]

        XCTAssertTrue(resumeButton.waitForExistence(timeout: 5))
        XCTAssertTrue(resumeButton.label.contains("question \(questionNumber) of \(questionCount)"))
        XCTAssertTrue(app.buttons["start-over"].exists)
        XCTAssertFalse(app.buttons["quiz"].exists)
    }

    func assertQuestionPosition(questionNumber: Int, questionCount: Int) {
        let position: XCUIElement = app.staticTexts["question-position"]

        XCTAssertTrue(position.waitForExistence(timeout: 5))
        XCTAssertTrue(position.label.hasSuffix("question \(questionNumber) of \(questionCount)"))
    }

    func solve(_ exercise: [String: Any]) throws {
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

    func typeAnswer(_ text: String) {
        let field: XCUIElement = app.textFields["answer-field"]

        XCTAssertTrue(field.waitForExistence(timeout: 5))
        field.tap()
        field.typeText(text)
        app.buttons["check"].tap()
    }

    func tapEnabledButton(identifier: String, label: String) {
        let candidates: XCUIElementQuery = app.buttons.matching(NSPredicate(format: "identifier == %@ AND label == %@ AND enabled == true", identifier, label))
        let button: XCUIElement = candidates.firstMatch

        XCTAssertTrue(button.waitForExistence(timeout: 5), "no \(identifier) button labelled \(label)")
        scrollUntilHittable(button)
        button.tap()
    }

    func assertCorrect(exercise: [String: Any], index: Int) {
        let heading: XCUIElement = app.staticTexts["feedback-heading"]

        XCTAssertTrue(heading.waitForExistence(timeout: 5), "no feedback for exercise \(index)")
        XCTAssertEqual(heading.label, "Correct", "exercise \(index): \(exercise)")
        scrollUntilHittable(app.buttons["continue-exercise"])
    }

    func assertSelected(_ element: XCUIElement) {
        let selected: XCTNSPredicateExpectation = XCTNSPredicateExpectation(predicate: NSPredicate(format: "isSelected == true"), object: element)
        let waitResult: XCTWaiter.Result = XCTWaiter().wait(for: [selected], timeout: 5)

        XCTAssertEqual(waitResult, .completed)
    }

    func assertValue(_ element: XCUIElement, equals value: String, timeout: TimeInterval) {
        let hasValue: XCTNSPredicateExpectation = XCTNSPredicateExpectation(predicate: NSPredicate(format: "value == %@", value), object: element)
        let waitResult: XCTWaiter.Result = XCTWaiter().wait(for: [hasValue], timeout: timeout)

        XCTAssertEqual(waitResult, .completed)
    }

    func assertGone(_ element: XCUIElement, timeout: TimeInterval) {
        let gone: XCTNSPredicateExpectation = XCTNSPredicateExpectation(predicate: NSPredicate(format: "exists == false"), object: element)
        let waitResult: XCTWaiter.Result = XCTWaiter().wait(for: [gone], timeout: timeout)

        XCTAssertEqual(waitResult, .completed)
    }

    // MARK: - Grammar tables

    func openPrinciplesNote(unitId: String, noteNumber: Int) {
        openUnit(unitId)
        app.buttons["tab-grammar"].tap()
        XCTAssertTrue(app.staticTexts["unit-item-label"].firstMatch.waitForExistence(timeout: 5))

        withTheIndexRailShown {
            self.railEntry(noteNumber).tap()
        }
    }

    func groupedTableKeyLabel(_ header: String) -> XCUIElement {
        app.staticTexts.matching(NSPredicate(format: "identifier == %@ AND label == %@", "grouped-table-key", header.uppercased())).firstMatch
    }

    /// The row of a grouped or stacked table whose spoken text starts with its lead and ends with its last cell, each after its header.
    func groupedTableRow(headers: [String], cells: [String]) -> XCUIElement {
        let leadPair: String = "\(headers.first ?? ""): \(cells.first ?? "")"
        let lastPair: String = "\(headers.last ?? ""): \(cells.last ?? "")"
        let predicate: NSPredicate = NSPredicate(format: "identifier == %@ AND label BEGINSWITH %@ AND label CONTAINS %@", "grouped-table-row", leadPair, lastPair)

        return app.descendants(matching: .any).matching(predicate).firstMatch
    }

    /// The note titled `noteTitle`, numbered from 1, and its table.
    static func readGrammarTable(unitId: String, noteTitle: String) throws -> (Int, [String: Any]) {
        let notes: [[String: Any]] = try readUnit(unitId)["grammar"] as? [[String: Any]] ?? []
        let noteIndex: Int = try XCTUnwrap(notes.firstIndex { note in note["title"] as? String == noteTitle })
        let table: [String: Any] = try XCTUnwrap(notes[noteIndex]["table"] as? [String: Any])

        return (noteIndex + 1, table)
    }

    // MARK: - The box index rail

    func railEntry(_ itemNumber: Int) -> XCUIElement {
        app.buttons["unit-item-index-\(itemNumber)"]
    }

    func glossaryLevelRailEntry(levelCode: String) -> XCUIElement {
        app.buttons["glossary-level-index-\(levelCode)"]
    }

    func glossaryUnitRailEntry(unitNumber: Int) -> XCUIElement {
        app.buttons["glossary-unit-index-\(unitNumber)"]
    }

    func openTheGlossary() {
        XCTAssertTrue(app.buttons["unit-\(Self.practicedUnitId)"].waitForExistence(timeout: 20))
        app.tabBars.buttons["Glossary"].tap()
        XCTAssertTrue(app.staticTexts["Glossary"].waitForExistence(timeout: 5))
    }

    func assertNearTopOfThePage(_ element: XCUIElement) {
        let navigationBarBottom: CGFloat = app.navigationBars.firstMatch.frame.maxY

        XCTAssertTrue(element.isHittable)
        XCTAssertGreaterThanOrEqual(element.frame.minY, navigationBarBottom)
        XCTAssertLessThan(element.frame.minY, navigationBarBottom + app.frame.height / 4)
    }

    /// Fails when the page's content column keeps further from the trailing edge than from the leading edge.
    func assertTheContentIsInsetEquallyFromBothEdges() {
        let screenFrame: CGRect = app.frame
        let tabRowFrame: CGRect = app.scrollViews["unit-tabs"].frame

        XCTAssertFalse(tabRowFrame.isEmpty)
        XCTAssertEqual(screenFrame.maxX - tabRowFrame.maxX, tabRowFrame.minX - screenFrame.minX, accuracy: Self.layoutTolerance)
    }

    /// Fails when the rail's shadow reaches the screen's trailing edge.
    func assertTheIndexRailShadowClearsTheTrailingEdge() {
        let railTrailingClearance: CGFloat = app.frame.maxX - railEntry(1).frame.maxX

        XCTAssertGreaterThan(railTrailingClearance, Self.indexRailShadowOffset + Self.layoutTolerance)
    }

    /// Fails when a tap where the rail's entry for `itemNumber` is drawn jumps to that box.
    func assertTheIndexRailLetsTouchesThrough(itemNumber: Int) {
        let entry: XCUIElement = railEntry(itemNumber)
        let entryFrame: CGRect = entry.frame
        let entryCenter: XCUICoordinate = app.coordinate(withNormalizedOffset: .zero).withOffset(CGVector(dx: entryFrame.midX, dy: entryFrame.midY))

        XCTAssertFalse(entry.isSelected)
        entryCenter.tap()
        XCTAssertFalse(entry.isSelected)
    }

    /// Scrolls the page a little, which shows the index rail, and runs `railActions` before it can hide again.
    func withTheIndexRailShown(_ railActions: @escaping () -> Void) {
        performWithoutWaitingForTheAppToIdle {
            self.app.swipeUp(velocity: .slow)
            railActions()
        }
    }

    /// Synthesizes each event of `actions` as soon as it is reached, without waiting for scrolling or animation to finish.
    func performWithoutWaitingForTheAppToIdle(_ actions: @escaping () -> Void) {
        typealias PerformWithInteractionOptions = @convention(c) (AnyObject, Selector, UInt, @convention(block) () -> Void) -> Void

        /* A private XCUIApplication function; no public API skips the wait for the app to idle. */
        let selector: Selector = NSSelectorFromString("_performWithInteractionOptions:block:")
        let implementation: IMP = app.method(for: selector)
        let performWithInteractionOptions: PerformWithInteractionOptions = unsafeBitCast(implementation, to: PerformWithInteractionOptions.self)

        performWithInteractionOptions(app, selector, Self.skipQuiescenceInteractionOptions, actions)
    }

    func scrollUntilHittable(_ element: XCUIElement) {
        var attempts: Int = 0

        while !element.isHittable && attempts < 12 {
            app.swipeUp(velocity: .slow)
            attempts += 1
        }
    }

    func saveScreenshot(_ name: String) {
        let data: Data = XCUIScreen.main.screenshot().pngRepresentation

        try? data.write(to: URL(fileURLWithPath: "\(Self.screenshotDirectory)/\(name).png"))
    }

    static func readExercises(_ unitId: String, courseId: String = LanguageUITestCase.courseId) throws -> [[String: Any]] {
        try readUnit(unitId, courseId: courseId)["exercises"] as? [[String: Any]] ?? []
    }

    static func readUnit(_ unitId: String, courseId: String = LanguageUITestCase.courseId) throws -> [String: Any] {
        let path: URL = repositoryRoot.appendingPathComponent("data/courses/\(courseId)/units/\(unitId).json")
        let data: Data = try Data(contentsOf: path)

        return try JSONSerialization.jsonObject(with: data) as? [String: Any] ?? [:]
    }

    /// The first unit of the level in curriculum order.
    static func readFirstUnitId(levelCode: String) throws -> String {
        let curriculumUnits: [[String: Any]] = try readCurriculumUnits()
        let levelUnit: [String: Any] = try XCTUnwrap(curriculumUnits.first { curriculumUnit in curriculumUnit["level"] as? String == levelCode })

        return try XCTUnwrap(levelUnit["id"] as? String)
    }

    /// The number the glossary shows in the unit's label.
    static func readUnitNumber(unitId: String) throws -> Int {
        let curriculumUnits: [[String: Any]] = try readCurriculumUnits()
        let curriculumUnit: [String: Any] = try XCTUnwrap(curriculumUnits.first { curriculumUnit in curriculumUnit["id"] as? String == unitId })

        return try XCTUnwrap(curriculumUnit["order"] as? Int)
    }

    static func readFirstVocabularyTarget(unitId: String) throws -> String {
        let vocabulary: [[String: Any]] = try readUnit(unitId)["vocabulary"] as? [[String: Any]] ?? []

        return try XCTUnwrap(vocabulary.first?["target"] as? String)
    }

    static func readCurriculumUnits() throws -> [[String: Any]] {
        try readCurriculum()["units"] as? [[String: Any]] ?? []
    }

    static func readCurriculum() throws -> [String: Any] {
        let path: URL = repositoryRoot.appendingPathComponent("data/courses/\(courseId)/curriculum.json")
        let data: Data = try Data(contentsOf: path)

        return try JSONSerialization.jsonObject(with: data) as? [String: Any] ?? [:]
    }

    static func readCourseTitle() throws -> String {
        try readCurriculum()["title"] as? String ?? ""
    }
}

enum TestSetupError: Error, CustomStringConvertible {
    case missingEmbeddedBundle(String)

    var description: String {
        switch self {
        case .missingEmbeddedBundle(let path):
            return "\(path) is missing; run ./scripts/dev/build-local-repository.sh --skip-audio"
        }
    }
}
