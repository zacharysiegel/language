package ro.zach.language.design

import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.IntrinsicSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.layout.widthIn
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.rotate
import androidx.compose.ui.layout.onSizeChanged
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.selected
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.stateDescription
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.IntOffset
import androidx.compose.ui.window.Popup
import androidx.compose.ui.window.PopupProperties

private const val DOWNWARD_CHEVRON_ROTATION_DEGREES: Float = 90f

/** A hairline-bordered control showing the chosen option, which opens a square menu of every option. */
@Composable
fun <Option> SquareDropDown(
    options: List<Option>,
    selection: Option,
    label: (Option) -> String,
    testTag: (Option) -> String,
    onSelect: (Option) -> Unit,
    modifier: Modifier = Modifier,
) {
    var isExpanded: Boolean by remember { mutableStateOf(false) }
    var anchorHeight: Int by remember { mutableIntStateOf(0) }
    var anchorWidth: Int by remember { mutableIntStateOf(0) }
    val anchorWidthDp: Dp = with(LocalDensity.current) { anchorWidth.toDp() }

    Box(modifier = modifier) {
        Row(
            modifier = Modifier
                .onSizeChanged { size ->
                    anchorHeight = size.height
                    anchorWidth = size.width
                }
                .border(LanguageSpacing.hairline, LanguageColors.ink)
                .clickable(interactionSource = null, indication = null, role = Role.DropdownList) { isExpanded = true }
                .semantics { stateDescription = label(selection) }
                .padding(horizontal = LanguageSpacing.spaceSm + LanguageSpacing.spaceXs, vertical = LanguageSpacing.spaceSm),
            horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            LanguageText(text = label(selection), style = LanguageType.bodyText)
            Glyph(glyph = GlyphKind.Chevron, color = LanguageColors.ink, size = LanguageSpacing.spaceSm + LanguageSpacing.spaceXs, modifier = Modifier.rotate(DOWNWARD_CHEVRON_ROTATION_DEGREES))
        }

        if (isExpanded) {
            Popup(
                offset = IntOffset(0, anchorHeight),
                onDismissRequest = { isExpanded = false },
                properties = PopupProperties(focusable = true),
            ) {
                SquareDropDownMenu(
                    options = options,
                    selection = selection,
                    label = label,
                    testTag = testTag,
                    minimumWidth = anchorWidthDp,
                    onSelect = { option: Option ->
                        isExpanded = false
                        onSelect(option)
                    },
                )
            }
        }
    }
}

@Composable
private fun <Option> SquareDropDownMenu(
    options: List<Option>,
    selection: Option,
    label: (Option) -> String,
    testTag: (Option) -> String,
    minimumWidth: Dp,
    onSelect: (Option) -> Unit,
) {
    Column(
        modifier = Modifier
            .widthIn(min = minimumWidth)
            .width(IntrinsicSize.Max)
            .shadowedBox(LanguageSpacing.smallBoxShadowOffset),
    ) {
        for (option in options) {
            val isSelected: Boolean = option == selection

            Row(
                modifier = Modifier
                    .clickable(interactionSource = null, indication = null, role = Role.Button) { onSelect(option) }
                    .semantics { selected = isSelected }
                    .testTag(testTag(option))
                    .padding(horizontal = LanguageSpacing.spaceSm + LanguageSpacing.spaceXs, vertical = LanguageSpacing.spaceSm),
                horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Box(modifier = Modifier.size(LanguageSpacing.checkBoxSize), contentAlignment = Alignment.Center) {
                    if (isSelected) {
                        Glyph(glyph = GlyphKind.Check, color = LanguageColors.ink, size = LanguageSpacing.checkBoxSize - LanguageSpacing.spaceXs)
                    }
                }

                LanguageText(text = label(option), style = LanguageType.bodyText)
            }
        }
    }
}
