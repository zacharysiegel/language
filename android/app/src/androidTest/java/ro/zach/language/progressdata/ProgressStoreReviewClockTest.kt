package ro.zach.language.progressdata

import android.content.Context
import android.os.Handler
import android.os.Looper
import androidx.compose.runtime.snapshots.ObserverHandle
import androidx.compose.runtime.snapshots.Snapshot
import androidx.compose.runtime.snapshotFlow
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.cancel
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withTimeout
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import ro.zach.language.core.CoreConfiguration
import ro.zach.language.coursedata.CoreEnvironment
import ro.zach.language.coursedata.EmbeddedBundleInstaller
import ro.zach.language.testsupport.CourseData
import ro.zach.language.testsupport.UnreachableOrigins
import java.io.File
import java.util.UUID

private const val CARD_DUE_DELAY_MILLISECONDS: Long = 1_500
private const val PAST_DUE_MILLISECONDS: Long = 1_000
private const val CHANGE_TIMEOUT_MILLISECONDS: Long = 10_000
private const val DUE_CARD_ID_LIMIT: Int = 10

@RunWith(AndroidJUnit4::class)
class ProgressStoreReviewClockTest {
    private lateinit var temporaryDirectory: File
    private lateinit var storeScope: CoroutineScope
    /* Compose applies global snapshot writes for its observers once a frame; with no frames here, the test does. */
    private lateinit var globalWriteObserver: ObserverHandle

    @Before
    fun setUp() {
        val context: Context = InstrumentationRegistry.getInstrumentation().targetContext
        val mainHandler: Handler = Handler(Looper.getMainLooper())

        temporaryDirectory = File(context.cacheDir, "review-clock-test-${UUID.randomUUID()}")
        temporaryDirectory.mkdirs()
        storeScope = CoroutineScope(SupervisorJob() + Dispatchers.Main.immediate)
        globalWriteObserver = Snapshot.registerGlobalWriteObserver { mainHandler.post { Snapshot.sendApplyNotifications() } }
    }

    @After
    fun tearDown() {
        globalWriteObserver.dispose()
        storeScope.cancel()
        temporaryDirectory.deleteRecursively()
    }

    @Test
    fun dueCardCount_changesWhenACardComesDue() {
        val nowMilliseconds: Long = System.currentTimeMillis()

        runBlocking(Dispatchers.Main) {
            val store: ProgressStore = storeWithCardsDueAt(listOf(nowMilliseconds - PAST_DUE_MILLISECONDS, nowMilliseconds + CARD_DUE_DELAY_MILLISECONDS))

            assertEquals(1, store.dueCardCount(CourseData.COURSE_ID))
            assertEquals(listOf("a:0"), store.dueCardIds(CourseData.COURSE_ID, DUE_CARD_ID_LIMIT))

            withTimeout(CHANGE_TIMEOUT_MILLISECONDS) {
                snapshotFlow { store.dueCardCount(CourseData.COURSE_ID) }.first { dueCount: Int -> dueCount == 2 }
            }

            assertEquals(listOf("a:0", "a:1"), store.dueCardIds(CourseData.COURSE_ID, DUE_CARD_ID_LIMIT))
            store.close()
        }
    }

    private fun storeWithCardsDueAt(dueMilliseconds: List<Long>): ProgressStore {
        val context: Context = InstrumentationRegistry.getInstrumentation().targetContext
        val progressFile: File = File(temporaryDirectory, "progress.json")
        val cards: JSONObject = JSONObject()

        dueMilliseconds.forEachIndexed { cardIndex: Int, due: Long ->
            val cardId: String = "a:$cardIndex"

            cards.put(
                cardId,
                JSONObject()
                    .put("id", cardId)
                    .put("box_index", 0)
                    .put("due_milliseconds", due)
                    .put("lapse_count", 0)
                    .put("review_count", 0),
            )
        }

        val progress: JSONObject = JSONObject().put("courses", JSONObject().put(CourseData.COURSE_ID, JSONObject().put("cards", cards)))

        progressFile.writeText(progress.toString())

        val environment: CoreEnvironment = CoreEnvironment(
            configuration = CoreConfiguration(
                appOrigin = UnreachableOrigins.REFUSING_ORIGIN,
                staticRepositoryBaseUrl = UnreachableOrigins.REFUSING_ORIGIN,
                cacheDirectory = temporaryDirectory.path,
                embeddedBundleDirectory = temporaryDirectory.path,
            ),
            progressPath = progressFile.path,
            embeddedBundleInstaller = EmbeddedBundleInstaller(context),
        )

        return ProgressStore(environment, storeScope)
    }
}
