package ro.zach.language.coursedata

import androidx.compose.runtime.ProvidableCompositionLocal
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateMapOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.compose.runtime.snapshots.SnapshotStateMap
import androidx.compose.runtime.staticCompositionLocalOf
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.CompletableDeferred
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.CoroutineStart
import kotlinx.coroutines.Deferred
import kotlinx.coroutines.async
import kotlinx.coroutines.launch
import ro.zach.language.core.CourseLibrary
import ro.zach.language.core.CourseOutlineRecord
import ro.zach.language.core.CoursePublication
import ro.zach.language.core.CourseSummaryRecord
import ro.zach.language.core.PublishedCourseIndex
import ro.zach.language.core.UnitLessonRecord
import ro.zach.language.core.UnitOutlineRecord
import ro.zach.language.diagnostics.CoreErrorReport

private const val SERVER_UNREACHABLE_MESSAGE: String = "The course server could not be reached."

val LocalCourseStore: ProvidableCompositionLocal<CourseStore> = staticCompositionLocalOf { error("no CourseStore provided") }

sealed interface CourseIndexLoadState {
    data object Loading : CourseIndexLoadState

    data class Loaded(val courseIndex: PublishedCourseIndex) : CourseIndexLoadState

    data class Failed(val message: String) : CourseIndexLoadState
}

/** Whether this launch's first read of the repository's course index has finished. */
enum class RepositoryIndexLoadKind {
    Pending,
    Loaded,
    Failed,
}

sealed interface CoursePublicationLoadState {
    data object Loading : CoursePublicationLoadState

    data class Loaded(val publication: CoursePublication) : CoursePublicationLoadState

    data class Failed(val message: String) : CoursePublicationLoadState
}

sealed interface OutlineLoadState {
    data object Loading : OutlineLoadState

    data class Loaded(val outline: CourseOutlineRecord) : OutlineLoadState

    data class Failed(val message: String) : OutlineLoadState
}

sealed interface UnitLoadState {
    data object Loading : UnitLoadState

    data class Loaded(val unit: UnitOutlineRecord, val lesson: UnitLessonRecord) : UnitLoadState

    data class Failed(val message: String) : UnitLoadState
}

private enum class StartKind {
    NotStarted,
    Starting,
    Started,
}

private data class UnitKey(val courseId: String, val unitId: String)

/** One run of an operation at a time; a caller arriving while it runs awaits that run. */
private class JoinedRun(private val scope: CoroutineScope) {
    private var inFlight: Deferred<Unit>? = null

    /* The run belongs to `scope`, so a caller's cancellation ends only its own wait. */
    suspend fun run(operation: suspend () -> Unit) {
        val existing: Deferred<Unit>? = inFlight

        if (existing != null) {
            existing.await()
            return
        }

        val started: Deferred<Unit> = scope.async(start = CoroutineStart.LAZY) {
            try {
                operation()
            } finally {
                inFlight = null
            }
        }

        inFlight = started
        started.start()
        started.await()
    }
}

/**
 * The course index on screen, the publication of each course opened, and the outlines and lessons opened from them.
 * Every state is snapshot state, so composables that read it recompose when it changes. Call from the main thread.
 */
class CourseStore(private val environment: CoreEnvironment, private val scope: CoroutineScope) {
    private val library: CourseLibrary?
    private var indexState: CourseIndexLoadState by mutableStateOf(CourseIndexLoadState.Loading)
    private var repositoryIndexState: RepositoryIndexLoadKind by mutableStateOf(RepositoryIndexLoadKind.Pending)
    private val publicationStates: SnapshotStateMap<String, CoursePublicationLoadState> = mutableStateMapOf()
    private val outlineStates: SnapshotStateMap<String, OutlineLoadState> = mutableStateMapOf()
    private val unitStates: SnapshotStateMap<UnitKey, UnitLoadState> = mutableStateMapOf()
    private val updateMessages: SnapshotStateMap<String, String> = mutableStateMapOf()

    private var startKind: StartKind = StartKind.NotStarted
    private var embeddedIndexFailure: String? = null
    private val requestedPublicationCourseIds: MutableSet<String> = HashSet()
    private val indexRefresh: JoinedRun = JoinedRun(scope)
    private val publicationRefreshes: MutableMap<String, JoinedRun> = HashMap()
    private val firstRepositoryIndexRead: CompletableDeferred<Unit> = CompletableDeferred()

    init {
        library = openLibrary()
    }

    private fun openLibrary(): CourseLibrary? {
        try {
            return CourseLibrary(environment.configuration)
        } catch (error: Exception) {
            indexState = CourseIndexLoadState.Failed(CoreErrorReport.record(error, "opening the course library"))
            return null
        }
    }

