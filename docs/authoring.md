# Course authoring guide

How to write a good language course for this platform. The first sections apply to every course, whatever language is taught and whatever language it is taught in. Rules that belong to one course or one language are in "Course notes" at the end.

## Course layout

A course lives in `data/courses/<course id>/`. A course id is `<target language>.<source language>.<curriculum name>`, for example `pt_br.en.foundations`.

- `curriculum.json` names the course id, the title, the `target_language` taught and the `source_language` it is taught in (BCP 47 tags such as `pt-BR` and `en`), the levels (each with `code`, `title` and `requires_readings`), and the units with their ids, levels, titles, goals, grammar focus and vocabulary themes. Follow them.
- Each unit is one JSON file at `units/<unit-id>.json` in the course directory.
- `"test_course": true` marks a course that exists to test the apps. A build includes it only with `--include-test-courses`.
- Check a unit with `cargo run --package course_builder -- validate --course <course id> <unit-id>` and fix every error before finishing.

Fields named `target` hold text in the language taught. Fields named `source` hold text in the language the course is taught in. `source_literal` is the target text rendered word for word in the source language. Everywhere below, "target language" and "source language" mean these two roles, and examples are from `pt_br.en.foundations`.

## Content rules

These rules come from the course owner's feedback on the content. They apply to every text a learner reads: grammar notes, tables, culture text, vocabulary notes, exercise prompts and explanations, objectives and dialogue contexts.

1. Content feedback is handled in three steps, and this rule comes before all others.
   - Address the feedback given.
   - Ask whether it generalizes. If it does, write the generalization here as a numbered rule, in terms that fit any course.
   - Sweep the whole course for other instances of the same fault and fix them.
2. Describe a word, form or construction as what it is. Never place it in a category it does not belong to for convenience, in a list, table, table header, vocabulary `part_of_speech` or sentence.
   - A word that behaves like a member of a category without being one is explained in prose, with its real category and how it behaves like the other, and is kept out of that category's lists, tables and headers.
   - The course notes give the cases that came up.
3. The reader is assumed to know only the source language. Never ask the reader to compare with, recall or apply any other language, including languages related to the target language, and never use another language's terms without explaining them. A fact about another language is stated in full so that it stands on its own, and only when it helps the learner.
4. When the text mentions a concept, it explains it usefully: what it is in plain terms and an example with its natural translation, and its literal rendering where the structures differ, in its own paragraph or list. A sentence that only names a concept, such as one closing a paragraph about something else, is either expanded this way or removed.
5. When the text lists things, especially things to memorize, it writes a list, not prose.
   - A list is a block of lines that each start with `- `, with a blank line before and after the block. One item per line, no nesting, and no other markup. A sentence that introduces the list ends the paragraph before it.
   - Lists work in grammar `explanation` and culture `text`. Other text fields stay single-line plain text.
6. Never put a source-language article or determiner on a target-language word or phrase. Each language keeps its own slots, so a sentence never has to be parsed across a mix of the two.
   - Otherwise the reader cannot tell whether `a 'frase'` is a target-language word or a source-language word they do not know.
   - A target word is quoted by itself, with its own article where the target language marks nouns that way (`'a frase'`), or named by a source-language noun that says what it is: `the word 'a frase'`, `the form 'fala'`, `the phrase 'o posto de saúde'`, `the tense called 'pretérito perfeito'`.
   - A gloss pair keeps one language per side: `sentence: 'a frase'`, not `sentence: a 'frase' in Portuguese`.
   - A sentence that defines a target word puts it where no source-language article is needed: `'A feira' is an open-air street market` or `The word for an open-air street market is 'a feira'`, not `An open-air street market is a 'feira'`.
   - This covers words, phrases and forms. Letters, sound spellings and endings are named by their spelling (`the 'ch' is [ʃ]`, `the '-o' of 'isso'`).
7. Every text in either language is grammatically correct, complete and professional, as a carefully edited textbook would print it.
   - A sentence names its subject; no noun is left for the reader to supply (`The six most common prepositions are:`, not `The six commonest are:`).
   - Standard modern forms only: no archaic, regional or informal forms in source-language text (`most common`, not `commonest`), and no slang, abbreviations or chat register.

## Text rules

- Every `target` string is correct, natural and fully spelled, with every accent, mark and character the writing system requires. Re-read everything once for spelling, agreement and conjugation errors before you finish.
- Every `target` string is spoken by a text-to-speech engine, so it is plain speakable text.
  - No slashes: write two separate items instead of `obrigado/obrigada`.
  - No parentheses, markdown, symbols, em or en dashes, or `___`.
  - Write numbers as words when they are short.
  - Keep each string under about 200 characters.
- Source-language text (`source`, `explanation`, `prompt`, `prompt_source`, `hint_source`, `context`, `context_source`, `note`) is plain natural language. Explanations and culture text may contain `\n\n` to separate paragraphs and lists (content rule 5). Plain text only, no other markdown syntax.
- Use natural, current speech in dialogues, inside the level. Dialogues sound like real speakers, not textbook robots. Keep grammar notes honest about formal versus colloquial usage.
- Difficulty matches the level.
  - The lowest level uses very short sentences and only the grammar covered so far: the current unit's focus plus all earlier units in the curriculum.
  - Higher levels use longer, richer text and the grammar the curriculum has introduced.
  - Recycle vocabulary and grammar from earlier units, but each unit introduces its own new vocabulary and never requires grammar from later units.
- A target language can add its own checks in `shared/src/course/target_language_rules.rs`, and a new target language needs speech voices in `tools/course_builder/src/speech_engine/speech_language.rs`.

## Principles first

The learner wants to understand the language as a system before memorizing words. Vocabulary is secondary: it is easy to add to a firm foundation.

- Teach the rule, then the examples. Every grammar note states the underlying principle (what the pattern is, why it exists, where it stops working) before any list of examples, in plain source-language terms, with the technical term defined the first time it is used.
- Nothing is used before it is explained. Every letter, sound, mark, grammatical term, form and construction that appears in a `target` string, example or exercise has been explained in an earlier unit or in this one.
  - When a unit introduces a word with a mark, sound or spelling the learner has not met, a note in the same unit explains why it is written and pronounced that way, using the rules from the opening units.
  - A word is never written with a mark the reader cannot yet read.
- Prefer a small, well-chosen vocabulary that serves the grammar over long thematic lists: 12 to 20 items is enough, chosen to illustrate the patterns being taught.
- Explain in terms of the structure of the target language, with contrasts to the source language: what the source language does with word order or auxiliary words, and what the target language does with endings, agreement or contractions.
- Exercises test the principles (apply the rule, choose the form, explain the spelling, place the stress) at least as much as they test recall of words.

## Translation versus usage

The learner wants three separate things, never blended into one gloss: the literal meaning, the natural translation, and how the expression is used.

