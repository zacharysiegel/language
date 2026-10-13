package ro.zach.language.diagnostics

import android.util.Log
import ro.zach.language.core.CoreException

/** Sends failures to logcat, with the Rust call stack when the core raised them. */
object CoreErrorReport {
    /** Logs `error` with what was being done and returns the text to show the learner. */
    fun record(error: Throwable, activity: String): String {
        return when (error) {
            is CoreException.Failure -> {
                Log.e(LogTag.CORE, "$activity failed: ${error.failureMessage}\nRust backtrace:\n${error.backtrace}")
                error.failureMessage
            }

            else -> {
                val message: String = error.message ?: error.toString()

                Log.e(LogTag.CORE, "$activity failed: $message", error)
                message
            }
        }
    }
}
