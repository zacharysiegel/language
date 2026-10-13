package ro.zach.language.coursedata

import android.content.Context
import android.content.res.AssetManager
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.io.File
import java.io.IOException
import java.io.InputStream

private const val EMBEDDED_BUNDLE_ASSET_DIRECTORY: String = "embedded_bundle"
private const val INSTALLED_MARKER_FILE_NAME: String = "embedded_bundle.installed"
private const val INSTALLING_DIRECTORY_NAME: String = "embedded_bundle.installing"

/**
 * Copies the course content included in the app out of its assets into a directory of plain files, once per install
 * of the app, since the core reads it by path.
 */
class EmbeddedBundleInstaller(private val context: Context) {
    fun installedDirectory(): File {
        return File(context.noBackupFilesDir, EMBEDDED_BUNDLE_ASSET_DIRECTORY)
    }

    /** Returns at once when this install of the app already copied it. */
    suspend fun install() {
        withContext(Dispatchers.IO) {
            installUnlessInstalled()
        }
    }

    private fun installUnlessInstalled() {
        val stamp: String = installStamp()
        val marker: File = File(context.noBackupFilesDir, INSTALLED_MARKER_FILE_NAME)
        val destination: File = installedDirectory()
        val isInstalled: Boolean = marker.isFile && marker.readText() == stamp && destination.isDirectory

        if (isInstalled) {
            return
        }

        /* Copied beside the destination, then renamed over it, so an interrupted copy is redone on the next launch. */
        val installing: File = File(context.noBackupFilesDir, INSTALLING_DIRECTORY_NAME)

        installing.deleteRecursively()
        copyAssetTree(context.assets, EMBEDDED_BUNDLE_ASSET_DIRECTORY, installing)
        destination.deleteRecursively()

        val renamed: Boolean = installing.renameTo(destination)

        if (!renamed) {
            throw IOException("could not move the installed course content to $destination")
        }

        marker.writeText(stamp)
    }

    private fun installStamp(): String {
        return context.packageManager.getPackageInfo(context.packageName, 0).lastUpdateTime.toString()
    }
}

/* An asset path that lists no children is a file; the bundle has no empty directories. */
private fun copyAssetTree(assets: AssetManager, assetPath: String, destination: File) {
    val childNames: Array<String> = assets.list(assetPath) ?: emptyArray()

    if (childNames.isEmpty()) {
        copyAssetFile(assets, assetPath, destination)
        return
    }

    destination.mkdirs()

    for (childName in childNames) {
        copyAssetTree(assets, "$assetPath/$childName", File(destination, childName))
    }
}

private fun copyAssetFile(assets: AssetManager, assetPath: String, destination: File) {
    destination.parentFile?.mkdirs()

    assets.open(assetPath).use { input: InputStream ->
        destination.outputStream().use { output -> input.copyTo(output) }
    }
}
