import AVFoundation
import Foundation
import Observation

/// Something to play: a clip's recording, or its text spoken by the system voice when it has none.
struct AudioClip: Equatable {
    let clipKey: String?
    let text: String
    var voiceChoice: VoiceChoiceRecord = .ownText
}

/// Plays clips one at a time; starting a clip or a sequence stops whatever was playing.
@MainActor
@Observable
final class AudioPlayer {
    static let slowRate: Float = 0.7
    static let systemVoiceLanguage: String = "pt-BR"

    private var playbackGeneration: Int = 0
    private var activePlayback: ClipPlayback?
    private let speechSynthesizer: AVSpeechSynthesizer = AVSpeechSynthesizer()
    private let speechDelegate: SpeechDelegate = SpeechDelegate()

    init() {
        speechSynthesizer.delegate = speechDelegate
        try? AVAudioSession.sharedInstance().setCategory(.playback, mode: .spokenAudio)
    }

    func stop() {
        playbackGeneration += 1
        activePlayback?.stop()
        activePlayback = nil
        speechSynthesizer.stopSpeaking(at: .immediate)
        speechDelegate.finishCurrent()
    }

    func play(_ clip: AudioClip, version: CourseVersion?, engine: AudioEngineRecord, slow: Bool) {
        stop()

        let generation: Int = playbackGeneration

        Task {
            await playWithoutStopping(clip, version: version, engine: engine, slow: slow, generation: generation)
        }
    }

    func playSequence(_ clips: [AudioClip], version: CourseVersion?, engine: AudioEngineRecord, slow: Bool) {
        stop()

        let generation: Int = playbackGeneration

        Task {
            for clip in clips {
                guard playbackGeneration == generation else {
                    return
                }

                await playWithoutStopping(clip, version: version, engine: engine, slow: slow, generation: generation)
            }
        }
    }

    private func playWithoutStopping(_ clip: AudioClip, version: CourseVersion?, engine: AudioEngineRecord, slow: Bool, generation: Int) async {
        let recordingPath: String? = await findRecording(clip, version: version, engine: engine)

        guard playbackGeneration == generation else {
            return
        }

        guard let recordingPath else {
            await speakWithSystemVoice(clip.text, slow: slow)
            return
        }

        let played: Bool = await playFile(path: recordingPath, slow: slow)

        if !played, playbackGeneration == generation {
            await speakWithSystemVoice(clip.text, slow: slow)
        }
    }

    private func findRecording(_ clip: AudioClip, version: CourseVersion?, engine: AudioEngineRecord) async -> String? {
        guard let clipKey = clip.clipKey, let version else {
            return nil
        }

        do {
            return try await version.recordingPath(clipKey: clipKey, engine: engine, spokenText: clip.text, voiceChoice: clip.voiceChoice)
        } catch {
            _ = CoreErrorReport.record(error, while: "finding the recording of clip \(clipKey)")
            return nil
        }
    }

    private func playFile(path: String, slow: Bool) async -> Bool {
        guard let playback = ClipPlayback(path: path, rate: slow ? Self.slowRate : 1.0) else {
            return false
        }

        activePlayback = playback

        return await playback.play()
    }

    private func speakWithSystemVoice(_ text: String, slow: Bool) async {
        let utterance: AVSpeechUtterance = AVSpeechUtterance(string: text)

        utterance.voice = AVSpeechSynthesisVoice(language: Self.systemVoiceLanguage)
        utterance.rate = AVSpeechUtteranceDefaultSpeechRate * (slow ? Self.slowRate : 1.0)

        await withCheckedContinuation { (continuation: CheckedContinuation<Void, Never>) in
            speechDelegate.begin(continuation)
            speechSynthesizer.speak(utterance)
        }
    }
}

/// One recording playing, resolving its `play()` when it ends, fails, or is stopped.
@MainActor
private final class ClipPlayback: NSObject, AVAudioPlayerDelegate {
    private let player: AVAudioPlayer
    private var continuation: CheckedContinuation<Bool, Never>?

    init?(path: String, rate: Float) {
        guard let player = try? AVAudioPlayer(contentsOf: URL(fileURLWithPath: path)) else {
            return nil
        }

        self.player = player
        super.init()
        player.enableRate = true
        player.rate = rate
        player.delegate = self
    }

    func play() async -> Bool {
        await withCheckedContinuation { (continuation: CheckedContinuation<Bool, Never>) in
            self.continuation = continuation

            let started: Bool = player.play()

            if !started {
                finish(succeeded: false)
            }
        }
    }

    func stop() {
        player.stop()
        finish(succeeded: true)
    }

    nonisolated func audioPlayerDidFinishPlaying(_ player: AVAudioPlayer, successfully flag: Bool) {
        MainActor.assumeIsolated {
            finish(succeeded: flag)
        }
    }

    nonisolated func audioPlayerDecodeErrorDidOccur(_ player: AVAudioPlayer, error: Error?) {
        MainActor.assumeIsolated {
            finish(succeeded: false)
        }
    }

    private func finish(succeeded: Bool) {
        continuation?.resume(returning: succeeded)
        continuation = nil
    }
}

/// Resolves the pending system-voice utterance when it finishes or is cancelled.
private final class SpeechDelegate: NSObject, AVSpeechSynthesizerDelegate, @unchecked Sendable {
    private let lock: NSLock = NSLock()
    private var continuation: CheckedContinuation<Void, Never>?

    func begin(_ continuation: CheckedContinuation<Void, Never>) {
        finishCurrent()
        lock.withLock { self.continuation = continuation }
    }

    func finishCurrent() {
        let pending: CheckedContinuation<Void, Never>? = lock.withLock {
            let pending: CheckedContinuation<Void, Never>? = continuation
            continuation = nil
            return pending
        }

        pending?.resume()
    }

    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didFinish utterance: AVSpeechUtterance) {
        finishCurrent()
    }

    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didCancel utterance: AVSpeechUtterance) {
        finishCurrent()
    }
}
