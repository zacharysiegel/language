use serde::{Deserialize, Serialize};

use shared::grading::{CharacterSpan, GraderFinding, GraderResponse, grader_response, text_span};

use crate::grading_case::{GradingCase, GradingCaseKind, ReferenceError};

const EXTRA_QUOTE_WORDS_BEFORE_IMPRECISE: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingOutcomeKind {
    Matched,
    FalsePositive,
    ChangesNothing,
    HallucinatedQuote,
    Duplicate,
}

impl FindingOutcomeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            FindingOutcomeKind::Matched => "matched",
            FindingOutcomeKind::FalsePositive => "false positive",
            FindingOutcomeKind::ChangesNothing => "changes nothing",
            FindingOutcomeKind::HallucinatedQuote => "quote not in the answer",
            FindingOutcomeKind::Duplicate => "duplicate",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FindingMatch {
    pub reference_index: usize,
    pub category_correct: bool,
    pub correction_agrees: bool,
    pub imprecise_quote: bool,
}

#[derive(Debug, Clone)]
pub struct CaseScore {
    pub kind: GradingCaseKind,
    pub response_valid: bool,
    pub written_in_portuguese_correct: bool,
    pub answers_the_question_correct: bool,
    pub findings_scored: bool,
    pub finding_count: usize,
    pub matches: Vec<FindingMatch>,
    pub finding_outcomes: Vec<FindingOutcomeKind>,
    pub false_positives: usize,
    pub hallucinated_quotes: usize,
    pub duplicates: usize,
    pub missed_reference_indexes: Vec<usize>,
}

impl CaseScore {
    pub fn true_positives(&self) -> usize {
        self.matches.len()
    }

    pub fn reference_error_count(&self) -> usize {
        self.matches.len() + self.missed_reference_indexes.len()
    }

    pub fn flagged_a_control(&self) -> bool {
        self.kind == GradingCaseKind::Control && self.finding_count > 0
    }

    pub fn has_failure(&self) -> bool {
        !self.response_valid
            || !self.written_in_portuguese_correct
            || !self.answers_the_question_correct
            || self.false_positives > 0
            || self.hallucinated_quotes > 0
            || !self.missed_reference_indexes.is_empty()
            || self.flagged_a_control()
    }
}

pub fn score_case(case: &GradingCase, response: Option<&GraderResponse>) -> CaseScore {
    let kind: GradingCaseKind = case.kind();
    let findings_scored: bool = kind != GradingCaseKind::NotPortuguese;

    let Some(response) = response else {
        return CaseScore {
            kind,
            response_valid: false,
            written_in_portuguese_correct: false,
            answers_the_question_correct: false,
            findings_scored,
            finding_count: 0,
            matches: Vec::new(),
            finding_outcomes: Vec::new(),
            false_positives: 0,
            hallucinated_quotes: 0,
            duplicates: 0,
            missed_reference_indexes: all_reference_indexes(case, findings_scored),
        };
    };

    let tally: FindingTally = if findings_scored {
        match_findings(case, &response.findings)
    } else {
        FindingTally::empty()
    };
    let missed_reference_indexes: Vec<usize> = (0..case.reference.errors.len())
        .filter(|reference_index| {
            !tally.matches.iter().any(|finding_match| finding_match.reference_index == *reference_index)
        })
        .collect();

    CaseScore {
        kind,
        response_valid: true,
        written_in_portuguese_correct: response.written_in_portuguese == case.reference.written_in_portuguese,
        answers_the_question_correct: response.answers_the_question == case.reference.answers_the_question,
        findings_scored,
        finding_count: response.findings.len(),
        matches: tally.matches,
        finding_outcomes: tally.finding_outcomes,
        false_positives: tally.false_positives,
        hallucinated_quotes: tally.hallucinated_quotes,
        duplicates: tally.duplicates,
        missed_reference_indexes,
    }
}

fn all_reference_indexes(case: &GradingCase, findings_scored: bool) -> Vec<usize> {
    if !findings_scored {
        return Vec::new();
    }

    (0..case.reference.errors.len()).collect()
}

