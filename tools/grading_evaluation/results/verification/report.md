# Finding verification

| Decider | Input            | Model      | Real findings | False findings | Mean probability, real | Mean probability, false | AUROC |
| ------- | ---------------- | ---------- | ------------- | -------------- | ---------------------- | ----------------------- | ----- |
| kev     | bare             | kev-latest | 194           | 52             | 0.53                   | 0.44                    | 0.69  |
| kev     | with_explanation | kev-latest | 194           | 52             | 0.64                   | 0.51                    | 0.70  |
| von     | bare             | von-1.2.0  | 194           | 52             | 0.58                   | 0.55                    | 0.60  |
| von     | with_explanation | von-1.2.0  | 194           | 52             | 0.56                   | 0.51                    | 0.63  |

## kev (bare input)

### Calibration

| Stated probability | Findings | Mean stated probability | Proportion real |
| ------------------ | -------- | ----------------------- | --------------- |
| 0.0 to 0.2         | 0        | n/a                     | n/a             |
| 0.2 to 0.4         | 87       | 0.35                    | 0.61            |
| 0.4 to 0.6         | 87       | 0.48                    | 0.91            |
| 0.6 to 0.8         | 54       | 0.70                    | 0.81            |
| 0.8 to 1.0         | 18       | 0.83                    | 1.00            |

### Grading with findings below the threshold removed

| Model          | Prompt   | Threshold | Findings kept | Precision    | Recall      | False positive rate (correct answers) |
| -------------- | -------- | --------- | ------------- | ------------ | ----------- | ------------------------------------- |
| qwen3.5:latest | few_shot | none      | 57            | 72% (41/57)  | 84% (41/49) | 38% (9/24)                            |
| qwen3.5:latest | few_shot | 0.3       | 53            | 75% (40/53)  | 82% (40/49) | 29% (7/24)                            |
| qwen3.5:latest | few_shot | 0.5       | 22            | 77% (17/22)  | 35% (17/49) | 0% (0/24)                             |
| qwen3.5:latest | few_shot | 0.7       | 7             | 71% (5/7)    | 10% (5/49)  | 0% (0/24)                             |
| qwen3.5:latest | few_shot | 0.8       | 2             | 100% (2/2)   | 4% (2/49)   | 0% (0/24)                             |
| qwen3.5:latest | few_shot | 0.9       | 0             | n/a          | 0% (0/49)   | 0% (0/24)                             |
| gemma3:12b     | few_shot | none      | 41            | 93% (38/41)  | 78% (38/49) | 4% (1/24)                             |
| gemma3:12b     | few_shot | 0.3       | 38            | 95% (36/38)  | 73% (36/49) | 4% (1/24)                             |
| gemma3:12b     | few_shot | 0.5       | 21            | 100% (21/21) | 43% (21/49) | 0% (0/24)                             |
| gemma3:12b     | few_shot | 0.7       | 6             | 100% (6/6)   | 12% (6/49)  | 0% (0/24)                             |
| gemma3:12b     | few_shot | 0.8       | 4             | 100% (4/4)   | 8% (4/49)   | 0% (0/24)                             |
| gemma3:12b     | few_shot | 0.9       | 0             | n/a          | 0% (0/49)   | 0% (0/24)                             |
| qwen3:8b       | few_shot | none      | 56            | 68% (38/56)  | 78% (38/49) | 29% (7/24)                            |
| qwen3:8b       | few_shot | 0.3       | 47            | 81% (38/47)  | 78% (38/49) | 29% (7/24)                            |
| qwen3:8b       | few_shot | 0.5       | 17            | 100% (17/17) | 35% (17/49) | 0% (0/24)                             |
| qwen3:8b       | few_shot | 0.7       | 7             | 100% (7/7)   | 14% (7/49)  | 0% (0/24)                             |
| qwen3:8b       | few_shot | 0.8       | 4             | 100% (4/4)   | 8% (4/49)   | 0% (0/24)                             |
| qwen3:8b       | few_shot | 0.9       | 0             | n/a          | 0% (0/49)   | 0% (0/24)                             |
| qwen3.5:latest | plain    | none      | 52            | 69% (36/52)  | 73% (36/49) | 12% (3/24)                            |
| qwen3.5:latest | plain    | 0.3       | 47            | 74% (35/47)  | 71% (35/49) | 12% (3/24)                            |
| qwen3.5:latest | plain    | 0.5       | 23            | 70% (16/23)  | 33% (16/49) | 0% (0/24)                             |
| qwen3.5:latest | plain    | 0.7       | 11            | 64% (7/11)   | 14% (7/49)  | 0% (0/24)                             |
| qwen3.5:latest | plain    | 0.8       | 3             | 100% (3/3)   | 6% (3/49)   | 0% (0/24)                             |
| qwen3.5:latest | plain    | 0.9       | 0             | n/a          | 0% (0/49)   | 0% (0/24)                             |
| qwen3:8b       | plain    | none      | 63            | 65% (41/63)  | 84% (41/49) | 17% (4/24)                            |
| qwen3:8b       | plain    | 0.3       | 52            | 79% (41/52)  | 84% (41/49) | 17% (4/24)                            |
| qwen3:8b       | plain    | 0.5       | 19            | 95% (18/19)  | 37% (18/49) | 0% (0/24)                             |
| qwen3:8b       | plain    | 0.7       | 7             | 86% (6/7)    | 12% (6/49)  | 0% (0/24)                             |
| qwen3:8b       | plain    | 0.8       | 5             | 100% (5/5)   | 10% (5/49)  | 0% (0/24)                             |
| qwen3:8b       | plain    | 0.9       | 0             | n/a          | 0% (0/49)   | 0% (0/24)                             |

