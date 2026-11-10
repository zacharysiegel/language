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
- Found (false positive): `escritório` -> `escritório` [spelling] The word needs an accent mark: escritório.
- Missed: `uma escritório`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `escritório` -> `escritório` [spelling] The word needs an accent mark: escritório.
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
- Found (false positive): `um filme novo` -> `um filme novo` [article_preposition] The correct phrasing is 'um filme novo' or 'um filme'.

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
- Found (false positive): `tenho visitado` -> `tenho visitado` [verb_form] The correct tense is present perfect, which requires 'tenho visitado'.
- Missed: `tenho visitado`

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tenho visitado` -> `tenho visitado` [verb_form] The verb tense should be 'tenho visitado' to match the present perfect tense used in the first part of the sentence.
- Missed: `tenho visitado`

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tenho visitado` -> `tenho visitado` [verb_form] The correct tense is present perfect, which requires 'tenho visitado'.
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

## qwen3.5:latest / plain

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cansado` -> `cansa` [agreement] The adjective 'cansado' is masculine and does not agree with the implied feminine subject 'eu' (I), which should be 'cansa'.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Response: invalid or missing

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cansado` -> `cansada` [agreement] The adjective must agree in gender with the implied speaker 'eu' (male), so it should be 'cansada'.

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
- Found (false positive): `De` -> `Em` [article_preposition] The preposition 'em' is required before the time expression 'manhã'.
- Found (matched): `cafe` -> `café` [spelling] Nouns in Portuguese must be capitalized when referring to specific items or at the start of a sentence, and 'café' requires an accent mark.
- Missed: `manha`

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `De` -> `Da` [article_preposition] The preposition 'em' is required before the time of day in Brazilian Portuguese.
- Found (matched): `cafe` -> `café` [spelling] Nouns in Brazilian Portuguese must have an accent mark on the 'e' when it is stressed.
- Missed: `manha`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `um` -> `um` [agreement] The masculine noun 'escritório' requires the masculine article 'um', but the learner used the feminine 'uma'.
- Missed: `uma escritório`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma` -> `um` [agreement] The article 'um' is masculine to agree with the masculine noun 'escritório'.
- Found (false positive): `escritório` -> `escritório` [agreement] Nouns ending in -o typically require a masculine article, but 'escritório' is an exception that requires the feminine article 'uma'.

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `do` -> `de` [article_preposition] The preposition 'de' is not contracted with the article 'o' in Brazilian Portuguese when indicating origin.
- Found (matched): `moro Toronto` -> `moro em Toronto` [article_preposition] The verb 'morar' requires a preposition before the city name to indicate location.

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `do` -> `da` [article_preposition] The preposition 'de' should not be contracted with the article 'o' when indicating location of residence in this context.
- Found (matched): `moro Toronto` -> `moro em Toronto` [article_preposition] The verb 'morar' requires a preposition before the city name to indicate location.

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `jantar`

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `jantar`

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `jantar`

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

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vermelha` -> `vermelha` [agreement] Adjectives must agree in gender with the noun they modify; 'camisa' is feminine.
- Missed: `vermelha camisa`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vermelha` -> `vermelha` [agreement] Adjectives must agree in gender with the noun they modify; 'camisa' is feminine.
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `muito` -> `muito` [word_order] The adverb 'muito' is unnecessary and incorrect here; the correct placement or form would be 'gosto muito de café' or simply 'gosto de café'.
- Missed: `gosto de café muito`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `muito` -> `muito` [word_order] The adverb 'muito' is incorrectly placed after the direct object; it should come before the verb or be replaced by 'adore' for natural flow.
- Missed: `gosto de café muito`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `comi` [word_choice] The verb 'gostar' means 'to like', but the question asks what was eaten for breakfast.
- Found (false positive): `futebol` -> `café da manhã` [word_choice] 'Futebol' means football, which is not something one eats for breakfast.
- Found (false positive): `Flamengo` -> `` [content_mismatch] The Flamengo football team is irrelevant to the question about food eaten for breakfast.

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol` -> `Comi...` [content_mismatch] The answer talks about liking football instead of stating what was eaten for breakfast.
- Found (false positive): `e meu time favorito é o Flamengo` -> `` [content_mismatch] This information is irrelevant to the question about breakfast.

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `comi` [word_choice] The verb 'gostar' means 'to like', but the question asks what was eaten for breakfast.
- Found (false positive): `futebol` -> `pão` [word_choice] 'Futebol' means football, which is not something you eat for breakfast.
- Found (false positive): `Flamengo` -> `ovos` [content_mismatch] The Flamengo football team cannot be eaten for breakfast.

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

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The noun 'coffee' in Brazilian Portuguese requires an accent mark.
- Found (matched): `onibus` -> `ônibus` [spelling] The noun 'bus' in Brazilian Portuguese requires an accent mark.
- Missed: `as nove`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The noun 'coffee' must have an accent mark in Portuguese.
- Missed: `onibus`
- Missed: `as nove`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The noun 'coffee' in Brazilian Portuguese requires an accent mark: café.
- Found (matched): `onibus` -> `ônibus` [spelling] The word 'bus' in Brazilian Portuguese requires an accent mark: ônibus.
- Missed: `as nove`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective must agree in gender with the feminine noun 'amiga'.
- Missed: `curtas`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engradada` [agreement] The adjective must agree in gender with the feminine noun 'amiga'.
- Missed: `curtas`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective must agree in gender with the feminine noun 'amiga'.
- Missed: `curtas`

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `por passar`

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `por passar`

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `por passar`

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

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de` -> `ao` [article_preposition] The preposition 'para' is required before the mode of transport when indicating direction to a place.
- Missed: `autocarro`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `to` -> `tô` [spelling] The correct contraction of 'estou' in spoken Brazilian Portuguese is 'tô', but 'to' is a misspelling.
- Found (false positive): `pequeno-almoço` -> `pequeno almoço` [spelling] The word 'almoço' should be written as 'almoço' without the hyphen in Brazilian Portuguese.
- Missed: `pequeno-almoço`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pequeno-almoço` -> `pequeno almoço` [not_brazilian] The hyphenated form is European Portuguese; Brazilian Portuguese uses 'pequeno almoço' or 'café da manhã'.
- Missed: `pequeno-almoço`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `chovendo` [content_mismatch] This contradicts the scene fact that it is raining.
- Found (duplicate): `muito calor` -> `frio` [content_mismatch] This contradicts the scene fact that it is cold.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `está fazendo sol` -> `está chovendo` [content_mismatch] This contradicts the fact that it is raining.
- Found (duplicate): `muito calor` -> `frio` [content_mismatch] This contradicts the fact that it is cold.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `chovendo` [content_mismatch] This contradicts the fact that it is raining.
- Found (duplicate): `muito calor` -> `frio` [content_mismatch] This contradicts the fact that it is cold.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro` -> `Estou comendo` [word_choice] The sentence talks about a meal, but the verb 'moro' means 'I live', which is unrelated to eating.
- Found (false positive): `São Paulo` -> `um restaurante` [content_mismatch] The scene facts mention a restaurant setting with spaghetti and wine, not the city of São Paulo.
- Found (false positive): `cinco anos` -> `espaguete` [word_choice] The sentence describes a current meal, so mentioning how long someone has lived in a city is irrelevant.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `moro` -> `comemos` [word_choice] The sentence describes the current meal, but 'moro' means 'I live', which is unrelated to eating.
- Found (false positive): `São Paulo há cinco anos e gosto muito da cidade` -> `estamos comendo espaguete e bebendo vinho tinto` [content_mismatch] This information about living in the city contradicts the task requirement to describe the meal being eaten.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro` -> `` [content_mismatch] The answer describes the speaker's residence instead of describing the meal as requested.
- Found (false positive): `São Paulo` -> `` [content_mismatch] The answer mentions a city instead of describing the food items (spaghetti and red wine) mentioned in the scene.

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `tenho` -> `tive` [verb_form] The present tense is incorrect because the action started in the past and continues to the present; the present perfect compound tense (pretérito perfeito composto) is required.
- Found (duplicate): `visitado` -> `visitei` [verb_form] The participle 'visitado' is incorrect in this context; the auxiliary verb 'ter' requires a past participle that agrees with the subject, but the tense combination here implies a need for 'tido' + past participle or simply using 'sido', however, the intended meaning 'I have never visited' uses 'tive visitado' (colloquial) or better 'nunca visitei'. Actually, the strict grammatical error is mixing tenses. The learner likely meant 'nunca tive visitado' (incorrect usage of ter + participle for experience) or simply 'nunca visitei'. Let's correct to standard B1: 'nunca visitei'.
- Found (duplicate): `tenho visitado` -> `visitei` [verb_form] The construction 'ter + past participle' is not used to express life experience in Brazilian Portuguese; the simple past ('tive') or present perfect ('tenho sido') is needed, but for 'have never visited', the simple past 'nunca visitei' is the correct form.

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `tenho` -> `tive` [verb_form] The present tense is incorrect because the action started in the past and continues to the present; 'tinha' or 'tem' with a different structure is needed for the perfect aspect.
- Found (false positive): `visitado` -> `visitado` [verb_form] The past participle 'visitado' cannot be used with the auxiliary verb 'ter' to form the present perfect in Brazilian Portuguese.
- Found (duplicate): `nunca tenho visitado o Nordeste` -> `nunca visitei` [word_order] The sentence structure is grammatically incorrect for expressing a past experience up to now; it should use 'nunca visitei' or rephrase entirely.

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `tenho` -> `tinha` [verb_form] The present tense is incorrect here because the sentence describes an action that started in the past and continues to the present.
- Found (false positive): `visitado` -> `visitado` [verb_form] This part of the verb form is incorrect because 'ter' requires a past participle that agrees in gender and number with the subject.
- Found (duplicate): `nunca tenho visitado o Nordeste` -> `` [content_mismatch] The meaning of this clause contradicts the context implied by living in a specific place since 2019 without mentioning visiting another region, or it creates a logical inconsistency regarding the timeframe.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós` -> `` [pronoun] The pronoun 'nós' is unnecessary because the infinitive 'podermos' already indicates the subject.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `` [pronoun] The pronoun 'nós' is unnecessary here because the infinitive 'podermos' already indicates the subject.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós` -> `` [pronoun] The pronoun 'nós' is unnecessary and incorrect here because the infinitive 'podermos' already includes the subject.

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `tinha começado` -> `tinha começado` [verb_form] The past perfect tense requires the auxiliary verb 'ter' followed by the past participle of the main verb.
- Missed: `tinha começou`

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `tinha começado` -> `tinha começado` [verb_form] The past perfect tense in Portuguese requires the auxiliary verb 'ter' followed by the past participle, which must agree in gender and number with the subject.
- Missed: `tinha começou`

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `tinha começado` -> `tinha começado` [verb_form] The past perfect tense in Portuguese requires the auxiliary verb 'ter' followed by the past participle of the main verb.
- Missed: `tinha começou`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de` -> `do` [article_preposition] The preposition 'de' is incorrectly used before the article 'o', creating a non-standard contraction.
- Missed: `sobre do`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajará` [verb_form] The future tense is required here because the speaker is reporting a future plan from the present moment, not a past conditional.
- Found (false positive): `tinha` -> `tinha` [verb_form] Since the main verb 'disse' is in the past, the subordinate clause describing an action completed before that (buying tickets) requires the past perfect tense ('tinha comprado') to maintain sequence of tenses.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `` [pronoun] The indirect object pronoun 'lhe' is not used with the verb 'dizer' when referring to a family member like 'brother'; it should be omitted or replaced by 'para ele'.
- Found (false positive): `não me esperasse` -> `esperava` [verb_form] The subjunctive form 'esperasse' is incorrect here because the main clause uses the past tense 'disse', requiring the imperfect indicative 'esperava' to match the sequence of tenses.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `disse que não me esperasse` [word_choice] In Brazilian Portuguese, the indirect object pronoun 'lhe' is often replaced by 'para ele' or omitted in informal speech when referring to a specific person like a brother, though not strictly wrong, it can sound formal; however, the bigger issue is the verb form.
- Found (false positive): `não me esperasse` -> `disse que não esperasse` [word_choice] The subjunctive 'esperasse' is correct after 'dizer que', but in this context of telling someone not to wait, the imperative or a more natural phrasing like 'não esperasse' without the reflexive 'me' (since 'esperar' doesn't take an indirect object here) might be preferred, but the main error is actually the use of 'lhe' which is formal.
- Found (false positive): `lhe` -> `disse que não esperasse` [word_choice] Using 'lhe' to refer to 'brother' in this context is overly formal for B2 level Brazilian Portuguese; it's better to use 'para ele' or rephrase.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `disse para o meu irmão` [word_choice] The indirect object pronoun 'lhe' is not used with the verb 'dizer' when referring to a person in Brazilian Portuguese; instead, 'para ele' or simply omitting the pronoun is preferred.
- Found (false positive): `me esperasse` -> `esperasse` [verb_form] The subjunctive form 'esperasse' is incorrect here because the clause expresses a fact or a command in the past, not a hypothetical situation; the infinitive 'esperar' or indicative 'esperou' would be more appropriate depending on the intended meaning.
- Found (false positive): `me ouviu` -> `ouviu` [pronoun] The reflexive pronoun 'me' is unnecessary and incorrect in this context; the verb 'ouvir' does not require a reflexive pronoun when used transitively.

## qwen3:8b / plain

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `escritório` -> `escritório` [article_preposition] The definite article 'o' is missing.
- Missed: `uma escritório`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `escritório` -> `escritório` [article_preposition] The definite article 'o' is missing.
- Missed: `uma escritório`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `escritório` -> `escritório` [article_preposition] The definite article 'o' is missing.
- Missed: `uma escritório`

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `sou` -> `estou` [ser_estar] The verb 'ser' is incorrect here; 'estar' is used for temporary states like being tired.
- Found (false positive): `cansado` -> `cansado` [agreement] The adjective 'cansado' is correct, but it should agree in number with the subject 'eu', which is singular. However, since 'cansado' is already singular, this is correct.

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `sou` -> `estou` [ser_estar] The verb 'ser' is incorrect here; the correct verb is 'estar' for temporary states like being tired.
- Found (false positive): `cansado` -> `cansado` [agreement] The adjective 'cansado' is correct, but it should agree in number with the subject 'eu', which is singular. However, since 'cansado' is already singular, this is correct.

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `assisto televisão` -> `assistō à televisão` [article_preposition] The preposition 'a' is missing before 'televisão'
- Missed: `jantar`

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `assisto televisão` -> `assistir à televisão` [article_preposition] The preposition 'a' is missing before 'televisão'
- Missed: `jantar`

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `assisto televisão` -> `assistō à televisão` [article_preposition] The preposition 'a' is missing before 'televisão'
- Missed: `jantar`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `chamo` -> `me chamo` [word_choice] The correct form is 'chamo' for 'chamar' with 'eu', but in Brazilian Portuguese, it's more common to say 'me chamo' for a more natural expression.
- Found (false positive): `tenho vinte anos` -> `tenho vinte anos` [word_order] This is incorrect; the correct phrase is 'tenho vinte anos' (which is correct), but the structure 'tenho vinte anos' is correct. However, in Brazilian Portuguese, it's more natural to say 'tenho vinte anos' (which is correct).

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `chamo` -> `me chamo` [word_choice] The correct form is 'chamo' for 'chamar' in the first person singular, but in Brazilian Portuguese, it's more common to say 'me chamo' or 'me chamo Pedro' when introducing oneself.
- Found (false positive): `tenho` -> `tenho` [word_choice] The verb 'tenho' is correct, but in Brazilian Portuguese, it's more natural to say 'tenho vinte anos' without the 'tenho' when stating age in a simple way.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `chamo` -> `me chamo` [word_choice] The correct form is 'chamo' for 'chamar' when using 'eu', but in Brazilian Portuguese, it's more common to say 'me chamo' for a more natural expression.
- Found (false positive): `tenho vinte anos` -> `tenho vinte anos` [word_order] This is incorrect because 'tenho vinte anos' means 'I have twenty years', which is not natural in Portuguese. The correct expression is 'tenho vinte anos' (which is correct) but the structure is not natural.

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `chama` -> `se chama` [word_choice] The correct form is 'chama' for 'chamar' when using 'a minha amiga' as the subject. However, in Brazilian Portuguese, it is more natural to use 'se chama' in this context.
- Found (false positive): `tem` -> `trinta anos` [word_order] The verb 'tem' is correct, but in Brazilian Portuguese, it is more natural to say 'tem trinta anos' rather than 'ela tem trinta anos' when stating age.

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vermelha` -> `vermelha` [agreement] The adjective 'vermelha' is feminine, but the noun 'camisa' is masculine.
- Missed: `vermelha camisa`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vermelha` -> `vermelha` [agreement] The adjective 'vermelha' is feminine, but the noun 'camisa' is masculine.
- Missed: `vermelha camisa`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vermelha` -> `vermelha` [agreement] The adjective 'vermelha' is feminine, but the noun 'camisa' is masculine.
- Missed: `vermelha camisa`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo.` -> `Eu comi pão com manteiga e ovos.` [content_mismatch] The answer does not address the question about what was eaten for breakfast.

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo.` -> `Eu comi pão com manteiga e ovos.` [content_mismatch] The answer does not address the question about what was eaten for breakfast.

