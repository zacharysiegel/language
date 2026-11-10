# Grading evaluation

## Error detection

| Model      | Prompt       | Precision | Recall | F1  | False positive rate | Quotes not in answer | Category | Correction | Imprecise quote | Duplicates |
| ---------- | ------------ | --------- | ------ | --- | ------------------- | -------------------- | -------- | ---------- | --------------- | ---------- |
| gemma3:12b | few_shot     | 91%       | 81%    | 86% | 0%                  | 0%                   | 92%      | 52%        | 0%              | 1%         |
| gemma3:12b | few_shot_six | 91%       | 81%    | 86% | 0%                  | 0%                   | 89%      | 62%        | 0%              | 0%         |

- Precision: findings that match a real mistake ÷ (those findings + findings that match none); a repeat finding of an already matched mistake counts in neither
- Recall: real mistakes matched by a finding ÷ all real mistakes
- False positive rate: correct answers with at least one finding ÷ all correct answers

## Counts behind the rates

| Model      | Prompt       | Answers graded | Reference mistakes | Found correctly | False alarms | Quotes not in answer | Correct answers | Correct answers flagged |
| ---------- | ------------ | -------------- | ------------------ | --------------- | ------------ | -------------------- | --------------- | ----------------------- |
| gemma3:12b | few_shot     | 156            | 78                 | 63              | 6            | 0                    | 66              | 0                       |
| gemma3:12b | few_shot_six | 156            | 78                 | 63              | 6            | 0                    | 66              | 0                       |

## Behavior and cost

| Model      | Prompt       | Parameters | Download | Valid response | On-topic flag | Language flag | Unstable cases | p50 s | p95 s | Tokens/s |
| ---------- | ------------ | ---------- | -------- | -------------- | ------------- | ------------- | -------------- | ----- | ----- | -------- |
| gemma3:12b | few_shot     | 11.8B      | 8.2 GB   | 100%           | 100%          | 100%          | 0%             | 1.3   | 2.2   | 46       |
| gemma3:12b | few_shot_six | 11.8B      | 8.2 GB   | 100%           | 100%          | 100%          | 0%             | 1.4   | 2.7   | 45       |

## Recall by error category

| Model      | Prompt       | spelling     | verb_form    | ser_estar  | agreement  | article_preposition | word_choice | word_order | pronoun  | not_brazilian | content_mismatch |
| ---------- | ------------ | ------------ | ------------ | ---------- | ---------- | ------------------- | ----------- | ---------- | -------- | ------------- | ---------------- |
| gemma3:12b | few_shot     | 100% (12/12) | 100% (21/21) | 100% (3/3) | 100% (6/6) | 75% (9/12)          | 100% (6/6)  | 0% (0/3)   | 0% (0/6) | 50% (3/6)     | 100% (3/3)       |
| gemma3:12b | few_shot_six | 100% (12/12) | 100% (21/21) | 100% (3/3) | 100% (6/6) | 75% (9/12)          | 100% (6/6)  | 0% (0/3)   | 0% (0/6) | 50% (3/6)     | 100% (3/3)       |

## Bar

- False positive rate at most 10%
- Quotes not in the answer at most 2%
- Precision at least 70%
- Recall at least 60%
- Correction agreement at least 60%
- p95 latency at most 15 s

| Model      | Prompt       | Control | Quotes | Precision | Recall | Correction | Latency | Meets all |
| ---------- | ------------ | ------- | ------ | --------- | ------ | ---------- | ------- | --------- |
| gemma3:12b | few_shot     | yes     | yes    | yes       | yes    | no         | yes     | no        |
| gemma3:12b | few_shot_six | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
