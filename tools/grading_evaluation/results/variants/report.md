# Grading evaluation

## Error detection

| Model          | Prompt                  | Precision | Recall | F1  | False positive rate | Quotes not in answer | Category | Correction | Imprecise quote | Duplicates |
| -------------- | ----------------------- | --------- | ------ | --- | ------------------- | -------------------- | -------- | ---------- | --------------- | ---------- |
| gemma3:12b     | few_shot_six            | 95%       | 78%    | 85% | 2%                  | 0%                   | 79%      | 74%        | 0%              | 0%         |
| gemma3:12b     | few_shot                | 93%       | 78%    | 85% | 4%                  | 0%                   | 75%      | 64%        | 0%              | 2%         |
| gemma3:12b     | few_shot_reversed       | 90%       | 79%    | 84% | 7%                  | 0%                   | 81%      | 65%        | 0%              | 0%         |
| gemma3:12b     | one_shot_mistake        | 91%       | 78%    | 84% | 2%                  | 2%                   | 87%      | 68%        | 0%              | 2%         |
| gemma3:12b     | few_shot_in_system      | 86%       | 81%    | 83% | 13%                 | 0%                   | 76%      | 69%        | 0%              | 2%         |
| gemma3:12b     | few_shot_mostly_correct | 81%       | 82%    | 82% | 18%                 | 0%                   | 79%      | 73%        | 0%              | 1%         |
| qwen3:8b       | few_shot_six            | 78%       | 85%    | 82% | 11%                 | 3%                   | 78%      | 66%        | 0%              | 0%         |
| qwen3.5:latest | few_shot_six            | 72%       | 88%    | 79% | 33%                 | 0%                   | 88%      | 60%        | 0%              | 5%         |
| qwen3.5:latest | few_shot_mostly_correct | 72%       | 87%    | 79% | 27%                 | 2%                   | 92%      | 58%        | 0%              | 1%         |
| qwen3.5:latest | one_shot_correct        | 73%       | 81%    | 77% | 13%                 | 1%                   | 84%      | 55%        | 0%              | 4%         |
| qwen3.5:latest | few_shot_reversed       | 68%       | 85%    | 76% | 31%                 | 0%                   | 88%      | 55%        | 0%              | 3%         |
| qwen3.5:latest | few_shot                | 70%       | 82%    | 76% | 27%                 | 0%                   | 93%      | 61%        | 0%              | 5%         |
| qwen3.5:latest | one_shot_mistake        | 70%       | 82%    | 76% | 18%                 | 1%                   | 89%      | 61%        | 0%              | 5%         |
| gemma3:12b     | one_shot_correct        | 77%       | 74%    | 75% | 2%                  | 0%                   | 76%      | 74%        | 0%              | 3%         |
| qwen3.5:latest | few_shot_in_system      | 73%       | 78%    | 75% | 18%                 | 1%                   | 91%      | 58%        | 0%              | 4%         |
| gemma3:12b     | plain                   | 69%       | 81%    | 74% | 29%                 | 0%                   | 80%      | 65%        | 0%              | 1%         |
| qwen3:8b       | few_shot_mostly_correct | 66%       | 84%    | 74% | 42%                 | 1%                   | 81%      | 61%        | 2%              | 1%         |
| qwen3:8b       | one_shot_mistake        | 65%       | 84%    | 73% | 38%                 | 0%                   | 82%      | 60%        | 2%              | 1%         |
| qwen3:8b       | plain                   | 68%       | 79%    | 73% | 11%                 | 0%                   | 81%      | 59%        | 2%              | 1%         |
| qwen3:8b       | few_shot_reversed       | 65%       | 81%    | 72% | 31%                 | 1%                   | 85%      | 60%        | 2%              | 1%         |
| qwen3:8b       | few_shot                | 67%       | 78%    | 72% | 20%                 | 2%                   | 87%      | 60%        | 0%              | 1%         |
| qwen3.5:latest | plain                   | 70%       | 74%    | 72% | 7%                  | 1%                   | 92%      | 54%        | 0%              | 5%         |
| qwen3:8b       | few_shot_in_system      | 57%       | 81%    | 67% | 49%                 | 1%                   | 80%      | 65%        | 0%              | 0%         |
| qwen3:4b       | few_shot_six            | 69%       | 65%    | 67% | 16%                 | 0%                   | 75%      | 59%        | 0%              | 0%         |
| qwen3:4b       | few_shot                | 70%       | 63%    | 67% | 9%                  | 0%                   | 81%      | 56%        | 0%              | 0%         |
| qwen3:4b       | one_shot_correct        | 65%       | 66%    | 66% | 18%                 | 1%                   | 82%      | 51%        | 0%              | 3%         |
| qwen3:8b       | one_shot_correct        | 52%       | 85%    | 64% | 71%                 | 0%                   | 79%      | 62%        | 2%              | 2%         |
| qwen3:4b       | few_shot_in_system      | 57%       | 69%    | 62% | 38%                 | 1%                   | 81%      | 51%        | 0%              | 0%         |
| qwen3:4b       | few_shot_reversed       | 57%       | 63%    | 60% | 31%                 | 0%                   | 81%      | 63%        | 0%              | 0%         |
| gemma3:4b      | few_shot_six            | 64%       | 56%    | 60% | 20%                 | 2%                   | 55%      | 50%        | 0%              | 0%         |
| qwen3:4b       | few_shot_mostly_correct | 62%       | 57%    | 60% | 24%                 | 0%                   | 77%      | 56%        | 0%              | 0%         |
| gemma3:4b      | few_shot                | 57%       | 53%    | 55% | 24%                 | 2%                   | 50%      | 44%        | 3%              | 0%         |
| qwen3:4b       | one_shot_mistake        | 49%       | 62%    | 55% | 38%                 | 1%                   | 67%      | 55%        | 0%              | 0%         |
| gemma3:4b      | few_shot_in_system      | 58%       | 51%    | 55% | 22%                 | 2%                   | 60%      | 46%        | 0%              | 0%         |
| gemma3:4b      | few_shot_reversed       | 45%       | 65%    | 53% | 73%                 | 4%                   | 43%      | 43%        | 0%              | 0%         |
| gemma3:4b      | one_shot_mistake        | 45%       | 63%    | 53% | 71%                 | 2%                   | 47%      | 49%        | 0%              | 0%         |
| gemma3:4b      | one_shot_correct        | 62%       | 44%    | 52% | 7%                  | 2%                   | 40%      | 47%        | 0%              | 0%         |
| gemma3:4b      | few_shot_mostly_correct | 59%       | 44%    | 50% | 18%                 | 2%                   | 40%      | 53%        | 0%              | 0%         |
| qwen3:4b       | plain                   | 39%       | 66%    | 49% | 47%                 | 3%                   | 78%      | 51%        | 0%              | 2%         |
| gemma3:4b      | plain                   | 35%       | 63%    | 45% | 100%                | 0%                   | 44%      | 42%        | 0%              | 1%         |

