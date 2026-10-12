import SwiftUI

/// An entry of an index rail, numbered from 1 in rail order.
struct IndexRailEntry: Hashable {
    /// Short enough to fit the rail's width; unique within the rail.
    let visibleText: String
    let accessibilityLabel: String
}

/// The scroll target of an index rail entry.
struct IndexRailAnchor: Hashable {
    let entryNumber: Int
}

/// The top of each index rail target in the page's content, keyed by entry number.
struct IndexRailTargetTopsPreferenceKey: PreferenceKey {
    static let defaultValue: [Int: CGFloat] = [:]

    static func reduce(value: inout [Int: CGFloat], nextValue: () -> [Int: CGFloat]) {
        value.merge(nextValue()) { _, next in next }
    }
}

extension View {
    /// Makes this view the target of its index rail entry.
    func indexRailTarget(entryNumber: Int) -> some View {
        id(IndexRailAnchor(entryNumber: entryNumber))
            .background {
                GeometryReader { proxy in
                    let contentTop: CGFloat = proxy.frame(in: .named(IndexRail.contentCoordinateSpaceName)).minY

                    Color.clear.preference(key: IndexRailTargetTopsPreferenceKey.self, value: [entryNumber: contentTop])
                }
            }
    }

    /// Marks this view as the scrolled content holding the targets `tracker` follows.
    func indexRailContent(_ tracker: IndexRailTracker) -> some View {
        coordinateSpace(.named(IndexRail.contentCoordinateSpaceName))
            .scrollMomentumStopper(tracker.momentumStopper)
    }

    /// Reports this scroll view's targets, scrolled position and scroll phase to `tracker`.
    func indexRailScrollTracking(_ tracker: IndexRailTracker) -> some View {
        onPreferenceChange(IndexRailTargetTopsPreferenceKey.self) { contentTops in
            tracker.recordTargetContentTops(contentTops)
        }
        .onScrollGeometryChange(for: CGFloat.self) { geometry in
            geometry.visibleRect.minY + geometry.contentInsets.top
        } action: { _, visibleContentTop in
            tracker.recordVisibleContentTop(visibleContentTop)
        }
        .onScrollPhaseChange { _, newPhase in
            tracker.recordScrollPhase(newPhase)
        }
    }
}

/// What an index rail follows of its page's scroll view.
@MainActor
@Observable
final class IndexRailTracker {
    private static let readingLineOffset: CGFloat = .spaceLg

    private var shownEntryNumber: Int = 1
    private var scrollPhase: ScrollPhase = .idle

    @ObservationIgnored let momentumStopper: ScrollMomentumStopper = ScrollMomentumStopper()
    @ObservationIgnored private var targetContentTops: [Int: CGFloat] = [:]
    @ObservationIgnored private var visibleContentTop: CGFloat = 0
    /* Held until the learner scrolls, since a target near the end cannot scroll up to the top. */
    @ObservationIgnored private var jumpedEntryNumber: Int?

    /// The entry the learner is reading: the one jumped to, or else the last whose target's top has passed the top of the page.
    func currentEntryNumber() -> Int {
        shownEntryNumber
    }

    func isPageScrolling() -> Bool {
        scrollPhase.isScrolling
    }

    func recordTargetContentTops(_ contentTops: [Int: CGFloat]) {
        targetContentTops = contentTops
        updateShownEntryNumber()
    }

    func recordVisibleContentTop(_ contentTop: CGFloat) {
        visibleContentTop = contentTop
        updateShownEntryNumber()
    }

    func recordScrollPhase(_ phase: ScrollPhase) {
        scrollPhase = phase

        if phase == .interacting {
            jumpedEntryNumber = nil
            updateShownEntryNumber()
        }
    }

    /// Records that the targets were replaced, so an entry number may name a different target.
    func recordContentChange() {
        jumpedEntryNumber = nil
        updateShownEntryNumber()
    }

    func jump(toEntryNumber entryNumber: Int, scrollProxy: ScrollViewProxy) {
        momentumStopper.stopMomentum()
        jumpedEntryNumber = entryNumber
        updateShownEntryNumber()
        scrollProxy.scrollTo(IndexRailAnchor(entryNumber: entryNumber), anchor: .top)
    }

    private func updateShownEntryNumber() {
        let entryNumber: Int = jumpedEntryNumber ?? scrolledEntryNumber()

        if entryNumber != shownEntryNumber {
            shownEntryNumber = entryNumber
        }
    }

    private func scrolledEntryNumber() -> Int {
        let readingLine: CGFloat = visibleContentTop + Self.readingLineOffset
        let passedEntryNumbers: [Int] = targetContentTops.filter { _, contentTop in contentTop <= readingLine }.map { entryNumber, _ in entryNumber }

        return passedEntryNumbers.max() ?? 1
    }
}

/// A column of short entries over the page's trailing edge; tapping or dragging along it jumps to an entry's target.
/// It shows nothing with fewer than `IndexRail.minimumEntryCount` entries.
/// It is shown while the page scrolls or the rail is in use, and hidden after `IndexRail.idleHideDelay` without either.
struct IndexRail: View {
    static let minimumEntryCount: Int = Int(unitItemIndexMinimumItemCount())
    static let contentCoordinateSpaceName: String = "index-rail-content"
    /* About as long as the system scroll indicator stays after scrolling stops. */
    static let idleHideDelay: Duration = .milliseconds(1500)
    static let fadeDuration: TimeInterval = 0.2

