package ro.zach.language.audio

import android.content.Context
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.cancel
import kotlinx.coroutines.delay
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withContext
import org.junit.After
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Assert.fail
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import ro.zach.language.core.AudioEngineRecord

private const val SPOKEN_TEXT: String = "Olá"
private const val POLL_INTERVAL_MILLISECONDS: Long = 20
private const val POLL_LIMIT: Int = 500

@RunWith(AndroidJUnit4::class)
class AudioPlayerPendingTest {
    private lateinit var playerScope: CoroutineScope
    private lateinit var audioPlayer: AudioPlayer

    @Before
    fun createPlayer() {
        val context: Context = InstrumentationRegistry.getInstrumentation().targetContext

        playerScope = CoroutineScope(SupervisorJob() + Dispatchers.Main.immediate)
        audioPlayer = onMainThread { AudioPlayer(library = null, output = AudioOutput(context), scope = playerScope) }
    }

    @After
    fun closePlayer() {
        onMainThread { audioPlayer.close() }
        playerScope.cancel()
    }

    @Test
    fun play_endsThePendingRequestOnceTheSystemVoiceStartsSpeaking() {
        onMainThread { audioPlayer.play(AudioClip(clipKey = null, text = SPOKEN_TEXT), AudioEngineRecord.AUTO, false, PlaybackControlKind.NormalSpeed) }
        assertTrue(onMainThread { audioPlayer.hasPendingRequest() })

        waitUntilNoPendingRequest()
        onMainThread { audioPlayer.stop() }
    }

    @Test
    fun stop_endsThePendingRequest() {
        onMainThread { audioPlayer.play(AudioClip(clipKey = null, text = SPOKEN_TEXT), AudioEngineRecord.AUTO, false, PlaybackControlKind.NormalSpeed) }
        assertTrue(onMainThread { audioPlayer.hasPendingRequest() })
        onMainThread { audioPlayer.stop() }

        assertFalse(onMainThread { audioPlayer.hasPendingRequest() })
    }

    private fun waitUntilNoPendingRequest() {
        runBlocking {
            repeat(POLL_LIMIT) {
                val hasPendingRequest: Boolean = withContext(Dispatchers.Main) { audioPlayer.hasPendingRequest() }

                if (!hasPendingRequest) {
                    return@runBlocking
                }

                delay(POLL_INTERVAL_MILLISECONDS)
            }

            fail()
        }
    }

    private fun <T> onMainThread(action: () -> T): T {
        return runBlocking(Dispatchers.Main) { action() }
    }
}