- Precision: findings that match a real mistake ÷ (those findings + findings that match none); a repeat finding of an already matched mistake counts in neither
- Recall: real mistakes matched by a finding ÷ all real mistakes
- False positive rate: correct answers with at least one finding ÷ all correct answers

## Counts behind the rates

| Model          | Prompt                  | Answers graded | Reference mistakes | Found correctly | False alarms | Quotes not in answer | Correct answers | Correct answers flagged |
| -------------- | ----------------------- | -------------- | ------------------ | --------------- | ------------ | -------------------- | --------------- | ----------------------- |
| gemma3:12b     | few_shot_six            | 114            | 68                 | 53              | 3            | 0                    | 45              | 1                       |
| gemma3:12b     | few_shot                | 114            | 68                 | 53              | 4            | 0                    | 45              | 2                       |
| gemma3:12b     | few_shot_reversed       | 114            | 68                 | 54              | 6            | 0                    | 45              | 3                       |
| gemma3:12b     | one_shot_mistake        | 114            | 68                 | 53              | 4            | 1                    | 45              | 1                       |
| gemma3:12b     | few_shot_in_system      | 114            | 68                 | 55              | 9            | 0                    | 45              | 6                       |
| gemma3:12b     | few_shot_mostly_correct | 114            | 68                 | 56              | 13           | 0                    | 45              | 8                       |
| qwen3:8b       | few_shot_six            | 114            | 68                 | 58              | 14           | 2                    | 45              | 5                       |
| qwen3.5:latest | few_shot_six            | 114            | 68                 | 60              | 23           | 0                    | 45              | 15                      |
| qwen3.5:latest | few_shot_mostly_correct | 114            | 68                 | 59              | 21           | 2                    | 45              | 12                      |
| qwen3.5:latest | one_shot_correct        | 114            | 68                 | 55              | 19           | 1                    | 45              | 6                       |
| qwen3.5:latest | few_shot_reversed       | 114            | 68                 | 58              | 27           | 0                    | 45              | 14                      |
| qwen3.5:latest | few_shot                | 114            | 68                 | 56              | 24           | 0                    | 45              | 12                      |
| qwen3.5:latest | one_shot_mistake        | 114            | 68                 | 56              | 23           | 1                    | 45              | 8                       |
| gemma3:12b     | one_shot_correct        | 114            | 68                 | 50              | 15           | 0                    | 45              | 1                       |
| qwen3.5:latest | few_shot_in_system      | 114            | 68                 | 53              | 19           | 1                    | 45              | 8                       |
| gemma3:12b     | plain                   | 114            | 68                 | 55              | 25           | 0                    | 45              | 13                      |
| qwen3:8b       | few_shot_mostly_correct | 114            | 68                 | 57              | 29           | 1                    | 45              | 19                      |
| qwen3:8b       | one_shot_mistake        | 114            | 68                 | 57              | 31           | 0                    | 45              | 17                      |
| qwen3:8b       | plain                   | 114            | 68                 | 54              | 26           | 0                    | 45              | 5                       |
| qwen3:8b       | few_shot_reversed       | 114            | 68                 | 55              | 28           | 1                    | 45              | 14                      |
| qwen3:8b       | few_shot                | 114            | 68                 | 53              | 24           | 2                    | 45              | 9                       |
| qwen3.5:latest | plain                   | 114            | 68                 | 50              | 20           | 1                    | 45              | 3                       |
| qwen3:8b       | few_shot_in_system      | 114            | 68                 | 55              | 40           | 1                    | 45              | 22                      |
| qwen3:4b       | few_shot_six            | 114            | 68                 | 44              | 20           | 0                    | 45              | 7                       |
| qwen3:4b       | few_shot                | 114            | 68                 | 43              | 18           | 0                    | 45              | 4                       |
| qwen3:4b       | one_shot_correct        | 114            | 68                 | 45              | 23           | 1                    | 45              | 8                       |
| qwen3:8b       | one_shot_correct        | 114            | 68                 | 58              | 54           | 0                    | 45              | 32                      |
| qwen3:4b       | few_shot_in_system      | 114            | 68                 | 47              | 35           | 1                    | 45              | 17                      |
| qwen3:4b       | few_shot_reversed       | 114            | 68                 | 43              | 32           | 0                    | 45              | 14                      |
| gemma3:4b      | few_shot_six            | 114            | 68                 | 38              | 20           | 1                    | 45              | 9                       |
| qwen3:4b       | few_shot_mostly_correct | 114            | 68                 | 39              | 24           | 0                    | 45              | 11                      |
| gemma3:4b      | few_shot                | 114            | 68                 | 36              | 26           | 1                    | 45              | 11                      |
| qwen3:4b       | one_shot_mistake        | 114            | 68                 | 42              | 42           | 1                    | 45              | 17                      |
| gemma3:4b      | few_shot_in_system      | 114            | 68                 | 35              | 24           | 1                    | 45              | 10                      |
| gemma3:4b      | few_shot_reversed       | 114            | 68                 | 44              | 49           | 4                    | 45              | 33                      |
| gemma3:4b      | one_shot_mistake        | 114            | 68                 | 43              | 50           | 2                    | 45              | 32                      |
| gemma3:4b      | one_shot_correct        | 114            | 68                 | 30              | 17           | 1                    | 45              | 3                       |
| gemma3:4b      | few_shot_mostly_correct | 114            | 68                 | 30              | 20           | 1                    | 45              | 8                       |
| qwen3:4b       | plain                   | 114            | 68                 | 45              | 68           | 3                    | 45              | 21                      |
| gemma3:4b      | plain                   | 114            | 68                 | 43              | 79           | 0                    | 45              | 45                      |

