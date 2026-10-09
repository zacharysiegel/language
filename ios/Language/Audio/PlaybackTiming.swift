import Foundation
import OSLog

/// A step on the way from a play request to sound.
enum PlaybackStepKind {
    case recordingLookup
    case sessionActivation
    case playerCreation
    case playerPreparation
    case playStart
    case systemVoiceLookup
    case systemVoiceSpeakStart

    func name() -> StaticString {
        switch self {
        case .recordingLookup:
            return "recording_lookup"
        case .sessionActivation:
            return "session_activation"
        case .playerCreation:
            return "player_creation"
        case .playerPreparation:
            return "player_preparation"
        case .playStart:
            return "play_start"
        case .systemVoiceLookup:
            return "system_voice_lookup"
        case .systemVoiceSpeakStart:
            return "system_voice_speak_start"
        }
    }
}

/// A step being timed, from `PlaybackTiming.begin` until it is passed to `PlaybackTiming.end`.
struct PlaybackStepInterval {
    let step: PlaybackStepKind
    let signpostState: OSSignpostIntervalState
    let startUptimeNanoseconds: UInt64
    let startedOnMainThread: Bool
}

/// Times playback steps as signpost intervals for Instruments, and as a debug line per step for `log show`.
enum PlaybackTiming {
    static let category: String = "audio"
    static let nanosecondsPerMillisecond: Double = 1_000_000
    static let signposter: OSSignposter = OSSignposter(subsystem: LogSubsystem.identifier, category: category)
    static let logger: Logger = Logger(subsystem: LogSubsystem.identifier, category: category)

    static func begin(_ step: PlaybackStepKind) -> PlaybackStepInterval {
        let startedOnMainThread: Bool = isMainThread()
        let signpostState: OSSignpostIntervalState = signposter.beginInterval(step.name(), id: signposter.makeSignpostID(), "main_thread=\(startedOnMainThread)")

        return PlaybackStepInterval(
            step: step,
            signpostState: signpostState,
            startUptimeNanoseconds: DispatchTime.now().uptimeNanoseconds,
            startedOnMainThread: startedOnMainThread,
        )
    }

    static func end(_ interval: PlaybackStepInterval) {
        let elapsedNanoseconds: UInt64 = DispatchTime.now().uptimeNanoseconds - interval.startUptimeNanoseconds
        let durationMilliseconds: Double = Double(elapsedNanoseconds) / nanosecondsPerMillisecond
        let stepName: String = String(describing: interval.step.name())

        signposter.endInterval(interval.step.name(), interval.signpostState)
        logger.debug("playback step finished; [step=\(stepName, privacy: .public) duration_ms=\(durationMilliseconds, format: .fixed(precision: 2)) main_thread=\(interval.startedOnMainThread)]")
    }

    private static func isMainThread() -> Bool {
        pthread_main_np() != 0
    }
}
