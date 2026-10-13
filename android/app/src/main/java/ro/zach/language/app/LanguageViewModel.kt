package ro.zach.language.app

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.launch
import ro.zach.language.coursedata.CoreEnvironment
import ro.zach.language.coursedata.CourseStore
import ro.zach.language.progressdata.ProgressStore

/** The app's stores and navigation, kept across configuration changes and released when the activity finishes. */
class LanguageViewModel(environment: CoreEnvironment) : ViewModel() {
    val courseStore: CourseStore = CourseStore(environment, viewModelScope)
    val progressStore: ProgressStore = ProgressStore(environment, viewModelScope)
    val rootNavigation: RootNavigationState = RootNavigationState()

    init {
        viewModelScope.launch { courseStore.start() }
    }

    /** The app came to the foreground. */
    fun onResume() {
        progressStore.advanceReviewClock()

        val activeCourseId: String? = courseStore.activeCourseId(progressStore.lastCourseId())

        viewModelScope.launch { courseStore.refresh(activeCourseId) }
    }

    override fun onCleared() {
        courseStore.close()
        progressStore.close()
    }
}
