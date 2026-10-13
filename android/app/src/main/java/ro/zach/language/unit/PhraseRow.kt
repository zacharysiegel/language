package ro.zach.language.unit

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.text.font.FontStyle
import ro.zach.language.core.PhrasePairRecord
import ro.zach.language.core.VoiceChoiceRecord
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType
import ro.zach.language.design.MicroLabel
import ro.zach.language.design.SquareToggle
import ro.zach.language.progressdata.LocalProgressStore
import ro.zach.language.progressdata.ProgressStore

/** Toggles the translations and literal glosses in a lesson. */
@Composable
fun TranslationToggle(modifier: Modifier = Modifier) {
    val progressStore: ProgressStore = LocalProgressStore.current

    SquareToggle(
        isOn = progressStore.settings().showTranslations,
        onToggle = { isOn: Boolean -> progressStore.updateSettings { settings -> settings.copy(showTranslations = isOn) } },
        modifier = modifier.testTag("show-translations"),
    ) {
        MicroLabel(text = "Show translations")
    }
}

/** A spoken target-language phrase with its translation and, when it differs, its literal gloss. */
@Composable
fun PhraseRow(
    pair: PhrasePairRecord,
    modifier: Modifier = Modifier,
    speaker: String? = null,
    voiceChoice: VoiceChoiceRecord = VoiceChoiceRecord.OwnText,
) {
    val progressStore: ProgressStore = LocalProgressStore.current
    val showsTranslations: Boolean = progressStore.settings().showTranslations

    Row(
        modifier = modifier.fillMaxWidth().padding(vertical = LanguageSpacing.spaceXs),
        horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm),
        verticalAlignment = Alignment.Top,
    ) {
        PlayButton(clip = AudioClip(clipKey = pair.clipKey, text = pair.target, voiceChoice = voiceChoice))

        Column(modifier = Modifier.weight(1f), verticalArrangement = Arrangement.spacedBy(LanguageSpacing.phraseLineGap)) {
            if (speaker != null) {
                MicroLabel(text = speaker)
            }

            LanguageText(text = pair.target, style = LanguageType.targetText)

            if (showsTranslations) {
                PhraseTranslation(pair = pair)
            }
        }
    }
}

@Composable
private fun PhraseTranslation(pair: PhrasePairRecord) {
    val literal: String? = pair.sourceLiteral

    LanguageText(text = pair.source, style = LanguageType.bodyText, color = LanguageColors.inkMuted)

    if (literal != null) {
        LiteralGloss(literal = literal)
    }
}

/** A phrase's word-for-word gloss, after "literally:". */
@Composable
fun LiteralGloss(literal: String, modifier: Modifier = Modifier) {
    LanguageText(
        text = "literally: $literal",
        style = LanguageType.captionText.copy(fontStyle = FontStyle.Italic),
        color = LanguageColors.inkMuted,
        modifier = modifier,
    )
}
