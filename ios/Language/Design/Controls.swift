import SwiftUI

/// An uppercase micro-label that carries structure, as the web's `.field-label` and captions do.
struct MicroLabel: View {
    let text: String

    var body: some View {
        Text(text.uppercased())
            .font(.labelText)
            .tracking(0.6)
            .foregroundStyle(Color.inkMuted)
    }
}

struct HairlineRule: View {
    var body: some View {
        Rectangle()
            .fill(Color.rule)
            .frame(height: .hairline)
    }
}

/// A square, hairline-bordered button: ink by default, filled red when `isPrimary`.
struct SquareButtonStyle: ButtonStyle {
    var isPrimary: Bool = false
    var isCompact: Bool = false

    func makeBody(configuration: Configuration) -> some View {
        SquareButtonBody(configuration: configuration, isPrimary: isPrimary, isCompact: isCompact)
    }
}

private struct SquareButtonBody: View {
    let configuration: ButtonStyleConfiguration
    let isPrimary: Bool
    let isCompact: Bool

    @Environment(\.isEnabled) private var isEnabled

    var body: some View {
        configuration.label
            .font(.bodyText)
            .padding(.horizontal, isCompact ? .spaceSm : .spaceMd)
            .padding(.vertical, isCompact ? .spaceXs : .spaceSm)
            .foregroundStyle(foreground())
            .background(background())
            .overlay(Rectangle().stroke(border(), lineWidth: .hairline))
            .opacity(isEnabled ? 1 : 0.4)
            .contentShape(Rectangle())
    }

    private func foreground() -> Color {
        isPrimary ? Color.paper : Color.ink
    }

    private func background() -> Color {
        if isPrimary {
            return configuration.isPressed ? Color.accentHover : Color.accentActive
        }

        return configuration.isPressed ? Color.rule : Color.paper
    }

    private func border() -> Color {
        isPrimary ? Color.accentActive : Color.ink
    }
}

/// The state a choice button shows after the learner answers.
enum ChoiceMarkKind {
    case unmarked
    case selected
    case right
    case wrong
}

struct ChoiceButtonStyle: ButtonStyle {
    let mark: ChoiceMarkKind

    func makeBody(configuration: Configuration) -> some View {
        configuration.label
            .font(.bodyText)
            .multilineTextAlignment(.leading)
            .frame(maxWidth: .infinity, alignment: .leading)
            .padding(.spaceSm + .spaceXs)
            .foregroundStyle(foreground())
            .background(background(isPressed: configuration.isPressed))
            .overlay(Rectangle().stroke(border(), lineWidth: mark == .unmarked ? .hairline : 2))
            .contentShape(Rectangle())
    }

    private func foreground() -> Color {
        switch mark {
        case .right:
            return Color.goodText
        case .wrong:
            return Color.accentActive
        case .unmarked, .selected:
            return Color.ink
        }
    }

    private func background(isPressed: Bool) -> Color {
        isPressed ? Color.rule : Color.paper
    }

    private func border() -> Color {
        switch mark {
        case .unmarked:
            return Color.ink
        case .selected:
            return Color.accentLink
        case .right:
            return Color.goodText
        case .wrong:
            return Color.accentActive
        }
    }
}

/// A small hairline-bordered tag; `isInk` fills it black, `isActive` outlines it red.
struct TagView: View {
    let text: String
    var isInk: Bool = false
    var isActive: Bool = false

    var body: some View {
        Text(text.uppercased())
            .font(.labelText)
            .tracking(0.4)
            .padding(.horizontal, .spaceXs + 2)
            .padding(.vertical, 2)
            .foregroundStyle(isInk ? Color.paper : (isActive ? Color.accentActive : Color.ink))
            .background(isInk ? Color.ink : Color.paper)
            .overlay(Rectangle().stroke(isActive ? Color.accentActive : Color.ink, lineWidth: .hairline))
    }
}

