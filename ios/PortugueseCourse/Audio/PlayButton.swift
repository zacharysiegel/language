import SwiftUI

/// Plays a clip at the learner's chosen voice and speed, or slowly when `slow` is set.
struct PlayButton: View {
    let clip: AudioClip
    var slow: Bool = false
    var label: String? = nil

    @Environment(AudioPlayer.self) private var audioPlayer
    @Environment(CourseStore.self) private var courseStore
    @Environment(ProgressStore.self) private var progressStore

    var body: some View {
        Button {
            let settings: SettingsRecord = progressStore.settings()

            audioPlayer.play(clip, version: courseStore.shownVersion(), engine: settings.audioEngine, slow: slow || settings.slowAudio)
        } label: {
            HStack(spacing: .spaceXs) {
                if slow {
                    Text("0.7×")
                        .font(.dataText)
                } else {
                    Image(systemName: "play.fill")
                        .font(.system(size: 11))
                }

                if let label {
                    Text(label)
                        .font(.bodyText)
                }
            }
            .frame(minWidth: .buttonIconSize, minHeight: .buttonIconSize)
            .padding(.horizontal, label == nil ? 0 : .spaceSm)
            .foregroundStyle(Color.ink)
            .overlay(Rectangle().stroke(Color.ink, lineWidth: .hairline))
            .contentShape(Rectangle())
        }
        .buttonStyle(.plain)
        .accessibilityLabel(slow ? "Play slowly: \(clip.text)" : "Play audio: \(clip.text)")
        .accessibilityIdentifier("play")
    }
}

/// Plays every clip in order, and stops them.
struct PlayAllButton: View {
    let clips: [AudioClip]

    @Environment(AudioPlayer.self) private var audioPlayer
    @Environment(CourseStore.self) private var courseStore
    @Environment(ProgressStore.self) private var progressStore

    var body: some View {
        HStack(spacing: .spaceSm) {
            Button {
                let settings: SettingsRecord = progressStore.settings()

                audioPlayer.playSequence(clips, version: courseStore.shownVersion(), engine: settings.audioEngine, slow: settings.slowAudio)
            } label: {
                Label("Play all", systemImage: "play.fill")
            }
            .buttonStyle(SquareButtonStyle(isCompact: true))

            Button {
                audioPlayer.stop()
            } label: {
                Label("Stop", systemImage: "stop.fill")
            }
            .buttonStyle(SquareButtonStyle(isCompact: true))
        }
    }
}
