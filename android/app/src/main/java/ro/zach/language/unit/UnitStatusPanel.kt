package ro.zach.language.unit

import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import ro.zach.language.core.UnitProgressRecord
import ro.zach.language.core.UnitStatusRecord
import ro.zach.language.core.passPercent
import ro.zach.language.design.FloatingTipState
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType
import ro.zach.language.design.MicroLabel
import ro.zach.language.design.SquareSegmentedControl
import ro.zach.language.design.accessibilityName
import ro.zach.language.design.floatingTipAnchor
import ro.zach.language.design.label
import ro.zach.language.design.shadowedBox
import ro.zach.language.design.unitStatusRecords
import ro.zach.language.progressdata.LocalProgressStore
import ro.zach.language.progressdata.ProgressStore
import kotlin.math.roundToInt

const val UNIT_STATUS_HELP_AUTO_CLOSE_MILLISECONDS: Long = 8_000

/** The learner's own control over a unit's status. */
@Composable
fun UnitStatusPanel(
    courseId: String,
    unitId: String,
    isHelpShown: Boolean,
    onToggleHelp: () -> Unit,
    floatingTipState: FloatingTipState,
    modifier: Modifier = Modifier,
) {
    val progressStore: ProgressStore = LocalProgressStore.current
    val unitProgress: UnitProgressRecord = progressStore.unitProgress(courseId, unitId)
    val status: UnitStatusRecord = unitProgress.status
    val helpColor: Color = if (isHelpShown) LanguageColors.accentHover else LanguageColors.ink

    Column(modifier = modifier, verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
        Row(horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm), verticalAlignment = Alignment.CenterVertically) {
            MicroLabel(text = "Status")

            Box(
                modifier = Modifier
                    .floatingTipAnchor(floatingTipState)
                    .size(LanguageSpacing.buttonIconSize)
                    .border(LanguageSpacing.hairline, helpColor)
                    .clickable(interactionSource = null, indication = null, role = Role.Button, onClick = onToggleHelp)
                    .semantics { contentDescription = "How a unit's status changes" }
                    .testTag("unit-status-help"),
                contentAlignment = Alignment.Center,
            ) {
                LanguageText(text = "?", style = LanguageType.bodyText, color = helpColor)
            }

            if (unitProgress.attemptCount > 0u) {
                Spacer(modifier = Modifier.weight(1f))
                LanguageText(
                    text = "Quiz: ${(unitProgress.bestProportion * 100).roundToInt()}%",
                    style = LanguageType.dataText,
                    color = LanguageColors.inkMuted,
                    modifier = Modifier.testTag("unit-best-score"),
                )
            }
        }

        SquareSegmentedControl(
            options = unitStatusRecords,
            selection = status,
            label = { option: UnitStatusRecord -> option.label() },
            testTag = { option: UnitStatusRecord -> "unit-status-${option.accessibilityName()}" },
            onSelect = { option: UnitStatusRecord -> progressStore.setUnitStatus(courseId, unitId, option) },
        )
    }
}

/** The rules by which a unit's status changes without the learner setting it. */
@Composable
fun UnitStatusHelpTip(onClose: () -> Unit, modifier: Modifier = Modifier) {
    val rules: List<String> = listOf(
        "Starting the quiz or a practice also starts the unit.",
        "Passing the quiz with at least ${passPercent()}% completes it.",
    )

    Box(modifier = modifier.shadowedBox().testTag("unit-status-help-tip")) {
        Column(
            modifier = Modifier
                .padding(LanguageSpacing.spaceMd)
                .padding(end = LanguageSpacing.buttonIconSize),
            verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs),
        ) {
            rules.forEachIndexed { ruleIndex: Int, rule: String ->
                Row(horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs)) {
                    LanguageText(text = "${ruleIndex + 1}.", style = LanguageType.captionFigureText)
                    LanguageText(text = rule, style = LanguageType.captionText)
                }
            }
        }

        Box(
            modifier = Modifier
                .align(Alignment.TopEnd)
                .size(LanguageSpacing.buttonIconSize)
                .clickable(interactionSource = null, indication = null, role = Role.Button, onClick = onClose)
                .semantics { contentDescription = "Close" }
                .testTag("unit-status-help-close"),
            contentAlignment = Alignment.Center,
        ) {
            LanguageText(text = "×", style = LanguageType.titleText)
        }
    }
}
