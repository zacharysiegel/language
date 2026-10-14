package ro.zach.language.audio

import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import ro.zach.language.core.SettingsRecord
import ro.zach.language.design.Glyph
import ro.zach.language.design.GlyphKind
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType
import ro.zach.language.design.PendingGlyphSwap
import ro.zach.language.design.SquareButton
import ro.zach.language.design.SquareButtonPendingKind
import ro.zach.language.design.pendingAccessibility
import ro.zach.language.progressdata.LocalProgressStore
import ro.zach.language.progressdata.ProgressStore

const val PLAY_BUTTON_PENDING_ACCESSIBILITY_VALUE: String = "Loading the recording"
private const val SLOW_SPEED_GLYPH: String = "0.7×"

/** Plays a clip at the learner's chosen voice and speed, or slowly when `slow` is set. */
@Composable
fun PlayButton(clip: AudioClip, modifier: Modifier = Modifier, slow: Boolean = false, label: String? = null) {
    val audioPlayer: AudioPlayer = LocalAudioPlayer.current
    val progressStore: ProgressStore = LocalProgressStore.current
    val control: PlaybackControlKind = if (slow) PlaybackControlKind.SlowSpeed else PlaybackControlKind.NormalSpeed
    val isPending: Boolean = audioPlayer.isIndicatedPending(clip, control)
    val outlineColor: Color = if (isPending) LanguageColors.accentActive else LanguageColors.ink
    val accessibilityLabel: String = if (slow) "Play slowly: ${clip.text}" else "Play audio: ${clip.text}"

    Row(
        modifier = modifier
            .defaultMinSize(minWidth = LanguageSpacing.buttonIconSize, minHeight = LanguageSpacing.buttonIconSize)
            .border(LanguageSpacing.hairline, outlineColor)
            .clickable(interactionSource = null, indication = null, role = Role.Button) {
                val settings: SettingsRecord = progressStore.settings()

                audioPlayer.play(clip, settings.audioEngine, slow || settings.slowAudio, control)
            }
            .semantics(mergeDescendants = true) { contentDescription = accessibilityLabel }
            .pendingAccessibility(isIndicated = isPending, value = PLAY_BUTTON_PENDING_ACCESSIBILITY_VALUE)
            .testTag("play")
            .padding(horizontal = if (label == null) LanguageSpacing.spaceXs else LanguageSpacing.spaceSm),
        horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs, Alignment.CenterHorizontally),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        PendingGlyphSwap(isIndicated = isPending, glyphColor = LanguageColors.accentActive) {
            SpeedGlyph(slow = slow)
        }

        if (label != null) {
            LanguageText(text = label, style = LanguageType.bodyText)
        }
    }
}

@Composable
private fun SpeedGlyph(slow: Boolean) {
    if (slow) {
        LanguageText(text = SLOW_SPEED_GLYPH, style = LanguageType.dataText)
    } else {
        Glyph(glyph = GlyphKind.Play, color = LanguageColors.ink, size = LanguageSpacing.playGlyphSize)
    }
}

/** Plays every clip in order, and stops them. */
@Composable
fun PlayAllButton(clips: List<AudioClip>, modifier: Modifier = Modifier) {
    val audioPlayer: AudioPlayer = LocalAudioPlayer.current
    val progressStore: ProgressStore = LocalProgressStore.current
    val isPending: Boolean = audioPlayer.isIndicatedPending(clips)

    Row(modifier = modifier, horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
        SquareButton(
            onClick = {
                val settings: SettingsRecord = progressStore.settings()

                audioPlayer.playSequence(clips, settings.audioEngine, settings.slowAudio)
            },
            isCompact = true,
            pending = if (isPending) SquareButtonPendingKind.PendingWithinLabel else SquareButtonPendingKind.NotPending,
            modifier = Modifier
                .pendingAccessibility(isIndicated = isPending, value = PLAY_BUTTON_PENDING_ACCESSIBILITY_VALUE)
                .testTag("play-all"),
        ) { contentColor ->
            PendingGlyphSwap(isIndicated = isPending, glyphColor = LanguageColors.accentActive) {
                Glyph(glyph = GlyphKind.Play, color = contentColor)
            }
            LanguageText(text = "Play all", style = LanguageType.bodyText, color = contentColor)
        }

        SquareButton(onClick = audioPlayer::stop, isCompact = true, modifier = Modifier.testTag("stop")) { contentColor ->
            Glyph(glyph = GlyphKind.Stop, color = contentColor)
            LanguageText(text = "Stop", style = LanguageType.bodyText, color = contentColor)
        }
    }
}
