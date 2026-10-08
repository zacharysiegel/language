import Foundation
import Observation

enum CourseLoadState {
    case loading
    case loaded(CourseVersion, CourseOutlineRecord)
    case failed(String)
}

enum UnitLoadState {
    case loading
    case loaded(UnitOutlineRecord, UnitLessonRecord)
    case failed(String)
}

/// The course version on screen and the lessons opened from it.
@MainActor
@Observable
final class CourseStore {
    private let library: CourseLibrary?
    private var state: CourseLoadState = .loading
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

    func loadState() -> CourseLoadState {
        state
    }

    func shownVersion() -> CourseVersion? {
        if case .loaded(let version, _) = state {
            return version
        }

        return nil
    }

    func outline() -> CourseOutlineRecord? {
        if case .loaded(_, let outline) = state {
            return outline
        }

        return nil
    }

    func unitState(unitId: String) -> UnitLoadState? {
        unitStates[unitId]
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

    func ensureUnitLoaded(unitId: String) {
        guard case .loaded(let version, let outline) = state, unitStates[unitId] == nil else {
            return
        }

        guard let unitOutline = outline.units.first(where: { unit in unit.id == unitId }) else {
            return
        }

        unitStates[unitId] = .loading

        Task {
            await loadUnit(version: version, unitOutline: unitOutline)
        }
    }

    private func loadUnit(version: CourseVersion, unitOutline: UnitOutlineRecord) async {
        let loaded: UnitLoadState

        do {
            let lesson: UnitLessonRecord = try await version.loadUnit(unitId: unitOutline.id)
            loaded = .loaded(unitOutline, lesson)
        } catch {
            loaded = .failed(String(describing: error))
        }

        guard shownVersion() === version else {
            return
        }

        unitStates[unitOutline.id] = loaded
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
        unitStates = [:]
        state = .loaded(version, version.outline())
    }
}
