package ro.zach.language.coursedata

import android.content.Context
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import kotlinx.coroutines.runBlocking
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.Assert.fail
import org.junit.Assume.assumeNoException
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import ro.zach.language.core.AnswerStatusRecord
import ro.zach.language.core.AudioEngineRecord
import ro.zach.language.core.CoreConfiguration
import ro.zach.language.core.CoreException
import ro.zach.language.core.CourseLibrary
import ro.zach.language.core.CourseOutlineRecord
import ro.zach.language.core.CoursePublication
import ro.zach.language.core.ExerciseRecord
import ro.zach.language.core.PracticeModeRecord
import ro.zach.language.core.PracticeOutcomeRecord
import ro.zach.language.core.ProgressBook
import ro.zach.language.core.PublishedCourseIndex
import ro.zach.language.core.UnitLessonRecord
import ro.zach.language.core.UnitStatusRecord
import ro.zach.language.core.VocabularyItemRecord
import ro.zach.language.core.VoiceChoiceRecord
import ro.zach.language.core.checkTypedAnswer
import ro.zach.language.core.exerciseSpokenText
import ro.zach.language.practice.speaksAfterAnswer
import ro.zach.language.testsupport.CourseData
import ro.zach.language.testsupport.UnreachableOrigins
import java.io.File
import java.util.UUID

/* The Mac's servers, reachable from the device through `adb reverse`. */
private const val LOCAL_ORIGIN: String = "http://127.0.0.1:3000"
private const val LOCAL_REPOSITORY_BASE_URL: String = "http://127.0.0.1:3002"

/** Exercises the Rust core through the generated bindings the app uses. */
@RunWith(AndroidJUnit4::class)
class CoreIntegrationTest {
    private lateinit var context: Context
    private lateinit var temporaryDirectory: File

    @Before
    fun setUp() {
        context = InstrumentationRegistry.getInstrumentation().targetContext
        temporaryDirectory = File(context.cacheDir, "core-test-${UUID.randomUUID()}")
        temporaryDirectory.mkdirs()
    }

    @After
    fun tearDown() {
        temporaryDirectory.deleteRecursively()
    }

    @Test
    fun embeddedIndexAndPublication_openAndAUnitLoads() {
        runBlocking {
            val installer: EmbeddedBundleInstaller = EmbeddedBundleInstaller(context)

            installer.install()

            val library: CourseLibrary = CourseLibrary(configuration(UnreachableOrigins.REFUSING_ORIGIN, UnreachableOrigins.REFUSING_ORIGIN, embeddedBundleDirectory = installer.installedDirectory()))
            val courseIndex: PublishedCourseIndex = library.loadEmbeddedCourseIndex()
            val courseId: String = courseIndex.courses().first().id
            val publication: CoursePublication = library.loadEmbeddedCoursePublication(courseId)
            val outline: CourseOutlineRecord = publication.loadCourse()
            val lesson: UnitLessonRecord = publication.loadUnit(outline.units[0].id)

            assertTrue(courseIndex.isEmbedded())
            assertTrue(publication.isEmbedded())
            assertEquals(courseId, publication.courseId())
            assertFalse(publication.publicationLabel().isEmpty())
            assertEquals("Portuguese", outline.targetLanguageName)
            assertFalse(outline.units.isEmpty())
            assertFalse(lesson.exercises.isEmpty())
        }
    }

