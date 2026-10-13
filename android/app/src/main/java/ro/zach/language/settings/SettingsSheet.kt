package ro.zach.language.settings

import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.PageHeader
import ro.zach.language.design.readableWidth

/** The settings page's content; `courseId` is the course open behind it. */
@Composable
fun SettingsSheet(courseId: String?, modifier: Modifier = Modifier) {
    PageHeader(caption = null, title = "Settings", modifier = modifier.readableWidth().padding(LanguageSpacing.spaceMd))
}
