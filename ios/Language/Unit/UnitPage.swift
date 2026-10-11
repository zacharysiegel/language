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

    /// What one box of the tab is called, for a tab made of a series of boxes.
    func indexedItemNoun() -> String? {
        switch self {
        case .grammar:
            return "note"
        case .dialogues:
            return "dialogue"
        case .readings:
            return "reading"
        case .overview, .vocabulary, .practice:
            return nil
        }
    }

    func indexedItemCount(in lesson: UnitLessonRecord) -> Int {
        switch self {
        case .grammar:
            return lesson.grammar.count
        case .dialogues:
            return lesson.dialogues.count
        case .readings:
            return lesson.readings.count
        case .overview, .vocabulary, .practice:
            return 0
        }
    }

    func record() -> UnitTabRecord {
        switch self {
        case .overview:
            return .overview
        case .grammar:
            return .grammar
        case .dialogues:
            return .dialogues
        case .readings:
            return .readings
        case .vocabulary:
            return .vocabulary
        case .practice:
            return .practice
        }
    }
}

/// What a shown tab's recording prefetch depends on; a change of any restarts it.
private struct RecordingPrefetchRequest: Equatable {
    let tab: UnitTabKind
    let engine: AudioEngineRecord
    let contentKey: String?
}

struct UnitPage: View {
    let courseId: String
    let unitId: String
    let initialTab: UnitTabKind

    @Environment(CourseStore.self) private var courseStore
    @Environment(AudioPlayer.self) private var audioPlayer
    @Environment(ProgressStore.self) private var progressStore
    @State private var activeTab: UnitTabKind?
    @State private var isStatusHelpShown: Bool = false
    @State private var itemIndexTracker: UnitItemIndexTracker = UnitItemIndexTracker()
    @State private var pageScrollPhase: ScrollPhase = .idle
    @State private var pageScrollMomentumStopper: ScrollMomentumStopper = ScrollMomentumStopper()

    var body: some View {
        content()
            .navigationBarTitleDisplayMode(.inline)
            .onAppear { ensureLoaded() }
            .onChange(of: courseStore.shownContentKey(courseId: courseId)) { ensureLoaded() }
            .onDisappear {
                audioPlayer.stop()
                closeStatusHelp()
            }
    }

    private func closeStatusHelp() {
        isStatusHelpShown = false
    }

