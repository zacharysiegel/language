package ro.zach.language.unit

import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import ro.zach.language.core.UnitLessonRecord
import ro.zach.language.core.UnitOutlineRecord
import ro.zach.language.design.PageHeader

@Composable
fun PracticeSection(courseId: String, unit: UnitOutlineRecord, lesson: UnitLessonRecord, modifier: Modifier = Modifier) {
    PageHeader(caption = null, title = "Practice", modifier = modifier)
}
