package ro.zach.language.unit

import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.Dp
import ro.zach.language.core.ExerciseRecord
import ro.zach.language.core.VoiceChoiceRecord
import ro.zach.language.design.Glyph
import ro.zach.language.design.GlyphKind
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType
import ro.zach.language.design.SquareButton
import ro.zach.language.design.SquareTextButton

/*
 * Stand-ins, with the iOS names and parameters, for the audio and practice packages. Once they land, `AudioClip`,
 * `PlayButton` and `PlayAllButton` come from `ro.zach.language.audio` and `ReadingQuestionsSession` draws practice's
 * `ExerciseSession` with the quit exit.
 */

private val playGlyphSize: Dp = LanguageSpacing.spaceMd - LanguageSpacing.spaceXs

data class AudioClip(
    val clipKey: String?,
    val text: String,
    val voiceChoice: VoiceChoiceRecord = VoiceChoiceRecord.OwnText,
)

@Composable
fun PlayButton(clip: AudioClip, modifier: Modifier = Modifier) {
    Box(
        modifier = modifier
            .defaultMinSize(minWidth = LanguageSpacing.buttonIconSize, minHeight = LanguageSpacing.buttonIconSize)
            .border(LanguageSpacing.hairline, LanguageColors.ink)
            .clickable(interactionSource = null, indication = null, role = Role.Button, onClick = {})
            .semantics { contentDescription = "Play audio: ${clip.text}" }
            .testTag("play"),
        contentAlignment = Alignment.Center,
    ) {
        Glyph(glyph = GlyphKind.Play, color = LanguageColors.ink, size = playGlyphSize)
    }
}

@Composable
fun PlayAllButton(clips: List<AudioClip>, modifier: Modifier = Modifier) {
    Row(modifier = modifier, horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
        SquareButton(onClick = {}, isCompact = true, modifier = Modifier.testTag("play-all")) { contentColor ->
            Glyph(glyph = GlyphKind.Play, color = contentColor)
            LanguageText(text = "Play all", style = LanguageType.bodyText, color = contentColor)
        }

        SquareButton(onClick = {}, isCompact = true, modifier = Modifier.testTag("stop")) { contentColor ->
            Glyph(glyph = GlyphKind.Stop, color = contentColor)
            LanguageText(text = "Stop", style = LanguageType.bodyText, color = contentColor)
        }
    }
}

/** A reading's comprehension questions as an exercise session that can only be quit. */
@Composable
fun ReadingQuestionsSession(title: String, questions: List<ExerciseRecord>, onExit: () -> Unit, modifier: Modifier = Modifier) {
    Column(modifier = modifier, verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
        LanguageText(text = title, style = LanguageType.titleText)
        LanguageText(text = "${questions.size} questions", style = LanguageType.captionText, color = LanguageColors.inkMuted)
        SquareTextButton(text = "Quit", onClick = onExit, isCompact = true, modifier = Modifier.testTag("quit"))
    }
}