    /* Needs `./scripts/dev/serve.sh` on the Mac and `adb reverse` of its ports; skipped when they are not reachable. */
    @Test
    fun latestPublicationsAndARecording_downloadFromTheLocalServer() {
        runBlocking {
            val library: CourseLibrary = CourseLibrary(configuration(LOCAL_ORIGIN, LOCAL_REPOSITORY_BASE_URL, embeddedBundleDirectory = temporaryDirectory))
            val courseIndex: PublishedCourseIndex = loadLatestCourseIndexOrSkip(library)
            val latestByCourseId: MutableMap<String, CoursePublication> = HashMap()

            for (course in courseIndex.courses()) {
                latestByCourseId[course.id] = library.loadLatestCoursePublication(courseIndex, course.id)
            }

            for ((courseId: String, latest: CoursePublication) in latestByCourseId) {
                val cached: CoursePublication? = library.loadCachedCoursePublication(courseId)

                assertEquals(latest.manifestSha256(), cached?.manifestSha256())
                assertEquals(latest.publicationLabel(), cached?.publicationLabel())
            }

            val latest: CoursePublication = latestByCourseId.getValue(CourseData.COURSE_ID)
            val outline: CourseOutlineRecord = latest.loadCourse()
            val item: VocabularyItemRecord = outline.units.first().vocabulary.first()
            val clipKey: String = checkNotNull(item.clipKey)
            val path: String? = library.recordingPath(clipKey, AudioEngineRecord.KOKORO, item.target, VoiceChoiceRecord.OwnText)
            val cachedIndex: PublishedCourseIndex? = library.loadCachedCourseIndex()

            assertFalse(courseIndex.isEmbedded())
            assertEquals(courseIndex.courses().size, cachedIndex?.courses()?.size)
            assertNotNull(path)
            assertTrue(File(path!!).isFile)
        }
    }

    @Test
    fun unreachableRepository_raisesAFailureNamingIt() {
        runBlocking {
            val library: CourseLibrary = CourseLibrary(configuration(UnreachableOrigins.REFUSING_ORIGIN, UnreachableOrigins.REFUSING_ORIGIN, embeddedBundleDirectory = temporaryDirectory))

            try {
                library.loadLatestCourseIndex()
                fail()
            } catch (failure: CoreException.Failure) {
                assertTrue(failure.failureMessage.contains(UnreachableOrigins.REFUSING_HOST))
                assertFalse(failure.backtrace.isBlank())
            }
        }
    }

    @Test
    fun progressBook_recordsAPassedPracticeAndPersists() {
        val path: String = File(temporaryDirectory, "progress.json").path
        val book: ProgressBook = ProgressBook.open(path)
        val outcome: PracticeOutcomeRecord = book.recordPracticeSession("c", "u", PracticeModeRecord.FULL, 9u, 10u, 3u)

        assertTrue(outcome.passed)
        assertEquals(3u, outcome.wordsAddedCount)
        assertEquals(UnitStatusRecord.COMPLETED, ProgressBook.open(path).unitProgress("c", "u").status)
        assertEquals(0u, ProgressBook.open(path).cardCount("other"))
    }

    @Test
    fun typedAnswers_ignoreCaseAndFlagAccents() {
        assertEquals(AnswerStatusRecord.CORRECT, checkTypedAnswer("eu MORO aqui", listOf("Eu moro aqui.")))
        assertEquals(AnswerStatusRecord.ACCENT_MISTAKE, checkTypedAnswer("voce", listOf("você")))
        assertEquals(AnswerStatusRecord.WRONG, checkTypedAnswer("tchau", listOf("oi")))
    }

    @Test
    fun fillBlankFeedback_speaksTheCompletedSentence() {
        val exercise: ExerciseRecord = ExerciseRecord.FillBlank(promptTarget = "Eu ___ aqui.", answers = listOf("moro"), hintSource = "I live here.", explanation = null, clipKey = null)

        assertEquals("Eu moro aqui.", exerciseSpokenText(exercise))
        assertTrue(exercise.speaksAfterAnswer())
    }

    private fun configuration(appOrigin: String, staticRepositoryBaseUrl: String, embeddedBundleDirectory: File): CoreConfiguration {
        return CoreConfiguration(
            appOrigin = appOrigin,
            staticRepositoryBaseUrl = staticRepositoryBaseUrl,
            cacheDirectory = temporaryDirectory.path,
            embeddedBundleDirectory = embeddedBundleDirectory.path,
        )
    }

    private suspend fun loadLatestCourseIndexOrSkip(library: CourseLibrary): PublishedCourseIndex {
        try {
            return library.loadLatestCourseIndex()
        } catch (failure: CoreException.Failure) {
            assumeNoException(failure)
            throw failure
        }
    }
}
