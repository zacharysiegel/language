# Grading evaluation

## Error detection

| Model             | Prompt       | Precision | Recall | F1  | False positive rate | Quotes not in answer | Category | Correction | Imprecise quote | Duplicates |
| ----------------- | ------------ | --------- | ------ | --- | ------------------- | -------------------- | -------- | ---------- | --------------- | ---------- |
| gemma4:12b-it-qat | few_shot_six | 100%      | 90%    | 95% | 0%                  | 0%                   | 95%      | 70%        | 0%              | 3%         |
| gemma4:12b-it-qat | few_shot     | 98%       | 88%    | 93% | 0%                  | 0%                   | 93%      | 72%        | 0%              | 3%         |
| gemma4:12b        | few_shot     | 98%       | 88%    | 93% | 0%                  | 2%                   | 90%      | 82%        | 0%              | 0%         |
| gemma4:12b        | few_shot_six | 97%       | 87%    | 91% | 2%                  | 2%                   | 97%      | 76%        | 0%              | 0%         |
| gemma4:12b        | plain        | 87%       | 96%    | 91% | 9%                  | 1%                   | 88%      | 80%        | 0%              | 4%         |
| gemma4:12b-it-qat | plain        | 88%       | 94%    | 91% | 7%                  | 0%                   | 98%      | 80%        | 0%              | 3%         |
| ministral-3:14b   | few_shot_six | 88%       | 87%    | 87% | 9%                  | 0%                   | 90%      | 75%        | 0%              | 3%         |
| ministral-3:14b   | few_shot     | 82%       | 87%    | 84% | 13%                 | 0%                   | 90%      | 68%        | 0%              | 1%         |
| gemma4:e4b        | few_shot_six | 79%       | 79%    | 79% | 20%                 | 0%                   | 93%      | 83%        | 0%              | 1%         |
| gemma4:e4b        | few_shot     | 80%       | 75%    | 77% | 13%                 | 0%                   | 94%      | 84%        | 0%              | 2%         |
| gemma4:e4b-it-qat | plain        | 77%       | 75%    | 76% | 13%                 | 0%                   | 88%      | 76%        | 0%              | 1%         |
| gemma4:e4b        | plain        | 69%       | 81%    | 74% | 33%                 | 0%                   | 85%      | 71%        | 0%              | 1%         |
| qwen3.5:4b        | few_shot_six | 68%       | 76%    | 72% | 24%                 | 0%                   | 79%      | 46%        | 0%              | 1%         |
| gemma4:e4b-it-qat | few_shot     | 97%       | 56%    | 71% | 0%                  | 0%                   | 82%      | 76%        | 0%              | 0%         |
| qwen3.5:4b        | few_shot     | 62%       | 78%    | 69% | 29%                 | 0%                   | 79%      | 49%        | 0%              | 1%         |
| gemma4:e4b-it-qat | few_shot_six | 90%       | 54%    | 68% | 4%                  | 0%                   | 86%      | 76%        | 0%              | 0%         |
| gemma3n:e4b       | few_shot     | 80%       | 59%    | 68% | 13%                 | 0%                   | 78%      | 65%        | 0%              | 0%         |
| ministral-3:14b   | plain        | 52%       | 96%    | 67% | 93%                 | 0%                   | 78%      | 68%        | 0%              | 2%         |
| gemma3n:e4b       | few_shot_six | 75%       | 60%    | 67% | 20%                 | 0%                   | 66%      | 54%        | 0%              | 0%         |
| gemma4:e2b-it-qat | few_shot     | 81%       | 56%    | 66% | 9%                  | 0%                   | 79%      | 79%        | 0%              | 0%         |
| gemma3n:e4b       | plain        | 51%       | 87%    | 64% | 98%                 | 0%                   | 64%      | 61%        | 0%              | 1%         |
| qwen3.5:4b        | plain        | 67%       | 59%    | 62% | 7%                  | 0%                   | 75%      | 48%        | 0%              | 2%         |
| gemma4:e2b-it-qat | few_shot_six | 74%       | 50%    | 60% | 13%                 | 0%                   | 88%      | 82%        | 0%              | 0%         |
| gemma3n:e2b       | few_shot     | 64%       | 50%    | 56% | 9%                  | 0%                   | 65%      | 56%        | 0%              | 0%         |
| gemma4:e2b-it-qat | plain        | 44%       | 69%    | 54% | 93%                 | 1%                   | 70%      | 66%        | 0%              | 0%         |
| gemma4:e2b        | few_shot_six | 50%       | 57%    | 53% | 40%                 | 1%                   | 72%      | 67%        | 0%              | 0%         |
| gemma4:e2b        | few_shot     | 52%       | 53%    | 53% | 40%                 | 1%                   | 67%      | 72%        | 3%              | 0%         |
| gemma3n:e2b       | plain        | 38%       | 66%    | 48% | 98%                 | 1%                   | 47%      | 53%        | 0%              | 1%         |
| gemma3n:e2b       | few_shot_six | 59%       | 38%    | 46% | 7%                  | 0%                   | 81%      | 50%        | 0%              | 0%         |
| gemma4:e2b        | plain        | 32%       | 62%    | 42% | 98%                 | 0%                   | 64%      | 69%        | 7%              | 1%         |
| granite4.2:3b     | few_shot_six | 88%       | 10%    | 18% | 0%                  | 0%                   | 71%      | 29%        | 0%              | 0%         |
| granite4.2:3b     | plain        | 50%       | 7%     | 13% | 0%                  | 10%                  | 40%      | 20%        | 0%              | 0%         |
| granite4.2:3b     | few_shot     | 100%      | 3%     | 6%  | 0%                  | 0%                   | 100%     | 50%        | 0%              | 0%         |

