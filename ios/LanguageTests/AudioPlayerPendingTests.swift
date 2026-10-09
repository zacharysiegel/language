import XCTest
@testable import Language

@MainActor
final class AudioPlayerPendingTests: XCTestCase {
    static let pollInterval: Duration = .milliseconds(20)
    static let pollLimit: Int = 500

    func testPlay_endsThePendingRequestOnceTheSystemVoiceStartsSpeaking() async throws {
        let audioPlayer: AudioPlayer = AudioPlayer(library: nil)

        audioPlayer.play(AudioClip(clipKey: nil, text: "Olá"), engine: .auto, slow: false, control: .normalSpeed)
        XCTAssertTrue(audioPlayer.hasPendingRequest())

        try await waitUntilNoPendingRequest(audioPlayer)
        audioPlayer.stop()
    }

    func testStop_endsThePendingRequest() {
        let audioPlayer: AudioPlayer = AudioPlayer(library: nil)

        audioPlayer.play(AudioClip(clipKey: nil, text: "Olá"), engine: .auto, slow: false, control: .normalSpeed)
        XCTAssertTrue(audioPlayer.hasPendingRequest())
        audioPlayer.stop()

        XCTAssertFalse(audioPlayer.hasPendingRequest())
    }

    private func waitUntilNoPendingRequest(_ audioPlayer: AudioPlayer) async throws {
        for _ in 0..<Self.pollLimit {
            guard audioPlayer.hasPendingRequest() else {
                return
            }

            try await Task.sleep(for: Self.pollInterval)
        }

        XCTFail()
    }
}