## Behavior and cost

| Model          | Prompt                  | Parameters | Download | Valid response | On-topic flag | Language flag | Unstable cases | p50 s | p95 s | Tokens/s |
| -------------- | ----------------------- | ---------- | -------- | -------------- | ------------- | ------------- | -------------- | ----- | ----- | -------- |
| gemma3:12b     | few_shot_six            | 11.8B      | 8.2 GB   | 100%           | 98%           | 100%          | n/a            | 2.7   | 4.0   | 41       |
| gemma3:12b     | few_shot                | 11.8B      | 8.2 GB   | 100%           | 97%           | 100%          | n/a            | 2.9   | 4.9   | 36       |
| gemma3:12b     | few_shot_reversed       | 11.8B      | 8.2 GB   | 100%           | 97%           | 100%          | n/a            | 2.7   | 5.4   | 32       |
| gemma3:12b     | one_shot_mistake        | 11.8B      | 8.2 GB   | 100%           | 100%          | 99%           | n/a            | 2.1   | 3.1   | 48       |
| gemma3:12b     | few_shot_in_system      | 11.8B      | 8.2 GB   | 100%           | 99%           | 100%          | n/a            | 2.9   | 4.0   | 41       |
| gemma3:12b     | few_shot_mostly_correct | 11.8B      | 8.2 GB   | 100%           | 99%           | 100%          | n/a            | 3.0   | 4.1   | 42       |
| qwen3:8b       | few_shot_six            | 8.2B       | 5.2 GB   | 100%           | 95%           | 100%          | n/a            | 1.2   | 2.4   | 50       |
| qwen3.5:latest | few_shot_six            | 9.7B       | 6.6 GB   | 100%           | 100%          | 99%           | n/a            | 2.0   | 6.2   | 48       |
| qwen3.5:latest | few_shot_mostly_correct | 9.7B       | 6.6 GB   | 99%            | 97%           | 99%           | n/a            | 1.9   | 5.8   | 53       |
| qwen3.5:latest | one_shot_correct        | 9.7B       | 6.6 GB   | 100%           | 98%           | 99%           | n/a            | 2.3   | 6.8   | 44       |
| qwen3.5:latest | few_shot_reversed       | 9.7B       | 6.6 GB   | 99%            | 97%           | 99%           | n/a            | 2.0   | 7.7   | 49       |
| qwen3.5:latest | few_shot                | 9.7B       | 6.6 GB   | 100%           | 96%           | 100%          | n/a            | 2.1   | 4.3   | 50       |
| qwen3.5:latest | one_shot_mistake        | 9.7B       | 6.6 GB   | 98%            | 96%           | 98%           | n/a            | 2.4   | 6.2   | 38       |
| gemma3:12b     | one_shot_correct        | 11.8B      | 8.2 GB   | 100%           | 100%          | 99%           | n/a            | 2.2   | 4.1   | 45       |
| qwen3.5:latest | few_shot_in_system      | 9.7B       | 6.6 GB   | 100%           | 96%           | 97%           | n/a            | 1.8   | 5.2   | 54       |
| gemma3:12b     | plain                   | 11.8B      | 8.2 GB   | 100%           | 100%          | 100%          | n/a            | 3.2   | 4.6   | 43       |
| qwen3:8b       | few_shot_mostly_correct | 8.2B       | 5.2 GB   | 100%           | 99%           | 99%           | n/a            | 1.4   | 2.3   | 50       |
| qwen3:8b       | one_shot_mistake        | 8.2B       | 5.2 GB   | 100%           | 96%           | 99%           | n/a            | 1.4   | 2.5   | 47       |
| qwen3:8b       | plain                   | 8.2B       | 5.2 GB   | 100%           | 100%          | 100%          | n/a            | 1.8   | 4.1   | 47       |
| qwen3:8b       | few_shot_reversed       | 8.2B       | 5.2 GB   | 100%           | 98%           | 99%           | n/a            | 1.3   | 2.1   | 51       |
| qwen3:8b       | few_shot                | 8.2B       | 5.2 GB   | 100%           | 95%           | 100%          | n/a            | 1.2   | 2.1   | 52       |
| qwen3.5:latest | plain                   | 9.7B       | 6.6 GB   | 100%           | 96%           | 100%          | n/a            | 2.9   | 6.0   | 45       |
| qwen3:8b       | few_shot_in_system      | 8.2B       | 5.2 GB   | 100%           | 99%           | 100%          | n/a            | 1.3   | 2.5   | 49       |
| qwen3:4b       | few_shot_six            | 4.0B       | 2.5 GB   | 99%            | 98%           | 99%           | n/a            | 1.1   | 1.9   | 73       |
| qwen3:4b       | few_shot                | 4.0B       | 2.5 GB   | 100%           | 99%           | 100%          | n/a            | 0.6   | 1.6   | 76       |
| qwen3:4b       | one_shot_correct        | 4.0B       | 2.5 GB   | 99%            | 96%           | 99%           | n/a            | 1.0   | 2.6   | 77       |
| qwen3:8b       | one_shot_correct        | 8.2B       | 5.2 GB   | 100%           | 99%           | 100%          | n/a            | 1.7   | 3.4   | 47       |
| qwen3:4b       | few_shot_in_system      | 4.0B       | 2.5 GB   | 100%           | 100%          | 100%          | n/a            | 1.1   | 1.7   | 75       |
| qwen3:4b       | few_shot_reversed       | 4.0B       | 2.5 GB   | 100%           | 99%           | 100%          | n/a            | 1.2   | 2.1   | 74       |
| gemma3:4b      | few_shot_six            | 3.9B       | 3.4 GB   | 99%            | 96%           | 98%           | n/a            | 1.2   | 1.4   | 107      |
| qwen3:4b       | few_shot_mostly_correct | 4.0B       | 2.5 GB   | 100%           | 96%           | 100%          | n/a            | 1.0   | 2.0   | 75       |
| gemma3:4b      | few_shot                | 3.9B       | 3.4 GB   | 100%           | 72%           | 98%           | n/a            | 1.1   | 1.3   | 107      |
| qwen3:4b       | one_shot_mistake        | 4.0B       | 2.5 GB   | 100%           | 99%           | 100%          | n/a            | 1.1   | 2.6   | 78       |
| gemma3:4b      | few_shot_in_system      | 3.9B       | 3.4 GB   | 100%           | 92%           | 98%           | n/a            | 1.1   | 1.3   | 108      |
| gemma3:4b      | few_shot_reversed       | 3.9B       | 3.4 GB   | 100%           | 87%           | 98%           | n/a            | 1.2   | 1.3   | 107      |
| gemma3:4b      | one_shot_mistake        | 3.9B       | 3.4 GB   | 100%           | 100%          | 100%          | n/a            | 1.2   | 1.4   | 88       |
| gemma3:4b      | one_shot_correct        | 3.9B       | 3.4 GB   | 100%           | 100%          | 98%           | n/a            | 0.7   | 1.5   | 93       |
| gemma3:4b      | few_shot_mostly_correct | 3.9B       | 3.4 GB   | 100%           | 100%          | 99%           | n/a            | 0.9   | 1.2   | 109      |
| qwen3:4b       | plain                   | 4.0B       | 2.5 GB   | 100%           | 96%           | 100%          | n/a            | 1.2   | 3.0   | 80       |
| gemma3:4b      | plain                   | 3.9B       | 3.4 GB   | 100%           | 96%           | 99%           | n/a            | 1.7   | 2.4   | 73       |

