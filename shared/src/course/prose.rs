const PARAGRAPH_SEPARATOR: &str = "\n\n";
const LIST_ITEM_MARKER: &str = "- ";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProseBlock {
    Paragraph(String),
    List(Vec<String>),
}

/// Splits explanatory text on blank lines. A block whose every line starts with `- ` is a list.
pub fn prose_blocks(text: &str) -> Vec<ProseBlock> {
    text.split(PARAGRAPH_SEPARATOR)
        .map(str::trim)
        .filter(|block| !block.is_empty())
        .map(into_block)
        .collect()
}

/// The first problem in how a text mixes lists and paragraphs, if there is one.
pub fn find_prose_problem(text: &str) -> Option<String> {
    for block in text.split(PARAGRAPH_SEPARATOR).map(str::trim).filter(|block| !block.is_empty()) {
        let lines: Vec<&str> = block.lines().map(str::trim).collect();
        let list_line_count: usize =
            lines.iter().filter(|line| line.starts_with(LIST_ITEM_MARKER) || **line == "-").count();

        if list_line_count > 0 && list_line_count < lines.len() {
            return Some(format!(
                "a list item and other text share a block, separate them with a blank line: {block}"
            ));
        }

        if list_line_count == lines.len() && lines.iter().any(|line| line.trim_start_matches('-').trim().is_empty()) {
            return Some(format!("a list item is empty: {block}"));
        }
    }

    None
}

fn into_block(block: &str) -> ProseBlock {
    let lines: Vec<&str> = block.lines().map(str::trim).collect();
    let is_list: bool = lines.iter().all(|line| line.starts_with(LIST_ITEM_MARKER));

    if !is_list {
        return ProseBlock::Paragraph(block.to_string());
    }

    ProseBlock::List(lines.iter().map(|line| line[LIST_ITEM_MARKER.len()..].trim().to_string()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prose_blocks_splits_paragraphs_on_blank_lines() {
        let blocks: Vec<ProseBlock> = prose_blocks("First.\n\n  Second.  \n\n\n\nThird.");

        assert_eq!(
            blocks,
            vec![
                ProseBlock::Paragraph("First.".to_string()),
                ProseBlock::Paragraph("Second.".to_string()),
                ProseBlock::Paragraph("Third.".to_string()),
            ]
        );
    }

    #[test]
    fn prose_blocks_reads_a_block_of_dash_lines_as_a_list() {
        let blocks: Vec<ProseBlock> = prose_blocks("Intro.\n\n- one 'a'\n- two 'b'\n\nAfter.");

        assert_eq!(
            blocks,
            vec![
                ProseBlock::Paragraph("Intro.".to_string()),
                ProseBlock::List(vec!["one 'a'".to_string(), "two 'b'".to_string()]),
                ProseBlock::Paragraph("After.".to_string()),
            ]
        );
    }

    #[test]
    fn prose_blocks_keeps_a_hyphen_inside_a_word_as_a_paragraph() {
        let blocks: Vec<ProseBlock> = prose_blocks("-ar verbs end in -ar.");

        assert_eq!(blocks, vec![ProseBlock::Paragraph("-ar verbs end in -ar.".to_string())]);
    }

    #[test]
    fn find_prose_problem_rejects_a_list_item_sharing_a_block_with_text() {
        assert!(find_prose_problem("Intro:\n- one\n- two").is_some());
    }

    #[test]
    fn find_prose_problem_rejects_an_empty_list_item() {
        assert!(find_prose_problem("- one\n-").is_some());
    }

    #[test]
    fn find_prose_problem_accepts_lists_and_paragraphs_apart() {
        assert_eq!(find_prose_problem("Intro.\n\n- one\n- two\n\nAfter."), None);
    }
}
