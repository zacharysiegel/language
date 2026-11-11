use shared::interface_text;

#[uniffi::export]
pub fn counted_noun_phrase(count: u32, singular_noun: String, plural_noun: String) -> String {
    interface_text::counted_noun_phrase(count as usize, &singular_noun, &plural_noun)
}
