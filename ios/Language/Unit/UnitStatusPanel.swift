import SwiftUI

/// The learner's own control over a unit's status.
struct UnitStatusPanel: View {
    static let helpAutoCloseDuration: Duration = .seconds(8)

    let courseId: String
    let unitId: String
    @Binding var isHelpShown: Bool

    @Environment(ProgressStore.self) private var progressStore

    var body: some View {
        let unitProgress: UnitProgressRecord = progressStore.unitProgress(courseId: courseId, unitId: unitId)
        let status: UnitStatusRecord = unitProgress.status

        VStack(alignment: .leading, spacing: .spaceSm) {
            HStack(spacing: .spaceSm) {
                MicroLabel(text: "Status")

                Button {
                    isHelpShown.toggle()
                } label: {
                    Text("?")
                        .font(.bodyText)
                        .foregroundStyle(isHelpShown ? Color.accentHover : Color.ink)
                        .frame(width: .buttonIconSize, height: .buttonIconSize)
                        .overlay(Rectangle().stroke(isHelpShown ? Color.accentHover : Color.ink, lineWidth: .hairline))
                        .contentShape(Rectangle())
                }
                .buttonStyle(.plain)
                .accessibilityLabel("How a unit's status changes")
                .accessibilityIdentifier("unit-status-help")
                .floatingTipAnchor()

                if unitProgress.attemptCount > 0 {
                    Spacer()

                    Text("Best score \(Int((unitProgress.bestProportion * 100).rounded()))%")
                        .font(.dataText)
                        .foregroundStyle(Color.inkMuted)
                        .accessibilityIdentifier("unit-best-score")
                }
            }

            SquareSegmentedControl(
                options: UnitStatusRecord.allCases,
                selection: status,
                label: { option in option.label() },
                accessibilityIdentifier: { option in "unit-status-\(option.accessibilityName())" },
                onSelect: { option in progressStore.setUnitStatus(courseId: courseId, unitId: unitId, status: option) },
            )
        }
    }
}

/// The rules by which a unit's status changes without the learner setting it.
struct UnitStatusHelpTip: View {
    let onClose: () -> Void

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceXs) {
            ForEach(Array(rules().enumerated()), id: \.offset) { index, rule in
                numberedRule(number: index + 1, rule: rule)
            }
        }
        .font(.captionText)
        .foregroundStyle(Color.ink)
        .padding(.spaceMd)
        .padding(.trailing, .buttonIconSize)
        .overlay(alignment: .topTrailing) {
            closeButton()
        }
        .shadowedBox()
        .accessibilityElement(children: .contain)
        .accessibilityIdentifier("unit-status-help-tip")
    }

    private func rules() -> [String] {
        [
            "Starting a practice also starts the unit.",
            "Passing the full practice with at least \(passPercent())% completes it.",
        ]
    }

    private func numberedRule(number: Int, rule: String) -> some View {
        HStack(alignment: .firstTextBaseline, spacing: .spaceXs) {
            Text("\(number).")
                .monospacedDigit()

            Text(rule)
                .fixedSize(horizontal: false, vertical: true)
        }
    }

    private func closeButton() -> some View {
        Button {
            onClose()
        } label: {
            Text("×")
                .font(.titleText)
                .foregroundStyle(Color.ink)
                .frame(width: .buttonIconSize, height: .buttonIconSize)
                .contentShape(Rectangle())
        }
        .buttonStyle(.plain)
        .accessibilityLabel("Close")
        .accessibilityIdentifier("unit-status-help-close")
    }
}