## kev (with_explanation input)

### Calibration

| Stated probability | Findings | Mean stated probability | Proportion real |
| ------------------ | -------- | ----------------------- | --------------- |
| 0.0 to 0.2         | 2        | 0.14                    | 0.00            |
| 0.2 to 0.4         | 28       | 0.34                    | 0.61            |
| 0.4 to 0.6         | 86       | 0.49                    | 0.73            |
| 0.6 to 0.8         | 78       | 0.70                    | 0.83            |
| 0.8 to 1.0         | 52       | 0.85                    | 0.94            |

### Grading with findings below the threshold removed

| Model          | Prompt   | Threshold | Findings kept | Precision    | Recall      | False positive rate (correct answers) |
| -------------- | -------- | --------- | ------------- | ------------ | ----------- | ------------------------------------- |
| qwen3.5:latest | few_shot | none      | 57            | 72% (41/57)  | 84% (41/49) | 38% (9/24)                            |
| qwen3.5:latest | few_shot | 0.3       | 51            | 80% (41/51)  | 84% (41/49) | 17% (4/24)                            |
| qwen3.5:latest | few_shot | 0.5       | 38            | 79% (30/38)  | 61% (30/49) | 8% (2/24)                             |
| qwen3.5:latest | few_shot | 0.7       | 17            | 94% (16/17)  | 33% (16/49) | 0% (0/24)                             |
| qwen3.5:latest | few_shot | 0.8       | 10            | 100% (10/10) | 20% (10/49) | 0% (0/24)                             |
| qwen3.5:latest | few_shot | 0.9       | 0             | n/a          | 0% (0/49)   | 0% (0/24)                             |
| gemma3:12b     | few_shot | none      | 41            | 93% (38/41)  | 78% (38/49) | 4% (1/24)                             |
| gemma3:12b     | few_shot | 0.3       | 38            | 95% (36/38)  | 73% (36/49) | 4% (1/24)                             |
| gemma3:12b     | few_shot | 0.5       | 26            | 100% (26/26) | 53% (26/49) | 0% (0/24)                             |
| gemma3:12b     | few_shot | 0.7       | 16            | 100% (16/16) | 33% (16/49) | 0% (0/24)                             |
| gemma3:12b     | few_shot | 0.8       | 11            | 100% (11/11) | 22% (11/49) | 0% (0/24)                             |
| gemma3:12b     | few_shot | 0.9       | 2             | 100% (2/2)   | 4% (2/49)   | 0% (0/24)                             |
| qwen3:8b       | few_shot | none      | 56            | 68% (38/56)  | 78% (38/49) | 29% (7/24)                            |
| qwen3:8b       | few_shot | 0.3       | 46            | 78% (36/46)  | 73% (36/49) | 25% (6/24)                            |
| qwen3:8b       | few_shot | 0.5       | 29            | 90% (26/29)  | 53% (26/49) | 8% (2/24)                             |
| qwen3:8b       | few_shot | 0.7       | 19            | 100% (19/19) | 39% (19/49) | 0% (0/24)                             |
| qwen3:8b       | few_shot | 0.8       | 11            | 100% (11/11) | 22% (11/49) | 0% (0/24)                             |
| qwen3:8b       | few_shot | 0.9       | 1             | 100% (1/1)   | 2% (1/49)   | 0% (0/24)                             |
| qwen3.5:latest | plain    | none      | 52            | 69% (36/52)  | 73% (36/49) | 12% (3/24)                            |
| qwen3.5:latest | plain    | 0.3       | 47            | 74% (35/47)  | 71% (35/49) | 12% (3/24)                            |
| qwen3.5:latest | plain    | 0.5       | 37            | 76% (28/37)  | 57% (28/49) | 4% (1/24)                             |
| qwen3.5:latest | plain    | 0.7       | 23            | 74% (17/23)  | 35% (17/49) | 0% (0/24)                             |
| qwen3.5:latest | plain    | 0.8       | 11            | 73% (8/11)   | 16% (8/49)  | 0% (0/24)                             |
| qwen3.5:latest | plain    | 0.9       | 1             | 100% (1/1)   | 2% (1/49)   | 0% (0/24)                             |
| qwen3:8b       | plain    | none      | 63            | 65% (41/63)  | 84% (41/49) | 17% (4/24)                            |
| qwen3:8b       | plain    | 0.3       | 53            | 77% (41/53)  | 84% (41/49) | 17% (4/24)                            |
| qwen3:8b       | plain    | 0.5       | 35            | 89% (31/35)  | 63% (31/49) | 4% (1/24)                             |
| qwen3:8b       | plain    | 0.7       | 16            | 94% (15/16)  | 31% (15/49) | 0% (0/24)                             |
| qwen3:8b       | plain    | 0.8       | 9             | 100% (9/9)   | 18% (9/49)  | 0% (0/24)                             |
| qwen3:8b       | plain    | 0.9       | 1             | 100% (1/1)   | 2% (1/49)   | 0% (0/24)                             |

