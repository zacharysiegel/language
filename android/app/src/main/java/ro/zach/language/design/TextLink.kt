package ro.zach.language.design

import androidx.compose.foundation.clickable
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.Role

/** A caption-sized link that leaves or steps through what the page shows. */
@Composable
fun TextLink(text: String, onClick: () -> Unit, modifier: Modifier = Modifier) {
    LanguageText(
        text = text,
        style = LanguageType.captionText,
        color = LanguageColors.accentLink,
        modifier = modifier.clickable(interactionSource = null, indication = null, role = Role.Button, onClick = onClick),
    )
}