- Precision: findings that match a real mistake ÷ (those findings + findings that match none); a repeat finding of an already matched mistake counts in neither
- Recall: real mistakes matched by a finding ÷ all real mistakes
- False positive rate: correct answers with at least one finding ÷ all correct answers

## Counts behind the rates

| Model             | Prompt       | Answers graded | Reference mistakes | Found correctly | False alarms | Quotes not in answer | Correct answers | Correct answers flagged |
| ----------------- | ------------ | -------------- | ------------------ | --------------- | ------------ | -------------------- | --------------- | ----------------------- |
| gemma4:12b-it-qat | few_shot_six | 114            | 68                 | 61              | 0            | 0                    | 45              | 0                       |
| gemma4:12b-it-qat | few_shot     | 114            | 68                 | 60              | 1            | 0                    | 45              | 0                       |
| gemma4:12b        | few_shot     | 114            | 68                 | 60              | 0            | 1                    | 45              | 0                       |
| gemma4:12b        | few_shot_six | 114            | 68                 | 59              | 1            | 1                    | 45              | 1                       |
| gemma4:12b        | plain        | 114            | 68                 | 65              | 9            | 1                    | 45              | 4                       |
| gemma4:12b-it-qat | plain        | 114            | 68                 | 64              | 9            | 0                    | 45              | 3                       |
| ministral-3:14b   | few_shot_six | 114            | 68                 | 59              | 8            | 0                    | 45              | 4                       |
| ministral-3:14b   | few_shot     | 114            | 68                 | 59              | 13           | 0                    | 45              | 6                       |
| gemma4:e4b        | few_shot_six | 114            | 68                 | 54              | 14           | 0                    | 45              | 9                       |
| gemma4:e4b        | few_shot     | 114            | 68                 | 51              | 13           | 0                    | 45              | 6                       |
| gemma4:e4b-it-qat | plain        | 114            | 68                 | 51              | 15           | 0                    | 45              | 6                       |
| gemma4:e4b        | plain        | 114            | 68                 | 55              | 25           | 0                    | 45              | 15                      |
| qwen3.5:4b        | few_shot_six | 114            | 68                 | 52              | 24           | 0                    | 45              | 11                      |
| gemma4:e4b-it-qat | few_shot     | 114            | 68                 | 38              | 1            | 0                    | 45              | 0                       |
| qwen3.5:4b        | few_shot     | 114            | 68                 | 53              | 32           | 0                    | 45              | 13                      |
| gemma4:e4b-it-qat | few_shot_six | 114            | 68                 | 37              | 4            | 0                    | 45              | 2                       |
| gemma3n:e4b       | few_shot     | 114            | 68                 | 40              | 10           | 0                    | 45              | 6                       |
| ministral-3:14b   | plain        | 114            | 68                 | 65              | 60           | 0                    | 45              | 42                      |
| gemma3n:e4b       | few_shot_six | 114            | 68                 | 41              | 14           | 0                    | 45              | 9                       |
| gemma4:e2b-it-qat | few_shot     | 114            | 68                 | 38              | 9            | 0                    | 45              | 4                       |
| gemma3n:e4b       | plain        | 114            | 68                 | 59              | 57           | 0                    | 45              | 44                      |
| qwen3.5:4b        | plain        | 114            | 68                 | 40              | 20           | 0                    | 45              | 3                       |
| gemma4:e2b-it-qat | few_shot_six | 114            | 68                 | 34              | 12           | 0                    | 45              | 6                       |
| gemma3n:e2b       | few_shot     | 114            | 68                 | 34              | 19           | 0                    | 45              | 4                       |
| gemma4:e2b-it-qat | plain        | 114            | 68                 | 47              | 59           | 1                    | 45              | 42                      |
| gemma4:e2b        | few_shot_six | 114            | 68                 | 39              | 38           | 1                    | 45              | 18                      |
| gemma4:e2b        | few_shot     | 114            | 68                 | 36              | 32           | 1                    | 45              | 18                      |
| gemma3n:e2b       | plain        | 114            | 68                 | 45              | 72           | 1                    | 45              | 44                      |
| gemma3n:e2b       | few_shot_six | 114            | 68                 | 26              | 18           | 0                    | 45              | 3                       |
| gemma4:e2b        | plain        | 114            | 68                 | 42              | 90           | 0                    | 45              | 44                      |
| granite4.2:3b     | few_shot_six | 114            | 68                 | 7               | 1            | 0                    | 45              | 0                       |
| granite4.2:3b     | plain        | 114            | 68                 | 5               | 4            | 1                    | 45              | 0                       |
| granite4.2:3b     | few_shot     | 114            | 68                 | 2               | 0            | 0                    | 45              | 0                       |

