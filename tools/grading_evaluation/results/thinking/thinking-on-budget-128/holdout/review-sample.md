# Grading review sample

The same cases for every run, for rating the explanations by hand.

## cross-llama:gemma-4-12b-it-qat-q4_0:thinking-on-budget-128 / few_shot_six

### h-a1-control-weather-cold (control, a1)

- Task: Describe the weather today.
- Answer: Hoje está frio e o céu está nublado.
- Reasoning (123 tokens): The user wants me to check a Portuguese answer for mistakes based on a specific task and scene facts. Task: Describe the weather today. Scene facts: It is cold and cloudy. Level: A1 Answer: Hoje está frio e o céu está nublado. Analysis: - "Hoje está frio" (Today it is cold) - Correct. - "e o céu está nublado" (and the sky is cloudy) - Correct. - The answer matches the scene facts. - No spelling, grammar, or usage errors found. Result: No mistakes.
- Flags: written_in_portuguese=true answers_the_question=true

### h-a1-control-market-purchases (control, a1)

- Task: What did you buy at the market?
- Answer: Comprei bananas, pão e queijo no mercado.
- Reasoning (128 tokens): The user wants me to check a Portuguese answer for mistakes based on a specific task and scene facts. Task: What did you buy at the market? Scene facts: You bought bananas, bread and cheese. Level: A1 Answer: Comprei bananas, pão e queijo no mercado. Analysis: - "Comprei" (I bought) - Correct. - "bananas, pão e queijo" (bananas, bread and cheese) - Correct. - "no mercado" (at the market) - Correct. The answer is correct and matches the scene facts
- Flags: written_in_portuguese=true answers_the_question=true

### h-a2-control-comparison-cities (control, a2)

- Task: Compare the two cities.
- Answer: Recife é mais quente do que Curitiba.
- Reasoning (128 tokens): The user wants me to check a Portuguese answer for mistakes based on a specific task and scene facts. Task: Compare the two cities. Scene facts: Recife is hotter than Curitiba. Level: A2 Answer: Recife é mais quente do que Curitiba. Analysis: - "Recife é mais quente do que Curitiba." - "Recife" (Subject) - "é" (Verb - ser, correct for characteristics) - "mais quente" (Comparative - correct) - "do que" (Comparison - correct) - "Curitiba" (Object of comparison
- Flags: written_in_portuguese=true answers_the_question=true

### h-a2-control-a-gente-sunday (control, a2)

- Task: How do you and your friends spend Sundays?
- Answer: Aos domingos a gente almoça junto e depois assiste a um filme.
- Reasoning (128 tokens): The user wants me to check a Portuguese answer for mistakes based on specific rules. Task: How do you and your friends spend Sundays? Level: A2 Answer: Aos domingos a gente almoça junto e depois assiste a um filme. Analysis: - "Aos domingos": Correct (On Sundays). - "a gente almoça": Correct (we eat lunch - "a gente" is a common Brazilian form for "nós"). - "junto": Correct (together). - "e depois": Correct (and then). - "assiste a um filme":
- Flags: written_in_portuguese=true answers_the_question=true

### h-b2-control-reported-doctor (control, b2)

- Task: Report what the doctor said.
- Answer: O médico disse que eu precisava descansar e beber mais água.
- Reasoning (128 tokens): * Task: Report what the doctor said. * Scene facts: The doctor said: 'You must rest and drink more water.' * Level: B2. * Answer: "O médico disse que eu precisava descansar e beber mais água." * "O médico disse" (The doctor said) - Correct. * "que eu precisava" (that I needed) - The doctor said "You must" (present/imperative). In reported speech, "precisava" (imperfect) is often used, but "preciso"
- Flags: written_in_portuguese=true answers_the_question=true

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Reasoning (128 tokens): * Task: "How old are you?" * Level: A1 * Answer: "Eu tem vinte e cinco anos." * Language: Portuguese (pt-BR) * "Eu" (Subject: I) * "tem" (Verb: have - 3rd person singular) * "vinte e cinco anos" (Age: 25 years) * The verb "ter" (to have) should be conjugated for the first person singular "Eu". * "Eu tenho" is
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tem` -> `tenho` [verb_form] The verb must agree with 'eu': tenho.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Reasoning (128 tokens): * Task: Describe your house. * Level: A1. * Answer: "Eu moro numa branca casa perto da praia." * Analysis: * "Eu moro" (I live) - Correct. * "numa branca casa" - In Portuguese, adjectives usually follow the noun. "casa branca" is the standard order. "numa branca casa" sounds like a poetic inversion or a direct translation from English "a white house". * "perto da praia" (near the beach) - Correct.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `branca casa` -> `casa branca` [word_order] In Portuguese, adjectives usually come after the noun.

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Reasoning (128 tokens): * Task: Why are you wet? * Scene facts: (Implicitly, the user is wet because they forgot an umbrella, based on the answer). * Level: A2. * Answer: "Eu esqueci de o guarda-chuva em casa." * "Eu esqueci": Correct (Past tense of esquecer). * "de o": This is a contraction error. In Portuguese, "de" + "o" becomes "do". * "guarda-chuva": Correct. * "em
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `do` [article_preposition] The preposition 'de' and the article 'o' must contract to 'do'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Reasoning (128 tokens): * Task: What did you do at the restaurant? * Level: A2 * Answer: "No restaurante eu perguntei uma pizza e um suco." * Context: The user is describing an action at a restaurant. * "No restaurante" (At the restaurant) - Correct. * "eu perguntei" (I asked) - This is the error. In Portuguese, "perguntar" means "to ask a question". To "order" food, the verb is "pedir". * "uma pizza e
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] The verb 'perguntar' means 'to ask a question'; 'pedir' should be used for 'to order'.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Reasoning (128 tokens): * Task: What did you do at eight? * Level: B2 * Answer: "Eu cheguei a escola às oito." * Scene facts: None provided. * "Eu cheguei" (I arrived) - Correct. * "a escola" (to the school) - In Portuguese, the verb "chegar" (to arrive) requires the preposition "a". When followed by a definite article "a" (the school), it should contract to "à". * "às oito" (at eight
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `a escola` -> `à escola` [article_preposition] The verb 'chegar' requires the preposition 'a', which contracts with the article 'a' to form 'à'.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Reasoning (128 tokens): The user wants me to check a short written answer from an English-speaking learner of Brazilian Portuguese. Task: Where do you live? Level: A1 Answer: I live in a small apartment near the beach. Analysis: - The answer is in English. - The instructions state: `language_is_portuguese` is false when the answer is written in another language. - The instructions state: `answers_the_question` is true when the answer is about what the task asked, even if it contains mistakes. - Since the answer is in English, `language_is
- Flags: written_in_portuguese=false answers_the_question=true
