/// Which divisions of the glossary its index rail lists.
enum GlossaryRailKind {
    case levels
    case units

    static func forLevelFilter(_ levelFilter: String?) -> GlossaryRailKind {
        switch levelFilter {
        case .none:
            .levels
        case .some:
            .units
        }
    }

    /// Entries with the same key belong to the same rail entry.
    func railKey(_ entry: GlossaryEntry) -> String {
        switch self {
        case .levels:
            entry.unit.levelCode
        case .units:
            entry.unit.id
        }
    }

    func railEntry(_ entry: GlossaryEntry) -> IndexRailEntry {
        switch self {
        case .levels:
            IndexRailEntry(visibleText: entry.unit.levelCode, accessibilityLabel: "Go to level \(entry.unit.levelCode)")
        case .units:
            IndexRailEntry(visibleText: entry.unitNumberText(), accessibilityLabel: "Go to unit \(entry.unitLabel())")
        }
    }

    func railAccessibilityLabel() -> String {
        switch self {
        case .levels:
            "Level index"
        case .units:
            "Unit index"
        }
    }

    func railAccessibilityIdentifier() -> String {
        switch self {
        case .levels:
            "glossary-level-index"
        case .units:
            "glossary-unit-index"
        }
    }
}

/// Consecutive shown entries of one rail entry.
struct GlossaryRun {
    let id: String
    let entries: [GlossaryEntry]
    /// Set on the first run of each rail entry.
    let indexRailEntryNumber: Int?
}

/// The glossary's shown entries in runs, with the index rail entries that jump to them.
struct GlossaryRail {
    let runs: [GlossaryRun]
    /// In order of their first shown entry.
    let entries: [IndexRailEntry]

    static func dividing(_ shownEntries: [GlossaryEntry], by kind: GlossaryRailKind) -> GlossaryRail {
        var runs: [GlossaryRun] = []
        var railEntries: [IndexRailEntry] = []
        var listedRailKeys: Set<String> = []
        var runStart: Int = shownEntries.startIndex

        while runStart < shownEntries.endIndex {
            let firstEntry: GlossaryEntry = shownEntries[runStart]
            let railKey: String = kind.railKey(firstEntry)
            let runEnd: Int = shownEntries[runStart...].firstIndex { entry in kind.railKey(entry) != railKey } ?? shownEntries.endIndex
            let startsItsRailEntry: Bool = listedRailKeys.insert(railKey).inserted

            if startsItsRailEntry {
                railEntries.append(kind.railEntry(firstEntry))
            }

            runs.append(GlossaryRun(
                id: firstEntry.id,
                entries: Array(shownEntries[runStart..<runEnd]),
                indexRailEntryNumber: startsItsRailEntry ? railEntries.count : nil,
            ))

            runStart = runEnd
        }

        return GlossaryRail(runs: runs, entries: railEntries)
    }
}