    fun courseLibrary(): CourseLibrary? {
        return library
    }

    fun indexLoadState(): CourseIndexLoadState {
        return indexState
    }

    fun repositoryIndexLoadState(): RepositoryIndexLoadKind {
        return repositoryIndexState
    }

    fun shownIndex(): PublishedCourseIndex? {
        return (indexState as? CourseIndexLoadState.Loaded)?.courseIndex
    }

    fun courses(): List<CourseSummaryRecord> {
        return shownIndex()?.courses() ?: emptyList()
    }

    fun listsCourse(courseId: String): Boolean {
        return courses().any { course -> course.id == courseId }
    }

    /** The course the learner opened last when the index lists it, else the only course the index lists. */
    fun activeCourseId(lastCourseId: String?): String? {
        val courses: List<CourseSummaryRecord> = courses()

        if (lastCourseId != null && courses.any { course -> course.id == lastCourseId }) {
            return lastCourseId
        }

        return if (courses.size == 1) courses[0].id else null
    }

    fun publicationState(courseId: String): CoursePublicationLoadState? {
        return publicationStates[courseId]
    }

    fun shownPublication(courseId: String): CoursePublication? {
        return (publicationStates[courseId] as? CoursePublicationLoadState.Loaded)?.publication
    }

    /** Changes whenever the course's loaded outline and units are discarded for a newly shown publication. */
    fun shownContentKey(courseId: String): String? {
        val publication: CoursePublication = shownPublication(courseId)
            ?: return null

        return "${publication.manifestSha256()} ${publication.recordingsBaseUrl()}"
    }

    fun outlineState(courseId: String): OutlineLoadState? {
        return outlineStates[courseId]
    }

    fun outline(courseId: String): CourseOutlineRecord? {
        return (outlineStates[courseId] as? OutlineLoadState.Loaded)?.outline
    }

    fun unitState(courseId: String, unitId: String): UnitLoadState? {
        return unitStates[UnitKey(courseId, unitId)]
    }

    fun updateMessage(courseId: String): String? {
        return updateMessages[courseId]
    }

    /** Shows the cached course index at once, else the one included in the app, then reads the repository's. */
    suspend fun start() {
        val library: CourseLibrary = library
            ?: return

        if (startKind != StartKind.NotStarted) {
            return
        }

        startKind = StartKind.Starting
        installEmbeddedBundle()
        showCachedOrEmbeddedIndex(library)
        startKind = StartKind.Started
        refresh(courseId = null)
    }

    /**
     * Reads the repository's course index and, for `courseId`, the course's latest pointer. A read of either already
     * under way is joined rather than repeated.
     */
    suspend fun refresh(courseId: String?) {
        val library: CourseLibrary = library
            ?: return

        if (startKind != StartKind.Started) {
            return
        }

        indexRefresh.run { readRepositoryIndex(library) }

        if (courseId != null) {
            refreshPublication(library, courseId)
        }
    }

    /** Retries a course whose publication or outline failed to load. */
    suspend fun retry(courseId: String) {
        /* Runs in the store's scope, so the outline is requested even when the caller leaves first. */
        val retrying: Deferred<Unit> = scope.async {
            if (outlineStates[courseId] is OutlineLoadState.Failed) {
                outlineStates.remove(courseId)
            }

            refresh(courseId)
            ensureOutlineLoaded(courseId)
        }

        retrying.await()
    }

    /**
     * Once per launch: shows the cached publication at once, else the one included in the app, then the publication the
     * course's latest pointer names once the repository's course index has been read.
     */
    fun ensurePublicationLoaded(courseId: String) {
        val library: CourseLibrary = library
            ?: return

        if (courseId in requestedPublicationCourseIds) {
            return
        }

        requestedPublicationCourseIds.add(courseId)

        if (publicationStates[courseId] == null) {
            publicationStates[courseId] = CoursePublicationLoadState.Loading
        }

        scope.launch {
            if (shownPublication(courseId) == null) {
                showCachedOrEmbeddedPublication(library, courseId)
            }

            awaitFirstRepositoryIndexRead()
            refreshPublication(library, courseId)
        }
    }

    fun ensureOutlineLoaded(courseId: String) {
        val publication: CoursePublication = shownPublication(courseId)
            ?: return

        if (outlineStates[courseId] != null) {
            return
        }

        outlineStates[courseId] = OutlineLoadState.Loading

        scope.launch {
            val loaded: OutlineLoadState = loadOutline(publication, courseId)

            if (isStillShown(publication)) {
                outlineStates[courseId] = loaded
            }
        }
    }

