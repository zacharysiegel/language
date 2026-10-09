import Foundation

/// The unified log subsystem the app's loggers and signposters write under.
enum LogSubsystem {
    static let identifier: String = Bundle.main.bundleIdentifier ?? "language"
}
