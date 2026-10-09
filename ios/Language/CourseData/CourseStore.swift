import Foundation
import Observation

enum CourseIndexLoadState {
    case loading
    case loaded(PublishedCourseIndex)
    case failed(String)
}

/// Whether this launch's first read of the repository's course index has finished.
enum RepositoryIndexLoadKind {
    case pending
    case loaded
    case failed
}

enum CoursePublicationLoadState {
    case loading
    case loaded(CoursePublication)
    case failed(String)
}

enum OutlineLoadState {
    case loading
    case loaded(CourseOutlineRecord)
    case failed(String)
}

enum UnitLoadState {
    case loading
    case loaded(UnitOutlineRecord, UnitLessonRecord)
    case failed(String)
}

private enum StartKind {
    case notStarted
    case starting
    case started
}

private struct UnitKey: Hashable {
    let courseId: String
    let unitId: String
}

/// The course index on screen, the publication of each course opened, and the outlines and lessons opened from them.
@MainActor
@Observable
final class CourseStore {
    private let library: CourseLibrary?
    private var indexState: CourseIndexLoadState = .loading
    private var repositoryIndexState: RepositoryIndexLoadKind = .pending
    private var publicationStates: [String: CoursePublicationLoadState] = [:]
    private var outlineStates: [String: OutlineLoadState] = [:]
    private var unitStates: [UnitKey: UnitLoadState] = [:]
    private var updateMessages: [String: String] = [:]

    @ObservationIgnored private var startKind: StartKind = .notStarted
    @ObservationIgnored private var embeddedIndexFailure: String?
    @ObservationIgnored private var requestedPublicationCourseIds: Set<String> = []
    @ObservationIgnored private var indexRefreshTask: Task<Void, Never>?
    @ObservationIgnored private var publicationRefreshTasks: [String: Task<Void, Never>] = [:]
    @ObservationIgnored private var repositoryIndexWaiters: [CheckedContinuation<Void, Never>] = []

    init(environment: CoreEnvironment) {
        do {
            library = try CourseLibrary(configuration: environment.configuration)
        } catch {
            library = nil
            indexState = .failed(CoreErrorReport.record(error, while: "opening the course library"))
        }
    }

    func courseLibrary() -> CourseLibrary? {
        library
    }

    func indexLoadState() -> CourseIndexLoadState {
        indexState
    }

    func repositoryIndexLoadState() -> RepositoryIndexLoadKind {
        repositoryIndexState
    }

    func shownIndex() -> PublishedCourseIndex? {
        if case .loaded(let courseIndex) = indexState {
            return courseIndex
        }

        return nil
    }

    func courses() -> [CourseSummaryRecord] {
        shownIndex()?.courses() ?? []
    }

    func listsCourse(courseId: String) -> Bool {
        courses().contains { course in course.id == courseId }
    }

    /// The course the learner opened last when the index lists it, else the only course the index lists.
    func activeCourseId(lastCourseId: String?) -> String? {
        let courses: [CourseSummaryRecord] = courses()

        if let lastCourseId, courses.contains(where: { course in course.id == lastCourseId }) {
            return lastCourseId
        }

        return courses.count == 1 ? courses[0].id : nil
    }

    func publicationState(courseId: String) -> CoursePublicationLoadState? {
        publicationStates[courseId]
    }

    func shownPublication(courseId: String) -> CoursePublication? {
        if case .loaded(let publication) = publicationStates[courseId] {
            return publication
        }

        return nil
    }

    /// Changes whenever the course's loaded outline and units are discarded for a newly shown publication.
    func shownContentKey(courseId: String) -> String? {
        shownPublication(courseId: courseId).map { publication in "\(publication.manifestSha256()) \(publication.recordingsBaseUrl())" }
    }

    func outlineState(courseId: String) -> OutlineLoadState? {
        outlineStates[courseId]
    }

    func outline(courseId: String) -> CourseOutlineRecord? {
        if case .loaded(let outline) = outlineStates[courseId] {
            return outline
        }

        return nil
    }

    func unitState(courseId: String, unitId: String) -> UnitLoadState? {
        unitStates[UnitKey(courseId: courseId, unitId: unitId)]
    }

    func updateMessage(courseId: String) -> String? {
        updateMessages[courseId]
    }

    /// Shows the cached course index at once, else the one included in the app, then reads the repository's.
    func start() async {
        guard let library, startKind == .notStarted else {
            return
        }

        startKind = .starting
        await showCachedOrEmbeddedIndex(library: library)
        startKind = .started
        await refresh(courseId: nil)
    }

