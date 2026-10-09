import AVFoundation
import Foundation
import Synchronization

/// How a recording or utterance ended.
enum PlaybackOutcome: Sendable {
    case finished
    case failed
    case stopped
    case interrupted
}

/// The app's audio session as the output last set or observed it.
private enum AudioSessionActivationKind {
    case inactive
    case active
    case interrupted
}

/// Whether the session category is set and the session notifications are observed.
private enum AudioOutputPreparationKind {
    case unprepared
    case prepared
}

/// An audio session interruption as its notification reports it.
enum AudioSessionInterruptionEvent: Sendable {
    case began
    case endedShouldResume
    case endedShouldNotResume
}

/// Whether the output has a sound it started and has not yet ended.
enum AudioOutputActivityKind {
    case idle
    case playing
}

/// The system voice for `AudioOutput.systemVoiceLanguage`, once looked up; `nil` when it is not installed.
private enum SystemVoiceLookupState {
    case notLookedUp
    case lookedUp(AVSpeechSynthesisVoice?)
}

/// The sound the output started last and has not yet ended.
private enum CurrentPlayback {
    case nothing
    case recording(AVAudioPlayer, RecordingPlayerDelegate)
    case speech(SpeechUtteranceDelegate)

    func completion() -> PlaybackCompletion? {
        switch self {
        case .nothing:
            return nil
        case .recording(_, let delegate):
            return delegate.completion
        case .speech(let delegate):
            return delegate.completion
        }
    }
}

