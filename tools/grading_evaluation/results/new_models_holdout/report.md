# Grading evaluation

## Error detection

| Model             | Prompt       | Precision | Recall | F1  | False positive rate | Quotes not in answer | Category | Correction | Imprecise quote | Duplicates |
| ----------------- | ------------ | --------- | ------ | --- | ------------------- | -------------------- | -------- | ---------- | --------------- | ---------- |
| gemma4:12b        | few_shot     | 96%       | 96%    | 96% | 5%                  | 0%                   | 96%      | 56%        | 0%              | 4%         |
| gemma4:12b        | plain        | 89%       | 96%    | 93% | 5%                  | 0%                   | 88%      | 60%        | 0%              | 3%         |
| gemma4:12b-it-qat | few_shot     | 92%       | 92%    | 92% | 5%                  | 0%                   | 96%      | 62%        | 0%              | 4%         |
| gemma4:12b-it-qat | few_shot_six | 96%       | 85%    | 90% | 5%                  | 0%                   | 95%      | 64%        | 0%              | 4%         |
| gemma4:12b        | few_shot_six | 96%       | 85%    | 90% | 5%                  | 0%                   | 95%      | 64%        | 0%              | 4%         |
| gemma4:12b-it-qat | plain        | 86%       | 92%    | 89% | 5%                  | 0%                   | 92%      | 62%        | 0%              | 3%         |
| qwen3.5:4b        | few_shot     | 82%       | 88%    | 85% | 0%                  | 0%                   | 78%      | 39%        | 0%              | 3%         |
| ministral-3:14b   | few_shot_six | 79%       | 85%    | 81% | 14%                 | 0%                   | 91%      | 55%        | 0%              | 3%         |
| gemma4:e4b        | few_shot     | 90%       | 73%    | 81% | 5%                  | 0%                   | 89%      | 63%        | 0%              | 5%         |
| gemma4:e4b-it-qat | few_shot_six | 95%       | 69%    | 80% | 0%                  | 0%                   | 83%      | 61%        | 0%              | 0%         |
| qwen3.5:4b        | plain        | 83%       | 73%    | 78% | 0%                  | 0%                   | 74%      | 47%        | 0%              | 4%         |
| gemma4:e4b        | few_shot_six | 77%       | 77%    | 77% | 9%                  | 0%                   | 95%      | 60%        | 0%              | 4%         |
| qwen3.5:4b        | few_shot_six | 79%       | 73%    | 76% | 0%                  | 0%                   | 95%      | 37%        | 0%              | 4%         |
| ministral-3:14b   | few_shot     | 74%       | 77%    | 75% | 9%                  | 0%                   | 85%      | 65%        | 0%              | 4%         |
| gemma4:e4b        | plain        | 68%       | 73%    | 70% | 18%                 | 0%                   | 84%      | 63%        | 0%              | 3%         |
| gemma3n:e4b       | few_shot_six | 80%       | 62%    | 70% | 0%                  | 0%                   | 69%      | 50%        | 0%              | 0%         |
| gemma4:e4b-it-qat | plain        | 74%       | 65%    | 69% | 9%                  | 0%                   | 88%      | 47%        | 0%              | 4%         |
| gemma3n:e4b       | few_shot     | 79%       | 58%    | 67% | 0%                  | 0%                   | 73%      | 60%        | 0%              | 0%         |
| gemma3n:e4b       | plain        | 45%       | 88%    | 60% | 100%                | 0%                   | 65%      | 43%        | 0%              | 2%         |
| ministral-3:14b   | plain        | 45%       | 88%    | 60% | 82%                 | 0%                   | 87%      | 65%        | 0%              | 2%         |
| gemma4:e4b-it-qat | few_shot     | 100%      | 42%    | 59% | 0%                  | 0%                   | 82%      | 55%        | 0%              | 0%         |
| gemma4:e2b        | few_shot     | 58%       | 54%    | 56% | 23%                 | 0%                   | 93%      | 57%        | 0%              | 0%         |
| gemma4:e2b-it-qat | plain        | 42%       | 77%    | 54% | 95%                 | 0%                   | 65%      | 55%        | 5%              | 0%         |
| gemma4:e2b-it-qat | few_shot     | 73%       | 42%    | 54% | 5%                  | 0%                   | 91%      | 73%        | 0%              | 0%         |
| gemma3n:e2b       | few_shot     | 63%       | 46%    | 53% | 5%                  | 0%                   | 67%      | 58%        | 0%              | 0%         |
| gemma3n:e2b       | few_shot_six | 65%       | 42%    | 51% | 5%                  | 0%                   | 91%      | 64%        | 0%              | 0%         |
| gemma4:e2b        | few_shot_six | 48%       | 54%    | 51% | 23%                 | 0%                   | 93%      | 64%        | 0%              | 0%         |
| gemma4:e2b-it-qat | few_shot_six | 61%       | 42%    | 50% | 14%                 | 0%                   | 100%     | 73%        | 0%              | 0%         |
| gemma4:e2b        | plain        | 30%       | 65%    | 41% | 100%                | 0%                   | 59%      | 76%        | 24%             | 0%         |
| gemma3n:e2b       | plain        | 29%       | 58%    | 38% | 91%                 | 0%                   | 47%      | 47%        | 0%              | 2%         |
| granite4.2:3b     | plain        | 100%      | 19%    | 32% | 0%                  | 0%                   | 80%      | 60%        | 20%             | 17%        |
| granite4.2:3b     | few_shot     | 100%      | 15%    | 27% | 0%                  | 0%                   | 75%      | 25%        | 0%              | 20%        |
| granite4.2:3b     | few_shot_six | 80%       | 15%    | 26% | 0%                  | 14%                  | 75%      | 25%        | 0%              | 29%        |

