use shared::course;

#[uniffi::export]
pub fn unit_item_label(level_code: String, unit_order: u32, item_number: u32) -> String {
    course::unit_item_label(&level_code, unit_order, item_number as usize)
}

#[uniffi::export]
pub fn unit_item_index_minimum_item_count() -> u32 {
    course::UNIT_ITEM_INDEX_MINIMUM_ITEM_COUNT as u32
}
