# Grading evaluation

## Error detection

| Model      | Prompt       | Precision | Recall | F1  | False positive rate | Quotes not in answer | Category | Correction | Imprecise quote | Duplicates |
| ---------- | ------------ | --------- | ------ | --- | ------------------- | -------------------- | -------- | ---------- | --------------- | ---------- |
| gemma3:12b | few_shot_six | 91%       | 77%    | 84% | 2%                  | 0%                   | 84%      | 71%        | 0%              | 1%         |
| gemma3:12b | few_shot     | 90%       | 77%    | 83% | 4%                  | 0%                   | 78%      | 68%        | 0%              | 2%         |

- Precision: findings that match a real mistake ÷ (those findings + findings that match none); a repeat finding of an already matched mistake counts in neither
- Recall: real mistakes matched by a finding ÷ all real mistakes
- False positive rate: correct answers with at least one finding ÷ all correct answers

## Counts behind the rates

| Model      | Prompt       | Answers graded | Reference mistakes | Found correctly | False alarms | Quotes not in answer | Correct answers | Correct answers flagged |
| ---------- | ------------ | -------------- | ------------------ | --------------- | ------------ | -------------------- | --------------- | ----------------------- |
| gemma3:12b | few_shot_six | 342            | 204                | 158             | 15           | 0                    | 135             | 3                       |
| gemma3:12b | few_shot     | 342            | 204                | 157             | 17           | 0                    | 135             | 6                       |

## Behavior and cost

| Model      | Prompt       | Parameters | Download | Valid response | On-topic flag | Language flag | Unstable cases | p50 s | p95 s | Tokens/s |
| ---------- | ------------ | ---------- | -------- | -------------- | ------------- | ------------- | -------------- | ----- | ----- | -------- |
| gemma3:12b | few_shot_six | 11.8B      | 8.2 GB   | 100%           | 98%           | 100%          | 6%             | 1.5   | 2.7   | 46       |
| gemma3:12b | few_shot     | 11.8B      | 8.2 GB   | 100%           | 97%           | 100%          | 3%             | 1.3   | 2.3   | 45       |

## Recall by error category

| Model      | Prompt       | spelling    | verb_form   | ser_estar   | agreement   | article_preposition | word_choice | word_order | pronoun     | not_brazilian | content_mismatch |
| ---------- | ------------ | ----------- | ----------- | ----------- | ----------- | ------------------- | ----------- | ---------- | ----------- | ------------- | ---------------- |
| gemma3:12b | few_shot_six | 75% (18/24) | 79% (38/48) | 80% (12/15) | 89% (24/27) | 75% (18/24)         | 40% (6/15)  | 100% (9/9) | 57% (12/21) | 100% (12/12)  | 100% (9/9)       |
| gemma3:12b | few_shot     | 75% (18/24) | 75% (36/48) | 80% (12/15) | 81% (22/27) | 88% (21/24)         | 40% (6/15)  | 100% (9/9) | 57% (12/21) | 100% (12/12)  | 100% (9/9)       |

## Bar

- False positive rate at most 10%
- Quotes not in the answer at most 2%
- Precision at least 70%
- Recall at least 60%
- Correction agreement at least 60%
- p95 latency at most 15 s

| Model      | Prompt       | Control | Quotes | Precision | Recall | Correction | Latency | Meets all |
| ---------- | ------------ | ------- | ------ | --------- | ------ | ---------- | ------- | --------- |
| gemma3:12b | few_shot_six | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| gemma3:12b | few_shot     | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