- Precision: findings that match a real mistake ÷ (those findings + findings that match none); a repeat finding of an already matched mistake counts in neither
- Recall: real mistakes matched by a finding ÷ all real mistakes
- False positive rate: correct answers with at least one finding ÷ all correct answers

## Counts behind the rates

| Model             | Prompt       | Answers graded | Reference mistakes | Found correctly | False alarms | Quotes not in answer | Correct answers | Correct answers flagged |
| ----------------- | ------------ | -------------- | ------------------ | --------------- | ------------ | -------------------- | --------------- | ----------------------- |
| gemma4:12b        | few_shot     | 52             | 26                 | 25              | 1            | 0                    | 22              | 1                       |
| gemma4:12b        | plain        | 52             | 26                 | 25              | 3            | 0                    | 22              | 1                       |
| gemma4:12b-it-qat | few_shot     | 52             | 26                 | 24              | 2            | 0                    | 22              | 1                       |
| gemma4:12b-it-qat | few_shot_six | 52             | 26                 | 22              | 1            | 0                    | 22              | 1                       |
| gemma4:12b        | few_shot_six | 52             | 26                 | 22              | 1            | 0                    | 22              | 1                       |
| gemma4:12b-it-qat | plain        | 52             | 26                 | 24              | 4            | 0                    | 22              | 1                       |
| qwen3.5:4b        | few_shot     | 52             | 26                 | 23              | 5            | 0                    | 22              | 0                       |
| ministral-3:14b   | few_shot_six | 52             | 26                 | 22              | 6            | 0                    | 22              | 3                       |
| gemma4:e4b        | few_shot     | 52             | 26                 | 19              | 2            | 0                    | 22              | 1                       |
| gemma4:e4b-it-qat | few_shot_six | 52             | 26                 | 18              | 1            | 0                    | 22              | 0                       |
| qwen3.5:4b        | plain        | 52             | 26                 | 19              | 4            | 0                    | 22              | 0                       |
| gemma4:e4b        | few_shot_six | 52             | 26                 | 20              | 6            | 0                    | 22              | 2                       |
| qwen3.5:4b        | few_shot_six | 52             | 26                 | 19              | 5            | 0                    | 22              | 0                       |
| ministral-3:14b   | few_shot     | 52             | 26                 | 20              | 7            | 0                    | 22              | 2                       |
| gemma4:e4b        | plain        | 52             | 26                 | 19              | 9            | 0                    | 22              | 4                       |
| gemma3n:e4b       | few_shot_six | 52             | 26                 | 16              | 4            | 0                    | 22              | 0                       |
| gemma4:e4b-it-qat | plain        | 52             | 26                 | 17              | 6            | 0                    | 22              | 2                       |
| gemma3n:e4b       | few_shot     | 52             | 26                 | 15              | 4            | 0                    | 22              | 0                       |
| gemma3n:e4b       | plain        | 52             | 26                 | 23              | 28           | 0                    | 22              | 22                      |
| ministral-3:14b   | plain        | 52             | 26                 | 23              | 28           | 0                    | 22              | 18                      |
| gemma4:e4b-it-qat | few_shot     | 52             | 26                 | 11              | 0            | 0                    | 22              | 0                       |
| gemma4:e2b        | few_shot     | 52             | 26                 | 14              | 10           | 0                    | 22              | 5                       |
| gemma4:e2b-it-qat | plain        | 52             | 26                 | 20              | 28           | 0                    | 22              | 21                      |
| gemma4:e2b-it-qat | few_shot     | 52             | 26                 | 11              | 4            | 0                    | 22              | 1                       |
| gemma3n:e2b       | few_shot     | 52             | 26                 | 12              | 7            | 0                    | 22              | 1                       |
| gemma3n:e2b       | few_shot_six | 52             | 26                 | 11              | 6            | 0                    | 22              | 1                       |
| gemma4:e2b        | few_shot_six | 52             | 26                 | 14              | 15           | 0                    | 22              | 5                       |
| gemma4:e2b-it-qat | few_shot_six | 52             | 26                 | 11              | 7            | 0                    | 22              | 3                       |
| gemma4:e2b        | plain        | 52             | 26                 | 17              | 40           | 0                    | 22              | 22                      |
| gemma3n:e2b       | plain        | 52             | 26                 | 15              | 37           | 0                    | 22              | 20                      |
| granite4.2:3b     | plain        | 52             | 26                 | 5               | 0            | 0                    | 22              | 0                       |
| granite4.2:3b     | few_shot     | 52             | 26                 | 4               | 0            | 0                    | 22              | 0                       |
| granite4.2:3b     | few_shot_six | 52             | 26                 | 4               | 0            | 1                    | 22              | 0                       |