- `source` is the natural equivalent in the source language, short and clean, with no parenthetical usage notes. For example `"source": "excuse me"`.
- `source_literal` is the word-for-word rendering that follows the target language's structure and word order, so the learner sees what the words actually say.
  - Join the source-language words that stand for one target word with a hyphen, and show structural differences honestly. For example `"with license"` for `com licença`, `"I-am with hunger"` for `estou com fome`, `"I have 30 years"` for `tenho trinta anos`, `"all well?"` for `tudo bem?`.
  - Add it to every vocabulary item, dialogue line, grammar example and reading paragraph where the natural translation differs significantly from the word-for-word rendering: idioms, fixed phrases, constructions that use a different structure, and false friends.
  - Omit it when the natural translation is already essentially word-for-word, for example `o livro`, `the book`.
  - Never fabricate a literal meaning. It is a faithful word-level gloss, and when a word is itself opaque, its parts are glossed as far as they are transparent.
- `note` and the lesson text explain the usage: when and to whom you say it, register, regional differences, and why the literal and natural meanings diverge (the etymology or logic behind the idiom).

The `source_literal` field is allowed on vocabulary items, dialogue lines and every phrase pair (grammar examples, reading paragraphs, match pairs). It is not spoken.

## File format

```json
{
  "id": "a1-01-greetings-introductions",
  "level": "A1",
  "order": 1,
  "title": "Greetings and introductions",
  "objectives": ["Three to five 'I can...' style goals as full source-language sentences"],
  "vocabulary": [
    {
      "target": "olá",
      "source": "hello",
      "source_literal": "optional word-for-word gloss, only when it differs significantly",
      "part_of_speech": "interjection",
      "note": "optional short usage note",
      "example": {"target": "Olá, tudo bem?", "source": "Hello, how are you?"}
    }
  ],
  "dialogues": [
    {
      "id": "d1",
      "title": "Meeting at a party",
      "context": "One or two sentences setting the scene.",
      "lines": [
        {"speaker": "Ana", "gender": "f", "target": "Oi, tudo bem?", "source": "Hi, how are you?"},
        {"speaker": "Carlos", "gender": "m", "target": "Tudo bem, e você?", "source": "Fine, and you?"}
      ]
    }
  ],
  "grammar": [
    {
      "title": "Short title",
      "explanation": "Clear plain-text explanation written for a speaker of the source language. Explain the rule, the pattern, the exceptions that matter at this level and where everyday usage differs from the textbook form. Use \n\n between blocks.",
      "table": {"headers": ["Pronoun", "Form"], "rows": [["eu", "sou"], ["você", "é"]]},
      "examples": [{"target": "Eu sou brasileiro.", "source": "I am Brazilian."}]
    }
  ],
  "culture": {"title": "Short title", "text": "Two or more plain-text paragraphs about the culture or usage that relates to the unit's theme. Include a few target-language words inline in plain text if helpful."},
  "readings": [
    {
      "id": "r1",
      "title": "Title",
      "paragraphs": [{"target": "A paragraph of target-language text.", "source": "Its translation."}],
      "questions": [{"type": "multiple_choice", "prompt": "...", "choices": ["...", "...", "..."], "answer_index": 0, "explanation": "..."}]
    }
  ],
  "exercises": []
}
```

Rules for each section:

- `vocabulary`: at least 12 items; 12 to 20 is typical. Every item needs an `example` pair. `part_of_speech` is optional and names the item's real class (noun, verb, adjective, phrase, noun phrase, and so on). Where gender or class matters for nouns, include what the language needs in `target` (in Portuguese, the article: `a cidade`, `o aluguel`). For verbs use the dictionary form. Do not repeat a `target` value within the unit.
- `dialogues`: 2 to 3 dialogues, each with 6 to 14 lines, 2 to 4 distinct speakers, speaker names consistent within a dialogue and `gender` of `f` or `m` on every line matching the speaker. Dialogues use the unit's vocabulary and grammar focus in realistic situations. A dialogue id is unique within the unit (`d1`, `d2`, ...).
- `grammar`: 2 to 4 notes covering the unit's grammar focus. Each explanation is at least 250 characters and genuinely teaches (rule, pattern, exceptions, real usage). Each note has at least 4 examples. Add a `table` where a paradigm, comparison or list of forms helps (all cells are strings, every row as wide as the headers).
- Table headers on a phone: a narrow screen shows a table as labelled rows.
  - The first column leads each row. Where consecutive rows repeat their first cell, that cell becomes a group heading and the second column leads.
  - Every later header becomes an uppercase label in a column about 11 letters wide beside its value.
  - Keep each of those headers to one line: a word or two that names the column (`Sound`, `English sound`, `Literal`, `Natural`, `Syllables`, `Default`, `Why`, `Example`).
  - Two lines are acceptable only where one cannot carry the meaning (`Masculine plural`, `você, ele, ela`). The validator warns at two lines and rejects more.
  - Put explanations of a convention in the note's explanation, not in a header, and give the same column concept the same header in every unit.
- `culture`: one note of at least 200 characters.
- `readings`: optional at the lowest levels (omit the key or use `[]`), required at the higher levels as the curriculum's `requires_readings` says. A reading has at least 2 paragraphs of real connected text at the level, and at least 2 (ideally 3 to 4) `multiple_choice` comprehension questions.
- `exercises`: at least 22 besides open responses, mixed in order of increasing difficulty, with at least this many of each type: multiple_choice 5, fill_blank 5, translate_to_target 3, order_words 3, listening_choice 3, dictation 2, match 1. Aim for about 26 to 30. Cover the whole unit: vocabulary, the grammar notes and the dialogues. Open responses are optional and follow the rules under their type.

## Exercise types

```json
{"type": "multiple_choice", "prompt": "How do you say 'good evening'?", "choices": ["Boa noite", "Boa tarde", "Bom dia"], "answer_index": 1, "explanation": "optional short explanation"}
```
Prompts may be in either language. 3 to 5 unique choices. `answer_index` is the zero-based index. Vary the position of the correct answer across exercises.

```json
{"type": "fill_blank", "prompt_target": "Eu ___ brasileiro.", "answers": ["sou"], "hint_source": "I am Brazilian.", "explanation": "optional"}
```
Exactly one `___` in `prompt_target`. `answers` is an array of acceptable fillers (lowercase, no punctuation, may be more than one word, for example `["vou comer"]`). Include every genuinely acceptable alternative.

```json
{"type": "translate_to_target", "prompt_source": "I live in São Paulo.", "answers": ["Eu moro em São Paulo.", "Moro em São Paulo."], "explanation": "optional"}
```
`answers` lists every natural correct translation the learner could reasonably type, including versions with and without a subject word where both are natural. Keep the sentence short and tightly constrained by the source text so the answer set is practical.

```json
{"type": "order_words", "prompt_source": "Where do you live?", "words": ["Onde", "você", "mora", "?"], "distractors": ["moro"]}
```
`words` are the tokens in the correct order. Punctuation is attached to the preceding word (`mora?`), not a separate token, and no token contains a space. The learner rebuilds the sentence from the shuffled tokens plus the optional `distractors` (at most 3). Keep the sentence to 3 to 10 tokens, and make sure the token order is the only correct order.

