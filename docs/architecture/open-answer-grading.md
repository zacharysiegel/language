# Open-answer exercises and their grading

Status: planned.

A new exercise type in which the learner writes a free answer to a task. In the browser, a local language model (gemma4:12b-it-qat through `cross-llama`'s wllama backend) grades it and lists mistakes. Where the model cannot run, the learner compares their answer with the author's example answers.

## Decisions

- Placement: mixed into a unit's quiz and quick practice, only in the units whose files contain open-answer exercises; no per-unit flag.
- Scoring: feedback only. An open answer never counts toward a quiz's pass mark or score.
- Grading runs only in the web app. The iOS and Android apps never see open-answer exercises.
- Fallback, on a computer or browser that cannot run the model: self-check against the example answers, with a notice that automatic grading by an AI model is available on supported computers.
- Model download: automatic in the background unless the learner turns it off in settings. The model is loaded into memory only when an open-answer question appears, and unloaded when the practice session ends.
- Every graded result is labelled as graded by an AI model.
- Grader: the evaluated prompt (`few_shot_six`, temperature 0), unchanged.

## Course data

- `Exercise::OpenAnswer(OpenAnswerExercise)`, kind code `open_answer`:
  - `prompt_source`: the task, in the source language.
  - `context_source`: optional scene facts, in the source language.
  - `example_answers`: at least one, in the target language, each checked by `check_spoken_target`.
  - `explanation`, `clip_key`: as on the other kinds; the clip speaks the first example answer.
- Validator:
  - No per-kind minimum; open answers do not count toward the 22-exercise minimum.
  - Not allowed as reading questions.
  - Rejected in a course whose language pair has no grader prompt.
- Shard: `prompt` holds `prompt_source`; a new nullable `context_source` column; example answers in `exercise_answer`.
- Versions: `SCHEMA_VERSION` 2 to 3, `COURSE_MANIFEST_SCHEMA_VERSION` 3 to 4.
- Native apps: the `ffi` crate leaves open answers out of `UnitLessonRecord` and native recording prefetch; `ExerciseRecord` gains no variant.
- The iOS and Android UI tests read a unit's exercises from its JSON without the open answers.
- The test course gets one open-answer exercise per unit.

## Scoring

- `ExerciseKind::is_scored`; false for `OpenAnswer`.
- `AnswerStatus::Unscored`, the result of an open answer.
- `SessionScore` and `PausedPractice::score` count only scored exercises; an open answer is never missed.
- `PausedPractice::unscored_exercise_indices`, so a paused quiz is scored without its exercises.
- The quiz button counts scored exercises and open answers separately.
- The grader's mistakes are shown, not saved.

## Grader

- `shared/src/grading/`: the system prompt, the six examples, `render_user_message`, `response_schema` and the response parser, moved from `tools/grading_evaluation`, which keeps the other prompt variants, scoring and reports.
- Keyed by target and source language; `pt-BR` with `en` is the only pair.
- `shared` does not depend on `cross-llama`; it returns its own message type.
- Requests: context 4096 tokens, `SlidingWindowCache::Full`, reply maximum 1024 tokens, temperature 0.
- Findings whose quote is not in the answer, or whose replacement changes nothing, are dropped before display.

## `cross-llama` 0.2.0

- Prompts are formatted with the model's own Jinja chat template (`tokenizer.chat_template`) on both backends: wllama with `jinja: true`, llama.cpp through `minijinja`. llama.cpp's built-in templates do not recognize gemma4.
- Thinking is off: `enable_thinking: false` in the template's variables, and an empty thought block is stripped from the reply (`reasoning_format: "deepseek"` in wllama).
- `download_model(&ModelSource, on_progress)`: downloads into wllama's cache without loading.
- `is_model_downloaded(&ModelSource)`: no network.
- `probe_browser_capability()`: WebGPU adapter present, cross-origin isolation, which wllama build applies, storage quota and usage, `navigator.deviceMemory` when present.
- A Web Lock per model URL around downloading and loading.

## Model hosting

- The text GGUF from Ollama's gemma4:12b-it-qat, split with `llama-gguf-split` into parts of at most 512 MiB.
- Keys: `models/<model name>/<sha256 of the unsplit file>/<name>-0000N-of-0000M.gguf`, immutable.
- `course_builder publish-model <target>`: uploads to R2 or copies into the local repository.
- The client derives the model URL from the repository base and a constant in `shared`.

## Web

- Headers: `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp`, in `web/static/_headers` and the dev server.
- wllama's files: fetched by the `fetch-wllama.sh` that ships in the `cross-llama` crate, into a generated, uncommitted `web/static/wllama/`; by `serve.sh` and `build-site.sh`.
- `web/src/grading/`, gated on `hydrate`:
  - `GraderStateKind`: unsupported, not downloaded, downloading, downloaded, loading, ready, failed.
  - Capable: WebGPU adapter, enough storage quota for the model, and no earlier failed load recorded in this browser.
  - Background download when a unit page with open-answer exercises opens, unless the setting is off; `navigator.storage.persist()` first.
  - Load when an open-answer question appears; unload when the session ends.
- Setting: `Settings::download_grading_model`, default on (a serde default), in the settings controls with the model's download size. Turning it off stops a running download; a downloaded model stays and keeps grading.
- Open-answer view: a multi-line field with spell checking off and a Check button, which shows the pending state.
  - States: loading the model, grading, then feedback; cancelled when the session unmounts.
  - Feedback: an "AI model" tag; whether the answer addresses the task and is in the target language; each mistake's quote, replacement and explanation; the example answers.
  - Self-check: the example answers, plus the notice; while the model is still downloading, the download's progress; with the setting off, that automatic grading needs the download turned on.

## Course content

`pt_br.en.foundations`: 80 open-answer tasks in 40 units, chosen by a review of the whole course against the grader's tested scope (answers of up to 27 words; scene facts checked; use of a taught structure not checked).

- Personal: about the learner's own life, no `context_source`. Scene: `context_source` holds the facts the answer must convey.
- No open answers: `a1-02-sounds-alphabet`, `a1-00-stress-and-diacritics`, `a1-00-how-portuguese-works` (no communicative goal yet), `b2-05-literature-poetry` (reading and recognition only).
- A1, one or two sentences:
  - `a1-01-greetings-introductions`: introduce yourself (personal; not where you live, since `morar` is not taught).
  - `a1-03-ser-estar-feelings`: how you are today (personal); a person needing both `ser` and `estar` (scene).
  - `a1-04-numbers-time`: your age and birthday (personal).
  - `a1-05-family-possessives`: your family (personal); a relative and their pet with `dele` or `dela` (scene).
  - `a1-06-food-drinks-ordering`: order at a padaria with `eu queria` (scene); food you like and dislike (personal).
  - `a1-07-daily-routine`: your weekday with times and frequency (personal, up to three sentences); someone else's routine with `se levanta` (scene).
  - `a1-08-places-directions`: directions with `vire`, `siga`, `atravesse` (scene); where a place is (scene).
  - `a1-09-shopping-colors-prices`: what someone is wearing (scene); ask for an item's size, color and price (scene).
  - `a1-10-home-housing`: your home with `tem` (personal); where the furniture is with `fica` (scene).
  - `a1-11-plans-weather-leisure`: weekend plans with `ir` and an infinitive (personal); today's weather (scene); decline an invitation (scene).
  - `a1-12-consolidation`: introduce yourself with `e`, `mas`, `então` (personal, three sentences); why you learn Portuguese, with `porque` (personal); lost, ask for help (scene).
- A2, up to three sentences:
  - `a2-01-past-regular`: last weekend (personal); yesterday with regular verbs including `paguei`, `cheguei` (scene).
  - `a2-02-irregular-preterite`: a trip with `primeiro`, `depois`, `por fim` (personal); an event needing `fui`, `fiz`, `trouxe` (scene).
  - `a2-03-imperfect`: childhood habits (personal); an old photo (scene).
  - `a2-04-preterite-vs-imperfect`: why you were late, `estava ... quando` (scene); a short story (personal).
  - `a2-05-health-body`: symptoms to a doctor, with since when (scene); advice for a cold (scene).
  - `a2-06-travel-transport`: buy a ticket or book a room (scene); your last trip (personal).
  - `a2-07-comparatives-superlatives`: compare two things (scene); your favorite place and why (personal).
  - `a2-08-imperative-instructions`: recipe steps (scene); rules for a colleague, with a negative command (scene).
  - `a2-09-object-pronouns`: two short replies that need an object pronoun (scene).
  - `a2-10-progressive-work`: your job against what you are doing now (personal); a phone message (scene).
  - `a2-11-future-conditional`: a polite request with `poderia` or `gostaria` (scene); advice with `Se eu fosse você` (scene).
  - `a2-12-consolidation`: reschedule a dinner, informal (scene); a formal complaint email (scene); a bad day with connectors (personal).
- B1, up to four sentences or about 30 words:
  - `b1-01-present-perfect-participles`: what you have been doing lately (personal); done and not done yet, with `já`, `ainda não`, `acabar de` (scene).
  - `b1-02-present-subjunctive`: wishes before an interview (scene); a note with `peço que` (scene).
  - `b1-03-subjunctive-opinion-doubt`: advise a friend quitting a job (scene); a plan with `a menos que`, `caso` (scene).
  - `b1-04-future-subjunctive`: after the course (personal); reply to a supplier with `assim que`, `se` (scene).
  - `b1-05-imperfect-subjunctive-conditionals`: a free month (personal); advise about a job abroad (scene).
  - `b1-06-relatives-connectors`: your neighborhood with `onde`, `que`, `cujo` (personal); remote work with `porém`, `portanto` (personal).
  - `b1-07-passive-impersonal`: a building's history in the passive (scene); a building notice (scene).
  - `b1-08-personal-infinitive`: what each person must do before a deadline (scene); why you all arrived early (scene).
  - `b1-09-work-bureaucracy`: a formal email asking for an appointment or document (scene); a polite refusal (scene).
  - `b1-10-opinions-debate`: partly agree with a proposal (scene); report what someone said or asked (scene).
- B2, up to four sentences or about 30 words:
  - `b2-01-reported-speech`: report a statement (scene); report a question or command (scene).
  - `b2-02-pluperfect-compound-tenses`: what had happened when you arrived (scene); regret with `se eu tivesse ..., teria` (scene).
  - `b2-03-idioms-slang-regional`: a relaxed message about a hard day, with an idiom (personal).
  - `b2-04-pronoun-placement-advanced`: a short formal letter with pronouns after the verb (scene).
  - `b2-06-formal-writing`: a cover letter's opening (scene); one essay paragraph (personal).
  - `b2-07-brazilian-culture-history`: a historical event (scene, with dates); cause and consequence (scene).
  - `b2-08-capstone`: a nuanced opinion with `por mais que` or `ainda que` (scene); one matter written formally, then again to a friend (two tasks, scene).

Authoring rules, added to `docs/authoring.md` with the exercise type:

- Answerable with grammar and vocabulary taught in the unit or earlier.
- The task's wording makes the unit's grammar the natural answer; never rely on naming a grammar term.
- One communicative act per task.
- Length: A1 one or two sentences, A2 up to three, B1 and B2 up to about four or 30 words.
- `context_source` whenever the content can be checked; every fact in it sayable with taught vocabulary.
- Personal tasks take no `context_source` and never fix the writer's gender; example answers give both forms where agreement depends on it.
- Example answers use only taught structures and follow every `target` rule; more than one where several are natural.
- At most two per unit, three in consolidation and writing units; never in place of a required exercise type.
- None in a unit whose objectives are only reading or recognition.

## Phases

Each `language` phase merges into `main` locally once its checks pass; `cross-llama` 0.2.0 is published before phase 5.

1. `language`: R2 CORS for the production origin (done).
2. `language`: grader prompt into `shared` (done; every rendered prompt byte-identical).
3. `language`: open-answer course data, validator, shard, native exclusion, unscored results, and the web view with self-check only (done).
4. `cross-llama`: 0.2.0.
5. `language`: model hosting and `publish-model`.
6. `language`: isolation headers, wllama files, and grading in the web view.
7. `language`: author open-answer exercises into the chosen units; docs.

## Pull request description

- `cross-llama`: Models whose chat template llama.cpp's built-in formats do not recognize, such as Gemma 4, failed every request. Both backends now format prompts with the model's own Jinja template, with thinking off. A download-only function, a download check and a browser capability check let an app download a model in the background and decide whether it can run, without loading the model into memory. A Web Lock keeps two tabs from writing the same cached model file.
