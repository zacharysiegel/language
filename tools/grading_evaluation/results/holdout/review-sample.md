# Grading review sample

The same cases for every run, for rating the explanations by hand.

## gemma3:12b / few_shot

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
- Found (matched): `tem` -> `tenho` [verb_form] With 'eu', the verb 'ter' conjugates to 'tenho'.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective needs to agree in gender with 'casa', which is feminine.
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `o guarda-chuva` -> `de guarda-chuva` [article_preposition] When 'guarda-chuva' is the object of a preposition, it requires 'de' instead of 'o'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [verb_form] 'Pedir' (to order) is the correct verb to use when ordering food.
- Found (changes nothing): `um suco` -> `um suco` [word_choice] 'Um suco' is correct, but 'um suco' is more natural.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `a escola` -> `escola` [article_preposition] When using a time reference, 'a' is not used before 'escola'.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## gemma3:12b / few_shot_six

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
- Found (matched): `tem` -> `tenho` [verb_form] With 'eu', the verb 'ter' conjugates to 'tenho'.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective must agree in gender with 'casa' (feminine).
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `do` [article_preposition] The article 'o' is not needed after the preposition 'de'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] 'pedir' (to order) is the correct verb here, not 'perguntar' (to ask).
- Found (false positive): `um suco` -> `suco` [article_preposition] When ordering food or drinks, 'um' is not used.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `a escola` -> `na escola` [article_preposition] After 'a' you need the preposition 'em' when referring to a location.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true
