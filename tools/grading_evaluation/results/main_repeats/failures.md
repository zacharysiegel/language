# Grading failures

Every call with a missed error, a false alarm, a quote not in the answer, a wrong flag, or an invalid response.

## gemma3:12b / few_shot

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fejão`

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fejão`

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fejão`

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `manha` -> `manhã` [spelling] The word needs an accent mark: manhã.
- Missed: `cafe`

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `manha` -> `manhã` [spelling] The correct word is 'manhã', with an accent.
- Missed: `cafe`

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `manha` -> `manhã` [spelling] The correct word is 'manhã', with an accent mark.
- Missed: `cafe`

### a1-agreement-trousers (erroneous, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: Marina veste uma camiseta azul e uma calça pretos.
- Reference: `pretos` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pretos`

### a1-agreement-trousers (erroneous, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: Marina veste uma camiseta azul e uma calça pretos.
- Reference: `pretos` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pretos`

### a1-agreement-trousers (erroneous, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: Marina veste uma camiseta azul e uma calça pretos.
- Reference: `pretos` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pretos`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [spelling] The word needs an accent mark: escritório.
- Missed: `uma escritório`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [spelling] The word needs an accent mark: escritório.
- Missed: `uma escritório`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chamo`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chamo`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chamo`

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chama`

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chama`

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chama`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `vermelha` -> `vermelhada` [agreement] Adjectives describing clothes need to agree in gender with the noun.
- Found (false positive): `uma calça` -> `calça` [article_preposition] When describing clothes, 'uma' is not used.

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `vermelha` -> `vermelhada` [agreement] Adjectives must agree in gender with the noun they describe; camisa is feminine.
- Found (false positive): `está com uma` -> `está usando` [word_choice] A more natural way to describe what someone is wearing is 'está usando'.

### a1-english-lunch (not_portuguese, a1)

- Task: What did you eat for lunch?
- Answer: I ate rice and chicken for lunch.
- Flags: written_in_portuguese=false answers_the_question=false

### a1-english-lunch (not_portuguese, a1)

- Task: What did you eat for lunch?
- Answer: I ate rice and chicken for lunch.
- Flags: written_in_portuguese=false answers_the_question=false

### a1-english-lunch (not_portuguese, a1)

- Task: What did you eat for lunch?
- Answer: I ate rice and chicken for lunch.
- Flags: written_in_portuguese=false answers_the_question=false

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `assisti` [word_choice] The verb 'assistir' (to watch) is more appropriate here than 'atender' (to attend to).
- Found (changes nothing): `um filme novo` -> `um filme novo` [article_preposition] The correct phrasing is 'um filme novo' or 'um filme'.

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `está chovendo` [content_mismatch] It is not sunny, it is raining.
- Found (duplicate): `calor` -> `frio` [content_mismatch] It is not hot, it is cold.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `está chovendo` [content_mismatch] It is not sunny, it is raining.
- Found (duplicate): `muito calor` -> `muito frio` [content_mismatch] It is not hot, it is cold.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `está chovendo` [content_mismatch] It is not sunny, it is raining.
- Found (duplicate): `muito calor` -> `muito frio` [content_mismatch] It is not hot, it is cold.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> ` ` [article_preposition] 'Dele' is not needed after 'gostei'.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> `muito` [word_choice] 'Dele' is not usually used after 'gostar'.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> ` ` [article_preposition] 'Dele' is not needed after 'gostei'.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tenho visitado` -> `tenho visitado` [verb_form] The correct tense is present perfect, which requires 'tenho visitado'.
- Missed: `tenho visitado`

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tenho visitado` -> `tenho visitado` [verb_form] The verb tense should be 'tenho visitado' to match the present perfect tense used in the first part of the sentence.
- Missed: `tenho visitado`

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tenho visitado` -> `tenho visitado` [verb_form] The correct tense is present perfect, which requires 'tenho visitado'.
- Missed: `tenho visitado`

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] After 'recomendo que', the verb should be in the subjunctive mood.
- Missed: `assiste`
- Missed: `exige`

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] After 'recomendo que', the verb should be in the subjunctive.
- Missed: `assiste`
- Missed: `exige`

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] After 'recomendo que', the verb should be in the subjunctive mood.
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `no dia 10` [article_preposition] In formal writing, it's better to use the full date.
- Missed: `sobre do`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `no dia dez do` [article_preposition] When referring to a specific date, 'no' is followed by 'do' + the number.
- Missed: `sobre do`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `dia dez` [article_preposition] 'No' is redundant here; 'dia dez' is sufficient.
- Missed: `sobre do`

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu diverti`

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu diverti`

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu diverti`

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `é doente`

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `é doente`

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `é doente`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sempre fui`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sempre fui`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sempre fui`

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `em casa` -> `a casa` [article_preposition] The correct preposition to use here is 'a'.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `em casa` -> `a casa` [article_preposition] The correct preposition is 'a'.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `em casa` -> `a casa` [article_preposition] The correct preposition to use here is 'a'.

## gemma3:12b / few_shot_six

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fejão`

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fejão`

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fejão`

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha` -> `De manhã` [spelling] 'De manhã' needs an accent mark.
- Missed: `cafe`

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha` -> `De manhã` [spelling] The correct expression is 'De manhã'.
- Missed: `cafe`

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha` -> `De manhã` [spelling] The correct expression is 'De manhã'.
- Missed: `cafe`

### a1-agreement-trousers (erroneous, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: Marina veste uma camiseta azul e uma calça pretos.
- Reference: `pretos` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pretos`