/// Owns the audio session and the system players, so their blocking calls run off the main thread.
actor AudioOutput {
    static let systemVoiceLanguage: String = "pt-BR"

    private var preparation: AudioOutputPreparationKind = .unprepared
    private var sessionActivation: AudioSessionActivationKind = .inactive
    private var systemVoiceLookup: SystemVoiceLookupState = .notLookedUp
    private var speechSynthesizer: AVSpeechSynthesizer?
    private var currentPlayback: CurrentPlayback = .nothing

    /// Sets the session category and loads the system voice.
    func prepareWithoutActivating() {
        prepareIfUnprepared()
        _ = systemVoice()
        _ = speechSynthesizerCreatingIfAbsent()
    }

    /// Returns once the recording has ended, failed, been stopped or been interrupted.
    func playRecording(path: String, rate: Float) async -> PlaybackOutcome {
        prepareIfUnprepared()
        stopCurrentPlayback(endingWith: .stopped)

        let createdPlayer: AVAudioPlayer? = Self.createPlayer(path: path, rate: rate)

        guard let player = createdPlayer else {
            return .failed
        }

        guard !Task.isCancelled else {
            return .stopped
        }

        activateSessionUnlessActive()
        Self.prepareToPlay(player)

        guard !Task.isCancelled else {
            return .stopped
        }

        let delegate: RecordingPlayerDelegate = RecordingPlayerDelegate(completion: PlaybackCompletion())
        let completion: PlaybackCompletion = delegate.completion

        player.delegate = delegate
        currentPlayback = .recording(player, delegate)

        let outcome: PlaybackOutcome = await withTaskCancellationHandler {
            Self.startPlaying(player, completion: completion)

            return await completion.outcome()
        } onCancel: {
            completion.finish(.stopped)
        }

        endPlayback(completion)

        return outcome
    }

    /// Returns once the utterance has ended, been stopped or been interrupted.
    func speak(_ text: String, rate: Float) async -> PlaybackOutcome {
        prepareIfUnprepared()
        stopCurrentPlayback(endingWith: .stopped)

        let voice: AVSpeechSynthesisVoice? = systemVoice()
        let synthesizer: AVSpeechSynthesizer = speechSynthesizerCreatingIfAbsent()

        guard !Task.isCancelled else {
            return .stopped
        }

        activateSessionUnlessActive()

        guard !Task.isCancelled else {
            return .stopped
        }

        let utterance: AVSpeechUtterance = AVSpeechUtterance(string: text)

        utterance.voice = voice
        utterance.rate = AVSpeechUtteranceDefaultSpeechRate * rate

        let delegate: SpeechUtteranceDelegate = SpeechUtteranceDelegate(utterance: utterance, completion: PlaybackCompletion())
        let completion: PlaybackCompletion = delegate.completion

        synthesizer.delegate = delegate
        currentPlayback = .speech(delegate)

        let outcome: PlaybackOutcome = await withTaskCancellationHandler {
            Self.startSpeaking(utterance, synthesizer: synthesizer)

            return await completion.outcome()
        } onCancel: {
            completion.finish(.stopped)
        }

        endPlayback(completion)

        return outcome
    }

    func activity() -> AudioOutputActivityKind {
        switch currentPlayback {
        case .nothing:
            return .idle
        case .recording, .speech:
            return .playing
        }
    }

    private func prepareIfUnprepared() {
        guard preparation == .unprepared else {
            return
        }

        preparation = .prepared
        Self.configureSessionCategory()
        observeSessionNotifications()
    }

    private func systemVoice() -> AVSpeechSynthesisVoice? {
        switch systemVoiceLookup {
        case .lookedUp(let voice):
            return voice
        case .notLookedUp:
            let voiceLookup: PlaybackStepInterval = PlaybackTiming.begin(.systemVoiceLookup)
            let voice: AVSpeechSynthesisVoice? = AVSpeechSynthesisVoice(language: Self.systemVoiceLanguage)

            PlaybackTiming.end(voiceLookup)
            systemVoiceLookup = .lookedUp(voice)

            return voice
        }
    }

    private func speechSynthesizerCreatingIfAbsent() -> AVSpeechSynthesizer {
        if let speechSynthesizer {
            return speechSynthesizer
        }

        let synthesizer: AVSpeechSynthesizer = AVSpeechSynthesizer()

        speechSynthesizer = synthesizer

        return synthesizer
    }

    private func stopCurrentPlayback(endingWith outcome: PlaybackOutcome) {
        switch currentPlayback {
        case .nothing:
            break
        case .recording(let player, _):
            player.stop()
        case .speech:
            speechSynthesizer?.stopSpeaking(at: .immediate)
        }

        currentPlayback.completion()?.finish(outcome)
        currentPlayback = .nothing
    }

    /* A newer play may already have replaced and stopped this one. */
    private func endPlayback(_ completion: PlaybackCompletion) {
        guard currentPlayback.completion() === completion else {
            return
        }

        stopCurrentPlayback(endingWith: .stopped)
    }

    private func activateSessionUnlessActive() {
        guard sessionActivation != .active else {
            return
        }

        activateSession()
    }

    private func activateSession() {
        let activation: PlaybackStepInterval = PlaybackTiming.begin(.sessionActivation)
        let activated: Result<Void, Error> = Result { try AVAudioSession.sharedInstance().setActive(true) }

        PlaybackTiming.end(activation)

        switch activated {
        case .success:
            sessionActivation = .active
        case .failure(let error):
            sessionActivation = .inactive
            _ = CoreErrorReport.record(error, while: "activating the audio session")
        }
    }

    /* The system deactivates the session and pauses playback when an interruption begins; reactivating and resuming are left to the app. */
    func observeInterruption(_ interruption: AudioSessionInterruptionEvent?) {
        switch (interruption, sessionActivation) {
        case (.began, .active):
            sessionActivation = .interrupted
        case (.endedShouldResume, .interrupted):
            resumeAfterInterruption()
        case (.endedShouldNotResume, .interrupted):
            sessionActivation = .inactive
            stopCurrentPlayback(endingWith: .interrupted)
        default:
            break
        }
    }

    private func resumeAfterInterruption() {
        activateSession()

        guard sessionActivation == .active else {
            stopCurrentPlayback(endingWith: .interrupted)
            return
        }

        switch currentPlayback {
        case .nothing:
            break
        case .recording(let player, let delegate):
            let resumed: Bool = player.play()

            if !resumed {
                delegate.completion.finish(.interrupted)
            }
        case .speech:
            _ = speechSynthesizer?.continueSpeaking()
        }
    }

    /* A reset invalidates every player and the session's configuration. */
    private func observeMediaServicesReset() {
        let activationBeforeReset: AudioSessionActivationKind = sessionActivation

        stopCurrentPlayback(endingWith: .stopped)
        speechSynthesizer = nil
        sessionActivation = .inactive
        Self.configureSessionCategory()

        if activationBeforeReset != .inactive {
            activateSession()
        }
    }

    private func observeSessionNotifications() {
        let notificationCenter: NotificationCenter = NotificationCenter.default
        let session: AVAudioSession = AVAudioSession.sharedInstance()

        _ = notificationCenter.addObserver(
            forName: AVAudioSession.interruptionNotification,
            object: session,
            queue: nil,
        ) { [weak self] notification in
            let interruption: AudioSessionInterruptionEvent? = Self.interruptionEvent(notification)

            guard let self else {
                return
            }

            Task {
                await self.observeInterruption(interruption)
            }
        }

        _ = notificationCenter.addObserver(
            forName: AVAudioSession.mediaServicesWereResetNotification,
            object: session,
            queue: nil,
        ) { [weak self] _ in
            guard let self else {
                return
            }

            Task {
                await self.observeMediaServicesReset()
            }
        }
    }

    private static func configureSessionCategory() {
        let configuration: PlaybackStepInterval = PlaybackTiming.begin(.sessionConfiguration)
        let configured: Result<Void, Error> = Result { try AVAudioSession.sharedInstance().setCategory(.playback, mode: .spokenAudio) }

        PlaybackTiming.end(configuration)

        if case .failure(let error) = configured {
            _ = CoreErrorReport.record(error, while: "setting the audio session category")
        }
    }

    private static func createPlayer(path: String, rate: Float) -> AVAudioPlayer? {
        let creation: PlaybackStepInterval = PlaybackTiming.begin(.playerCreation)
        let createdPlayer: AVAudioPlayer? = try? AVAudioPlayer(contentsOf: URL(fileURLWithPath: path))

        PlaybackTiming.end(creation)
        createdPlayer?.enableRate = true
        createdPlayer?.rate = rate

        return createdPlayer
    }

    private static func prepareToPlay(_ player: AVAudioPlayer) {
        let preparation: PlaybackStepInterval = PlaybackTiming.begin(.playerPreparation)

        player.prepareToPlay()
        PlaybackTiming.end(preparation)
    }

    private static func startPlaying(_ player: AVAudioPlayer, completion: PlaybackCompletion) {
        let playStart: PlaybackStepInterval = PlaybackTiming.begin(.playStart)
        let started: Bool = player.play()

        PlaybackTiming.end(playStart)

        if !started {
            completion.finish(.failed)
        }
    }

    private static func startSpeaking(_ utterance: AVSpeechUtterance, synthesizer: AVSpeechSynthesizer) {
        let speakStart: PlaybackStepInterval = PlaybackTiming.begin(.systemVoiceSpeakStart)

        synthesizer.speak(utterance)
        PlaybackTiming.end(speakStart)
    }

    static func interruptionEvent(_ notification: Notification) -> AudioSessionInterruptionEvent? {
        let typeRawValue: UInt? = notification.userInfo?[AVAudioSessionInterruptionTypeKey] as? UInt
        let optionsRawValue: UInt? = notification.userInfo?[AVAudioSessionInterruptionOptionKey] as? UInt
        let interruptionType: AVAudioSession.InterruptionType? = typeRawValue.flatMap(AVAudioSession.InterruptionType.init(rawValue:))
        let options: AVAudioSession.InterruptionOptions = optionsRawValue.map(AVAudioSession.InterruptionOptions.init(rawValue:)) ?? []

        guard let interruptionType else {
            return nil
        }

        switch interruptionType {
        case .began:
            return .began
        case .ended:
            return options.contains(.shouldResume) ? .endedShouldResume : .endedShouldNotResume
        @unknown default:
            return nil
        }
    }
}

