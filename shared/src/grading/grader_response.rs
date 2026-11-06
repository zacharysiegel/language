use serde::{Deserialize, Serialize};

use crate::grading::CharacterSpan;
use crate::grading::text_span;

// A serialized schema lists properties alphabetically, so the wire names sort in the order the model should write them.
#[derive(Debug, Deserialize, Serialize)]
pub struct GraderResponse {
    pub answers_the_question: bool,
    #[serde(rename = "language_is_portuguese")]
    pub written_in_portuguese: bool,
    #[serde(rename = "mistakes")]
    pub findings: Vec<GraderFinding>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GraderFinding {
    #[serde(rename = "error_text")]
    pub quote: String,
    pub explanation: String,
    #[serde(rename = "kind")]
    pub category: String,
    #[serde(rename = "replacement")]
    pub correction: String,
}

pub fn parse_grader_response(content: &str) -> Result<GraderResponse, serde_json::Error> {
    serde_json::from_str(content)
}

/// The findings whose quote is in the answer and whose correction changes more than spacing or surrounding punctuation.
pub fn retain_applicable_findings(answer: &str, findings: Vec<GraderFinding>) -> Vec<GraderFinding> {
    findings.into_iter().filter(|finding| finding_applies(answer, finding)).collect()
}

pub fn changes_nothing(answer: &str, quote_span: &CharacterSpan, finding: &GraderFinding) -> bool {
    let quoted_text: String = text_span::slice_text(answer, quote_span);

    text_span::normalize_for_comparison(&quoted_text) == text_span::normalize_for_comparison(&finding.correction)
}

fn finding_applies(answer: &str, finding: &GraderFinding) -> bool {
    let quote_span: Option<CharacterSpan> = text_span::locate_text(answer, &finding.quote, &[]);

    let Some(quote_span) = quote_span else {
        return false;
    };

    let quoted_text: String = text_span::slice_text(answer, &quote_span);

    trim_punctuation_and_collapse_spacing(&quoted_text) != trim_punctuation_and_collapse_spacing(&finding.correction)
}

fn trim_punctuation_and_collapse_spacing(text: &str) -> String {
    let trimmed_text: &str = text.trim_matches(|character: char| !character.is_alphanumeric());

    trimmed_text.split_whitespace().collect::<Vec<&str>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn finding(quote: &str, correction: &str) -> GraderFinding {
        GraderFinding {
            quote: quote.to_string(),
            explanation: "explanation".to_string(),
            category: "pronoun".to_string(),
            correction: correction.to_string(),
        }
    }

    #[test]
    fn retain_applicable_findings_drops_absent_quotes_and_corrections_that_change_nothing() {
        let findings: Vec<GraderFinding> = vec![
            finding("chamo", "me chamo"),
            finding("Ana", "a Ana"),
            finding("Pedro", " Pedro. "),
        ];

        let retained_findings: Vec<GraderFinding> = retain_applicable_findings("Eu chamo Pedro.", findings);
        let retained_quotes: Vec<&str> = retained_findings.iter().map(|finding| finding.quote.as_str()).collect();

        assert_eq!(retained_quotes, vec!["chamo"]);
    }

    #[test]
    fn retain_applicable_findings_keeps_corrections_of_capital_letters_and_hyphens() {
        let findings: Vec<GraderFinding> =
            vec![finding("Janeiro", "janeiro"), finding("segunda feira", "segunda-feira")];

        let retained_findings: Vec<GraderFinding> =
            retain_applicable_findings("Eu nasci em Janeiro, numa segunda feira.", findings);
        let retained_quotes: Vec<&str> = retained_findings.iter().map(|finding| finding.quote.as_str()).collect();

        assert_eq!(retained_quotes, vec!["Janeiro", "segunda feira"]);
    }
}
