# Open-answer exercises and their grading

Status: implemented and deployed. `overview.md` describes the current design; this file keeps the plan and its deviations.

A new exercise type in which the learner writes a free answer to a task. In the browser, a local language model (Google's Gemma 4 12B QAT Q4_0 GGUF, the weights of Ollama's gemma4:12b-it-qat, through `cross-llama`'s wllama backend) grades it and lists mistakes. Where the model cannot run, the learner compares their answer with the author's example answers.

## Decisions

- Placement: mixed into a unit's quiz and quick practice, only in the units whose files contain open-answer exercises; no per-unit flag.
  - Reversed by the owner after deployment: open answers are a separate writing practice (see Scoring).
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
- Deviations, as built:
  - The owner reversed the placement. Writing practice: a third Practice-tab option on units with open answers, in unit order; no score, nothing recorded, ends with the learner's answers.
  - `shared::practice` builds the three lists: `quiz_exercises`, `quick_practice_exercises` and `writing_practice_exercises`.
  - Removed: `ExerciseKind::is_scored`, `PausedPractice::unscored_exercise_indices` (a saved one still parses; serde ignores the field), the two-count quiz label and the summary's note on open answers. `AnswerStatus::Unscored` stays as an open answer's result.
  - A paused quiz's fingerprint covers the quiz's exercises without open answers, so a quiz paused on a unit with open answers under the previous build is cleared with the existing note.

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
- Deviations, as built:
  - A Rust splitter in `course_builder` (`model_hosting`) writes what `llama-gguf-split --split-max-size` writes, byte for byte for a file aligned to 32 whose first tensor fits the limit, instead of running it.
  - The limit counts a part's padded tensor data, as `llama-gguf-split` does, so a part may exceed 512 MiB: the 826 MB `token_embd.weight` has a part of its own. 14 parts.
  - `shared::grading` also holds the unsplit file's size, checked before hashing.
  - `ListedObject` gained `size_bytes`, so a rerun skips a part present at its size.
  - Then replaced: production downloads Google's `gemma-4-12b-it-qat-q4_0.gguf`, unsplit, from its Hugging Face repository at a pinned commit (Apache 2.0, not gated; the CDN allows cross-origin reads and sends `Content-Length`). `shared::grading` names it.
    - Against Ollama's copy: tensor infos and data section byte-identical; metadata 50 entries against 44 (name, license, base model, tags, and a newer chat template); all 12 grader cases give Ollama's replies byte for byte through llama.cpp.
    - `scripts/dev/fetch-grading-model.sh` downloads it into `models/`; `publish-model local` hard links it into the local repository at `models/<name>/<sha256>/<file name>`.
    - The client takes the Hugging Face URL when built with the web feature `grading-model-from-hugging-face`, which `build-site.sh` enables, and the local repository's key otherwise. A build switch rather than a discovery field: the model's size and hash are compiled into the client anyway, and the native apps, which read discovery, never grade.
    - R2 model hosting stays behind `R2_MODEL_HOSTING_ENABLED`, false: `publish-model cloudflare-r2` warns and uploads nothing; set true, it splits the file `shared::grading` names (14 parts).
    - No client requests those parts. Serving the model from R2 also needs the client to request the first part's key, `repository_key::model_part_key(name, sha256, 1, GRADING_MODEL_PART_COUNT)`, `build-site.sh` to drop `grading-model-from-hugging-face`, and the removal of an incomplete split model from wllama's cache restored (see Web).
    - wllama 3.8.1 downloads, caches and loads the 6.98 GB unsplit file in headless Chrome.

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
- Deviations, as built:
  - `GraderStateKind` also has `Unchecked`, before the browser is checked, and `InUseInAnotherPage`.
  - An app-level Web Lock (`ifAvailable`) is held while a page loads or holds the model, so one copy is in memory; a page without it gets the self-check and tries again at its next open answer.
  - Only a failure in wllama's load step is recorded as a failed load, not one in the download or the cache.
  - The model is also unloaded when the session reaches its summary.
  - `scripts/build/prepare-wllama.sh` runs the crate's script, found through `cargo metadata`, and records wllama's version in `web/static/wllama/VERSION`; `verify-site-tree.sh` checks the files.
  - Outside the site build, the model URL is built against the repository of the open course's publication, which also serves its recordings.
  - An unknown storage estimate counts as too little room.
  - Once loaded, the model reads the question's prompt without the answer in one throwaway generation (one token, no schema), so the answer reads only its own tokens.
  - A failed download is not retried in the same page.
  - Removed with the unsplit model: deleting a split model's files from wllama's cache, and downloading it again, after wllama 3.8.1 fails with "Model file not found" on a split model whose download stopped before every part's metadata was written (commit `4542479`, language). An unsplit model cannot reach that state.
  - The native apps keep the stored `download_grading_model` when they save settings.
  - Measured in headless Chrome against the local repository: Check to feedback 3.1 s after the warm-up, 1,018 of 1,043 prompt tokens reused; a Check during the load waited 4 s for it, then graded in 9 s.

## Course content

`pt_br.en.foundations`: 79 open-answer tasks in 40 units, chosen by a review of the whole course against the grader's tested scope (answers of up to 27 words; scene facts checked; use of a taught structure not checked).

- Personal: about the learner's own life, no `context_source`. Scene: `context_source` holds the facts the answer must convey.
- No open answers: `a1-02-sounds-alphabet`, `a1-00-stress-and-diacritics`, `a1-00-how-portuguese-works` (no communicative goal yet), `b2-05-literature-poetry` (reading and recognition only).
- A1, one or two sentences:
  - `a1-01-greetings-introductions`: introduce yourself (personal; not where you live, since `morar` is not taught).
  - `a1-03-ser-estar-feelings`: how you are today (personal); a person needing both `ser` and `estar` (scene).
  - `a1-04-numbers-time`: your age and birthday (personal).
  - `a1-05-family-possessives`: your family (personal); a relative and their pet with `dele` or `dela` (scene).
  - `a1-06-food-drinks-ordering`: order at a padaria with `eu queria` (scene); food you like and dislike (personal).
  - `a1-07-daily-routine`: your weekday with times and frequency (personal); someone else's routine with `se levanta` (scene).
  - `a1-08-places-directions`: directions with `vire`, `siga`, `atravesse` (scene); where a place is (scene).
  - `a1-09-shopping-colors-prices`: what someone is wearing (scene); ask for an item's size, color and price (scene).
  - `a1-10-home-housing`: your home with `tem` (personal); where the furniture is with `fica` (scene).
  - `a1-11-plans-weather-leisure`: weekend plans with `ir` and an infinitive (personal); today's weather (scene).
  - `a1-12-consolidation`: introduce yourself with `e`, `mas`, `então` (personal); why you learn Portuguese, with `porque` (personal); lost, ask for help (scene).
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
4. `cross-llama`: 0.2.0 (done; 0.2.1 later resumed a stopped split download).
5. `language`: model hosting and `publish-model` (done; production later moved to Google's file on Hugging Face, with R2 model hosting kept but turned off).
6. `language`: isolation headers, wllama files, and grading in the web view (done).
7. `language`: author open-answer exercises into the chosen units; docs (done; 79 tasks, published to R2 as `2026-11-08+henderson` and deployed).

## Pull request description

- `cross-llama`: Models whose chat template llama.cpp's built-in formats do not recognize, such as Gemma 4, failed every request. Both backends now format prompts with the model's own Jinja template, with thinking off. A download-only function, a download check and a browser capability check let an app download a model in the background and decide whether it can run, without loading the model into memory. A Web Lock keeps two tabs from writing the same cached model file.
