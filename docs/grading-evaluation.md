# Grading evaluation

Measures how well small local language models grade a learner's short written answer to an open question, by pointing at specific mistakes. The grader is meant to be labelled to the learner as model-graded, so some inaccuracy is tolerated; false alarms on correct answers are the costliest failure.

The tool is `tools/grading_evaluation`; `./scripts/dev/evaluate-grading.sh` wraps it. Models run through Ollama.

## What is graded

- Input: the task prompt, optional facts about the scene, the learner's level, the learner's answer
- Output: `written_in_portuguese`, `answers_the_question`, and a list of findings
- Finding: `quote` (copied from the answer), `category`, `correction`, `explanation` (English, one sentence)
- Categories: `spelling`, `verb_form`, `ser_estar`, `agreement`, `article_preposition`, `word_choice`, `word_order`, `pronoun`, `not_brazilian`, `content_mismatch`
- Scope: writing only, answers of a sentence to a short paragraph

## Cases

`tools/grading_evaluation/cases/pt_br.en.json`; `check-cases` validates it and counts it by kind, level and category.

- Each case has the task, the answer, and a reference annotation: the mistakes, each with its exact wrong text, category and accepted corrections
- Kinds: control (no mistakes), erroneous, off topic, not in Portuguese
- Controls include spoken Brazilian forms (`tô`, `a gente`) which must not be flagged
- Only unambiguous mistakes are annotated; usages that Brazilian speakers and teachers disagree on stay out
- Reference annotations are written by one author; a second reader should review them before the results drive a decision
- After tuning a prompt against these cases, add unseen cases before the final comparison

## Scoring

A finding matches a reference mistake when its quote overlaps the mistake's text; each reference mistake matches at most one finding.

- Matched: counts toward precision and recall
- Duplicate: a further finding on an already matched mistake; reported, not penalized
- Quote not in the answer: counts against precision and is reported separately
- No-op: a correction identical to the quoted text counts as a false positive
- Imprecise quote: a match whose quote is more than three words longer than the mistake
- Category accuracy and correction agreement: measured among matches; the correction must equal an accepted correction after normalizing case and punctuation, so it understates valid alternatives
- Control false alarm rate: the proportion of control answers with at least one finding
- Not in Portuguese cases are excluded from precision and recall
- Scores are computed from the recorded replies, so scoring changes need no new model run
- A case edited after a run is rejected by `report`; grade again

## Bar

A model is a candidate only if every line holds; the thresholds are in `evaluation_report.rs`.

- Control false alarms at most 10%
- Quotes not in the answer at most 2%
- Precision at least 70%
- Recall at least 60%
- Correction agreement at least 60%
- p95 latency at most 15 s per answer

## Protocol

1. `run` every candidate model with both prompt variants at temperature 0
2. `report` and read the comparison, then `failures.md` for the best candidates to separate model errors from case errors
3. Repeat the best candidates with `--repeats 3 --temperature 0.3` and compare the unstable case rate
4. Rate the explanations in `review-sample.md` by hand for the best candidates; explanations are not scored automatically
5. Change the prompt only for a failure pattern seen across models, and rerun everything

Latency excludes the model load time. Results come from one machine; the download size and parameter count indicate whether a model could run on a phone, which this evaluation does not test.

## Candidate models

The lists are in `scripts/dev/evaluate-grading.sh`. A model shipped inside the app needs a licence that allows redistribution and commercial use.

- Qwen3 (1.7B, 4B, 8B): Apache 2.0
- Gemma 3 (1B, 4B, 12B): Gemma terms of use
- Llama 3.2 (3B) and 3.1 (8B): Llama community licence
- Phi-4-mini: MIT
- Aya Expanse (8B): non-commercial licence, included as a multilingual reference

## Running

```sh
./scripts/dev/evaluate-grading.sh pull small
./scripts/dev/evaluate-grading.sh check-cases
./scripts/dev/evaluate-grading.sh run --model gemma3:4b
./scripts/dev/evaluate-grading.sh run-candidates
./scripts/dev/evaluate-grading.sh report
```

`run` records raw replies in `build/grading_evaluation/`; `report` writes `report.md`, `failures.md` and `review-sample.md` beside them.
