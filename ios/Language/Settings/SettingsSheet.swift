import SwiftUI

struct SettingsSheet: View {
    static let checkingMessage: String = "Checking for updated course content…"
    static let checkingAccessibilityValue: String = "Checking"

    /// The course open behind the sheet, if any.
    let courseId: String?

    @Environment(ProgressStore.self) private var progressStore
    @Environment(CourseStore.self) private var courseStore
    @Environment(\.dismiss) private var dismiss
    @State private var updateCheck: PendingActivity = PendingActivity()

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: .spaceLg) {
                    voiceSection()
                    speedSection()
                    courseContentSection()
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

    @ViewBuilder
    private func courseContentSection() -> some View {
        if let courseId {
            VStack(alignment: .leading, spacing: .spaceSm) {
                MicroLabel(text: "Course content")

                if let publication = courseStore.shownPublication(courseId: courseId) {
                    Text("Published as \(publication.publicationLabel())")
                        .font(.dataText)
                        .accessibilityIdentifier("publication-label")
                    Text(publication.isEmbedded() ? "The copy included in the app" : "Downloaded from \(publication.repositoryBaseUrl())")
                        .font(.captionText)
                        .foregroundStyle(Color.inkMuted)
                }

                updateCheckButton(courseId: courseId)
                updateCheckMessage(courseId: courseId)
            }
        }
    }

    private func updateCheckButton(courseId: String) -> some View {
        let isPending: Bool = updateCheck.isIndicated()

        return Button("Check for updated course content") {
            updateCheck.run {
                await courseStore.refresh(courseId: courseId)
            }
        }
        .buttonStyle(SquareButtonStyle(isCompact: true, pending: isPending ? .pendingInPlaceOfLabel : .notPending))
        .pendingAccessibility(isIndicated: isPending, value: Self.checkingAccessibilityValue)
        .accessibilityIdentifier("check-for-updated-content")
    }

    @ViewBuilder
    private func updateCheckMessage(courseId: String) -> some View {
        if updateCheck.isIndicated() {
            Text(Self.checkingMessage).font(.captionText).foregroundStyle(Color.inkMuted)
        } else if let message = courseStore.updateMessage(courseId: courseId) {
            Text(message).font(.captionText).foregroundStyle(Color.inkMuted)
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
