# Grading evaluation results

Results of measuring small local language models on grading a learner's short written answer in Portuguese, by pointing at specific mistakes. The method, cases, scoring and bar are in `grading-evaluation.md`. Runs took place from 2026-10-29 to 2026-11-02 on one Apple M4 Max with 48 GB, through Ollama.

## Decision

- Model: `gemma3:12b` (8.2 GB download)
- Prompt: the six-example form (`few_shot_six`)
- Temperature: 0
- Where: web and desktop only; no model small enough for a phone met the bar

## What was run

- First comparison: 11 models, a plain and a three-example prompt, 71 cases
- Prompt experiment: 5 models, 8 prompt forms, 114 cases (the 71 plus 43 more; 45 are correct answers)
- Held-out check: 52 unseen cases (22 correct answers, 26 with one mistake, 4 off topic or not in Portuguese), for the top two prompt forms
- Stability: 3 gradings per answer at temperature 0.3
- Verification: two decision models asked whether each finding is a real mistake

## Results

- First comparison: only `gemma3:12b` with three examples met the bar (precision 93%, recall 78%, 4% of correct answers flagged). No model of 4B parameters or fewer met the bar.
- Output order: Ollama writes a schema's properties alphabetically, so models first wrote the kind and the replacement and the quoted text last. Renaming the properties so they sort as text, reason, kind, replacement fixed the order. Results taken before the change were discarded.
- Prompt form: examples matter much more than their number or order, and the effect depends on the model.
  - `gemma3:12b` flags 29% of correct answers with no examples and 2% to 7% with examples that include a mistake.
  - `qwen3.5` flags 7% with no examples and 27% to 33% with several.
  - `qwen3:8b` flags 11% with no examples and 20% to 71% with examples.

Best prompt form per model on the 114 cases, temperature 0:

| Model          | Prompt form  | Precision | Recall | False positive rate | Meets bar |
| -------------- | ------------ | --------- | ------ | ------------------- | --------- |
| gemma3:12b     | few_shot_six | 95%       | 78%    | 2%                  | yes       |
| qwen3:8b       | few_shot_six | 78%       | 85%    | 11%                 | no        |
| qwen3.5:latest | few_shot_six | 72%       | 88%    | 33%                 | no        |
| qwen3:4b       | few_shot_six | 69%       | 65%    | 16%                 | no        |
| gemma3:4b      | few_shot_six | 64%       | 56%    | 20%                 | no        |

`qwen3:4b` with three examples comes closest among models a phone could hold (precision 70%, recall 63%, false positive rate 9%, 2.5 GB); it fails only correction agreement, at 56% against 60%.

Held-out check, `gemma3:12b`:

| Run                         | Prompt form  | Precision | Recall | False positive rate | Correction agreement | Meets bar |
| --------------------------- | ------------ | --------- | ------ | ------------------- | -------------------- | --------- |
| 52 unseen cases             | few_shot_six | 91%       | 81%    | 0%                  | 67%                  | yes       |
| 52 unseen cases             | few_shot     | 91%       | 81%    | 0%                  | 52%                  | no        |
| 52 unseen cases, 3 repeats  | few_shot_six | 91%       | 81%    | 0%                  | 62%                  | yes       |
| 114 cases, 3 repeats        | few_shot_six | 91%       | 77%    | 2%                  | 71%                  | yes       |
| 114 cases, 3 repeats        | few_shot     | 90%       | 77%    | 4%                  | 68%                  | yes       |

