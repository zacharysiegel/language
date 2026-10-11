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
    /* The unit captioned "A1 · 1". */
    static let firstUnitId: String = "a1-02-sounds-alphabet"
    static let firstUnitLabelPrefix: String = "A1.1."
    static let jumpedNoteNumber: Int = 4
    static let groupedTableUnitId: String = "a1-00-how-portuguese-works"
    static let groupedTableNoteTitle: String = "Gender, number and agreement"
    static let stackedTableUnitId: String = "a1-09-shopping-colors-prices"
    static let stackedTableNoteTitle: String = "Demonstratives: este, esse, aquele"
    static let playedButtonIndexes: [Int] = [0, 0, 1, 2]
    static let clipPlaybackAllowance: TimeInterval = 3
    /* A TEST-NET-1 address (RFC 5737), never routed, so a connection to it waits rather than failing. */
    static let unreachableOrigin: String = "http://192.0.2.1:3002"
    static let pendingRecordingValue: String = "Loading the recording"

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

        let publicationLabel: XCUIElement = app.staticTexts["publication-label"]

        XCTAssertTrue(publicationLabel.waitForExistence(timeout: 10))
        XCTAssertTrue(publicationLabel.label.hasPrefix("Published as "), publicationLabel.label)
        saveScreenshot("11-settings")
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

    func testPlayingClipsLeavesThePageUsable() throws {
        openUnit(Self.practicedUnitId)
        app.buttons["tab-vocabulary"].tap()

        let playButtons: XCUIElementQuery = app.buttons.matching(identifier: "play")

        XCTAssertTrue(playButtons.firstMatch.waitForExistence(timeout: 5))

        for buttonIndex in Self.playedButtonIndexes {
            let playButton: XCUIElement = playButtons.element(boundBy: buttonIndex)

            scrollUntilHittable(playButton)
            playButton.tap()
            Thread.sleep(forTimeInterval: Self.clipPlaybackAllowance)
        }

        app.buttons["tab-overview"].tap()
        XCTAssertTrue(app.buttons["unit-status-not-started"].waitForExistence(timeout: 5))
    }

    func testAPlayWaitingForItsRecordingShowsPendingUntilStopped() throws {
        relaunchWithUnreachableRepository()
        openUnit(Self.practicedUnitId)
        app.buttons["tab-dialogues"].tap()

        let playButton: XCUIElement = app.buttons.matching(identifier: "play").firstMatch
        let playAllButton: XCUIElement = app.buttons["play-all"].firstMatch
        let stopButton: XCUIElement = app.buttons["stop"].firstMatch

        XCTAssertTrue(playButton.waitForExistence(timeout: 5))
        XCTAssertEqual(playButton.value as? String ?? "", "")

        playButton.tap()
        assertValue(playButton, equals: Self.pendingRecordingValue, timeout: 5)
        saveScreenshot("14-play-pending")
        stopButton.tap()
        assertValue(playButton, equals: "", timeout: 5)

        playAllButton.tap()
        assertValue(playAllButton, equals: Self.pendingRecordingValue, timeout: 5)
        XCTAssertEqual(playButton.value as? String ?? "", "")
        saveScreenshot("15-play-all-pending")
        stopButton.tap()
        assertValue(playAllButton, equals: "", timeout: 5)
    }

    @MainActor
    func testPrinciplesNotesAreLabelledAndTheIndexRailJumpsToThem() throws {
        let unit: [String: Any] = try Self.readUnit(Self.firstUnitId)
        let noteCount: Int = (unit["grammar"] as? [[String: Any]] ?? []).count

        openUnit(Self.firstUnitId)
        app.buttons["tab-grammar"].tap()

        let itemLabels: XCUIElementQuery = app.staticTexts.matching(identifier: "unit-item-label")

        XCTAssertTrue(itemLabels.firstMatch.waitForExistence(timeout: 5))
        XCTAssertEqual(itemLabels.count, noteCount)
        XCTAssertEqual(itemLabels.element(boundBy: 0).label, "\(Self.firstUnitLabelPrefix)1")
        XCTAssertEqual(itemLabels.element(boundBy: 1).label, "\(Self.firstUnitLabelPrefix)2")
        assertSelected(railEntry(1))
        try assertNothingReachesUnder(railEntry(1))
        saveScreenshot("16-unit-item-index-rail")

        let jumpedLabel: XCUIElement = itemLabels.element(boundBy: Self.jumpedNoteNumber - 1)

        XCTAssertFalse(jumpedLabel.isHittable)
        XCTAssertEqual(railEntry(Self.jumpedNoteNumber).label, "Go to note \(Self.firstUnitLabelPrefix)\(Self.jumpedNoteNumber)")
        railEntry(Self.jumpedNoteNumber).tap()
        assertNearTopOfThePage(jumpedLabel)
        assertSelected(railEntry(Self.jumpedNoteNumber))
        try assertNothingReachesUnder(railEntry(1))
        saveScreenshot("17-unit-item-index-jump")

        railEntry(1).press(forDuration: 0.1, thenDragTo: railEntry(noteCount))
        XCTAssertTrue(itemLabels.element(boundBy: noteCount - 1).isHittable)
        assertSelected(railEntry(noteCount))
        saveScreenshot("18-unit-item-index-drag")
    }

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

    // MARK: - Choosing the course

    /// Starts the app again with a cache of its own and neither the origin nor the static repository reachable.
    private func relaunchWithUnreachableRepository() {
        app.terminate()
        app.launchArguments = [
            "-LanguageProgressFile", "ui-test-progress-\(UUID().uuidString).json",
            "-LanguageCacheDirectory", "ui-test-cache-\(UUID().uuidString)",
            "-LanguageOrigin", Self.unreachableOrigin,
            "-LanguageRepositoryBaseUrl", Self.unreachableOrigin,
        ]
        app.launch()
        chooseTheCourseIfTheCourseListOpens()
    }

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

    private func assertValue(_ element: XCUIElement, equals value: String, timeout: TimeInterval) {
        let hasValue: XCTNSPredicateExpectation = XCTNSPredicateExpectation(predicate: NSPredicate(format: "value == %@", value), object: element)
        let waitResult: XCTWaiter.Result = XCTWaiter().wait(for: [hasValue], timeout: timeout)

        XCTAssertEqual(waitResult, .completed)
    }

    private func assertGone(_ element: XCUIElement, timeout: TimeInterval) {
        let gone: XCTNSPredicateExpectation = XCTNSPredicateExpectation(predicate: NSPredicate(format: "exists == false"), object: element)
        let waitResult: XCTWaiter.Result = XCTWaiter().wait(for: [gone], timeout: timeout)

        XCTAssertEqual(waitResult, .completed)
    }

    // MARK: - Grammar tables

    private func openPrinciplesNote(unitId: String, noteNumber: Int) {
        openUnit(unitId)
        app.buttons["tab-grammar"].tap()
        XCTAssertTrue(railEntry(noteNumber).waitForExistence(timeout: 5))
        railEntry(noteNumber).tap()
    }

    private func groupedTableKeyLabel(_ header: String) -> XCUIElement {
        app.staticTexts.matching(NSPredicate(format: "identifier == %@ AND label == %@", "grouped-table-key", header.uppercased())).firstMatch
    }

    /// The row of a grouped or stacked table whose spoken text starts with its lead and ends with its last cell, each after its header.
    private func groupedTableRow(headers: [String], cells: [String]) -> XCUIElement {
        let leadPair: String = "\(headers.first ?? ""): \(cells.first ?? "")"
        let lastPair: String = "\(headers.last ?? ""): \(cells.last ?? "")"
        let predicate: NSPredicate = NSPredicate(format: "identifier == %@ AND label BEGINSWITH %@ AND label CONTAINS %@", "grouped-table-row", leadPair, lastPair)

        return app.descendants(matching: .any).matching(predicate).firstMatch
    }

    /// The note titled `noteTitle`, numbered from 1, and its table.
    private static func readGrammarTable(unitId: String, noteTitle: String) throws -> (Int, [String: Any]) {
        let notes: [[String: Any]] = try readUnit(unitId)["grammar"] as? [[String: Any]] ?? []
        let noteIndex: Int = try XCTUnwrap(notes.firstIndex { note in note["title"] as? String == noteTitle })
        let table: [String: Any] = try XCTUnwrap(notes[noteIndex]["table"] as? [String: Any])

        return (noteIndex + 1, table)
    }

    // MARK: - The box index rail

    private func railEntry(_ itemNumber: Int) -> XCUIElement {
        app.buttons["unit-item-index-\(itemNumber)"]
    }

    private func assertNearTopOfThePage(_ element: XCUIElement) {
        let navigationBarBottom: CGFloat = app.navigationBars.firstMatch.frame.maxY

        XCTAssertTrue(element.isHittable)
        XCTAssertGreaterThanOrEqual(element.frame.minY, navigationBarBottom)
        XCTAssertLessThan(element.frame.minY, navigationBarBottom + app.frame.height / 4)
    }

    /// Fails when any text on screen extends to the left edge of the rail or past it.
    @MainActor
    private func assertNothingReachesUnder(_ railEntry: XCUIElement) throws {
        let railLeadingEdge: CGFloat = railEntry.frame.minX
        let screenFrame: CGRect = app.frame
        let textFrames: [CGRect] = try Self.staticTextFrames(in: app.snapshot())
        /* A table cell scrolled out of its sideways scroll view reports a frame past the screen. */
        let onScreenTextFrames: [CGRect] = textFrames.filter { textFrame in screenFrame.contains(textFrame) && !textFrame.isEmpty }

        XCTAssertFalse(onScreenTextFrames.isEmpty)

        for textFrame in onScreenTextFrames {
            XCTAssertLessThanOrEqual(textFrame.maxX, railLeadingEdge)
        }
    }

    @MainActor
    private static func staticTextFrames(in snapshot: XCUIElementSnapshot) -> [CGRect] {
        let ownFrames: [CGRect] = snapshot.elementType == .staticText ? [snapshot.frame] : []
        let childFrames: [CGRect] = snapshot.children.flatMap { child in staticTextFrames(in: child) }

        return ownFrames + childFrames
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
