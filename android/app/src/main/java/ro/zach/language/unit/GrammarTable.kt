package ro.zach.language.unit

import androidx.compose.foundation.border
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.IntrinsicSize
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.layout.widthIn
import androidx.compose.foundation.rememberScrollState
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.platform.LocalWindowInfo
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.heading
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.testTag
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.Dp
import ro.zach.language.core.GrammarTableRecord
import ro.zach.language.core.GroupedTableFieldRecord
import ro.zach.language.core.GroupedTableRecord
import ro.zach.language.core.GroupedTableRowRecord
import ro.zach.language.core.groupedTable
import ro.zach.language.design.HairlineRule
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType
import ro.zach.language.design.MicroLabel

private val sentenceEndings: Set<Char> = setOf('.', '?', '!')

/** A grammar table, rearranged into labelled rows at compact width. */
@Composable
fun GrammarTableView(table: GrammarTableRecord, modifier: Modifier = Modifier) {
    if (isCompactWidth()) {
        GroupedTableView(layout = groupedTable(table), modifier = modifier)
    } else {
        ParadigmTable(table = table, modifier = modifier)
    }
}

@Composable
private fun isCompactWidth(): Boolean {
    val windowWidth: Dp = with(LocalDensity.current) { LocalWindowInfo.current.containerSize.width.toDp() }

    return windowWidth < LanguageSpacing.compactWidthThreshold
}

/** A grammar table drawn as a hairline grid, scrolling sideways when wider than the screen. */
@Composable
fun ParadigmTable(table: GrammarTableRecord, modifier: Modifier = Modifier) {
    Row(
        modifier = modifier
            .horizontalScroll(rememberScrollState())
            .border(LanguageSpacing.hairline, LanguageColors.ink),
    ) {
        /* Column by column, so each column takes its widest cell's width; unwrapped cells keep every row one line high. */
        table.headers.forEachIndexed { columnIndex: Int, header: String ->
            Column(modifier = Modifier.width(IntrinsicSize.Max)) {
                ParadigmTableCell {
                    LanguageText(
                        text = header.uppercase(),
                        style = LanguageType.labelText.copy(letterSpacing = LanguageType.tagTracking),
                        color = LanguageColors.inkMuted,
                        maxLines = 1,
                    )
                }

                for (row in table.rows) {
                    ParadigmTableCell {
                        LanguageText(text = row.getOrElse(columnIndex) { "" }, style = LanguageType.bodyText, maxLines = 1)
                    }
                }
            }
        }
    }
}

@Composable
private fun ParadigmTableCell(content: @Composable () -> Unit) {
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .widthIn(min = LanguageSpacing.tableCellMinimumWidth)
            .border(LanguageSpacing.hairline, LanguageColors.rule)
            .padding(horizontal = LanguageSpacing.spaceSm, vertical = LanguageSpacing.tableCellVerticalPadding),
    ) {
        content()
    }
}

/** A grammar table as one entry per row, every cell beside its column's header. */
@Composable
fun GroupedTableView(layout: GroupedTableRecord, modifier: Modifier = Modifier) {
    Column(modifier = modifier.fillMaxWidth()) {
        when (layout) {
            is GroupedTableRecord.Grouped -> {
                GroupedTableKey(headers = listOf(layout.firstColumnHeader) + leadHeader(layout.groups.firstOrNull()?.rows.orEmpty()))

                for (group in layout.groups) {
                    GroupedTableHeading(text = group.heading)
                    GroupedTableRows(rows = group.rows)
                }
            }

            is GroupedTableRecord.Stacked -> {
                GroupedTableKey(headers = leadHeader(layout.rows))
                GroupedTableRows(rows = layout.rows)
            }
        }
    }
}

private fun leadHeader(rows: List<GroupedTableRowRecord>): List<String> {
    val firstRow: GroupedTableRowRecord = rows.firstOrNull()
        ?: return emptyList()

    return listOf(firstRow.lead.header)
}

/** The names of the columns shown as group headings and as row leads, above the rows. */
@Composable
fun GroupedTableKey(headers: List<String>, modifier: Modifier = Modifier) {
    Column(modifier = modifier.fillMaxWidth(), verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs)) {
        for (header in headers) {
            MicroLabel(text = header, modifier = Modifier.testTag("grouped-table-key"))
        }

        HairlineRule(color = LanguageColors.ink)
    }
}

/** A group's heading, which keeps its cell's case. */
@Composable
fun GroupedTableHeading(text: String, modifier: Modifier = Modifier) {
    Column(
        modifier = modifier.fillMaxWidth().padding(top = LanguageSpacing.spaceSm + LanguageSpacing.spaceXs),
        verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs),
    ) {
        LanguageText(
            text = text,
            style = LanguageType.bodyText.copy(fontWeight = FontWeight.SemiBold),
            modifier = Modifier
                .semantics { heading() }
                .testTag("grouped-table-heading"),
        )

        HairlineRule(color = LanguageColors.ink)
    }
}

@Composable
fun GroupedTableRows(rows: List<GroupedTableRowRecord>) {
    for (row in rows) {
        GroupedTableRow(row = row)
        HairlineRule()
    }
}

@Composable
fun GroupedTableRow(row: GroupedTableRowRecord, modifier: Modifier = Modifier) {
    val spokenText: String = groupedTableRowSpokenText(row)

    Column(
        modifier = modifier
            .fillMaxWidth()
            .clearAndSetSemantics {
                contentDescription = spokenText
                testTag = "grouped-table-row"
            }
            .padding(vertical = LanguageSpacing.spaceSm),
        verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs),
    ) {
        LanguageText(text = row.lead.value, style = LanguageType.bodyText.copy(fontWeight = FontWeight.SemiBold))

        for (field in row.fields) {
            GroupedTableFieldLine(field = field)
        }
    }
}

@Composable
private fun GroupedTableFieldLine(field: GroupedTableFieldRecord) {
    Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
        MicroLabel(text = field.header, modifier = Modifier.width(LanguageSpacing.tableFieldLabelWidth).alignByBaseline())
        LanguageText(text = field.value, style = LanguageType.bodyText, modifier = Modifier.weight(1f).alignByBaseline())
    }
}

/** Every cell of the row after its column's header, the lead first, each ending as a sentence. */
fun groupedTableRowSpokenText(row: GroupedTableRowRecord): String {
    val fields: List<GroupedTableFieldRecord> = listOf(row.lead) + row.fields

    return fields.joinToString(separator = " ") { field -> spokenSentence("${field.header}: ${field.value}") }
}

private fun spokenSentence(text: String): String {
    val endsAsASentence: Boolean = text.lastOrNull()?.let { lastCharacter -> lastCharacter in sentenceEndings } ?: false

    return if (endsAsASentence) text else "$text."
}