```json
{"type": "listening_choice", "audio_target": "Eu moro no Rio de Janeiro.", "choices": ["I live in Rio de Janeiro.", "I work in Rio de Janeiro.", "I study in Rio de Janeiro."], "answer_index": 0}
```
The learner hears `audio_target` and picks the meaning. Distractors differ in a way that can be heard (a different key word), not in a way the audio cannot resolve.

```json
{"type": "dictation", "audio_target": "Bom dia, tudo bem?", "hint_source": "Good morning, how are you?"}
```
The learner types what they hear. Keep it short (3 to 10 words) and unambiguous in spelling.

```json
{"type": "match", "prompt": "Match the Portuguese with the English", "pairs": [{"target": "o pai", "source": "the father"}, {"target": "a mãe", "source": "the mother"}, {"target": "o irmão", "source": "the brother"}, {"target": "a irmã", "source": "the sister"}]}
```
4 to 6 pairs, all unambiguous. A phone shows the two sides in two narrow columns, so keep each side to a short phrase of at most 35 characters, which the validator enforces. A full sentence of explanation belongs in `explanation`.

```json
{"type": "open_response", "prompt_source": "Tell a friend about Paulo: what he is like and how he is today.", "context_source": "Paulo is tall and he is a doctor. Today he is tired and hungry.", "example_answers": ["O Paulo é alto e é médico. Hoje ele está cansado e com fome."], "explanation": "optional"}
```
The learner writes their own answer to the task. A language model grades it where the web app can run one; elsewhere the learner compares it with `example_answers`. The learner practises a unit's open responses in its Open response option on the Practice tab, in file order, apart from the quiz and quick practice, so an open response never counts toward the score and its position among the exercises does not matter. `context_source` holds the facts of a scene the answer must convey. Only a course whose language pair has a grader prompt (`shared/src/grading/`) can have open responses, and a reading question is never one.

- Answerable with grammar and vocabulary taught in the unit or earlier.
- The task's wording makes the unit's grammar the natural answer; never rely on naming a grammar term.
- One communicative act per task.
- Length: A1 one or two sentences, A2 up to three, B1 and B2 up to about four or 30 words.
- `context_source` whenever the content can be checked; every fact in it sayable with taught vocabulary.
- Personal tasks, about the learner's own life, take no `context_source` and never fix the writer's gender; example answers give both forms where agreement depends on it.
- Example answers use only taught structures and follow every `target` rule; more than one where several are natural.
- At most two per unit, three in consolidation and writing units; never in place of a required exercise type.
- None in a unit whose objectives are only reading or recognition.

## Quality bar

- Content is teachable from the page alone by a complete beginner who speaks only the source language. Explain with source-language comparisons where they help.
- Exercises other than open responses have exactly one defensible correct answer, or every defensible answer listed. No trick questions. Explanations on multiple_choice and fill_blank are strongly encouraged.
- No two exercises in a unit test the same sentence.
- Write valid JSON (double quotes, no trailing commas, escaped quotes inside strings as `\"`, newlines in strings as `\n`). Use real UTF-8 characters, not escapes.
- Write the file, run the validator, fix all errors, then do a last careful pass over the target-language text for mistakes.

## Course notes

Rules that belong to one language or one course. Each subsection also records the cases behind content rule 2 for that course.

### Brazilian Portuguese for English speakers (`pt_br.en`)

Language rules:

- Brazilian vocabulary, spelling and grammar throughout: `você`, `a gente`, `ônibus`, `trem`, `celular`, `café da manhã`, `banheiro`, `geladeira`, `suco`, `moça`, `fato`, `contato`. The validator rejects European forms in a `target` field.
- The progressive is `estou fazendo`, never `estou a fazer`. Colloquial object pronouns go before the verb (`me disse`, `te amo`).
- `tá`, `pra` and `né` are fine in informal dialogue at A2 and above, and sparingly at A1.
- A mention of European Portuguese as a contrast belongs only in a `note` or `explanation`, never inside a `target` field, and follows content rule 3.

Content rule 2 cases:

- `a gente` is a noun phrase (literally 'the people') that takes a third person singular verb. In speech it stands in for `nós`.
  - It is never listed among the pronouns or beside `ele`, `ela` and `você` in a table, header or list, and its vocabulary `part_of_speech` is `noun phrase`.
  - It is explained in prose where the forms of address are taught and wherever a note needs it, saying that it works like a pseudo-pronoun.
- `o senhor` and `a senhora` are noun phrases ('the gentleman', 'the lady') used to address someone, not pronouns.

Unit rules:

- A1 uses mainly the present tense, with `ir` plus an infinitive for the near future. A2 can use the past tenses once introduced. B1 and B2 use longer, richer text.
- A unit that introduces a word with an accent mark, a nasal vowel, an unusual spelling or a silent letter has a note titled "Spelling and sound: ..." explaining why it is written and pronounced that way, using the rules from the first three units. An accented word is never written without the reader knowing what the accent does.
- `source_literal` is added for constructions where Portuguese uses a different structure: `estar com`, `ter de`, `gostar de`, `ficar`, `dar`, `haver`.

### Mexican Spanish for English speakers (`es_mx.en`)

Language rules:

- Mexican Spanish throughout: Mexican vocabulary (`carro`, `celular`, `computadora`, `jugo`, `camión` for the city bus, `autobús`, `departamento`, `recámara`, `platicar`, `manejar`, `rentar`, `jitomate`, `boleto`, `playera`, `chamarra`, `la gripa`), and the Mexican forms `¿mande?`, `¿bueno?` on the phone, `ahorita` and `al rato`.
- `ustedes` is the only plural 'you' and takes the verb forms of `ellos`. Spain's informal plural pronoun, its verb forms and its object and possessive forms never appear in a `target` field, a table or an example; the validator rejects them in spoken text. Voseo is not taught.
- Pronunciation is Mexican: `c` before `e` or `i`, `z` and `s` all sound [s], and `ll` and `y` are one sound. Spain's [θ] for `c` and `z`, and any other contrast with Spain, belongs only in a `note` or `explanation`, stated in full (content rule 3).
- Spelling follows the Real Academia Española's 2010 rules: `solo` and the demonstratives without an accent, accents on question words and on capital letters, and `¿` and `¡` opening every question and exclamation. Where the Academy accepts two forms, use the Mexican one: `futbol`, `video`, `chofer`, `periodo`.
- `lo` and `la` are the direct object pronouns for people (`Lo vi`). The spoken `se los dije` for 'I told it to them' is explained in a note, and `target` text writes `se lo dije`.
- Mexican speakers use the preterite for an action completed earlier the same day (`¿Ya comiste?`) and the present perfect for experiences and for situations that continue to now; dialogues follow this.
- The imperfect subjunctive is written with `-ra` (`hablara`, `quisiera`). The `-se` forms appear only in literary readings, explained there.
- Slang (`güey`, `chido`, `padre`, `la neta`, `no manches`, `órale`) stays out of `target` fields before b2-05, and `güey` stays out of them throughout. `pues` is used from a2-13.
- `don` and `doña` go before a first name (`doña Lupe`); `señor`, `señora`, `señorita` and `joven` are used to address strangers; professional titles (`licenciado`, `ingeniero`, `maestro`) are used as forms of address.

Content rule 2 cases:

