# Grading review sample

The same cases for every run, for rating the explanations by hand.

## cross-llama:gemma-4-12b-it-qat-q4_0:thinking-on-budget-128 / few_shot_six

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Reasoning (128 tokens): The user wants me to check a Portuguese answer for mistakes based on a specific task and scene facts. Task: Describe the clothes Marina is wearing. Scene facts: Marina is at the beach. She wears a red swimsuit, a straw hat and white sandals. Level: A1 Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas. Analysis: - "A Marina": In Portuguese, using the definite article before a person's name is common in Brazil, but often omitted in formal contexts. However, it's not a mistake. - "
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Reasoning (128 tokens): * Task: Where do you work? * Level: A1 * Answer: "Eu trabalho em uma escritório no centro." * Scene facts: None provided. * "Eu trabalho" (I work) - Correct. * "em uma escritório" (in a office) - "Escritório" is a masculine noun. "Uma" is feminine. It should be "um escritório". * "no centro" (in the center) - Correct. * Error: "uma" * Explanation: "
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma` -> `um` [agreement] The word 'escritório' is masculine, so the article should be 'um'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Reasoning (128 tokens): * Task: What do you eat for lunch? * Level: A1 * Answer: "No almoço eu como arroz, beans e frango." * Language: Portuguese (pt-BR). * "No almoço": Correct (In lunch). * "eu como": Correct (I eat). * "arroz": Correct (rice). * "beans": Incorrect. This is English. The Portuguese word is "feijão". * "e frango": Correct (and chicken).
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word for 'beans' in Portuguese is 'feijão'.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Reasoning (128 tokens): The user wants me to check a Portuguese answer for mistakes based on specific rules. Task: What did you do on the weekend? Level: A2 Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa. Analysis: 1. "No fim de semana" - Correct. 2. "eu visitei meus avós" - Correct. 3. "e nós almoçamos juntos" - Correct. 4. "Depois eu volti para casa" - "volti" is incorrect. The past tense of "vol
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The past tense of 'voltar' for the first person singular is 'voltei'.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Reasoning (128 tokens): * Task: How do you go to work? * Level: A2 * Answer: "Eu vou para o trabalho de autocarro todos os dias." * Context: Brazilian Portuguese (pt-BR). * "Eu vou para o trabalho" - Correct. * "de autocarro" - "Autocarro" is the word for "bus" in European Portuguese (Portugal). In Brazilian Portuguese, the word is "ônibus". * "todos os dias" - Correct. * The user is learning Brazilian
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] In Brazil, the word for bus is 'ônibus'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Reasoning (128 tokens): The user wants me to evaluate a Portuguese answer based on a specific task and scene facts. Task: Describe the meal these people are eating. Scene facts: Two people at a restaurant eat spaghetti and drink red wine. Level: A2 Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade. Analysis: 1. `answers_the_question`: The task asks to describe the meal. The answer talks about living in São Paulo. This is a content mismatch. 2. `language_is_portuguese`: True. 3. Mistakes: - The
- Flags: written_in_portuguese=true answers_the_question=false

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reasoning (128 tokens): * Task: What would you do if you won the lottery? * Level: B1 * Answer: "Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo." * Analysis: * "Se eu ganhasse" (If I won) - Correct use of Imperfeito do Subjuntivo. * "na loteria" - Correct prepositional contraction (em + a). * "eu compraria" (I would buy) - Correct use of Futuro do Pretérito.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Reasoning (128 tokens): * Task: Tell what you said to your brother. * Level: B2 (Upper Intermediate). * Answer: "Eu lhe disse que não me esperasse, mas ele não me ouviu." * Analysis: * "Eu lhe disse": Correct. "lhe" is the indirect object pronoun for "to him". * "que não me esperasse": Correct. Subjunctive mood (imperfeito do subjuntivo) is used correctly here after "disse que não...". * "mas ele não me ouviu": Correct.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Reasoning (128 tokens): The user wants me to check a Portuguese answer for mistakes based on specific rules. Task: What happened on your test? Level: A2 Answer: Ontem eu fiz um erro na prova de matemática. Analysis: - "Ontem" (Yesterday) - Correct. - "eu fiz" (I did/made) - Correct conjugation of "fazer" in the pretérito perfeito for "eu". - "um erro" (a mistake) - Correct. - "na prova" (on the test) - Correct contraction of "em" + "a". - "
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Reasoning (128 tokens): * Task: Do you have siblings? * Level: A1 * Answer: "Sim, tenho uma irmã." * Language: Portuguese (pt-BR) * Analysis: * "Sim" (Yes) - Correct. * "tenho" (I have) - Correct conjugation of "ter" for "eu". * "uma irmã" (a sister) - Correct gender and number agreement. * Mistakes: None. * `answers_the_question`: true.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Reasoning (128 tokens): * Task: How is your brother? * Level: A2 * Answer: "Meu irmão está doente e não vai trabalhar hoje." * Analysis: * "Meu irmão" (Subject) - Correct. * "está" (Verb ser/estar) - Correct (temporary state of being sick). * "doente" (Adjective) - Correct. * "e" (Conjunction) - Correct. * "não vai trabalhar" (Verb phrase) - Correct (future/present intention
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Reasoning (128 tokens): * Task: Did you invite Ana to the party? * Level: B2 (Upper Intermediate) * Answer: "Sim, falei com a Ana e convidei-a para a festa." * Language: Portuguese (pt-BR) * "Sim" (Yes) - Correct. * "falei com a Ana" (I spoke with Ana) - Correct. * "e convidei-a" (and invited her) - Correct. In Brazilian Portuguese, "convidei-a" is grammatically correct
- Flags: written_in_portuguese=true answers_the_question=true
