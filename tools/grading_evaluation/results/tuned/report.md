# Grading evaluation

## Error detection

| Model          | Prompt   | Precision | Recall | F1  | Control false alarms | Quotes not in answer | Category | Correction | Imprecise quote | Duplicates |
| -------------- | -------- | --------- | ------ | --- | -------------------- | -------------------- | -------- | ---------- | --------------- | ---------- |
| gemma3:1b      | few_shot | n/a       | 0%     | n/a | 0%                   | n/a                  | n/a      | n/a        | n/a             | n/a        |
| gemma3:12b     | few_shot | 93%       | 78%    | 84% | 4%                   | 0%                   | 82%      | 66%        | 0%              | 2%         |
| qwen3.5:latest | few_shot | 72%       | 84%    | 77% | 38%                  | 0%                   | 93%      | 66%        | 0%              | 5%         |
| gemma3:12b     | plain    | 72%       | 80%    | 76% | 33%                  | 0%                   | 79%      | 67%        | 0%              | 2%         |
| qwen3:8b       | plain    | 65%       | 84%    | 73% | 17%                  | 0%                   | 85%      | 61%        | 0%              | 2%         |
| qwen3:8b       | few_shot | 68%       | 78%    | 72% | 29%                  | 2%                   | 84%      | 66%        | 0%              | 2%         |
| qwen3.5:latest | plain    | 69%       | 73%    | 71% | 12%                  | 2%                   | 94%      | 61%        | 0%              | 5%         |
| qwen3:4b       | few_shot | 70%       | 61%    | 65% | 12%                  | 0%                   | 80%      | 67%        | 0%              | 0%         |
| aya-expanse:8b | plain    | 46%       | 65%    | 54% | 100%                 | 6%                   | 47%      | 59%        | 28%             | 0%         |
| gemma3:4b      | few_shot | 56%       | 49%    | 52% | 33%                  | 2%                   | 50%      | 42%        | 4%              | 0%         |
| qwen3:4b       | plain    | 40%       | 67%    | 50% | 58%                  | 2%                   | 79%      | 58%        | 0%              | 2%         |
| qwen3:1.7b     | plain    | 36%       | 55%    | 44% | 83%                  | 4%                   | 19%      | 26%        | 37%             | 0%         |
| gemma3:4b      | plain    | 36%       | 55%    | 43% | 100%                 | 0%                   | 48%      | 44%        | 0%              | 0%         |
| llama3.1:8b    | plain    | 30%       | 73%    | 43% | 96%                  | 1%                   | 42%      | 33%        | 0%              | 2%         |
| qwen3:1.7b     | few_shot | 57%       | 33%    | 42% | 21%                  | 7%                   | 50%      | 44%        | 6%              | 0%         |
| llama3.1:8b    | few_shot | 37%       | 37%    | 37% | 62%                  | 4%                   | 78%      | 67%        | 0%              | 0%         |
| llama3.2:3b    | plain    | 24%       | 63%    | 35% | 100%                 | 4%                   | 39%      | 42%        | 3%              | 1%         |
| phi4-mini      | plain    | 18%       | 55%    | 27% | 100%                 | 16%                  | 33%      | 15%        | 19%             | 6%         |
| llama3.2:3b    | few_shot | 30%       | 22%    | 26% | 42%                  | 5%                   | 64%      | 55%        | 0%              | 0%         |
| aya-expanse:8b | few_shot | 60%       | 12%    | 20% | 0%                   | 9%                   | 50%      | 67%        | 0%              | 9%         |
| gemma3:1b      | plain    | 14%       | 20%    | 17% | 100%                 | 12%                  | 30%      | 0%         | 0%              | 0%         |
| phi4-mini      | few_shot | 40%       | 4%     | 7%  | 0%                   | 0%                   | 50%      | 50%        | 0%              | 0%         |

## Counts behind the rates

