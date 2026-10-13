package ro.zach.language.progresspage

import android.content.ClipData
import android.content.ClipboardManager
import android.content.Context
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.Role
import ro.zach.language.app.LocalRootNavigation
import ro.zach.language.app.RootNavigationState
import ro.zach.language.app.UnitDestination
import ro.zach.language.core.CourseOutlineRecord
import ro.zach.language.core.UnitOutlineRecord
import ro.zach.language.core.reviewIntervalDays
import ro.zach.language.design.HairlineRule
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType
import ro.zach.language.design.PageHeader
import ro.zach.language.design.SquareTextButton
import ro.zach.language.design.SquareTextField
import ro.zach.language.design.UnitStatusTag
import ro.zach.language.design.readableWidth
import ro.zach.language.progressdata.LocalProgressStore
import ro.zach.language.progressdata.ProgressChangeOutcome
import ro.zach.language.progressdata.ProgressStore
import ro.zach.language.unit.UnitTabKind

private const val EXPORT_CLIP_LABEL: String = "Progress"

@Composable
fun ProgressPage(outline: CourseOutlineRecord, modifier: Modifier = Modifier) {
    Column(modifier = modifier.fillMaxSize().verticalScroll(rememberScrollState())) {
        Column(
            modifier = Modifier.readableWidth().padding(LanguageSpacing.spaceMd),
            verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceLg),
        ) {
            ProgressSummary(outline)
            UnitsSection(outline)
            ReviewDeckSection(outline)
            ProgressTransfer()
        }
    }
}

@Composable
private fun ProgressSummary(outline: CourseOutlineRecord) {
    val progressStore: ProgressStore = LocalProgressStore.current

    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs)) {
        PageHeader(caption = null, title = "Progress")
        LanguageText(
            text = "${progressStore.completedUnitCount(outline.id)} / ${outline.units.size} units completed",
            style = LanguageType.dataText,
            color = LanguageColors.inkMuted,
        )
    }
}

@Composable
private fun UnitsSection(outline: CourseOutlineRecord) {
    val progressStore: ProgressStore = LocalProgressStore.current
    val rootNavigation: RootNavigationState = LocalRootNavigation.current

    Column {
        SectionTitle(text = "Units")
        HairlineRule()

        for (unit in outline.units) {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .clickable(interactionSource = null, indication = null, role = Role.Button) {
                        rootNavigation.openUnit(UnitDestination(courseId = outline.id, unitId = unit.id, tab = UnitTabKind.Overview))
                    }
                    .padding(vertical = LanguageSpacing.spaceSm),
                horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                LanguageText(text = unitRowText(unit), style = LanguageType.bodyText, color = LanguageColors.accentLink, modifier = Modifier.weight(1f))
                UnitStatusTag(unitProgress = progressStore.unitProgress(outline.id, unit.id))
            }

            HairlineRule()
        }
    }
}

private fun unitRowText(unit: UnitOutlineRecord): String {
    return "${unit.levelCode} ${unit.order}: ${unit.title}"
}

@Composable
private fun ReviewDeckSection(outline: CourseOutlineRecord) {
    val progressStore: ProgressStore = LocalProgressStore.current
    val intervalDays: List<Int> = remember { reviewIntervalDays().map { days -> days.toInt() } }
    val cardCounts: List<Int> = progressStore.countByBox(outline.id)

    Column {
        SectionTitle(text = "Review deck")
        HairlineRule()

        for ((boxIntervalDays, cardCount) in intervalDays.zip(cardCounts)) {
            Row(modifier = Modifier.fillMaxWidth().padding(vertical = LanguageSpacing.spaceSm), verticalAlignment = Alignment.CenterVertically) {
                LanguageText(text = reviewBoxLabel(boxIntervalDays), style = LanguageType.bodyText, modifier = Modifier.weight(1f))
                LanguageText(text = "$cardCount", style = LanguageType.dataText)
            }

            HairlineRule()
        }
    }
}