### a1-agreement-trousers (erroneous, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: Marina veste uma camiseta azul e uma calça pretos.
- Reference: `pretos` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pretos`

### a1-agreement-trousers (erroneous, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: Marina veste uma camiseta azul e uma calça pretos.
- Reference: `pretos` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pretos`

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `moro` -> `moro` [verb_form] The verb needs to be in the present tense to match 'sou'.
- Missed: `moro Toronto`

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `moro` -> `moro` [verb_form] The verb must agree with 'Eu': moro.
- Missed: `moro Toronto`

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `moro` -> `moro` [verb_form] The verb needs to be in the present tense to match 'sou'.
- Missed: `moro Toronto`

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `jantar` -> `janto` [verb_form] The verb needs to be conjugated in the first person singular.
- Found (false positive): `às oito` -> `às oito horas` [article_preposition] The correct expression is 'às oito horas', but 'às oito' is also acceptable.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chamo`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chamo`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chamo`

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chama`

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chama`

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chama`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `vermelha` -> `vermelhada` [agreement] Adjectives must agree in gender with the noun they describe; camisa is feminine.
- Found (false positive): `uma calça` -> `calça` [article_preposition] When describing clothes, 'uma' is not used.

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `vermelha` -> `vermelhada` [agreement] Adjectives must agree in gender with the noun they describe; camisa is feminine.
- Found (false positive): `uma calça` -> `calça` [article_preposition] When describing clothes, it's more natural to omit the article.

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `assisti` [word_choice] The verb 'assistir' is needed to mean 'to watch'.
- Found (changes nothing): `um filme novo` -> `um filme novo` [word_order] The correct order is 'um filme novo'.

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `está chovendo` [content_mismatch] The scene facts state it is raining, not sunny.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `está chovendo` [content_mismatch] The scene facts state it is raining, not sunny.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `está chovendo` [content_mismatch] The scene facts state it is raining, not sunny.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

### b1-verb-form-subjunctive-present (erroneous, b1)

- Task: What do you hope for the future?
- Answer: Espero que meus filhos são felizes e que eles tenham uma boa educação.
- Reference: `são` -> sejam [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `são` -> `sejam` [verb_form] After 'esperar que', the subjunctive mood is needed: sejam.
- Found (changes nothing): `tenham` -> `tenham` [verb_form] After 'esperar que', the subjunctive mood is needed: tenham.

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tenho visitado` -> `tenho visitado` [verb_form] The correct verb form is 'tenho visitado' to match the present perfect tense.
- Missed: `tenho visitado`

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] After 'recomendo que', the verb should be in the subjunctive: estude.
- Missed: `assiste`
- Missed: `exige`

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] After 'recomendo que', the verb should be in the subjunctive: estude.
- Missed: `assiste`
- Missed: `exige`

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] After 'recomendo que', the verb should be in the subjunctive: estude.
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sobre do`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sobre do`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sobre do`

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu diverti`

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu diverti`

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu diverti`

### a1-short-agreement-books (erroneous, a1)

- Task: What is in your bag?
- Answer: Na minha bolsa tem um livros.
- Reference: `um livros` -> um livro | uns livros [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tem` -> `há` [verb_form] After 'há', the verb must be in the infinitive form.
- Found (matched): `um livros` -> `um livro` [agreement] The indefinite article must agree in number and gender with the noun.

### a1-short-agreement-books (erroneous, a1)

- Task: What is in your bag?
- Answer: Na minha bolsa tem um livros.
- Reference: `um livros` -> um livro | uns livros [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tem` -> `há` [agreement] After 'há' or 'tem', the following word should be in the plural.
- Found (matched): `um livros` -> `uns livros` [agreement] The word needs to be plural.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a2-article-contraction-de-o (erroneous, a2)

- Task: Whose is the car?
- Answer: O carro é de o meu pai.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `de` [article_preposition] The article 'o' is incorrect after 'de'; it should be omitted.
- Found (false positive): `meu` -> `dele` [agreement] After 'de', the possessive adjective should be in the masculine singular form.

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `é doente`

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `é doente`

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `é doente`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sempre fui`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sempre fui`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sempre fui`

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tem` -> `há` [verb_form] After 'na minha bolsa', you need 'há', not 'tem'.

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tem` -> `há` [verb_form] After 'ter', use the infinitive form.

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tem` -> `há` [verb_form] After 'ter', use the infinitive form.
