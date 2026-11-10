# Grading review sample

The same cases for every run, for rating the explanations by hand.

## gemma3n:e2b / few_shot

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
- Missed: `Eu tem`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [word_choice] The word 'branca' is invariable.
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `o guarda-chuva` -> `o guarda-chuva` [article_preposition] The article 'o' is unnecessary before 'guarda-chuva'.
- Missed: `de o`

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `uma pizza` -> `uma pizza` [word_choice] The word 'uma' is unnecessary and incorrect.
- Missed: `perguntei`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a escola`

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## gemma3n:e2b / few_shot_six

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
- Missed: `Eu tem`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `o guarda-chuva` -> `guarda-chuva` [article_preposition] The article 'o' is unnecessary.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `uma pizza` -> `uma pizza` [word_choice] The word 'uma' is unnecessary.
- Missed: `perguntei`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a escola`

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## gemma3n:e2b / plain

### h-a1-control-weather-cold (control, a1)

- Task: Describe the weather today.
- Answer: Hoje está frio e o céu está nublado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está` -> `é` [ser_estar] The verb 'estar' is used to describe temporary conditions like weather, while 'ser' is used for permanent characteristics.

### h-a1-control-market-purchases (control, a1)

- Task: What did you buy at the market?
- Answer: Comprei bananas, pão e queijo no mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `no` -> `no` [article_preposition] The preposition 'no' is unnecessary and incorrect in this context.

### h-a2-control-comparison-cities (control, a2)

- Task: Compare the two cities.
- Answer: Recife é mais quente do que Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `mais quente` -> `quente` [word_choice] The word 'mais' is redundant when used with 'quente'.

### h-a2-control-a-gente-sunday (control, a2)

- Task: How do you and your friends spend Sundays?
- Answer: Aos domingos a gente almoça junto e depois assiste a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a gente` -> `nós` [pronoun] While 'a gente' is common, it's often better to use 'nós' (we) for clarity in this context.

### h-b2-control-reported-doctor (control, b2)

- Task: Report what the doctor said.
- Answer: O médico disse que eu precisava descansar e beber mais água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `eu` -> `precisava descansar e beber mais água` [pronoun] The pronoun 'eu' is unnecessary and redundant in this sentence.

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tem` -> `tenho` [verb_form] The verb 'ter' (to have) is used with the noun 'anos' (years), not 'tem' (to have).

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective 'branca' (white) doesn't agree with the noun 'casa' (house) in number. It should be 'branca' (white) or 'casas' (houses) depending on the number.
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `o` -> `o` [article_preposition] The article 'o' is unnecessary before 'guarda-chuva' in this context.
- Missed: `de o`

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `perguntei` -> `perguntei` [verb_form] The verb 'perguntar' (to ask) needs to be conjugated to the past tense, which is 'perguntei'.
- Missed: `perguntei`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `cheguei` -> `cheguei` [verb_form] The verb 'chegar' (to arrive) is in the past tense. The correct form for 'at eight' is the present tense.
- Missed: `a escola`

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## gemma3n:e4b / few_shot

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
- Found (matched): `tem` -> `tenho` [verb_form] The verb 'ter' does not agree with the subject 'Eu'. It should be 'tenho'.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `numa` -> `em` [article_preposition] The preposition 'em' is more appropriate here.
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `o` [article_preposition] The preposition 'de' is not needed here.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `uma pizza` -> `o pizza` [article_preposition] The word 'pizza' is a masculine noun, so it needs an article 'o'.
- Missed: `perguntei`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `a escola` -> `à escola` [article_preposition] The preposition 'a' is not needed here.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## gemma3n:e4b / few_shot_six

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
- Found (matched): `tem` -> `tenho` [verb_form] The verb must agree with the subject 'eu': tenho.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `numa` -> `em uma` [article_preposition] The preposition 'em' should be used with 'casa'.
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `o` [article_preposition] The preposition 'de' does not need the article 'o'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [verb_form] The verb must agree with 'eu': pedi.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `a escola` -> `à` [article_preposition] The preposition 'a' is not needed here.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## gemma3n:e4b / plain

### h-a1-control-weather-cold (control, a1)

- Task: Describe the weather today.
- Answer: Hoje está frio e o céu está nublado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está nublado` -> `o céu está nublado` [word_order] The verb 'estar' is correct here, but 'nublado' is an adjective and should be placed after the verb.

