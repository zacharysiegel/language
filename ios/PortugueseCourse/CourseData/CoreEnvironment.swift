import Foundation

/// Where the app keeps its cache and progress, and where it finds its course.
struct CoreEnvironment {
    static let originInfoKey: String = "PortugueseCourseOrigin"
    static let progressFileArgumentKey: String = "PortugueseCourseProgressFile"
    static let embeddedBundleDirectoryName: String = "embedded_bundle"
    static let cacheDirectoryName: String = "artifacts"
    static let progressFileName: String = "progress.json"

    let configuration: CoreConfiguration
    let progressPath: String

    /// The app's own sandbox and bundle. Launch arguments `-PortugueseCourseOrigin <url>` and
    /// `-PortugueseCourseProgressFile <name>` override the origin and the progress file's name.
    static func standard() -> CoreEnvironment {
        let fileManager: FileManager = .default
        let cachesDirectory: URL = fileManager.urls(for: .cachesDirectory, in: .userDomainMask)[0]
        let supportDirectory: URL = fileManager.urls(for: .applicationSupportDirectory, in: .userDomainMask)[0]
        let cacheDirectory: URL = cachesDirectory.appendingPathComponent(cacheDirectoryName, isDirectory: true)

        prepareCacheDirectory(cacheDirectory)
        try? fileManager.createDirectory(at: supportDirectory, withIntermediateDirectories: true)

        let embeddedDirectory: URL = Bundle.main.url(forResource: embeddedBundleDirectoryName, withExtension: nil)
            ?? Bundle.main.bundleURL.appendingPathComponent(embeddedBundleDirectoryName)

        return CoreEnvironment(
            configuration: CoreConfiguration(
                appOrigin: appOrigin(),
                cacheDirectory: cacheDirectory.path,
                embeddedBundleDirectory: embeddedDirectory.path,
            ),
            progressPath: supportDirectory.appendingPathComponent(UserDefaults.standard.string(forKey: progressFileArgumentKey) ?? progressFileName).path,
        )
    }

    private static func appOrigin() -> String {
        let launchOverride: String? = UserDefaults.standard.string(forKey: originInfoKey)
        let declared: String? = Bundle.main.object(forInfoDictionaryKey: originInfoKey) as? String

        return launchOverride ?? declared ?? "http://127.0.0.1:3000"
    }

    /* The cache is reproducible from the repository, so it stays out of device backups. */
    private static func prepareCacheDirectory(_ directory: URL) {
        var mutableDirectory: URL = directory

        try? FileManager.default.createDirectory(at: directory, withIntermediateDirectories: true)

        var resourceValues: URLResourceValues = URLResourceValues()
        resourceValues.isExcludedFromBackup = true
        try? mutableDirectory.setResourceValues(resourceValues)
    }
}