- `usted` and `ustedes` are second person pronouns: they name the listener. They take the third person forms of verbs, of object pronouns (`lo`, `la`, `le`, `se`) and of the possessive `su`.
  - A table, header or list never labels them third person. Paradigm rows are labelled by the pronouns themselves (`usted, él, ella`), and only a1-03 has a column naming person and number, with `usted` and `ustedes` left out of it.
  - Their behavior is explained in prose: they began as `vuestra merced` ('your grace'), a noun phrase spoken of in the third person.
- `gustar`, `encantar`, `interesar`, `doler`, `quedar`, `alegrar`, `molestar`, `preocupar` and `sorprender` take the thing as their subject and the person as an indirect object.
  - `me` in `me gusta` is never called a subject, and `gustar` is never conjugated in a table of subject forms. Their tables head the pronoun column `Indirect object` and give the verb in agreement with the thing (`gusta`, `gustan`).
  - The note says that 'I like coffee' is literally 'to-me pleases the coffee'.
- `señor`, `señora`, `señorita`, `joven`, `don` and `doña` are nouns used as titles or forms of address, not pronouns; the pronoun used with them is `usted`.
- `hay` is a form of `haber` used with no subject. It is not a form of 'to be' and stays out of the tables of `ser` and `estar`.
- `hace` in `hace frío` and `hace dos días` is a form of `hacer` used with no subject. It is not a preposition and not an adverb meaning 'ago'.
- Only the affirmative commands for `tú` (`habla`, `di`, `ven`) are imperative forms. The commands for `usted` and `ustedes` and every negative command are present subjunctive forms used as commands. A table may group all of them by use; its explanation says which forms are subjunctive.
- The conditional (`hablaría`) is a tense of the indicative mood, never listed as a mood. `quisiera` is an imperfect subjunctive, not a conditional, although both are polite.
- `ir a` plus an infinitive, `estar` plus a gerund, `acabar de`, `tener que`, `hay que` and `llevar` plus a gerund are verbal periphrases, not tenses. They stay out of tense tables and lists, or are labelled as periphrases.
- The gerund (`hablando`) is a verb form, not a noun and not the English '-ing' noun: 'Swimming is fun' is `Nadar es divertido`, with the infinitive. It is never called a present participle.
- `el agua`, `el águila`, `el alma`, `el hambre` and `el acta` are feminine nouns that take `el` and `un` before a stressed 'a' sound. They are never listed as masculine, and their adjectives stay feminine (`el agua fría`).
- `el día`, `el mapa`, `el clima`, `el problema`, `el idioma`, `el tema`, `el poema` and `el poeta` are masculine despite '-a'; `la mano` and `la foto` are feminine despite '-o'. Each is listed by its real gender with a note.
- The `a` in `Conozco a Diego` is the preposition `a` marking a specific person as direct object. It is not an article and is glossed 'to'.
- `se` has distinct roles, each taught in its own unit and named as what it is in every note:
  - reflexive (`se levanta`)
  - reciprocal (`se conocieron`)
  - the form `le` and `les` take before `lo`, `la`, `los`, `las` (`se lo di`)
  - passive and impersonal (`se vende`, `se dice`)
  - unplanned events (`se me olvidó`)
  - It is never called reflexive outside the first role.
- `lo` is the neuter article in `lo bueno` and `lo que`, the neuter pronoun in `no lo sé`, and the masculine object pronoun in `lo veo`: three uses of one spelling, each named as what it is.
- `ojalá` is an interjection (from Arabic, 'if God wills'), not a verb or a conjunction, although the subjunctive follows it.
- `gracias` is a plural noun, `perdón` a noun and `mucho gusto` a noun phrase. `¿mande?`, `disculpe`, `oye` and `cuídate` are command forms of verbs used as fixed phrases, and their `part_of_speech` is `verb`.
- `ahorita` is the adverb `ahora` with the diminutive suffix '-ita'. Its Mexican meanings ('right now', 'in a moment', a polite 'not now') are usage, not separate words.
- `padre` meaning 'great' is an adjective with one form for both genders (`una fiesta muy padre`); it is not the noun for 'father'.
- `muy` is only an adverb. `mucho` is an adjective that agrees (`muchas casas`) or an adverb that does not (`trabaja mucho`).

Unit rules:

- A1 uses the present indicative, `ir a` plus an infinitive, the verbs followed by an infinitive, and only the fixed phrases the syllabus lists for each unit (`quisiera`, `me encantaría`, `¿me da...?`, `disculpe`, `siga`, `dé vuelta`, `lo siento`). A2 adds each tense and structure from its own unit on: preterite (a2-01), imperfect (a2-03), object pronouns (a2-05, a2-06), the progressive (a2-07), commands (a2-08), the future and the conditional (a2-12). The present perfect starts at b1-01; the subjunctive mood, beyond commands and the listed fixed phrases, starts at b1-02. B1 and B2 use longer, richer text.
- A unit that introduces a word with a written accent, `ñ` or `ü`, an `x` or `tl` from an indigenous language, a Mexican stress the Academy also spells another way (`futbol`), or a form whose accent appears or disappears (a plural, an attached pronoun, a form for 'nosotros') has a note titled "Spelling and sound: ..." explaining why it is written and said that way with the rules of the first two units.
  - In a1-01 an accented word has a note saying that the mark shows the stressed syllable and that a1-02 explains when it is written.
  - After a1-02, an accented word is never written before the reader knows what its accent does.
- `source_literal` is added where Spanish uses another structure: `tener` plus a noun (`tengo hambre`, `tengo treinta años`), `hacer` in weather and time (`hace frío`, `hace dos días`), `gustar` and its family, reflexive and pronominal verbs (`me llamo`, `se me olvidó`), `acabar de`, `llevar` plus a gerund, and the fixed greetings (`buenos días`, `¿qué tal?`, `con permiso`).

### French of France for English speakers (`fr_fr.en`)

Language rules:

- Standard French of France throughout: 'tu' and 'vous', 'on', 'le petit déjeuner', 'le déjeuner', 'le dîner', 'soixante-dix', 'quatre-vingts', 'quatre-vingt-dix', 'le mail'. Belgian, Swiss, Canadian and African forms ('septante', 'nonante', 'le dîner' for lunch, 'le courriel') appear only in a `note` or `explanation`.
- Traditional spelling in every `target` field: 'connaître', 'il connaît', 'l’île', 'le goût', 'l’événement', 'l’oignon', 'je paie', and hyphens only below one hundred ('vingt et un', 'quatre-vingt-dix-sept', 'deux cent trois'). A note may name the 1990 reformed spelling as also correct.
- The apostrophe is ’ in all French text, also in French words quoted in explanations ('l’ami').
- A space goes before '?', '!', ':' and ';' ("Ça va ?"). In `order_words`, '?' and '!' are separate final tokens, and an elided word stays joined to the next word ('j’habite').
- Spoken 'on' for 'nous' is normal in informal dialogue from A1. Readings and formal dialogue use 'nous'.
- Informal dialogue between friends may drop 'ne' once `a1-06-etre-avoir-feelings` has taught it: sparingly at A1, freely from A2. 'Ne' stays in every reading, every dialogue that uses 'vous', every example that teaches negation and every expected answer, unless an informal exercise lists both forms.
- `target` spelling is standard even in familiar speech. Reduced pronunciations ('t’as', 'y a', 'chais pas') are described in notes, never written.
- Pronunciation is given in IPA with English comparisons where a close one exists. Never respell [y], [ø], [œ], [ʁ] or a nasal vowel in English letters, and never mark stress with capitals: French has no word stress.