### h-a1-control-market-purchases (control, a1)

- Task: What did you buy at the market?
- Answer: Comprei bananas, pão e queijo no mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no mercado` -> `` [article_preposition] The preposition 'no' is redundant when used with 'mercado'.

### h-a2-control-comparison-cities (control, a2)

- Task: Compare the two cities.
- Answer: Recife é mais quente do que Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `é mais quente` -> `está mais quente` [ser_estar] The verb 'ser' is not needed here; 'estar' is required to express temperature.

### h-a2-control-a-gente-sunday (control, a2)

- Task: How do you and your friends spend Sundays?
- Answer: Aos domingos a gente almoça junto e depois assiste a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `junto` -> `` [word_choice] While not strictly incorrect, 'junto' is redundant here; 'juntos' is more natural.

### h-b2-control-reported-doctor (control, b2)

- Task: Report what the doctor said.
- Answer: O médico disse que eu precisava descansar e beber mais água.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `precisava` -> `preciso` [verb_form] The use of the conditional 'precisava' implies a past action that was not completed, which is not the intended meaning. The doctor is giving a present recommendation.

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tem` -> `tenho` [verb_form] The verb 'ter' (to have) needs to be conjugated correctly with 'eu'.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `numa branca` -> `em uma branca` [article_preposition] The preposition 'em' is needed before 'branca' to indicate location.

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `de` [article_preposition] The preposition 'de' is not needed here.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei uma pizza` -> `pedi` [word_choice] The verb 'perguntar' (to ask) is not appropriate here; 'pedir' (to order) is needed.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `a escola` -> `à escola` [article_preposition] The preposition 'a' is not needed before 'escola' when indicating arrival time.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## gemma4:12b-it-qat / few_shot

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
- Found (matched): `tem` -> `tenho` [verb_form] The verb 'ter' must be conjugated as 'tenho' for the first person singular.

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
- Found (matched): `de o` -> `do` [article_preposition] The preposition 'de' and the article 'o' must contract into 'do'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] The verb 'perguntar' means 'to ask'; the correct verb for 'to order' is 'pedir'.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `a escola` -> `à escola` [article_preposition] The preposition 'a' must contract with 'a' to form 'à' when indicating a destination like a school.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## gemma4:12b-it-qat / few_shot_six

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

## gemma4:12b-it-qat / plain

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
- Found (matched): `tem` -> `tenho` [verb_form] The verb 'ter' must be conjugated as 'tenho' for the first person singular.

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
- Found (matched): `de o` -> `do` [article_preposition] The preposition 'de' and the article 'o' must contract into 'do' before a masculine noun.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] The verb 'perguntar' means 'to ask', but 'pedir' is used for 'to order' food.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `a escola` -> `na escola` [article_preposition] The verb 'chegar' requires the preposition 'em', which contracts with the article 'a' to form 'na'.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## gemma4:12b / few_shot

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
- Found (matched): `tem` -> `tenho` [verb_form] The verb 'ter' must be conjugated as 'tenho' for the first person singular.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `branca casa` -> `casa branca` [word_order] In Portuguese, adjectives of color usually come after the noun.

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `do` [article_preposition] The preposition 'de' and the article 'o' must contract into 'do'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] The verb 'perguntar' means 'to ask' (a question); 'pedir' should be used for ordering food.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `a escola` -> `à escola` [article_preposition] The preposition 'a' requires a contraction with the article 'a' to form 'à'.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## gemma4:12b / few_shot_six

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
- Found (matched): `tem` -> `tenho` [verb_form] The verb 'ter' must be conjugated for the first person: tenho.

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
- Found (matched): `perguntei` -> `pedi` [word_choice] The verb 'perguntar' means 'to ask'; the correct verb for 'to order' is 'pedir'.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `a escola` -> `à escola` [article_preposition] The verb 'chegar' requires the preposition 'em' or 'a' with an article, resulting in 'à' for 'a escola'.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## gemma4:12b / plain

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
- Found (matched): `tem` -> `tenho` [verb_form] The verb 'ter' must be conjugated as 'tenho' for the first person singular.

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
- Found (matched): `de o` -> `do` [article_preposition] The preposition 'de' combines with the article 'o' to form the contraction 'do'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] The verb 'perguntar' means 'to ask' (a question), while 'pedir' is used for ordering food.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `a escola` -> `à escola` [article_preposition] The preposition 'a' needs a contraction with the article 'a' to become 'à' before a feminine noun.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## gemma4:e2b-it-qat / few_shot

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
- Found (changes nothing): `a gente` -> `a gente` [word_order] The spoken Brazilian form is 'a gente', but in written form, it is usually written as 'a gente' or 'nós'. However, the prompt asks for real mistakes, and 'a gente' is acceptable in spoken context. No clear grammatical error is present here based on the rules.

### h-b2-control-reported-doctor (control, b2)

- Task: Report what the doctor said.
- Answer: O médico disse que eu precisava descansar e beber mais água.
- Flags: written_in_portuguese=true answers_the_question=true

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tem` -> `tenho` [verb_form] The verb 'ter' conjugated for 'eu' is 'tenho'.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma branca casa` -> `uma casa branca` [agreement] The adjective must agree with the feminine noun 'casa'.

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `o` -> `de` [article_preposition] The preposition 'de' is required here, not the article 'o'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No restaurante` -> `No` [article_preposition] The correct preposition to indicate location is 'No'.
- Missed: `perguntei`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a escola`

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=true answers_the_question=false

## gemma4:e2b-it-qat / few_shot_six

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
- Found (changes nothing): `a gente` -> `a gente` [word_order] The spoken Brazilian form is 'a gente', but in written form, it is usually written as 'a gente' or 'nós'. However, the prompt asks for real mistakes, and 'a gente' is acceptable in spoken context, but let's check for more concrete errors. The sentence structure is fine for A2.

### h-b2-control-reported-doctor (control, b2)

- Task: Report what the doctor said.
- Answer: O médico disse que eu precisava descansar e beber mais água.
- Flags: written_in_portuguese=true answers_the_question=true

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tem` -> `tenho` [verb_form] The verb 'ter' conjugated for 'eu' is 'tenho'.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `uma branca` -> `uma branca` [agreement] The adjective must agree with the feminine noun 'casa': uma branca.
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `o` [article_preposition] The preposition 'de' should not be followed by the personal 'o' in this context.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No restaurante` -> `No` [article_preposition] The correct preposition is 'No'.
- Missed: `perguntei`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a escola`

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=true answers_the_question=false

