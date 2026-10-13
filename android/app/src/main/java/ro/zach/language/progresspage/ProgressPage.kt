package ro.zach.language.progresspage

import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import ro.zach.language.core.CourseOutlineRecord
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.PageHeader
import ro.zach.language.design.readableWidth

@Composable
fun ProgressPage(outline: CourseOutlineRecord, modifier: Modifier = Modifier) {
    PageHeader(caption = null, title = "Progress", modifier = modifier.readableWidth().padding(LanguageSpacing.spaceMd))
}
