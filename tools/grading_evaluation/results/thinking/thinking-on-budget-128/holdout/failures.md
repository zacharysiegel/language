# Grading failures

Every call with a missed error, a false alarm, a quote not in the answer, a wrong flag, or an invalid response.

## cross-llama:gemma-4-12b-it-qat-q4_0:thinking-on-budget-128 / few_shot_six

### h-a2-control-childhood-house (control, a2)

- Task: Where did you live as a child?
- Answer: Quando eu era criança, morávamos numa casa perto do rio.
- Reasoning (128 tokens): * Task: Where did you live as a child? * Level: A2 * Answer: "Quando eu era criança, morávamos numa casa perto do rio." * Analysis: * "Quando eu era criança" (When I was a child) - Correct. * "morávamos" (we lived) - The subject is "eu" (I), so the verb should be "morava" (I lived). "Morávamos" is the first-person plural (we). * "numa casa perto do
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morávamos` -> `morava` [verb_form] The verb must agree with the subject 'eu': morava.

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Reasoning (128 tokens): * Task: What do you do at night? * Level: A1 * Answer: "À noite eu assisto a rádio." * Analysis: * "À noite" (At night) - Correct. * "eu assisto" (I watch/listen to) - Correct. * "a rádio" (the radio) - In Portuguese, "assistir" is often used for watching TV or movies. For radio, "ouvir" is more common, but "assistir" is sometimes used in specific contexts
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `assisto a rádio`

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Reasoning (128 tokens): * Task: How do you go to work? * Level: A2 * Answer: "Eu vou para o trabalho em bicicleta." * Analysis: * "Eu vou" (I go) - Correct. * "para o trabalho" (to work) - Correct. * "em bicicleta" (by bicycle) - Correct. * Mistakes: None. * `answers_the_question`: true * `language_is_portuguese`: true * `mist
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`
