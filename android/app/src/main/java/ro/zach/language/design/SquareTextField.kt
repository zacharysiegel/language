package ro.zach.language.design

import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardCapitalization

/** A hairline-bordered text field, with `placeholder` shown in muted ink while it is empty. */
@Composable
fun SquareTextField(
    value: String,
    onValueChange: (String) -> Unit,
    placeholder: String,
    modifier: Modifier = Modifier,
    textStyle: TextStyle = LanguageType.bodyText,
    singleLine: Boolean = true,
    enabled: Boolean = true,
    imeAction: ImeAction = ImeAction.Default,
    keyboardActions: KeyboardActions = KeyboardActions.Default,
) {
    BasicTextField(
        value = value,
        onValueChange = onValueChange,
        modifier = modifier.fillMaxWidth(),
        enabled = enabled,
        textStyle = textStyle.copy(color = LanguageColors.ink),
        singleLine = singleLine,
        keyboardOptions = KeyboardOptions(
            capitalization = KeyboardCapitalization.None,
            autoCorrectEnabled = false,
            imeAction = imeAction,
        ),
        keyboardActions = keyboardActions,
        cursorBrush = SolidColor(LanguageColors.ink),
        decorationBox = { innerTextField: @Composable () -> Unit ->
            Box(
                modifier = Modifier
                    .border(LanguageSpacing.hairline, LanguageColors.ink)
                    .padding(LanguageSpacing.spaceSm),
            ) {
                if (value.isEmpty()) {
                    LanguageText(text = placeholder, style = textStyle, color = LanguageColors.inkMuted, maxLines = 1)
                }

                innerTextField()
            }
        },
    )
}
