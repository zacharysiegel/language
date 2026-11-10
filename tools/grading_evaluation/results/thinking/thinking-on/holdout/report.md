# Grading evaluation

## Error detection

| Model                                           | Prompt       | Precision | Recall | F1  | False positive rate | Quotes not in answer | Category | Correction | Imprecise quote | Duplicates |
| ----------------------------------------------- | ------------ | --------- | ------ | --- | ------------------- | -------------------- | -------- | ---------- | --------------- | ---------- |
| cross-llama:gemma-4-12b-it-qat-q4_0:thinking-on | few_shot_six | 96%       | 96%    | 96% | 5%                  | 0%                   | 100%     | 56%        | 0%              | 4%         |

- Precision: findings that match a real mistake ÷ (those findings + findings that match none); a repeat finding of an already matched mistake counts in neither
- Recall: real mistakes matched by a finding ÷ all real mistakes
- False positive rate: correct answers with at least one finding ÷ all correct answers

## Counts behind the rates

| Model                                           | Prompt       | Answers graded | Reference mistakes | Found correctly | False alarms | Quotes not in answer | Correct answers | Correct answers flagged |
| ----------------------------------------------- | ------------ | -------------- | ------------------ | --------------- | ------------ | -------------------- | --------------- | ----------------------- |
| cross-llama:gemma-4-12b-it-qat-q4_0:thinking-on | few_shot_six | 52             | 26                 | 25              | 1            | 0                    | 22              | 1                       |

## Behavior and cost

| Model                                           | Prompt       | Parameters | Download | Valid response | On-topic flag | Language flag | Unstable cases | p50 s | p95 s | Reasoning tokens | Reasoning p95 s | Tokens/s |
| ----------------------------------------------- | ------------ | ---------- | -------- | -------------- | ------------- | ------------- | -------------- | ----- | ----- | ---------------- | --------------- | -------- |
| cross-llama:gemma-4-12b-it-qat-q4_0:thinking-on | few_shot_six |            | 7.0 GB   | 100%           | 100%          | 100%          | n/a            | 6.9   | 178.2 | 927              | 176.6           | 45       |

## Recall by error category

| Model                                           | Prompt       | spelling   | verb_form  | ser_estar  | agreement  | article_preposition | word_choice | word_order | pronoun    | not_brazilian | content_mismatch |
| ----------------------------------------------- | ------------ | ---------- | ---------- | ---------- | ---------- | ------------------- | ----------- | ---------- | ---------- | ------------- | ---------------- |
| cross-llama:gemma-4-12b-it-qat-q4_0:thinking-on | few_shot_six | 100% (4/4) | 100% (7/7) | 100% (1/1) | 100% (2/2) | 75% (3/4)           | 100% (2/2)  | 100% (1/1) | 100% (2/2) | 100% (2/2)    | 100% (1/1)       |

## Bar

- False positive rate at most 10%
- Quotes not in the answer at most 2%
- Precision at least 70%
- Recall at least 60%
- Correction agreement at least 60%
- p95 latency at most 15 s

| Model                                           | Prompt       | Control | Quotes | Precision | Recall | Correction | Latency | Meets all |
| ----------------------------------------------- | ------------ | ------- | ------ | --------- | ------ | ---------- | ------- | --------- |
| cross-llama:gemma-4-12b-it-qat-q4_0:thinking-on | few_shot_six | yes     | yes    | yes       | yes    | no         | no      | no        |