Content rule 2 cases:

- 'On' is a third person singular pronoun meaning 'one', 'people' or 'someone', which in speech also means 'we'. In a table it shares the row headed 'il, elle, on', never the row of 'nous', and its literal gloss is 'one'.
- 'Il' in 'il y a', 'il faut', 'il fait beau', 'il pleut', 'il est deux heures' and 'il s’agit de' is an impersonal subject that refers to nothing. It is glossed 'it', never 'he'.
  - 'Il y a' is a phrase built from 'il', the pronoun 'y' and 'a' from 'avoir'. Its `part_of_speech` is 'phrase', and it is never listed as a verb.
  - 'Il faut' is the verb 'falloir', which has only this subject.
- 'Ce' in 'c’est' and 'ça' are demonstrative pronouns. They are kept out of tables of personal pronouns, and 'c’est' versus 'il est' is explained in prose.
- 'Est-ce que' and 'qu’est-ce que' are question phrases ('is it that', 'what is it that'), not question words. They are kept out of lists of question words.
- 'Ne... que' means 'only': it restricts and does not negate. The expletive 'ne' ('avant qu’il ne parte') negates nothing. Neither appears in a list of negations.
- 'Du', 'de la' and 'des' as partitives are articles. A note may say that 'du' is built from 'de' plus 'le', but the partitive is never called a contraction or a preposition and never glossed 'of-the'. The word 'de' in 'pas de pain' and 'beaucoup de pain' is the preposition, not a partitive article.
- 'Des' is two words with one spelling: the plural indefinite article and the contraction of 'de' plus 'les'. Each is named for what it is where it appears.
- 'Mon', 'ton' and 'son' before a feminine noun that begins with a vowel ('mon amie') are not masculine. 'Bel', 'nouvel', 'vieil' and 'cet' are masculine forms used before a vowel, not feminines.
- 'Y' and 'en' are pronouns. The pronoun 'en' and the preposition 'en' are different words. The gerund ('en parlant') is the preposition 'en' plus the present participle, so it is not listed as a separate verb form or tense.
- 'Aller' plus an infinitive ('le futur proche'), 'venir de' plus an infinitive and 'être en train de' plus an infinitive are constructions, not tenses. Tense tables list only the simple and compound tenses, and 'être en train de' is never called a progressive tense: French has none.
- 'Voici' and 'voilà' are presentatives, built from an old imperative of 'voir' and 'ici' or 'là'. Their `part_of_speech` is 'presentative', and they are never listed as verbs.
- 'Ce qui', 'ce que' and 'ce dont' are 'ce' plus a relative pronoun, taught as constructions and kept out of lists of single relative pronouns. 'Dont' is a relative pronoun, not a preposition.
- 'Chez' is a preposition meaning 'at the home of', not a noun.
- 'Personne' meaning 'nobody' is a pronoun. 'La personne' meaning 'the person' is a feminine noun.
- 'Madame' and 'Monsieur' are nouns used to address someone. The formal pronoun is 'vous'.

Unit rules:

- A1 uses the present tense, the imperative from `a1-12-places-directions` on and 'aller' plus an infinitive from `a1-15-plans-weather-leisure` on. Any other form or construction appears at A1 before its unit only in a fixed phrase listed in the syllabus ('je voudrais', 'excusez-moi', 'quel âge as-tu'), and no past tense appears at A1. 'Je voudrais' is a fixed phrase until the conditional.
- A2 adds the compound past (`a2-01-passe-compose-avoir`), the imperfect (`a2-04-imparfait`), 'venir de' and 'être en train de' (`a2-05-passe-compose-vs-imparfait`), the future (`a2-11-future`) and the conditional (`a2-12-conditional`). The subjunctive, the pluperfect and the passive do not appear at A2.
- B1 and B2 use longer, richer text. B1 adds the subjunctive, the pluperfect, the past conditional, the passive, the present participle and the gerund, and reported speech. B2 adds the future perfect and the past subjunctive. The past historic, the imperfect subjunctive and the past anterior appear only from `b2-05-literature-passe-simple` on, for recognition: in readings and in exercises that ask for the meaning or the infinitive, never in an answer the learner must write. Familiar words and 'verlan' appear in dialogue only from `b2-03-spoken-french-registers` on, and vulgar words never appear in a `target` field.
- A unit that introduces a word with an accent mark, a nasal vowel, a silent letter that changes the reading, a liaison, an elision, an aspirated 'h' or a letter group ('ill', 'gn', 'œu') has a note titled "Spelling and sound: ..." that explains it with the rules of the first three units. A word that no rule predicts is named as an exception ('la femme', 'le fils', 'monsieur', 'second', 'la poêle', 'août'). French marks never show stress, so the note says what the mark does to the sound, which lost letter it records, or which word it tells apart.
- `source_literal` is added for constructions where French uses a different structure: 'avoir' plus a noun ('j’ai faim'), 'il y a', 'il faut', 'il fait' for weather, pronominal verbs, the two-part negation ('ne... pas' glossed 'not ... not'), object pronouns before the verb, 'est-ce que', 'depuis' plus the present, 'venir de', 'manquer à', 'faire' plus an infinitive and 'chez'.

### German of Germany for English speakers (`de_de.en`)

Language rules:

- Standard German of Germany in the reformed spelling (1996, revised 2006): 'ss' after a short vowel ('dass', 'muss', 'Fluss'), 'ß' after a long vowel or a diphthong ('Straße', 'groß', 'heißen'). The validator rejects pre-reform spellings in a `target` field.
- Spellings to use where the rules allow two forms or the reform changed the old one: 'kennenlernen', 'infrage', 'zurzeit', 'sodass', 'es tut mir leid', 'Rad fahren', 'nach Hause', 'zu Hause', 'heute Abend', 'tschüss'.
- Words of Germany: 'Brötchen', 'Kartoffel', 'Januar', 'Sahne', 'Fahrrad', 'parken', 'Samstag'. Austrian and Swiss forms, and the Swiss 'ss' for 'ß', belong only in a `note` or `explanation` and follow content rule 3.
- The compound past of 'stehen', 'liegen', 'sitzen' and 'hängen' takes 'haben'; the southern 'ist gestanden' belongs only in a note.
- 'du' and 'ihr' for family, friends, children and peers; 'Sie' for strangers, officials and service, always capitalized with 'Ihnen' and 'Ihr'. Every dialogue context says which one the speakers use.
- `target` text carries no stress marks, no German quotation marks and no abbreviations. Numbers up to a hundred, clock times and prices are words.
- 'hab', 'gibt's', 'nee' and 'ne?' are fine in informal dialogue at A2 and above; 'Wie geht's?' from A1.
- A noun in `target` is cited with its article ('der Tisch'), and its note gives the plural.

