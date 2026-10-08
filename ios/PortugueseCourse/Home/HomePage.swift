import SwiftUI

struct HomePage: View {
    @Environment(CourseStore.self) private var courseStore
    @Environment(ProgressStore.self) private var progressStore

    var body: some View {
        if let outline = courseStore.outline() {
            ScrollView {
                VStack(alignment: .leading, spacing: .spaceLg) {
                    header(outline)

                    ForEach(outline.levels, id: \.code) { level in
                        levelSection(level, units: outline.units.filter { unit in unit.level == level.level })
                    }
                }
                .padding(.spaceMd)
                .readableWidth()
            }
            .refreshable { await courseStore.refresh() }
            .navigationTitle("Course")
            .navigationBarTitleDisplayMode(.inline)
        }
    }

    private func header(_ outline: CourseOutlineRecord) -> some View {
        VStack(alignment: .leading, spacing: .spaceSm) {
            PageHeader(caption: "\(outline.units.count) units", title: outline.title)

            HStack {
                Text("\(progressStore.completedUnitCount()) / \(outline.units.count) units completed")
                    .font(.dataText)
                    .foregroundStyle(Color.inkMuted)

                Spacer()

                if let nextUnit = nextIncompleteUnit(outline) {
                    NavigationLink(value: UnitDestination(unitId: nextUnit.id, tab: .overview)) {
                        Text("Continue")
                    }
                    .buttonStyle(SquareButtonStyle(isPrimary: true))
                    .accessibilityIdentifier("continue")
                }
            }
        }
    }

    private func nextIncompleteUnit(_ outline: CourseOutlineRecord) -> UnitOutlineRecord? {
        outline.units.first { unit in !progressStore.unitProgress(unitId: unit.id).completed }
    }

    private func levelSection(_ level: LevelSummaryRecord, units: [UnitOutlineRecord]) -> some View {
        VStack(alignment: .leading, spacing: 0) {
            Text("\(level.code) · \(level.title)")
                .font(.titleText)
                .padding(.bottom, .spaceSm)

            HairlineRule()

            ForEach(units, id: \.id) { unit in
                NavigationLink(value: UnitDestination(unitId: unit.id, tab: .overview)) {
                    unitRow(unit)
                }
                .buttonStyle(.plain)
                .accessibilityIdentifier("unit-\(unit.id)")

                HairlineRule()
            }
        }
    }

    private func unitRow(_ unit: UnitOutlineRecord) -> some View {
        HStack(alignment: .firstTextBaseline, spacing: .spaceSm) {
            Text("\(unit.order)")
                .font(.dataText)
                .foregroundStyle(Color.inkMuted)
                .frame(width: 28, alignment: .trailing)

            VStack(alignment: .leading, spacing: .spaceXs) {
                Text(unit.title)
                    .font(.bodyText)
                    .foregroundStyle(Color.accentLink)
                    .multilineTextAlignment(.leading)

                UnitStatusTag(unitProgress: progressStore.unitProgress(unitId: unit.id))
            }

            Spacer()

            Image(systemName: "chevron.right")
                .font(.system(size: 11))
                .foregroundStyle(Color.rule)
        }
        .padding(.vertical, .spaceSm + .spaceXs)
        .contentShape(Rectangle())
    }
}
