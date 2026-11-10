# Grading review sample

The same cases for every run, for rating the explanations by hand.

## cross-llama:gemma-4-12b-it-qat-q4_0:thinking-on-budget-512 / few_shot_six

### h-a1-control-weather-cold (control, a1)

- Task: Describe the weather today.
- Answer: Hoje está frio e o céu está nublado.
- Reasoning (123 tokens): The user wants me to check a Portuguese answer for mistakes based on a specific task and scene facts. Task: Describe the weather today. Scene facts: It is cold and cloudy. Level: A1 Answer: Hoje está frio e o céu está nublado. Analysis: - "Hoje está frio" (Today it is cold) - Correct. - "e o céu está nublado" (and the sky is cloudy) - Correct. - The answer matches the scene facts. - No spelling, grammar, or usage errors found. Result: No mistakes.
- Flags: written_in_portuguese=true answers_the_question=true

### h-a1-control-market-purchases (control, a1)

- Task: What did you buy at the market?
- Answer: Comprei bananas, pão e queijo no mercado.
- Reasoning (157 tokens): The user wants me to check a Portuguese answer for mistakes based on a specific task and scene facts. Task: What did you buy at the market? Scene facts: You bought bananas, bread and cheese. Level: A1 Answer: Comprei bananas, pão e queijo no mercado. Analysis: - "Comprei" (I bought) - Correct. - "bananas, pão e queijo" (bananas, bread and cheese) - Correct. - "no mercado" (at the market) - Correct. The answer is correct and matches the scene facts. Result: - answers_the_question: true - language_is_portuguese: true - mistakes: []
- Flags: written_in_portuguese=true answers_the_question=true

### h-a2-control-comparison-cities (control, a2)

- Task: Compare the two cities.
- Answer: Recife é mais quente do que Curitiba.
- Reasoning (176 tokens): The user wants me to check a Portuguese answer for mistakes based on a specific task and scene facts. Task: Compare the two cities. Scene facts: Recife is hotter than Curitiba. Level: A2 Answer: Recife é mais quente do que Curitiba. Analysis: - "Recife é mais quente do que Curitiba." - "Recife" (Subject) - "é" (Verb - ser, correct for characteristics) - "mais quente" (Comparative - correct) - "do que" (Comparison - correct) - "Curitiba" (Object of comparison) The sentence is grammatically correct and matches the scene facts. Mistakes: None. Output format: {"answers_the_question":true,"language_is_portuguese":true,"mistakes":[]}
- Flags: written_in_portuguese=true answers_the_question=true

### h-a2-control-a-gente-sunday (control, a2)

- Task: How do you and your friends spend Sundays?
- Answer: Aos domingos a gente almoça junto e depois assiste a um filme.
- Reasoning (186 tokens): The user wants me to check a Portuguese answer for mistakes based on specific rules. Task: How do you and your friends spend Sundays? Level: A2 Answer: Aos domingos a gente almoça junto e depois assiste a um filme. Analysis: - "Aos domingos": Correct (On Sundays). - "a gente almoça": Correct (we eat lunch - "a gente" is a common Brazilian form for "nós"). - "junto": Correct (together). - "e depois": Correct (and then). - "assiste a um filme": Correct (watches a movie). The answer is grammatically correct and follows Brazilian Portuguese conventions (using "a gente"). Mistakes: None. Output format: {"answers_the_question":true,"language_is_portuguese":true,"mistakes":[]}
- Flags: written_in_portuguese=true answers_the_question=true

### h-b2-control-reported-doctor (control, b2)