## Behavior and cost

| Model             | Prompt       | Parameters | Download | Valid response | On-topic flag | Language flag | Unstable cases | p50 s | p95 s | Tokens/s |
| ----------------- | ------------ | ---------- | -------- | -------------- | ------------- | ------------- | -------------- | ----- | ----- | -------- |
| gemma4:12b        | few_shot     | 12.4B      | 7.7 GB   | 100%           | 100%          | 100%          | n/a            | 0.8   | 1.4   | 74       |
| gemma4:12b        | plain        | 12.4B      | 7.7 GB   | 100%           | 100%          | 100%          | n/a            | 1.2   | 2.1   | 63       |
| gemma4:12b-it-qat | few_shot     | 11.9B      | 7.2 GB   | 100%           | 100%          | 100%          | n/a            | 1.4   | 2.6   | 48       |
| gemma4:12b-it-qat | few_shot_six | 11.9B      | 7.2 GB   | 100%           | 100%          | 100%          | n/a            | 2.6   | 3.5   | 45       |
| gemma4:12b        | few_shot_six | 12.4B      | 7.7 GB   | 100%           | 100%          | 100%          | n/a            | 0.5   | 2.0   | 70       |
| gemma4:12b-it-qat | plain        | 11.9B      | 7.2 GB   | 100%           | 100%          | 100%          | n/a            | 2.8   | 4.1   | 41       |
| qwen3.5:4b        | few_shot     | 4.5B       | 4.0 GB   | 100%           | 98%           | 98%           | n/a            | 0.5   | 1.0   | 102      |
| ministral-3:14b   | few_shot_six | 13.9B      | 9.1 GB   | 98%            | 98%           | 98%           | n/a            | 1.6   | 3.4   | 34       |
| gemma4:e4b        | few_shot     | 8.1B       | 9.5 GB   | 96%            | 83%           | 92%           | n/a            | 0.2   | 1.4   | 216      |
| gemma4:e4b-it-qat | few_shot_six | 7.5B       | 6.1 GB   | 100%           | 96%           | 98%           | n/a            | 0.5   | 1.5   | 67       |
| qwen3.5:4b        | plain        | 4.5B       | 4.0 GB   | 100%           | 100%          | 100%          | n/a            | 0.4   | 1.1   | 101      |
| gemma4:e4b        | few_shot_six | 8.1B       | 9.5 GB   | 100%           | 92%           | 96%           | n/a            | 0.3   | 0.5   | 185      |
| qwen3.5:4b        | few_shot_six | 4.5B       | 4.0 GB   | 100%           | 96%           | 96%           | n/a            | 0.5   | 1.1   | 102      |
| ministral-3:14b   | few_shot     | 13.9B      | 9.1 GB   | 100%           | 100%          | 100%          | n/a            | 1.5   | 4.0   | 33       |
| gemma4:e4b        | plain        | 8.1B       | 9.5 GB   | 100%           | 98%           | 100%          | n/a            | 0.5   | 0.7   | 162      |
| gemma3n:e4b       | few_shot_six | 6.9B       | 9.8 GB   | 100%           | 100%          | 98%           | n/a            | 1.2   | 2.7   | 30       |
| gemma4:e4b-it-qat | plain        | 7.5B       | 6.1 GB   | 100%           | 96%           | 100%          | n/a            | 1.0   | 2.2   | 67       |
| gemma3n:e4b       | few_shot     | 6.9B       | 9.8 GB   | 100%           | 98%           | 100%          | n/a            | 1.2   | 2.8   | 29       |
| gemma3n:e4b       | plain        | 6.9B       | 9.8 GB   | 100%           | 96%           | 88%           | n/a            | 3.7   | 5.0   | 34       |
| ministral-3:14b   | plain        | 13.9B      | 9.1 GB   | 100%           | 100%          | 100%          | n/a            | 2.7   | 5.8   | 36       |
| gemma4:e4b-it-qat | few_shot     | 7.5B       | 6.1 GB   | 100%           | 98%           | 100%          | n/a            | 0.4   | 1.0   | 71       |
| gemma4:e2b        | few_shot     | 5.2B       | 7.5 GB   | 100%           | 81%           | 100%          | n/a            | 0.2   | 0.7   | 192      |
| gemma4:e2b-it-qat | plain        | 4.6B       | 4.3 GB   | 100%           | 100%          | 100%          | n/a            | 0.9   | 1.5   | 103      |
| gemma4:e2b-it-qat | few_shot     | 4.6B       | 4.3 GB   | 100%           | 92%           | 96%           | n/a            | 0.2   | 1.1   | 110      |
| gemma3n:e2b       | few_shot     | 4.5B       | 7.8 GB   | 100%           | 98%           | 100%          | n/a            | 0.5   | 1.4   | 64       |
| gemma3n:e2b       | few_shot_six | 4.5B       | 7.8 GB   | 100%           | 98%           | 100%          | n/a            | 0.6   | 1.4   | 55       |
| gemma4:e2b        | few_shot_six | 5.2B       | 7.5 GB   | 100%           | 85%           | 100%          | n/a            | 0.2   | 0.9   | 185      |
| gemma4:e2b-it-qat | few_shot_six | 4.6B       | 4.3 GB   | 100%           | 94%           | 96%           | n/a            | 0.3   | 0.8   | 120      |
| gemma4:e2b        | plain        | 5.2B       | 7.5 GB   | 98%            | 96%           | 15%           | n/a            | 0.5   | 0.9   | 208      |
| gemma3n:e2b       | plain        | 4.5B       | 7.8 GB   | 100%           | 96%           | 96%           | n/a            | 1.8   | 2.9   | 69       |
| granite4.2:3b     | plain        | 3.7B       | 2.2 GB   | 100%           | 96%           | 98%           | n/a            | 0.6   | 2.0   | 66       |
| granite4.2:3b     | few_shot     | 3.7B       | 2.2 GB   | 100%           | 96%           | 100%          | n/a            | 0.6   | 2.0   | 69       |
| granite4.2:3b     | few_shot_six | 3.7B       | 2.2 GB   | 100%           | 96%           | 100%          | n/a            | 0.6   | 1.9   | 73       |