    /// Reads the repository's course index and, for `courseId`, the course's latest pointer. A read of either already
    /// under way is joined rather than repeated.
    func refresh(courseId: String?) async {
        guard let library, startKind == .started else {
            return
        }

        await refreshIndex(library: library)

        if let courseId {
            await refreshPublication(library: library, courseId: courseId)
        }
    }

    /// Retries a course whose publication or outline failed to load.
    func retry(courseId: String) async {
        if case .failed = outlineStates[courseId] {
            outlineStates[courseId] = nil
        }

        await refresh(courseId: courseId)
        ensureOutlineLoaded(courseId: courseId)
    }

    /// Once per launch: shows the cached publication at once, else the one included in the app, then the publication
    /// the course's latest pointer names once the repository's course index has been read.
    func ensurePublicationLoaded(courseId: String) {
        guard let library, !requestedPublicationCourseIds.contains(courseId) else {
            return
        }

        requestedPublicationCourseIds.insert(courseId)

        if publicationStates[courseId] == nil {
            publicationStates[courseId] = .loading
        }

        Task {
            if shownPublication(courseId: courseId) == nil {
                await showCachedOrEmbeddedPublication(library: library, courseId: courseId)
            }

            await repositoryIndexLoadFinished()
            await refreshPublication(library: library, courseId: courseId)
        }
    }

    func ensureOutlineLoaded(courseId: String) {
        guard let publication = shownPublication(courseId: courseId), outlineStates[courseId] == nil else {
            return
        }

        outlineStates[courseId] = .loading

        Task {
            let loaded: OutlineLoadState

            do {
                loaded = .loaded(try await publication.loadCourse())
            } catch {
                loaded = .failed(await reportShardFailure(error, of: publication, while: "loading course \(courseId)"))
            }

            if isStillShown(publication) {
                outlineStates[courseId] = loaded
            }
        }
    }

    func ensureUnitLoaded(courseId: String, unitId: String) {
        let key: UnitKey = UnitKey(courseId: courseId, unitId: unitId)

        guard let publication = shownPublication(courseId: courseId), unitStates[key] == nil else {
            return
        }

        guard let unitOutline = outline(courseId: courseId)?.units.first(where: { unit in unit.id == unitId }) else {
            return
        }

        unitStates[key] = .loading

        Task {
            let loaded: UnitLoadState

            do {
                loaded = .loaded(unitOutline, try await publication.loadUnit(unitId: unitId))
            } catch {
                loaded = .failed(await reportShardFailure(error, of: publication, while: "loading unit \(unitId) of \(courseId)"))
            }

            if isStillShown(publication) {
                unitStates[key] = loaded
            }
        }
    }

    // MARK: - The course index

    private func showCachedOrEmbeddedIndex(library: CourseLibrary) async {
        if let cached = await library.loadCachedCourseIndex() {
            indexState = .loaded(cached)
            return
        }

        do {
            indexState = .loaded(try await library.loadEmbeddedCourseIndex())
        } catch {
            embeddedIndexFailure = CoreErrorReport.record(error, while: "opening the course index included in the app")
        }
    }

    private func refreshIndex(library: CourseLibrary) async {
        if let indexRefreshTask {
            await indexRefreshTask.value
            return
        }

        let task: Task<Void, Never> = Task { await readRepositoryIndex(library: library) }

        indexRefreshTask = task
        await task.value
        indexRefreshTask = nil
    }

    private func readRepositoryIndex(library: CourseLibrary) async {
        do {
            showRepositoryIndex(try await library.loadLatestCourseIndex())
            repositoryIndexState = .loaded
        } catch {
            let message: String = CoreErrorReport.record(error, while: "fetching the course index")

            if repositoryIndexState == .pending {
                repositoryIndexState = .failed
            }

            if shownIndex() == nil {
                indexState = .failed(embeddedIndexFailure.map { embeddedFailure in "\(message) (the copy included in the app: \(embeddedFailure))" } ?? message)
            }
        }

        resumeRepositoryIndexWaiters()
    }

    /* The repository's index is authoritative for the course list, so a course it omits is forgotten. */
    private func showRepositoryIndex(_ courseIndex: PublishedCourseIndex) {
        let listedCourseIds: Set<String> = Set(courseIndex.courses().map { course in course.id })
        let omittedCourseIds: [String] = publicationStates.keys.filter { courseId in !listedCourseIds.contains(courseId) }

        for courseId in omittedCourseIds {
            forget(courseId: courseId)
        }

        indexState = .loaded(courseIndex)
    }

    private func forget(courseId: String) {
        publicationStates[courseId] = nil
        outlineStates[courseId] = nil
        unitStates = unitStates.filter { key, _ in key.courseId != courseId }
        updateMessages[courseId] = nil
        requestedPublicationCourseIds.remove(courseId)
    }