## gemma4:e2b-it-qat / plain

### h-a1-control-weather-cold (control, a1)

- Task: Describe the weather today.
- Answer: Hoje está frio e o céu está nublado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `está` -> `está` [ser_estar] The verb 'estar' is used correctly here, but 'está' is the correct form for the subject 'o céu' (the sky).

### h-a1-control-market-purchases (control, a1)

- Task: What did you buy at the market?
- Answer: Comprei bananas, pão e queijo no mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `pão` -> `pão` [spelling] The correct spelling for bread in Portuguese is 'pão'.

### h-a2-control-comparison-cities (control, a2)

- Task: Compare the two cities.
- Answer: Recife é mais quente do que Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `mais` -> `é` [word_choice] The comparative structure 'mais' is used for adding more, not for comparison.

### h-a2-control-a-gente-sunday (control, a2)

- Task: How do you and your friends spend Sundays?
- Answer: Aos domingos a gente almoça junto e depois assiste a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a gente` -> `nós` [word_choice] In this context, 'a gente' is informal and often used in spoken Brazilian Portuguese, but for a general answer, a more formal or standard construction is better.

### h-b2-control-reported-doctor (control, b2)

- Task: Report what the doctor said.
- Answer: O médico disse que eu precisava descansar e beber mais água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `eu` -> `você` [pronoun] The pronoun should be 'você' or implied, as the doctor is speaking to the patient.

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tem` -> `tenho` [verb_form] The correct verb for 'to be' in this context is 'ter'.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `numa` -> `uma` [article_preposition] The correct article before a feminine noun starting with a consonant is 'uma'.
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `do` [article_preposition] The preposition 'de' should not be followed by the personal 'o' in this context.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No restaurante` -> `No restaurante` [article_preposition] The correct preposition to use here is 'No' (in the context of location).
- Missed: `perguntei`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `Eu cheguei a escola` -> `Eu cheguei na escola` [article_preposition] The preposition 'a' is not needed before 'escola' in this context.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## gemma4:e2b / few_shot

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
- Found (false positive): `a gente` -> `aos domingos` [article_preposition] The correct pronoun for 'we' or 'people' in this context is 'nós' or 'a gente' is acceptable but 'a gente' is often used without the article 'a' when referring to people in general. However, the structure here is slightly awkward. The main issue is the missing article or the structure. A more natural phrasing would be 'nós'. Given the context, 'a gente' is acceptable, but the structure needs correction. The most direct error is the missing article or the phrasing. Let's focus on the common usage. The structure 'a gente almoça' is common but the lack of 'nós' or a clearer subject is a point of concern. However, the most concrete error is the missing article before 'domingos' if we consider the full sentence structure. Let's stick to the most obvious grammatical issue for A2 level. The structure is acceptable, but the lack of a definite article or a clearer subject is a weak point. Let's re-evaluate. The most common error in this context is the missing article or the structure. Let's check the verb agreement. 'almoça' is correct for 'a gente'. The main issue is the missing article before 'domingos'.

### h-b2-control-reported-doctor (control, b2)

- Task: Report what the doctor said.
- Answer: O médico disse que eu precisava descansar e beber mais água.
- Flags: written_in_portuguese=true answers_the_question=true

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Eu tem` -> `Eu tenho` [verb_form] The verb 'ter' must be conjugated correctly for the first person singular.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `numa` -> `na` [article_preposition] The correct preposition for 'live in' with a place is 'em' or 'na'.
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `de o` -> `de` [article_preposition] The structure for forgetting something is 'esqueci de' without the pronoun 'o'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No restaurante` -> `no restaurante` [article_preposition] The correct way to say 'at the restaurant' is 'no restaurante' or 'no restaurante' depending on context, but 'no restaurante' is more natural here. The main issue is the missing article/preposition.
- Missed: `perguntei`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `cheguei` -> `cheguei` [verb_form] The correct past tense conjugation for 'eu' is 'cheguei' (with an accent).
- Missed: `a escola`

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=false

## gemma4:e2b / few_shot_six

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
- Found (false positive): `a gente` -> `nos domingos` [article_preposition] The correct pronoun for 'we' or 'people' in this context is 'nós' or 'a gente' is acceptable but 'a gente' is often used in spoken Brazilian Portuguese, but the structure is slightly awkward. However, the main issue is the missing article or structure. Let's focus on the most obvious grammatical issue for A2. The structure is acceptable but 'a gente' is often used without the article 'a' when referring to a group. The main issue here is the missing article before 'domingos'.

### h-b2-control-reported-doctor (control, b2)

- Task: Report what the doctor said.
- Answer: O médico disse que eu precisava descansar e beber mais água.
- Flags: written_in_portuguese=true answers_the_question=true

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tem` -> `tenho` [verb_form] The verb 'ter' must be conjugated for the first person singular: tenho.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `numa` -> `uma` [article_preposition] The correct contraction for 'numa' is 'numa' (or 'numa' is acceptable, but the structure is slightly awkward). The main issue is the article usage here. It should be 'uma'.
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `de o` -> `de` [article_preposition] The structure for forgetting something is 'esqueci de' followed by the infinitive or noun. 'de' alone is correct here.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `perguntei` -> `perguntei` [verb_form] The verb needs to be in the past tense for 'eu': perguntei.
- Missed: `perguntei`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `cheguei` -> `cheguei` [verb_form] The correct past tense conjugation for 'eu' is 'cheguei'.
- Missed: `a escola`

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=false

