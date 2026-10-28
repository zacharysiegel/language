import XCTest

/// Playing a clip and waiting for its recording.
final class AudioUITests: LanguageUITestCase {
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
}
