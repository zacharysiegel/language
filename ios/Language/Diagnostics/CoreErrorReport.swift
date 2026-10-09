import Foundation
import OSLog

/// Sends failures to the unified log, with the Rust call stack when the core raised them.
enum CoreErrorReport {
    static let logger: Logger = Logger(subsystem: LogSubsystem.identifier, category: "core")

    /// Logs `error` with what was being done and returns the text to show the learner.
    static func record(_ error: Error, while activity: String) -> String {
        switch error {
        case CoreError.Failure(let message, let backtrace):
            logger.error("\(activity, privacy: .public) failed: \(message, privacy: .public)\nRust backtrace:\n\(backtrace, privacy: .public)")
            return message
        default:
            let message: String = String(describing: error)
            logger.error("\(activity, privacy: .public) failed: \(message, privacy: .public)")
            return message
        }
    }
}