| Model          | Prompt   | Answers graded | Reference mistakes | Found correctly | False alarms | Quotes not in answer | Correct answers | Correct answers flagged |
| -------------- | -------- | -------------- | ------------------ | --------------- | ------------ | -------------------- | --------------- | ----------------------- |
| gemma3:1b      | few_shot | 71             | 49                 | 0               | 0            | 0                    | 24              | 0                       |
| gemma3:12b     | few_shot | 71             | 49                 | 38              | 3            | 0                    | 24              | 1                       |
| qwen3.5:latest | few_shot | 71             | 49                 | 41              | 16           | 0                    | 24              | 9                       |
| gemma3:12b     | plain    | 71             | 49                 | 39              | 15           | 0                    | 24              | 8                       |
| qwen3:8b       | plain    | 71             | 49                 | 41              | 22           | 0                    | 24              | 4                       |
| qwen3:8b       | few_shot | 71             | 49                 | 38              | 17           | 1                    | 24              | 7                       |
| qwen3.5:latest | plain    | 71             | 49                 | 36              | 15           | 1                    | 24              | 3                       |
| qwen3:4b       | few_shot | 71             | 49                 | 30              | 13           | 0                    | 24              | 3                       |
| aya-expanse:8b | plain    | 71             | 49                 | 32              | 34           | 4                    | 24              | 24                      |
| gemma3:4b      | few_shot | 71             | 49                 | 24              | 18           | 1                    | 24              | 8                       |
| qwen3:4b       | plain    | 71             | 49                 | 33              | 48           | 2                    | 24              | 14                      |
| qwen3:1.7b     | plain    | 71             | 49                 | 27              | 44           | 3                    | 24              | 20                      |
| gemma3:4b      | plain    | 71             | 49                 | 27              | 49           | 0                    | 24              | 24                      |
| llama3.1:8b    | plain    | 71             | 49                 | 36              | 83           | 1                    | 24              | 23                      |
| qwen3:1.7b     | few_shot | 71             | 49                 | 16              | 10           | 2                    | 24              | 5                       |
| llama3.1:8b    | few_shot | 71             | 49                 | 18              | 29           | 2                    | 24              | 15                      |
| llama3.2:3b    | plain    | 71             | 49                 | 31              | 94           | 5                    | 24              | 24                      |
| phi4-mini      | plain    | 71             | 49                 | 27              | 97           | 26                   | 24              | 24                      |
| llama3.2:3b    | few_shot | 71             | 49                 | 11              | 24           | 2                    | 24              | 10                      |
| aya-expanse:8b | few_shot | 71             | 49                 | 6               | 3            | 1                    | 24              | 0                       |
| gemma3:1b      | plain    | 71             | 49                 | 10              | 53           | 9                    | 24              | 24                      |
| phi4-mini      | few_shot | 71             | 49                 | 2               | 3            | 0                    | 24              | 0                       |

## Behavior and cost

