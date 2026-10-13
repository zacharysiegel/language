package ro.zach.language.practice

import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.ProvidableCompositionLocal
import androidx.compose.runtime.staticCompositionLocalOf
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import ro.zach.language.core.AudioEngineRecord
import ro.zach.language.core.SettingsRecord
import ro.zach.language.design.Glyph
import ro.zach.language.design.GlyphKind
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType
import ro.zach.language.progressdata.LocalProgressStore
import ro.zach.language.progressdata.ProgressStore

internal const val PLAY_BUTTON_PENDING_ACCESSIBILITY_VALUE: String = "Loading the recording"
private const val SLOW_SPEED_GLYPH: String = "0.7×"

/*
 * Stand-ins, with the names and parameters of the audio step's `ro.zach.language.audio`, for practice and review.
 * They draw the controls and play nothing; once the audio package lands, this file goes and its imports take their place.
 */

internal data class AudioClip(val clipKey: String?, val text: String)

internal enum class PlaybackControlKind {
    NormalSpeed,
    SlowSpeed,
}

internal class AudioPlayer {
    fun play(clip: AudioClip, engine: AudioEngineRecord, slow: Boolean, control: PlaybackControlKind) {
    }

    fun stop() {
    }

    fun isIndicatedPending(clip: AudioClip, control: PlaybackControlKind): Boolean {
        return false
    }
}

internal val LocalAudioPlayer: ProvidableCompositionLocal<AudioPlayer> = staticCompositionLocalOf { AudioPlayer() }

@Composable
internal fun PlayButton(clip: AudioClip, modifier: Modifier = Modifier, slow: Boolean = false, label: String? = null) {
    val audioPlayer: AudioPlayer = LocalAudioPlayer.current
    val progressStore: ProgressStore = LocalProgressStore.current
    val control: PlaybackControlKind = if (slow) PlaybackControlKind.SlowSpeed else PlaybackControlKind.NormalSpeed
    val accessibilityLabel: String = if (slow) "Play slowly: ${clip.text}" else "Play audio: ${clip.text}"

    Row(
        modifier = modifier
            .defaultMinSize(minWidth = LanguageSpacing.buttonIconSize, minHeight = LanguageSpacing.buttonIconSize)
            .border(LanguageSpacing.hairline, LanguageColors.ink)
            .clickable(interactionSource = null, indication = null, role = Role.Button) {
                val settings: SettingsRecord = progressStore.settings()

                audioPlayer.play(clip, settings.audioEngine, slow || settings.slowAudio, control)
            }
            .semantics(mergeDescendants = true) { contentDescription = accessibilityLabel }
            .testTag("play")
            .padding(horizontal = if (label == null) LanguageSpacing.spaceXs else LanguageSpacing.spaceSm),
        horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs, Alignment.CenterHorizontally),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        if (slow) {
            LanguageText(text = SLOW_SPEED_GLYPH, style = LanguageType.dataText)
        } else {
            Glyph(glyph = GlyphKind.Play, color = LanguageColors.ink, size = LanguageSpacing.spaceMd - LanguageSpacing.spaceXs)
        }

        if (label != null) {
            LanguageText(text = label, style = LanguageType.bodyText)
        }
    }
}