/// Where a `PlaybackCompletion` is between its start and the single outcome it resolves to.
private enum PlaybackCompletionState: Sendable {
    case pending
    case awaited(CheckedContinuation<PlaybackOutcome, Never>)
    case finished(PlaybackOutcome)
}

/// The end of one recording or utterance; the first `finish` decides the outcome, from any thread.
private final class PlaybackCompletion: Sendable {
    private let state: Mutex<PlaybackCompletionState> = Mutex(.pending)

    func finish(_ outcome: PlaybackOutcome) {
        let awaitingContinuation: CheckedContinuation<PlaybackOutcome, Never>? = state.withLock { state in
            switch state {
            case .pending:
                state = .finished(outcome)
                return nil
            case .awaited(let continuation):
                state = .finished(outcome)
                return continuation
            case .finished:
                return nil
            }
        }

        awaitingContinuation?.resume(returning: outcome)
    }

    /// Awaits the outcome; called at most once.
    func outcome() async -> PlaybackOutcome {
        await withCheckedContinuation { (continuation: CheckedContinuation<PlaybackOutcome, Never>) in
            let finishedOutcome: PlaybackOutcome? = state.withLock { state in
                switch state {
                case .pending:
                    state = .awaited(continuation)
                    return nil
                case .awaited:
                    preconditionFailure("a playback completion was awaited twice")
                case .finished(let outcome):
                    return outcome
                }
            }

            if let finishedOutcome {
                continuation.resume(returning: finishedOutcome)
            }
        }
    }
}

/// Finishes a recording's completion when its player ends.
private final class RecordingPlayerDelegate: NSObject, AVAudioPlayerDelegate, Sendable {
    let completion: PlaybackCompletion

    init(completion: PlaybackCompletion) {
        self.completion = completion
    }

    func audioPlayerDidFinishPlaying(_ player: AVAudioPlayer, successfully flag: Bool) {
        completion.finish(flag ? .finished : .failed)
    }

    func audioPlayerDecodeErrorDidOccur(_ player: AVAudioPlayer, error: Error?) {
        completion.finish(.failed)
    }
}

/// Finishes one utterance's completion, ignoring callbacks for utterances spoken before it.
private final class SpeechUtteranceDelegate: NSObject, AVSpeechSynthesizerDelegate, Sendable {
    let completion: PlaybackCompletion
    private let utteranceIdentifier: ObjectIdentifier

    init(utterance: AVSpeechUtterance, completion: PlaybackCompletion) {
        self.utteranceIdentifier = ObjectIdentifier(utterance)
        self.completion = completion
    }

    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didFinish utterance: AVSpeechUtterance) {
        finishIfOwn(utterance, outcome: .finished)
    }

    func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didCancel utterance: AVSpeechUtterance) {
        finishIfOwn(utterance, outcome: .stopped)
    }

    private func finishIfOwn(_ utterance: AVSpeechUtterance, outcome: PlaybackOutcome) {
        guard ObjectIdentifier(utterance) == utteranceIdentifier else {
            return
        }

        completion.finish(outcome)
    }
}
