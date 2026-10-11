import SwiftUI

enum UnitItemIndex {
    static let minimumItemCount: Int = Int(unitItemIndexMinimumItemCount())
    static let contentCoordinateSpaceName: String = "unit-content"
}

/// The scroll target of a box of a unit tab, numbered from 1.
struct UnitItemAnchor: Hashable {
    let itemNumber: Int
}

extension UnitOutlineRecord {
    func itemLabel(itemNumber: Int) -> String {
        unitItemLabel(levelCode: levelCode, unitOrder: order, itemNumber: UInt32(itemNumber))
    }
}

/// The top of each box of the shown tab in the unit page's content, keyed by item number.
struct UnitItemContentTopsPreferenceKey: PreferenceKey {
    static let defaultValue: [Int: CGFloat] = [:]

    static func reduce(value: inout [Int: CGFloat], nextValue: () -> [Int: CGFloat]) {
        value.merge(nextValue()) { _, next in next }
    }
}

extension View {
    /// Makes the box the target of its index rail entry.
    func unitIndexItem(itemNumber: Int) -> some View {
        id(UnitItemAnchor(itemNumber: itemNumber))
            .background {
                GeometryReader { proxy in
                    let contentTop: CGFloat = proxy.frame(in: .named(UnitItemIndex.contentCoordinateSpaceName)).minY

                    Color.clear.preference(key: UnitItemContentTopsPreferenceKey.self, value: [itemNumber: contentTop])
                }
            }
    }
}

/// The box of the shown tab the learner is reading: the last whose top has passed the top of the page.
@MainActor
@Observable
final class UnitItemIndexTracker {
    private static let readingLineOffset: CGFloat = .spaceLg

    private var shownItemNumber: Int = 1

    @ObservationIgnored private var itemContentTops: [Int: CGFloat] = [:]
    @ObservationIgnored private var visibleContentTop: CGFloat = 0
    /* Held until the learner scrolls, since a box near the end cannot scroll up to the top. */
    @ObservationIgnored private var jumpedItemNumber: Int?

    func currentItemNumber() -> Int {
        shownItemNumber
    }

    func recordItemContentTops(_ contentTops: [Int: CGFloat]) {
        itemContentTops = contentTops
        updateShownItemNumber()
    }

    func recordVisibleContentTop(_ contentTop: CGFloat) {
        visibleContentTop = contentTop
        updateShownItemNumber()
    }

    func recordJump(itemNumber: Int) {
        jumpedItemNumber = itemNumber
        updateShownItemNumber()
    }

    func recordLearnerScroll() {
        jumpedItemNumber = nil
        updateShownItemNumber()
    }

    func recordTabChange() {
        jumpedItemNumber = nil
        updateShownItemNumber()
    }

    private func updateShownItemNumber() {
        let itemNumber: Int = jumpedItemNumber ?? scrolledItemNumber()

        if itemNumber != shownItemNumber {
            shownItemNumber = itemNumber
        }
    }

    private func scrolledItemNumber() -> Int {
        let readingLine: CGFloat = visibleContentTop + Self.readingLineOffset
        let passedItemNumbers: [Int] = itemContentTops.filter { _, contentTop in contentTop <= readingLine }.map { itemNumber, _ in itemNumber }

        return passedItemNumbers.max() ?? 1
    }
}

/// A column of box numbers at the page's trailing edge; tapping or dragging along it jumps to a box.
struct UnitItemIndexRail: View {
    /// One label per box, in box order.
    let itemLabels: [String]
    let itemNoun: String
    let currentItemNumber: Int
    let onJump: (Int) -> Void

    @State private var draggedItemNumber: Int?
    @AccessibilityFocusState private var focusedItemNumber: Int?

    var body: some View {
        GeometryReader { geometry in
            let entryHeight: CGFloat = min(.unitItemIndexEntryMaximumHeight, geometry.size.height / CGFloat(max(itemLabels.count, 1)))

            VStack(spacing: 0) {
                ForEach(Array(itemLabels.enumerated()), id: \.offset) { itemIndex, itemLabel in
                    entry(itemNumber: itemIndex + 1, itemLabel: itemLabel, height: entryHeight)
                }
            }
            .contentShape(Rectangle())
            .highPriorityGesture(jumpGesture(entryHeight: entryHeight))
            .frame(maxWidth: .infinity, maxHeight: .infinity)
        }
        .frame(width: .unitItemIndexRailWidth)
        .sensoryFeedback(.impact(weight: .light), trigger: draggedItemNumber) { _, newItemNumber in newItemNumber != nil }
        .accessibilityElement(children: .contain)
        .accessibilityLabel("\(itemNoun.capitalized) index")
        .accessibilityIdentifier("unit-item-index")
    }

    private func entry(itemNumber: Int, itemLabel: String, height: CGFloat) -> some View {
        let isCurrent: Bool = itemNumber == currentItemNumber

        return Button {
            onJump(itemNumber)
        } label: {
            Text("\(itemNumber)")
                .font(.labelText)
                .monospacedDigit()
                .foregroundStyle(isCurrent ? Color.accentActive : Color.inkMuted)
                .frame(width: .unitItemIndexRailWidth, height: height)
        }
        .buttonStyle(.plain)
        .accessibilityLabel("Go to \(itemNoun) \(itemLabel)")
        .accessibilityAddTraits(isCurrent ? .isSelected : [])
        .accessibilityIdentifier("unit-item-index-\(itemNumber)")
        .accessibilityFocused($focusedItemNumber, equals: itemNumber)
        .accessibilityAdjustableAction { direction in
            adjust(direction, fromItemNumber: itemNumber)
        }
    }

    /* A drag of no distance is a tap, so one gesture serves both. */
    private func jumpGesture(entryHeight: CGFloat) -> some Gesture {
        DragGesture(minimumDistance: 0)
            .onChanged { drag in
                let itemNumber: Int = itemNumber(atY: drag.location.y, entryHeight: entryHeight)

                if itemNumber != draggedItemNumber {
                    draggedItemNumber = itemNumber
                    onJump(itemNumber)
                }
            }
            .onEnded { _ in
                draggedItemNumber = nil
            }
    }

    private func itemNumber(atY y: CGFloat, entryHeight: CGFloat) -> Int {
        let itemIndex: Int = Int((y / entryHeight).rounded(.down))

        return min(max(itemIndex, 0), itemLabels.count - 1) + 1
    }

    private func adjust(_ direction: AccessibilityAdjustmentDirection, fromItemNumber startItemNumber: Int) {
        let targetItemNumber: Int

        switch direction {
        case .increment:
            targetItemNumber = min(startItemNumber + 1, itemLabels.count)
        case .decrement:
            targetItemNumber = max(startItemNumber - 1, 1)
        @unknown default:
            return
        }

        onJump(targetItemNumber)
        focusedItemNumber = targetItemNumber
    }
}
