use leptos::prelude::*;
use shared::course::{GrammarTable, GroupedTable, GroupedTableField, GroupedTableGroup, GroupedTableRow};

/// A grammar table as a grid on a wide screen and as labelled rows, grouped where its first column repeats, on a narrow one.
#[component]
pub fn GrammarTableView(table: GrammarTable) -> impl IntoView {
    let grouped_table: GroupedTable = shared::course::grouped_table(&table);

    view! {
        <div class="table-scroll">
            <GridTable table />
        </div>
        <GroupedTableView grouped_table />
    }
}

#[component]
fn GridTable(table: GrammarTable) -> impl IntoView {
    let header_cells: Vec<_> = table.headers.iter().map(|header| view! { <th>{header.clone()}</th> }).collect();
    let body_rows: Vec<_> = table
        .rows
        .iter()
        .map(|row| {
            let cells: Vec<_> = row.iter().map(|cell| view! { <td>{cell.clone()}</td> }).collect();

            view! { <tr>{cells}</tr> }
        })
        .collect();

    view! {
        <table class="grid paradigm">
            <thead><tr>{header_cells}</tr></thead>
            <tbody>{body_rows}</tbody>
        </table>
    }
}

#[component]
fn GroupedTableView(grouped_table: GroupedTable) -> impl IntoView {
    match grouped_table {
        GroupedTable::Grouped { first_column_header, groups } => {
            let lead_header: String = groups
                .first()
                .and_then(|group| group.rows.first())
                .map(|row| row.lead.header.clone())
                .unwrap_or_default();
            let group_views: Vec<_> = groups.into_iter().map(|group| view! { <GroupView group /> }).collect();

            view! {
                <div class="grouped-table">
                    <p class="grouped-table-key">
                        <span class="grouped-table-key-heading">{first_column_header}</span>
                        <span class="grouped-table-key-lead">{lead_header}</span>
                    </p>
                    {group_views}
                </div>
            }
            .into_any()
        }
        GroupedTable::Stacked { rows } => {
            let lead_header: String = rows.first().map(|row| row.lead.header.clone()).unwrap_or_default();

            view! {
                <div class="grouped-table">
                    <p class="grouped-table-key">
                        <span class="grouped-table-key-lead">{lead_header}</span>
                    </p>
                    <RowList rows />
                </div>
            }
            .into_any()
        }
    }
}

#[component]
fn GroupView(group: GroupedTableGroup) -> impl IntoView {
    view! {
        <section class="grouped-table-group">
            <h4 class="grouped-table-heading">{group.heading}</h4>
            <RowList rows=group.rows />
        </section>
    }
}

#[component]
fn RowList(rows: Vec<GroupedTableRow>) -> impl IntoView {
    let row_views: Vec<_> = rows.into_iter().map(|row| view! { <RowView row /> }).collect();

    view! { <ul class="grouped-table-rows">{row_views}</ul> }
}

#[component]
fn RowView(row: GroupedTableRow) -> impl IntoView {
    let field_views: Vec<_> = row.fields.into_iter().map(|field| view! { <FieldView field /> }).collect();

    view! {
        <li class="grouped-table-row">
            <dl>
                // The key above the rows names the lead's column, so its header here is for assistive technology only.
                <div class="grouped-table-lead">
                    <dt class="visually-hidden">{row.lead.header}</dt>
                    <dd>{row.lead.value}</dd>
                </div>
                {field_views}
            </dl>
        </li>
    }
}

#[component]
fn FieldView(field: GroupedTableField) -> impl IntoView {
    view! {
        <div class="grouped-table-field">
            <dt>{field.header}</dt>
            <dd>{field.value}</dd>
        </div>
    }
}
