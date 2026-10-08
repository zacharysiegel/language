const BRAZILIAN_PORTUGUESE: &str = "pt-BR";

/* Stems match any word beginning with them; the remaining lists match whole words or whole phrases. */
const EUROPEAN_PORTUGUESE_STEMS: [&str; 12] = [
    "autocarro",
    "comboio",
    "telemóvel",
    "telemovel",
    "telemóveis",
    "telemoveis",
    "rapariga",
    "frigorífico",
    "frigorifico",
    "miúdo",
    "miudo",
    "machimbombo",
];
const EUROPEAN_PORTUGUESE_WORDS: [&str; 7] = ["ecrã", "ecra", "equipa", "equipas", "facto", "factos", "estás"];
const EUROPEAN_PORTUGUESE_PHRASES: [&str; 6] = [
    "pequeno almoço",
    "pequeno almoco",
    "casa de banho",
    "tu és",
    "bilhete de identidade",
    "passadeira",
];
const EUROPEAN_PROGRESSIVE_AUXILIARIES: [&str; 9] = [
    "estou", "estás", "está", "estamos", "estão", "estava", "estavam", "estive", "esteve",
];
const SHORT_INFINITIVES: [&str; 7] = ["ler", "ver", "ter", "dar", "ser", "vir", "pôr"];
const INFINITIVE_ENDINGS: [&str; 4] = ["ar", "er", "ir", "or"];
const INFINITIVE_LOOKALIKES: [&str; 1] = ["favor"];

/// The check a course's target language adds to every spoken text, if any; the message describes the problem.
pub fn target_language_issue(target_language: &str, text: &str) -> Option<String> {
    match target_language {
        BRAZILIAN_PORTUGUESE => contains_european_portuguese(text).then(|| format!("European Portuguese form in spoken text: {text}")),
        _ => None,
    }
}

fn contains_european_portuguese(text: &str) -> bool {
    let lowercase_text: String = text.to_lowercase();
    let tokens: Vec<&str> = lowercase_text
        .split(|character: char| !character.is_alphabetic())
        .filter(|token| !token.is_empty())
        .collect();

    let stem_found: bool = tokens
        .iter()
        .any(|token| EUROPEAN_PORTUGUESE_STEMS.iter().any(|stem| token.starts_with(stem)));
    let word_found: bool = tokens.iter().any(|token| EUROPEAN_PORTUGUESE_WORDS.contains(token));
    let padded_tokens: String = format!(" {} ", tokens.join(" "));
    let phrase_found: bool = EUROPEAN_PORTUGUESE_PHRASES
        .iter()
        .any(|phrase| padded_tokens.contains(&format!(" {phrase} ")));

    stem_found || word_found || phrase_found || contains_european_progressive(&tokens)
}

fn contains_european_progressive(tokens: &[&str]) -> bool {
    tokens.windows(3).any(|window| {
        let [auxiliary, preposition, verb]: [&str; 3] = [window[0], window[1], window[2]];
        let long_infinitive: bool = verb.chars().count() > 3
            && INFINITIVE_ENDINGS.iter().any(|ending| verb.ends_with(ending))
            && !INFINITIVE_LOOKALIKES.contains(&verb);
        let is_infinitive: bool = long_infinitive || SHORT_INFINITIVES.contains(&verb);

        EUROPEAN_PROGRESSIVE_AUXILIARIES.contains(&auxiliary) && preposition == "a" && is_infinitive
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn european_portuguese_detects_stems_words_phrases_and_progressive() {
        assert!(contains_european_portuguese("Vou de autocarro."));
        assert!(contains_european_portuguese("O comboio chegou."));
        assert!(contains_european_portuguese("Onde fica a casa de banho?"));
        assert!(contains_european_portuguese("Estou a fazer o jantar."));
        assert!(contains_european_portuguese("Tu estás cansado?"));
        assert!(contains_european_portuguese("Ela está a cozinhar."));
        assert!(contains_european_portuguese("Eles estavam a ver televisão."));
        assert!(contains_european_portuguese("Estou a ler um livro."));
    }

    #[test]
    fn european_portuguese_accepts_brazilian_forms() {
        assert!(!contains_european_portuguese("Vou de ônibus."));
        assert!(!contains_european_portuguese("Estou fazendo o jantar."));
        assert!(!contains_european_portuguese("Estou a favor da ideia."));
        assert!(!contains_european_portuguese("Ele está a par de tudo."));
        assert!(!contains_european_portuguese("Onde fica o banheiro?"));
    }
}