/// A hairline-bordered block that groups one dialogue, note, reading or session.
struct PanelModifier: ViewModifier {
    func body(content: Content) -> some View {
        content
            .padding(.spaceMd)
            .frame(maxWidth: .infinity, alignment: .leading)
            .overlay(Rectangle().stroke(Color.ink, lineWidth: .hairline))
    }
}

/// A paper box with an ink outline and a solid ink block offset down and to the right behind it.
struct ShadowedBoxModifier: ViewModifier {
    func body(content: Content) -> some View {
        content
            .background(Color.paper)
            .overlay(Rectangle().stroke(Color.ink, lineWidth: .hairline))
            .background(
                Rectangle()
                    .fill(Color.ink)
                    .offset(x: .boxShadowOffset, y: .boxShadowOffset),
            )
    }
}

extension View {
    func panel() -> some View {
        modifier(PanelModifier())
    }

    func shadowedBox() -> some View {
        modifier(ShadowedBoxModifier())
    }

    /// Keeps prose to a readable measure on wide screens, centred.
    func readableWidth() -> some View {
        frame(maxWidth: .measure, alignment: .leading)
            .frame(maxWidth: .infinity)
    }
}

/// A square hairline progress bar, the web's `<progress>`.
struct SquareProgressBar: View {
    let value: Int
    let total: Int

    var body: some View {
        GeometryReader { geometry in
            ZStack(alignment: .leading) {
                Rectangle().fill(Color.paper)
                Rectangle()
                    .fill(Color.ink)
                    .frame(width: geometry.size.width * proportion())
            }
            .overlay(Rectangle().stroke(Color.ink, lineWidth: .hairline))
        }
        .frame(height: 6)
    }

    private func proportion() -> CGFloat {
        total == 0 ? 0 : CGFloat(value) / CGFloat(total)
    }
}

/// Splits prose on blank lines into paragraphs, as the web does.
struct Paragraphs: View {
    let text: String

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceSm) {
            ForEach(Array(paragraphs().enumerated()), id: \.offset) { _, paragraph in
                Text(paragraph)
                    .font(.bodyText)
                    .lineSpacing(4)
                    .fixedSize(horizontal: false, vertical: true)
            }
        }
    }

    private func paragraphs() -> [String] {
        text.components(separatedBy: "\n\n")
            .map { paragraph in paragraph.trimmingCharacters(in: .whitespacesAndNewlines) }
            .filter { paragraph in !paragraph.isEmpty }
    }
}

/// Places its children left to right, wrapping onto new lines, for word tokens.
struct FlowLayout: Layout {
    var spacing: CGFloat = .spaceSm

    func sizeThatFits(proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) -> CGSize {
        let rows: [[CGSize]] = arrangeRows(proposal: proposal, subviews: subviews)
        let width: CGFloat = rows.map { row in row.map(\.width).reduce(0, +) + spacing * CGFloat(max(row.count - 1, 0)) }.max() ?? 0
        let height: CGFloat = rows.map { row in row.map(\.height).max() ?? 0 }.reduce(0, +) + spacing * CGFloat(max(rows.count - 1, 0))

        return CGSize(width: proposal.width ?? width, height: height)
    }

    func placeSubviews(in bounds: CGRect, proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) {
        var origin: CGPoint = bounds.origin
        var rowHeight: CGFloat = 0

        for subview in subviews {
            let size: CGSize = subview.sizeThatFits(.unspecified)

            if origin.x + size.width > bounds.maxX, origin.x > bounds.minX {
                origin.x = bounds.minX
                origin.y += rowHeight + spacing
                rowHeight = 0
            }

            subview.place(at: origin, proposal: ProposedViewSize(size))
            origin.x += size.width + spacing
            rowHeight = max(rowHeight, size.height)
        }
    }

    private func arrangeRows(proposal: ProposedViewSize, subviews: Subviews) -> [[CGSize]] {
        let maximumWidth: CGFloat = proposal.width ?? .infinity
        var rows: [[CGSize]] = [[]]
        var rowWidth: CGFloat = 0

        for subview in subviews {
            let size: CGSize = subview.sizeThatFits(.unspecified)

            if rowWidth + size.width > maximumWidth, !(rows.last?.isEmpty ?? true) {
                rows.append([])
                rowWidth = 0
            }

            rows[rows.count - 1].append(size)
            rowWidth += size.width + spacing
        }

        return rows
    }
}

