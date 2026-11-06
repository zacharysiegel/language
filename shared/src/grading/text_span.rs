const QUOTE_CHARACTERS: [char; 8] = ['"', '\'', '“', '”', '‘', '’', '«', '»'];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CharacterSpan {
    pub start: usize,
    pub end: usize,
}

impl CharacterSpan {
    pub fn length(&self) -> usize {
        self.end - self.start
    }

    pub fn overlap_length(&self, other: &CharacterSpan) -> usize {
        let overlap_start: usize = self.start.max(other.start);
        let overlap_end: usize = self.end.min(other.end);

        overlap_end.saturating_sub(overlap_start)
    }

    pub fn contains(&self, other: &CharacterSpan) -> bool {
        self.start <= other.start && other.end <= self.end
    }
}

/// Finds `needle` in `text`, exactly first and ignoring case if that fails, preferring the occurrence that overlaps
/// `preferred_spans` the most.
pub fn locate_text(text: &str, needle: &str, preferred_spans: &[CharacterSpan]) -> Option<CharacterSpan> {
    let text_characters: Vec<char> = text.chars().collect();
    let needle_characters: Vec<char> = trim_quotation(needle).chars().collect();

    if needle_characters.is_empty() {
        return None;
    }

    let mut starts: Vec<usize> = find_starts(&text_characters, &needle_characters, |character| character);

    if starts.is_empty() {
        starts = find_starts(&text_characters, &needle_characters, fold_character);
    }

    let best_start: Option<usize> = starts.iter().copied().max_by_key(|start| {
        let candidate: CharacterSpan = CharacterSpan {
            start: *start,
            end: start + needle_characters.len(),
        };
        let overlap: usize = preferred_spans.iter().map(|preferred| candidate.overlap_length(preferred)).sum();

        (overlap, usize::MAX - start)
    });

    best_start.map(|start| CharacterSpan {
        start,
        end: start + needle_characters.len(),
    })
}

pub fn slice_text(text: &str, span: &CharacterSpan) -> String {
    text.chars().skip(span.start).take(span.length()).collect()
}

pub fn normalize_for_comparison(text: &str) -> String {
    let lowered: String = text
        .chars()
        .map(|character| {
            if character.is_alphanumeric() {
                fold_character(character)
            } else {
                ' '
            }
        })
        .collect();

    lowered.split_whitespace().collect::<Vec<&str>>().join(" ")
}

fn trim_quotation(text: &str) -> &str {
    text.trim().trim_matches(|character: char| QUOTE_CHARACTERS.contains(&character)).trim()
}

fn fold_character(character: char) -> char {
    character.to_lowercase().next().unwrap_or(character)
}

fn find_starts(text_characters: &[char], needle_characters: &[char], fold: fn(char) -> char) -> Vec<usize> {
    if needle_characters.len() > text_characters.len() {
        return Vec::new();
    }

    let mut starts: Vec<usize> = Vec::new();

    for start in 0..=(text_characters.len() - needle_characters.len()) {
        let window: &[char] = &text_characters[start..start + needle_characters.len()];
        let equal: bool = window.iter().zip(needle_characters).all(|(left, right)| fold(*left) == fold(*right));

        if equal {
            starts.push(start);
        }
    }

    starts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locate_text_counts_characters_not_bytes() {
        let span: Option<CharacterSpan> = locate_text("De manhã eu bebo café.", "café", &[]);

        assert_eq!(span, Some(CharacterSpan { start: 17, end: 21 }));
    }

    #[test]
    fn locate_text_ignores_case_when_no_exact_match_exists() {
        let span: Option<CharacterSpan> = locate_text("Eu moro em o Brasil", "em O brasil", &[]);

        assert_eq!(span, Some(CharacterSpan { start: 8, end: 19 }));
    }

    #[test]
    fn locate_text_prefers_the_occurrence_overlapping_a_preferred_span() {
        let preferred: CharacterSpan = CharacterSpan { start: 18, end: 20 };

        let span: Option<CharacterSpan> = locate_text("Eu vi o gato e o gato.", "gato", &[preferred]);

        assert_eq!(span, Some(CharacterSpan { start: 17, end: 21 }));
    }

    #[test]
    fn locate_text_takes_the_first_occurrence_without_preference() {
        let span: Option<CharacterSpan> = locate_text("Eu vi o gato e o gato.", "gato", &[]);

        assert_eq!(span, Some(CharacterSpan { start: 8, end: 12 }));
    }

    #[test]
    fn locate_text_trims_surrounding_quotation_marks() {
        let span: Option<CharacterSpan> = locate_text("Eu chamo Pedro.", "“chamo”", &[]);

        assert_eq!(span, Some(CharacterSpan { start: 3, end: 8 }));
    }

    #[test]
    fn locate_text_returns_none_when_the_text_is_absent() {
        assert_eq!(locate_text("Eu chamo Pedro.", "Ana", &[]), None);
        assert_eq!(locate_text("Eu chamo Pedro.", "", &[]), None);
    }

    #[test]
    fn normalize_for_comparison_ignores_case_punctuation_and_spacing() {
        assert_eq!(normalize_for_comparison("  Me,  CHAMO. "), "me chamo");
    }
}
