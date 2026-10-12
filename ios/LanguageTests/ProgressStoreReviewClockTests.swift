import Observation
import XCTest
@testable import Language

@MainActor
final class ProgressStoreReviewClockTests: XCTestCase {
    static let courseId: String = "pt_br.en.foundations"
    static let cardDueDelayMilliseconds: Int64 = 1500
    static let changeTimeout: TimeInterval = 10

    private var temporaryDirectory: URL!

    override func setUp() {
        temporaryDirectory = FileManager.default.temporaryDirectory.appendingPathComponent(UUID().uuidString, isDirectory: true)
        try? FileManager.default.createDirectory(at: temporaryDirectory, withIntermediateDirectories: true)
    }

    override func tearDown() {
        try? FileManager.default.removeItem(at: temporaryDirectory)
    }

    func testDueCardCountChangesWhenACardComesDue() async throws {
        let nowMilliseconds: Int64 = Int64((Date().timeIntervalSince1970 * 1000).rounded(.down))
        let store: ProgressStore = try storeWithCardsDueAt([nowMilliseconds - 1000, nowMilliseconds + Self.cardDueDelayMilliseconds])
        let countChanged: XCTestExpectation = expectation(description: "the due count changes")

        XCTAssertEqual(store.dueCardCount(courseId: Self.courseId), 1)
        XCTAssertEqual(store.dueCardIds(courseId: Self.courseId, limit: 10), ["a:0"])

        withObservationTracking {
            _ = store.dueCardCount(courseId: Self.courseId)
        } onChange: {
            countChanged.fulfill()
        }

        await fulfillment(of: [countChanged], timeout: Self.changeTimeout)

        XCTAssertEqual(store.dueCardCount(courseId: Self.courseId), 2)
        XCTAssertEqual(store.dueCardIds(courseId: Self.courseId, limit: 10), ["a:0", "a:1"])
    }

    private func storeWithCardsDueAt(_ dueMilliseconds: [Int64]) throws -> ProgressStore {
        let progressPath: URL = temporaryDirectory.appendingPathComponent("progress.json")
        var cards: [String: [String: Any]] = [:]

        for (cardIndex, due) in dueMilliseconds.enumerated() {
            let cardId: String = "a:\(cardIndex)"

            cards[cardId] = ["id": cardId, "box_index": 0, "due_milliseconds": due, "lapse_count": 0, "review_count": 0]
        }

        let progress: [String: Any] = ["courses": [Self.courseId: ["cards": cards]]]
        let progressData: Data = try JSONSerialization.data(withJSONObject: progress)

        try progressData.write(to: progressPath)

        return ProgressStore(environment: CoreEnvironment(
            configuration: CoreConfiguration(
                appOrigin: "http://127.0.0.1:9",
                staticRepositoryBaseUrl: "http://127.0.0.1:9",
                cacheDirectory: temporaryDirectory.path,
                embeddedBundleDirectory: temporaryDirectory.path,
            ),
            progressPath: progressPath.path,
        ))
    }
}