struct FindingTally {
    matches: Vec<FindingMatch>,
    finding_outcomes: Vec<FindingOutcomeKind>,
    false_positives: usize,
    hallucinated_quotes: usize,
    duplicates: usize,
}

impl FindingTally {
    fn empty() -> FindingTally {
        FindingTally {
            matches: Vec::new(),
            finding_outcomes: Vec::new(),
            false_positives: 0,
            hallucinated_quotes: 0,
            duplicates: 0,
        }
    }

    fn record_false_positive(&mut self) {
        self.false_positives += 1;
        self.finding_outcomes.push(FindingOutcomeKind::FalsePositive);
    }

    fn record_change_nothing(&mut self) {
        self.false_positives += 1;
        self.finding_outcomes.push(FindingOutcomeKind::ChangesNothing);
    }

    fn record_hallucinated_quote(&mut self) {
        self.hallucinated_quotes += 1;
        self.finding_outcomes.push(FindingOutcomeKind::HallucinatedQuote);
    }

    fn record_duplicate(&mut self) {
        self.duplicates += 1;
        self.finding_outcomes.push(FindingOutcomeKind::Duplicate);
    }

    fn record_match(&mut self, finding_match: FindingMatch) {
        self.matches.push(finding_match);
        self.finding_outcomes.push(FindingOutcomeKind::Matched);
    }
}

fn match_findings(case: &GradingCase, findings: &[GraderFinding]) -> FindingTally {
    let reference_spans: Vec<Option<CharacterSpan>> = case.reference_spans();
    let present_reference_spans: Vec<CharacterSpan> = reference_spans.iter().flatten().copied().collect();
    let mut tally: FindingTally = FindingTally::empty();

    for finding in findings {
        let quote_span: Option<CharacterSpan> =
            text_span::locate_text(&case.submission, &finding.quote, &present_reference_spans);

        let Some(quote_span) = quote_span else {
            tally.record_hallucinated_quote();
            continue;
        };

        if grader_response::changes_nothing(&case.submission, &quote_span, finding) {
            tally.record_change_nothing();
            continue;
        }

        let best_unmatched_reference_index: Option<usize> =
            best_unmatched_reference_index(&reference_spans, &tally.matches, &quote_span);

        if let Some(reference_index) = best_unmatched_reference_index {
            let finding_match: FindingMatch =
                score_match(case, finding, &quote_span, reference_index, &reference_spans);
            tally.record_match(finding_match);
            continue;
        }

        let repeats_a_matched_reference: bool =
            reference_spans.iter().enumerate().any(|(reference_index, reference_span)| {
                is_matched(&tally.matches, reference_index) && overlaps(reference_span, &quote_span)
            });

        if repeats_a_matched_reference {
            tally.record_duplicate();
        } else {
            tally.record_false_positive();
        }
    }

    tally
}

fn best_unmatched_reference_index(
    reference_spans: &[Option<CharacterSpan>],
    matches: &[FindingMatch],
    quote_span: &CharacterSpan,
) -> Option<usize> {
    let mut best: Option<(usize, usize)> = None;

    for (reference_index, reference_span) in reference_spans.iter().enumerate() {
        let Some(reference_span) = reference_span else {
            continue;
        };

        if is_matched(matches, reference_index) {
            continue;
        }

        let overlap_length: usize = quote_span.overlap_length(reference_span);

        if overlap_length == 0 {
            continue;
        }

        let better: bool = match best {
            Some((_, best_overlap)) => overlap_length > best_overlap,
            None => true,
        };

        if better {
            best = Some((reference_index, overlap_length));
        }
    }

    best.map(|(reference_index, _)| reference_index)
}

fn is_matched(matches: &[FindingMatch], reference_index: usize) -> bool {
    matches.iter().any(|finding_match| finding_match.reference_index == reference_index)
}

fn overlaps(reference_span: &Option<CharacterSpan>, quote_span: &CharacterSpan) -> bool {
    match reference_span {
        Some(reference_span) => quote_span.overlap_length(reference_span) > 0,
        None => false,
    }
}

