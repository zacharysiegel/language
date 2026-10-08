import Foundation
import Observation

enum VersionLoadState {
    case loading
    case loaded(CourseVersion)
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

/// The version of the courses on screen, and the outlines and lessons opened from it.
@MainActor
@Observable
final class CourseStore {
    private let library: CourseLibrary?
    private var state: VersionLoadState = .loading
    private var outlineStates: [String: OutlineLoadState] = [:]
    private var unitStates: [String: UnitLoadState] = [:]
    private var updateMessage: String?

    init(environment: CoreEnvironment) {
        do {
            library = try CourseLibrary(configuration: environment.configuration)
        } catch {
            library = nil
            state = .failed(String(describing: error))
        }
    }

    func loadState() -> VersionLoadState {
        state
    }

    func shownVersion() -> CourseVersion? {
        if case .loaded(let version) = state {
            return version
        }

        return nil
    }

    func courses() -> [CourseSummaryRecord] {
        shownVersion()?.courses() ?? []
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
        unitStates[Self.unitKey(courseId: courseId, unitId: unitId)]
    }

    func lastUpdateMessage() -> String? {
        updateMessage
    }

    /// Shows the cached version at once when there is one, then the latest version if it differs. The embedded
    /// bundle is the floor when nothing else can be read.
    func start() async {
        guard let library else {
            return
        }

        let cached: CourseVersion? = await library.loadCachedCourse()

        if let cached {
            show(cached)
        }

        await checkForLatest(library: library, hasShownVersion: cached != nil)
    }

    /// Fetches the latest version and shows it when it differs from the one on screen.
    func refresh() async {
        guard let library else {
            return
        }

        await checkForLatest(library: library, hasShownVersion: shownVersion() != nil)
    }

    func ensureOutlineLoaded(courseId: String) {
        guard let version = shownVersion(), outlineStates[courseId] == nil else {
            return
        }

        outlineStates[courseId] = .loading

        Task {
            let loaded: OutlineLoadState

            do {
                loaded = .loaded(try await version.loadCourse(courseId: courseId))
            } catch {
                loaded = .failed(String(describing: error))
            }

            if shownVersion() === version {
                outlineStates[courseId] = loaded
            }
        }
    }

    func ensureUnitLoaded(courseId: String, unitId: String) {
        let key: String = Self.unitKey(courseId: courseId, unitId: unitId)

        guard let version = shownVersion(), unitStates[key] == nil else {
            return
        }

        guard let unitOutline = outline(courseId: courseId)?.units.first(where: { unit in unit.id == unitId }) else {
            return
        }

        unitStates[key] = .loading

        Task {
            let loaded: UnitLoadState

            do {
                loaded = .loaded(unitOutline, try await version.loadUnit(courseId: courseId, unitId: unitId))
            } catch {
                loaded = .failed(String(describing: error))
            }

            if shownVersion() === version {
                unitStates[key] = loaded
            }
        }
    }

    private static func unitKey(courseId: String, unitId: String) -> String {
        "\(courseId)/\(unitId)"
    }

    private func checkForLatest(library: CourseLibrary, hasShownVersion: Bool) async {
        do {
            let latest: CourseVersion = try await library.loadLatestCourse()
            let isNewVersion: Bool = latest.version() != shownVersion()?.version()

            if isNewVersion {
                show(latest)
                updateMessage = "Updated to version \(latest.version())."
            } else {
                updateMessage = "You have the latest version."
            }
        } catch {
            updateMessage = "The course server could not be reached."

            if !hasShownVersion {
                await showEmbedded(library: library, latestError: error)
            }
        }
    }

    private func showEmbedded(library: CourseLibrary, latestError: Error) async {
        do {
            show(try await library.loadEmbeddedCourse())
        } catch {
            state = .failed("\(error) (the course server: \(latestError))")
        }
    }

    private func show(_ version: CourseVersion) {
        outlineStates = [:]
        unitStates = [:]
        state = .loaded(version)
    }
}
