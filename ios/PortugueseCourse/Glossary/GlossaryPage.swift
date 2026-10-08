import SwiftUI

/// A vocabulary item with its unit and the key it is searched by.
struct GlossaryEntry: Identifiable {
    let id: String
    let unit: UnitOutlineRecord
    let item: VocabularyItemRecord
    let searchKeys: [String]
}

struct GlossaryPage: View {
    let outline: CourseOutlineRecord

    @State private var searchText: String = ""
    @State private var levelFilter: String?
    @State private var entries: [GlossaryEntry] = []
    @State private var indexedCourseId: String?

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
                    GlossaryRow(courseId: outline.id, entry: entry)
                    HairlineRule()
                }
            }
            .padding(.spaceMd)
            .readableWidth()
        }
        .searchable(text: $searchText, prompt: "Search \(outline.targetLanguageName) or \(outline.sourceLanguageName)")
        .navigationTitle("Glossary")
        .navigationBarTitleDisplayMode(.inline)
        .onAppear { indexIfNeeded() }
        .onChange(of: outline.id) { indexIfNeeded() }
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