## gemma4:e2b / plain

### h-a1-control-weather-cold (control, a1)

- Task: Describe the weather today.
- Answer: Hoje está frio e o céu está nublado.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `o céu está nublado` -> `o céu está nublado` [word_choice] The correct way to say the sky is cloudy is 'o céu está nublado' or 'o céu está nublado'. The structure is correct but the phrasing is slightly unnatural.

### h-a1-control-market-purchases (control, a1)

- Task: What did you buy at the market?
- Answer: Comprei bananas, pão e queijo no mercado.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `pão` -> `pão` [word_choice] The correct word for bread in this context is 'pão'.

### h-a2-control-comparison-cities (control, a2)

- Task: Compare the two cities.
- Answer: Recife é mais quente do que Curitiba.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `é` -> `está` [verb_form] The verb 'ser' is used for inherent characteristics like temperature.

### h-a2-control-a-gente-sunday (control, a2)

- Task: How do you and your friends spend Sundays?
- Answer: Aos domingos a gente almoça junto e depois assiste a um filme.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Aos` -> `No` [article_preposition] The correct preposition for this context is 'No'
- Found (false positive): `a gente` -> `nós` [pronoun] The correct way to say 'we' is 'nós' or 'a gente' is acceptable but 'nós' is more standard in this context.
- Found (false positive): `assiste` -> `assistimos` [verb_form] The verb should be in the infinitive form or conjugated correctly for the subject.

### h-b2-control-reported-doctor (control, b2)

- Task: Report what the doctor said.
- Answer: O médico disse que eu precisava descansar e beber mais água.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `eu precisava` -> `você precisa` [verb_form] The doctor said 'You must rest', which is a command/suggestion, not a past necessity.

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `Eu tem` -> `Eu tenho` [verb_form] The correct conjugation for 'eu' in the present tense is 'tenho'.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `numa` -> `uma` [article_preposition] The correct article for a feminine noun is 'uma'.
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective should agree in gender with 'casa', which is feminine.
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `Eu esqueci de o guarda-chuva em casa.` -> `Eu esqueci o guarda-chuva em casa.` [word_order] The structure for forgetting something is usually 'esquecer de' followed by the infinitive, not 'de o'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `No restaurante` -> `No restaurante` [article_preposition] The correct way to say 'at the restaurant' is 'no restaurante' or 'no restaurante' depending on context, but the structure is wrong here.
- Missed: `perguntei`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Eu cheguei` -> `Eu cheguei` [verb_form] The verb 'chegar' in the past tense requires the auxiliary verb 'ter' for the first person singular.
- Missed: `a escola`

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## gemma4:e4b-it-qat / few_shot

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
- Found (matched): `tem` -> `ten` [verb_form] The verb 'ter' must be conjugated in the first person singular for 'Eu'.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `do` [article_preposition] The preposition 'de' should contract with the article 'o' to form 'do'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `perguntei`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a escola`

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## gemma4:e4b-it-qat / few_shot_six

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
- Found (matched): `tem` -> `tenho` [verb_form] The verb 'ter' must be conjugated for 'eu': tenho.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `numa` -> `em uma` [article_preposition] The contraction of 'em' and 'uma' is 'numa'.
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `do` [article_preposition] The preposition 'de' and the article 'o' contract to 'do'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] The verb 'pedir' (to order) is more appropriate here than 'perguntar' (to ask).

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a escola`

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## gemma4:e4b-it-qat / plain

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
- Found (false positive): `eu precisava` -> `eu deveria` [verb_form] The doctor gave a direct command, so the reported speech should use the imperative or a modal verb, not the imperfect indicative.

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tem` -> `tenho` [verb_form] The verb 'ter' (to have) must be conjugated to agree with the subject 'Eu'.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `numa` -> `uma` [article_preposition] The correct contraction for 'em uma' is 'numa', but since 'casa' is feminine, the correct contraction is 'em uma' or 'uma'.
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `esqueci de o` -> `esqueci o` [article_preposition] The correct structure after the verb 'esquecer' when referring to an object requires the preposition 'de' followed by the object, not 'de o'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No restaurante` -> `No restaurante` [article_preposition] The preposition 'em' is needed before 'restaurante' when indicating location.
- Missed: `perguntei`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a escola`

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## gemma4:e4b / few_shot

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
- Found (matched): `tem` -> `tenho` [verb_form] The verb 'ter' must be conjugated in the first person singular (Eu tenho).

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective must be feminine to agree with 'casa'.
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `do` [article_preposition] The preposition 'de' contracts with the masculine definite article 'o' to form 'do'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei uma pizza` -> `pedi uma pizza` [word_choice] The verb 'perguntar' (to ask) requires the preposition 'por' or 'por algo' when referring to food, or a different verb should be used.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `a escola` -> `na escola` [article_preposition] The preposition 'em' is needed before 'escola' to indicate location.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=true answers_the_question=false

