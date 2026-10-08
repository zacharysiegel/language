import SwiftUI

/// Toggles the translations and literal glosses in a lesson.
struct TranslationToggle: View {
    @Environment(ProgressStore.self) private var progressStore

    var body: some View {
        Toggle(isOn: Binding(
            get: { progressStore.settings().showTranslations },
            set: { isOn in progressStore.updateSettings { settings in settings.showTranslations = isOn } },
        )) {
            MicroLabel(text: "Show translations")
        }
        .toggleStyle(SquareToggleStyle())
        .accessibilityIdentifier("show-translations")
    }
}

/// A spoken target-language phrase with its translation and, when it differs, its literal gloss.
struct PhraseRow: View {
    let pair: PhrasePairRecord
    var speaker: String? = nil

    @Environment(ProgressStore.self) private var progressStore

    var body: some View {
        HStack(alignment: .top, spacing: .spaceSm) {
            PlayButton(clip: AudioClip(clipKey: pair.clipKey, text: pair.target))

            VStack(alignment: .leading, spacing: 2) {
                if let speaker {
                    MicroLabel(text: speaker)
                }

                Text(pair.target)
                    .font(.targetText)
                    .fixedSize(horizontal: false, vertical: true)

                if progressStore.settings().showTranslations {
                    Text(pair.source)
                        .font(.bodyText)
                        .foregroundStyle(Color.inkMuted)
                        .fixedSize(horizontal: false, vertical: true)

                    if let literal = pair.sourceLiteral {
                        Text("literally: \(literal)")
                            .font(.captionText)
                            .italic()
                            .foregroundStyle(Color.inkMuted)
                            .fixedSize(horizontal: false, vertical: true)
                    }
                }
            }

            Spacer(minLength: 0)
        }
        .padding(.vertical, .spaceXs)
    }
}
