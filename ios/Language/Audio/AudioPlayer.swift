import Foundation
import Observation

/// Something to play: a clip's recording, or its text spoken by the system voice when it has none.
struct AudioClip: Equatable {
    let clipKey: String?
    let text: String
    var voiceChoice: VoiceChoiceRecord = .ownText
}

/// Which of a clip's play controls a request is shown on.
enum PlaybackControlKind: Equatable {
    case normalSpeed
    case slowSpeed
}

/// A play request as the control which shows it pending identifies it.
enum PlaybackPendingTarget: Equatable {
    case clip(clipKey: String?, text: String, control: PlaybackControlKind)
    case sequence([AudioClip])
}

/// A play request whose sound has not started; `serial` tells it apart from later requests for the same target.
private struct PendingPlaybackRequest {
    let target: PlaybackPendingTarget
    let serial: Int
}

/// Plays clips one at a time; starting a clip or a sequence stops whatever was playing.
@MainActor
@Observable
final class AudioPlayer {
    static let normalRate: Float = 1.0
    static let slowRate: Float = 0.7

    private let library: CourseLibrary?
    private let output: AudioOutput = AudioOutput()
    private var playbackGeneration: Int = 0
    private var playbackTask: Task<Void, Never>?
    /// Set once the pending request has outlasted `PendingIndication.delay`.
    private var indicatedPendingTarget: PlaybackPendingTarget?

    @ObservationIgnored private var pendingRequest: PendingPlaybackRequest?
    @ObservationIgnored private var pendingRequestSerial: Int = 0
    @ObservationIgnored private var pendingIndicationTask: Task<Void, Never>?

    init(library: CourseLibrary?) {
        self.library = library

        let output: AudioOutput = output

        Task {
            await output.prepareWithoutActivating()
        }
    }

    func isIndicatedPending(clip: AudioClip, control: PlaybackControlKind) -> Bool {
        indicatedPendingTarget == Self.pendingTarget(clip, control: control)
    }

    func isIndicatedPending(sequence clips: [AudioClip]) -> Bool {
        indicatedPendingTarget == .sequence(clips)
    }

    /// Whether a request is waiting for its sound, indicated or not yet.
    func hasPendingRequest() -> Bool {
        pendingRequest != nil
    }

    func stop() {
        playbackGeneration += 1
        playbackTask?.cancel()
        playbackTask = nil
        endAnyPendingRequest()
    }

    func play(_ clip: AudioClip, engine: AudioEngineRecord, slow: Bool, control: PlaybackControlKind) {
        stop()

        let generation: Int = playbackGeneration
        let pendingSerial: Int = beginPendingRequest(Self.pendingTarget(clip, control: control))

        playbackTask = Task {
            _ = await playWithoutStopping(clip, engine: engine, slow: slow, generation: generation, pendingSerial: pendingSerial)
            endPendingRequest(serial: pendingSerial)
        }
    }

    func playSequence(_ clips: [AudioClip], engine: AudioEngineRecord, slow: Bool) {
        stop()

        let generation: Int = playbackGeneration

        playbackTask = Task {
            for clip in clips {
                guard playbackGeneration == generation else {
                    return
                }

                let pendingSerial: Int = beginPendingRequest(.sequence(clips))
                let clipOutcome: PlaybackOutcome = await playWithoutStopping(clip, engine: engine, slow: slow, generation: generation, pendingSerial: pendingSerial)

                endPendingRequest(serial: pendingSerial)

                guard clipOutcome != .interrupted else {
                    return
                }
            }
        }
    }

    private static func pendingTarget(_ clip: AudioClip, control: PlaybackControlKind) -> PlaybackPendingTarget {
        .clip(clipKey: clip.clipKey, text: clip.text, control: control)
    }

    private func beginPendingRequest(_ target: PlaybackPendingTarget) -> Int {
        endAnyPendingRequest()
        pendingRequestSerial += 1

        let serial: Int = pendingRequestSerial

        pendingRequest = PendingPlaybackRequest(target: target, serial: serial)
        pendingIndicationTask = Task {
            let delayOutcome: PendingDelayOutcome = await PendingIndication.waitForDelay()

            guard delayOutcome == .elapsed, pendingRequest?.serial == serial else {
                return
            }

            indicatedPendingTarget = target
        }

        return serial
    }

    /* A request already replaced by a newer one leaves the newer one's pending state alone. */
    private func endPendingRequest(serial: Int) {
        guard pendingRequest?.serial == serial else {
            return
        }

        endAnyPendingRequest()
    }

    private func endAnyPendingRequest() {
        pendingIndicationTask?.cancel()
        pendingIndicationTask = nil
        pendingRequest = nil

        /* Every play control reads this, so it is written only when it changes. */
        if indicatedPendingTarget != nil {
            indicatedPendingTarget = nil
        }
    }

    private func soundStartReport(pendingSerial: Int) -> @Sendable () -> Void {
        { [weak self] in
            Task { @MainActor in
                self?.endPendingRequest(serial: pendingSerial)
            }
        }
    }

    private func playWithoutStopping(_ clip: AudioClip, engine: AudioEngineRecord, slow: Bool, generation: Int, pendingSerial: Int) async -> PlaybackOutcome {
        let recordingPath: String? = await findRecording(clip, engine: engine)
        let rate: Float = slow ? Self.slowRate : Self.normalRate
        let onSoundStarted: @Sendable () -> Void = soundStartReport(pendingSerial: pendingSerial)

        guard playbackGeneration == generation else {
            return .stopped
        }

        guard let recordingPath else {
            return await output.speak(clip.text, rate: rate, onSoundStarted: onSoundStarted)
        }

        let recordingOutcome: PlaybackOutcome = await output.playRecording(path: recordingPath, rate: rate, onSoundStarted: onSoundStarted)

        guard recordingOutcome == .failed, playbackGeneration == generation else {
            return recordingOutcome
        }

        return await output.speak(clip.text, rate: rate, onSoundStarted: onSoundStarted)
    }

    private nonisolated func findRecording(_ clip: AudioClip, engine: AudioEngineRecord) async -> String? {
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
}
