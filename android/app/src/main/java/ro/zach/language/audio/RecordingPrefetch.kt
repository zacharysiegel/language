package ro.zach.language.audio

import kotlinx.coroutines.CoroutineStart
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.NonCancellable
import kotlinx.coroutines.awaitCancellation
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import ro.zach.language.core.AudioEngineRecord
import ro.zach.language.core.CoursePublication
import ro.zach.language.core.PrefetchCancellation
import ro.zach.language.unit.UnitTabKind

/** Downloads the recordings a unit tab can play into the recording cache while the calling coroutine runs. */
object RecordingPrefetch {
    /**
     * Returns once every download has finished or the repository could not be reached, or, once the calling coroutine
     * is cancelled, once the downloads already started have.
     */
    suspend fun prefetchTab(tab: UnitTabKind, unitId: String, publication: CoursePublication, engine: AudioEngineRecord) {
        PrefetchCancellation().use { cancellation: PrefetchCancellation ->
            coroutineScope {
                /* Undispatched, so a caller cancelled before it starts still reaches the cancel. */
                val cancellationWatch: Job = launch(start = CoroutineStart.UNDISPATCHED) {
                    try {
                        awaitCancellation()
                    } finally {
                        cancellation.cancel()
                    }
                }

                /* Never cancelled itself: dropping the core's future would abandon a download midway. */
                withContext(NonCancellable + Dispatchers.IO) {
                    publication.prefetchUnitTabRecordings(unitId, tab.record(), engine, cancellation)
                }

                cancellationWatch.cancel()
            }
        }
    }
}
