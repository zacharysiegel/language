import SwiftUI

/// The learner's own control over a unit's status.
struct UnitStatusPanel: View {
    let courseId: String
    let unitId: String

    @Environment(ProgressStore.self) private var progressStore

    var body: some View {
        let status: UnitStatusRecord = progressStore.unitProgress(courseId: courseId, unitId: unitId).status

        VStack(alignment: .leading, spacing: .spaceSm) {
            if status == .notStarted {
                Button("Start this unit") { progressStore.startUnit(courseId: courseId, unitId: unitId) }
                    .buttonStyle(SquareButtonStyle(isPrimary: true))
                    .accessibilityIdentifier("start-unit")
            }

            MicroLabel(text: "Status")

            SquareSegmentedControl(
                options: UnitStatusRecord.allCases,
                selection: status,
                label: { option in option.label() },
                accessibilityIdentifier: { option in "unit-status-\(option.accessibilityName())" },
                onSelect: { option in progressStore.setUnitStatus(courseId: courseId, unitId: unitId, status: option) },
            )

            Text("Starting a practice also starts the unit. Passing the full practice with at least \(passPercent())% completes it.")
                .font(.captionText)
                .foregroundStyle(Color.inkMuted)
                .fixedSize(horizontal: false, vertical: true)
        }
    }
}