- Unseen cases: 21 of 26 mistakes found, 2 false alarms, 0 of 22 correct answers flagged.
- Stability: no unseen answer changed outcome across repeats; 3% to 6% of the 114 did.
- Latency: p95 of 2.7 s per answer on an idle machine, plus 4 s to 7 s to load the model after it has been idle.
- Verification: Kev separated real from false findings only weakly (AUROC 0.69, or 0.70 with the grader's explanation) and Von more weakly (0.60 and 0.63). Dropping low-probability findings raised `qwen3:8b` precision from 65% to 79% without losing recall, but 17% of correct answers stayed flagged; a stricter threshold cost `gemma3:12b` about half its recall. No verifier is used.

## Follow-up investigations (2026-11-03)

### Newer models

Eleven newer models were run on Ollama 0.40.14 against both case sets. Two Gemma 4 12B builds meet the bar on both; no phone-sized model does.

Each cell lists precision / recall / false positive rate / correction agreement / p95 latency.

| Model                  | Prompt       | Download | Tuned (114 cases)     | Unseen (52 cases)    |
| ---------------------- | ------------ | -------- | --------------------- | -------------------- |
| gemma3:12b (reference) | few_shot_six | 8.2 GB   | 95%/78%/2%/74%/4.0 s  | 91%/81%/0%/67%/2.7 s |
| gemma4:12b-it-qat      | few_shot_six | 7.2 GB   | 100%/90%/0%/70%/3.7 s | 96%/85%/5%/64%/3.5 s |
| gemma4:12b             | few_shot_six | 7.7 GB   | 97%/87%/2%/76%/1.8 s  | 96%/85%/5%/64%/2.0 s |
| gemma4:e4b-it-qat      | few_shot_six | 6.1 GB   | 90%/54%/4%/76%/1.1 s  | 95%/69%/0%/61%/1.5 s |

- gemma4:12b-it-qat passes with all three prompts and is the passing build that runs unchanged in llama.cpp-based runtimes.
- Both Gemma 4 12B builds flag 1 of 22 correct unseen answers; gemma3:12b flagged none.
- gemma4:e4b-it-qat comes closest among smaller models but fails recall on the tuned cases.
- gemma3n:e2b and e4b, gemma4:e2b, granite4.2:3b and qwen3.5:4b fail on both sets.

### Runtime for web learners

- In-browser with wllama (llama.cpp compiled to wasm, WebGPU): gemma3 12B Q4_K_M ran fully on the GPU in Chrome 156 on this Mac.
  - p95 3.0 s per answer with `swa_full`, which keeps the cached prompt usable; 8.1 s without it.
  - Every reply matched the JSON schema; on 10 unseen cases it agreed with the Ollama replies 9 times out of 10.
  - First load is a 7.3 GB download; later visits load from the browser's cache with no download in about 15 to 18 s to the first token.
  - Peak memory about 13.6 GiB (GPU process 9.9 GiB, page 3.7 GiB, near the 4 GiB wasm limit).
  - The default build needs Chrome or Edge; Safari and Firefox need a slower compatibility build.
- Ruled out: Chrome's built-in model (no Portuguese), MediaPipe (superseded), WebNN (not shipped), WebLLM and Transformers.js (no ready 12B model), Rust wasm engines (no maintained WebGPU language-model path).
- Alternatives: a native helper beside the browser (Ollama, llama-server or a small Rust program), which needs an install and a local network permission in Chrome; Google's LiteRT-LM web SDK, which has a Gemma 4 12B build but no confirmed constrained JSON.

### Kev fine-tuned on the labelled findings

- Fine-tuned locally (Kev-4B, 171 training findings, 6.4 minutes on the Apple GPU), evaluated on 75 findings from 17 held-out cases.
- AUROC 0.863 against 0.858 for stock Kev (95% interval of the gain −0.066 to +0.096): no real improvement in separating real from false findings.
- Calibration improved, mostly by learning how often findings are real.
- Too few false findings (16 held out) to show a gain; Kev also runs only through Python, so it cannot ship in the browser.

## Limits

- One author wrote every reference annotation. A native speaker should review `tools/grading_evaluation/cases/`.
- The unseen set has 26 mistakes, so one miss moves recall by about 4 points.
- Correction agreement counts only exact matches with an accepted correction, so it understates valid alternatives.
- Explanations were not scored; `review-sample.md` from `report` supports rating them by hand.
- Phone feasibility was reasoned from memory limits, not tested.

## Next steps

- Native-speaker review of the cases, then rerun.
- Rate the explanations of `gemma3:12b` by hand.
- Try Kev fine-tuned on the labelled findings, and newer small models, against the same cases.
