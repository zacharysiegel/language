import Observation
import SwiftUI

/// How long an operation the learner started runs before its pending state is shown.
enum PendingIndication {
    static let delay: Duration = .milliseconds(Int(pendingIndicationDelayMilliseconds()))
    static let glyphSystemName: String = "ellipsis"

    /// Returns `.elapsed` once the delay has passed, `.cancelled` when the waiting task is cancelled first.
    static func waitForDelay() async -> PendingDelayOutcome {
        do {
            try await Task.sleep(for: delay)
        } catch {
            return .cancelled
        }

        return .elapsed
    }
}

enum PendingDelayOutcome {
    case elapsed
    case cancelled
}

/// Shows `PendingGlyph` in place of its content while `isIndicated`, keeping the content's size.
struct PendingGlyphSwapModifier: ViewModifier {
    let isIndicated: Bool
    let glyphFont: Font

    func body(content: Content) -> some View {
        content
            .opacity(isIndicated ? 0 : 1)
            .overlay {
                if isIndicated {
                    PendingGlyph(font: glyphFont)
                        .fixedSize()
                }
            }
    }
}

/// The static glyph a control shows while what it started is still running.
struct PendingGlyph: View {
    let font: Font

    var body: some View {
        Image(systemName: PendingIndication.glyphSystemName)
            .font(font)
            .accessibilityHidden(true)
    }
}

/// Gives a pending control its accessibility value and announces it once it is indicated.
struct PendingAccessibilityModifier: ViewModifier {
    let isIndicated: Bool
    let value: String

    func body(content: Content) -> some View {
        content
            .accessibilityValue(isIndicated ? value : "")
            .onChange(of: isIndicated) { _, isNowIndicated in
                if isNowIndicated {
                    AccessibilityNotification.Announcement(value).post()
                }
            }
    }
}

extension View {
    func pendingGlyphSwap(isIndicated: Bool, glyphFont: Font) -> some View {
        modifier(PendingGlyphSwapModifier(isIndicated: isIndicated, glyphFont: glyphFont))
    }

    func pendingAccessibility(isIndicated: Bool, value: String) -> some View {
        modifier(PendingAccessibilityModifier(isIndicated: isIndicated, value: value))
    }
}

/// Where a control-started task is between its start and its end.
enum PendingActivityPhase {
    case idle
    case running
    case indicated
}

/// One task a control starts at a time, indicated once it outlasts `PendingIndication.delay`.
@MainActor
@Observable
final class PendingActivity {
    private var phase: PendingActivityPhase = .idle

    func isIndicated() -> Bool {
        phase == .indicated
    }

    /// Ignored while an earlier task is still running.
    func run(_ operation: @escaping @MainActor () async -> Void) {
        guard phase == .idle else {
            return
        }

        phase = .running

        let indication: Task<Void, Never> = Task {
            let delayOutcome: PendingDelayOutcome = await PendingIndication.waitForDelay()

            if delayOutcome == .elapsed, phase == .running {
                phase = .indicated
            }
        }

        Task {
            await operation()
            indication.cancel()
            phase = .idle
        }
    }
}

/// A loading message which appears only once the load has outlasted `PendingIndication.delay`.
struct PendingStatusMessage: View {
    let text: String

    @State private var isShown: Bool = false

    var body: some View {
        StatusMessage(text: text)
            .opacity(isShown ? 1 : 0)
            .accessibilityHidden(!isShown)
            .accessibilityAddTraits(.updatesFrequently)
            .accessibilityIdentifier("pending-status")
            .task {
                await showAfterDelay()
            }
    }

    private func showAfterDelay() async {
        let delayOutcome: PendingDelayOutcome = await PendingIndication.waitForDelay()

        guard delayOutcome == .elapsed else {
            return
        }

        isShown = true
        AccessibilityNotification.Announcement(text).post()
    }
}