| Model          | Prompt   | Parameters | Download | Valid response | On-topic flag | Language flag | Unstable cases | p50 s | p95 s | Tokens/s |
| -------------- | -------- | ---------- | -------- | -------------- | ------------- | ------------- | -------------- | ----- | ----- | -------- |
| gemma3:1b      | few_shot | 999.89M    | 0.8 GB   | 100%           | 97%           | 97%           | n/a            | 0.2   | 0.3   | 161      |
| gemma3:12b     | few_shot | 11.8B      | 8.2 GB   | 100%           | 96%           | 100%          | n/a            | 3.4   | 5.3   | 36       |
| qwen3.5:latest | few_shot | 9.7B       | 6.6 GB   | 100%           | 94%           | 100%          | n/a            | 3.3   | 8.0   | 40       |
| gemma3:12b     | plain    | 11.8B      | 8.2 GB   | 100%           | 100%          | 100%          | n/a            | 4.5   | 6.5   | 36       |
| qwen3:8b       | plain    | 8.2B       | 5.2 GB   | 100%           | 100%          | 100%          | n/a            | 2.9   | 6.7   | 30       |
| qwen3:8b       | few_shot | 8.2B       | 5.2 GB   | 100%           | 94%           | 100%          | n/a            | 1.9   | 3.3   | 32       |
| qwen3.5:latest | plain    | 9.7B       | 6.6 GB   | 100%           | 96%           | 100%          | n/a            | 2.7   | 6.8   | 48       |
| qwen3:4b       | few_shot | 4.0B       | 2.5 GB   | 100%           | 99%           | 100%          | n/a            | 1.8   | 3.3   | 40       |
| aya-expanse:8b | plain    | 8.0B       | 5.1 GB   | 100%           | 99%           | 96%           | n/a            | 3.5   | 4.7   | 31       |
| gemma3:4b      | few_shot | 3.9B       | 3.4 GB   | 100%           | 75%           | 99%           | n/a            | 1.7   | 2.3   | 54       |
| qwen3:4b       | plain    | 4.0B       | 2.5 GB   | 100%           | 97%           | 100%          | n/a            | 2.4   | 5.5   | 48       |
| qwen3:1.7b     | plain    | 2.0B       | 1.4 GB   | 100%           | 97%           | 97%           | n/a            | 1.5   | 3.2   | 86       |
| gemma3:4b      | plain    | 3.9B       | 3.4 GB   | 100%           | 97%           | 100%          | n/a            | 2.7   | 3.7   | 44       |
| llama3.1:8b    | plain    | 8.0B       | 4.9 GB   | 97%            | 96%           | 96%           | n/a            | 3.8   | 12.4  | 35       |
| qwen3:1.7b     | few_shot | 2.0B       | 1.4 GB   | 100%           | 87%           | 99%           | n/a            | 0.3   | 1.3   | 100      |
| llama3.1:8b    | few_shot | 8.0B       | 4.9 GB   | 97%            | 94%           | 96%           | n/a            | 1.6   | 3.2   | 39       |
| llama3.2:3b    | plain    | 3.2B       | 2.0 GB   | 100%           | 97%           | 96%           | n/a            | 1.9   | 5.0   | 58       |
| phi4-mini      | plain    | 3.8B       | 2.5 GB   | 96%            | 10%           | 94%           | n/a            | 2.3   | 32.0  | 36       |
| llama3.2:3b    | few_shot | 3.2B       | 2.0 GB   | 100%           | 89%           | 99%           | n/a            | 0.9   | 1.7   | 53       |
| aya-expanse:8b | few_shot | 8.0B       | 5.1 GB   | 100%           | 97%           | 96%           | n/a            | 0.8   | 2.9   | 35       |
| gemma3:1b      | plain    | 999.89M    | 0.8 GB   | 100%           | 20%           | 3%            | n/a            | 0.8   | 1.5   | 130      |
| phi4-mini      | few_shot | 3.8B       | 2.5 GB   | 100%           | 99%           | 97%           | n/a            | 0.8   | 2.1   | 28       |

## Recall by error category

