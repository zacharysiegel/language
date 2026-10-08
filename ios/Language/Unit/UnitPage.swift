import SwiftUI

enum UnitTabKind: String, CaseIterable, Hashable {
    case overview
    case grammar
    case dialogues
    case readings
    case vocabulary
    case practice

    func label() -> String {
        switch self {
        case .overview:
            return "Overview"
        case .grammar:
            return "Principles"
        case .dialogues:
            return "Dialogues"
        case .readings:
            return "Reading"
        case .vocabulary:
            return "Vocabulary"
        case .practice:
            return "Practice"
        }
    }

    func isAvailable(for lesson: UnitLessonRecord) -> Bool {
        self != .readings || !lesson.readings.isEmpty
    }
}

struct UnitPage: View {
    let courseId: String
    let unitId: String
    let initialTab: UnitTabKind

    @Environment(CourseStore.self) private var courseStore
    @Environment(AudioPlayer.self) private var audioPlayer
    @State private var activeTab: UnitTabKind?

    var body: some View {
        content()
            .navigationBarTitleDisplayMode(.inline)
            .onAppear { ensureLoaded() }
            .onChange(of: courseStore.shownVersion()?.version()) { ensureLoaded() }
            .onDisappear { audioPlayer.stop() }
    }

    private func ensureLoaded() {
        courseStore.ensureOutlineLoaded(courseId: courseId)
        courseStore.ensureUnitLoaded(courseId: courseId, unitId: unitId)
    }

    @ViewBuilder
    private func content() -> some View {
        switch courseStore.unitState(courseId: courseId, unitId: unitId) {
        case .loaded(let unit, let lesson):
            loadedView(unit: unit, lesson: lesson)
        case .failed(let message):
            Text("This unit could not be loaded. (\(message))")
                .font(.bodyText)
                .foregroundStyle(Color.accentActive)
                .padding(.spaceMd)
        case .none:
            Text("Loading the unit…")
                .font(.bodyText)
                .foregroundStyle(Color.inkMuted)
                .frame(maxWidth: .infinity, maxHeight: .infinity)
                .onChange(of: courseStore.outline(courseId: courseId)?.id) { ensureLoaded() }
        case .loading:
            Text("Loading the unit…")
                .font(.bodyText)
                .foregroundStyle(Color.inkMuted)
                .frame(maxWidth: .infinity, maxHeight: .infinity)
        }
    }

    private func loadedView(unit: UnitOutlineRecord, lesson: UnitLessonRecord) -> some View {
        let tab: UnitTabKind = activeTab ?? initialTab

        return ScrollView {
            VStack(alignment: .leading, spacing: .spaceMd) {
                PageHeader(caption: "\(unit.levelCode) · \(unit.order)", title: unit.title)
                UnitStatusPanel(courseId: courseId, unitId: unit.id)
                tabRow(lesson: lesson, active: tab)
                section(tab: tab, unit: unit, lesson: lesson)
                unitNavigation(unit: unit)
            }
            .padding(.spaceMd)
            .readableWidth()
            .id(tab)
        }
        .navigationTitle(unit.title)
    }

    private func tabRow(lesson: UnitLessonRecord, active: UnitTabKind) -> some View {
        ScrollView(.horizontal, showsIndicators: false) {
            HStack(spacing: 0) {
                ForEach(UnitTabKind.allCases.filter { tab in tab.isAvailable(for: lesson) }, id: \.self) { tab in
                    Button {
                        audioPlayer.stop()
                        activeTab = tab
                    } label: {
                        Text(tab.label().uppercased())
                            .font(.labelText)
                            .tracking(0.6)
                            .padding(.horizontal, .spaceSm + .spaceXs)
                            .padding(.vertical, .spaceSm)
                            .foregroundStyle(tab == active ? Color.accentActive : Color.ink)
                            .overlay(alignment: .bottom) {
                                Rectangle()
                                    .fill(tab == active ? Color.accentActive : Color.rule)
                                    .frame(height: tab == active ? 2 : .hairline)
                            }
                    }
                    .buttonStyle(.plain)
                    .accessibilityIdentifier("tab-\(tab.rawValue)")
                }
            }
        }
    }

    @ViewBuilder
    private func section(tab: UnitTabKind, unit: UnitOutlineRecord, lesson: UnitLessonRecord) -> some View {
        switch tab {
        case .overview:
            OverviewSection(courseId: courseId, unit: unit, lesson: lesson)
        case .grammar:
            GrammarSection(notes: lesson.grammar)
        case .dialogues:
            DialoguesSection(dialogues: lesson.dialogues)
        case .readings:
            ReadingsSection(readings: lesson.readings)
        case .vocabulary:
            VocabularySection(items: unit.vocabulary)
        case .practice:
            PracticeSection(courseId: courseId, unit: unit, lesson: lesson)
        }
    }

    @ViewBuilder
    private func unitNavigation(unit: UnitOutlineRecord) -> some View {
        if let units = courseStore.outline(courseId: courseId)?.units, let position = units.firstIndex(where: { candidate in candidate.id == unit.id }) {
            HStack(alignment: .top) {
                if position > 0 {
                    NavigationLink(value: UnitDestination(courseId: courseId, unitId: units[position - 1].id, tab: .overview)) {
                        Text("← \(units[position - 1].title)").font(.captionText).foregroundStyle(Color.accentLink)
                    }
                }

                Spacer()

                if position + 1 < units.count {
                    NavigationLink(value: UnitDestination(courseId: courseId, unitId: units[position + 1].id, tab: .overview)) {
                        Text("\(units[position + 1].title) →").font(.captionText).foregroundStyle(Color.accentLink)
                    }
                    .accessibilityIdentifier("next-unit")
                }
            }
            .padding(.top, .spaceLg)
        }
    }
}