    fun ensureUnitLoaded(courseId: String, unitId: String) {
        val key: UnitKey = UnitKey(courseId, unitId)
        val publication: CoursePublication = shownPublication(courseId)
            ?: return

        if (unitStates[key] != null) {
            return
        }

        val unitOutline: UnitOutlineRecord = outline(courseId)?.units?.firstOrNull { unit -> unit.id == unitId }
            ?: return

        unitStates[key] = UnitLoadState.Loading

        scope.launch {
            val loaded: UnitLoadState = loadUnit(publication, unitOutline, courseId)

            if (isStillShown(publication)) {
                unitStates[key] = loaded
            }
        }
    }

    private suspend fun loadOutline(publication: CoursePublication, courseId: String): OutlineLoadState {
        try {
            return OutlineLoadState.Loaded(publication.loadCourse())
        } catch (cancellation: CancellationException) {
            throw cancellation
        } catch (error: Exception) {
            return OutlineLoadState.Failed(reportShardFailure(error, publication, "loading course $courseId"))
        }
    }

    private suspend fun loadUnit(publication: CoursePublication, unitOutline: UnitOutlineRecord, courseId: String): UnitLoadState {
        try {
            return UnitLoadState.Loaded(unitOutline, publication.loadUnit(unitOutline.id))
        } catch (cancellation: CancellationException) {
            throw cancellation
        } catch (error: Exception) {
            return UnitLoadState.Failed(reportShardFailure(error, publication, "loading unit ${unitOutline.id} of $courseId"))
        }
    }

    /* A failed install leaves the embedded index to fail with the core's own message. */
    private suspend fun installEmbeddedBundle() {
        try {
            environment.embeddedBundleInstaller.install()
        } catch (cancellation: CancellationException) {
            throw cancellation
        } catch (error: Exception) {
            CoreErrorReport.record(error, "installing the course content included in the app")
        }
    }

    private suspend fun showCachedOrEmbeddedIndex(library: CourseLibrary) {
        val cached: PublishedCourseIndex? = library.loadCachedCourseIndex()

        if (cached != null) {
            indexState = CourseIndexLoadState.Loaded(cached)
            return
        }

        try {
            indexState = CourseIndexLoadState.Loaded(library.loadEmbeddedCourseIndex())
        } catch (cancellation: CancellationException) {
            throw cancellation
        } catch (error: Exception) {
            embeddedIndexFailure = CoreErrorReport.record(error, "opening the course index included in the app")
        }
    }

    private suspend fun readRepositoryIndex(library: CourseLibrary) {
        try {
            showRepositoryIndex(library.loadLatestCourseIndex())
            repositoryIndexState = RepositoryIndexLoadKind.Loaded
        } catch (cancellation: CancellationException) {
            throw cancellation
        } catch (error: Exception) {
            recordRepositoryIndexFailure(CoreErrorReport.record(error, "fetching the course index"))
        }

        firstRepositoryIndexRead.complete(Unit)
    }

    private fun recordRepositoryIndexFailure(message: String) {
        if (repositoryIndexState == RepositoryIndexLoadKind.Pending) {
            repositoryIndexState = RepositoryIndexLoadKind.Failed
        }

        if (shownIndex() == null) {
            val embeddedFailure: String? = embeddedIndexFailure

            indexState = CourseIndexLoadState.Failed(
                if (embeddedFailure == null) message else "$message (the copy included in the app: $embeddedFailure)",
            )
        }
    }

    /* The repository's index is authoritative for the course list, so a course it omits is forgotten. */
    private fun showRepositoryIndex(courseIndex: PublishedCourseIndex) {
        val listedCourseIds: Set<String> = courseIndex.courses().map { course -> course.id }.toSet()
        val omittedCourseIds: List<String> = publicationStates.keys.filter { courseId -> courseId !in listedCourseIds }

        for (courseId in omittedCourseIds) {
            forget(courseId)
        }

        indexState = CourseIndexLoadState.Loaded(courseIndex)
    }

    private fun forget(courseId: String) {
        publicationStates.remove(courseId)
        outlineStates.remove(courseId)
        removeUnitStates(courseId)
        updateMessages.remove(courseId)
        requestedPublicationCourseIds.remove(courseId)
    }

    private fun removeUnitStates(courseId: String) {
        val courseUnitKeys: List<UnitKey> = unitStates.keys.filter { key -> key.courseId == courseId }

        for (key in courseUnitKeys) {
            unitStates.remove(key)
        }
    }