## Behavior and cost

| Model             | Prompt       | Parameters | Download | Valid response | On-topic flag | Language flag | Unstable cases | p50 s | p95 s | Tokens/s |
| ----------------- | ------------ | ---------- | -------- | -------------- | ------------- | ------------- | -------------- | ----- | ----- | -------- |
| gemma4:12b-it-qat | few_shot_six | 11.9B      | 7.2 GB   | 100%           | 100%          | 98%           | n/a            | 2.5   | 3.7   | 45       |
| gemma4:12b-it-qat | few_shot     | 11.9B      | 7.2 GB   | 100%           | 100%          | 98%           | n/a            | 2.0   | 3.4   | 43       |
| gemma4:12b        | few_shot     | 12.4B      | 7.7 GB   | 100%           | 100%          | 100%          | n/a            | 0.5   | 1.6   | 77       |
| gemma4:12b        | few_shot_six | 12.4B      | 7.7 GB   | 100%           | 100%          | 99%           | n/a            | 0.6   | 1.8   | 74       |
| gemma4:12b        | plain        | 12.4B      | 7.7 GB   | 100%           | 100%          | 100%          | n/a            | 1.3   | 2.5   | 56       |
| gemma4:12b-it-qat | plain        | 11.9B      | 7.2 GB   | 100%           | 100%          | 100%          | n/a            | 3.0   | 5.3   | 35       |
| ministral-3:14b   | few_shot_six | 13.9B      | 9.1 GB   | 100%           | 97%           | 99%           | n/a            | 1.7   | 4.1   | 32       |
| ministral-3:14b   | few_shot     | 13.9B      | 9.1 GB   | 100%           | 97%           | 100%          | n/a            | 1.8   | 4.4   | 35       |
| gemma4:e4b        | few_shot_six | 8.1B       | 9.5 GB   | 100%           | 95%           | 98%           | n/a            | 0.3   | 0.5   | 185      |
| gemma4:e4b        | few_shot     | 8.1B       | 9.5 GB   | 100%           | 90%           | 99%           | n/a            | 0.3   | 0.8   | 167      |
| gemma4:e4b-it-qat | plain        | 7.5B       | 6.1 GB   | 100%           | 96%           | 100%          | n/a            | 1.4   | 2.2   | 74       |
| gemma4:e4b        | plain        | 8.1B       | 9.5 GB   | 100%           | 98%           | 100%          | n/a            | 0.6   | 0.8   | 167      |
| qwen3.5:4b        | few_shot_six | 4.5B       | 4.0 GB   | 100%           | 99%           | 99%           | n/a            | 0.7   | 1.3   | 88       |
| gemma4:e4b-it-qat | few_shot     | 7.5B       | 6.1 GB   | 100%           | 96%           | 100%          | n/a            | 0.4   | 1.1   | 75       |
| qwen3.5:4b        | few_shot     | 4.5B       | 4.0 GB   | 100%           | 99%           | 99%           | n/a            | 0.7   | 1.5   | 91       |
| gemma4:e4b-it-qat | few_shot_six | 7.5B       | 6.1 GB   | 100%           | 97%           | 100%          | n/a            | 0.4   | 1.1   | 79       |
| gemma3n:e4b       | few_shot     | 6.9B       | 9.8 GB   | 100%           | 99%           | 100%          | n/a            | 0.9   | 1.8   | 52       |
| ministral-3:14b   | plain        | 13.9B      | 9.1 GB   | 100%           | 96%           | 99%           | n/a            | 3.1   | 6.1   | 33       |
| gemma3n:e4b       | few_shot_six | 6.9B       | 9.8 GB   | 100%           | 99%           | 100%          | n/a            | 0.5   | 1.1   | 71       |
| gemma4:e2b-it-qat | few_shot     | 4.6B       | 4.3 GB   | 100%           | 95%           | 99%           | n/a            | 0.3   | 0.8   | 123      |
| gemma3n:e4b       | plain        | 6.9B       | 9.8 GB   | 100%           | 98%           | 92%           | n/a            | 3.6   | 4.9   | 34       |
| qwen3.5:4b        | plain        | 4.5B       | 4.0 GB   | 100%           | 99%           | 98%           | n/a            | 0.7   | 1.3   | 97       |
| gemma4:e2b-it-qat | few_shot_six | 4.6B       | 4.3 GB   | 100%           | 97%           | 98%           | n/a            | 0.3   | 0.8   | 126      |
| gemma3n:e2b       | few_shot     | 4.5B       | 7.8 GB   | 100%           | 99%           | 100%          | n/a            | 0.5   | 1.4   | 64       |
| gemma4:e2b-it-qat | plain        | 4.6B       | 4.3 GB   | 100%           | 100%          | 100%          | n/a            | 0.9   | 1.3   | 108      |
| gemma4:e2b        | few_shot_six | 5.2B       | 7.5 GB   | 99%            | 88%           | 99%           | n/a            | 0.2   | 0.9   | 225      |
| gemma4:e2b        | few_shot     | 5.2B       | 7.5 GB   | 98%            | 80%           | 96%           | n/a            | 0.3   | 0.9   | 221      |
| gemma3n:e2b       | plain        | 4.5B       | 7.8 GB   | 100%           | 98%           | 96%           | n/a            | 2.1   | 3.3   | 59       |
| gemma3n:e2b       | few_shot_six | 4.5B       | 7.8 GB   | 100%           | 98%           | 100%          | n/a            | 0.6   | 1.4   | 55       |
| gemma4:e2b        | plain        | 5.2B       | 7.5 GB   | 97%            | 96%           | 7%            | n/a            | 0.4   | 0.9   | 226      |
| granite4.2:3b     | few_shot_six | 3.7B       | 2.2 GB   | 100%           | 96%           | 100%          | n/a            | 0.5   | 1.4   | 79       |
| granite4.2:3b     | plain        | 3.7B       | 2.2 GB   | 100%           | 96%           | 98%           | n/a            | 0.6   | 1.6   | 68       |
| granite4.2:3b     | few_shot     | 3.7B       | 2.2 GB   | 100%           | 96%           | 100%          | n/a            | 0.5   | 0.8   | 70       |

