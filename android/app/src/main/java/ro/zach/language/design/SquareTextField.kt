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
import androidx.compose.ui.text.intl.LocaleList

/**
 * A hairline-bordered text field, with `placeholder` shown in muted ink while it is empty.
 * `keyboardLanguage` is the BCP 47 tag of the language the keyboard is asked to offer; `null` leaves the choice to the keyboard.
 */
@Composable
fun SquareTextField(
    value: String,
    onValueChange: (String) -> Unit,
    placeholder: String,
    modifier: Modifier = Modifier,
    keyboardLanguage: String? = null,
    textStyle: TextStyle = LanguageType.bodyText,
    singleLine: Boolean = true,
    minLines: Int = 1,
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
        minLines = minLines,
        keyboardOptions = KeyboardOptions(
            capitalization = KeyboardCapitalization.None,
            autoCorrectEnabled = false,
            imeAction = imeAction,
            hintLocales = keyboardLanguage?.let { language: String -> LocaleList(language) },
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