    private suspend fun awaitFirstRepositoryIndexRead() {
        if (repositoryIndexState == RepositoryIndexLoadKind.Pending) {
            firstRepositoryIndexRead.await()
        }
    }

    private suspend fun showCachedOrEmbeddedPublication(library: CourseLibrary, courseId: String) {
        val cached: CoursePublication? = library.loadCachedCoursePublication(courseId)

        if (cached != null) {
            showIfNoneShown(cached)
            return
        }

        val embeddedIndexListsTheCourse: Boolean = embeddedIndexListsCourse(library, courseId)

        if (!embeddedIndexListsTheCourse) {
            return
        }

        try {
            showIfNoneShown(library.loadEmbeddedCoursePublication(courseId))
        } catch (cancellation: CancellationException) {
            throw cancellation
        } catch (error: Exception) {
            CoreErrorReport.record(error, "opening the copy of $courseId included in the app")
        }
    }

    private suspend fun embeddedIndexListsCourse(library: CourseLibrary, courseId: String): Boolean {
        try {
            return library.loadEmbeddedCourseIndex().courses().any { course -> course.id == courseId }
        } catch (cancellation: CancellationException) {
            throw cancellation
        } catch (error: Exception) {
            return false
        }
    }

    private suspend fun refreshPublication(library: CourseLibrary, courseId: String) {
        val publicationRefresh: JoinedRun = publicationRefreshes.getOrPut(courseId) { JoinedRun(scope) }

        publicationRefresh.run { readLatestPublication(library, courseId) }
    }

    /* A course's latest pointer is read only through an index read from a repository: this launch's, else the cached one. */
    private suspend fun readLatestPublication(library: CourseLibrary, courseId: String) {
        val courseIndex: PublishedCourseIndex? = shownIndex()

        if (courseIndex == null || courseIndex.isEmbedded()) {
            updateMessages[courseId] = SERVER_UNREACHABLE_MESSAGE
            failIfNoneShown(courseId, "The course server could not be reached, and the app includes no copy of this course.")
            return
        }

        if (courseIndex.courses().none { course -> course.id == courseId }) {
            return
        }

        try {
            val latest: CoursePublication = library.loadLatestCoursePublication(courseIndex, courseId)
            val isDifferentContent: Boolean = shownPublication(courseId)?.manifestSha256() != latest.manifestSha256()

            show(latest)
            updateMessages[courseId] = if (isDifferentContent) {
                "Downloaded the course content published as ${latest.publicationLabel()}."
            } else {
                "The course content is up to date."
            }
        } catch (cancellation: CancellationException) {
            throw cancellation
        } catch (error: Exception) {
            val message: String = CoreErrorReport.record(error, "fetching the latest publication of $courseId")

            updateMessages[courseId] = SERVER_UNREACHABLE_MESSAGE
            failIfNoneShown(courseId, message)
        }
    }

    private fun showIfNoneShown(publication: CoursePublication) {
        if (shownPublication(publication.courseId()) == null) {
            show(publication)
        }
    }

    private fun failIfNoneShown(courseId: String, message: String) {
        if (shownPublication(courseId) == null) {
            publicationStates[courseId] = CoursePublicationLoadState.Failed(message)
        }
    }

    /* The outline and units stay only when their clips are already registered under this publication's recordings base. */
    private fun show(publication: CoursePublication) {
        val courseId: String = publication.courseId()
        val shown: CoursePublication? = shownPublication(courseId)
        val keepsLoadedContent: Boolean = shown != null && servesTheSameContent(shown, publication)

        if (!keepsLoadedContent) {
            outlineStates.remove(courseId)
            removeUnitStates(courseId)
        }

        publicationStates[courseId] = CoursePublicationLoadState.Loaded(publication)
    }

    private fun isStillShown(publication: CoursePublication): Boolean {
        val shown: CoursePublication = shownPublication(publication.courseId())
            ?: return false

        return servesTheSameContent(shown, publication)
    }

    /* A shard a repository failed to serve rereads the course's pointer once, and adopts it, before the failure is reported. */
    private suspend fun reportShardFailure(error: Exception, publication: CoursePublication, activity: String): String {
        val message: String = CoreErrorReport.record(error, activity)
        val library: CourseLibrary? = library

        if (library != null && !publication.isEmbedded()) {
            refreshPublication(library, publication.courseId())
        }

        return message
    }

    /** Releases the core's library; the store does nothing after. */
    fun close() {
        library?.close()
    }

    private companion object {
        fun servesTheSameContent(first: CoursePublication, second: CoursePublication): Boolean {
            return first.manifestSha256() == second.manifestSha256() && first.recordingsBaseUrl() == second.recordingsBaseUrl()
        }
    }
}