| Model          | Prompt   | spelling   | verb_form   | ser_estar  | agreement  | article_preposition | word_choice | word_order | pronoun    | not_brazilian | content_mismatch |
| -------------- | -------- | ---------- | ----------- | ---------- | ---------- | ------------------- | ----------- | ---------- | ---------- | ------------- | ---------------- |
| gemma3:1b      | few_shot | 0% (0/7)   | 0% (0/13)   | 0% (0/4)   | 0% (0/6)   | 0% (0/5)            | 0% (0/3)    | 0% (0/2)   | 0% (0/3)   | 0% (0/3)      | 0% (0/3)         |
| gemma3:12b     | few_shot | 71% (5/7)  | 85% (11/13) | 100% (4/4) | 67% (4/6)  | 80% (4/5)           | 33% (1/3)   | 100% (2/2) | 33% (1/3)  | 100% (3/3)    | 100% (3/3)       |
| qwen3.5:latest | few_shot | 71% (5/7)  | 85% (11/13) | 100% (4/4) | 100% (6/6) | 100% (5/5)          | 67% (2/3)   | 50% (1/2)  | 33% (1/3)  | 100% (3/3)    | 100% (3/3)       |
| gemma3:12b     | plain    | 71% (5/7)  | 85% (11/13) | 100% (4/4) | 100% (6/6) | 80% (4/5)           | 33% (1/3)   | 50% (1/2)  | 33% (1/3)  | 100% (3/3)    | 100% (3/3)       |
| qwen3:8b       | plain    | 100% (7/7) | 85% (11/13) | 100% (4/4) | 83% (5/6)  | 100% (5/5)          | 67% (2/3)   | 0% (0/2)   | 100% (3/3) | 33% (1/3)     | 100% (3/3)       |
| qwen3:8b       | few_shot | 86% (6/7)  | 77% (10/13) | 100% (4/4) | 83% (5/6)  | 100% (5/5)          | 67% (2/3)   | 0% (0/2)   | 67% (2/3)  | 33% (1/3)     | 100% (3/3)       |
| qwen3.5:latest | plain    | 71% (5/7)  | 77% (10/13) | 100% (4/4) | 83% (5/6)  | 60% (3/5)           | 67% (2/3)   | 50% (1/2)  | 33% (1/3)  | 67% (2/3)     | 100% (3/3)       |
| qwen3:4b       | few_shot | 86% (6/7)  | 62% (8/13)  | 75% (3/4)  | 83% (5/6)  | 40% (2/5)           | 33% (1/3)   | 50% (1/2)  | 33% (1/3)  | 0% (0/3)      | 100% (3/3)       |
| aya-expanse:8b | plain    | 43% (3/7)  | 69% (9/13)  | 100% (4/4) | 50% (3/6)  | 80% (4/5)           | 33% (1/3)   | 100% (2/2) | 33% (1/3)  | 67% (2/3)     | 100% (3/3)       |
| gemma3:4b      | few_shot | 43% (3/7)  | 46% (6/13)  | 50% (2/4)  | 33% (2/6)  | 40% (2/5)           | 33% (1/3)   | 50% (1/2)  | 100% (3/3) | 33% (1/3)     | 100% (3/3)       |
| qwen3:4b       | plain    | 71% (5/7)  | 69% (9/13)  | 75% (3/4)  | 67% (4/6)  | 80% (4/5)           | 33% (1/3)   | 50% (1/2)  | 33% (1/3)  | 67% (2/3)     | 100% (3/3)       |
| qwen3:1.7b     | plain    | 43% (3/7)  | 62% (8/13)  | 25% (1/4)  | 83% (5/6)  | 80% (4/5)           | 33% (1/3)   | 100% (2/2) | 0% (0/3)   | 33% (1/3)     | 67% (2/3)        |
| gemma3:4b      | plain    | 43% (3/7)  | 46% (6/13)  | 100% (4/4) | 83% (5/6)  | 40% (2/5)           | 33% (1/3)   | 0% (0/2)   | 33% (1/3)  | 67% (2/3)     | 100% (3/3)       |
| llama3.1:8b    | plain    | 57% (4/7)  | 69% (9/13)  | 100% (4/4) | 83% (5/6)  | 60% (3/5)           | 67% (2/3)   | 100% (2/2) | 67% (2/3)  | 67% (2/3)     | 100% (3/3)       |
| qwen3:1.7b     | few_shot | 29% (2/7)  | 15% (2/13)  | 50% (2/4)  | 33% (2/6)  | 20% (1/5)           | 33% (1/3)   | 100% (2/2) | 0% (0/3)   | 67% (2/3)     | 67% (2/3)        |
| llama3.1:8b    | few_shot | 14% (1/7)  | 38% (5/13)  | 25% (1/4)  | 50% (3/6)  | 40% (2/5)           | 67% (2/3)   | 0% (0/2)   | 33% (1/3)  | 67% (2/3)     | 33% (1/3)        |
| llama3.2:3b    | plain    | 43% (3/7)  | 62% (8/13)  | 50% (2/4)  | 50% (3/6)  | 80% (4/5)           | 33% (1/3)   | 100% (2/2) | 100% (3/3) | 100% (3/3)    | 67% (2/3)        |
| phi4-mini      | plain    | 43% (3/7)  | 46% (6/13)  | 100% (4/4) | 33% (2/6)  | 60% (3/5)           | 67% (2/3)   | 50% (1/2)  | 33% (1/3)  | 100% (3/3)    | 67% (2/3)        |
| llama3.2:3b    | few_shot | 0% (0/7)   | 23% (3/13)  | 25% (1/4)  | 33% (2/6)  | 40% (2/5)           | 33% (1/3)   | 0% (0/2)   | 0% (0/3)   | 33% (1/3)     | 33% (1/3)        |
| aya-expanse:8b | few_shot | 0% (0/7)   | 15% (2/13)  | 0% (0/4)   | 17% (1/6)  | 20% (1/5)           | 0% (0/3)    | 0% (0/2)   | 0% (0/3)   | 0% (0/3)      | 67% (2/3)        |
| gemma3:1b      | plain    | 0% (0/7)   | 23% (3/13)  | 0% (0/4)   | 17% (1/6)  | 40% (2/5)           | 33% (1/3)   | 50% (1/2)  | 33% (1/3)  | 0% (0/3)      | 33% (1/3)        |
| phi4-mini      | few_shot | 0% (0/7)   | 0% (0/13)   | 0% (0/4)   | 0% (0/6)   | 0% (0/5)            | 0% (0/3)    | 0% (0/2)   | 0% (0/3)   | 0% (0/3)      | 67% (2/3)        |

