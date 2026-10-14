package ro.zach.language.settings

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import ro.zach.language.core.AudioEngineRecord
import ro.zach.language.core.CoursePublication
import ro.zach.language.core.SettingsRecord
import ro.zach.language.coursedata.CourseStore
import ro.zach.language.coursedata.LocalCourseStore
import ro.zach.language.design.ChoiceButton
import ro.zach.language.design.ChoiceMarkKind
import ro.zach.language.design.Glyph
import ro.zach.language.design.GlyphKind
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType
import ro.zach.language.design.MicroLabel
import ro.zach.language.design.PageHeader
import ro.zach.language.design.PendingActivity
import ro.zach.language.design.SquareButtonPendingKind
import ro.zach.language.design.SquareTextButton
import ro.zach.language.design.SquareToggle
import ro.zach.language.design.pendingAccessibility
import ro.zach.language.design.readableWidth
import ro.zach.language.design.rememberPendingActivity
import ro.zach.language.progressdata.LocalProgressStore
import ro.zach.language.progressdata.ProgressStore

const val UPDATE_CHECKING_MESSAGE: String = "Checking for updated course content…"
private const val UPDATE_CHECKING_STATE_DESCRIPTION: String = "Checking"

/** The settings page's content; `courseId` is the course open behind it. */
@Composable
fun SettingsSheet(courseId: String?, modifier: Modifier = Modifier) {
    Column(modifier = modifier.fillMaxSize().verticalScroll(rememberScrollState())) {
        Column(
            modifier = Modifier.readableWidth().padding(LanguageSpacing.spaceMd),
            verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceLg),
        ) {
            PageHeader(caption = null, title = "Settings")
            VoiceSection()
            SpeedSection()

            if (courseId != null) {
                CourseContentSection(courseId)
            }
        }
    }
}

/** A voice the recordings can be played in. */
private data class VoiceOption(val identifier: String, val title: String, val detail: String, val engine: AudioEngineRecord)

private val voiceOptions: List<VoiceOption> = listOf(
    VoiceOption(identifier = "auto", title = "Auto", detail = "The macOS voice for one- and two-word clips, the neural voice for longer ones", engine = AudioEngineRecord.AUTO),
    VoiceOption(identifier = "kokoro", title = "Neural (Kokoro)", detail = "Every clip in the neural voice", engine = AudioEngineRecord.KOKORO),
    VoiceOption(identifier = "macos_say", title = "macOS (Luciana)", detail = "Every clip in the macOS voices", engine = AudioEngineRecord.MACOS_SAY),
)

@Composable
private fun VoiceSection() {
    val progressStore: ProgressStore = LocalProgressStore.current
    val chosenEngine: AudioEngineRecord = progressStore.settings().audioEngine

    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
        MicroLabel(text = "Voice")

        for (option in voiceOptions) {
            val isChosen: Boolean = option.engine == chosenEngine

            ChoiceButton(
                mark = if (isChosen) ChoiceMarkKind.Selected else ChoiceMarkKind.Unmarked,
                onClick = { progressStore.updateSettings { settings: SettingsRecord -> settings.copy(audioEngine = option.engine) } },
                modifier = Modifier.testTag("voice-${option.identifier}"),
            ) { contentColor ->
                Column(modifier = Modifier.weight(1f), verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs)) {
                    LanguageText(text = option.title, style = LanguageType.bodyText, color = contentColor)
                    LanguageText(text = option.detail, style = LanguageType.captionText, color = LanguageColors.inkMuted)
                }

                if (isChosen) {
                    Glyph(glyph = GlyphKind.Check, color = LanguageColors.accentActive)
                }
            }
        }
    }
}

@Composable
private fun SpeedSection() {
    val progressStore: ProgressStore = LocalProgressStore.current

    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
        MicroLabel(text = "Speed")

        SquareToggle(
            isOn = progressStore.settings().slowAudio,
            onToggle = { isOn: Boolean -> progressStore.updateSettings { settings: SettingsRecord -> settings.copy(slowAudio = isOn) } },
        ) {
            LanguageText(text = "Slow audio", style = LanguageType.bodyText)
        }
    }
}

@Composable
private fun CourseContentSection(courseId: String) {
    val courseStore: CourseStore = LocalCourseStore.current
    val publication: CoursePublication? = courseStore.shownPublication(courseId)
    val updateCheck: PendingActivity = rememberPendingActivity()

    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
        MicroLabel(text = "Course content")

        if (publication != null) {
            LanguageText(text = "Published as ${publication.publicationLabel()}", style = LanguageType.dataText, modifier = Modifier.testTag("publication-label"))
            LanguageText(text = publicationSourceText(publication), style = LanguageType.captionText, color = LanguageColors.inkMuted)
        }

        UpdateCheckButton(courseId = courseId, updateCheck = updateCheck)

        val message: String? = updateCheckMessage(updateCheck, courseStore.updateMessage(courseId))

        if (message != null) {
            LanguageText(text = message, style = LanguageType.captionText, color = LanguageColors.inkMuted)
        }
    }
}

private fun publicationSourceText(publication: CoursePublication): String {
    return if (publication.isEmbedded()) "The copy included in the app" else "Downloaded from ${publication.repositoryBaseUrl()}"
}

@Composable
private fun UpdateCheckButton(courseId: String, updateCheck: PendingActivity) {
    val courseStore: CourseStore = LocalCourseStore.current
    val isPending: Boolean = updateCheck.isIndicated()

    SquareTextButton(
        text = "Check for updated course content",
        isCompact = true,
        pending = if (isPending) SquareButtonPendingKind.PendingInPlaceOfLabel else SquareButtonPendingKind.NotPending,
        onClick = { updateCheck.run { courseStore.refresh(courseId) } },
        modifier = Modifier
            .pendingAccessibility(isIndicated = isPending, value = UPDATE_CHECKING_STATE_DESCRIPTION)
            .testTag("check-for-updated-content"),
    )
}

private fun updateCheckMessage(updateCheck: PendingActivity, storeMessage: String?): String? {
    return if (updateCheck.isIndicated()) UPDATE_CHECKING_MESSAGE else storeMessage
}
