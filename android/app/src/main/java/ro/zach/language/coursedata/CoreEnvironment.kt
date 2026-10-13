package ro.zach.language.coursedata

import android.content.Context
import android.content.Intent
import ro.zach.language.BuildConfig
import ro.zach.language.core.CoreConfiguration
import java.io.File

/** Where the app keeps its cache and progress, and where it finds its course. */
class CoreEnvironment(
    val configuration: CoreConfiguration,
    val progressPath: String,
    val embeddedBundleInstaller: EmbeddedBundleInstaller,
) {
    companion object {
        const val ORIGIN_EXTRA: String = "ro.zach.language.extra.ORIGIN"
        const val REPOSITORY_BASE_URL_EXTRA: String = "ro.zach.language.extra.REPOSITORY_BASE_URL"
        const val PROGRESS_FILE_EXTRA: String = "ro.zach.language.extra.PROGRESS_FILE"
        const val CACHE_DIRECTORY_EXTRA: String = "ro.zach.language.extra.CACHE_DIRECTORY"
        const val CACHE_DIRECTORY_NAME: String = "artifacts"
        const val PROGRESS_FILE_NAME: String = "language-progress.json"

        /**
         * The app's own storage and assets. In a debuggable build the extras `ORIGIN_EXTRA`, `REPOSITORY_BASE_URL_EXTRA`,
         * `PROGRESS_FILE_EXTRA` and `CACHE_DIRECTORY_EXTRA` override the origin, the static repository, the progress
         * file's name and the cache directory's name.
         */
        fun standard(context: Context, intent: Intent): CoreEnvironment {
            val launchOverrides: LaunchOverrides = LaunchOverrides.from(intent)
            /* The cache directory stays out of backups: it is reproducible from the repository. */
            val cacheDirectory: File = File(context.cacheDir, launchOverrides.cacheDirectoryName ?: CACHE_DIRECTORY_NAME)
            val embeddedBundleInstaller: EmbeddedBundleInstaller = EmbeddedBundleInstaller(context)

            cacheDirectory.mkdirs()
            context.filesDir.mkdirs()

            return CoreEnvironment(
                configuration = CoreConfiguration(
                    appOrigin = launchOverrides.origin ?: BuildConfig.LANGUAGE_ORIGIN,
                    staticRepositoryBaseUrl = launchOverrides.repositoryBaseUrl ?: BuildConfig.LANGUAGE_REPOSITORY_BASE_URL,
                    cacheDirectory = cacheDirectory.path,
                    embeddedBundleDirectory = embeddedBundleInstaller.installedDirectory().path,
                ),
                progressPath = File(context.filesDir, launchOverrides.progressFileName ?: PROGRESS_FILE_NAME).path,
                embeddedBundleInstaller = embeddedBundleInstaller,
            )
        }
    }
}

/** What a launch's Intent overrides of the environment; nothing outside a debuggable build. */
private class LaunchOverrides(
    val origin: String?,
    val repositoryBaseUrl: String?,
    val progressFileName: String?,
    val cacheDirectoryName: String?,
) {
    companion object {
        fun from(intent: Intent): LaunchOverrides {
            if (!BuildConfig.DEBUG) {
                return LaunchOverrides(origin = null, repositoryBaseUrl = null, progressFileName = null, cacheDirectoryName = null)
            }

            return LaunchOverrides(
                origin = intent.getStringExtra(CoreEnvironment.ORIGIN_EXTRA),
                repositoryBaseUrl = intent.getStringExtra(CoreEnvironment.REPOSITORY_BASE_URL_EXTRA),
                progressFileName = intent.getStringExtra(CoreEnvironment.PROGRESS_FILE_EXTRA),
                cacheDirectoryName = intent.getStringExtra(CoreEnvironment.CACHE_DIRECTORY_EXTRA),
            )
        }
    }
}
