import SwiftUI

/// Plays a clip at the learner's chosen voice and speed, or slowly when `slow` is set.
struct PlayButton: View {
    static let glyphFont: Font = .system(size: 11)
    static let pendingAccessibilityValue: String = "Loading the recording"

    let clip: AudioClip
    var slow: Bool = false
    var label: String? = nil

    @Environment(AudioPlayer.self) private var audioPlayer
    @Environment(ProgressStore.self) private var progressStore

    var body: some View {
        let isPending: Bool = audioPlayer.isIndicatedPending(clip: clip, control: control())

        Button {
            let settings: SettingsRecord = progressStore.settings()

            audioPlayer.play(clip, engine: settings.audioEngine, slow: slow || settings.slowAudio, control: control())
        } label: {
            HStack(spacing: .spaceXs) {
                speedGlyph()
                    .pendingGlyphSwap(isIndicated: isPending, glyphFont: Self.glyphFont)
                    .foregroundStyle(isPending ? Color.accentActive : Color.ink)

                if let label {
                    Text(label)
                        .font(.bodyText)
                        .foregroundStyle(Color.ink)
                }
            }
            .frame(minWidth: .buttonIconSize, minHeight: .buttonIconSize)
            .padding(.horizontal, label == nil ? 0 : .spaceSm)
            .overlay(Rectangle().stroke(isPending ? Color.accentActive : Color.ink, lineWidth: .hairline))
            .contentShape(Rectangle())
        }
        .buttonStyle(.plain)
        .accessibilityLabel(slow ? "Play slowly: \(clip.text)" : "Play audio: \(clip.text)")
        .pendingAccessibility(isIndicated: isPending, value: Self.pendingAccessibilityValue)
        .accessibilityIdentifier("play")
    }

    private func control() -> PlaybackControlKind {
        slow ? .slowSpeed : .normalSpeed
    }

    @ViewBuilder
    private func speedGlyph() -> some View {
        if slow {
            Text("0.7×")
                .font(.dataText)
        } else {
            Image(systemName: "play.fill")
                .font(Self.glyphFont)
        }
    }
}

/// Plays every clip in order, and stops them.
struct PlayAllButton: View {
    let clips: [AudioClip]

    @Environment(AudioPlayer.self) private var audioPlayer
    @Environment(ProgressStore.self) private var progressStore

    var body: some View {
        let isPending: Bool = audioPlayer.isIndicatedPending(sequence: clips)

        HStack(spacing: .spaceSm) {
            Button {
                let settings: SettingsRecord = progressStore.settings()

                audioPlayer.playSequence(clips, engine: settings.audioEngine, slow: settings.slowAudio)
            } label: {
                Label {
                    Text("Play all")
                } icon: {
                    Image(systemName: "play.fill")
                        .pendingGlyphSwap(isIndicated: isPending, glyphFont: .bodyText)
                        .foregroundStyle(isPending ? Color.accentActive : Color.ink)
                }
            }
            .buttonStyle(SquareButtonStyle(isCompact: true, pending: isPending ? .pendingWithinLabel : .notPending))
            .pendingAccessibility(isIndicated: isPending, value: PlayButton.pendingAccessibilityValue)
            .accessibilityIdentifier("play-all")

            Button {
                audioPlayer.stop()
            } label: {
                Label("Stop", systemImage: "stop.fill")
            }
            .buttonStyle(SquareButtonStyle(isCompact: true))
            .accessibilityIdentifier("stop")
        }
    }
}
