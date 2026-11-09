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
- `exercises`: at least 22 besides open answers, mixed in order of increasing difficulty, with at least this many of each type: multiple_choice 5, fill_blank 5, translate_to_target 3, order_words 3, listening_choice 3, dictation 2, match 1. Aim for about 26 to 30. Cover the whole unit: vocabulary, the grammar notes and the dialogues. Open answers are optional and follow the rules under their type.

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
{"type": "open_answer", "prompt_source": "Tell a friend about Paulo: what he is like and how he is today.", "context_source": "Paulo is tall and he is a doctor. Today he is tired and hungry.", "example_answers": ["O Paulo é alto e é médico. Hoje ele está cansado e com fome."], "explanation": "optional"}
```
The learner writes their own answer to the task. A language model grades it where the web app can run one; elsewhere the learner compares it with `example_answers`. The learner practises a unit's open answers in its writing practice, in file order, apart from the quiz and quick practice, so an open answer never counts toward the score and its position among the exercises does not matter. The iOS and Android apps leave it out. `context_source` holds the facts of a scene the answer must convey. Only a course whose language pair has a grader prompt (`shared/src/grading/`) can have open answers, and a reading question is never one.

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
- Exercises other than open answers have exactly one defensible correct answer, or every defensible answer listed. No trick questions. Explanations on multiple_choice and fill_blank are strongly encouraged.
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
