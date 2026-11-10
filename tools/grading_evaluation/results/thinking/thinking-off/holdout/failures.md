# Grading failures

Every call with a missed error, a false alarm, a quote not in the answer, a wrong flag, or an invalid response.

## cross-llama:gemma-4-12b-it-qat-q4_0:thinking-off / few_shot_six

### h-a2-control-childhood-house (control, a2)

- Task: Where did you live as a child?
- Answer: Quando eu era criança, morávamos numa casa perto do rio.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morávamos` -> `morava` [verb_form] The verb should be in the first person singular 'morava' to agree with 'eu'.

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `assisto a rádio`

### h-a2-verb-form-pôr-table (erroneous, a2)

- Task: What did you do before dinner?
- Answer: Antes do jantar eu pôs a mesa e chamei todo mundo.
- Reference: `eu pôs` -> eu pus [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu pôs`

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
