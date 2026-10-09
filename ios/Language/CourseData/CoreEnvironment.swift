import Foundation

/// Where the app keeps its cache and progress, and where it finds its course.
struct CoreEnvironment {
    static let originInfoKey: String = "LanguageOrigin"
    static let repositoryBaseUrlInfoKey: String = "LanguageRepositoryBaseUrl"
    static let progressFileArgumentKey: String = "LanguageProgressFile"
    static let embeddedBundleDirectoryName: String = "embedded_bundle"
    static let cacheDirectoryName: String = "artifacts"
    static let progressFileName: String = "language-progress.json"

    let configuration: CoreConfiguration
    let progressPath: String

    /// The app's own sandbox and bundle. Launch arguments `-LanguageOrigin <url>`, `-LanguageRepositoryBaseUrl <url>`
    /// and `-LanguageProgressFile <name>` override the origin, the static repository and the progress file's name.
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
                appOrigin: declaredUrl(infoKey: originInfoKey, fallback: "http://127.0.0.1:3000"),
                staticRepositoryBaseUrl: declaredUrl(infoKey: repositoryBaseUrlInfoKey, fallback: "http://127.0.0.1:3002"),
                cacheDirectory: cacheDirectory.path,
                embeddedBundleDirectory: embeddedDirectory.path,
            ),
            progressPath: supportDirectory.appendingPathComponent(UserDefaults.standard.string(forKey: progressFileArgumentKey) ?? progressFileName).path,
        )
    }

    private static func declaredUrl(infoKey: String, fallback: String) -> String {
        let launchOverride: String? = UserDefaults.standard.string(forKey: infoKey)
        let declared: String? = Bundle.main.object(forInfoDictionaryKey: infoKey) as? String

        return launchOverride ?? declared ?? fallback
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
