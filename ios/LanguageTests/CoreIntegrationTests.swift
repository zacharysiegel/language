import XCTest
@testable import Language

/// Exercises the Rust core through the generated bindings the app uses.
final class CoreIntegrationTests: XCTestCase {
    private var temporaryDirectory: URL!

    override func setUp() {
        temporaryDirectory = FileManager.default.temporaryDirectory.appendingPathComponent(UUID().uuidString, isDirectory: true)
        try? FileManager.default.createDirectory(at: temporaryDirectory, withIntermediateDirectories: true)
    }

    override func tearDown() {
        try? FileManager.default.removeItem(at: temporaryDirectory)
    }

    func testEmbeddedCourseOpensAndAUnitLoads() async throws {
        let embeddedDirectory: URL = try XCTUnwrap(Bundle.main.url(forResource: CoreEnvironment.embeddedBundleDirectoryName, withExtension: nil))
        let library: CourseLibrary = try CourseLibrary(configuration: CoreConfiguration(
            appOrigin: "http://127.0.0.1:9",
            cacheDirectory: temporaryDirectory.path,
            embeddedBundleDirectory: embeddedDirectory.path,
        ))

        let version: CourseVersion = try await library.loadEmbeddedCourse()
        let courseId: String = try XCTUnwrap(version.courses().first?.id)
        let outline: CourseOutlineRecord = try await version.loadCourse(courseId: courseId)
        let lesson: UnitLessonRecord = try await version.loadUnit(courseId: courseId, unitId: outline.units[0].id)

        XCTAssertTrue(version.isEmbedded())
        XCTAssertEqual(outline.targetLanguageName, "Portuguese")
        XCTAssertFalse(outline.units.isEmpty)
        XCTAssertFalse(lesson.exercises.isEmpty)
    }

    /* Needs the local repository served by `./scripts/dev/serve.sh`; skipped when it is not running. */
    func testLatestCourseAndARecordingDownloadFromTheLocalServer() async throws {
        let library: CourseLibrary = try CourseLibrary(configuration: CoreConfiguration(
            appOrigin: "http://127.0.0.1:3000",
            cacheDirectory: temporaryDirectory.path,
            embeddedBundleDirectory: temporaryDirectory.path,
        ))
        let latest: CourseVersion

        do {
            latest = try await library.loadLatestCourse()
        } catch {
            throw XCTSkip("the local server is not running: \(error)")
        }

        let courseId: String = try XCTUnwrap(latest.courses().first?.id)
        let outline: CourseOutlineRecord = try await latest.loadCourse(courseId: courseId)
        let item: VocabularyItemRecord = try XCTUnwrap(outline.units.first?.vocabulary.first)
        let path: String? = try await latest.recordingPath(clipKey: try XCTUnwrap(item.clipKey), engine: .kokoro, spokenText: item.target)
        let cached: CourseVersion? = await library.loadCachedCourse()

        XCTAssertTrue(FileManager.default.fileExists(atPath: try XCTUnwrap(path)))
        XCTAssertEqual(cached?.version(), latest.version())
    }

    func testUnreachableRepositoryThrows() async throws {
        let library: CourseLibrary = try CourseLibrary(configuration: CoreConfiguration(
            appOrigin: "http://127.0.0.1:9",
            cacheDirectory: temporaryDirectory.path,
            embeddedBundleDirectory: temporaryDirectory.path,
        ))

        do {
            _ = try await library.loadLatestCourse()
            XCTFail("an unreachable repository loaded")
        } catch CoreError.Failure(let message, let backtrace) {
            XCTAssertTrue(message.contains("127.0.0.1:9"))
            XCTAssertTrue(backtrace.contains("language_core") || backtrace.contains("shared::"))
        }
    }

    func testProgressBookRecordsAPassedPracticeAndPersists() throws {
        let path: String = temporaryDirectory.appendingPathComponent("progress.json").path
        let book: ProgressBook = ProgressBook.open(path: path)

        let outcome: PracticeOutcomeRecord = try book.recordPracticeSession(courseId: "c", unitId: "u", mode: .full, correctCount: 9, totalCount: 10, vocabularyCount: 3)

        XCTAssertTrue(outcome.passed)
        XCTAssertEqual(outcome.wordsAddedCount, 3)
        XCTAssertTrue(ProgressBook.open(path: path).unitProgress(courseId: "c", unitId: "u").completed)
        XCTAssertEqual(ProgressBook.open(path: path).cardCount(courseId: "other"), 0)
    }

    func testTypedAnswersIgnoreCaseAndFlagAccents() {
        XCTAssertEqual(checkTypedAnswer(typedText: "eu MORO aqui", acceptedAnswers: ["Eu moro aqui."]), .correct)
        XCTAssertEqual(checkTypedAnswer(typedText: "voce", acceptedAnswers: ["você"]), .accentMistake)
        XCTAssertEqual(checkTypedAnswer(typedText: "tchau", acceptedAnswers: ["oi"]), .wrong)
    }

    func testOrderWordsFeedbackSpeaksTheCompletedSentence() {
        let exercise: ExerciseRecord = .fillBlank(promptTarget: "Eu ___ aqui.", answers: ["moro"], hintSource: "I live here.", explanation: nil, clipKey: nil)

        XCTAssertEqual(exerciseSpokenText(exercise: exercise), "Eu moro aqui.")
        XCTAssertTrue(exercise.speaksAfterAnswer())
    }
}
