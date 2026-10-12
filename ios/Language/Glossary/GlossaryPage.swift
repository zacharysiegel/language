import SwiftUI

/// A vocabulary item with its unit and the key it is searched by.
struct GlossaryEntry: Identifiable {
    let id: String
    let unit: UnitOutlineRecord
    let item: VocabularyItemRecord
    let searchKeys: [String]
}

/// Consecutive shown entries of one level.
struct GlossaryLevelRun {
    let id: String
    let levelCode: String
    let entries: [GlossaryEntry]
    /// Set on the first run of each level.
    let indexRailEntryNumber: Int?
}

struct GlossaryPage: View {
    let outline: CourseOutlineRecord

    @State private var searchText: String = ""
    @State private var levelFilter: String?
    @State private var entries: [GlossaryEntry] = []
    @State private var indexedCourseId: String?
    @State private var indexRailTracker: IndexRailTracker = IndexRailTracker()

    var body: some View {
        let matching: [GlossaryEntry] = matchingEntries()
        let levelRuns: [GlossaryLevelRun] = Self.levelRuns(matching)
        let indexRailEntries: [IndexRailEntry] = Self.indexRailEntries(levelRuns)

        ScrollViewReader { scrollProxy in
            ScrollView {
                VStack(alignment: .leading, spacing: 0) {
                    PageHeader(caption: "\(matching.count) entries", title: "Glossary")
                        .padding(.bottom, .spaceMd)

                    levelPicker()
                        .padding(.bottom, .spaceMd)

                    HairlineRule()

                    ForEach(levelRuns, id: \.id) { levelRun in
                        levelRunView(levelRun)
                    }
                }
                .padding(.spaceMd)
                .readableWidth()
                .indexRailContent(indexRailTracker)
            }
            .indexRailScrollTracking(indexRailTracker)
            .onChange(of: indexRailEntries) {
                indexRailTracker.recordContentChange()
            }
            .overlay(alignment: .trailing) {
                IndexRail(
                    entries: indexRailEntries,
                    railAccessibilityLabel: "Level index",
                    railAccessibilityIdentifier: "glossary-level-index",
                    tracker: indexRailTracker,
                    scrollProxy: scrollProxy,
                )
            }
        }
        .searchable(text: $searchText, prompt: "Search \(outline.targetLanguageName) or \(outline.sourceLanguageName)")
        .navigationTitle("Glossary")
        .navigationBarTitleDisplayMode(.inline)
        .onAppear { indexIfNeeded() }
        .onChange(of: outline.id) { indexIfNeeded() }
    }

    @ViewBuilder
    private func levelRunView(_ levelRun: GlossaryLevelRun) -> some View {
        if let indexRailEntryNumber = levelRun.indexRailEntryNumber {
            levelRunRows(levelRun)
                .indexRailTarget(entryNumber: indexRailEntryNumber)
        } else {
            levelRunRows(levelRun)
        }
    }

    /* Each run is a stack of its own, so a run's top is known while its rows are not built. */
    private func levelRunRows(_ levelRun: GlossaryLevelRun) -> some View {
        LazyVStack(alignment: .leading, spacing: 0) {
            ForEach(levelRun.entries) { entry in
                GlossaryRow(courseId: outline.id, entry: entry)
                HairlineRule()
            }
        }
    }

    private func levelPicker() -> some View {
        Picker("Level", selection: $levelFilter) {
            Text("All levels").tag(String?.none)

            ForEach(outline.levels, id: \.code) { level in
                Text(level.code).tag(String?.some(level.code))
            }
        }
        .pickerStyle(.segmented)
    }

    private func matchingEntries() -> [GlossaryEntry] {
        let query: String = searchKey(text: searchText)

        return entries.filter { entry in
            let levelMatches: Bool = levelFilter.map { levelCode in entry.unit.levelCode == levelCode } ?? true
            let textMatches: Bool = query.isEmpty || entry.searchKeys.contains { key in key.contains(query) }

            return levelMatches && textMatches
        }
    }

    private static func levelRuns(_ shownEntries: [GlossaryEntry]) -> [GlossaryLevelRun] {
        var levelRuns: [GlossaryLevelRun] = []
        var indexedLevelCodes: Set<String> = []
        var runStart: Int = shownEntries.startIndex

        while runStart < shownEntries.endIndex {
            let levelCode: String = shownEntries[runStart].unit.levelCode
            let runEnd: Int = shownEntries[runStart...].firstIndex { entry in entry.unit.levelCode != levelCode } ?? shownEntries.endIndex
            let isFirstRunOfItsLevel: Bool = indexedLevelCodes.insert(levelCode).inserted

            levelRuns.append(GlossaryLevelRun(
                id: shownEntries[runStart].id,
                levelCode: levelCode,
                entries: Array(shownEntries[runStart..<runEnd]),
                indexRailEntryNumber: isFirstRunOfItsLevel ? indexedLevelCodes.count : nil,
            ))

            runStart = runEnd
        }

        return levelRuns
    }

    private static func indexRailEntries(_ levelRuns: [GlossaryLevelRun]) -> [IndexRailEntry] {
        levelRuns.filter { levelRun in levelRun.indexRailEntryNumber != nil }.map { levelRun in
            IndexRailEntry(visibleText: levelRun.levelCode, accessibilityLabel: "Go to level \(levelRun.levelCode)")
        }
    }

    private func indexIfNeeded() {
        guard indexedCourseId != outline.id else {
            return
        }

        entries = outline.units.flatMap { unit in
            unit.vocabulary.map { item in
                GlossaryEntry(id: item.cardId, unit: unit, item: item, searchKeys: [searchKey(text: item.target), searchKey(text: item.source)])
            }
        }
        indexedCourseId = outline.id
    }
}

struct GlossaryRow: View {
    let courseId: String
    let entry: GlossaryEntry

    var body: some View {
        HStack(alignment: .top, spacing: .spaceSm) {
            PlayButton(clip: AudioClip(clipKey: entry.item.clipKey, text: entry.item.target))

            VStack(alignment: .leading, spacing: 2) {
                Text(entry.item.target).font(.targetText)
                Text(entry.item.source).font(.bodyText).foregroundStyle(Color.inkMuted)

                if let literal = entry.item.sourceLiteral {
                    Text("literally: \(literal)").font(.captionText).italic().foregroundStyle(Color.inkMuted)
                }
            }
            .fixedSize(horizontal: false, vertical: true)

            Spacer(minLength: .spaceSm)

            NavigationLink(value: UnitDestination(courseId: courseId, unitId: entry.unit.id, tab: .vocabulary)) {
                Text("\(entry.unit.levelCode) \(entry.unit.order)").font(.dataText).foregroundStyle(Color.accentLink)
            }
        }
        .padding(.vertical, .spaceSm)
    }
}