Content rule 2 cases:

- The umlaut ('ä', 'ö', 'ü') is part of a vowel letter, not an accent or stress mark, and 'ß' is a letter of its own, not 'double s' or a 'B'.
- 'möchte' is the subjunctive II of 'mögen'. It is never listed as an infinitive or as a modal verb of its own; a table shows it as a form of 'mögen'.
- 'würde' plus an infinitive is the subjunctive II of 'werden'. German has no conditional tense or mood, and no header or sentence calls it one.
- 'Sie' is a pronoun of address that takes the third person plural verb. It shares the row 'sie, Sie' in paradigms and is never given a verb form or person of its own.
- 'man' is an indefinite pronoun with a third person singular verb, literally 'one'. It is not listed with the personal pronouns, and it is not a passive.
- 'es' in 'es regnet' and 'es gibt' is a placeholder subject that refers to nothing. 'es' in 'Es kommen viele Gäste' is not the subject: it only fills the first position, and the verb agrees with 'Gäste'. Both are explained apart from 'es' the neuter pronoun.
- 'kein' is the negative article and declines like 'ein'; it is kept apart from 'nicht', an adverb. 'mein', 'dein' and the other possessives before a noun are possessive articles, not pronouns.
- 'im', 'am', 'zum', 'zur', 'beim', 'vom', 'ins' and 'ans' are a preposition fused with an article. They appear in tables of contractions, never in lists of prepositions.
- A separable prefix ('an', 'auf', 'mit', 'zu') is part of the verb even when it is spelled like a preposition, and is never shown in a preposition table. 'zu' before an infinitive is an infinitive marker, not the preposition, and 'um' in 'um ... zu' is not the preposition of time.
- One spelling, several classes: 'während' and 'bis' are prepositions and conjunctions; 'damit' is a 'da' word ('with it') and a conjunction ('so that'); 'denn' is a conjunction ('because') and a modal particle; 'laut' is an adjective ('loud') and a preposition ('according to'). Each use is taught as its own word.
- 'darauf', 'damit' and the other 'da' words, and 'worauf' and the other 'wo' words, are adverbs formed from a preposition. They are not prepositions and not pronouns.
- 'doch', 'ja', 'mal', 'denn', 'eben', 'halt', 'schon', 'eigentlich', 'wohl' and 'bloß' used as modal particles have the `part_of_speech` particle. They are explained by sentence type, not glossed as the adverb or answer word they are spelled like.
- 'gern' is an adverb ('gladly'), not a verb meaning 'to like'; 'Ich spiele gern' is 'I play gladly'.
- 'Deutsche', 'der Bekannte' and 'der Erwachsene' are adjectives used as nouns and keep adjective endings ('ein Deutscher', 'die Deutschen'). They are not listed with ordinary nouns of nationality such as 'der Amerikaner', and 'deutsch' the adjective is kept apart from 'Deutsch' the language.
- 'Entschuldigung' is a noun used as an apology, capitalized, with the `part_of_speech` noun.
- 'frühstücken' is an inseparable verb formed from 'das Frühstück', although its first syllable is stressed; it is never listed with the separable verbs.
- The compound past is not the English present perfect, and the simple past is not an imperfect. Notes name them by what they are and describe their use: the compound past is the past of speech, the simple past is the past of writing.
- Grammatical gender is not sex: 'das Mädchen' is neuter because of '-chen'. A gender rule is stated as a rule about the word's ending or class.

Unit rules:

- A1 uses the present tense, with a time word for the future, the modal verbs from a1-11, the 'Sie' imperative from a1-12, and main clauses joined by 'und', 'aber', 'oder', 'denn' and 'sondern'. 'möchte' and 'hätte gern' are polite forms taught as such. No past tenses and no subordinate clauses.
- A2 adds the compound past (a2-01), the simple past of 'sein', 'haben' and the modal verbs (a2-03), the full imperative (a2-07), subordinate clauses (a2-09), and the future with 'werden' and the polite 'würde', 'könnte' and 'wäre' (a2-11). Other verbs appear in the simple past only in 'es gab'.
- B1 adds the simple past of every verb (b1-02), the genitive (b1-03), relative clauses (b1-04), subjunctive II (b1-05), infinitive clauses (b1-06), the pluperfect (b1-07) and the passive (b1-08), and uses longer connected text.
- B2 adds subjunctive I, subjunctive II in the past, the state passive and its alternatives, extended attributes, the future perfect and modal verbs of inference.
- A unit that introduces a word with an umlaut, 'ß', 'ch', 'sp' or 'st', a final 'b', 'd' or 'g', '-ig', a spelling that marks vowel length, a stress off the first syllable of the stem, or a foreign spelling ('Restaurant', 'Chef', 'Cousine') has a note titled "Spelling and sound: ..." explaining it with the rules of the first three units. Where spelling does not decide a vowel's length, the note says so and gives the length.
- `source_literal` is added for 'es gibt', 'gern', 'haben' with 'Hunger', 'Durst', 'Lust' or 'Angst', the dative of feeling ('Mir ist kalt'), 'gefallen', 'wehtun', a separable verb split by the verb bracket, the verb at the end of a subordinate clause, compounds (glossed by their parts) and idioms.

### Russian for English speakers (`ru.en`)

Language rules:

- Standard contemporary Russian, as spoken and written in Moscow.
- Every word of two or more syllables carries one stress mark, wherever Russian appears: the combining acute accent U+0301 typed after the stressed vowel ('молоко́'), never a precomposed Latin letter. A word with 'ё' carries none, and 'ё' is always written, never replaced by 'е'. One-syllable words carry none, except a one-syllable word that takes the stress of the next word ('не́ был', 'не́ было'). A hyphenated word has one mark ('кто́-то'). The unstressed prepositions 'обо', 'ото', 'изо', and 'надо' and 'передо' before 'мной', carry none. Endings, prefixes and letters named by their spelling carry none ('-ого').
- 'ты' for family, friends, children and peers in informal settings; 'вы' for one person who is not close, anyone older or senior, service and official situations, and more than one person. Formal 'вы' goes with the first name and patronymic.
- A `target` field never contains the dash written Russian puts between a noun subject and a noun predicate; such sentences use a pronoun subject, 'э́то' or another structure. No quotation marks in `target`; titles are written plainly.
- Modern spelling only. Pre-1918 letters appear only when a culture note shows them as history, never in `target`; Ukrainian letters never appear.
- Colloquial forms ('щас', 'чё') only in the dialogues and notes of the B2 unit on spoken Russian.
- No open responses until `shared/src/grading/` has a grader prompt for this pair.

Content rule 2 cases:

