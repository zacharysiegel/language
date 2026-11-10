# Grading review sample

The same cases for every run, for rating the explanations by hand.

## cross-llama:gemma-4-12b-it-qat-q4_0:thinking-off / few_shot_six

### h-a1-control-weather-cold (control, a1)

- Task: Describe the weather today.
- Answer: Hoje está frio e o céu está nublado.
- Flags: written_in_portuguese=true answers_the_question=true

### h-a1-control-market-purchases (control, a1)

- Task: What did you buy at the market?
- Answer: Comprei bananas, pão e queijo no mercado.
- Flags: written_in_portuguese=true answers_the_question=true

### h-a2-control-comparison-cities (control, a2)

- Task: Compare the two cities.
- Answer: Recife é mais quente do que Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true

### h-a2-control-a-gente-sunday (control, a2)

- Task: How do you and your friends spend Sundays?
- Answer: Aos domingos a gente almoça junto e depois assiste a um filme.
- Flags: written_in_portuguese=true answers_the_question=true

### h-b2-control-reported-doctor (control, b2)

- Task: Report what the doctor said.
- Answer: O médico disse que eu precisava descansar e beber mais água.
- Flags: written_in_portuguese=true answers_the_question=true

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tem` -> `tenho` [verb_form] The verb 'ter' must be conjugated as 'tenho' for the first person singular 'eu'.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `branca casa` -> `casa branca` [word_order] In Portuguese, adjectives usually come after the noun.

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `do` [article_preposition] The preposition 'de' and the article 'o' must contract to 'do'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] The verb 'pedir' (to order/ask for) should be used instead of 'perguntar' (to ask a question).

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `a escola` -> `à escola` [article_preposition] The preposition 'a' must contract with the article 'a' to form 'à' when indicating destination.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true