## Recall by error category

| Model             | Prompt       | spelling   | verb_form  | ser_estar  | agreement  | article_preposition | word_choice | word_order | pronoun    | not_brazilian | content_mismatch |
| ----------------- | ------------ | ---------- | ---------- | ---------- | ---------- | ------------------- | ----------- | ---------- | ---------- | ------------- | ---------------- |
| gemma4:12b        | few_shot     | 100% (4/4) | 100% (7/7) | 100% (1/1) | 100% (2/2) | 75% (3/4)           | 100% (2/2)  | 100% (1/1) | 100% (2/2) | 100% (2/2)    | 100% (1/1)       |
| gemma4:12b        | plain        | 100% (4/4) | 100% (7/7) | 100% (1/1) | 100% (2/2) | 75% (3/4)           | 100% (2/2)  | 100% (1/1) | 100% (2/2) | 100% (2/2)    | 100% (1/1)       |
| gemma4:12b-it-qat | few_shot     | 100% (4/4) | 100% (7/7) | 100% (1/1) | 100% (2/2) | 75% (3/4)           | 50% (1/2)   | 100% (1/1) | 100% (2/2) | 100% (2/2)    | 100% (1/1)       |
| gemma4:12b-it-qat | few_shot_six | 100% (4/4) | 86% (6/7)  | 100% (1/1) | 100% (2/2) | 75% (3/4)           | 50% (1/2)   | 100% (1/1) | 50% (1/2)  | 100% (2/2)    | 100% (1/1)       |
| gemma4:12b        | few_shot_six | 100% (4/4) | 86% (6/7)  | 100% (1/1) | 100% (2/2) | 75% (3/4)           | 50% (1/2)   | 100% (1/1) | 50% (1/2)  | 100% (2/2)    | 100% (1/1)       |
| gemma4:12b-it-qat | plain        | 100% (4/4) | 100% (7/7) | 100% (1/1) | 100% (2/2) | 75% (3/4)           | 50% (1/2)   | 100% (1/1) | 100% (2/2) | 100% (2/2)    | 100% (1/1)       |
| qwen3.5:4b        | few_shot     | 100% (4/4) | 86% (6/7)  | 100% (1/1) | 100% (2/2) | 75% (3/4)           | 100% (2/2)  | 100% (1/1) | 100% (2/2) | 50% (1/2)     | 100% (1/1)       |
| ministral-3:14b   | few_shot_six | 100% (4/4) | 86% (6/7)  | 100% (1/1) | 100% (2/2) | 75% (3/4)           | 100% (2/2)  | 0% (0/1)   | 100% (2/2) | 50% (1/2)     | 100% (1/1)       |
| gemma4:e4b        | few_shot     | 75% (3/4)  | 100% (7/7) | 100% (1/1) | 100% (2/2) | 75% (3/4)           | 50% (1/2)   | 0% (0/1)   | 0% (0/2)   | 50% (1/2)     | 100% (1/1)       |
| gemma4:e4b-it-qat | few_shot_six | 75% (3/4)  | 100% (7/7) | 100% (1/1) | 50% (1/2)  | 50% (2/4)           | 100% (2/2)  | 0% (0/1)   | 50% (1/2)  | 50% (1/2)     | 0% (0/1)         |
| qwen3.5:4b        | plain        | 50% (2/4)  | 71% (5/7)  | 100% (1/1) | 100% (2/2) | 75% (3/4)           | 100% (2/2)  | 100% (1/1) | 50% (1/2)  | 50% (1/2)     | 100% (1/1)       |
| gemma4:e4b        | few_shot_six | 100% (4/4) | 100% (7/7) | 100% (1/1) | 100% (2/2) | 75% (3/4)           | 100% (2/2)  | 0% (0/1)   | 0% (0/2)   | 0% (0/2)      | 100% (1/1)       |
| qwen3.5:4b        | few_shot_six | 75% (3/4)  | 86% (6/7)  | 100% (1/1) | 50% (1/2)  | 75% (3/4)           | 50% (1/2)   | 100% (1/1) | 50% (1/2)  | 50% (1/2)     | 100% (1/1)       |
| ministral-3:14b   | few_shot     | 100% (4/4) | 100% (7/7) | 100% (1/1) | 100% (2/2) | 75% (3/4)           | 50% (1/2)   | 0% (0/1)   | 0% (0/2)   | 50% (1/2)     | 100% (1/1)       |
| gemma4:e4b        | plain        | 75% (3/4)  | 100% (7/7) | 100% (1/1) | 100% (2/2) | 50% (2/4)           | 50% (1/2)   | 0% (0/1)   | 50% (1/2)  | 50% (1/2)     | 100% (1/1)       |
| gemma3n:e4b       | few_shot_six | 75% (3/4)  | 71% (5/7)  | 100% (1/1) | 100% (2/2) | 75% (3/4)           | 50% (1/2)   | 0% (0/1)   | 0% (0/2)   | 0% (0/2)      | 100% (1/1)       |
| gemma4:e4b-it-qat | plain        | 50% (2/4)  | 100% (7/7) | 100% (1/1) | 100% (2/2) | 50% (2/4)           | 0% (0/2)    | 0% (0/1)   | 50% (1/2)  | 50% (1/2)     | 100% (1/1)       |
| gemma3n:e4b       | few_shot     | 50% (2/4)  | 86% (6/7)  | 100% (1/1) | 100% (2/2) | 75% (3/4)           | 0% (0/2)    | 0% (0/1)   | 0% (0/2)   | 0% (0/2)      | 100% (1/1)       |
| gemma3n:e4b       | plain        | 100% (4/4) | 86% (6/7)  | 100% (1/1) | 100% (2/2) | 75% (3/4)           | 100% (2/2)  | 100% (1/1) | 100% (2/2) | 50% (1/2)     | 100% (1/1)       |
| ministral-3:14b   | plain        | 100% (4/4) | 100% (7/7) | 100% (1/1) | 100% (2/2) | 75% (3/4)           | 100% (2/2)  | 0% (0/1)   | 50% (1/2)  | 100% (2/2)    | 100% (1/1)       |
| gemma4:e4b-it-qat | few_shot     | 0% (0/4)   | 71% (5/7)  | 100% (1/1) | 100% (2/2) | 50% (2/4)           | 0% (0/2)    | 0% (0/1)   | 0% (0/2)   | 50% (1/2)     | 0% (0/1)         |
| gemma4:e2b        | few_shot     | 75% (3/4)  | 71% (5/7)  | 0% (0/1)   | 100% (2/2) | 50% (2/4)           | 0% (0/2)    | 0% (0/1)   | 50% (1/2)  | 0% (0/2)      | 100% (1/1)       |
| gemma4:e2b-it-qat | plain        | 75% (3/4)  | 86% (6/7)  | 100% (1/1) | 100% (2/2) | 100% (4/4)          | 0% (0/2)    | 0% (0/1)   | 100% (2/2) | 50% (1/2)     | 100% (1/1)       |
| gemma4:e2b-it-qat | few_shot     | 50% (2/4)  | 43% (3/7)  | 0% (0/1)   | 100% (2/2) | 50% (2/4)           | 0% (0/2)    | 100% (1/1) | 0% (0/2)   | 0% (0/2)      | 100% (1/1)       |
| gemma3n:e2b       | few_shot     | 75% (3/4)  | 71% (5/7)  | 100% (1/1) | 50% (1/2)  | 25% (1/4)           | 0% (0/2)    | 0% (0/1)   | 0% (0/2)   | 0% (0/2)      | 100% (1/1)       |
| gemma3n:e2b       | few_shot_six | 50% (2/4)  | 71% (5/7)  | 0% (0/1)   | 50% (1/2)  | 50% (2/4)           | 0% (0/2)    | 0% (0/1)   | 0% (0/2)   | 0% (0/2)      | 100% (1/1)       |
| gemma4:e2b        | few_shot_six | 75% (3/4)  | 71% (5/7)  | 0% (0/1)   | 100% (2/2) | 50% (2/4)           | 0% (0/2)    | 0% (0/1)   | 50% (1/2)  | 0% (0/2)      | 100% (1/1)       |
| gemma4:e2b-it-qat | few_shot_six | 75% (3/4)  | 43% (3/7)  | 0% (0/1)   | 100% (2/2) | 50% (2/4)           | 0% (0/2)    | 0% (0/1)   | 0% (0/2)   | 0% (0/2)      | 100% (1/1)       |
| gemma4:e2b        | plain        | 50% (2/4)  | 71% (5/7)  | 100% (1/1) | 100% (2/2) | 75% (3/4)           | 0% (0/2)    | 0% (0/1)   | 100% (2/2) | 50% (1/2)     | 100% (1/1)       |
| gemma3n:e2b       | plain        | 75% (3/4)  | 86% (6/7)  | 100% (1/1) | 100% (2/2) | 0% (0/4)            | 0% (0/2)    | 0% (0/1)   | 50% (1/2)  | 50% (1/2)     | 100% (1/1)       |
| granite4.2:3b     | plain        | 0% (0/4)   | 29% (2/7)  | 0% (0/1)   | 50% (1/2)  | 25% (1/4)           | 0% (0/2)    | 0% (0/1)   | 0% (0/2)   | 0% (0/2)      | 100% (1/1)       |
| granite4.2:3b     | few_shot     | 0% (0/4)   | 14% (1/7)  | 0% (0/1)   | 50% (1/2)  | 25% (1/4)           | 0% (0/2)    | 0% (0/1)   | 0% (0/2)   | 0% (0/2)      | 100% (1/1)       |
| granite4.2:3b     | few_shot_six | 0% (0/4)   | 14% (1/7)  | 0% (0/1)   | 50% (1/2)  | 25% (1/4)           | 0% (0/2)    | 0% (0/1)   | 0% (0/2)   | 0% (0/2)      | 100% (1/1)       |