- 'э́то' in 'Э́то кни́га' is a pronoun used as the subject; the sentence has no verb. It is never glossed or tabled as a form of 'to be', and it is taught apart from 'э́тот', 'э́та', 'э́то', 'э́ти', which agree with a noun.
- The written dash between two nouns is punctuation, not a word for 'is'.
- 'есть' in 'У меня́ есть' is the present form of 'быть' that asserts existence. It is not a copula and never fills a present paradigm of 'быть'. 'есть', to eat, is a different verb.
- 'у меня́ есть' is a preposition, a pronoun in the genitive and 'есть'; it is never listed as a verb 'to have' or conjugated in a table. Its `part_of_speech` is `phrase`.
- 'нет' with the genitive ('нет маши́ны') means 'there is no' and is a different word from 'нет', no.
- In 'Мне нра́вится Москва́' the thing liked is the subject and the verb agrees with it; the person is in the dative. A table never sets out 'мне нра́вится', 'тебе́ нра́вится' as a conjugation of a verb 'to like'. 'мне хо́чется' is the impersonal verb 'хоте́ться' with the dative, and it is never set out as a conjugation of 'хоте́ть'.
- 'на́до', 'ну́жно', 'мо́жно', 'нельзя́', 'хо́лодно', 'ску́чно' and their kind are predicatives: words that serve as the predicate of a sentence with no subject. Some, such as 'хо́лодно', are also adverbs in other sentences. Predicatives are not verbs and are kept out of verb tables. Their `part_of_speech` is `predicative`.
- 'до́лжен', 'ну́жен', 'рад', 'бо́лен', 'прав', 'согла́сен', 'жена́т', 'похо́ж' are short adjectives that agree in gender and number. 'до́лжен' translates 'must' but is never listed among verbs.
- 'бы' is a particle. The conditional is the past form with 'бы'; 'бы' is never an auxiliary verb in a tense table.
- 'идти́' and 'ходи́ть', 'е́хать' and 'е́здить' are two imperfective verbs (unidirectional and multidirectional), not aspect partners. 'пойти́' is a perfective meaning to set off.
- After 2, 3 and 4 the noun is in the genitive singular ('два бра́та'); it is never called a plural.
- Nouns with adjective endings ('ва́нная', 'моро́женое', 'учёный', 'знако́мый', 'существи́тельное', 'уча́щийся') are nouns: `part_of_speech` gives the noun and its gender, and the note says it declines like an adjective. 'ру́сский' is an adjective, and a noun when it means a Russian man.
- Masculine nouns in '-а' or '-я' ('па́па', 'дя́дя', 'де́душка', 'мужчи́на') are masculine and decline like feminine nouns. They are never placed under feminine in a gender table. Nouns of common gender ('колле́га', 'у́мница') agree with the person's sex.
- 'его́', 'её' and 'их' meaning his, her and their are genitive forms of 'он', 'она́' and 'они́' and never change. They are never set in the agreement table of 'мой', 'твой', 'наш', 'ваш'.
- 'вы' for one person is the second person plural used politely: the verb and short adjectives are plural. A header or table never calls it a formal singular pronoun.
- Verbs with '-ся' are called reflexive verbs in grammar, but many have no reflexive meaning ('учи́ться', 'нра́виться', 'боя́ться'). The course calls them verbs with '-ся' and names a reflexive meaning only where there is one.
- A verbal adverb ('дееприча́стие') is never called a gerund. A participle ('прича́стие') is a verb form with adjective endings, not an adjective; words that have become adjectives or nouns ('бу́дущий', 'люби́мый', 'уважа́емый', 'веду́щий') are described as such, with their origin in the note.
- 'дава́й' and 'дава́йте' before a verb are the imperative of 'дава́ть' used to make a suggestion; 'пусть' is a particle. Neither is a separate mood.
- Ordinal numbers ('пе́рвый') are numerals with adjective endings; 'ты́сяча' and 'миллио́н' are numerals that decline as nouns; 'мно́го', 'ма́ло', 'ско́лько' and 'не́сколько' are numerals of indefinite quantity, take the genitive and do not agree. None of them is listed among adjectives.
- 'у́тром', 'ве́чером', 'зимо́й' and 'ле́том' are adverbs formed from the instrumental, not the instrumental of a noun in the sentence.
- 'ли', 'же', 'ведь', 'ра́зве', 'неуже́ли', 'не' and 'ни' are particles. 'ли' in an indirect question translates 'whether' or 'if' and is never confused with 'е́сли'.
- 'ничего́' is the genitive of 'ничто́' used as the ordinary word for nothing. 'не́кого', 'не́чего' and 'не́когда' with an infinitive are a different construction from 'никто́', 'ничего́' and 'никогда́'.
- 'здра́вствуйте', 'извини́те', 'скажи́те' and 'да́йте' in A1 phrases are imperatives; their `part_of_speech` is `phrase` and the note names the verb.
- 'по-ру́сски' is an adverb; the language is 'ру́сский язы́к'.

Unit rules:

- A1 uses the present tense, the six cases in the singular and the nominative plural. Imperatives, perfectives, the past and the future appear only as fixed phrases, each with a note.
- A2 adds, unit by unit, the past, aspect, the future, the imperative, the genitive plural, the plural in every case and the verbs of motion. No participles or verbal adverbs, except words that are now plain adjectives or adverbs.
- B1 adds aspect under negation and in the infinitive, 'кото́рый', the conditional, 'что́бы', reported speech, negative and indefinite pronouns, and numbers in every case. Participles and verbal adverbs still appear only as plain adjectives and adverbs.
- B2 adds participles, the passive, verbal adverbs and literary and formal style, and may use everything.
- The first five units teach the letters and sounds in order. In units 1 to 4 every `target` string, choice and dictation uses only the letters taught so far, and dialogue speakers are limited to the cast names those letters can spell.
- From unit 6 on, every unit has a note titled "Spelling and sound: ..." that applies the rules of the first five units to its new words: stress and reduction, softness, voicing and devoicing, the spelling rules, and the words read differently from their spelling ('его́', 'что', silent consonants). It names each new word whose pronunciation the rules do not predict, such as a hard consonant before 'е' in a loanword ('кафе́', 'те́ннис') or a stress that moves in other forms ('го́род', 'города́').
- `source_literal` is added for 'у меня́ есть' and 'у меня́ нет', the dative of liking, needing, age and states, 'меня́ зову́т', impersonal sentences, 'идёт дождь', verbs whose case differs from English, and every fixed phrase. It is not added only because Russian lacks articles or a present 'is'.

### Ukrainian for English speakers (`uk.en`)

Language rules:

- Standard contemporary Ukrainian in the 2019 orthography. No Russian words, calques or surzhyk in any field; the validator rejects 'ы', 'э', 'ъ' and 'ё' in a `target` field.
  - An incorrect form is never quoted, not even as a warning; a note gives only the standard form ('бра́ти у́часть', 'протя́гом').
  - A culture text that needs Russian states the facts without Russian words and follows content rule 3.
