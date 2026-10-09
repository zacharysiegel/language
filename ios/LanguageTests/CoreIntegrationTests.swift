import XCTest
@testable import Language

/// Exercises the Rust core through the generated bindings the app uses.
final class CoreIntegrationTests: XCTestCase {
    static let recordedCourseId: String = "pt_br.en.foundations"

    private var temporaryDirectory: URL!

    override func setUp() {
        temporaryDirectory = FileManager.default.temporaryDirectory.appendingPathComponent(UUID().uuidString, isDirectory: true)
        try? FileManager.default.createDirectory(at: temporaryDirectory, withIntermediateDirectories: true)
    }

    override func tearDown() {
        try? FileManager.default.removeItem(at: temporaryDirectory)
    }

    func testEmbeddedIndexAndPublicationOpenAndAUnitLoads() async throws {
        let embeddedDirectory: URL = try XCTUnwrap(Bundle.main.url(forResource: CoreEnvironment.embeddedBundleDirectoryName, withExtension: nil))
        let library: CourseLibrary = try CourseLibrary(configuration: CoreConfiguration(
            appOrigin: "http://127.0.0.1:9",
            cacheDirectory: temporaryDirectory.path,
            embeddedBundleDirectory: embeddedDirectory.path,
        ))

        let courseIndex: PublishedCourseIndex = try await library.loadEmbeddedCourseIndex()
        let courseId: String = try XCTUnwrap(courseIndex.courses().first?.id)
        let publication: CoursePublication = try await library.loadEmbeddedCoursePublication(courseId: courseId)
        let outline: CourseOutlineRecord = try await publication.loadCourse()
        let lesson: UnitLessonRecord = try await publication.loadUnit(unitId: outline.units[0].id)

        XCTAssertTrue(courseIndex.isEmbedded())
        XCTAssertTrue(publication.isEmbedded())
        XCTAssertEqual(publication.courseId(), courseId)
        XCTAssertFalse(publication.publicationLabel().isEmpty)
        XCTAssertEqual(outline.targetLanguageName, "Portuguese")
        XCTAssertFalse(outline.units.isEmpty)
        XCTAssertFalse(lesson.exercises.isEmpty)
    }

    /* Needs the local repository served by `./scripts/dev/serve.sh`; skipped when it is not running. */
    func testLatestPublicationsAndARecordingDownloadFromTheLocalServer() async throws {
        let library: CourseLibrary = try CourseLibrary(configuration: CoreConfiguration(
            appOrigin: "http://127.0.0.1:3000",
            cacheDirectory: temporaryDirectory.path,
            embeddedBundleDirectory: temporaryDirectory.path,
        ))
        let courseIndex: PublishedCourseIndex

        do {
            courseIndex = try await library.loadLatestCourseIndex()
        } catch {
            throw XCTSkip("the local server is not running: \(error)")
        }

        var latestByCourseId: [String: CoursePublication] = [:]

        for course in courseIndex.courses() {
            latestByCourseId[course.id] = try await library.loadLatestCoursePublication(courseIndex: courseIndex, courseId: course.id)
        }

        for (courseId, latest) in latestByCourseId {
            let cached: CoursePublication? = await library.loadCachedCoursePublication(courseId: courseId)

            XCTAssertEqual(cached?.manifestSha256(), latest.manifestSha256())
            XCTAssertEqual(cached?.publicationLabel(), latest.publicationLabel())
        }

        let latest: CoursePublication = try XCTUnwrap(latestByCourseId[Self.recordedCourseId])
        let outline: CourseOutlineRecord = try await latest.loadCourse()
        let item: VocabularyItemRecord = try XCTUnwrap(outline.units.first?.vocabulary.first)
        let path: String? = try await library.recordingPath(clipKey: try XCTUnwrap(item.clipKey), engine: .kokoro, spokenText: item.target, voiceChoice: .ownText)
        let cachedIndex: PublishedCourseIndex? = await library.loadCachedCourseIndex()

        XCTAssertFalse(courseIndex.isEmbedded())
        XCTAssertEqual(cachedIndex?.courses().count, courseIndex.courses().count)
        XCTAssertTrue(FileManager.default.fileExists(atPath: try XCTUnwrap(path)))
    }

    func testUnreachableRepositoryThrows() async throws {
        let library: CourseLibrary = try CourseLibrary(configuration: CoreConfiguration(
            appOrigin: "http://127.0.0.1:9",
            cacheDirectory: temporaryDirectory.path,
            embeddedBundleDirectory: temporaryDirectory.path,
        ))

        do {
            _ = try await library.loadLatestCourseIndex()
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
        XCTAssertEqual(ProgressBook.open(path: path).unitProgress(courseId: "c", unitId: "u").status, .completed)
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
