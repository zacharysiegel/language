import SwiftUI

struct ProgressPage: View {
    let outline: CourseOutlineRecord

    @Environment(ProgressStore.self) private var progressStore

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: .spaceLg) {
                summary(outline)
                unitsSection(outline)
                deckSection()
                ProgressTransfer()
            }
            .padding(.spaceMd)
            .readableWidth()
        }
        .navigationTitle("Progress")
        .navigationBarTitleDisplayMode(.inline)
    }

    private func summary(_ outline: CourseOutlineRecord) -> some View {
        VStack(alignment: .leading, spacing: .spaceXs) {
            PageHeader(caption: nil, title: "Progress")
            Text("\(progressStore.completedUnitCount(courseId: outline.id)) / \(outline.units.count) units completed").font(.dataText).foregroundStyle(Color.inkMuted)
            Text("Study streak \(progressStore.streakDays()) day(s)").font(.dataText).foregroundStyle(Color.inkMuted)
        }
    }

    private func unitsSection(_ outline: CourseOutlineRecord) -> some View {
        VStack(alignment: .leading, spacing: 0) {
            Text("Units").font(.titleText).padding(.bottom, .spaceSm)
            HairlineRule()

            ForEach(outline.units, id: \.id) { unit in
                NavigationLink(value: UnitDestination(courseId: outline.id, unitId: unit.id, tab: .overview)) {
                    HStack {
                        Text("\(unit.levelCode) \(unit.order): \(unit.title)")
                            .font(.bodyText)
                            .foregroundStyle(Color.accentLink)
                            .multilineTextAlignment(.leading)

                        Spacer()

                        UnitStatusTag(unitProgress: progressStore.unitProgress(courseId: outline.id, unitId: unit.id))
                    }
                    .padding(.vertical, .spaceSm)
                }
                .buttonStyle(.plain)

                HairlineRule()
            }
        }
    }

    private func deckSection() -> some View {
        let intervals: [UInt32] = reviewIntervalDays()
        let counts: [Int] = progressStore.countByBox(courseId: outline.id)

        return VStack(alignment: .leading, spacing: 0) {
            Text("Review deck").font(.titleText).padding(.bottom, .spaceSm)
            HairlineRule()

            ForEach(Array(zip(intervals, counts).enumerated()), id: \.offset) { _, entry in
                HStack {
                    Text(entry.0 == 0 ? "New or forgotten" : "Next review after \(entry.0) day(s)").font(.bodyText)
                    Spacer()
                    Text("\(entry.1)").font(.dataText)
                }
                .padding(.vertical, .spaceSm)

                HairlineRule()
            }
        }
    }
}

/// Copies progress out as text, restores it from text, or erases it.
struct ProgressTransfer: View {
    @Environment(ProgressStore.self) private var progressStore
    @State private var transferText: String = ""
    @State private var message: String?
    @State private var resetRequested: Bool = false

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceSm) {
            Text("Back up or move your progress").font(.titleText)
            Text("Progress is saved on this device only. Export it to keep a backup, or paste a backup to restore it.")
                .font(.captionText)
                .foregroundStyle(Color.inkMuted)
                .fixedSize(horizontal: false, vertical: true)

            HStack(spacing: .spaceSm) {
                Button("Export") {
                    transferText = progressStore.exportJson() ?? ""
                    UIPasteboard.general.string = transferText
                    message = "Progress exported and copied."
                }
                .buttonStyle(SquareButtonStyle(isCompact: true))

                Button("Import from the text box") {
                    let imported: Bool = progressStore.importJson(transferText)
                    message = imported ? "Progress imported." : "Could not import: \(progressStore.lastError() ?? "unreadable text")"
                }
                .buttonStyle(SquareButtonStyle(isCompact: true))
            }

            TextEditor(text: $transferText)
                .font(.system(size: 11, design: .monospaced))
                .frame(height: 140)
                .scrollContentBackground(.hidden)
                .overlay(Rectangle().stroke(Color.ink, lineWidth: .hairline))
                .accessibilityIdentifier("transfer-text")

            if let message {
                Text(message).font(.captionText).foregroundStyle(Color.inkMuted)
            }

            Text("Reset").font(.titleText).padding(.top, .spaceMd)

            Button(resetRequested ? "Tap again to erase all progress" : "Reset all progress") {
                guard resetRequested else {
                    resetRequested = true
                    return
                }

                progressStore.reset()
                resetRequested = false
                message = "All progress was reset."
            }
            .buttonStyle(SquareButtonStyle())
        }
    }
}
