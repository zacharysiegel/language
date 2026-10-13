package ro.zach.language.courses

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.Role
import ro.zach.language.core.CourseSummaryRecord
import ro.zach.language.coursedata.CourseStore
import ro.zach.language.coursedata.LocalCourseStore
import ro.zach.language.design.HairlineRule
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType
import ro.zach.language.design.PageHeader
import ro.zach.language.design.readableWidth
import ro.zach.language.progressdata.LocalProgressStore
import ro.zach.language.progressdata.ProgressStore

/** Every published course; choosing one makes it the course the other pages show. */
@Composable
fun CoursesPage(onChoose: () -> Unit, modifier: Modifier = Modifier) {
    val courseStore: CourseStore = LocalCourseStore.current
    val progressStore: ProgressStore = LocalProgressStore.current
    val courses: List<CourseSummaryRecord> = courseStore.courses()

    Column(modifier = modifier.fillMaxSize().verticalScroll(rememberScrollState())) {
        Column(modifier = Modifier.readableWidth().padding(LanguageSpacing.spaceMd)) {
            PageHeader(
                caption = if (courses.size == 1) "1 course" else "${courses.size} courses",
                title = "Courses",
                modifier = Modifier.padding(bottom = LanguageSpacing.spaceMd),
            )

            HairlineRule()

            for (course in courses) {
                CourseRow(
                    course = course,
                    onChoose = {
                        progressStore.rememberCourse(course.id)
                        onChoose()
                    },
                )

                HairlineRule()
            }
        }
    }
}

@Composable
private fun CourseRow(course: CourseSummaryRecord, onChoose: () -> Unit) {
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(interactionSource = null, indication = null, role = Role.Button, onClick = onChoose)
            .testTag("course-${course.id}")
            .padding(vertical = LanguageSpacing.spaceSm + LanguageSpacing.spaceXs),
        verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs),
    ) {
        LanguageText(text = course.title, style = LanguageType.bodyText, color = LanguageColors.accentLink)
        LanguageText(
            text = "${course.targetLanguageName} for ${course.sourceLanguageName} speakers",
            style = LanguageType.captionText,
            color = LanguageColors.inkMuted,
        )
    }
}