    private func ensureLoaded() {
        courseStore.ensurePublicationLoaded(courseId: courseId)
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
        case .none, .loading:
            PendingStatusMessage(text: "Loading the unit…")
                .onChange(of: courseStore.outline(courseId: courseId)?.id) { ensureLoaded() }
        }
    }

    private func loadedView(unit: UnitOutlineRecord, lesson: UnitLessonRecord) -> some View {
        let tab: UnitTabKind = activeTab ?? initialTab
        let prefetchRequest: RecordingPrefetchRequest = RecordingPrefetchRequest(
            tab: tab,
            engine: progressStore.settings().audioEngine,
            contentKey: courseStore.shownContentKey(courseId: courseId),
        )

        let itemLabels: [String] = Self.indexedItemLabels(tab: tab, unit: unit, lesson: lesson)
        let itemNoun: String? = itemLabels.count >= UnitItemIndex.minimumItemCount ? tab.indexedItemNoun() : nil

        return ScrollViewReader { scrollProxy in
            ScrollView {
                VStack(alignment: .leading, spacing: .spaceMd) {
                    PageHeader(caption: "\(unit.levelCode) · \(unit.order)", title: unit.title)
                    UnitStatusPanel(courseId: courseId, unitId: unit.id, isHelpShown: $isStatusHelpShown)
                    tabRow(lesson: lesson, active: tab)
                    section(tab: tab, unit: unit, lesson: lesson)
                    tabNavigation(lesson: lesson, active: tab)
                }
                .padding(.spaceMd)
                .readableWidth()
                .id(tab)
                .coordinateSpace(.named(UnitItemIndex.contentCoordinateSpaceName))
                .scrollMomentumStopper(pageScrollMomentumStopper)
            }
            .onPreferenceChange(UnitItemContentTopsPreferenceKey.self) { contentTops in
                itemIndexTracker.recordItemContentTops(contentTops)
            }
            .onScrollGeometryChange(for: CGFloat.self) { geometry in
                geometry.visibleRect.minY + geometry.contentInsets.top
            } action: { _, visibleContentTop in
                itemIndexTracker.recordVisibleContentTop(visibleContentTop)
            }
            .onScrollPhaseChange { _, newPhase in
                recordPageScrollPhase(newPhase)
            }
            .onChange(of: tab) {
                itemIndexTracker.recordTabChange()
            }
            .overlay(alignment: .trailing) {
                if let itemNoun {
                    UnitItemIndexRail(
                        itemLabels: itemLabels,
                        itemNoun: itemNoun,
                        currentItemNumber: itemIndexTracker.currentItemNumber(),
                        isPageScrolling: pageScrollPhase.isScrolling,
                        onJump: { itemNumber in jumpToItem(itemNumber, scrollProxy: scrollProxy) },
                    )
                }
            }
        }
        .floatingTip(isShown: isStatusHelpShown, autoCloseDuration: UnitStatusPanel.helpAutoCloseDuration, onDismiss: closeStatusHelp) {
            UnitStatusHelpTip(onClose: closeStatusHelp)
        }
        .navigationTitle(unit.title)
        .task(id: prefetchRequest, priority: .utility) {
            await prefetchRecordings(prefetchRequest, unitId: unit.id)
        }
    }

    private static func indexedItemLabels(tab: UnitTabKind, unit: UnitOutlineRecord, lesson: UnitLessonRecord) -> [String] {
        let itemCount: Int = tab.indexedItemCount(in: lesson)

        return (0..<itemCount).map { itemIndex in unit.itemLabel(itemNumber: itemIndex + 1) }
    }

    private func recordPageScrollPhase(_ phase: ScrollPhase) {
        pageScrollPhase = phase

        if phase == .interacting {
            itemIndexTracker.recordLearnerScroll()
        }
    }

    private func jumpToItem(_ itemNumber: Int, scrollProxy: ScrollViewProxy) {
        pageScrollMomentumStopper.stopMomentum()
        itemIndexTracker.recordJump(itemNumber: itemNumber)
        scrollProxy.scrollTo(UnitItemAnchor(itemNumber: itemNumber), anchor: .top)
    }

    private func prefetchRecordings(_ request: RecordingPrefetchRequest, unitId: String) async {
        guard let publication = courseStore.shownPublication(courseId: courseId) else {
            return
        }

        await RecordingPrefetch.prefetchTab(request.tab, unitId: unitId, publication: publication, engine: request.engine)
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
        .accessibilityIdentifier("unit-tabs")
    }

    @ViewBuilder
    private func section(tab: UnitTabKind, unit: UnitOutlineRecord, lesson: UnitLessonRecord) -> some View {
        switch tab {
        case .overview:
            OverviewSection(courseId: courseId, unit: unit, lesson: lesson)
        case .grammar:
            GrammarSection(unit: unit, notes: lesson.grammar)
        case .dialogues:
            DialoguesSection(unit: unit, dialogues: lesson.dialogues)
        case .readings:
            ReadingsSection(unit: unit, readings: lesson.readings)
        case .vocabulary:
            VocabularySection(items: unit.vocabulary)
        case .practice:
            PracticeSection(courseId: courseId, unit: unit, lesson: lesson)
        }
    }

    @ViewBuilder
    private func tabNavigation(lesson: UnitLessonRecord, active: UnitTabKind) -> some View {
        let availableTabs: [UnitTabKind] = UnitTabKind.allCases.filter { tab in tab.isAvailable(for: lesson) }

        if let position = availableTabs.firstIndex(of: active) {
            HStack(alignment: .top) {
                if position > 0 {
                    tabStepButton(availableTabs[position - 1], label: "← \(availableTabs[position - 1].label())")
                }

                Spacer()

                if position + 1 < availableTabs.count {
                    tabStepButton(availableTabs[position + 1], label: "\(availableTabs[position + 1].label()) →")
                        .accessibilityIdentifier("next-tab")
                }
            }
            .padding(.top, .spaceLg)
        }
    }

    private func tabStepButton(_ tab: UnitTabKind, label: String) -> some View {
        Button {
            audioPlayer.stop()
            activeTab = tab
        } label: {
            Text(label).font(.captionText).foregroundStyle(Color.accentLink)
        }
        .buttonStyle(.plain)
    }

}