## gemma4:e4b / few_shot_six

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
- Found (matched): `tem` -> `tenho` [verb_form] The verb 'ter' must be conjugated to agree with 'eu': tenho.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective must be feminine to agree with 'casa'.
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `do` [article_preposition] The preposition 'de' and the article 'o' should contract to 'do'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei uma pizza` -> `pedi uma pizza` [word_choice] The verb 'perguntar' requires a direct object that is a question or a request, not a food item.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `a escola` -> `na escola` [article_preposition] The preposition 'a' is not needed before 'escola' in this context.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=true answers_the_question=false

## gemma4:e4b / plain

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
- Found (false positive): `eu` -> `você` [pronoun] The doctor was speaking to the patient, so the pronoun should be 'você' or 'vocês' depending on the context, but 'você' is the most direct translation for 'you'.

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tem` -> `tenho` [verb_form] The verb 'ter' (to have) must be conjugated to agree with the subject 'Eu'.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective 'branca' should be preceded by the article 'uma' which is feminine, but the noun 'casa' is feminine, so the adjective should agree in gender and number.
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `do` [article_preposition] The preposition 'de' does not combine with the masculine definite article 'o' in this way.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] The verb 'perguntar' means 'to ask (a question)', but the action here is ordering food.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `oito` -> `oito` [article_preposition] The time 'oito' needs the article 'as' when referring to the hour.
- Missed: `a escola`

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## granite4.2:3b / few_shot

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
- Missed: `Eu tem`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `de o`

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `perguntei`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a escola`

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## granite4.2:3b / few_shot_six

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
- Missed: `Eu tem`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `de o`

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `perguntei`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a escola`

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## granite4.2:3b / plain

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
- Missed: `Eu tem`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `de o`

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `perguntei`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a escola`

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## ministral-3:14b / few_shot

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
- Found (matched): `tem` -> `tenho` [verb_form] The verb must agree with the subject 'eu' (I).

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective must match the gender of 'casa' (feminine).
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `do` [article_preposition] The correct contraction for 'de + o' is 'do'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [verb_form] The verb should be 'pedi' (past tense of 'pedir' meaning 'to order').
- Found (false positive): `uma pizza e um suco` -> `uma pizza e um suco de laranja` [word_choice] The correct word for 'juice' in this context is 'suco de' + the drink (e.g., 'suco de laranja'). 'Suco' alone is too vague.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `a escola` -> `à escola` [article_preposition] The correct preposition for arriving *at* a place is 'à' (contraction of 'a' + 'a'), not 'a'.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## ministral-3:14b / few_shot_six

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
- Found (false positive): `precisava` -> `preciso` [verb_form] The doctor is giving a present directive, so the present tense 'preciso' is needed.

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tem` -> `tenho` [verb_form] The verb must agree with 'eu': tenho.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [word_choice] 'Branca' does not describe a house; it describes color.
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `do` [article_preposition] 'de' and 'o' contract to 'do'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] The verb should be 'pedi' (to order).
- Found (false positive): `uma pizza e um suco` -> `uma pizza **por** e um suco` [article_preposition] The preposition 'por' is missing before 'uma pizza' (you ordered *for* yourself).

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `a escola` -> `na escola` [article_preposition] The correct preposition for arrival is 'na' (feminine singular contraction of 'em a').

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## ministral-3:14b / plain