## Recall by error category

| Model          | Prompt                  | spelling   | verb_form   | ser_estar  | agreement  | article_preposition | word_choice | word_order | pronoun    | not_brazilian | content_mismatch |
| -------------- | ----------------------- | ---------- | ----------- | ---------- | ---------- | ------------------- | ----------- | ---------- | ---------- | ------------- | ---------------- |
| gemma3:12b     | few_shot_six            | 75% (6/8)  | 81% (13/16) | 80% (4/5)  | 89% (8/9)  | 75% (6/8)           | 40% (2/5)   | 100% (3/3) | 57% (4/7)  | 100% (4/4)    | 100% (3/3)       |
| gemma3:12b     | few_shot                | 75% (6/8)  | 81% (13/16) | 80% (4/5)  | 78% (7/9)  | 88% (7/8)           | 40% (2/5)   | 100% (3/3) | 57% (4/7)  | 100% (4/4)    | 100% (3/3)       |
| gemma3:12b     | few_shot_reversed       | 75% (6/8)  | 81% (13/16) | 80% (4/5)  | 89% (8/9)  | 88% (7/8)           | 40% (2/5)   | 100% (3/3) | 57% (4/7)  | 100% (4/4)    | 100% (3/3)       |
| gemma3:12b     | one_shot_mistake        | 62% (5/8)  | 81% (13/16) | 80% (4/5)  | 89% (8/9)  | 88% (7/8)           | 40% (2/5)   | 100% (3/3) | 57% (4/7)  | 100% (4/4)    | 100% (3/3)       |
| gemma3:12b     | few_shot_in_system      | 75% (6/8)  | 81% (13/16) | 80% (4/5)  | 89% (8/9)  | 88% (7/8)           | 40% (2/5)   | 100% (3/3) | 71% (5/7)  | 100% (4/4)    | 100% (3/3)       |
| gemma3:12b     | few_shot_mostly_correct | 75% (6/8)  | 81% (13/16) | 100% (5/5) | 100% (9/9) | 88% (7/8)           | 40% (2/5)   | 100% (3/3) | 57% (4/7)  | 100% (4/4)    | 100% (3/3)       |
| qwen3:8b       | few_shot_six            | 88% (7/8)  | 81% (13/16) | 100% (5/5) | 100% (9/9) | 100% (8/8)          | 60% (3/5)   | 100% (3/3) | 57% (4/7)  | 75% (3/4)     | 100% (3/3)       |
| qwen3.5:latest | few_shot_six            | 88% (7/8)  | 94% (15/16) | 80% (4/5)  | 100% (9/9) | 88% (7/8)           | 60% (3/5)   | 100% (3/3) | 71% (5/7)  | 100% (4/4)    | 100% (3/3)       |
| qwen3.5:latest | few_shot_mostly_correct | 88% (7/8)  | 88% (14/16) | 100% (5/5) | 100% (9/9) | 88% (7/8)           | 60% (3/5)   | 100% (3/3) | 71% (5/7)  | 75% (3/4)     | 100% (3/3)       |
| qwen3.5:latest | one_shot_correct        | 88% (7/8)  | 75% (12/16) | 80% (4/5)  | 89% (8/9)  | 100% (8/8)          | 60% (3/5)   | 67% (2/3)  | 71% (5/7)  | 75% (3/4)     | 100% (3/3)       |
| qwen3.5:latest | few_shot_reversed       | 88% (7/8)  | 94% (15/16) | 100% (5/5) | 100% (9/9) | 88% (7/8)           | 60% (3/5)   | 67% (2/3)  | 57% (4/7)  | 75% (3/4)     | 100% (3/3)       |
| qwen3.5:latest | few_shot                | 75% (6/8)  | 81% (13/16) | 80% (4/5)  | 100% (9/9) | 100% (8/8)          | 60% (3/5)   | 67% (2/3)  | 57% (4/7)  | 100% (4/4)    | 100% (3/3)       |
| qwen3.5:latest | one_shot_mistake        | 75% (6/8)  | 75% (12/16) | 100% (5/5) | 100% (9/9) | 75% (6/8)           | 60% (3/5)   | 100% (3/3) | 71% (5/7)  | 100% (4/4)    | 100% (3/3)       |
| gemma3:12b     | one_shot_correct        | 62% (5/8)  | 81% (13/16) | 80% (4/5)  | 78% (7/9)  | 75% (6/8)           | 40% (2/5)   | 67% (2/3)  | 57% (4/7)  | 100% (4/4)    | 100% (3/3)       |
| qwen3.5:latest | few_shot_in_system      | 75% (6/8)  | 69% (11/16) | 100% (5/5) | 89% (8/9)  | 75% (6/8)           | 60% (3/5)   | 100% (3/3) | 71% (5/7)  | 75% (3/4)     | 100% (3/3)       |
| gemma3:12b     | plain                   | 75% (6/8)  | 88% (14/16) | 80% (4/5)  | 89% (8/9)  | 88% (7/8)           | 40% (2/5)   | 67% (2/3)  | 71% (5/7)  | 100% (4/4)    | 100% (3/3)       |
| qwen3:8b       | few_shot_mostly_correct | 75% (6/8)  | 75% (12/16) | 100% (5/5) | 89% (8/9)  | 88% (7/8)           | 60% (3/5)   | 100% (3/3) | 100% (7/7) | 75% (3/4)     | 100% (3/3)       |
| qwen3:8b       | one_shot_mistake        | 100% (8/8) | 81% (13/16) | 100% (5/5) | 89% (8/9)  | 75% (6/8)           | 60% (3/5)   | 100% (3/3) | 86% (6/7)  | 50% (2/4)     | 100% (3/3)       |
| qwen3:8b       | plain                   | 100% (8/8) | 75% (12/16) | 100% (5/5) | 89% (8/9)  | 100% (8/8)          | 60% (3/5)   | 33% (1/3)  | 71% (5/7)  | 25% (1/4)     | 100% (3/3)       |
| qwen3:8b       | few_shot_reversed       | 88% (7/8)  | 81% (13/16) | 100% (5/5) | 100% (9/9) | 88% (7/8)           | 60% (3/5)   | 33% (1/3)  | 71% (5/7)  | 50% (2/4)     | 100% (3/3)       |
| qwen3:8b       | few_shot                | 88% (7/8)  | 81% (13/16) | 100% (5/5) | 89% (8/9)  | 100% (8/8)          | 60% (3/5)   | 33% (1/3)  | 57% (4/7)  | 25% (1/4)     | 100% (3/3)       |
| qwen3.5:latest | plain                   | 75% (6/8)  | 75% (12/16) | 80% (4/5)  | 89% (8/9)  | 75% (6/8)           | 60% (3/5)   | 67% (2/3)  | 57% (4/7)  | 50% (2/4)     | 100% (3/3)       |
| qwen3:8b       | few_shot_in_system      | 88% (7/8)  | 88% (14/16) | 100% (5/5) | 78% (7/9)  | 75% (6/8)           | 60% (3/5)   | 33% (1/3)  | 86% (6/7)  | 75% (3/4)     | 100% (3/3)       |
| qwen3:4b       | few_shot_six            | 88% (7/8)  | 75% (12/16) | 60% (3/5)  | 89% (8/9)  | 88% (7/8)           | 40% (2/5)   | 33% (1/3)  | 43% (3/7)  | 0% (0/4)      | 33% (1/3)        |
| qwen3:4b       | few_shot                | 88% (7/8)  | 62% (10/16) | 60% (3/5)  | 89% (8/9)  | 62% (5/8)           | 40% (2/5)   | 67% (2/3)  | 43% (3/7)  | 0% (0/4)      | 100% (3/3)       |
| qwen3:4b       | one_shot_correct        | 88% (7/8)  | 69% (11/16) | 80% (4/5)  | 89% (8/9)  | 62% (5/8)           | 40% (2/5)   | 67% (2/3)  | 43% (3/7)  | 0% (0/4)      | 100% (3/3)       |
| qwen3:8b       | one_shot_correct        | 88% (7/8)  | 81% (13/16) | 100% (5/5) | 89% (8/9)  | 88% (7/8)           | 80% (4/5)   | 67% (2/3)  | 86% (6/7)  | 75% (3/4)     | 100% (3/3)       |
| qwen3:4b       | few_shot_in_system      | 88% (7/8)  | 75% (12/16) | 60% (3/5)  | 89% (8/9)  | 88% (7/8)           | 40% (2/5)   | 67% (2/3)  | 43% (3/7)  | 25% (1/4)     | 67% (2/3)        |
| qwen3:4b       | few_shot_reversed       | 88% (7/8)  | 69% (11/16) | 60% (3/5)  | 78% (7/9)  | 75% (6/8)           | 40% (2/5)   | 33% (1/3)  | 57% (4/7)  | 0% (0/4)      | 67% (2/3)        |
| gemma3:4b      | few_shot_six            | 50% (4/8)  | 50% (8/16)  | 60% (3/5)  | 78% (7/9)  | 50% (4/8)           | 40% (2/5)   | 33% (1/3)  | 57% (4/7)  | 50% (2/4)     | 100% (3/3)       |
| qwen3:4b       | few_shot_mostly_correct | 88% (7/8)  | 56% (9/16)  | 60% (3/5)  | 67% (6/9)  | 62% (5/8)           | 40% (2/5)   | 67% (2/3)  | 43% (3/7)  | 0% (0/4)      | 67% (2/3)        |
| gemma3:4b      | few_shot                | 50% (4/8)  | 50% (8/16)  | 40% (2/5)  | 33% (3/9)  | 62% (5/8)           | 40% (2/5)   | 67% (2/3)  | 71% (5/7)  | 50% (2/4)     | 100% (3/3)       |
| qwen3:4b       | one_shot_mistake        | 75% (6/8)  | 56% (9/16)  | 40% (2/5)  | 67% (6/9)  | 75% (6/8)           | 60% (3/5)   | 67% (2/3)  | 57% (4/7)  | 50% (2/4)     | 67% (2/3)        |
| gemma3:4b      | few_shot_in_system      | 38% (3/8)  | 44% (7/16)  | 60% (3/5)  | 78% (7/9)  | 50% (4/8)           | 40% (2/5)   | 33% (1/3)  | 43% (3/7)  | 50% (2/4)     | 100% (3/3)       |
| gemma3:4b      | few_shot_reversed       | 50% (4/8)  | 69% (11/16) | 80% (4/5)  | 89% (8/9)  | 62% (5/8)           | 40% (2/5)   | 67% (2/3)  | 43% (3/7)  | 50% (2/4)     | 100% (3/3)       |
| gemma3:4b      | one_shot_mistake        | 50% (4/8)  | 62% (10/16) | 80% (4/5)  | 67% (6/9)  | 62% (5/8)           | 60% (3/5)   | 33% (1/3)  | 57% (4/7)  | 75% (3/4)     | 100% (3/3)       |
| gemma3:4b      | one_shot_correct        | 38% (3/8)  | 38% (6/16)  | 40% (2/5)  | 56% (5/9)  | 50% (4/8)           | 40% (2/5)   | 33% (1/3)  | 29% (2/7)  | 50% (2/4)     | 100% (3/3)       |
| gemma3:4b      | few_shot_mostly_correct | 25% (2/8)  | 31% (5/16)  | 60% (3/5)  | 67% (6/9)  | 38% (3/8)           | 40% (2/5)   | 0% (0/3)   | 57% (4/7)  | 50% (2/4)     | 100% (3/3)       |
| qwen3:4b       | plain                   | 75% (6/8)  | 62% (10/16) | 60% (3/5)  | 78% (7/9)  | 88% (7/8)           | 40% (2/5)   | 67% (2/3)  | 43% (3/7)  | 50% (2/4)     | 100% (3/3)       |
| gemma3:4b      | plain                   | 50% (4/8)  | 50% (8/16)  | 100% (5/5) | 89% (8/9)  | 62% (5/8)           | 40% (2/5)   | 33% (1/3)  | 57% (4/7)  | 75% (3/4)     | 100% (3/3)       |

