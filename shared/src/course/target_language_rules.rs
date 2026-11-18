use crate::course;
use crate::course::stress_mark;

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
const VOSOTROS_WORDS: [&str; 11] = [
    "vosotros", "vosotras", "os", "vuestro", "vuestra", "vuestros", "vuestras", "sois", "vais", "dais", "veis",
];
const VOSOTROS_ENDINGS: [&str; 5] = ["áis", "éis", "íais", "abais", "steis"];
const VOSOTROS_ENDING_LOOKALIKES: [&str; 6] = ["dieciséis", "veintiséis", "samuráis", "bonsáis", "jerséis", "espráis"];
/* Words whose ß follows a short vowel, which the 1996 reform spells ss. */
const PRE_REFORM_GERMAN_WORDS: [&str; 39] = [
    "daß", "muß", "mußt", "mußte", "mußten", "mußtest", "müßt", "wußte", "wußten", "wußtest", "gewußt", "wißt", "gewiß",
    "läßt", "laß", "paßt", "ißt", "vergißt", "bißchen", "küßt", "faßt", "haßt", "schluß", "fluß", "kuß", "nuß", "schloß",
    "naß", "haß", "paß", "faß", "streß", "prozeß", "kompromiß", "imbiß", "biß", "riß", "goß", "genoß",
];
const PRE_REFORM_GERMAN_STEMS: [&str; 3] = ["miß", "bewuß", "unbewuß"];
const RUSSIAN_SPELLING: CyrillicSpelling = CyrillicSpelling {
    language_name: "Russian",
    vowels: "аеёиоуыэюя",
    other_language_name: "Ukrainian",
    other_language_letters: "іїєґ",
};
const UKRAINIAN_SPELLING: CyrillicSpelling = CyrillicSpelling {
    language_name: "Ukrainian",
    vowels: "аеєиіїоуюя",
    other_language_name: "Russian",
    other_language_letters: "ыэъё",
};
const WORD_APOSTROPHES: &str = "'’ʼ";

struct CyrillicSpelling {
    language_name: &'static str,
    /// Lowercase.
    vowels: &'static str,
    other_language_name: &'static str,
    /// Lowercase letters of the other language's alphabet that this language's alphabet lacks.
    other_language_letters: &'static str,
}

/// The check a course's target language adds to every spoken text, if any; the message describes the problem.
pub fn target_language_issue(target_language: &str, text: &str) -> Option<String> {
    match target_language {
        course::BRAZILIAN_PORTUGUESE_LANGUAGE_TAG => contains_european_portuguese(text).then(|| format!("European Portuguese form in spoken text: {text}")),
        course::MEXICAN_SPANISH_LANGUAGE_TAG => contains_vosotros_form(text).then(|| format!("Spain's vosotros form in Mexican Spanish spoken text: {text}")),
        course::GERMAN_LANGUAGE_TAG => contains_pre_reform_german(text).then(|| format!("pre-1996 German spelling in spoken text: {text}")),
        course::RUSSIAN_LANGUAGE_TAG => cyrillic_spelling_issue(&RUSSIAN_SPELLING, text).map(|problem| format!("{problem}: {text}")),
        course::UKRAINIAN_LANGUAGE_TAG => cyrillic_spelling_issue(&UKRAINIAN_SPELLING, text).map(|problem| format!("{problem}: {text}")),
        _ => None,
    }
}

fn lowercase_composed(text: &str) -> String {
    stress_mark::compose(text).to_lowercase()
}

fn words(text: &str) -> Vec<&str> {
    text.split(|character: char| !character.is_alphabetic()).filter(|word| !word.is_empty()).collect()
}

