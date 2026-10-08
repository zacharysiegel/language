import SwiftUI

struct SettingsSheet: View {
    @Environment(ProgressStore.self) private var progressStore
    @Environment(CourseStore.self) private var courseStore
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: .spaceLg) {
                    voiceSection()
                    speedSection()
                    versionSection()
                }
                .padding(.spaceMd)
                .readableWidth()
            }
            .background(Color.paper)
            .navigationTitle("Settings")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .confirmationAction) {
                    Button("Done") { dismiss() }
                }
            }
        }
    }

    private func voiceSection() -> some View {
        VStack(alignment: .leading, spacing: .spaceSm) {
            MicroLabel(text: "Voice")

            ForEach(VoiceOption.all, id: \.title) { option in
                Button {
                    progressStore.updateSettings { settings in settings.audioEngine = option.engine }
                } label: {
                    HStack {
                        VStack(alignment: .leading, spacing: 2) {
                            Text(option.title).font(.bodyText)
                            Text(option.detail).font(.captionText).foregroundStyle(Color.inkMuted)
                        }

                        Spacer()

                        if progressStore.settings().audioEngine == option.engine {
                            Image(systemName: "checkmark").foregroundStyle(Color.accentActive)
                        }
                    }
                }
                .buttonStyle(ChoiceButtonStyle(mark: progressStore.settings().audioEngine == option.engine ? .selected : .unmarked))
                .accessibilityIdentifier("voice-\(option.identifier)")
            }
        }
    }

    private func speedSection() -> some View {
        VStack(alignment: .leading, spacing: .spaceSm) {
            MicroLabel(text: "Speed")

            Toggle(isOn: Binding(
                get: { progressStore.settings().slowAudio },
                set: { isOn in progressStore.updateSettings { settings in settings.slowAudio = isOn } },
            )) {
                Text("Slow audio").font(.bodyText)
            }
            .toggleStyle(SquareToggleStyle())
        }
    }

    private func versionSection() -> some View {
        VStack(alignment: .leading, spacing: .spaceSm) {
            MicroLabel(text: "Course version")

            if let version = courseStore.shownVersion() {
                Text(version.version()).font(.dataText)
                Text(version.isEmbedded() ? "Shipped with the app" : version.repositoryBaseUrl())
                    .font(.captionText)
                    .foregroundStyle(Color.inkMuted)
            }

            Button("Check for a newer version") {
                Task { await courseStore.refresh() }
            }
            .buttonStyle(SquareButtonStyle(isCompact: true))

            if let message = courseStore.lastUpdateMessage() {
                Text(message).font(.captionText).foregroundStyle(Color.inkMuted)
            }
        }
    }
}

private struct VoiceOption {
    let identifier: String
    let title: String
    let detail: String
    let engine: AudioEngineRecord

    static let all: [VoiceOption] = [
        VoiceOption(identifier: "auto", title: "Auto", detail: "The macOS voice for one- and two-word clips, the neural voice for longer ones", engine: .auto),
        VoiceOption(identifier: "kokoro", title: "Neural (Kokoro)", detail: "Every clip in the neural voice", engine: .kokoro),
        VoiceOption(identifier: "macos_say", title: "macOS (Luciana)", detail: "Every clip in the macOS voices", engine: .macosSay),
    ]
}