- Task: Report what the doctor said.
- Answer: O médico disse que eu precisava descansar e beber mais água.
- Reasoning (442 tokens): * Task: Report what the doctor said. * Scene facts: The doctor said: 'You must rest and drink more water.' * Level: B2. * Answer: "O médico disse que eu precisava descansar e beber mais água." * "O médico disse" (The doctor said) - Correct. * "que eu precisava" (that I needed) - The doctor said "You must" (present/imperative). In reported speech, "precisava" (imperfect) is often used, but "preciso" (present) or "deveria" (conditional) might be more direct depending on the context. However, "precisava" is a standard way to report a past command/instruction in Portuguese. * "descansar e beber mais água" (rest and drink more water) - Correct. * The doctor said "You must rest". * The learner wrote "O médico disse que eu precisava descansar". * Is "precisava" a mistake? In reported speech, if the reporting verb is in the past ("disse"), the subordinate clause often shifts back. "Preciso" would be "The doctor said that I need...". "Precisava" is "The doctor said that I needed...". * Wait, "must" is a modal of obligation. "Precisa" or "deve". * Actually, "precisava" is perfectly acceptable for reporting a past statement. * Let's check for any other errors. * "O médico disse que eu precisava descansar e beber mais água." * Grammar: Correct. * Spelling: Correct. * Agreement: Correct. * Content: Matches the facts. * The answer is correct. * `answers_the_question`: true * `language_is_portuguese`: true * `mistakes`: []
- Flags: written_in_portuguese=true answers_the_question=true

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Reasoning (243 tokens): * Task: "How old are you?" * Level: A1 * Answer: "Eu tem vinte e cinco anos." * Language: Portuguese (pt-BR) * "Eu" (Subject: I) * "tem" (Verb: have - 3rd person singular) * "vinte e cinco anos" (Age: 25 years) * The verb "ter" (to have) should be conjugated for the first person singular "Eu". * "Eu tenho" is the correct form. * "Eu tem" is incorrect (3rd person singular). * error_text: "tem" * explanation: The verb must agree with the subject 'eu': tenho. * kind: verb_form * replacement: "tenho" * answers_the_question: true (The user is answering the question about age). * language_is_portuguese: true. * mistakes: list containing the error.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tem` -> `tenho` [verb_form] The verb must agree with the subject 'eu': tenho.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Reasoning (512 tokens): * Task: Describe your house. * Level: A1. * Answer: "Eu moro numa branca casa perto da praia." * Analysis: * "Eu moro" (I live) - Correct. * "numa branca casa" - In Portuguese, adjectives usually follow the noun. "casa branca" is the standard order. "numa branca casa" sounds like a poetic inversion or a direct translation from English "a white house". * "perto da praia" (near the beach) - Correct. * "branca casa" is a word order issue. * Error text: "branca casa" * Explanation: In Portuguese, adjectives usually come after the noun. * Kind: word_order * Replacement: "casa branca" * Wait, the prompt says "Report only real mistakes". Is "branca casa" a mistake? Yes, for an A1 learner, it's a word order error. * Let's check the rules: "Do not report style, correct but unusual wording". "Branca casa" is definitely unusual/incorrect for standard Brazilian Portuguese. * Error: "branca casa" * Explanation: Adjectives usually follow the noun in Portuguese. * Kind: word_order * Replacement: "casa branca" * Wait, the user's answer is "Eu moro numa branca casa". * The error is "branca casa". * Let's double check the "word_order" definition: "words in an order a Portuguese speaker would not use". * "branca casa" is not used in standard speech. * Wait, should I just report "branca"? No, the phrase "branca casa" is the issue. * Actually, "numa branca casa" -> "numa casa branca". * Let's refine the error text. * error_text: "branca casa" * replacement: "casa branca" * Wait, is "numa" correct? Yes, "em + uma". * Is "perto da praia" correct? Yes. * One more check: "
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `branca casa` -> `casa branca` [word_order] In Portuguese, adjectives usually come after the noun.

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Reasoning (512 tokens): * Task: Why are you wet? * Scene facts: (Implicitly, the user is wet because they forgot an umbrella, based on the answer). * Level: A2. * Answer: "Eu esqueci de o guarda-chuva em casa." * "Eu esqueci": Correct (Past tense of esquecer). * "de o": This is a contraction error. In Portuguese, "de" + "o" becomes "do". * "guarda-chuva": Correct. * "em casa": Correct. * Error: "de o" * Explanation: "de" and "o" must contract to "do". * Kind: article_preposition. * Replacement: "do". * "Eu esqueci de o guarda-chuva em casa." * Wait, "esquecer" can be used with "de" or without "de" (esquecer algo vs. esquecer-se de algo). * If the user says "Eu esqueci o guarda-chuva", it's correct. * If the user says "Eu me esqueci do guarda-chuva", it's correct. * "Eu esqueci de o" is definitely wrong because of the lack of contraction. * However, "esqueci de" is often used colloquially, but "de o" is never correct. * error_text: "de o" * explanation: "The preposition 'de' and the article 'o' must contract to 'do'." * kind: "article_preposition" * replacement: "do" * Wait, is "esqueci de" correct? * Grammatically, "esquecer" (transitive direct) doesn't take "de". "Esquecer-se" (pronominal) takes "de". * So "Eu esqueci o guarda-chuva" is correct. * "Eu esqueci de o" is wrong for two reasons: contraction and the preposition itself (unless using the pronominal form). * Since the user wrote "de o", the most immediate grammatical error is the contraction.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `do` [article_preposition] The preposition 'de' and the article 'o' must contract to 'do'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Reasoning (305 tokens): * Task: What did you do at the restaurant? * Level: A2 * Answer: "No restaurante eu perguntei uma pizza e um suco." * Context: The user is describing an action at a restaurant. * "No restaurante" (At the restaurant) - Correct. * "eu perguntei" (I asked) - This is the error. In Portuguese, "perguntar" means "to ask a question". To "order" food, the verb is "pedir". * "uma pizza e um suco" (a pizza and a juice) - Correct. * Error: "perguntei" * Explanation: "Perguntar" means "to ask a question"; "pedir" is used for ordering food. * Kind: word_choice * Replacement: "pedi" (since the sentence is in the past tense "perguntei"). * "perguntei" is the wrong word for "ordered". * "pedi" is the correct past tense of "pedir". * error_text: perguntei * explanation: The verb 'perguntar' means 'to ask a question'; 'pedir' is used for ordering food. * kind: word_choice * replacement: pedi
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] The verb 'perguntar' means 'to ask a question'; 'pedir' is used for ordering food.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Reasoning (512 tokens): * Task: What did you do at eight? * Level: B2 * Answer: "Eu cheguei a escola às oito." * Scene facts: None provided. * "Eu cheguei" (I arrived) - Correct. * "a escola" (to the school) - In Portuguese, the verb "chegar" (to arrive) requires the preposition "a". When followed by a definite article "a" (the school), it should contract to "à". * "às oito" (at eight) - Correct (time expression). * "a escola" should be "à escola". * Kind: `article_preposition` (missing contraction). * error_text: "a escola" * explanation: "The preposition 'a' and the article 'a' must contract to 'à' after the verb 'chegar'." * kind: "article_preposition" * replacement: "à escola" * Wait, is "a escola" acceptable in some contexts? No, "chegar a" is the standard. "Chegar em" is common in spoken Brazilian Portuguese, but "a escola" without the accent is grammatically incorrect for "chegar a". * Actually, "chegar a" is the standard. "Chegar em" is very common in Brazil. If the user wrote "a escola", they are trying to use "chegar a". * Let's check the prompt rules: "Do not report style, correct but unusual wording...". * "Chegar a" vs "Chegar em": "Chegar em" is common in Brazil. "Chegar a" is the standard. * If the user wrote "a escola", they are using the "a" preposition. It needs the crase (accent). * error_text: "a escola" * explanation: "The preposition 'a' and the article 'a' should contract to 'à' after the verb 'chegar'." * kind: "article_preposition" * replacement: "à escola" * Wait, let me double check if "a escola" is ever correct. No, "chegar a" + "a
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `a escola` -> `à escola` [article_preposition] The preposition 'a' and the article 'a' must contract to 'à' after the verb 'chegar'.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Reasoning (512 tokens): The user wants me to check a short written answer from an English-speaking learner of Brazilian Portuguese. Task: Where do you live? Level: A1 Answer: I live in a small apartment near the beach. Analysis: - The answer is in English. - The instructions state: `language_is_portuguese` is false when the answer is written in another language. - The instructions state: `answers_the_question` is true when the answer is about what the task asked, even if it contains mistakes. - Since the answer is in English, `language_is_portuguese` should be `false`. - The answer does answer the question "Where do you live?", so `answers_the_question` should be `true`. - There are no Portuguese mistakes to report because the text is not in Portuguese. Wait, looking at the previous examples: Example 1: Portuguese answer, mistakes reported. Example 2: Portuguese answer, no mistakes. Example 3: Portuguese answer, mistakes reported. Example 4: Portuguese answer, no mistakes. Example 5: Portuguese answer, mistakes reported. Example 6: Portuguese answer, `answers_the_question` is false (because the content is irrelevant), mistakes reported. In the current case: Answer: "I live in a small apartment near the beach." Language: English. `language_is_portuguese`: false. `answers_the_question`: true. Mistakes: None (since it's not Portuguese). Wait, if the user is supposed to be learning Portuguese, and they answer in English, is that a mistake? The prompt says "Report only real mistakes in the answer." Usually, if the task is to answer in Portuguese and they answer in English, it's a language mismatch, but the specific instruction says `language_is_portuguese` is false when written in another language. Let's check the JSON structure. `{"answers_the_question":true,"language_is_portuguese":false,"mistakes":[]}` Wait, I should check if there are any specific instructions about answering in the wrong language. "language_is_portuguese is false when the answer is written in another language." "answers_the_question is true when the answer is about what the task asked, even if it contains mistakes
- Flags: written_in_portuguese=false answers_the_question=true