## Recall by error category

| Model             | Prompt       | spelling   | verb_form    | ser_estar  | agreement  | article_preposition | word_choice | word_order | pronoun    | not_brazilian | content_mismatch |
| ----------------- | ------------ | ---------- | ------------ | ---------- | ---------- | ------------------- | ----------- | ---------- | ---------- | ------------- | ---------------- |
| gemma4:12b-it-qat | few_shot_six | 75% (6/8)  | 94% (15/16)  | 80% (4/5)  | 100% (9/9) | 88% (7/8)           | 60% (3/5)   | 100% (3/3) | 100% (7/7) | 100% (4/4)    | 100% (3/3)       |
| gemma4:12b-it-qat | few_shot     | 75% (6/8)  | 88% (14/16)  | 80% (4/5)  | 100% (9/9) | 100% (8/8)          | 60% (3/5)   | 100% (3/3) | 86% (6/7)  | 100% (4/4)    | 100% (3/3)       |
| gemma4:12b        | few_shot     | 88% (7/8)  | 88% (14/16)  | 80% (4/5)  | 100% (9/9) | 88% (7/8)           | 60% (3/5)   | 100% (3/3) | 86% (6/7)  | 100% (4/4)    | 100% (3/3)       |
| gemma4:12b        | few_shot_six | 75% (6/8)  | 88% (14/16)  | 80% (4/5)  | 100% (9/9) | 88% (7/8)           | 60% (3/5)   | 100% (3/3) | 86% (6/7)  | 100% (4/4)    | 100% (3/3)       |
| gemma4:12b        | plain        | 100% (8/8) | 94% (15/16)  | 80% (4/5)  | 100% (9/9) | 100% (8/8)          | 80% (4/5)   | 100% (3/3) | 100% (7/7) | 100% (4/4)    | 100% (3/3)       |
| gemma4:12b-it-qat | plain        | 88% (7/8)  | 100% (16/16) | 80% (4/5)  | 100% (9/9) | 100% (8/8)          | 80% (4/5)   | 67% (2/3)  | 100% (7/7) | 100% (4/4)    | 100% (3/3)       |
| ministral-3:14b   | few_shot_six | 100% (8/8) | 94% (15/16)  | 80% (4/5)  | 89% (8/9)  | 100% (8/8)          | 40% (2/5)   | 100% (3/3) | 57% (4/7)  | 100% (4/4)    | 100% (3/3)       |
| ministral-3:14b   | few_shot     | 100% (8/8) | 94% (15/16)  | 80% (4/5)  | 100% (9/9) | 88% (7/8)           | 40% (2/5)   | 100% (3/3) | 57% (4/7)  | 100% (4/4)    | 100% (3/3)       |
| gemma4:e4b        | few_shot_six | 50% (4/8)  | 81% (13/16)  | 80% (4/5)  | 100% (9/9) | 88% (7/8)           | 60% (3/5)   | 67% (2/3)  | 71% (5/7)  | 100% (4/4)    | 100% (3/3)       |
| gemma4:e4b        | few_shot     | 75% (6/8)  | 69% (11/16)  | 80% (4/5)  | 100% (9/9) | 75% (6/8)           | 60% (3/5)   | 33% (1/3)  | 57% (4/7)  | 100% (4/4)    | 100% (3/3)       |
| gemma4:e4b-it-qat | plain        | 62% (5/8)  | 69% (11/16)  | 60% (3/5)  | 100% (9/9) | 100% (8/8)          | 60% (3/5)   | 67% (2/3)  | 43% (3/7)  | 100% (4/4)    | 100% (3/3)       |
| gemma4:e4b        | plain        | 75% (6/8)  | 88% (14/16)  | 80% (4/5)  | 100% (9/9) | 88% (7/8)           | 60% (3/5)   | 33% (1/3)  | 57% (4/7)  | 100% (4/4)    | 100% (3/3)       |
| qwen3.5:4b        | few_shot_six | 62% (5/8)  | 75% (12/16)  | 100% (5/5) | 89% (8/9)  | 75% (6/8)           | 60% (3/5)   | 67% (2/3)  | 71% (5/7)  | 75% (3/4)     | 100% (3/3)       |
| gemma4:e4b-it-qat | few_shot     | 38% (3/8)  | 38% (6/16)   | 60% (3/5)  | 100% (9/9) | 62% (5/8)           | 40% (2/5)   | 33% (1/3)  | 43% (3/7)  | 100% (4/4)    | 67% (2/3)        |
| qwen3.5:4b        | few_shot     | 75% (6/8)  | 75% (12/16)  | 100% (5/5) | 89% (8/9)  | 62% (5/8)           | 80% (4/5)   | 33% (1/3)  | 86% (6/7)  | 75% (3/4)     | 100% (3/3)       |
| gemma4:e4b-it-qat | few_shot_six | 25% (2/8)  | 56% (9/16)   | 60% (3/5)  | 78% (7/9)  | 62% (5/8)           | 60% (3/5)   | 33% (1/3)  | 29% (2/7)  | 100% (4/4)    | 33% (1/3)        |
| gemma3n:e4b       | few_shot     | 38% (3/8)  | 62% (10/16)  | 80% (4/5)  | 100% (9/9) | 62% (5/8)           | 40% (2/5)   | 33% (1/3)  | 29% (2/7)  | 50% (2/4)     | 67% (2/3)        |
| ministral-3:14b   | plain        | 100% (8/8) | 94% (15/16)  | 100% (5/5) | 100% (9/9) | 100% (8/8)          | 100% (5/5)  | 100% (3/3) | 71% (5/7)  | 100% (4/4)    | 100% (3/3)       |
| gemma3n:e4b       | few_shot_six | 38% (3/8)  | 62% (10/16)  | 80% (4/5)  | 100% (9/9) | 62% (5/8)           | 40% (2/5)   | 33% (1/3)  | 43% (3/7)  | 25% (1/4)     | 100% (3/3)       |
| gemma4:e2b-it-qat | few_shot     | 75% (6/8)  | 50% (8/16)   | 0% (0/5)   | 89% (8/9)  | 62% (5/8)           | 20% (1/5)   | 67% (2/3)  | 43% (3/7)  | 50% (2/4)     | 100% (3/3)       |
| gemma3n:e4b       | plain        | 75% (6/8)  | 81% (13/16)  | 100% (5/5) | 100% (9/9) | 88% (7/8)           | 80% (4/5)   | 100% (3/3) | 86% (6/7)  | 75% (3/4)     | 100% (3/3)       |
| qwen3.5:4b        | plain        | 38% (3/8)  | 56% (9/16)   | 60% (3/5)  | 89% (8/9)  | 50% (4/8)           | 40% (2/5)   | 33% (1/3)  | 57% (4/7)  | 75% (3/4)     | 100% (3/3)       |
| gemma4:e2b-it-qat | few_shot_six | 62% (5/8)  | 50% (8/16)   | 20% (1/5)  | 89% (8/9)  | 75% (6/8)           | 20% (1/5)   | 0% (0/3)   | 29% (2/7)  | 0% (0/4)      | 100% (3/3)       |
| gemma3n:e2b       | few_shot     | 50% (4/8)  | 44% (7/16)   | 60% (3/5)  | 78% (7/9)  | 50% (4/8)           | 60% (3/5)   | 0% (0/3)   | 14% (1/7)  | 50% (2/4)     | 100% (3/3)       |
| gemma4:e2b-it-qat | plain        | 38% (3/8)  | 75% (12/16)  | 60% (3/5)  | 78% (7/9)  | 88% (7/8)           | 20% (1/5)   | 100% (3/3) | 71% (5/7)  | 75% (3/4)     | 100% (3/3)       |
| gemma4:e2b        | few_shot_six | 25% (2/8)  | 56% (9/16)   | 60% (3/5)  | 89% (8/9)  | 62% (5/8)           | 20% (1/5)   | 100% (3/3) | 57% (4/7)  | 25% (1/4)     | 100% (3/3)       |
| gemma4:e2b        | few_shot     | 38% (3/8)  | 56% (9/16)   | 60% (3/5)  | 89% (8/9)  | 62% (5/8)           | 0% (0/5)    | 100% (3/3) | 14% (1/7)  | 25% (1/4)     | 100% (3/3)       |
| gemma3n:e2b       | plain        | 75% (6/8)  | 38% (6/16)   | 80% (4/5)  | 89% (8/9)  | 62% (5/8)           | 80% (4/5)   | 33% (1/3)  | 57% (4/7)  | 100% (4/4)    | 100% (3/3)       |
| gemma3n:e2b       | few_shot_six | 38% (3/8)  | 56% (9/16)   | 40% (2/5)  | 44% (4/9)  | 38% (3/8)           | 20% (1/5)   | 33% (1/3)  | 14% (1/7)  | 25% (1/4)     | 33% (1/3)        |
| gemma4:e2b        | plain        | 38% (3/8)  | 75% (12/16)  | 60% (3/5)  | 78% (7/9)  | 50% (4/8)           | 20% (1/5)   | 100% (3/3) | 71% (5/7)  | 25% (1/4)     | 100% (3/3)       |
| granite4.2:3b     | few_shot_six | 0% (0/8)   | 12% (2/16)   | 0% (0/5)   | 11% (1/9)  | 12% (1/8)           | 20% (1/5)   | 0% (0/3)   | 0% (0/7)   | 0% (0/4)      | 67% (2/3)        |
| granite4.2:3b     | plain        | 0% (0/8)   | 0% (0/16)    | 0% (0/5)   | 11% (1/9)  | 25% (2/8)           | 0% (0/5)    | 33% (1/3)  | 0% (0/7)   | 0% (0/4)      | 33% (1/3)        |
| granite4.2:3b     | few_shot     | 0% (0/8)   | 0% (0/16)    | 0% (0/5)   | 11% (1/9)  | 12% (1/8)           | 0% (0/5)    | 0% (0/3)   | 0% (0/7)   | 0% (0/4)      | 0% (0/3)         |