### a1-english-lunch (not_portuguese, a1)

- Task: What did you eat for lunch?
- Answer: I ate rice and chicken for lunch.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `estudei` -> `estudei` [verb_form] The verb should be in the past historic tense for a narrative context.
- Found (matched): `portugues` -> `português` [spelling] The word is missing an accent mark.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `estudei` -> `estudei` [verb_form] The verb should be in the past historic tense for a narrative context.
- Found (matched): `portugues` -> `português` [spelling] The word 'portugues' is missing an accent mark.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `estudei` -> `estudei` [verb_form] The verb should be in the past historic tense for a narrative context.
- Found (matched): `portugues` -> `português` [spelling] The word is missing an accent mark.

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tomei cafe` -> `tomei café` [spelling] cafe is not the correct spelling for coffee in Brazilian Portuguese
- Found (matched): `de onibus` -> `de ônibus` [spelling] onibus is not the correct spelling for bus in Brazilian Portuguese
- Missed: `as nove`

### a2-verb-form-irregular-fazer (erroneous, a2)

- Task: What did you do yesterday evening?
- Answer: Ontem à noite eu fazi o jantar e depois fui dormir cedo.
- Reference: `fazi` -> fiz [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `fazi` -> `fiz` [verb_form] incorrect verb form; the correct form is 'fiz'
- Found (false positive): `dormir` -> `dormi` [word_choice] the verb 'dormir' is incorrect; the correct verb is 'dormi'

### a2-verb-form-imperfect (erroneous, a2)

- Task: Describe your childhood.
- Answer: Quando eu era criança, eu moro numa cidade pequena e brincava na rua.
- Reference: `moro` -> morava [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `moro` -> `morava` [verb_form] The verb should be in the past tense because the sentence is about a childhood memory.
- Found (false positive): `brincava` -> `brincava` [verb_form] The verb should be in the past tense to match the rest of the sentence.

### a2-verb-form-imperfect (erroneous, a2)

- Task: Describe your childhood.
- Answer: Quando eu era criança, eu moro numa cidade pequena e brincava na rua.
- Reference: `moro` -> morava [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `moro` -> `morava` [verb_form] The verb should be in the past tense because the sentence is about a childhood memory.
- Found (false positive): `brincava` -> `brincava` [verb_form] The verb should be in the past tense to match the tense of the first part of the sentence.
- Found (false positive): `brincava` -> `brincava` [verb_form] The verb should be in the past tense to match the tense of the first part of the sentence.

### a2-verb-form-imperfect (erroneous, a2)

- Task: Describe your childhood.
- Answer: Quando eu era criança, eu moro numa cidade pequena e brincava na rua.
- Reference: `moro` -> morava [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `moro` -> `morava` [verb_form] The verb should be in the past tense because the sentence is about a childhood memory.
- Found (false positive): `brincava` -> `brincava` [verb_form] The verb should be in the past tense to match the tense of the first part of the sentence.
- Found (false positive): `brincava` -> `brincava` [verb_form] The verb should be in the past tense to match the tense of the first part of the sentence.

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `era` -> `estava` [ser_estar] The verb 'ser' is used incorrectly; 'estar' should be used for temporary states like being tired.
- Found (false positive): `fiquei` -> `fiquei` [verb_form] The verb 'ficar' is used with the wrong conjugation; 'fiquei' is first-person singular of 'ficar', but the subject is 'eu' and the correct form is 'fiquei' (which is correct). However, the context is about staying home, which is better expressed with 'fiquei em casa' (correct).
- Found (false positive): `dormi o dia todo` -> `todo o dia` [word_order] The phrase 'o dia todo' is not natural in Brazilian Portuguese; the correct expression is 'todo o dia' with the article before 'dia'.

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `era` -> `estava` [verb_form] The past tense 'era' is used for permanent characteristics, not for temporary states like being tired.
- Found (false positive): `o dia todo` -> `todo o dia` [word_order] In Brazilian Portuguese, 'todo o dia' is the correct phrase, not 'o dia todo'.

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `era` -> `estava` [ser_estar] The verb 'ser' is used incorrectly; 'estar' should be used for temporary states like being tired.
- Found (false positive): `fiquei` -> `fiquei` [verb_form] The verb 'ficar' is used with the wrong conjugation; 'fiquei' is first-person singular of 'ficar', but the correct form for 'eu' in pretérito perfeito is 'fiquei' (correct).
- Found (false positive): `dormi o dia todo` -> `todo o dia` [article_preposition] The phrase 'o dia todo' is incorrect; the correct expression is 'todo o dia' with the article before 'dia'.

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `vi` [word_choice] The verb 'atender' is not used to mean 'watch a movie'.
- Found (false positive): `um filme novo` -> `um novo filme` [word_order] In Brazilian Portuguese, 'filme novo' is correct, but the phrase 'um filme novo' is not natural in this context.

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

### a2-pronoun-reciprocal (erroneous, a2)

- Task: Describe your relationship with your neighbors.
- Answer: Eu me dou bem com meus vizinhos e sempre nós ajudamos quando precisamos.
- Reference: `nós ajudamos` -> nos ajudamos [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `nós` [word_choice] nós is not used in Brazilian Portuguese in this context with the verb 'ajudamos' when referring to a general habit.
- Found (matched): `ajudamos` -> `ajudam` [verb_form] ajudamos is incorrect in this context; the correct form is 'ajudam' for third person plural.

### a2-not-brazilian-gerund (erroneous, a2)

- Task: What are you doing now?
- Answer: Estou a comer pão com queijo na cozinha.
- Reference: `a comer` -> comendo [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a comer`

