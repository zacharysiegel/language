/// The fewest boxes a unit tab needs for its box index to be shown.
pub const UNIT_ITEM_INDEX_MINIMUM_ITEM_COUNT: usize = 2;

/// The label of a box in a unit tab, `<level code>.<unit order>.<item number>`, with items numbered from 1.
pub fn unit_item_label(level_code: &str, unit_order: u32, item_number: usize) -> String {
    format!("{level_code}.{unit_order}.{item_number}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_item_label_joins_the_level_code_unit_order_and_item_number() {
        assert_eq!(unit_item_label("A1", 1, 1), "A1.1.1");
        assert_eq!(unit_item_label("A1", 1, 2), "A1.1.2");
        assert_eq!(unit_item_label("A2", 15, 1), "A2.15.1");
    }
}
