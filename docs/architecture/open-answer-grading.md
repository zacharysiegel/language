# Open-answer exercises and their grading

Status: planned.

A new exercise type in which the learner writes a free answer to a task. In the browser, a local language model (gemma4:12b-it-qat through `cross-llama`'s wllama backend) grades it and lists mistakes. Where the model cannot run, the learner compares their answer with the author's example answers.

## Decisions

- Placement: mixed into a unit's quiz and quick practice, only in the units whose files contain open-answer exercises; no per-unit flag.
- Scoring: feedback only. An open answer never counts toward a quiz's pass mark or score.
- Grading runs only in the web app. The iOS and Android apps never see open-answer exercises.
- Fallback, on a computer or browser that cannot run the model: self-check against the example answers, with a notice that automatic grading by an AI model is available on supported computers.
- Model download: automatic in the background. The model is loaded into memory only when an open-answer question appears, and unloaded when the practice session ends.
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
- The test course gets one open-answer exercise per unit.

## Scoring

- `ExerciseKind::is_scored`; false for `OpenAnswer`.
- `AnswerStatus::Unscored`, the result of an open answer.
- `SessionScore` and `PausedPractice::score` count only scored exercises.
- The grader's mistakes are shown, not saved.

## Grader

- `shared/src/grading/`: the system prompt, the six examples, `render_user_message`, `response_schema` and the response parser, moved from `tools/grading_evaluation`, which keeps the other prompt variants, scoring and reports.
- Keyed by target and source language; `pt-BR` with `en` is the only pair.
- `shared` does not depend on `cross-llama`; it returns its own message type.
- Requests: context 4096 tokens, `SlidingWindowCache::Full`, reply maximum 1024 tokens, temperature 0.
- Findings whose quote is not in the answer, or whose replacement changes nothing, are dropped before display.

## `cross-llama` 0.1.1

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
  - Background download when a unit page with open-answer exercises opens; `navigator.storage.persist()` first.
  - Load when an open-answer question appears; unload when the session ends.
- Open-answer view: a multi-line field with spell checking off and a Check button, which shows the pending state.
  - States: loading the model, grading, then feedback; cancelled when the session unmounts.
  - Feedback: an "AI model" tag; whether the answer addresses the task and is in the target language; each mistake's quote, replacement and explanation; the example answers.
  - Self-check: the example answers, plus the notice; while the model is still downloading, the download's progress.

## Phases

Linear stack in `language`; `cross-llama` 0.1.1 is published before phase 5.

1. `language`: R2 CORS for the production origin (PR #1).
2. `language`: grader prompt into `shared`; the evaluation reproduces its recorded results.
3. `language`: open-answer course data, validator, shard, native exclusion, unscored results, and the web view with self-check only.
4. `cross-llama`: 0.1.1.
5. `language`: model hosting and `publish-model`.
6. `language`: isolation headers, wllama files, and grading in the web view.
7. `language`: author open-answer exercises into the chosen units; docs.

## Pull request descriptions

- `cross-llama`: A download-only function, a download check and a browser capability check let an app download a model in the background and decide whether it can run, without loading the model into memory. A Web Lock keeps two tabs from writing the same cached model file.
- `language`: Adds open-answer exercises, which the learner answers in free text. In the browser, a local AI model grades the answer and lists mistakes; elsewhere the learner compares with example answers. Open answers do not count toward a quiz's score.
