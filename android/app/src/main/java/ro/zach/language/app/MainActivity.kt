package ro.zach.language.app

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.SystemBarStyle
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.activity.viewModels
import androidx.compose.ui.graphics.toArgb
import androidx.lifecycle.DefaultLifecycleObserver
import androidx.lifecycle.LifecycleOwner
import androidx.lifecycle.createSavedStateHandle
import androidx.lifecycle.viewmodel.initializer
import androidx.lifecycle.viewmodel.viewModelFactory
import ro.zach.language.core.initializeCoreLogging
import ro.zach.language.coursedata.CoreEnvironment
import ro.zach.language.design.LanguageColors

class MainActivity : ComponentActivity() {
    private val languageViewModel: LanguageViewModel by viewModels {
        viewModelFactory {
            initializer { LanguageViewModel(CoreEnvironment.standard(applicationContext, intent), applicationContext, createSavedStateHandle()) }
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        /* Before the view model opens the library, so its first log lines reach logcat. */
        initializeCoreLogging()
        super.onCreate(savedInstanceState)
        enableEdgeToEdge(
            statusBarStyle = SystemBarStyle.light(LanguageColors.paper.toArgb(), LanguageColors.ink.toArgb()),
            navigationBarStyle = SystemBarStyle.light(LanguageColors.paper.toArgb(), LanguageColors.ink.toArgb()),
        )

        val viewModel: LanguageViewModel = languageViewModel

        lifecycle.addObserver(
            object : DefaultLifecycleObserver {
                override fun onResume(owner: LifecycleOwner) {
                    viewModel.onResume()
                }
            },
        )

        setContent {
            RootScreen(
                courseStore = viewModel.courseStore,
                progressStore = viewModel.progressStore,
                audioPlayer = viewModel.audioPlayer,
                rootNavigation = viewModel.rootNavigation,
            )
        }
    }
}