fn score_match(
    case: &GradingCase,
    finding: &GraderFinding,
    quote_span: &CharacterSpan,
    reference_index: usize,
    reference_spans: &[Option<CharacterSpan>],
) -> FindingMatch {
    let reference_error: &ReferenceError = &case.reference.errors[reference_index];
    let reference_span: CharacterSpan = reference_spans[reference_index].expect("a matched reference error has a span");

    let quote_word_count: usize = count_words(&text_span::slice_text(&case.submission, quote_span));
    let reference_word_count: usize = count_words(&reference_error.wrong_text);

    FindingMatch {
        reference_index,
        category_correct: finding.category == reference_error.category.as_str(),
        correction_agrees: correction_agrees(&case.submission, finding, quote_span, &reference_span, reference_error),
        imprecise_quote: quote_word_count > reference_word_count + EXTRA_QUOTE_WORDS_BEFORE_IMPRECISE,
    }
}

fn correction_agrees(
    submission: &str,
    finding: &GraderFinding,
    quote_span: &CharacterSpan,
    reference_span: &CharacterSpan,
    reference_error: &ReferenceError,
) -> bool {
    let found_correction: String = text_span::normalize_for_comparison(&finding.correction);

    reference_error.accepted_corrections.iter().any(|accepted_correction| {
        let direct: bool = text_span::normalize_for_comparison(accepted_correction) == found_correction;

        if direct {
            return true;
        }

        if !quote_span.contains(reference_span) {
            return false;
        }

        let before: String = text_span::slice_text(
            submission,
            &CharacterSpan {
                start: quote_span.start,
                end: reference_span.start,
            },
        );
        let after: String = text_span::slice_text(
            submission,
            &CharacterSpan {
                start: reference_span.end,
                end: quote_span.end,
            },
        );

        text_span::normalize_for_comparison(&format!("{before}{accepted_correction}{after}")) == found_correction
    })
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::grading::ErrorCategoryKind;

    use crate::grading_case::{GradingTask, ReferenceAnnotation};

    fn finding(quote: &str, category: &str, correction: &str) -> GraderFinding {
        GraderFinding {
            quote: quote.to_string(),
            category: category.to_string(),
            correction: correction.to_string(),
            explanation: "because".to_string(),
        }
    }

    fn response(findings: Vec<GraderFinding>) -> GraderResponse {
        GraderResponse {
            written_in_portuguese: true,
            answers_the_question: true,
            findings,
        }
    }

    fn reference_error(wrong_text: &str, category: ErrorCategoryKind, accepted_correction: &str) -> ReferenceError {
        ReferenceError {
            wrong_text: wrong_text.to_string(),
            category,
            accepted_corrections: vec![accepted_correction.to_string()],
        }
    }

    fn case(submission: &str, errors: Vec<ReferenceError>) -> GradingCase {
        GradingCase {
            id: "case".to_string(),
            level: "a1".to_string(),
            task: GradingTask {
                prompt_source: "Describe the morning.".to_string(),
                context_source: None,
            },
            submission: submission.to_string(),
            reference: ReferenceAnnotation {
                written_in_portuguese: true,
                answers_the_question: true,
                errors,
            },
        }
    }

    #[test]
    fn score_case_matches_a_precise_finding_with_category_and_correction() {
        let case: GradingCase = case(
            "De manha eu bebo café.",
            vec![reference_error("manha", ErrorCategoryKind::Spelling, "manhã")],
        );
        let response: GraderResponse = response(vec![finding("manha", "spelling", "manhã")]);

        let score: CaseScore = score_case(&case, Some(&response));

        assert_eq!(score.true_positives(), 1);
        assert!(score.matches[0].category_correct);
        assert!(score.matches[0].correction_agrees);
        assert!(!score.matches[0].imprecise_quote);
        assert_eq!(score.false_positives, 0);
        assert!(score.missed_reference_indexes.is_empty());
    }

    #[test]
    fn score_case_counts_a_whole_sentence_quote_as_a_match_but_imprecise() {
        let case: GradingCase = case(
            "Hoje eu acordei tarde, tomei cafe e fui trabalhar de ônibus.",
            vec![reference_error("cafe", ErrorCategoryKind::Spelling, "café")],
        );
        let response: GraderResponse = response(vec![finding(
            "Hoje eu acordei tarde, tomei cafe e fui trabalhar de ônibus.",
            "spelling",
            "Hoje eu acordei tarde, tomei café e fui trabalhar de ônibus.",
        )]);

        let score: CaseScore = score_case(&case, Some(&response));

        assert_eq!(score.true_positives(), 1);
        assert!(score.matches[0].imprecise_quote);
        assert!(score.matches[0].correction_agrees);
    }

    #[test]
    fn score_case_counts_a_quote_absent_from_the_answer_as_hallucinated() {
        let case: GradingCase = case(
            "Eu moro em o Brasil.",
            vec![reference_error("em o", ErrorCategoryKind::ArticlePreposition, "no")],
        );
        let response: GraderResponse = response(vec![finding(
            "moro em um Brasil",
            "article_preposition",
            "moro no Brasil",
        )]);

        let score: CaseScore = score_case(&case, Some(&response));

        assert_eq!(score.hallucinated_quotes, 1);
        assert_eq!(score.true_positives(), 0);
        assert_eq!(score.missed_reference_indexes, vec![0]);
    }

    #[test]
    fn score_case_counts_a_finding_on_a_correct_word_as_a_false_positive() {
        let case: GradingCase = case("Eu como arroz.", Vec::new());
        let response: GraderResponse = response(vec![finding("arroz", "word_choice", "o arroz")]);

        let score: CaseScore = score_case(&case, Some(&response));

        assert_eq!(score.false_positives, 1);
        assert!(score.flagged_a_control());
    }

    #[test]
    fn score_case_counts_a_second_finding_on_a_matched_error_as_a_duplicate() {
        let case: GradingCase = case(
            "Eu chamo Pedro.",
            vec![reference_error("chamo", ErrorCategoryKind::Pronoun, "me chamo")],
        );
        let response: GraderResponse = response(vec![
            finding("chamo", "pronoun", "me chamo"),
            finding("Eu chamo", "verb_form", "Eu me chamo"),
        ]);

        let score: CaseScore = score_case(&case, Some(&response));

        assert_eq!(score.true_positives(), 1);
        assert_eq!(score.duplicates, 1);
        assert_eq!(score.false_positives, 0);
    }

    #[test]
    fn score_case_counts_a_finding_that_changes_nothing_as_a_false_positive() {
        let case: GradingCase = case(
            "Eu chamo Pedro.",
            vec![reference_error("chamo", ErrorCategoryKind::Pronoun, "me chamo")],
        );
        let response: GraderResponse = response(vec![finding("chamo", "pronoun", "chamo")]);

        let score: CaseScore = score_case(&case, Some(&response));

        assert_eq!(score.false_positives, 1);
        assert_eq!(score.true_positives(), 0);
    }

    #[test]
    fn score_case_assigns_each_reference_error_to_at_most_one_finding() {
        let case: GradingCase = case(
            "Minha amiga é engraçado e tem cabelos curtas.",
            vec![
                reference_error("engraçado", ErrorCategoryKind::Agreement, "engraçada"),
                reference_error("curtas", ErrorCategoryKind::Agreement, "curtos"),
            ],
        );
        let response: GraderResponse = response(vec![
            finding("engraçado", "agreement", "engraçada"),
            finding("curtas", "agreement", "curtos"),
        ]);

        let score: CaseScore = score_case(&case, Some(&response));

        assert_eq!(score.true_positives(), 2);
        assert!(score.missed_reference_indexes.is_empty());
    }

    #[test]
    fn score_case_treats_a_missing_response_as_every_error_missed() {
        let case: GradingCase = case(
            "Eu chamo Pedro.",
            vec![reference_error("chamo", ErrorCategoryKind::Pronoun, "me chamo")],
        );

        let score: CaseScore = score_case(&case, None);

        assert!(!score.response_valid);
        assert_eq!(score.missed_reference_indexes, vec![0]);
        assert!(score.has_failure());
    }
}