## Bar

- False positive rate at most 10%
- Quotes not in the answer at most 2%
- Precision at least 70%
- Recall at least 60%
- Correction agreement at least 60%
- p95 latency at most 15 s

| Model             | Prompt       | Control | Quotes | Precision | Recall | Correction | Latency | Meets all |
| ----------------- | ------------ | ------- | ------ | --------- | ------ | ---------- | ------- | --------- |
| gemma4:12b-it-qat | few_shot_six | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| gemma4:12b-it-qat | few_shot     | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| gemma4:12b        | few_shot     | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| gemma4:12b        | few_shot_six | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| gemma4:12b        | plain        | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| gemma4:12b-it-qat | plain        | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| ministral-3:14b   | few_shot_six | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| ministral-3:14b   | few_shot     | no      | yes    | yes       | yes    | yes        | yes     | no        |
| gemma4:e4b        | few_shot_six | no      | yes    | yes       | yes    | yes        | yes     | no        |
| gemma4:e4b        | few_shot     | no      | yes    | yes       | yes    | yes        | yes     | no        |
| gemma4:e4b-it-qat | plain        | no      | yes    | yes       | yes    | yes        | yes     | no        |
| gemma4:e4b        | plain        | no      | yes    | no        | yes    | yes        | yes     | no        |
| qwen3.5:4b        | few_shot_six | no      | yes    | no        | yes    | no         | yes     | no        |
| gemma4:e4b-it-qat | few_shot     | yes     | yes    | yes       | no     | yes        | yes     | no        |
| qwen3.5:4b        | few_shot     | no      | yes    | no        | yes    | no         | yes     | no        |
| gemma4:e4b-it-qat | few_shot_six | yes     | yes    | yes       | no     | yes        | yes     | no        |
| gemma3n:e4b       | few_shot     | no      | yes    | yes       | no     | yes        | yes     | no        |
| ministral-3:14b   | plain        | no      | yes    | no        | yes    | yes        | yes     | no        |
| gemma3n:e4b       | few_shot_six | no      | yes    | yes       | yes    | no         | yes     | no        |
| gemma4:e2b-it-qat | few_shot     | yes     | yes    | yes       | no     | yes        | yes     | no        |
| gemma3n:e4b       | plain        | no      | yes    | no        | yes    | yes        | yes     | no        |
| qwen3.5:4b        | plain        | yes     | yes    | no        | no     | no         | yes     | no        |
| gemma4:e2b-it-qat | few_shot_six | no      | yes    | yes       | no     | yes        | yes     | no        |
| gemma3n:e2b       | few_shot     | yes     | yes    | no        | no     | no         | yes     | no        |
| gemma4:e2b-it-qat | plain        | no      | yes    | no        | yes    | yes        | yes     | no        |
| gemma4:e2b        | few_shot_six | no      | yes    | no        | no     | yes        | yes     | no        |
| gemma4:e2b        | few_shot     | no      | yes    | no        | no     | yes        | yes     | no        |
| gemma3n:e2b       | plain        | no      | yes    | no        | yes    | no         | yes     | no        |
| gemma3n:e2b       | few_shot_six | yes     | yes    | no        | no     | no         | yes     | no        |
| gemma4:e2b        | plain        | no      | yes    | no        | yes    | yes        | yes     | no        |
| granite4.2:3b     | few_shot_six | yes     | yes    | yes       | no     | no         | yes     | no        |
| granite4.2:3b     | plain        | yes     | no     | no        | no     | no         | yes     | no        |
| granite4.2:3b     | few_shot     | yes     | yes    | yes       | no     | no         | yes     | no        |