### a2-not-brazilian-gerund (erroneous, a2)

- Task: What are you doing now?
- Answer: Estou a comer pão com queijo na cozinha.
- Reference: `a comer` -> comendo [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a comer`

### a2-not-brazilian-gerund (erroneous, a2)

- Task: What are you doing now?
- Answer: Estou a comer pão com queijo na cozinha.
- Reference: `a comer` -> comendo [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a comer`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tomo` -> `como` [word_choice] The verb 'tomar' is not commonly used in Brazilian Portuguese for 'to have breakfast'.
- Missed: `pequeno-almoço`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eles estão comendo spaghetti e bebendo vinho tinto.` [content_mismatch] This sentence does not describe the meal being eaten, but rather talks about the speaker's residence and feelings about the city.

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `traga` -> `trazer` [verb_form] The verb should be in the infinitive form after 'quer' in a formal invitation.

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `traga` -> `trazer` [verb_form] The verb should be in the infinitive form after 'quer' in an invitation.

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `traga` -> `trazer` [verb_form] The verb should be in the infinitive form after 'que' in a subordinate clause.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós podermos` -> `para você podermos` [pronoun] The pronoun 'nós' is incorrect in this context; the correct pronoun is 'você' when addressing someone directly.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós podermos escolher` -> `para você podermos escolher` [pronoun] The pronoun 'nós' is incorrect in this context; the correct pronoun is 'você' (or the verb can be used without a pronoun).

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós podermos escolher` -> `para que nós pudéssemos escolher` [word_choice] the use of 'para' with an infinitive is incorrect in this context

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estudar` [verb_form] The verb form should be in the infinitive after 'recomendo que'
- Found (false positive): `exige` -> `exige` [verb_form] The verb form should be in the present tense third person singular after 'embora'
- Missed: `assiste`
- Missed: `exige`

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estudar` [verb_form] The verb form should be in the infinitive after 'recomendo que'
- Found (matched): `exige` -> `exigir` [verb_form] The verb form should be in the infinitive after 'embora'
- Missed: `assiste`

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estudar` [verb_form] The verb form should be in the infinitive after 'recomendo que'
- Found (matched): `exige` -> `exigir` [verb_form] The verb form should be in the infinitive after 'embora'
- Missed: `assiste`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `reclamar sobre do` -> `reclamar sobre` [word_order] The preposition 'sobre' is redundant here.
- Found (false positive): `deveria ter chegado no dia dez` -> `no dia dez` [article_preposition] In Brazilian Portuguese, it is more natural to say 'no dia dez' without 'no' before 'dia'.

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `reclamar sobre do` -> `reclamar sobre` [word_order] The preposition 'sobre' is redundant here.
- Found (false positive): `deveria ter chegado no dia dez` -> `no dia dez` [article_preposition] In Brazilian Portuguese, it is more natural to say 'no dia dez' without the preposition 'no' in this context.

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `reclamar sobre do` -> `reclamar sobre` [word_order] The preposition 'sobre' is redundant here.
- Found (false positive): `deveria ter chegado no dia dez` -> `no dia dez` [article_preposition] In Brazilian Portuguese, it is more natural to say 'no dia dez' without the preposition 'no' when referring to a specific date.

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajaria` [verb_form] The future tense is not used in reported speech in Brazilian Portuguese
- Found (false positive): `tinha` -> `tinha` [verb_form] The past perfect tense is not used in reported speech in Brazilian Portuguese

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajou` [verb_form] The future tense is not used in reported speech in Brazilian Portuguese; the simple past is used instead.
- Found (false positive): `tinha comprado` -> `comprado` [verb_form] The past perfect is not used in reported speech in Brazilian Portuguese; the simple past is used instead.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `apagar` -> `extinguir` [word_choice] apagar is not a common expression for 'put out' in the context of fires in Brazilian Portuguese.
- Found (false positive): `cheguei em casa morto de cansaço` -> `morta de cansaço` [agreement] The phrase is grammatically correct, but 'morto de cansaço' is more commonly expressed as 'morta de cansaço' when referring to a female speaker, or 'muito cansado' for a more natural expression.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `apagar` -> `extinguir` [word_choice] apagar is not a common expression for 'put out' in the context of fires in Brazilian Portuguese.
- Found (false positive): `cheguei em casa morto de cansaço` -> `morrer de cansaço` [word_choice] The phrase is grammatically correct, but 'morto de cansaço' is more commonly expressed as 'morrer de cansaço' in Brazilian Portuguese.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `apagar` -> `extinguir` [word_choice] apagar is not a common expression for 'put out' in the context of fires in Brazilian Portuguese.
- Found (false positive): `morto de cansaço` -> `muito cansado` [word_choice] morto de cansaço is not a common or natural expression in Brazilian Portuguese to describe being very tired.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `deveria ter chegado` -> `deveria ter chegado` [verb_form] the verb form is incorrect for the context of a formal complaint
- Found (false positive): `no dia dez` -> `no dia 10` [word_choice] the expression 'no dia dez' is not commonly used in formal Brazilian Portuguese

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `do atraso na entrega do meu pedido` -> `do atraso na entrega do meu pedido` [article_preposition] The preposition 'do' is incorrect in this context; the correct preposition is 'do' for 'de + o', but 'na entrega' requires 'na' as a preposition.
- Found (false positive): `que deveria ter chegado no dia dez` -> `que deveria ter chegado dia dez` [article_preposition] In Brazilian Portuguese, it is more natural to say 'no dia dez' without the preposition 'no' before 'dia'

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `do atraso na entrega do meu pedido` -> `sobre o atraso na entrega do meu pedido` [word_choice] The preposition 'do' is incorrect in this context; the correct preposition is 'do' for 'do atraso', but the phrase should be 'do atraso na entrega do meu pedido' is correct. However, the more natural and formal phrasing would be 'sobre o atraso na entrega do meu pedido'.
- Found (false positive): `que deveria ter chegado no dia dez` -> `que deveria ter chegado no dia dez` [word_order] In formal Brazilian Portuguese, it is more common to use 'no dia dez' without the preposition 'no' before 'dia'. The correct form is 'no dia dez' is correct, but in formal writing, it is often written as 'no dia dez' is correct. However, the more formal and natural phrasing is 'no dia dez' is correct. However, the more formal and natural phrasing is 'no dia dez' is correct. However, the more formal and natural phrasing is 'no dia dez' is correct. However, the more formal and natural phrasing is 'no dia dez' is correct. However, the more formal and natural phrasing is 'no dia dez' is correct. However, the more formal and natural phrasing is 'no dia dez' is correct. However, the more formal and natural phrasing is 'no dia dez' is correct. However, the more formal and natural phrasing is 'no dia dez' is correct. However, the more formal and natural phrasing is 'no dia dez' is correct. However, the more formal and natural phrasing is 'no dia dez' is correct. However, the more formal and natural phrasing is 'no dia dez' is correct. However, the more formal and natural phrasing is 'no dia dez' is correct. However, the more formal and natural phrasing is 'no dia dez' is correct. However, the more formal and natural phrasing is 'no dia dez' is correct. However, the more formal and natural phrasing is 'no dia dez' is correct. However, the more formal and natural phrasing is 'no dia dez' is correct. However, the more formal and natural phrasing is 'no dia dez' is correct. However, the more formal and natural phrasing is 'no dia dez' is correct. However, the more formal and natural ph, 