### h-a1-control-weather-cold (control, a1)

- Task: Describe the weather today.
- Answer: Hoje está frio e o céu está nublado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nublado` -> `encoberto` [not_brazilian] The adjective for 'cloudy' in Brazilian Portuguese is *encoberto* (or *coberto*), not *nublado* (which is used in European Portuguese).

### h-a1-control-market-purchases (control, a1)

- Task: What did you buy at the market?
- Answer: Comprei bananas, pão e queijo no mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Comprei` -> `Compro` [verb_form] The verb should be in the present tense for a general statement about purchases.

### h-a2-control-comparison-cities (control, a2)

- Task: Compare the two cities.
- Answer: Recife é mais quente do que Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true

### h-a2-control-a-gente-sunday (control, a2)

- Task: How do you and your friends spend Sundays?
- Answer: Aos domingos a gente almoça junto e depois assiste a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `assiste` -> `assistem` [verb_form] The verb should agree with the plural subject 'a gente'.

### h-b2-control-reported-doctor (control, b2)

- Task: Report what the doctor said.
- Answer: O médico disse que eu precisava descansar e beber mais água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `precisava` -> `preciso` [verb_form] The doctor gave a command, so the present tense is needed.

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tem` -> `tenho` [verb_form] The verb *ter* (to have) must agree with the subject *eu* (I) in the present tense.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective 'branca' does not match the noun 'casa' in gender.
- Missed: `branca casa`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `do` [article_preposition] The preposition *de* cannot be used with the definite article *o* in this context.
- Found (false positive): `esqueci` -> `Eu esqueci` [pronoun] The verb *esquecer* requires the preposition *de* before the object, but the subject pronoun *eu* is missing.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] The verb *pedir* (to order) is needed here, not *perguntar* (to ask).
- Found (changes nothing): `uma pizza e um suco` -> `uma pizza e um suco` [word_order] The correct word for *juice* in Brazilian Portuguese is *suco* (correct), but the article *um* should be *um* before *suco* (correct), but the mistake is in the word order—*uma pizza* is correct, but *um suco* is correct too, but the issue is the missing contraction or article before *suco* is not wrong here. However, the main mistake is the verb choice.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `a escola` -> `à escola` [article_preposition] The preposition *a* is incorrect before a school name unless it is a specific school with a proper name (e.g., *a Escola Estadual*). For general references, use *à escola* (contraction of *a + a*).

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true

## qwen3.5:4b / few_shot

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
- Found (matched): `tem` -> `tenho` [verb_form] The verb 'ter' must be conjugated as 'ter' for the first person singular subject 'eu'.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `branca` -> `uma casa branca` [word_order] The adjective 'branca' is placed after the noun instead of before it, which is incorrect for this type of description.

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de` -> `em` [article_preposition] The preposition 'em' is required before the noun 'casa'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] The verb 'perguntar' is not used with direct objects like food items; one should use 'pedir'.
- Found (false positive): `um suco` -> `o suco` [article_preposition] The structure 'pedir um [item]' is incorrect for ordering; the correct form is 'pedir o [item]'.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `escola` -> `à` [article_preposition] The word 'escola' is incorrect here; the correct term for 'school' in this context is 'colégio' or simply 'escola', but the verb 'chegar' requires a direct object without an article, and while 'escola' is acceptable, the more natural phrasing for arriving at school is 'eu cheguei à escola'. However, since 'à' is not used, it's a preposition issue. Let's re-evaluate: 'chegar' + 'a' + 'escola' -> 'cheguei à escola'. The learner wrote 'chegei a escola', missing the contraction 'à'.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=true answers_the_question=false

