use shared::course::{self, GrammarTable, GroupedTable, GroupedTableField, GroupedTableGroup, GroupedTableRow};

use crate::GrammarTableRecord;

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum GroupedTableRecord {
    Grouped {
        first_column_header: String,
        groups: Vec<GroupedTableGroupRecord>,
    },
    Stacked {
        rows: Vec<GroupedTableRowRecord>,
    },
}

impl From<GroupedTable> for GroupedTableRecord {
    fn from(table: GroupedTable) -> GroupedTableRecord {
        match table {
            GroupedTable::Grouped {
                first_column_header,
                groups,
            } => GroupedTableRecord::Grouped {
                first_column_header,
                groups: groups.into_iter().map(GroupedTableGroupRecord::from).collect(),
            },
            GroupedTable::Stacked { rows } => GroupedTableRecord::Stacked {
                rows: rows.into_iter().map(GroupedTableRowRecord::from).collect(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct GroupedTableGroupRecord {
    pub heading: String,
    pub rows: Vec<GroupedTableRowRecord>,
}

impl From<GroupedTableGroup> for GroupedTableGroupRecord {
    fn from(group: GroupedTableGroup) -> GroupedTableGroupRecord {
        GroupedTableGroupRecord {
            heading: group.heading,
            rows: group.rows.into_iter().map(GroupedTableRowRecord::from).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct GroupedTableRowRecord {
    pub lead: GroupedTableFieldRecord,
    pub fields: Vec<GroupedTableFieldRecord>,
}

impl From<GroupedTableRow> for GroupedTableRowRecord {
    fn from(row: GroupedTableRow) -> GroupedTableRowRecord {
        GroupedTableRowRecord {
            lead: GroupedTableFieldRecord::from(row.lead),
            fields: row.fields.into_iter().map(GroupedTableFieldRecord::from).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct GroupedTableFieldRecord {
    pub header: String,
    pub value: String,
}

impl From<GroupedTableField> for GroupedTableFieldRecord {
    fn from(field: GroupedTableField) -> GroupedTableFieldRecord {
        GroupedTableFieldRecord {
            header: field.header,
            value: field.value,
        }
    }
}

#[uniffi::export]
pub fn grouped_table(table: GrammarTableRecord) -> GroupedTableRecord {
    let grammar_table: GrammarTable = GrammarTable {
        headers: table.headers,
        rows: table.rows,
    };

    GroupedTableRecord::from(course::grouped_table(&grammar_table))
}
