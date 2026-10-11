import SwiftUI

/// A grammar table, rearranged into labelled rows at compact width.
struct GrammarTableView: View {
    let table: GrammarTableRecord

    @Environment(\.horizontalSizeClass) private var horizontalSizeClass

    var body: some View {
        if horizontalSizeClass == .compact {
            GroupedTableView(layout: groupedTable(table: table))
        } else {
            ParadigmTable(table: table)
        }
    }
}

/// A grammar table drawn as a hairline grid, scrolling sideways when wider than the screen.
struct ParadigmTable: View {
    let table: GrammarTableRecord

    var body: some View {
        ScrollView(.horizontal, showsIndicators: false) {
            Grid(horizontalSpacing: 0, verticalSpacing: 0) {
                GridRow {
                    ForEach(Array(table.headers.enumerated()), id: \.offset) { _, header in
                        cell(Text(header.uppercased()).font(.labelText).tracking(0.4).foregroundStyle(Color.inkMuted))
                    }
                }

                ForEach(Array(table.rows.enumerated()), id: \.offset) { _, row in
                    GridRow {
                        ForEach(Array(row.enumerated()), id: \.offset) { _, value in
                            cell(Text(value).font(.bodyText))
                        }
                    }
                }
            }
            .overlay(Rectangle().stroke(Color.ink, lineWidth: .hairline))
        }
    }

    private func cell(_ content: some View) -> some View {
        content
            .padding(.horizontal, .spaceSm)
            .padding(.vertical, .spaceXs + 2)
            .frame(minWidth: 80, maxWidth: .infinity, alignment: .leading)
            .overlay(Rectangle().stroke(Color.rule, lineWidth: .hairline))
    }
}

/// A grammar table as one entry per row, every cell beside its column's header.
struct GroupedTableView: View {
    let layout: GroupedTableRecord

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            switch layout {
            case .grouped(let firstColumnHeader, let groups):
                GroupedTableKey(headers: [firstColumnHeader] + Self.leadHeader(groups.first?.rows ?? []))

                ForEach(Array(groups.enumerated()), id: \.offset) { _, group in
                    GroupedTableHeading(text: group.heading)
                    GroupedTableRows(rows: group.rows)
                }
            case .stacked(let rows):
                GroupedTableKey(headers: Self.leadHeader(rows))
                GroupedTableRows(rows: rows)
            }
        }
    }

    private static func leadHeader(_ rows: [GroupedTableRowRecord]) -> [String] {
        rows.first.map { row in [row.lead.header] } ?? []
    }
}

/// The names of the columns shown as group headings and as row leads, above the rows.
struct GroupedTableKey: View {
    let headers: [String]

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceXs) {
            ForEach(Array(headers.enumerated()), id: \.offset) { _, header in
                MicroLabel(text: header)
                    .fixedSize(horizontal: false, vertical: true)
                    .accessibilityIdentifier("grouped-table-key")
            }

            HairlineRule(color: Color.ink)
        }
    }
}

/// A group's heading, which keeps its cell's case.
struct GroupedTableHeading: View {
    let text: String

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceXs) {
            Text(text)
                .font(.bodyText)
                .fontWeight(.semibold)
                .foregroundStyle(Color.ink)
                .fixedSize(horizontal: false, vertical: true)
                .accessibilityAddTraits(.isHeader)
                .accessibilityIdentifier("grouped-table-heading")

            HairlineRule(color: Color.ink)
        }
        .padding(.top, .spaceSm + .spaceXs)
    }
}

struct GroupedTableRows: View {
    let rows: [GroupedTableRowRecord]

    var body: some View {
        ForEach(Array(rows.enumerated()), id: \.offset) { _, row in
            GroupedTableRow(row: row)
            HairlineRule()
        }
    }
}

struct GroupedTableRow: View {
    static let sentenceEndings: Set<Character> = [".", "?", "!"]

    let row: GroupedTableRowRecord

    var body: some View {
        VStack(alignment: .leading, spacing: .spaceXs) {
            Text(row.lead.value)
                .font(.bodyText)
                .fontWeight(.semibold)
                .fixedSize(horizontal: false, vertical: true)

            ForEach(Array(row.fields.enumerated()), id: \.offset) { _, field in
                HStack(alignment: .firstTextBaseline, spacing: .spaceSm) {
                    MicroLabel(text: field.header)
                        .fixedSize(horizontal: false, vertical: true)
                        .frame(width: .tableFieldLabelWidth, alignment: .leading)

                    Text(field.value)
                        .font(.bodyText)
                        .fixedSize(horizontal: false, vertical: true)

                    Spacer(minLength: 0)
                }
            }
        }
        .padding(.vertical, .spaceSm)
        .accessibilityElement(children: .ignore)
        .accessibilityLabel(Self.spokenText(row))
        .accessibilityIdentifier("grouped-table-row")
    }

    /// Every cell of the row after its column's header, the lead first, each ending as a sentence.
    static func spokenText(_ row: GroupedTableRowRecord) -> String {
        let fields: [GroupedTableFieldRecord] = [row.lead] + row.fields

        return fields.map { field in spokenSentence("\(field.header): \(field.value)") }.joined(separator: " ")
    }

    private static func spokenSentence(_ text: String) -> String {
        let endsAsASentence: Bool = text.last.map { lastCharacter in sentenceEndings.contains(lastCharacter) } ?? false

        return endsAsASentence ? text : "\(text)."
    }
}
