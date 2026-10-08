import SwiftUI

/// A vocabulary item with its unit and the key it is searched by.
struct GlossaryEntry: Identifiable {
    let id: String
    let unit: UnitOutlineRecord
    let item: VocabularyItemRecord
    let searchKeys: [String]
}

struct GlossaryPage: View {
    @Environment(CourseStore.self) private var courseStore
    @State private var searchText: String = ""
    @State private var levelFilter: LevelRecord?
    @State private var entries: [GlossaryEntry] = []
    @State private var indexedVersion: String?

    var body: some View {
        let matching: [GlossaryEntry] = matchingEntries()

        ScrollView {
            LazyVStack(alignment: .leading, spacing: 0, pinnedViews: []) {
                PageHeader(caption: "\(matching.count) entries", title: "Glossary")
                    .padding(.bottom, .spaceMd)

                levelPicker()
                    .padding(.bottom, .spaceMd)

                HairlineRule()

                ForEach(matching) { entry in
                    GlossaryRow(entry: entry)
                    HairlineRule()
                }
            }
            .padding(.spaceMd)
            .readableWidth()
        }
        .searchable(text: $searchText, prompt: "Search Portuguese or English")
        .navigationTitle("Glossary")
        .navigationBarTitleDisplayMode(.inline)
        .onAppear { indexIfNeeded() }
        .onChange(of: courseStore.shownVersion()?.version()) { indexIfNeeded() }
    }

    private func levelPicker() -> some View {
        Picker("Level", selection: $levelFilter) {
            Text("All levels").tag(LevelRecord?.none)

            ForEach(courseStore.outline()?.levels ?? [], id: \.code) { level in
                Text(level.code).tag(LevelRecord?.some(level.level))
            }
        }
        .pickerStyle(.segmented)
    }

    private func matchingEntries() -> [GlossaryEntry] {
        let query: String = searchKey(text: searchText)

        return entries.filter { entry in
            let levelMatches: Bool = levelFilter.map { level in entry.unit.level == level } ?? true
            let textMatches: Bool = query.isEmpty || entry.searchKeys.contains { key in key.contains(query) }

            return levelMatches && textMatches
        }
    }

    private func indexIfNeeded() {
        guard let version = courseStore.shownVersion(), let outline = courseStore.outline(), indexedVersion != version.version() else {
            return
        }

        entries = outline.units.flatMap { unit in
            unit.vocabulary.map { item in
                GlossaryEntry(id: item.cardId, unit: unit, item: item, searchKeys: [searchKey(text: item.pt), searchKey(text: item.en)])
            }
        }
        indexedVersion = version.version()
    }
}

struct GlossaryRow: View {
    let entry: GlossaryEntry

    var body: some View {
        HStack(alignment: .top, spacing: .spaceSm) {
            PlayButton(clip: AudioClip(clipKey: entry.item.clipKey, text: entry.item.pt))

            VStack(alignment: .leading, spacing: 2) {
                Text(entry.item.pt).font(.portugueseText)
                Text(entry.item.en).font(.bodyText).foregroundStyle(Color.inkMuted)

                if let literal = entry.item.literal {
                    Text("literally: \(literal)").font(.captionText).italic().foregroundStyle(Color.inkMuted)
                }
            }
            .fixedSize(horizontal: false, vertical: true)

            Spacer(minLength: .spaceSm)

            NavigationLink(value: UnitDestination(unitId: entry.unit.id, tab: .vocabulary)) {
                Text("\(entry.unit.levelCode) \(entry.unit.order)").font(.dataText).foregroundStyle(Color.accentLink)
            }
        }
        .padding(.vertical, .spaceSm)
    }
}
