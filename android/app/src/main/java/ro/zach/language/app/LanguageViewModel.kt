package ro.zach.language.app

import android.content.Context
import android.os.Bundle
import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.launch
import ro.zach.language.audio.AudioOutput
import ro.zach.language.audio.AudioPlayer
import ro.zach.language.coursedata.CoreEnvironment
import ro.zach.language.coursedata.CourseStore
import ro.zach.language.progressdata.ProgressStore

private const val ROOT_NAVIGATION_STATE_KEY: String = "root-navigation"

/** The app's stores, audio player and navigation, kept across configuration changes and released when the activity finishes. */
class LanguageViewModel(environment: CoreEnvironment, applicationContext: Context, savedStateHandle: SavedStateHandle) : ViewModel() {
    val courseStore: CourseStore = CourseStore(environment, viewModelScope)
    val progressStore: ProgressStore = ProgressStore(environment, viewModelScope)
    val audioPlayer: AudioPlayer = AudioPlayer(courseStore.courseLibrary(), AudioOutput(applicationContext), viewModelScope)
    /* Saved with the activity's state, as the unit pages' state keyed by its entry ids is. */
    val rootNavigation: RootNavigationState = RootNavigationState.restoring(savedStateHandle.get<Bundle>(ROOT_NAVIGATION_STATE_KEY))

    init {
        savedStateHandle.setSavedStateProvider(ROOT_NAVIGATION_STATE_KEY) { rootNavigation.savedState() }
        viewModelScope.launch { courseStore.start() }
    }

    /** The app came to the foreground. */
    fun onResume() {
        progressStore.advanceReviewClock()

        val activeCourseId: String? = courseStore.activeCourseId(progressStore.lastCourseId())

        viewModelScope.launch { courseStore.refresh(activeCourseId) }
    }

    override fun onCleared() {
        audioPlayer.close()
        courseStore.close()
        progressStore.close()
    }
}