/// A page title with an uppercase caption above it.
struct PageHeader: View {
    let caption: String?
    let title: String

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceXs) {
            if let caption {
                MicroLabel(text: caption)
            }

            Text(title)
                .font(.headingText)
                .foregroundStyle(Color.ink)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
    }
}

extension UnitStatusRecord: CaseIterable {
    public static let allCases: [UnitStatusRecord] = [.notStarted, .inProgress, .completed]

    func label() -> String {
        switch self {
        case .notStarted:
            return "Not started"
        case .inProgress:
            return "In progress"
        case .completed:
            return "Completed"
        }
    }

    func accessibilityName() -> String {
        switch self {
        case .notStarted:
            return "not-started"
        case .inProgress:
            return "in-progress"
        case .completed:
            return "completed"
        }
    }
}

/// A unit's status, with its best score once it has been practised.
struct UnitStatusTag: View {
    let unitProgress: UnitProgressRecord

    var body: some View {
        HStack(spacing: .spaceXs) {
            switch unitProgress.status {
            case .completed:
                TagView(text: UnitStatusRecord.completed.label(), isInk: true)

                if unitProgress.attemptCount > 0 {
                    percentText()
                }
            case .inProgress:
                TagView(text: UnitStatusRecord.inProgress.label(), isActive: true)

                if unitProgress.attemptCount > 0 {
                    percentText()
                }
            case .notStarted:
                TagView(text: UnitStatusRecord.notStarted.label())
            }
        }
    }

    private func percentText() -> some View {
        Text("\(Int((unitProgress.bestProportion * 100).rounded()))%")
            .font(.dataText)
            .foregroundStyle(Color.inkMuted)
    }
}

/// A plain ink-coloured toggle, a hairline square with a check, labelled like the web's checkboxes.
struct SquareToggleStyle: ToggleStyle {
    func makeBody(configuration: Configuration) -> some View {
        Button {
            configuration.isOn.toggle()
        } label: {
            HStack(spacing: .spaceSm) {
                ZStack {
                    Rectangle()
                        .stroke(Color.ink, lineWidth: .hairline)
                        .frame(width: 18, height: 18)

                    if configuration.isOn {
                        Image(systemName: "checkmark")
                            .font(.system(size: 11, weight: .bold))
                            .foregroundStyle(Color.ink)
                    }
                }

                configuration.label
            }
        }
        .buttonStyle(.plain)
    }
}

/// A row of square hairline segments, one per option, with the chosen one filled ink.
struct SquareSegmentedControl<Option: Hashable>: View {
    let options: [Option]
    let selection: Option
    let label: (Option) -> String
    let accessibilityIdentifier: (Option) -> String
    let onSelect: (Option) -> Void

    var body: some View {
        HStack(spacing: 0) {
            ForEach(options, id: \.self) { option in
                segment(option: option, isSelected: option == selection)
            }
        }
    }

    private func segment(option: Option, isSelected: Bool) -> some View {
        Button {
            onSelect(option)
        } label: {
            Text(label(option))
                .font(.captionText)
                .lineLimit(1)
                .minimumScaleFactor(0.8)
                .frame(maxWidth: .infinity)
                .padding(.horizontal, .spaceXs)
                .padding(.vertical, .spaceSm)
                .foregroundStyle(isSelected ? Color.paper : Color.ink)
                .background(isSelected ? Color.ink : Color.paper)
                .overlay(Rectangle().stroke(Color.ink, lineWidth: .hairline))
                .contentShape(Rectangle())
        }
        .buttonStyle(.plain)
        .accessibilityIdentifier(accessibilityIdentifier(option))
        .accessibilityAddTraits(isSelected ? .isSelected : [])
    }
}