## qwen3.5:4b / few_shot_six

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
- Found (matched): `tem` -> `ter` [verb_form] The verb 'ter' must be conjugated as 'ter' for the first person singular subject 'eu'.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `branca` -> `uma casa branca` [word_order] The adjective 'branca' is placed after the noun instead of before it.

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `do` [article_preposition] The article 'o' must be contracted with the preposition 'de' to form 'do'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] The verb 'perguntar' is not used with a direct object like 'pizza'; one orders food, not asks it.
- Found (false positive): `um suco` -> `de um suco` [article_preposition] The verb 'pedir' requires the preposition 'de' before the object being ordered.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `escola` -> `a` [article_preposition] The verb 'chegar' requires the preposition 'a' before the destination noun.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=true answers_the_question=false

## qwen3.5:4b / plain

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
- Found (matched): `tem` -> `tenho` [verb_form] The verb 'ter' must be conjugated as 'tenho' for the first person singular.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `branca` -> `casa branca` [word_order] The adjective 'branca' is incorrectly placed before the noun and should follow it.

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `o` -> `` [article_preposition] The direct object pronoun 'o' is unnecessary and incorrect here; the verb 'esquecer' does not require an object pronoun before the noun.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] The verb 'perguntar' is not used with a direct object like 'a pizza'; one uses 'pedir' for ordering food.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `a` -> `à` [spelling] The preposition 'a' is incorrect; it should be the contraction 'à' before a vowel.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=true
