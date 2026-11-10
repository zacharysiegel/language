# Grading evaluation

## Error detection

| Model          | Prompt   | Precision | Recall | F1  | Control false alarms | Quotes not in answer | Category | Correction | Imprecise quote | Duplicates |
| -------------- | -------- | --------- | ------ | --- | -------------------- | -------------------- | -------- | ---------- | --------------- | ---------- |
| gemma3:12b     | few_shot | 89%       | 76%    | 82% | 4%                   | 0%                   | 80%      | 70%        | 0%              | 2%         |
| qwen3:8b       | plain    | 67%       | 86%    | 75% | 19%                  | 0%                   | 83%      | 61%        | 0%              | 3%         |
| qwen3.5:latest | plain    | 68%       | 71%    | 70% | 12%                  | 2%                   | 95%      | 68%        | 0%              | 5%         |

## Behavior and cost

| Model          | Prompt   | Parameters | Download | Valid response | On-topic flag | Language flag | Unstable cases | p50 s | p95 s | Tokens/s |
| -------------- | -------- | ---------- | -------- | -------------- | ------------- | ------------- | -------------- | ----- | ----- | -------- |
| gemma3:12b     | few_shot | 11.8B      | 8.2 GB   | 100%           | 96%           | 100%          | 4%             | 3.0   | 7.1   | 24       |
| qwen3:8b       | plain    | 8.2B       | 5.2 GB   | 100%           | 100%          | 100%          | 18%            | 1.8   | 4.0   | 46       |
| qwen3.5:latest | plain    | 9.7B       | 6.6 GB   | 100%           | 95%           | 100%          | 21%            | 2.7   | 7.9   | 32       |

## Recall by error category

| Model          | Prompt   | spelling     | verb_form   | ser_estar    | agreement   | article_preposition | word_choice | word_order | pronoun    | not_brazilian | content_mismatch |
| -------------- | -------- | ------------ | ----------- | ------------ | ----------- | ------------------- | ----------- | ---------- | ---------- | ------------- | ---------------- |
| gemma3:12b     | few_shot | 71% (15/21)  | 77% (30/39) | 100% (12/12) | 72% (13/18) | 80% (12/15)         | 33% (3/9)   | 100% (6/6) | 33% (3/9)  | 100% (9/9)    | 100% (9/9)       |
| qwen3:8b       | plain    | 100% (21/21) | 82% (32/39) | 100% (12/12) | 83% (15/18) | 93% (14/15)         | 67% (6/9)   | 50% (3/6)  | 100% (9/9) | 56% (5/9)     | 100% (9/9)       |
| qwen3.5:latest | plain    | 71% (15/21)  | 77% (30/39) | 100% (12/12) | 78% (14/18) | 53% (8/15)          | 67% (6/9)   | 33% (2/6)  | 33% (3/9)  | 67% (6/9)     | 100% (9/9)       |

## Bar

- Control false alarms at most 10%
- Quotes not in the answer at most 2%
- Precision at least 70%
- Recall at least 60%
- Correction agreement at least 60%
- p95 latency at most 15 s

| Model          | Prompt   | Control | Quotes | Precision | Recall | Correction | Latency | Meets all |
| -------------- | -------- | ------- | ------ | --------- | ------ | ---------- | ------- | --------- |
| gemma3:12b     | few_shot | yes     | yes    | yes       | yes    | yes        | yes     | yes       |
| qwen3:8b       | plain    | no      | yes    | no        | yes    | yes        | yes     | no        |
| qwen3.5:latest | plain    | no      | yes    | no        | yes    | yes        | yes     | no        |