    private func repositoryIndexLoadFinished() async {
        guard repositoryIndexState == .pending else {
            return
        }

        await withCheckedContinuation { (continuation: CheckedContinuation<Void, Never>) in
            repositoryIndexWaiters.append(continuation)
        }
    }

    private func resumeRepositoryIndexWaiters() {
        let waiters: [CheckedContinuation<Void, Never>] = repositoryIndexWaiters

        repositoryIndexWaiters = []

        for waiter in waiters {
            waiter.resume()
        }
    }

    // MARK: - A course's publication

    private func showCachedOrEmbeddedPublication(library: CourseLibrary, courseId: String) async {
        if let cached = await library.loadCachedCoursePublication(courseId: courseId) {
            showIfNoneShown(cached)
            return
        }

        let embeddedIndex: PublishedCourseIndex? = try? await library.loadEmbeddedCourseIndex()
        let embeddedIndexListsTheCourse: Bool = embeddedIndex?.courses().contains { course in course.id == courseId } ?? false

        guard embeddedIndexListsTheCourse else {
            return
        }

        do {
            showIfNoneShown(try await library.loadEmbeddedCoursePublication(courseId: courseId))
        } catch {
            _ = CoreErrorReport.record(error, while: "opening the copy of \(courseId) included in the app")
        }
    }

    private func refreshPublication(library: CourseLibrary, courseId: String) async {
        if let inFlight = publicationRefreshTasks[courseId] {
            await inFlight.value
            return
        }

        let task: Task<Void, Never> = Task { await readLatestPublication(library: library, courseId: courseId) }

        publicationRefreshTasks[courseId] = task
        await task.value
        publicationRefreshTasks[courseId] = nil
    }

    /* A course's latest pointer is read only through an index read from a repository: this launch's, else the cached one. */
    private func readLatestPublication(library: CourseLibrary, courseId: String) async {
        guard let courseIndex = shownIndex(), !courseIndex.isEmbedded() else {
            updateMessages[courseId] = "The course server could not be reached."
            failIfNoneShown(courseId: courseId, message: "The course server could not be reached, and the app includes no copy of this course.")
            return
        }

        guard courseIndex.courses().contains(where: { course in course.id == courseId }) else {
            return
        }

        do {
            let latest: CoursePublication = try await library.loadLatestCoursePublication(courseIndex: courseIndex, courseId: courseId)
            let isDifferentContent: Bool = shownPublication(courseId: courseId)?.manifestSha256() != latest.manifestSha256()

            show(latest)
            updateMessages[courseId] = isDifferentContent
                ? "Downloaded the course content published as \(latest.publicationLabel())."
                : "The course content is up to date."
        } catch {
            let message: String = CoreErrorReport.record(error, while: "fetching the latest publication of \(courseId)")

            updateMessages[courseId] = "The course server could not be reached."
            failIfNoneShown(courseId: courseId, message: message)
        }
    }

    private func showIfNoneShown(_ publication: CoursePublication) {
        if shownPublication(courseId: publication.courseId()) == nil {
            show(publication)
        }
    }

    private func failIfNoneShown(courseId: String, message: String) {
        if shownPublication(courseId: courseId) == nil {
            publicationStates[courseId] = .failed(message)
        }
    }

    /* The outline and units stay only when their clips are already registered under this publication's recordings base. */
    private func show(_ publication: CoursePublication) {
        let courseId: String = publication.courseId()
        let keepsLoadedContent: Bool = shownPublication(courseId: courseId).map { shown in Self.servesTheSameContent(shown, publication) } ?? false

        if !keepsLoadedContent {
            outlineStates[courseId] = nil
            unitStates = unitStates.filter { key, _ in key.courseId != courseId }
        }

        publicationStates[courseId] = .loaded(publication)
    }

    private func isStillShown(_ publication: CoursePublication) -> Bool {
        shownPublication(courseId: publication.courseId()).map { shown in Self.servesTheSameContent(shown, publication) } ?? false
    }

    private static func servesTheSameContent(_ first: CoursePublication, _ second: CoursePublication) -> Bool {
        first.manifestSha256() == second.manifestSha256() && first.recordingsBaseUrl() == second.recordingsBaseUrl()
    }

    /* A shard a repository failed to serve rereads the course's pointer once, and adopts it, before the failure is reported. */
    private func reportShardFailure(_ error: Error, of publication: CoursePublication, while activity: String) async -> String {
        let message: String = CoreErrorReport.record(error, while: activity)

        if let library, !publication.isEmbedded() {
            await refreshPublication(library: library, courseId: publication.courseId())
        }

        return message
    }
}