    let entries: [IndexRailEntry]
    let railAccessibilityLabel: String
    /// Each entry's identifier is this followed by `-` and its visible text.
    let railAccessibilityIdentifier: String
    let tracker: IndexRailTracker
    let scrollProxy: ScrollViewProxy

    @State private var isShown: Bool = false
    @GestureState private var draggedEntryNumber: Int?
    @AccessibilityFocusState private var accessibilityFocusedEntryNumber: Int?
    @FocusState private var keyboardFocusedEntryNumber: Int?

    var body: some View {
        if entries.count >= Self.minimumEntryCount {
            rail()
        }
    }

    private func rail() -> some View {
        GeometryReader { geometry in
            let entryHeight: CGFloat = min(.indexRailEntryMaximumHeight, geometry.size.height / CGFloat(entries.count))

            VStack(spacing: 0) {
                ForEach(Array(entries.enumerated()), id: \.offset) { entryIndex, entry in
                    entryButton(entryNumber: entryIndex + 1, entry: entry, height: entryHeight)
                }
            }
            .shadowedBox(shadowOffset: .smallBoxShadowOffset)
            .contentShape(Rectangle())
            .highPriorityGesture(jumpGesture(entryHeight: entryHeight))
            .frame(maxWidth: .infinity, maxHeight: .infinity)
        }
        .frame(width: .indexRailWidth)
        .opacity(isShown ? 1 : 0)
        .animation(.easeOut(duration: Self.fadeDuration), value: isShown)
        .allowsHitTesting(isShown)
        .task(id: isInUse()) {
            await updateShown(isInUse: isInUse())
        }
        .sensoryFeedback(.impact(weight: .light), trigger: draggedEntryNumber) { _, newEntryNumber in newEntryNumber != nil }
        .onChange(of: draggedEntryNumber) { _, newEntryNumber in
            guard let newEntryNumber else {
                return
            }

            jump(toEntryNumber: newEntryNumber)
        }
        .accessibilityElement(children: .contain)
        .accessibilityLabel(railAccessibilityLabel)
        .accessibilityIdentifier(railAccessibilityIdentifier)
        .padding(.trailing, .indexRailEdgeInset)
    }

    private func isInUse() -> Bool {
        tracker.isPageScrolling() || draggedEntryNumber != nil || accessibilityFocusedEntryNumber != nil || keyboardFocusedEntryNumber != nil
    }

    private func updateShown(isInUse: Bool) async {
        guard !isInUse else {
            isShown = true
            return
        }

        do {
            try await Task.sleep(for: Self.idleHideDelay)
        } catch {
            return
        }

        isShown = false
    }

    private func jump(toEntryNumber entryNumber: Int) {
        tracker.jump(toEntryNumber: entryNumber, scrollProxy: scrollProxy)
    }

    private func entryButton(entryNumber: Int, entry: IndexRailEntry, height: CGFloat) -> some View {
        let isCurrent: Bool = entryNumber == tracker.currentEntryNumber()

        return Button {
            jump(toEntryNumber: entryNumber)
        } label: {
            Text(entry.visibleText)
                .font(.indexEntryText)
                .fontWeight(isCurrent ? .bold : .regular)
                .foregroundStyle(isCurrent ? Color.ink : Color.inkMuted)
                .frame(width: .indexRailWidth, height: height)
                .background(isCurrent ? Color.accentTint : Color.clear)
        }
        .buttonStyle(.plain)
        .accessibilityLabel(entry.accessibilityLabel)
        .accessibilityAddTraits(isCurrent ? .isSelected : [])
        .accessibilityIdentifier("\(railAccessibilityIdentifier)-\(entry.visibleText)")
        .accessibilityFocused($accessibilityFocusedEntryNumber, equals: entryNumber)
        .focused($keyboardFocusedEntryNumber, equals: entryNumber)
        .accessibilityAdjustableAction { direction in
            adjust(direction, fromEntryNumber: entryNumber)
        }
    }

    /* A drag of no distance is a tap, so one gesture serves both. */
    private func jumpGesture(entryHeight: CGFloat) -> some Gesture {
        DragGesture(minimumDistance: 0)
            .updating($draggedEntryNumber) { drag, draggedEntryNumber, _ in
                draggedEntryNumber = entryNumber(atY: drag.location.y, entryHeight: entryHeight)
            }
    }

    private func entryNumber(atY y: CGFloat, entryHeight: CGFloat) -> Int {
        let entryIndex: Int = Int((y / entryHeight).rounded(.down))

        return min(max(entryIndex, 0), entries.count - 1) + 1
    }

    private func adjust(_ direction: AccessibilityAdjustmentDirection, fromEntryNumber startEntryNumber: Int) {
        let targetEntryNumber: Int

        switch direction {
        case .increment:
            targetEntryNumber = min(startEntryNumber + 1, entries.count)
        case .decrement:
            targetEntryNumber = max(startEntryNumber - 1, 1)
        @unknown default:
            return
        }

        jump(toEntryNumber: targetEntryNumber)
        accessibilityFocusedEntryNumber = targetEntryNumber
    }
}