## Bar

- False positive rate at most 10%
- Quotes not in the answer at most 2%
- Precision at least 70%
- Recall at least 60%
- Correction agreement at least 60%
- p95 latency at most 15 s

| Model             | Prompt       | Control | Quotes | Precision | Recall | Correction | Latency | Meets all |
| ----------------- | ------------ | ------- | ------ | --------- | ------ | ---------- | ------- | --------- |
| gemma4:12b        | few_shot     | yes     | yes    | yes       | yes    | no         | yes     | no        |
| gemma4:12b        | plain        | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| gemma4:12b-it-qat | few_shot     | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| gemma4:12b-it-qat | few_shot_six | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| gemma4:12b        | few_shot_six | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| gemma4:12b-it-qat | plain        | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| qwen3.5:4b        | few_shot     | yes     | yes    | yes       | yes    | no         | yes     | no        |
| ministral-3:14b   | few_shot_six | no      | yes    | yes       | yes    | no         | yes     | no        |
| gemma4:e4b        | few_shot     | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| gemma4:e4b-it-qat | few_shot_six | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| qwen3.5:4b        | plain        | yes     | yes    | yes       | yes    | no         | yes     | no        |
| gemma4:e4b        | few_shot_six | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| qwen3.5:4b        | few_shot_six | yes     | yes    | yes       | yes    | no         | yes     | no        |
| ministral-3:14b   | few_shot     | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| gemma4:e4b        | plain        | no      | yes    | no        | yes    | yes        | yes     | no        |
| gemma3n:e4b       | few_shot_six | yes     | yes    | yes       | yes    | no         | yes     | no        |
| gemma4:e4b-it-qat | plain        | yes     | yes    | yes       | yes    | no         | yes     | no        |
| gemma3n:e4b       | few_shot     | yes     | yes    | yes       | no     | yes        | yes     | no        |
| gemma3n:e4b       | plain        | no      | yes    | no        | yes    | no         | yes     | no        |
| ministral-3:14b   | plain        | no      | yes    | no        | yes    | yes        | yes     | no        |
| gemma4:e4b-it-qat | few_shot     | yes     | yes    | yes       | no     | no         | yes     | no        |
| gemma4:e2b        | few_shot     | no      | yes    | no        | no     | no         | yes     | no        |
| gemma4:e2b-it-qat | plain        | no      | yes    | no        | yes    | no         | yes     | no        |
| gemma4:e2b-it-qat | few_shot     | yes     | yes    | yes       | no     | yes        | yes     | no        |
| gemma3n:e2b       | few_shot     | yes     | yes    | no        | no     | no         | yes     | no        |
| gemma3n:e2b       | few_shot_six | yes     | yes    | no        | no     | yes        | yes     | no        |
| gemma4:e2b        | few_shot_six | no      | yes    | no        | no     | yes        | yes     | no        |
| gemma4:e2b-it-qat | few_shot_six | no      | yes    | no        | no     | yes        | yes     | no        |
| gemma4:e2b        | plain        | no      | yes    | no        | yes    | yes        | yes     | no        |
| gemma3n:e2b       | plain        | no      | yes    | no        | no     | no         | yes     | no        |
| granite4.2:3b     | plain        | yes     | yes    | yes       | no     | yes        | yes     | no        |
| granite4.2:3b     | few_shot     | yes     | yes    | yes       | no     | no         | yes     | no        |
| granite4.2:3b     | few_shot_six | yes     | no     | yes       | no     | no         | yes     | no        |