## Bar

- Control false alarms at most 10%
- Quotes not in the answer at most 2%
- Precision at least 70%
- Recall at least 60%
- Correction agreement at least 60%
- p95 latency at most 15 s

| Model          | Prompt   | Control | Quotes | Precision | Recall | Correction | Latency | Meets all |
| -------------- | -------- | ------- | ------ | --------- | ------ | ---------- | ------- | --------- |
| gemma3:1b      | few_shot | yes     | yes    | no        | no     | no         | yes     | no        |
| gemma3:12b     | few_shot | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| qwen3.5:latest | few_shot | no      | yes    | yes       | yes    | yes        | yes     | no        |
| gemma3:12b     | plain    | no      | yes    | yes       | yes    | yes        | yes     | no        |
| qwen3:8b       | plain    | no      | yes    | no        | yes    | yes        | yes     | no        |
| qwen3:8b       | few_shot | no      | yes    | no        | yes    | yes        | yes     | no        |
| qwen3.5:latest | plain    | no      | yes    | no        | yes    | yes        | yes     | no        |
| qwen3:4b       | few_shot | no      | yes    | no        | yes    | yes        | yes     | no        |
| aya-expanse:8b | plain    | no      | no     | no        | yes    | no         | yes     | no        |
| gemma3:4b      | few_shot | no      | no     | no        | no     | no         | yes     | no        |
| qwen3:4b       | plain    | no      | no     | no        | yes    | no         | yes     | no        |
| qwen3:1.7b     | plain    | no      | no     | no        | no     | no         | yes     | no        |
| gemma3:4b      | plain    | no      | yes    | no        | no     | no         | yes     | no        |
| llama3.1:8b    | plain    | no      | yes    | no        | yes    | no         | yes     | no        |
| qwen3:1.7b     | few_shot | no      | no     | no        | no     | no         | yes     | no        |
| llama3.1:8b    | few_shot | no      | no     | no        | no     | yes        | yes     | no        |
| llama3.2:3b    | plain    | no      | no     | no        | yes    | no         | yes     | no        |
| phi4-mini      | plain    | no      | no     | no        | no     | no         | no      | no        |
| llama3.2:3b    | few_shot | no      | no     | no        | no     | no         | yes     | no        |
| aya-expanse:8b | few_shot | yes     | no     | no        | no     | yes        | yes     | no        |
| gemma3:1b      | plain    | no      | no     | no        | no     | no         | yes     | no        |
| phi4-mini      | few_shot | yes     | yes    | no        | no     | no         | yes     | no        |
