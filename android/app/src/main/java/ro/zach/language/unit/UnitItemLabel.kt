package ro.zach.language.unit

import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import ro.zach.language.core.UnitOutlineRecord
import ro.zach.language.core.unitItemLabel
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType

/** The label of the unit's `itemNumber`th note, dialogue or reading, such as "A1.1.4". */
fun UnitOutlineRecord.itemLabel(itemNumber: Int): String {
    return unitItemLabel(levelCode, order, itemNumber.toUInt())
}

/** The `<level>.<unit>.<box>` label at the head of a box of a unit tab, a micro-label with tabular figures. */
@Composable
fun UnitItemLabel(text: String, modifier: Modifier = Modifier) {
    LanguageText(
        text = text.uppercase(),
        style = LanguageType.labelFigureText.copy(letterSpacing = LanguageType.labelTracking),
        color = LanguageColors.inkMuted,
        modifier = modifier.testTag("unit-item-label"),
    )
}
