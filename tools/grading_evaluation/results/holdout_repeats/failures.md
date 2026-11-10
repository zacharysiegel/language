# Grading failures

Every call with a missed error, a false alarm, a quote not in the answer, a wrong flag, or an invalid response.

## gemma3:12b / few_shot

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective needs to agree in gender with 'casa', which is feminine.
- Missed: `branca casa`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective needs to agree in gender with 'casa', which is feminine.
- Missed: `branca casa`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective needs to agree in gender with 'casa', which is feminine.
- Missed: `branca casa`

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`

### h-a2-pronoun-reflexive-hurt (erroneous, a2)

- Task: What happened to you?
- Answer: Eu machuquei ontem jogando futebol.
- Reference: `Eu machuquei` -> Eu me machuquei [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu machuquei`

### h-a2-pronoun-reflexive-hurt (erroneous, a2)

- Task: What happened to you?
- Answer: Eu machuquei ontem jogando futebol.
- Reference: `Eu machuquei` -> Eu me machuquei [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu machuquei`

### h-a2-pronoun-reflexive-hurt (erroneous, a2)

- Task: What happened to you?
- Answer: Eu machuquei ontem jogando futebol.
- Reference: `Eu machuquei` -> Eu me machuquei [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu machuquei`

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [verb_form] 'Pedir' (to order) is the correct verb to use when ordering food.
- Found (changes nothing): `um suco` -> `um suco` [word_choice] 'Um suco' is correct, but 'um suco' is more natural.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] 'Pedir' means 'to order', not 'to ask'.
- Found (changes nothing): `um suco` -> `um suco` [word_choice] 'Um suco' is correct, but 'um suco' is more common.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [verb_form] To order food, you need to use the verb 'pedir', not 'perguntar'.
- Found (changes nothing): `um suco` -> `um suco` [article_preposition] 'Suco' is masculine, so it needs the masculine article 'um'.

## gemma3:12b / few_shot_six

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [agreement] Adjectives must agree in gender with the noun they describe; casa is feminine.
- Missed: `branca casa`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective needs to agree in gender with 'casa' (feminine).
- Missed: `branca casa`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective must agree in gender with 'casa' (feminine).
- Missed: `branca casa`

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`

### h-a2-pronoun-reflexive-hurt (erroneous, a2)

- Task: What happened to you?
- Answer: Eu machuquei ontem jogando futebol.
- Reference: `Eu machuquei` -> Eu me machuquei [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu machuquei`

### h-a2-pronoun-reflexive-hurt (erroneous, a2)

- Task: What happened to you?
- Answer: Eu machuquei ontem jogando futebol.
- Reference: `Eu machuquei` -> Eu me machuquei [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu machuquei`

### h-a2-pronoun-reflexive-hurt (erroneous, a2)

- Task: What happened to you?
- Answer: Eu machuquei ontem jogando futebol.
- Reference: `Eu machuquei` -> Eu me machuquei [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu machuquei`

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] 'pedir' (to order) is the correct verb here, not 'perguntar' (to ask).
- Found (false positive): `um suco` -> `um copo de suco` [article_preposition] 'um suco' should be 'um copo de suco' or 'um suco' if it's a whole juice.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] 'Perguntar' means 'to ask'. To order food, you need 'pedi'.
- Found (changes nothing): `um suco` -> `um suco` [article_preposition] 'Um' is used for masculine nouns, 'um suco' is incorrect, it should be 'um suco'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] To order food, you need to use the verb 'pedir', not 'perguntar'.
- Found (changes nothing): `um suco` -> `um suco` [article_preposition] 'Suco' is masculine, so it needs the article 'um'.
