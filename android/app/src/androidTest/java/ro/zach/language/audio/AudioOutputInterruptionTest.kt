package ro.zach.language.audio

import android.content.Context
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import kotlinx.coroutines.Deferred
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import kotlinx.coroutines.delay
import kotlinx.coroutines.runBlocking
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.fail
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import java.io.DataOutputStream
import java.io.File
import java.io.FileOutputStream
import java.util.UUID

private const val SILENT_RECORDING_SECONDS: Int = 5
private const val SAMPLE_RATE: Int = 44_100
private const val BYTES_PER_SAMPLE: Int = 2
private const val BITS_PER_SAMPLE: Int = 16
private const val CHANNEL_COUNT: Int = 1
private const val PCM_FORMAT_TAG: Int = 1
private const val FORMAT_CHUNK_SIZE: Int = 16
/* The RIFF chunk's size counts everything after its own header except the data: "WAVE", the format chunk and the data chunk's header. */
private const val RIFF_HEADER_BYTES_AFTER_SIZE: Int = 36
private const val ACTIVITY_POLL_MILLISECONDS: Long = 10
private const val ACTIVITY_POLL_LIMIT: Int = 500

@RunWith(AndroidJUnit4::class)
class AudioOutputInterruptionTest {
    private lateinit var temporaryDirectory: File
    private lateinit var output: AudioOutput

    @Before
    fun createOutput() {
        val context: Context = InstrumentationRegistry.getInstrumentation().targetContext

        temporaryDirectory = File(context.cacheDir, "audio-test-${UUID.randomUUID()}")
        temporaryDirectory.mkdirs()
        output = AudioOutput(context)
    }

    @After
    fun closeOutput() {
        output.close()
        temporaryDirectory.deleteRecursively()
    }

    @Test
    fun playRecording_returnsInterruptedWhenAnInterruptionEndsWithoutResumption() {
        val recording: File = writeSilentRecording()

        runBlocking {
            val playback: Deferred<PlaybackOutcome> = async(Dispatchers.Default) {
                output.playRecording(recording.path, AudioPlayer.NORMAL_RATE, onSoundStarted = {})
            }

            waitUntilPlaying()
            output.observeInterruption(AudioInterruptionEvent.Began)
            output.observeInterruption(AudioInterruptionEvent.EndedShouldNotResume)

            assertEquals(PlaybackOutcome.Interrupted, playback.await())
        }
    }

    private suspend fun waitUntilPlaying() {
        repeat(ACTIVITY_POLL_LIMIT) {
            val activity: AudioOutputActivityKind = output.activity()

            if (activity == AudioOutputActivityKind.Playing) {
                return
            }

            delay(ACTIVITY_POLL_MILLISECONDS)
        }

        fail()
    }

    /* A 16-bit mono PCM WAV file of silence. */
    private fun writeSilentRecording(): File {
        val recording: File = File(temporaryDirectory, "silence.wav")
        val dataSize: Int = SILENT_RECORDING_SECONDS * SAMPLE_RATE * BYTES_PER_SAMPLE * CHANNEL_COUNT

        DataOutputStream(FileOutputStream(recording)).use { stream: DataOutputStream ->
            stream.writeBytes("RIFF")
            writeLittleEndianInt(stream, RIFF_HEADER_BYTES_AFTER_SIZE + dataSize)
            stream.writeBytes("WAVE")
            stream.writeBytes("fmt ")
            writeLittleEndianInt(stream, FORMAT_CHUNK_SIZE)
            writeLittleEndianShort(stream, PCM_FORMAT_TAG)
            writeLittleEndianShort(stream, CHANNEL_COUNT)
            writeLittleEndianInt(stream, SAMPLE_RATE)
            writeLittleEndianInt(stream, SAMPLE_RATE * BYTES_PER_SAMPLE * CHANNEL_COUNT)
            writeLittleEndianShort(stream, BYTES_PER_SAMPLE * CHANNEL_COUNT)
            writeLittleEndianShort(stream, BITS_PER_SAMPLE)
            stream.writeBytes("data")
            writeLittleEndianInt(stream, dataSize)
            stream.write(ByteArray(dataSize))
        }

        return recording
    }

    private fun writeLittleEndianInt(stream: DataOutputStream, value: Int) {
        stream.writeInt(Integer.reverseBytes(value))
    }

    private fun writeLittleEndianShort(stream: DataOutputStream, value: Int) {
        stream.writeShort(java.lang.Short.reverseBytes(value.toShort()).toInt())
    }
}