## Bar

- False positive rate at most 10%
- Quotes not in the answer at most 2%
- Precision at least 70%
- Recall at least 60%
- Correction agreement at least 60%
- p95 latency at most 15 s

| Model          | Prompt                  | Control | Quotes | Precision | Recall | Correction | Latency | Meets all |
| -------------- | ----------------------- | ------- | ------ | --------- | ------ | ---------- | ------- | --------- |
| gemma3:12b     | few_shot_six            | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| gemma3:12b     | few_shot                | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| gemma3:12b     | few_shot_reversed       | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| gemma3:12b     | one_shot_mistake        | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| gemma3:12b     | few_shot_in_system      | no      | yes    | yes       | yes    | yes        | yes     | no        |
| gemma3:12b     | few_shot_mostly_correct | no      | yes    | yes       | yes    | yes        | yes     | no        |
| qwen3:8b       | few_shot_six            | no      | no     | yes       | yes    | yes        | yes     | no        |
| qwen3.5:latest | few_shot_six            | no      | yes    | yes       | yes    | yes        | yes     | no        |
| qwen3.5:latest | few_shot_mostly_correct | no      | no     | yes       | yes    | no         | yes     | no        |
| qwen3.5:latest | one_shot_correct        | no      | yes    | yes       | yes    | no         | yes     | no        |
| qwen3.5:latest | few_shot_reversed       | no      | yes    | no        | yes    | no         | yes     | no        |
| qwen3.5:latest | few_shot                | no      | yes    | yes       | yes    | yes        | yes     | no        |
| qwen3.5:latest | one_shot_mistake        | no      | yes    | yes       | yes    | yes        | yes     | no        |
| gemma3:12b     | one_shot_correct        | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| qwen3.5:latest | few_shot_in_system      | no      | yes    | yes       | yes    | no         | yes     | no        |
| gemma3:12b     | plain                   | no      | yes    | no        | yes    | yes        | yes     | no        |
| qwen3:8b       | few_shot_mostly_correct | no      | yes    | no        | yes    | yes        | yes     | no        |
| qwen3:8b       | one_shot_mistake        | no      | yes    | no        | yes    | no         | yes     | no        |
| qwen3:8b       | plain                   | no      | yes    | no        | yes    | no         | yes     | no        |
| qwen3:8b       | few_shot_reversed       | no      | yes    | no        | yes    | yes        | yes     | no        |
| qwen3:8b       | few_shot                | no      | no     | no        | yes    | yes        | yes     | no        |
| qwen3.5:latest | plain                   | yes     | yes    | yes       | yes    | no         | yes     | no        |
| qwen3:8b       | few_shot_in_system      | no      | yes    | no        | yes    | yes        | yes     | no        |
| qwen3:4b       | few_shot_six            | no      | yes    | no        | yes    | no         | yes     | no        |
| qwen3:4b       | few_shot                | yes     | yes    | yes       | yes    | no         | yes     | no        |
| qwen3:4b       | one_shot_correct        | no      | yes    | no        | yes    | no         | yes     | no        |
| qwen3:8b       | one_shot_correct        | no      | yes    | no        | yes    | yes        | yes     | no        |
| qwen3:4b       | few_shot_in_system      | no      | yes    | no        | yes    | no         | yes     | no        |
| qwen3:4b       | few_shot_reversed       | no      | yes    | no        | yes    | yes        | yes     | no        |
| gemma3:4b      | few_shot_six            | no      | yes    | no        | no     | no         | yes     | no        |
| qwen3:4b       | few_shot_mostly_correct | no      | yes    | no        | no     | no         | yes     | no        |
| gemma3:4b      | few_shot                | no      | yes    | no        | no     | no         | yes     | no        |
| qwen3:4b       | one_shot_mistake        | no      | yes    | no        | yes    | no         | yes     | no        |
| gemma3:4b      | few_shot_in_system      | no      | yes    | no        | no     | no         | yes     | no        |
| gemma3:4b      | few_shot_reversed       | no      | no     | no        | yes    | no         | yes     | no        |
| gemma3:4b      | one_shot_mistake        | no      | no     | no        | yes    | no         | yes     | no        |
| gemma3:4b      | one_shot_correct        | yes     | no     | no        | no     | no         | yes     | no        |
| gemma3:4b      | few_shot_mostly_correct | no      | yes    | no        | no     | no         | yes     | no        |
| qwen3:4b       | plain                   | no      | no     | no        | yes    | no         | yes     | no        |
| gemma3:4b      | plain                   | no      | yes    | no        | yes    | no         | yes     | no        |
