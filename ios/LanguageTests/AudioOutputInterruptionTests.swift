import AVFoundation
import XCTest
@testable import Language

final class AudioOutputInterruptionTests: XCTestCase {
    static let silentRecordingSeconds: Double = 5.0
    static let sampleRate: Double = 44_100.0
    static let activityPollNanoseconds: UInt64 = 10_000_000
    static let activityPollLimit: Int = 500

    private var temporaryDirectory: URL!

    override func setUp() {
        temporaryDirectory = FileManager.default.temporaryDirectory.appendingPathComponent(UUID().uuidString, isDirectory: true)
        try? FileManager.default.createDirectory(at: temporaryDirectory, withIntermediateDirectories: true)
    }

    override func tearDown() {
        try? FileManager.default.removeItem(at: temporaryDirectory)
    }

    func testInterruptionEvent_readsTypeAndResumeOption() {
        let began: Notification = Self.interruptionNotification(type: .began, options: nil)
        let endedShouldResume: Notification = Self.interruptionNotification(type: .ended, options: .shouldResume)
        let endedWithoutOptions: Notification = Self.interruptionNotification(type: .ended, options: nil)
        let withoutType: Notification = Notification(name: AVAudioSession.interruptionNotification)

        XCTAssertEqual(AudioOutput.interruptionEvent(began), .began)
        XCTAssertEqual(AudioOutput.interruptionEvent(endedShouldResume), .endedShouldResume)
        XCTAssertEqual(AudioOutput.interruptionEvent(endedWithoutOptions), .endedShouldNotResume)
        XCTAssertNil(AudioOutput.interruptionEvent(withoutType))
    }

    func testPlayRecording_returnsInterruptedWhenAnInterruptionEndsWithoutResumption() async throws {
        let recordingUrl: URL = try writeSilentRecording()
        let output: AudioOutput = AudioOutput()
        let playback: Task<PlaybackOutcome, Never> = Task {
            await output.playRecording(path: recordingUrl.path, rate: 1.0)
        }

        try await waitUntilPlaying(output)
        await output.observeInterruption(.began)
        await output.observeInterruption(.endedShouldNotResume)

        let outcome: PlaybackOutcome = await playback.value

        XCTAssertEqual(outcome, .interrupted)
    }

    private func waitUntilPlaying(_ output: AudioOutput) async throws {
        for _ in 0..<Self.activityPollLimit {
            let activity: AudioOutputActivityKind = await output.activity()

            guard activity == .idle else {
                return
            }

            try await Task.sleep(nanoseconds: Self.activityPollNanoseconds)
        }

        XCTFail()
    }

    private func writeSilentRecording() throws -> URL {
        let recordingUrl: URL = temporaryDirectory.appendingPathComponent("silence.caf")
        let format: AVAudioFormat = try XCTUnwrap(AVAudioFormat(standardFormatWithSampleRate: Self.sampleRate, channels: 1))
        let frameCount: AVAudioFrameCount = AVAudioFrameCount(Self.sampleRate * Self.silentRecordingSeconds)
        let buffer: AVAudioPCMBuffer = try XCTUnwrap(AVAudioPCMBuffer(pcmFormat: format, frameCapacity: frameCount))
        let file: AVAudioFile = try AVAudioFile(forWriting: recordingUrl, settings: format.settings)

        buffer.frameLength = frameCount
        try file.write(from: buffer)

        return recordingUrl
    }

    private static func interruptionNotification(type: AVAudioSession.InterruptionType, options: AVAudioSession.InterruptionOptions?) -> Notification {
        var userInfo: [AnyHashable: Any] = [AVAudioSessionInterruptionTypeKey: type.rawValue]

        if let options {
            userInfo[AVAudioSessionInterruptionOptionKey] = options.rawValue
        }

        return Notification(name: AVAudioSession.interruptionNotification, object: nil, userInfo: userInfo)
    }

}
