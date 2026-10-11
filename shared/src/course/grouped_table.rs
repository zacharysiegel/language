use crate::course::GrammarTable;

/// The fewest columns a table needs to be grouped: one for the group headings and one for the row leads.
pub const GROUPED_TABLE_MINIMUM_COLUMN_COUNT: usize = 2;

/// A grammar table rearranged for a narrow screen, with every cell labelled by its column's header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupedTable {
    /// Consecutive rows sharing a first cell, under that cell as a heading.
    Grouped {
        /// Labels the group headings.
        first_column_header: String,
        groups: Vec<GroupedTableGroup>,
    },
    /// One entry per row, led by its first cell.
    Stacked { rows: Vec<GroupedTableRow> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupedTableGroup {
    pub heading: String,
    pub rows: Vec<GroupedTableRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupedTableRow {
    /// The second column's cell in a grouped table, the first column's in a stacked one.
    pub lead: GroupedTableField,
    pub fields: Vec<GroupedTableField>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupedTableField {
    pub header: String,
    pub value: String,
}

/// Groups a table by runs of its first column when any run spans more than one row, and stacks its rows otherwise; row order is kept.
pub fn grouped_table(table: &GrammarTable) -> GroupedTable {
    let first_column_runs: Vec<&[Vec<String>]> = first_column_runs(&table.rows);
    let has_enough_columns: bool = table.headers.len() >= GROUPED_TABLE_MINIMUM_COLUMN_COUNT;
    let any_run_spans_several_rows: bool = first_column_runs.len() < table.rows.len();

    if !has_enough_columns || !any_run_spans_several_rows {
        let rows: Vec<GroupedTableRow> = table.rows.iter().map(|row| table_row(&table.headers, row, 0)).collect();

        return GroupedTable::Stacked { rows };
    }

    let groups: Vec<GroupedTableGroup> = first_column_runs
        .into_iter()
        .map(|run| GroupedTableGroup {
            heading: cell(&run[0], 0),
            rows: run.iter().map(|row| table_row(&table.headers, row, 1)).collect(),
        })
        .collect();

    GroupedTable::Grouped {
        first_column_header: cell(&table.headers, 0),
        groups,
    }
}

fn first_column_runs(rows: &[Vec<String>]) -> Vec<&[Vec<String>]> {
    rows.chunk_by(|previous, next| previous.first() == next.first()).collect()
}

/// The row's cell at `lead_column` leads; every later cell with text becomes a field.
fn table_row(headers: &[String], row: &[String], lead_column: usize) -> GroupedTableRow {
    let lead: GroupedTableField = GroupedTableField {
        header: cell(headers, lead_column),
        value: cell(row, lead_column),
    };
    let fields: Vec<GroupedTableField> = row
        .iter()
        .enumerate()
        .skip(lead_column + 1)
        .filter(|(_, value)| !value.trim().is_empty())
        .map(|(column, value)| GroupedTableField {
            header: cell(headers, column),
            value: value.clone(),
        })
        .collect();

    GroupedTableRow { lead, fields }
}

/// Empty for a missing column.
fn cell(cells: &[String], column: usize) -> String {
    cells.get(column).cloned().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(headers: &[&str], rows: &[&[&str]]) -> GrammarTable {
        GrammarTable {
            headers: headers.iter().map(|header| header.to_string()).collect(),
            rows: rows.iter().map(|row| row.iter().map(|value| value.to_string()).collect()).collect(),
        }
    }

    fn field(header: &str, value: &str) -> GroupedTableField {
        GroupedTableField {
            header: header.to_string(),
            value: value.to_string(),
        }
    }

    #[test]
    fn grouped_table_groups_consecutive_rows_sharing_a_first_cell_and_leads_with_the_second_column() {
        let gender_table: GrammarTable = table(
            &["Topic", "Ending", "What happens", "Examples"],
            &[
                &["gender", "-o", "usually masculine", "o livro"],
                &["gender", "-a", "usually feminine", "a casa"],
                &["plural", "vowel", "add -s", "casas"],
                &["plural", "-m", "becomes -ns", "homens"],
            ],
        );

        let expected: GroupedTable = GroupedTable::Grouped {
            first_column_header: "Topic".to_string(),
            groups: vec![
                GroupedTableGroup {
                    heading: "gender".to_string(),
                    rows: vec![
                        GroupedTableRow {
                            lead: field("Ending", "-o"),
                            fields: vec![field("What happens", "usually masculine"), field("Examples", "o livro")],
                        },
                        GroupedTableRow {
                            lead: field("Ending", "-a"),
                            fields: vec![field("What happens", "usually feminine"), field("Examples", "a casa")],
                        },
                    ],
                },
                GroupedTableGroup {
                    heading: "plural".to_string(),
                    rows: vec![
                        GroupedTableRow {
                            lead: field("Ending", "vowel"),
                            fields: vec![field("What happens", "add -s"), field("Examples", "casas")],
                        },
                        GroupedTableRow {
                            lead: field("Ending", "-m"),
                            fields: vec![field("What happens", "becomes -ns"), field("Examples", "homens")],
                        },
                    ],
                },
            ],
        };

        assert_eq!(grouped_table(&gender_table), expected);
    }

    #[test]
    fn grouped_table_stacks_a_table_whose_first_cells_never_repeat_consecutively() {
        let preposition_table: GrammarTable = table(
            &["Preposition", "+ o", "+ a"],
            &[&["de", "do", "da"], &["em", "no", "na"], &["de", "do", "da"]],
        );

        let expected: GroupedTable = GroupedTable::Stacked {
            rows: vec![
                GroupedTableRow {
                    lead: field("Preposition", "de"),
                    fields: vec![field("+ o", "do"), field("+ a", "da")],
                },
                GroupedTableRow {
                    lead: field("Preposition", "em"),
                    fields: vec![field("+ o", "no"), field("+ a", "na")],
                },
                GroupedTableRow {
                    lead: field("Preposition", "de"),
                    fields: vec![field("+ o", "do"), field("+ a", "da")],
                },
            ],
        };

        assert_eq!(grouped_table(&preposition_table), expected);
    }

    #[test]
    fn grouped_table_gives_a_single_row_run_of_a_grouped_table_its_own_heading() {
        let mixed_table: GrammarTable = table(
            &["Topic", "Form"],
            &[
                &["gender", "-o"],
                &["gender", "-a"],
                &["plural", "-s"],
                &["gender", "-e"],
            ],
        );

        let headings: Vec<String> = match grouped_table(&mixed_table) {
            GroupedTable::Grouped { groups, .. } => groups.into_iter().map(|group| group.heading).collect(),
            GroupedTable::Stacked { .. } => Vec::new(),
        };

        assert_eq!(headings, vec!["gender", "plural", "gender"]);
    }

    #[test]
    fn grouped_table_leaves_out_a_field_whose_cell_is_empty() {
        let table: GrammarTable = GrammarTable {
            headers: vec!["Expression".to_string(), "Natural".to_string(), "Use".to_string()],
            rows: vec![
                vec!["sei lá".to_string(), "I have no idea".to_string(), "informal".to_string()],
                vec!["vale a pena".to_string(), "it is worth the effort".to_string(), String::new()],
            ],
        };

        let GroupedTable::Stacked { rows } = grouped_table(&table)
        else {
            panic!("a table whose first cells never repeat was grouped");
        };

        assert_eq!(rows[0].fields, vec![field("Natural", "I have no idea"), field("Use", "informal")]);
        assert_eq!(rows[1].fields, vec![field("Natural", "it is worth the effort")]);
    }

    #[test]
    fn grouped_table_stacks_a_one_column_table_even_when_its_cells_repeat() {
        let one_column_table: GrammarTable = table(&["Word"], &[&["sim"], &["sim"]]);

        let expected: GroupedTable = GroupedTable::Stacked {
            rows: vec![
                GroupedTableRow {
                    lead: field("Word", "sim"),
                    fields: Vec::new(),
                },
                GroupedTableRow {
                    lead: field("Word", "sim"),
                    fields: Vec::new(),
                },
            ],
        };

        assert_eq!(grouped_table(&one_column_table), expected);
    }
}
