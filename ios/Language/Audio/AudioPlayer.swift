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
    static let normalRate: Float = 1.0
    static let slowRate: Float = 0.7

    private let library: CourseLibrary?
    private let output: AudioOutput = AudioOutput()
    private var playbackGeneration: Int = 0
    private var playbackTask: Task<Void, Never>?

    init(library: CourseLibrary?) {
        self.library = library

        let output: AudioOutput = output

        Task {
            await output.prepareWithoutActivating()
        }
    }

    func stop() {
        playbackGeneration += 1
        playbackTask?.cancel()
        playbackTask = nil
    }

    func play(_ clip: AudioClip, engine: AudioEngineRecord, slow: Bool) {
        stop()

        let generation: Int = playbackGeneration

        playbackTask = Task {
            await playWithoutStopping(clip, engine: engine, slow: slow, generation: generation)
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

                await playWithoutStopping(clip, engine: engine, slow: slow, generation: generation)
            }
        }
    }

    private func playWithoutStopping(_ clip: AudioClip, engine: AudioEngineRecord, slow: Bool, generation: Int) async {
        let recordingPath: String? = await findRecording(clip, engine: engine)
        let rate: Float = slow ? Self.slowRate : Self.normalRate

        guard playbackGeneration == generation else {
            return
        }

        guard let recordingPath else {
            await output.speak(clip.text, rate: rate)
            return
        }

        let recordingOutcome: PlaybackOutcome = await output.playRecording(path: recordingPath, rate: rate)

        guard recordingOutcome == .failed, playbackGeneration == generation else {
            return
        }

        await output.speak(clip.text, rate: rate)
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
