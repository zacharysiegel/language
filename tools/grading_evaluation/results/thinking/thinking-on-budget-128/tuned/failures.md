# Grading failures

Every call with a missed error, a false alarm, a quote not in the answer, a wrong flag, or an invalid response.

## cross-llama:gemma-4-12b-it-qat-q4_0:thinking-on-budget-128 / few_shot_six

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Reasoning (128 tokens): * Task: What is your favorite food? * Level: A1 * Answer: "Minha comida favorita é fejão com arroz." * Scene facts: None provided. * "Minha comida favorita" (My favorite food) - Correct. * "é" (is) - Correct. * "fejão" (beans) - Correct. * "com" (with) - Correct. * "arroz" (rice) - Correct. * The sentence is grammatically
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fejão`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Reasoning (128 tokens): The user wants me to check a Portuguese answer for mistakes based on specific rules. Task: What did you do today? Level: A2 Answer: "Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove." Analysis: 1. "Hoje eu acordei tarde" - Correct. 2. "tomei cafe" - "cafe" should be "café" (missing accent). 3. "e fui trabalhar de onibus" - "onibus" should be "ônibus" (missing circumflex accent).
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent mark: café.
- Found (matched): `onibus` -> `ônibus` [spelling] The word needs a circumflex accent: ônibus.
- Missed: `as nove`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Reasoning (128 tokens): * Task: Describe your best friend. * Scene facts: Best friend is a woman, clever and funny, with short black hair. * Level: A2. * Answer: "Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos." * "Minha melhor amiga é muito inteligente e engraçado." * "Minha melhor amiga" (Subject: feminine singular). * "inteligente" (Adjective: neutral/feminine singular - OK). * "engra
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective must be feminine to agree with 'amiga'.
- Found (false positive): `cabelos` -> `cabelo` [word_choice] In Portuguese, 'cabelo' is usually used in the singular to describe hair as a whole.
- Found (matched): `curtas` -> `curto` [agreement] The adjective must be singular to agree with 'cabelo'.

### a2-pronoun-reciprocal (erroneous, a2)

- Task: Describe your relationship with your neighbors.
- Answer: Eu me dou bem com meus vizinhos e sempre nós ajudamos quando precisamos.
- Reference: `nós ajudamos` -> nos ajudamos [pronoun]
- Reasoning (128 tokens): The user wants me to check a Portuguese answer for mistakes based on specific rules. Task: Describe your relationship with your neighbors. Level: A2 Answer: "Eu me dou bem com meus vizinhos e sempre nós ajudamos quando precisamos." Analysis: 1. "Eu me dou bem com meus vizinhos" - Correct. "Se dar bem com alguém" is a common expression. 2. "e sempre nós ajudamos" - Grammatically correct, but "a gente ajuda" or "nós nos ajudamos" (if mutual) or "nos ajudamos" (if we help each other
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `nós ajudamos`

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Reasoning (128 tokens): The user wants me to check a Portuguese answer for mistakes based on specific rules. Task: What happened on your test? Level: A2 Answer: Ontem eu fiz um erro na prova de matemática. Analysis: - "Ontem" (Yesterday) - Correct. - "eu fiz" (I did/made) - Correct conjugation of "fazer" in the pretérito perfeito for "eu". - "um erro" (a mistake) - Correct. - "na prova" (on the test) - Correct contraction of "em" + "a". - "
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Reasoning (128 tokens): * Task: How is your brother? * Level: A2 * Answer: "Meu irmão é doente e não vai trabalhar hoje." * Scene facts: None provided. * Language: Portuguese (pt-BR). * "Meu irmão" (My brother) - Correct. * "é doente" (is sick) - "Doente" is correct, but in Brazilian Portuguese, "está doente" is much more common for a temporary state like being sick. However, "é doente" is grammatically correct
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `é doente`