/** A review box's label from the days its cards wait before their next review. */
fun reviewBoxLabel(intervalDays: Int): String {
    return if (intervalDays == 0) "New or forgotten" else "Next review after $intervalDays day(s)"
}

@Composable
private fun SectionTitle(text: String) {
    LanguageText(text = text, style = LanguageType.titleText, modifier = Modifier.padding(bottom = LanguageSpacing.spaceSm))
}

/** Whether the next press of the reset button erases the progress. */
private enum class ResetConfirmationKind {
    NotRequested,
    Requested,
}

/** Copies progress out as text, restores it from text, or erases it. */
@Composable
private fun ProgressTransfer() {
    val progressStore: ProgressStore = LocalProgressStore.current
    val context: Context = LocalContext.current
    /* Not saved with the page: an exported progress file can exceed the saved state's size limit. */
    var transferText: String by remember { mutableStateOf("") }
    var message: String? by rememberSaveable { mutableStateOf(null) }
    var resetConfirmation: ResetConfirmationKind by rememberSaveable { mutableStateOf(ResetConfirmationKind.NotRequested) }

    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
        LanguageText(text = "Back up or move your progress", style = LanguageType.titleText)
        LanguageText(
            text = "Progress is saved on this device only. Export it to keep a backup, or paste a backup to restore it.",
            style = LanguageType.captionText,
            color = LanguageColors.inkMuted,
        )

        Row(horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
            SquareTextButton(
                text = "Export",
                isCompact = true,
                onClick = {
                    val exported: String? = progressStore.exportJson()

                    if (exported == null) {
                        message = "Could not export: ${progressStore.lastError() ?: "unreadable progress"}"
                    } else {
                        transferText = exported
                        copyToClipboard(context, exported)
                        message = "Progress exported and copied."
                    }
                },
            )

            SquareTextButton(
                text = "Import from the text box",
                isCompact = true,
                onClick = {
                    val outcome: ProgressChangeOutcome = progressStore.importJson(transferText)

                    message = when (outcome) {
                        ProgressChangeOutcome.Saved -> "Progress imported."
                        ProgressChangeOutcome.Failed -> "Could not import: ${progressStore.lastError() ?: "unreadable text"}"
                    }
                },
            )
        }

        SquareTextField(
            value = transferText,
            onValueChange = { text: String -> transferText = text },
            placeholder = "",
            textStyle = LanguageType.transferText,
            singleLine = false,
            modifier = Modifier.height(LanguageSpacing.transferTextHeight).testTag("transfer-text"),
        )

        val shownMessage: String? = message

        if (shownMessage != null) {
            LanguageText(text = shownMessage, style = LanguageType.captionText, color = LanguageColors.inkMuted)
        }

        LanguageText(text = "Reset", style = LanguageType.titleText, modifier = Modifier.padding(top = LanguageSpacing.spaceMd))

        SquareTextButton(
            text = resetButtonText(resetConfirmation),
            onClick = {
                when (resetConfirmation) {
                    ResetConfirmationKind.NotRequested -> resetConfirmation = ResetConfirmationKind.Requested
                    ResetConfirmationKind.Requested -> {
                        val outcome: ProgressChangeOutcome = progressStore.reset()

                        resetConfirmation = ResetConfirmationKind.NotRequested
                        message = when (outcome) {
                            ProgressChangeOutcome.Saved -> "All progress was reset."
                            ProgressChangeOutcome.Failed -> "Could not reset: ${progressStore.lastError() ?: "the progress file could not be written"}"
                        }
                    }
                }
            },
        )
    }
}

private fun resetButtonText(resetConfirmation: ResetConfirmationKind): String {
    return when (resetConfirmation) {
        ResetConfirmationKind.NotRequested -> "Reset all progress"
        ResetConfirmationKind.Requested -> "Tap again to erase all progress"
    }
}

private fun copyToClipboard(context: Context, text: String) {
    val clipboard: ClipboardManager = context.getSystemService(ClipboardManager::class.java)

    clipboard.setPrimaryClip(ClipData.newPlainText(EXPORT_CLIP_LABEL, text))
}
