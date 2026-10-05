# Unit authoring guide

Each unit is one JSON file at `data/units/<unit-id>.json`. The unit ids, levels, titles, goals, grammar focus and vocabulary themes are in `data/curriculum.json`; follow them. Check your file with `node tools/validate.mjs <unit-id>` from the project root and fix every error before finishing.

## Language rules (Brazilian Portuguese only)

- Brazilian vocabulary, spelling and grammar throughout: `você`, `a gente`, `ônibus`, `trem`, `celular`, `café da manhã`, `banheiro`, `geladeira`, `suco`, `moça`, `fato`, `contato`.
- Progressive is `estou fazendo`, never `estou a fazer`. Colloquial object pronouns go before the verb (`me disse`, `te amo`).
- Use natural, current speech in dialogues: `tá`, `pra`, `né` are fine in informal dialogue at A2 and above (sparingly at A1). Keep grammar notes honest about formal vs colloquial usage.
- Every `pt` string must be correct, natural and fully spelled with accents. Re-read everything once for accent, agreement and conjugation errors before you finish.
- Every `pt` string is spoken by a text-to-speech engine. It must be plain speakable text: no slashes (write two separate items instead of `obrigado/obrigada`), no parentheses, no markdown, no symbols, no em or en dashes, no `___`. Write numbers as words when they are short. Keep each string under about 200 characters.
- English (`en`, `explanation`, `prompt`, `hint_en`, `context`, `note`) is plain natural English. Explanations and notes may contain `\n\n` to separate paragraphs. Plain text only, no markdown syntax.
- Mentions of European Portuguese as a contrast belong only in `note` or `explanation` text (English), never inside a `pt` field.
- Difficulty must match the level. A1 uses very short sentences and only the grammar covered so far (the current unit's focus plus all earlier units in the curriculum). A2 can use the past tenses once introduced; B1 and B2 use longer, richer text.
- Recycle vocabulary and grammar from earlier units in the sequence, but each unit must introduce its own new vocabulary and must not require grammar from later units.

## File format

```json
{
  "id": "a1-01-greetings-introductions",
  "level": "A1",
  "order": 1,
  "title": "Greetings and introductions",
  "objectives": ["Three to five 'I can...' style goals as full English sentences"],
  "vocabulary": [
    {
      "pt": "olá",
      "en": "hello",
      "pos": "interjection",
      "note": "optional short English usage note",
      "example": {"pt": "Olá, tudo bem?", "en": "Hello, how are you?"}
    }
  ],
  "dialogues": [
    {
      "id": "d1",
      "title": "Meeting at a party",
      "context": "One or two English sentences setting the scene.",
      "lines": [
        {"speaker": "Ana", "gender": "f", "pt": "Oi, tudo bem?", "en": "Hi, how are you?"},
        {"speaker": "Carlos", "gender": "m", "pt": "Tudo bem, e você?", "en": "Fine, and you?"}
      ]
    }
  ],
  "grammar": [
    {
      "title": "Short title",
      "explanation": "Clear plain-text English explanation written for an English speaker. Explain the rule, the pattern, the exceptions that matter at this level and where Brazilian usage differs from textbook or European forms. Use \n\n between paragraphs.",
      "table": {"headers": ["Pronoun", "Form"], "rows": [["eu", "sou"], ["você", "é"]]},
      "examples": [{"pt": "Eu sou brasileiro.", "en": "I am Brazilian."}]
    }
  ],
  "culture": {"title": "Short title", "text": "Two or more plain-text English paragraphs about Brazilian culture or usage that relates to the unit's theme. Include a few Portuguese words inline in plain text if helpful."},
  "readings": [
    {
      "id": "r1",
      "title": "Title",
      "paragraphs": [{"pt": "A paragraph of Portuguese text.", "en": "Its English translation."}],
      "questions": [{"type": "multiple_choice", "prompt": "...", "choices": ["...", "...", "..."], "answer": 0, "explanation": "..."}]
    }
  ],
  "exercises": []
}
```

Rules for each section:

- `vocabulary`: at least 18 items, more is better (20 to 28 is typical). Every item needs an `example` pair. `pos` is optional (noun, verb, adjective, phrase, and so on). For nouns, include the article in `pt` when gender matters (`a cidade`, `o aluguel`). For verbs use the infinitive. Do not repeat a `pt` value within the unit.
- `dialogues`: 2 to 3 dialogues, each with 6 to 14 lines, 2 to 4 distinct speakers, speaker names consistent within a dialogue and `gender` of `f` or `m` on every line matching the speaker. Dialogues must use the unit's vocabulary and grammar focus in realistic situations. A dialogue id must be unique within the unit (`d1`, `d2`, ...).
- `grammar`: 2 to 4 notes covering the unit's grammar focus. Each explanation is at least 250 characters and genuinely teaches (rule, pattern, exceptions, Brazilian usage). Each note has at least 4 examples. Add a `table` where a paradigm, comparison or list of forms helps (all cells are strings, every row as wide as the headers).
- `culture`: one note of at least 200 characters.
- `readings`: optional at A1 and A2 (omit the key or use `[]`), required at B1 and B2 (at least 1; 2 for B2 units). A reading has at least 2 paragraphs of real connected text at the level, and at least 2 (ideally 3 to 4) `multiple_choice` comprehension questions.
- `exercises`: at least 22 in total, mixed in order of increasing difficulty, with at least this many of each type: multiple_choice 5, fill_blank 5, translate_to_pt 3, order_words 3, listening_choice 3, dictation 2, match 1. Aim for about 26 to 30 exercises. Cover the whole unit: vocabulary, the grammar notes, and the dialogues.

## Exercise types

```json
{"type": "multiple_choice", "prompt": "How do you say 'good evening'?", "choices": ["Boa noite", "Boa tarde", "Bom dia"], "answer": 1, "explanation": "optional short English explanation"}
```
Prompts may be English or Portuguese questions. 3 to 5 unique choices. `answer` is the zero-based index. Vary the position of the correct answer across exercises.

```json
{"type": "fill_blank", "prompt_pt": "Eu ___ brasileiro.", "answer": ["sou"], "hint_en": "I am Brazilian.", "explanation": "optional"}
```
Exactly one `___` in `prompt_pt`. `answer` is an array of acceptable fillers (lowercase, no punctuation, may be more than one word, for example `["vou comer"]`). Include every genuinely acceptable alternative.

```json
{"type": "translate_to_pt", "prompt_en": "I live in São Paulo.", "answers": ["Eu moro em São Paulo.", "Moro em São Paulo."], "explanation": "optional"}
```
`answers` lists every natural correct translation the learner could reasonably type, including versions with and without the subject pronoun where both are natural. Keep the sentence short and tightly constrained by the English so the answer set is practical.

```json
{"type": "order_words", "prompt_en": "Where do you live?", "words": ["Onde", "você", "mora", "?"], "distractors": ["moro"]}
```
`words` are the tokens in the correct order. Punctuation must be attached to the preceding word (`mora?`), not a separate token, and no token contains a space. The learner rebuilds the sentence from the shuffled tokens plus the optional `distractors` (at most 3). Keep the sentence to 3 to 10 tokens, and make sure the token order is the only correct order.

```json
{"type": "listening_choice", "audio_pt": "Eu moro no Rio de Janeiro.", "choices": ["I live in Rio de Janeiro.", "I work in Rio de Janeiro.", "I study in Rio de Janeiro."], "answer": 0}
```
The learner hears `audio_pt` and picks the English meaning. Distractors must differ in a way that can be heard (a different key word), not in a way the audio cannot resolve.

```json
{"type": "dictation", "audio_pt": "Bom dia, tudo bem?", "hint_en": "Good morning, how are you?"}
```
The learner types what they hear. Keep it short (3 to 10 words) and unambiguous in spelling.

```json
{"type": "match", "prompt": "Match the Portuguese with the English", "pairs": [{"pt": "o pai", "en": "the father"}, {"pt": "a mãe", "en": "the mother"}, {"pt": "o irmão", "en": "the brother"}, {"pt": "a irmã", "en": "the sister"}]}
```
4 to 6 pairs, all unambiguous.

## Quality bar

- Content is teachable from the page alone by a complete beginner at A1 who speaks English. Explain with English comparisons where they help.
- Dialogues sound like real Brazilians, not textbook robots, while staying inside the level.
- Exercises have exactly one defensible correct answer (or every defensible answer listed). Do not write trick questions. Explanations on multiple_choice and fill_blank are strongly encouraged.
- No two exercises in a unit test the same sentence.
- Write valid JSON (double quotes, no trailing commas, escaped quotes inside strings as `\"`, newlines in strings as `\n`). Use real UTF-8 accented characters, not escapes.
- Use the Write tool to write the file, run the validator, fix all errors, then do a last careful pass over the Portuguese for mistakes.