fn contains_european_portuguese(text: &str) -> bool {
    let lowercase_text: String = lowercase_composed(text);
    let tokens: Vec<&str> = words(&lowercase_text);

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

fn contains_vosotros_form(text: &str) -> bool {
    let lowercase_text: String = lowercase_composed(text);

    words(&lowercase_text).into_iter().any(|word| {
        let vosotros_ending: bool = VOSOTROS_ENDINGS.iter().any(|ending| word.ends_with(ending)) && !VOSOTROS_ENDING_LOOKALIKES.contains(&word);

        VOSOTROS_WORDS.contains(&word) || vosotros_ending
    })
}

fn contains_pre_reform_german(text: &str) -> bool {
    let lowercase_text: String = lowercase_composed(text);

    words(&lowercase_text).into_iter().any(|word| {
        PRE_REFORM_GERMAN_WORDS.contains(&word) || PRE_REFORM_GERMAN_STEMS.iter().any(|stem| word.starts_with(stem))
    })
}

/// The spelling problem of `text`, if any, described without the text.
fn cyrillic_spelling_issue(spelling: &CyrillicSpelling, text: &str) -> Option<String> {
    let lowercase_text: String = lowercase_composed(text);
    let other_language_letter_found: bool = lowercase_text.contains(|character: char| spelling.other_language_letters.contains(character));

    if other_language_letter_found {
        return Some(format!("{} letter in {} spoken text", spelling.other_language_name, spelling.language_name));
    }

    /* Read as authored, since composition joins an acute accent with a consonant that has a precomposed form. */
    let stress_marked_words: Vec<&str> = text
        .split(|character: char| !(character.is_alphabetic() || character == stress_mark::STRESS_MARK || WORD_APOSTROPHES.contains(character)))
        .filter(|word| !word.is_empty())
        .collect();

    if stress_marked_words.iter().any(|word| mixes_cyrillic_and_latin(word)) {
        return Some("a word mixes Cyrillic and Latin letters".to_string());
    }
    if stress_marked_words.iter().any(|word| has_stress_mark_after_a_non_vowel(word, spelling.vowels)) {
        return Some("a stress mark follows something other than a vowel".to_string());
    }
    if stress_marked_words.iter().any(|word| word.matches(stress_mark::STRESS_MARK).count() > 1) {
        return Some("a word has more than one stress mark".to_string());
    }

    None
}

fn mixes_cyrillic_and_latin(word: &str) -> bool {
    let has_cyrillic: bool = word.chars().any(|character| ('\u{0400}'..='\u{04ff}').contains(&character));
    let has_latin: bool = word.chars().any(|character| character.is_alphabetic() && character <= '\u{024f}');

    has_cyrillic && has_latin
}

fn has_stress_mark_after_a_non_vowel(word: &str, vowels: &str) -> bool {
    let characters: Vec<char> = word.chars().collect();

    characters.iter().enumerate().any(|(position, character)| {
        let follows_a_vowel: bool = position > 0 && characters[position - 1].to_lowercase().all(|lowercase| vowels.contains(lowercase));

        *character == stress_mark::STRESS_MARK && !follows_a_vowel
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

    #[test]
    fn vosotros_detects_pronouns_possessives_and_verb_forms() {
        assert!(contains_vosotros_form("¿Vosotros sois de aquí?"));
        assert!(contains_vosotros_form("Os veo mañana."));
        assert!(contains_vosotros_form("¿Es vuestra casa?"));
        assert!(contains_vosotros_form("¿Habláis español?"));
        assert!(contains_vosotros_form("¿Qué coméis?"));
        assert!(contains_vosotros_form("¿Adónde vais?"));
        assert!(contains_vosotros_form("Vivíais en Madrid."));
        assert!(contains_vosotros_form("Estabais cansados."));
        assert!(contains_vosotros_form("¿Llegasteis tarde?"));
    }

    #[test]
    fn vosotros_accepts_ustedes_and_words_that_end_like_vosotros_forms() {
        assert!(!contains_vosotros_form("¿Ustedes son de aquí?"));
        assert!(!contains_vosotros_form("¿Hablan español?"));
        assert!(!contains_vosotros_form("Tengo dieciséis años y mi hermana veintiséis."));
        assert!(!contains_vosotros_form("Son seis samuráis."));
    }

    #[test]
    fn pre_reform_german_detects_sharp_s_after_a_short_vowel() {
        assert!(contains_pre_reform_german("Ich weiß, daß du kommst."));
        assert!(contains_pre_reform_german("Er muß jetzt gehen."));
        assert!(contains_pre_reform_german("Das war ein Mißverständnis."));
        assert!(contains_pre_reform_german("Ich bin mir dessen bewußt."));
    }

    #[test]
    fn pre_reform_german_accepts_sharp_s_after_a_long_vowel_or_diphthong() {
        assert!(!contains_pre_reform_german("Ich weiß, dass du kommst."));
        assert!(!contains_pre_reform_german("Die Straße ist groß, und das macht Spaß."));
        assert!(!contains_pre_reform_german("Er beißt in den Apfel."));
    }

    #[test]
    fn cyrillic_spelling_rejects_the_other_language_letters() {
        assert!(cyrillic_spelling_issue(&RUSSIAN_SPELLING, "Він іде додому.").is_some());
        assert!(cyrillic_spelling_issue(&UKRAINIAN_SPELLING, "Ты идёшь домой.").is_some());
        assert_eq!(cyrillic_spelling_issue(&RUSSIAN_SPELLING, "Ты идёшь домой."), None);
        assert_eq!(cyrillic_spelling_issue(&UKRAINIAN_SPELLING, "Він іде додому, а сім'я вдома."), None);
    }

    #[test]
    fn cyrillic_spelling_rejects_a_word_mixing_cyrillic_and_latin_letters() {
        assert!(cyrillic_spelling_issue(&RUSSIAN_SPELLING, "Это молокó.").is_some());
        assert_eq!(cyrillic_spelling_issue(&RUSSIAN_SPELLING, "Это мой iPhone."), None);
    }

    #[test]
    fn cyrillic_spelling_accepts_one_stress_mark_on_a_vowel_per_word() {
        assert_eq!(cyrillic_spelling_issue(&RUSSIAN_SPELLING, "Э\u{0301}то молоко\u{0301}."), None);
        assert_eq!(cyrillic_spelling_issue(&UKRAINIAN_SPELLING, "Мені\u{0301} подобається."), None);
    }

    #[test]
    fn cyrillic_spelling_rejects_a_misplaced_or_second_stress_mark() {
        assert!(cyrillic_spelling_issue(&RUSSIAN_SPELLING, "молок\u{0301}о").is_some());
        assert!(cyrillic_spelling_issue(&RUSSIAN_SPELLING, "\u{0301}молоко").is_some());
        assert!(cyrillic_spelling_issue(&RUSSIAN_SPELLING, "мо\u{0301}локо\u{0301}").is_some());
    }
}
