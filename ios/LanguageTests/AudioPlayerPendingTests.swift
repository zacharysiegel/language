import AVFoundation
import Synchronization
import XCTest
@testable import Language

@MainActor
final class AudioPlayerPendingTests: XCTestCase {
    static let pollInterval: Duration = .milliseconds(20)
    static let pollLimit: Int = 3000
    static let speechProbePollLimit: Int = 1000

    func testPlay_endsThePendingRequestOnceTheSystemVoiceStartsSpeaking() async throws {
        try await skipUnlessTheSystemVoiceCanStartSpeaking()

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

    /// A simulator clone can lack a working system voice, which no app code can make speak.
    private func skipUnlessTheSystemVoiceCanStartSpeaking() async throws {
        let probe: SpeechStartProbe = SpeechStartProbe()
        let synthesizer: AVSpeechSynthesizer = AVSpeechSynthesizer()

        synthesizer.delegate = probe
        synthesizer.speak(AVSpeechUtterance(string: "Olá"))

        for _ in 0..<Self.speechProbePollLimit {
            if probe.hasStarted() {
                synthesizer.stopSpeaking(at: .immediate)
                return
            }

            try await Task.sleep(for: Self.pollInterval)
        }

        synthesizer.stopSpeaking(at: .immediate)
        throw XCTSkip("the system voice does not start speaking on this simulator")
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

private final class SpeechStartProbe: NSObject, AVSpeechSynthesizerDelegate, Sendable {
    private let started: Mutex<Bool> = Mutex(false)

    func hasStarted() -> Bool {
        started.withLock { started in started }
    }

    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didStart utterance: AVSpeechUtterance) {
        started.withLock { started in started = true }
    }
}
