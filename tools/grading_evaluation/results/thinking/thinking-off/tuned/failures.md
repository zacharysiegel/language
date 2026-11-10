# Grading failures

Every call with a missed error, a false alarm, a quote not in the answer, a wrong flag, or an invalid response.

## cross-llama:gemma-4-12b-it-qat-q4_0:thinking-off / few_shot_six

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=false answers_the_question=true
- Missed: `beans`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent mark: café.
- Found (matched): `as nove` -> `às nove` [article_preposition] Time expressions require a grave accent for the contraction: às nove.
- Missed: `onibus`

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `por passar`

### a1-short-spelling-chocolate (erroneous, a1)

- Task: What do you like?
- Answer: Eu gosto de chocolat.
- Reference: `chocolat` -> chocolate [spelling]
- Flags: written_in_portuguese=false answers_the_question=true
- Missed: `chocolat`

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `é doente`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sempre fui`