## von (bare input)

### Calibration

| Stated probability | Findings | Mean stated probability | Proportion real |
| ------------------ | -------- | ----------------------- | --------------- |
| 0.0 to 0.2         | 0        | n/a                     | n/a             |
| 0.2 to 0.4         | 38       | 0.37                    | 0.74            |
| 0.4 to 0.6         | 53       | 0.53                    | 0.74            |
| 0.6 to 0.8         | 155      | 0.63                    | 0.82            |
| 0.8 to 1.0         | 0        | n/a                     | n/a             |

### Grading with findings below the threshold removed

| Model          | Prompt   | Threshold | Findings kept | Precision   | Recall      | False positive rate (correct answers) |
| -------------- | -------- | --------- | ------------- | ----------- | ----------- | ------------------------------------- |
| qwen3.5:latest | few_shot | none      | 57            | 72% (41/57) | 84% (41/49) | 38% (9/24)                            |
| qwen3.5:latest | few_shot | 0.3       | 52            | 77% (40/52) | 82% (40/49) | 29% (7/24)                            |
| qwen3.5:latest | few_shot | 0.5       | 39            | 82% (32/39) | 65% (32/49) | 17% (4/24)                            |
| qwen3.5:latest | few_shot | 0.7       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3.5:latest | few_shot | 0.8       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3.5:latest | few_shot | 0.9       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| gemma3:12b     | few_shot | none      | 41            | 93% (38/41) | 78% (38/49) | 4% (1/24)                             |
| gemma3:12b     | few_shot | 0.3       | 39            | 95% (37/39) | 76% (37/49) | 4% (1/24)                             |
| gemma3:12b     | few_shot | 0.5       | 31            | 94% (29/31) | 59% (29/49) | 4% (1/24)                             |
| gemma3:12b     | few_shot | 0.7       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| gemma3:12b     | few_shot | 0.8       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| gemma3:12b     | few_shot | 0.9       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3:8b       | few_shot | none      | 56            | 68% (38/56) | 78% (38/49) | 29% (7/24)                            |
| qwen3:8b       | few_shot | 0.3       | 49            | 78% (38/49) | 78% (38/49) | 29% (7/24)                            |
| qwen3:8b       | few_shot | 0.5       | 38            | 79% (30/38) | 61% (30/49) | 21% (5/24)                            |
| qwen3:8b       | few_shot | 0.7       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3:8b       | few_shot | 0.8       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3:8b       | few_shot | 0.9       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3.5:latest | plain    | none      | 52            | 69% (36/52) | 73% (36/49) | 12% (3/24)                            |
| qwen3.5:latest | plain    | 0.3       | 49            | 73% (36/49) | 73% (36/49) | 12% (3/24)                            |
| qwen3.5:latest | plain    | 0.5       | 37            | 78% (29/37) | 59% (29/49) | 12% (3/24)                            |
| qwen3.5:latest | plain    | 0.7       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3.5:latest | plain    | 0.8       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3.5:latest | plain    | 0.9       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3:8b       | plain    | none      | 63            | 65% (41/63) | 84% (41/49) | 17% (4/24)                            |
| qwen3:8b       | plain    | 0.3       | 54            | 76% (41/54) | 84% (41/49) | 17% (4/24)                            |
| qwen3:8b       | plain    | 0.5       | 45            | 71% (32/45) | 65% (32/49) | 17% (4/24)                            |
| qwen3:8b       | plain    | 0.7       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3:8b       | plain    | 0.8       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3:8b       | plain    | 0.9       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |

## von (with_explanation input)

### Calibration

| Stated probability | Findings | Mean stated probability | Proportion real |
| ------------------ | -------- | ----------------------- | --------------- |
| 0.0 to 0.2         | 0        | n/a                     | n/a             |
| 0.2 to 0.4         | 35       | 0.36                    | 0.71            |
| 0.4 to 0.6         | 97       | 0.52                    | 0.75            |
| 0.6 to 0.8         | 114      | 0.63                    | 0.84            |
| 0.8 to 1.0         | 0        | n/a                     | n/a             |

### Grading with findings below the threshold removed

| Model          | Prompt   | Threshold | Findings kept | Precision   | Recall      | False positive rate (correct answers) |
| -------------- | -------- | --------- | ------------- | ----------- | ----------- | ------------------------------------- |
| qwen3.5:latest | few_shot | none      | 57            | 72% (41/57) | 84% (41/49) | 38% (9/24)                            |
| qwen3.5:latest | few_shot | 0.3       | 54            | 76% (41/54) | 84% (41/49) | 29% (7/24)                            |
| qwen3.5:latest | few_shot | 0.5       | 37            | 84% (31/37) | 63% (31/49) | 17% (4/24)                            |
| qwen3.5:latest | few_shot | 0.7       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3.5:latest | few_shot | 0.8       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3.5:latest | few_shot | 0.9       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| gemma3:12b     | few_shot | none      | 41            | 93% (38/41) | 78% (38/49) | 4% (1/24)                             |
| gemma3:12b     | few_shot | 0.3       | 40            | 95% (38/40) | 78% (38/49) | 4% (1/24)                             |
| gemma3:12b     | few_shot | 0.5       | 30            | 97% (29/30) | 59% (29/49) | 0% (0/24)                             |
| gemma3:12b     | few_shot | 0.7       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| gemma3:12b     | few_shot | 0.8       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| gemma3:12b     | few_shot | 0.9       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3:8b       | few_shot | none      | 56            | 68% (38/56) | 78% (38/49) | 29% (7/24)                            |
| qwen3:8b       | few_shot | 0.3       | 49            | 78% (38/49) | 78% (38/49) | 29% (7/24)                            |
| qwen3:8b       | few_shot | 0.5       | 35            | 80% (28/35) | 57% (28/49) | 21% (5/24)                            |
| qwen3:8b       | few_shot | 0.7       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3:8b       | few_shot | 0.8       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3:8b       | few_shot | 0.9       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3.5:latest | plain    | none      | 52            | 69% (36/52) | 73% (36/49) | 12% (3/24)                            |
| qwen3.5:latest | plain    | 0.3       | 49            | 73% (36/49) | 73% (36/49) | 12% (3/24)                            |
| qwen3.5:latest | plain    | 0.5       | 33            | 82% (27/33) | 55% (27/49) | 12% (3/24)                            |
| qwen3.5:latest | plain    | 0.7       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3.5:latest | plain    | 0.8       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3.5:latest | plain    | 0.9       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3:8b       | plain    | none      | 63            | 65% (41/63) | 84% (41/49) | 17% (4/24)                            |
| qwen3:8b       | plain    | 0.3       | 54            | 76% (41/54) | 84% (41/49) | 17% (4/24)                            |
| qwen3:8b       | plain    | 0.5       | 40            | 75% (30/40) | 61% (30/49) | 17% (4/24)                            |
| qwen3:8b       | plain    | 0.7       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3:8b       | plain    | 0.8       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
| qwen3:8b       | plain    | 0.9       | 0             | n/a         | 0% (0/49)   | 0% (0/24)                             |