- Stress: every word of two or more syllables carries one combining acute accent (U+0301) directly after its stressed vowel, in every field and in Ukrainian quoted in source-language text ('молоко́', 'Окса́на'). One-syllable words carry none ('кіт', 'Львів'). The apps ignore the mark in typed answers and the speech engines never see it, so words told apart only by stress ('за́мок', 'замо́к') are never tested by listening or dictation.
- The apostrophe is U+2019 ('ім’я́', 'п’ять').
- No dash in a `target` string. Never write a sentence that Ukrainian punctuation would give a dash, a noun subject with a noun predicate ('Окса́на' with 'студе́нтка'): use a pronoun subject ('Вона́ студе́нтка.'), 'Це' ('Це моя́ сестра́.'), negation or a verb. Direct speech goes inside «».
- Numbers in `target` strings are always words, since their spoken form depends on case and gender.
- Euphony is applied in every `target` string: 'у' and 'в', 'і' and 'й', 'з', 'із' and 'зі', 'іти́' and 'йти'.
- Address: 'ти' among friends, family and children; 'ви' to a stranger, an older person, in service and in any group; 'Ви' only in a letter to one person. Formal address is 'па́не' or 'па́ні' with the vocative of a first name, or first name and patronymic in the vocative; women's professions use the feminine form ('лі́карка').
- Source-language text uses the current English names Kyiv, Lviv, Odesa, Kharkiv and Dnipro, and 'Ukraine' without 'the'; 'в Украї́ні', never 'на', in target text.

Content rule 2 cases:

- 'є' is the one present form of 'бу́ти' for every person. It states existence ('Тут є кафе́.') and possession ('У ме́не є брат.') and is otherwise left out. It is not a particle and is not presented as a present-tense conjugation of 'бу́ти'.
- 'нема́є' ('нема́') is a predicative word meaning 'there is not', fused from 'не' and 'ма́є', and takes the genitive. It is kept out of the conjugation tables of 'бу́ти' and 'ма́ти'.
- 'У ме́не є' is a preposition, a genitive pronoun and 'є': the thing owned is the subject. 'Мене́ зва́ти' is an accusative pronoun and an infinitive in a sentence without a subject. 'Мені́ подо́бається' has the thing liked as subject and the person in the dative, so 'подо́батися' is glossed 'to please'. None of these is shown as a verb with the person as subject.
- Predicative words ('те́пло', 'хо́лодно', 'тре́ба', 'мо́жна', 'не мо́жна', 'ва́рто', 'шко́да', 'ва́жливо', 'необхі́дно') are the predicate of a sentence without a grammatical subject, often with a dative person. Most look like adverbs in '-о'; all are kept out of adverb lists; `part_of_speech` is `predicative word`. 'Те́пло' and the noun 'тепло́' differ by stress.
- 'Це' is an invariable pointing pronoun ('Це Окса́на.') and, separately, the neuter of 'цей' ('це вікно́'). It is never Ukrainian for 'is'.
- 'Його́', 'її́' and 'їх' meaning 'his', 'her', 'their' are genitive forms of 'він', 'вона́', 'вони́' and never change. They are explained apart from the possessive pronouns that decline ('мій', 'твій', 'наш', 'ваш', 'ї́хній', 'свій') and never share their table.
- 'Пан' and 'па́ні' are nouns used with 'ви', not pronouns; 'па́ні' never changes.
- Nouns in '-а' or '-я' that name men ('Мико́ла', 'ко́лега' for a man, 'суддя́') are masculine in agreement and never listed among feminine nouns by their ending; 'ко́лега' is a noun of common gender.
- Nouns that decline like adjectives ('ва́нна', 'лю́тий', 'вихідні́', 'знайо́мий', 'майбу́тнє') are nouns: `part_of_speech` `feminine noun, declined like an adjective` and so on, with a note saying why they take adjective endings.
- Ordinal numerals ('пе́рший', 'дру́га') are numerals that decline like adjectives, never listed as adjectives; 'оди́н', 'два' and 'дві' are cardinal numerals that show gender.
- '-ся' is a suffix, not a pronoun; a verb with it is a reflexive verb.
- 'Би' and 'б' are a particle that makes the conditional with the past form, not an auxiliary verb; 'якби́' and 'щоб' are conjunctions that contain it, and the verb after 'щоб' is the ordinary past form, never called a subjunctive.
- 'Дава́й' and 'дава́йте' are imperatives of 'дава́ти' used as a particle meaning 'let's', distinct from the first person plural imperative ('чита́ймо', 'ході́мо'); 'неха́й' and 'хай' are particles.
- 'Чи' is a particle that opens a yes or no question and a conjunction meaning 'or' or 'whether'; it is kept out of lists of question words.
- 'Дя́кую' and 'перепро́шую' are first person verb forms ('I-thank'); 'ви́бачте' and 'бува́й' are imperatives; 'приві́т' is a noun used as a greeting. None is an interjection.
- 'Украї́нською' is the instrumental feminine of 'украї́нський' with 'мо́вою' left out, not an adverb; the adverb is 'по-украї́нськи'.
- 'Пови́нен' is the short form of an adjective used as a predicate, not a verb.
- 'Мо́же' meaning 'maybe' is the 'він' form of 'могти́' used as a parenthetical word; 'зда́ється' likewise.
- Adverbial participles ('чита́ючи', 'прочита́вши') are verb forms, never listed among adverbs or participles; the forms in '-но' and '-то' ('зро́блено') are invariable predicate forms, never listed as adverbs.
- Plural-only nouns ('гро́ші', 'штани́', 'две́рі', 'ме́блі', 'лі́ки', 'кані́кули') have no singular and are not plurals of one. 'Ді́ти' and 'лю́ди' are the plurals of 'дити́на' and 'люди́на' on another stem.
- 'Ма́ти', mother, and 'ма́ти', to have, are two words with one spelling.

Unit rules:

- A1 uses the present, and the future with 'бу́ду' plus an infinitive once taught, but no past and no perfective verbs except listed fixed phrases. Its cases are the nominative, vocative, accusative, locative and genitive singular, with only the nominative plural; datives and instrumentals appear only in listed fixed phrases ('мені́... ро́ків', 'украї́нською', 'ка́ва з молоко́м'), each with a note.
- A2 adds the past, aspect and all three futures, every case in both numbers, the imperative, the conditional with 'би', the unprefixed verbs of motion with 'піти́' and 'пої́хати', and comparison.
- B1 adds 'щоб' clauses, relative clauses, prefixed verbs of motion, passive participles and the forms in '-но' and '-то', adverbial participles and declined numerals, in longer text. B2 adds reported speech and the finer uses of prefixes and aspect; older and poetic forms appear only in readings, for recognition.
- Every unit from `a1-07` on has a note titled "Spelling and sound: ..." for its words that show the apostrophe, a doubled consonant, the soft sign, 'я', 'ю', 'є' softening a consonant, 'в' said [w], 'г' against 'ґ', a cluster said differently from its spelling ('-ться', '-шся', '-дцять', 'ле́гко'), a voicing change ('вокза́л'), an alternation ('кіт', 'кота́'; 'кни́га', 'у кни́зі'; 'ходи́ти', 'ходжу́') or stress that moves between forms, explained with the rules of the first four units. The stress mark itself never needs a justification.
- `source_literal` is given for 'у ме́не є', 'нема́є', 'мене́ зва́ти', 'мені́... ро́ків', 'подо́батися', predicative words with a dative person, 'болі́ти', 'іде́ дощ', double negatives, word order that differs from English, and idioms. It supplies no articles and no 'is' where Ukrainian has none; in the first eight units it is given for every sentence without 'is'.
