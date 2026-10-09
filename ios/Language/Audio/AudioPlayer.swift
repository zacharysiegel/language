import AVFoundation
import Foundation
import Observation

/// Something to play: a clip's recording, or its text spoken by the system voice when it has none.
struct AudioClip: Equatable {
    let clipKey: String?
    let text: String
    var voiceChoice: VoiceChoiceRecord = .ownText
}

/// The app's audio session as the player last set or observed it.
enum AudioSessionActivationKind {
    case inactive
    case active
}

/// Plays clips one at a time; starting a clip or a sequence stops whatever was playing.
@MainActor
@Observable
final class AudioPlayer {
    static let slowRate: Float = 0.7
    static let systemVoiceLanguage: String = "pt-BR"

    private let library: CourseLibrary?
    private var playbackGeneration: Int = 0
    private var activePlayback: ClipPlayback?
    private let speechSynthesizer: AVSpeechSynthesizer = AVSpeechSynthesizer()
    private let speechDelegate: SpeechDelegate = SpeechDelegate()
    private var sessionActivation: AudioSessionActivationKind = .inactive
    private var interruptionObserver: NSObjectProtocol?

    init(library: CourseLibrary?) {
        self.library = library
        speechSynthesizer.delegate = speechDelegate
        try? AVAudioSession.sharedInstance().setCategory(.playback, mode: .spokenAudio)

        interruptionObserver = NotificationCenter.default.addObserver(
            forName: AVAudioSession.interruptionNotification,
            object: AVAudioSession.sharedInstance(),
            queue: .main,
        ) { [weak self] notification in
            let interruptionType: AVAudioSession.InterruptionType? = Self.interruptionType(notification)

            MainActor.assumeIsolated {
                self?.observeInterruption(interruptionType)
            }
        }
    }

    func stop() {
        playbackGeneration += 1
        activePlayback?.stop()
        activePlayback = nil
        speechSynthesizer.stopSpeaking(at: .immediate)
        speechDelegate.finishCurrent()
    }

    func play(_ clip: AudioClip, engine: AudioEngineRecord, slow: Bool) {
        stop()

        let generation: Int = playbackGeneration

        Task {
            await playWithoutStopping(clip, engine: engine, slow: slow, generation: generation)
        }
    }

    func playSequence(_ clips: [AudioClip], engine: AudioEngineRecord, slow: Bool) {
        stop()

        let generation: Int = playbackGeneration

        Task {
            for clip in clips {
                guard playbackGeneration == generation else {
                    return
                }

                await playWithoutStopping(clip, engine: engine, slow: slow, generation: generation)
            }
        }
    }

    private func playWithoutStopping(_ clip: AudioClip, engine: AudioEngineRecord, slow: Bool, generation: Int) async {
        let recordingPath: String? = await findRecording(clip, engine: engine)

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

    private func findRecording(_ clip: AudioClip, engine: AudioEngineRecord) async -> String? {
        guard let clipKey = clip.clipKey, let library else {
            return nil
        }

        let lookup: PlaybackStepInterval = PlaybackTiming.begin(.recordingLookup)

        defer {
            PlaybackTiming.end(lookup)
        }

        do {
            return try await library.recordingPath(clipKey: clipKey, engine: engine, spokenText: clip.text, voiceChoice: clip.voiceChoice)
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
        activateSessionIfInactive()

        return await playback.play()
    }

    private func speakWithSystemVoice(_ text: String, slow: Bool) async {
        let utterance: AVSpeechUtterance = AVSpeechUtterance(string: text)
        let voiceLookup: PlaybackStepInterval = PlaybackTiming.begin(.systemVoiceLookup)
        let voice: AVSpeechSynthesisVoice? = AVSpeechSynthesisVoice(language: Self.systemVoiceLanguage)

        PlaybackTiming.end(voiceLookup)
        utterance.voice = voice
        utterance.rate = AVSpeechUtteranceDefaultSpeechRate * (slow ? Self.slowRate : 1.0)
        activateSessionIfInactive()

        await withCheckedContinuation { (continuation: CheckedContinuation<Void, Never>) in
            speechDelegate.begin(continuation)

            let speakStart: PlaybackStepInterval = PlaybackTiming.begin(.systemVoiceSpeakStart)

            speechSynthesizer.speak(utterance)
            PlaybackTiming.end(speakStart)
        }
    }

    private func activateSessionIfInactive() {
        guard sessionActivation == .inactive else {
            return
        }

        let activation: PlaybackStepInterval = PlaybackTiming.begin(.sessionActivation)
        let activated: Result<Void, Error> = Result { try AVAudioSession.sharedInstance().setActive(true) }

        PlaybackTiming.end(activation)

        switch activated {
        case .success:
            sessionActivation = .active
        case .failure(let error):
            _ = CoreErrorReport.record(error, while: "activating the audio session")
        }
    }

    /* The system deactivates the session when an interruption begins and leaves reactivating it to the app. */
    private func observeInterruption(_ interruptionType: AVAudioSession.InterruptionType?) {
        if interruptionType == .began {
            sessionActivation = .inactive
        }
    }

    private nonisolated static func interruptionType(_ notification: Notification) -> AVAudioSession.InterruptionType? {
        let rawValue: UInt? = notification.userInfo?[AVAudioSessionInterruptionTypeKey] as? UInt

        return rawValue.flatMap(AVAudioSession.InterruptionType.init(rawValue:))
    }
}

/// One recording playing, resolving its `play()` when it ends, fails, or is stopped.
@MainActor
private final class ClipPlayback: NSObject, AVAudioPlayerDelegate {
    private let player: AVAudioPlayer
    private var continuation: CheckedContinuation<Bool, Never>?

    init?(path: String, rate: Float) {
        let creation: PlaybackStepInterval = PlaybackTiming.begin(.playerCreation)
        let createdPlayer: AVAudioPlayer? = try? AVAudioPlayer(contentsOf: URL(fileURLWithPath: path))

        PlaybackTiming.end(creation)

        guard let player = createdPlayer else {
            return nil
        }

        self.player = player
        super.init()
        player.enableRate = true
        player.rate = rate
        player.delegate = self
    }

    func play() async -> Bool {
        let preparation: PlaybackStepInterval = PlaybackTiming.begin(.playerPreparation)

        player.prepareToPlay()
        PlaybackTiming.end(preparation)

        return await withCheckedContinuation { (continuation: CheckedContinuation<Bool, Never>) in
            self.continuation = continuation

            let playStart: PlaybackStepInterval = PlaybackTiming.begin(.playStart)
            let started: Bool = player.play()

            PlaybackTiming.end(playStart)

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
