package ro.zach.language.glossary

import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import ro.zach.language.core.CourseOutlineRecord
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.PageHeader
import ro.zach.language.design.readableWidth

@Composable
fun GlossaryPage(outline: CourseOutlineRecord, modifier: Modifier = Modifier) {
    PageHeader(caption = null, title = "Glossary", modifier = modifier.readableWidth().padding(LanguageSpacing.spaceMd))
}
