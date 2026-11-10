# Grading failures

Every call with a missed error, a false alarm, a quote not in the answer, a wrong flag, or an invalid response.

## gemma3:12b / few_shot

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
- Found (matched): `manha` -> `manhã` [spelling] The correct word is 'manhã', with an accent mark.
- Missed: `cafe`

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
- Found (changes nothing): `escritório` -> `escritório` [spelling] This word needs an accent mark.
- Missed: `uma escritório`

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

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a1-english-lunch (not_portuguese, a1)

- Task: What did you eat for lunch?
- Answer: I ate rice and chicken for lunch.
- Flags: written_in_portuguese=false answers_the_question=false

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
- Found (duplicate): `muito calor` -> `muito frio` [content_mismatch] It is not hot, it is cold.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> `gostei muito` [article_preposition] 'Dele' is not necessary after 'gostei'.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

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
- Found (false positive): `no dia dez` -> `no dez` [article_preposition] In formal writing, 'no dia' is often omitted when referring to a specific date.
- Missed: `sobre do`

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

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `em casa` -> `a casa` [article_preposition] The correct preposition to use here is 'a'.

## gemma3:12b / few_shot_in_system

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
- Found (matched): `De manha` -> `De manhã` [spelling] The correct expression is 'De manhã'.
- Missed: `cafe`

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

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `vermelha` -> `vermelhada` [agreement] Adjectives must agree in gender with the noun they describe; camisa is feminine.
- Found (false positive): `uma calça` -> `calça` [article_preposition] When describing clothes, 'uma' is not used before 'calça'.

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
- Found (matched): `fazendo sol` -> `chovendo` [content_mismatch] It is not sunny, it is raining.
- Found (duplicate): `calor` -> `frio` [content_mismatch] It is not hot, it is cold.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> ` ` [article_preposition] The pronoun 'dele' is not needed after 'gostei'.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós podermos` -> `para podermos` [pronoun] The correct construction is 'para podermos' as 'nós' is unnecessary.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] After 'que' in the subjunctive, the verb should be in the subjunctive mood.
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `do dia dez` [article_preposition] When referring to a specific date, use 'do dia dez'.
- Missed: `sobre do`

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe disse` -> `a ele disse` [pronoun] With 'Eu', the indirect object pronoun 'lhe' is not used; use 'a ele disse'.

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
- Found (false positive): `tem` -> `há` [verb_form] With impersonal sentences like this, use 'há' instead of 'ter'.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `para que eu passe` -> `para que eu passe` [verb_form] After 'que' the subjunctive is needed.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `em` -> `a` [article_preposition] The correct preposition to use with 'chegar' is 'a'.

## gemma3:12b / few_shot_mostly_correct

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
- Found (matched): `De manha` -> `De manhã` [article_preposition] The correct expression is 'De manhã'.
- Missed: `cafe`

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

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo` -> `I don't know` [content_mismatch] This answer does not address the question about breakfast.

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] 'Engraçado' is masculine, but your friend is female, so it should be 'engraçada'.
- Found (matched): `curtas` -> `curtos` [agreement] 'Curtos' is masculine plural, but 'cabelos' is masculine plural, so the adjective needs to be feminine plural.
- Found (changes nothing): `pretos` -> `pretos` [agreement] 'Pretos' is masculine plural, but 'cabelos' is masculine plural, so the adjective needs to be feminine plural.

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> `do filme` [article_preposition] 'Dele' is not needed here; 'do filme' is the correct construction.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `podermos` -> `poder` [verb_form] After 'para' the subjunctive is needed.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] After 'que você', the verb should be in the subjunctive mood.
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `no dia do dez` [article_preposition] When referring to a specific date, 'no' should be followed by 'do' before the number.
- Missed: `sobre do`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tinha comprado` -> `comprara` [verb_form] The pluperfect tense (tinha comprado) is correct, but the conditional tense (viajaria) implies a degree of uncertainty that isn't present in the original statement.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `do dia dez` [article_preposition] When referring to a specific date, use 'do dia dez'.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe disse` -> `disse a ele` [word_choice] With 'Eu', the indirect object pronoun 'lhe' is not commonly used in Brazil; 'Eu disse a ele' is more natural.

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

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sempre fui`

### a2-control-phone (control, a2)

- Task: Where is your phone?
- Answer: O meu celular está em cima do sofá.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `O meu` -> `Meu` [article_preposition] When referring to possession, 'meu' should be used without the article 'o'.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `para que eu passe` -> `para que eu passe` [verb_form] After 'que' the subjunctive is needed.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `em` -> `a` [article_preposition] After 'cheguei', the correct preposition is 'a'.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está chovendo muito e eu estou em casa.` -> `Meu nome é...` [content_mismatch] The answer does not answer the question.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei uma bicicleta nova no sábado.` -> `I'm sorry, I don't know how to answer that question.` [content_mismatch] This answer doesn't address the question about working from home.

## gemma3:12b / few_shot_reversed

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
- Found (matched): `manha` -> `manhã` [spelling] The word needs an accent on the 'a': manhã.
- Missed: `cafe`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [spelling] The word needs an accent on the r: escritório.
- Missed: `uma escritório`

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
- Found (matched): `vermelha` -> `uma camisa vermelha` [word_order] Adjectives describing clothes usually precede the noun.
- Found (false positive): `está com uma` -> `usa uma` [word_choice] 'Está com' is not the best way to describe what someone is wearing.

### a1-english-lunch (not_portuguese, a1)

- Task: What did you eat for lunch?
- Answer: I ate rice and chicken for lunch.
- Flags: written_in_portuguese=false answers_the_question=false

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
- Found (matched): `fazendo sol` -> `está chovendo` [content_mismatch] The scene states it is raining, not sunny.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> `do filme` [article_preposition] 'Dele' is not needed here; 'do filme' is correct.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] After 'que' in the subjunctive, the verb should be in the subjunctive mood.
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `no dia 10` [article_preposition] When referring to a specific date, 'no' requires the definite article.
- Missed: `sobre do`

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

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tem` -> `há` [verb_form] With impersonal sentences like this, 'ter' requires the inverted form 'há'.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `em` -> `a` [article_preposition] The correct preposition to use with 'casa' is 'a'.

## gemma3:12b / few_shot_six

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
- Found (matched): `De manha` -> `De manhã` [article_preposition] 'De manhã' is one word.
- Missed: `cafe`

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
- Found (matched): `fazendo sol` -> `está chovendo` [content_mismatch] The scene facts state that it is raining, not sunny.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

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

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tem` -> `há` [verb_form] After 'na minha bolsa', the verb should be 'há'.

## gemma3:12b / one_shot_correct

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
- Found (matched): `De manha` -> `De manhã` [article_preposition] The correct preposition to use here is 'De manhã'.
- Missed: `cafe`

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
- Found (changes nothing): `escritório` -> `escritório` [agreement] The word should be feminine to agree with 'uma'.
- Missed: `uma escritório`

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Toronto` -> `Toronto` [word_choice] This is a city name and should be 'Toronto' in Portuguese.
- Missed: `moro Toronto`

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `jantar` -> `janto` [verb_form] The verb 'jantar' needs to be conjugated in the first person singular.
- Found (changes nothing): `às` -> `às` [article_preposition] The correct preposition to use with 'oito' is 'à'.

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

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `muito` -> `muito` [word_order] The correct way to express 'very much' with 'gostar' is 'muito' after the verb.
- Missed: `gosto de café muito`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol` -> `Eu comi...` [content_mismatch] The question asks what you ate, not what you like.

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [word_choice] This is a French word, the Portuguese word for coffee is 'café'.
- Found (matched): `as nove` -> `às nove` [article_preposition] The correct preposition to use with hours is 'à'.
- Missed: `onibus`

### a2-verb-form-irregular-fazer (erroneous, a2)

- Task: What did you do yesterday evening?
- Answer: Ontem à noite eu fazi o jantar e depois fui dormir cedo.
- Reference: `fazi` -> fiz [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `fazi` -> `fiz` [verb_form] The verb 'fazer' needs to be conjugated in the past tense.
- Found (changes nothing): `dormir` -> `dormir` [verb_form] After 'fui', the infinitive 'dormir' should be used.

### a2-verb-form-imperfect (erroneous, a2)

- Task: Describe your childhood.
- Answer: Quando eu era criança, eu moro numa cidade pequena e brincava na rua.
- Reference: `moro` -> morava [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `moro` -> `morava` [verb_form] The verb 'morar' needs to be in the imperfect tense to describe a past state.
- Found (false positive): `numa` -> `em uma` [article_preposition] The correct preposition is 'em' before a feminine noun.

### a2-verb-form-subject-agreement (erroneous, a2)

- Task: What did your family do on Sunday?
- Answer: No domingo minha família almoçaram na casa da minha avó.
- Reference: `almoçaram` -> almoçou [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `almoçaram` -> `almoçaram` [verb_form] The verb needs to agree with the subject 'minha família', which is plural.
- Missed: `almoçaram`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `assisti` [word_choice] The verb 'atender' means to assist or attend to someone, not to watch a film.
- Found (false positive): `um filme novo` -> `o filme novo` [article_preposition] When referring to a specific film, you should use the definite article.

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> `muito` [word_choice] The pronoun 'dele' is unnecessary and sounds unnatural in this context.

### b1-verb-form-subjunctive-present (erroneous, b1)

- Task: What do you hope for the future?
- Answer: Espero que meus filhos são felizes e que eles tenham uma boa educação.
- Reference: `são` -> sejam [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `são` -> `estão` [ser_estar] The verb 'ser' is incorrect here; 'estar' is needed to express a state or condition.
- Found (false positive): `tenham` -> `têm` [verb_form] The verb 'ter' needs to be in the subjunctive mood here.

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `ter` -> `tiver` [verb_form] After 'se' in a future tense clause, the subjunctive form of the verb is needed.
- Found (false positive): `no mês que vem` -> `do mês que vem` [article_preposition] The correct preposition to use with 'mês' is 'do'.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] The verb 'estudar' needs to be in the subjunctive mood after 'recomendo que'.
- Found (matched): `assiste` -> `assista` [verb_form] The verb 'assistir' needs to be in the subjunctive mood after 'recomendo que'.
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `no dia dez` -> `no dia dez` [article_preposition] The preposition 'no' is needed before 'dia'.
- Missed: `sobre do`

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

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está` -> `Meu nome é` [content_mismatch] The question asks for the name, not a description of the weather.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei` -> `Eu acho` [content_mismatch] The answer describes a past action, but the question asks for an opinion about working from home.

### a2-english-coffee (not_portuguese, a2)

- Task: Do you like coffee?
- Answer: Yes, I like coffee a lot.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma3:12b / one_shot_mistake

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
- Found (matched): `De manha` -> `De manhã` [spelling] The correct expression is 'De manhã'.
- Missed: `cafe`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [spelling] 'escritório' needs a cedilla under the 'c'.
- Missed: `uma escritório`

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
- Found (matched): `vermelha` -> `vermelho` [agreement] Adjectives must agree in gender with the noun they describe; camisa is masculine.
- Found (quote not in the answer): `uma camisa` -> `um camisa` [article_preposition] When the article 'uma' is used before a word starting with a vowel, it should be 'um'.

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol` -> `Eu comi...` [content_mismatch] The question asks what you ate, not what you like.

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent mark: café.
- Found (matched): `as nove` -> `às nove` [article_preposition] The correct preposition to use with hours is 'à'.
- Missed: `onibus`

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> `disso` [word_choice] 'dele' is not used in this context; 'disso' is more appropriate.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] After 'você', the verb should be in the infinitive form.
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `no dia 10` [word_choice] It's more natural to say 'no dia 10' when referring to a date.
- Missed: `sobre do`

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

### a2-english-coffee (not_portuguese, a2)

- Task: Do you like coffee?
- Answer: Yes, I like coffee a lot.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma3:12b / plain

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Só` -> `Um pouco` [word_choice] The correct word to use here is 'um pouco'.

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fejão`

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `jantar` -> `janto` [verb_form] The verb 'jantar' needs to be conjugated in the first person singular.
- Found (changes nothing): `às` -> `às` [article_preposition] The correct preposition to use with the time is 'à'.

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A minha amiga` -> `Minha amiga` [agreement] When referring to a female friend, 'amiga' should agree in gender with the possessive adjective.
- Missed: `chama`

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
- Found (matched): `vermelha` -> `vermelho` [agreement] Adjectives must agree in gender with the noun they describe; camisa is masculine.
- Found (false positive): `uma calça` -> `calça` [article_preposition] When referring to clothes, we usually omit the indefinite article.

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `muito` -> `muito` [spelling] The correct way to express 'very much' after 'gostar' is 'muito' without an accent.
- Missed: `gosto de café muito`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol` -> `Eu comi...` [content_mismatch] The question asks what you ate, not what you like.

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The correct word for coffee is 'café'.
- Found (matched): `as nove` -> `às nove` [article_preposition] The correct way to say 'at nine' is 'às nove'.
- Missed: `onibus`

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
- Found (changes nothing): `nós ajudamos` -> `nós ajudamos` [agreement] The verb 'to help' needs to agree with the subject 'we' in this context.
- Missed: `nós ajudamos`

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> `` [word_choice] The pronoun 'dele' is unnecessary and sounds unnatural after 'gostei'.

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `à` -> `ao` [article_preposition] The preposition 'a' contracts to 'à' before feminine words starting with a vowel, but 'festa' is masculine.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós podermos` -> `podérmos` [verb_form] After 'para', the subjunctive mood is needed.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] The verb 'estudar' needs to be in the subjunctive mood here because it follows 'recomendo que'.
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `em dez` [article_preposition] The correct preposition to use with dates is 'em'.
- Missed: `sobre do`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajará` [verb_form] The conditional tense is not needed here, the future tense is more appropriate.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cheguei em casa` -> `cheguei a casa` [article_preposition] The correct preposition to use with 'chegar' is 'a'.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `em dez` [article_preposition] The correct preposition to use with dates is 'em'.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `a ele` [pronoun] The indirect object pronoun 'lhe' is not used in this context; 'a ele' is needed.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `eu preocupo` -> `eu me preocupo` [verb_form] The correct verb to use here is 'to worry' which is 'preocupar-se', and it requires a reflexive pronoun.
- Found (false positive): `muito` -> `bastante` [word_choice] While 'muito' is not incorrect, a more natural phrasing for an A1 level would be 'bastante'.

### a1-pronoun-possessive-pet (erroneous, a1)

- Task: Describe your pet.
- Answer: Eu tenho um cachorro. Mim cachorro se chama Bolt.
- Reference: `Mim cachorro` -> Meu cachorro [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `Mim` -> `Meu` [pronoun] The possessive pronoun 'mim' means 'me' and is used as an object, not to indicate possession.
- Found (false positive): `se chama` -> `chama` [verb_form] After 'chama', the verb should be in the present tense.

### a1-short-agreement-books (erroneous, a1)

- Task: What is in your bag?
- Answer: Na minha bolsa tem um livros.
- Reference: `um livros` -> um livro | uns livros [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `um livros` -> `um livros` [article_preposition] The word 'livros' is masculine plural, so it needs the masculine indefinite article 'um'.
- Missed: `um livros`

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Ontem` -> `Amanhã` [word_choice] The correct word to use here is 'Amanhã'.
- Missed: `fiz um erro`

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `é doente`

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tem` -> `há` [verb_form] The verb 'ter' means 'to have', and in this context, 'haver' is more appropriate to express existence.

### a2-control-imperfect-grandmother (control, a2)

- Task: Describe your grandmother's house.
- Answer: A casa da minha avó era pequena, mas tinha um jardim grande com muitas flores.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `era` -> `está` [ser_estar] To describe a house that still exists, use estar instead of ser.

### a2-control-pet (control, a2)

- Task: Describe your pet.
- Answer: Eu tenho um cachorro. O meu cachorro se chama Bolt e ele adora correr.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `O meu cachorro` -> `Meu cachorro` [agreement] Possessive adjectives should agree in gender with the noun they modify.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para que eu passe` -> `para passar` [verb_form] The subjunctive is not needed here, as the purpose is certain.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `em` -> `a` [article_preposition] The correct preposition to use with 'chegar' is 'a'.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei` -> `Eu acho` [verb_form] The correct verb tense to express an opinion is 'Eu acho', not the preterite tense.

## gemma3:4b / few_shot

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô` -> `Estou` [verb_form] This is a colloquial form of 'estou' which is not appropriate for this level.

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `manha`
- Missed: `cafe`

### a1-agreement-trousers (erroneous, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: Marina veste uma camiseta azul e uma calça pretos.
- Reference: `pretos` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `veste` -> `usa` [word_choice] The verb 'vestir' is not needed here.
- Missed: `pretos`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `escritório` -> `escritório` [spelling] The word needs an accent on the i: escritório.
- Missed: `uma escritório`

### a1-agreement-plural-adjective (erroneous, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos. Eles são muito simpático.
- Reference: `simpático` -> simpáticos [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `simpático` -> `simpáticos` [agreement] The adjective needs to be in the plural form to agree with 'irmãos'.

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `sou em casa` -> `estou em casa` [ser_estar] The correct expression is 'estou em casa'.

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `estou brasileiro`

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `moro Toronto`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Eu chamo` -> `Meu nome é` [word_choice] This phrase is not appropriate for answering 'What is your name?'

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `A minha amiga chama Maria` -> `Minha amiga se chama Maria` [word_order] The sentence structure is incorrect; it should be 'Minha amiga se chama Maria'.

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
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective needs to be masculine singular to agree with 'camisa'.
- Missed: `vermelha camisa`

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `uma jaqueta azul` -> `uma jaqueta verde` [word_choice] The jacket is green, not blue.

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de` -> `Eu comi` [word_choice] This phrase is not appropriate for answering the question.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `à tarde` -> `à tarde` [word_choice] The correct expression is 'à tarde'
- Missed: `portugues`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de onibus` -> `em ônibus` [word_choice] The correct preposition is 'em': 'em ônibus'.
- Missed: `cafe`
- Missed: `as nove`

### a2-verb-form-irregular-fazer (erroneous, a2)

- Task: What did you do yesterday evening?
- Answer: Ontem à noite eu fazi o jantar e depois fui dormir cedo.
- Reference: `fazi` -> fiz [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (quote not in the answer): `fizi` -> `fez` [verb_form] The verb needs to be in the past tense: fez.
- Missed: `fazi`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-verb-form-imperfect (erroneous, a2)

- Task: Describe your childhood.
- Answer: Quando eu era criança, eu moro numa cidade pequena e brincava na rua.
- Reference: `moro` -> morava [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `eu moro` -> `eu vivia` [verb_form] The correct form is 'eu vivia' or 'eu morei' to describe a past state.

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `era cansado`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `engraçado` -> `engraçado` [agreement] The adjective needs to agree in gender with 'amiga'.
- Missed: `engraçado`
- Missed: `curtas`

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `passar no exame` -> `passar no exame de proficiência` [word_choice] The correct expression is 'passar no exame de proficiência'.

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `atendi` -> `assisti` [verb_form] The verb 'atender' is not appropriate in this context; 'assisti' is needed.

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
- Found (changes nothing): `autocarro` -> `autocarro` [spelling] The word needs an accent on the ‘o’: autocarro.
- Missed: `autocarro`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pequeno-almoço`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `está chovendo` [word_choice] The weather is raining, so 'fazendo sol' is incorrect.

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós ficamos` -> `nós ficámos` [verb_form] The correct reflexive pronoun is 'nós ficámos'.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> `` [word_choice] The pronoun 'dele' is not needed here.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro` -> `Eles estão comendo` [content_mismatch] The answer should describe the meal, not the speaker's location.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-verb-form-subjunctive-present (erroneous, b1)

- Task: What do you hope for the future?
- Answer: Espero que meus filhos são felizes e que eles tenham uma boa educação.
- Reference: `são` -> sejam [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `são` -> `tenham` [verb_form] The verb needs to be 'tenham' to agree with 'eles'.

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Se eu ter`

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `ganhava`

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `nunca tenho visitado` -> `nunca visitei` [verb_form] The correct expression is 'nunca visitei'.

### b1-agreement-passive (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1750 e foi restaurado em 2005.
- Reference: `restaurado` -> restaurada [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `restaurado` -> `restaurada` [ser_estar] The verb needs to be in the passive voice: 'restaurado' should be 'restaurada'.

### b1-content-date (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1850 e restaurada em 2005.
- Reference: `1850` -> 1750 [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `1850` -> `1750` [content_mismatch] The year of construction is incorrect; it was built in 1750.

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `você possa` -> `pode` [verb_form] The correct form is 'pode', as it's a direct address.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The conditional form needs to be 'ganhasse'.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `nós` -> `` [pronoun] The pronoun 'nós' is not needed here.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Sr. Souza` -> `Senhor Souza` [word_choice] It's more formal to use 'Senhor Souza'.

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `já tinha começou` -> `já tinha começado` [verb_form] The correct form is ‘já tinha começado’

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `você` -> `` [pronoun] The pronoun is not needed here.
- Missed: `estuda`
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `do meu pedido` -> `do meu pedido` [word_choice] The phrase 'do meu pedido' is redundant and less formal in this context.
- Missed: `sobre do`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `viajaria` -> `viajaria` [verb_form] The future tense needs to be expressed correctly with 'viajaria'.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `preocupo` -> `preocupo` [verb_form] The verb needs to be in the first person singular.
- Missed: `eu preocupo`

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu diverti`

### b2-pronoun-clitic-gender (erroneous, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, eu falei com a Ana e convidei-o para a festa.
- Reference: `convidei-o` -> convidei-a [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `convidei-o` -> `convidei-a` [pronoun] The pronoun ‘o’ is incorrect; it should be ‘ela’ because Ana is a woman.

### a1-short-verb-morar (erroneous, a1)

- Task: Where do you live?
- Answer: Eu mora em Lisboa.
- Reference: `mora` -> moro [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `mora` -> `mor` [verb_form] The verb needs to agree with the subject 'Eu'.

### a1-short-agreement-house (erroneous, a1)

- Task: Describe your house.
- Answer: Minha casa é grande e bonito.
- Reference: `bonito` -> bonita [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `bonito` -> `bonito` [spelling] The adjective needs an accent mark: bonito.
- Missed: `bonito`

### a1-short-agreement-books (erroneous, a1)

- Task: What is in your bag?
- Answer: Na minha bolsa tem um livros.
- Reference: `um livros` -> um livro | uns livros [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `livros` -> `livro` [spelling] The word needs an accent on the i: livro.

### a1-word-choice-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu sou vinte anos velho.
- Reference: `sou vinte anos velho` -> tenho vinte anos [word_choice]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `vinte anos velho` -> `vinte anos` [word_choice] The correct phrase is 'vinte anos' to express age.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-article-contraction-a-o (erroneous, a1)

- Task: Where did you go on Saturday?
- Answer: No sábado eu fui a o cinema com meus amigos.
- Reference: `a o` -> ao [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `a o` -> `ao` [article_preposition] The preposition ‘a’ is not needed before ‘o’ in this context.

### a2-article-contraction-de-o (erroneous, a2)

- Task: Whose is the car?
- Answer: O carro é de o meu pai.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `o meu` -> `meu` [word_choice] The correct expression is 'meu'.

### a2-preposition-gostar-de (erroneous, a2)

- Task: What do you like about the city?
- Answer: Eu gosto a cidade porque é tranquila.
- Reference: `gosto a cidade` -> gosto da cidade [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `gosto a cidade` -> `gosto da cidade` [word_choice] The correct structure is 'gosto da cidade'.

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `é doente`

### b1-verb-form-para-que (erroneous, b1)

- Task: Why do you study every day?
- Answer: Eu estudo todos os dias para que eu passo no exame.
- Reference: `eu passo` -> eu passe [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `que eu passo` -> `para` [word_choice] The ‘que’ is redundant and unnecessary here.

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sempre fui`

### b2-agreement-news (erroneous, b2)

- Task: Comment on the news.
- Answer: As notícias de hoje são preocupante.
- Reference: `preocupante` -> preocupantes [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `notícias` -> `notícias` [word_choice] The correct word is 'notícias', not 'notícia'.
- Missed: `preocupante`

### a1-word-order-article-noun (erroneous, a1)

- Task: Where do you live?
- Answer: Eu moro com meus pais em casa uma pequena.
- Reference: `casa uma pequena` -> uma casa pequena [word_order]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `em casa uma pequena` -> `em uma casa pequena` [article_preposition] The preposition 'em' is not needed here, and 'uma' is incorrect.

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Tô` -> `Estou` [word_choice] This is a colloquial form of 'estou' and is not appropriate for this level.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `que eu passe` -> `para passar` [word_choice] The ‘que eu passe’ construction is less common and sounds unnatural in this context.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Trabalhei` -> `Eu trabalhei` [verb_form] The verb needs to be in the present perfect tense to describe a day.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `eu estou` -> `` [content_mismatch] This is a statement about the weather, not a question about the speaker's name.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `comprei` -> `Eu comprei` [verb_form] The verb needs to be in the present simple to answer the question.

### a2-english-coffee (not_portuguese, a2)

- Task: Do you like coffee?
- Answer: Yes, I like coffee a lot.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma3:4b / few_shot_in_system

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô` -> `Estou` [word_choice] This is a shortened, informal way of saying 'Estou', which is not appropriate for this level.

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
- Missed: `manha`
- Missed: `cafe`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `escritório` -> `escritório` [spelling] The word needs an accent on the i: escritório.
- Missed: `uma escritório`

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `estou brasileiro`

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `moro Toronto`

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
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `A minha amiga` -> `Minha amiga` [word_choice] The correct phrase is 'Minha amiga'.
- Missed: `chama`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No` -> `No` [spelling] The correct preposition is 'no' (with no accent mark).
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective needs to agree in gender with 'camisa'.
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `gosto de café muito`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol` -> `` [content_mismatch] This answer is irrelevant to the question about breakfast.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `à tarde` -> `à tarde` [word_choice] The correct expression is 'à tarde'
- Missed: `portugues`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de onibus` -> `em ônibus` [word_choice] The correct preposition is 'em' (em ônibus).
- Missed: `cafe`
- Missed: `as nove`

### a2-verb-form-irregular-fazer (erroneous, a2)

- Task: What did you do yesterday evening?
- Answer: Ontem à noite eu fazi o jantar e depois fui dormir cedo.
- Reference: `fazi` -> fiz [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `fizi` -> `fez` [verb_form] The verb needs to be conjugated correctly in the past: fez.
- Missed: `fazi`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-verb-form-subject-agreement (erroneous, a2)

- Task: What did your family do on Sunday?
- Answer: No domingo minha família almoçaram na casa da minha avó.
- Reference: `almoçaram` -> almoçou [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Em` [word_choice] The correct preposition is 'Em': Em domingo.
- Missed: `almoçaram`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective needs to agree in gender with the noun: engraçada.
- Missed: `curtas`

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

### a2-pronoun-reciprocal (erroneous, a2)

- Task: Describe your relationship with your neighbors.
- Answer: Eu me dou bem com meus vizinhos e sempre nós ajudamos quando precisamos.
- Reference: `nós ajudamos` -> nos ajudamos [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `me dou` -> `dou` [word_choice] The reflexive pronoun is not needed here.
- Missed: `nós ajudamos`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `autocarro` -> `autocarro` [spelling] The word needs an accent on the ‘o’: autocarro.
- Missed: `autocarro`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pequeno-almoço`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `chovendo` [content_mismatch] The scene states it is raining, so 'sol' is incorrect.

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Em` [article_preposition] The correct preposition is 'Em' before 'ano'.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A gente` -> `Nós` [not_brazilian] This is a European Portuguese construction, use 'nós' instead.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu` -> `` [content_mismatch] This sentence is about the meal, not the speaker's location.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ter` -> `ter` [word_choice] The correct form is 'ter' in this context, but the phrasing is awkward.
- Missed: `Se eu ter`

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `ganhava`

### b1-agreement-passive (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1750 e foi restaurado em 2005.
- Reference: `restaurado` -> restaurada [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `restaurado` -> `restaurada` [agreement] The verb needs to agree in gender with 'catedral' (feminine).

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `você` -> `` [pronoun] The pronoun is not necessary in this context.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `` [word_choice] The pronoun 'nós' is not needed here, it's redundant.

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `gente` -> `colegas` [word_choice] The word ‘gente’ is not appropriate for this level.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Sr.` -> `Senhor` [word_choice] It's more common to use 'Senhor' instead of 'Sr.' in a formal email.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `estuda`
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `do atraso` -> `do atraso` [word_choice] The correct expression is 'do atraso', not 'de atraso'.
- Missed: `sobre do`

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `no dia 10` [word_choice] The correct expression is 'no dia 10'.

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu diverti`

### a1-short-verb-morar (erroneous, a1)

- Task: Where do you live?
- Answer: Eu mora em Lisboa.
- Reference: `mora` -> moro [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `mora` -> `mor` [verb_form] The verb needs to agree with the subject 'Eu'.

### a1-short-agreement-house (erroneous, a1)

- Task: Describe your house.
- Answer: Minha casa é grande e bonito.
- Reference: `bonito` -> bonita [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `bonito` -> `bonita` [agreement] The adjective needs to be in the feminine form to agree with 'casa'.

### a1-short-agreement-books (erroneous, a1)

- Task: What is in your bag?
- Answer: Na minha bolsa tem um livros.
- Reference: `um livros` -> um livro | uns livros [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `livros` -> `livro` [spelling] The word needs an accent on the i: livro.

### a1-word-choice-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu sou vinte anos velho.
- Reference: `sou vinte anos velho` -> tenho vinte anos [word_choice]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `vinte anos velho` -> `tinha` [verb_form] The correct phrase is 'vinte anos velho' is incorrect, as 'velho' is an adjective and needs to be replaced with 'tinha'.

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

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sempre fui`

### a1-word-order-article-noun (erroneous, a1)

- Task: Where do you live?
- Answer: Eu moro com meus pais em casa uma pequena.
- Reference: `casa uma pequena` -> uma casa pequena [word_order]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `casa uma` -> `casa pequena` [article_preposition] The article 'a' is incorrect here; it should be 'a' or 'o'.

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô` -> `Estou` [word_choice] The learner should use 'Estou' instead of 'Tô' for 'I am'.

### a2-control-family-meal (control, a2)

- Task: Describe the meal these people are eating.
- Answer: A família está comendo arroz, feijão e frango grelhado. Eles bebem suco de laranja.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `família` -> `família` [spelling] The word needs an accent on the i: família.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `eu` -> `` [pronoun] The pronoun is not needed here.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `comprei` -> `penso` [verb_form] The verb tense is incorrect for this question.

### a2-english-coffee (not_portuguese, a2)

- Task: Do you like coffee?
- Answer: Yes, I like coffee a lot.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma3:4b / few_shot_mostly_correct

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô` -> `Estou` [verb_form] This is European Portuguese for 'I am'. The correct form is 'Estou'.

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `feijão` -> `feijão` [spelling] The word needs an accent on the i: feijão.
- Missed: `fejão`

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha` -> `De manhã` [word_choice] The correct expression is 'De manhã'.
- Missed: `cafe`

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `moro Toronto`

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A minha amiga` -> `Minha amiga` [word_choice] The correct phrase is 'Minha amiga'.
- Missed: `chama`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No` -> `no` [article_preposition] The correct preposition is 'no', but it needs a contraction with almoço: ‘no almoço’
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective needs to agree in gender with camisa: vermelha, not vermelha.
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `gosto de café muito`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol` -> `` [content_mismatch] This is irrelevant to the question about breakfast.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `portugues`

### a2-spelling-agua (erroneous, a2)

- Task: What did you buy at the market?
- Answer: No mercado eu comprei maçãs, dois quilos de arroz e uma garrafa de agua.
- Reference: `agua` -> água [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `agua`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `cafe`
- Missed: `onibus`
- Missed: `as nove`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-verb-form-imperfect (erroneous, a2)

- Task: Describe your childhood.
- Answer: Quando eu era criança, eu moro numa cidade pequena e brincava na rua.
- Reference: `moro` -> morava [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `moro`

### a2-verb-form-subject-agreement (erroneous, a2)

- Task: What did your family do on Sunday?
- Answer: No domingo minha família almoçaram na casa da minha avó.
- Reference: `almoçaram` -> almoçou [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `almoçaram` -> `almoçaram` [verb_form] The verb needs to agree with the subject: minha família (plural) so it should be 'almoçaram'.
- Missed: `almoçaram`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `era cansado`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `engraçado`
- Missed: `curtas`

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

### a2-pronoun-reciprocal (erroneous, a2)

- Task: Describe your relationship with your neighbors.
- Answer: Eu me dou bem com meus vizinhos e sempre nós ajudamos quando precisamos.
- Reference: `nós ajudamos` -> nos ajudamos [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `me dou` -> `dou` [word_choice] The correct reflexive construction is 'me dou'
- Missed: `nós ajudamos`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `autocarro` -> `autocarro` [spelling] The word needs an accent on the o: autocarro.
- Missed: `autocarro`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pequeno-almoço`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro` -> `Eles estão comendo` [content_mismatch] This sentence is irrelevant to the task and contains a personal statement.

### b1-verb-form-subjunctive-present (erroneous, b1)

- Task: What do you hope for the future?
- Answer: Espero que meus filhos são felizes e que eles tenham uma boa educação.
- Reference: `são` -> sejam [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `são`

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Se eu ter`

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `ganhava`

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `tenho visitado`

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `você possa` -> `possa` [verb_form] The correct form to use when inviting someone is 'possa'

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `` [pronoun] The pronoun 'nós' is not needed here, as the question is about the speaker's reason.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Sr.` -> `Sr.` [spelling] The title needs an accent on the i: Sr.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `você` -> `` [pronoun] The pronoun is not needed here.
- Missed: `estuda`
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sobre do`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajarei` [verb_form] The conditional form is not needed here, the simple future is correct.

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

### a1-article-contraction-a-o (erroneous, a1)

- Task: Where did you go on Saturday?
- Answer: No sábado eu fui a o cinema com meus amigos.
- Reference: `a o` -> ao [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a o`

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

### b2-agreement-news (erroneous, b2)

- Task: Comment on the news.
- Answer: As notícias de hoje são preocupante.
- Reference: `preocupante` -> preocupantes [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `As notícias` -> `As notícias` [word_choice] The correct phrase is 'As notícias'
- Missed: `preocupante`

### a1-word-order-article-noun (erroneous, a1)

- Task: Where do you live?
- Answer: Eu moro com meus pais em casa uma pequena.
- Reference: `casa uma pequena` -> uma casa pequena [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `casa uma pequena`

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pra` -> `para` [article_preposition] The correct preposition is 'para'.

### a2-control-phone (control, a2)

- Task: Where is your phone?
- Answer: O meu celular está em cima do sofá.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `O meu` -> `Meu` [word_choice] The ‘o’ is unnecessary here.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `que eu passe` -> `para passar` [word_choice] The phrase 'que eu passe' is overly formal and not typical for B1 level.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `eu estou` -> `` [content_mismatch] This is an irrelevant statement to the question.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `comprei` -> `Eu acho` [content_mismatch] This answer is about working from home, not buying a bicycle.

### a2-english-coffee (not_portuguese, a2)

- Task: Do you like coffee?
- Answer: Yes, I like coffee a lot.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma3:4b / few_shot_reversed

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A Marina` -> `` [article_preposition] The article 'A' is unnecessary here.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Tô` -> `Estou` [verb_form] This is a colloquial form of 'estou' which is not appropriate for this level.

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu me chamo Ana` -> `Meu nome é Ana` [word_choice] This is not the correct way to answer the question 'What is your name?'

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `feijão` -> `feijão` [spelling] The word needs an accent on the i: feijão.
- Missed: `fejão`

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha` -> `De manhã` [word_choice] The correct expression is 'De manhã'.
- Missed: `cafe`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `em uma escritório` -> `em um escritório` [word_choice] The correct word is 'escritório' (without 'a').

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `sou em casa` -> `estou em casa` [ser_estar] The correct expression is 'estou em casa'.

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `do Canadá` -> `do Canadá` [article_preposition] The preposition 'de' is not needed here.
- Missed: `moro Toronto`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Eu chamo` -> `Meu nome é` [word_choice] This phrase is not appropriate for answering 'What is your name?'

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `A minha amiga` -> `Minha amiga` [word_choice] The correct phrase is 'Minha amiga'.
- Missed: `chama`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No almoço` -> `Para o almoço` [word_choice] This phrase is not natural in this context.
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective needs to agree in gender with 'camisa': vermelha, not vermelha.
- Missed: `vermelha camisa`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol` -> `` [content_mismatch] This answer is irrelevant to the question about breakfast.

### a1-english-lunch (not_portuguese, a1)

- Task: What did you eat for lunch?
- Answer: I ate rice and chicken for lunch.
- Flags: written_in_portuguese=true answers_the_question=false

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `trabalhei` -> `trabalhei` [verb_form] The verb needs to be in the past: trabalhei.
- Missed: `portugues`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de onibus` -> `em ônibus` [word_choice] The correct preposition is 'em': 'em ônibus'.
- Missed: `cafe`
- Missed: `as nove`

### a2-verb-form-irregular-fazer (erroneous, a2)

- Task: What did you do yesterday evening?
- Answer: Ontem à noite eu fazi o jantar e depois fui dormir cedo.
- Reference: `fazi` -> fiz [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `fizi` -> `fiz` [verb_form] The verb 'fazer' needs to be conjugated in the past: fiz.
- Missed: `fazi`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective needs to agree in gender with 'amiga': engraçada.
- Missed: `curtas`

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `lavo` -> `lavo` [verb_form] The verb needs to be in the present tense: eu lavo.
- Missed: `lavo os dentes`

### a2-pronoun-reciprocal (erroneous, a2)

- Task: Describe your relationship with your neighbors.
- Answer: Eu me dou bem com meus vizinhos e sempre nós ajudamos quando precisamos.
- Reference: `nós ajudamos` -> nos ajudamos [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `me dou` -> `dou` [word_choice] The reflexive pronoun is not needed here.
- Missed: `nós ajudamos`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `autocarro` -> `autocarro` [spelling] The word needs an accent on the ‘o’: autocarro.
- Missed: `autocarro`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pequeno-almoço`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `está chovendo` [word_choice] The weather is raining, not sunny.

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós ficamos` -> `nós passamos` [word_choice] The correct expression is 'nós passamos' or 'nós estivemos'.

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `no mercado` -> `no mercado` [article_preposition] The preposition 'em' is needed before 'mercado'.

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `o seu` -> `seu` [word_choice] The pronoun 'o' is redundant here.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A gente` -> `Nós` [not_brazilian] This is a European Portuguese construction, not standard Brazilian Portuguese.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> `do filme` [word_choice] The pronoun 'dele' is redundant and can be removed.

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `com o meu cachorro` -> `com cachorro` [article_preposition] The preposition 'com' is not needed here.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro` -> `Eles estão comendo` [word_choice] This sentence is irrelevant to the task and uses the wrong verb form.

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `nunca tenho visitado` -> `nunca visitei` [verb_form] The correct expression is 'nunca visitei'.

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `você possa` -> `possa` [verb_form] The correct form is 'possa' without the 'você'.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ganhasse` -> `ganharia` [verb_form] The verb needs to be in the conditional: ganharia.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `` [pronoun] The pronoun 'nós' is not needed here.

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `gente` -> `colegas` [word_choice] The word 'gente' is not appropriate in this context.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Sr. Souza` -> `Sr. Souza` [spelling] The title needs an accent on the i: Sr. Souza.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `você` -> `` [pronoun] The pronoun is not needed here.
- Missed: `estuda`
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `do meu pedido` -> `do meu pedido` [word_choice] The phrase 'do meu pedido' is too informal for a formal complaint.
- Missed: `sobre do`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajarei` [verb_form] The future tense needs to be expressed correctly with 'viajarei'.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cheguei em casa morto de cansaço` -> `cheguei em casa exausto` [word_choice] This expression is not commonly used in Brazilian Portuguese.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `no dia 10` [word_choice] The correct expression is 'no dia 10'.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `a` [agreement] The correct pronoun to use here is 'a' because 'irmão' is masculine.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `preocupo` -> `preocupo` [verb_form] The verb needs to be in the first person: eu preocupo.
- Missed: `eu preocupo`

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
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `livros` -> `livro` [spelling] The word needs an accent on the i: livro.

### a1-word-choice-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu sou vinte anos velho.
- Reference: `sou vinte anos velho` -> tenho vinte anos [word_choice]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `vinte anos velho` -> `vinte anos` [word_choice] The correct phrase is 'vinte anos velho' which is incorrect.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `na prova de matemática` -> `na prova matemática` [word_choice] The phrase needs a contraction: 'na prova matemática'.
- Missed: `fiz um erro`

### a1-article-contraction-a-o (erroneous, a1)

- Task: Where did you go on Saturday?
- Answer: No sábado eu fui a o cinema com meus amigos.
- Reference: `a o` -> ao [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `a o` -> `ao` [article_preposition] The preposition ‘a’ is not needed before ‘o’ in this context.

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `irmão` -> `irmão` [spelling] The word needs an accent on the i: irmão.
- Missed: `é doente`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `pequeno` -> `pequeno` [spelling] The word needs an accent on the i: pequeno.
- Missed: `sempre fui`

### a1-word-order-article-noun (erroneous, a1)

- Task: Where do you live?
- Answer: Eu moro com meus pais em casa uma pequena.
- Reference: `casa uma pequena` -> uma casa pequena [word_order]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `em casa uma` -> `em minha` [article_preposition] The preposition 'em' is not needed here, and 'uma' is incorrect.

### a1-control-accents-present (control, a1)

- Task: What do you eat for breakfast?
- Answer: De manhã eu como pão com queijo e bebo café com leite.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `bebo café com leite` -> `bebo leite com café` [word_order] The order of the words is incorrect; it should be 'bebo leite com café'.

### a1-control-where-live (control, a1)

- Task: Where do you live?
- Answer: Moro no Brasil, numa cidade perto do mar.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Moro` -> `Eu moro` [word_choice] The verb 'morar' is more common in this context.

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Tô` -> `Estou` [word_choice] This is a colloquial form of 'estou' and is not appropriate for this level.

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tem` -> `tem` [word_choice] The sentence is correct, no mistake

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está` -> `Camila veste` [verb_form] The verb 'estar' is not needed here.

### a2-control-preterite-sunday (control, a2)

- Task: What did you do on Sunday?
- Answer: No domingo eu acordei tarde, fiz o almoço e depois assisti a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `assistí` -> `assistindo` [verb_form] The verb 'assistir' needs the gerund 'assistindo': assistindo a um filme.

### a2-control-pet (control, a2)

- Task: Describe your pet.
- Answer: Eu tenho um cachorro. O meu cachorro se chama Bolt e ele adora correr.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ele adora` -> `ele gosta` [word_choice] The correct expression is 'ele gosta' (he likes).

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `não vai trabalhar hoje` -> `não vai trabalhar amanhã` [word_choice] The correct expression is 'não vai trabalhar amanhã'.

### a2-control-family-meal (control, a2)

- Task: Describe the meal these people are eating.
- Answer: A família está comendo arroz, feijão e frango grelhado. Eles bebem suco de laranja.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `família` -> `família` [spelling] The word needs an accent on the i: família.

### a2-control-phone (control, a2)

- Task: Where is your phone?
- Answer: O meu celular está em cima do sofá.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `em cima` -> `em` [word_choice] The correct preposition is 'em': em cima is redundant.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `que eu passe` -> `para passar` [word_choice] The ‘que’ is redundant and can be removed.

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `pequeno` -> `pequeno` [spelling] The word needs an accent on the i: pequeno.

### b1-control-weather-hot (control, b1)

- Task: Describe the weather today.
- Answer: Hoje faz muito calor e o céu está sem nuvens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `sem nuvens` -> `sem nuvens` [word_choice] The correct expression is 'sem nuvens'

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Trabalhei` -> `Eu trabalhei` [verb_form] The correct phrase is 'Eu trabalhei' or 'Eu tenho trabalhado'.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `convidei-a` -> `convidei-la` [pronoun] The correct reflexive pronoun is 'a' not 'i'.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `eu estou` -> `Meu nome é...` [content_mismatch] This response answers a different question about the weather.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `comprei` -> `Eu comprei` [verb_form] The verb tense is incorrect for this context; it should be 'comprei' or 'eu comprei'.

### a2-english-coffee (not_portuguese, a2)

- Task: Do you like coffee?
- Answer: Yes, I like coffee a lot.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma3:4b / few_shot_six

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
- Found (matched): `De manha` -> `De manhã` [word_choice] The correct expression is 'De manhã'.
- Missed: `cafe`

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `estou brasileiro`

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `moro Toronto`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Response: invalid or missing

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
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective needs to agree in gender with 'camisa': vermelha.
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `gosto de café muito`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de` -> `` [content_mismatch] This is an irrelevant statement for the question.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `estudei` -> `estudei` [verb_form] The verb must agree with 'eu': estudei.
- Missed: `portugues`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de onibus` -> `em ônibus` [word_choice] The correct preposition is 'em': em ônibus.
- Missed: `cafe`
- Missed: `as nove`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-verb-form-imperfect (erroneous, a2)

- Task: Describe your childhood.
- Answer: Quando eu era criança, eu moro numa cidade pequena e brincava na rua.
- Reference: `moro` -> morava [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `morо` -> `vivi` [verb_form] The verb must agree with 'eu': vivi.
- Missed: `moro`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective needs to agree in gender with 'amiga': engraçada.
- Missed: `curtas`

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `passar` -> `passar` [verb_form] The correct verb form is 'passar', but it needs to be in the infinitive form.
- Missed: `por passar`

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
- Missed: `nós ajudamos`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `autocarro` -> `autocarro` [spelling] The word needs an accent on the ‘o’: autocarro.
- Missed: `autocarro`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pequeno-almoço`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `chovendo` [word_choice] The weather is raining, not sunny.

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ficamos` -> `ficamos` [verb_form] The verb must agree with 'nós': ficamos.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> `do filme` [word_choice] The pronoun needs to be 'do': do filme.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu` -> `Eles` [content_mismatch] The answer should describe the meal, not the speaker's location.

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Se eu ter`

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `ganhava`

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `você possa` -> `pode` [verb_form] The correct form is 'pode'.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `` [pronoun] The pronoun 'nós' is not needed here.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Sr. Souza` -> `Sr. Souza` [spelling] The title needs an accent mark: Sr. Souza.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `você` -> `a` [pronoun] The pronoun should be 'a' to agree with 'estuda'.
- Missed: `estuda`
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `do meu pedido` -> `do meu encomenda` [word_choice] The phrase 'do meu pedido' is too informal for a formal complaint.
- Missed: `sobre do`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajarei` [verb_form] The future tense needs to be 'viajarei'.

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu diverti`

### a1-short-agreement-house (erroneous, a1)

- Task: Describe your house.
- Answer: Minha casa é grande e bonito.
- Reference: `bonito` -> bonita [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `bonito` -> `bonito` [spelling] The adjective needs an accent mark: bonito.
- Missed: `bonito`

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

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sempre fui`

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pra` -> `para` [word_choice] The correct preposition is 'para'.

### a2-control-family-meal (control, a2)

- Task: Describe the meal these people are eating.
- Answer: A família está comendo arroz, feijão e frango grelhado. Eles bebem suco de laranja.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eles` -> `família` [pronoun] The correct pronoun to use is 'eles' to refer to 'a família'.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `que eu passe` -> `para passar` [word_choice] The relative clause is unnecessarily complex and can be simplified.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `eu estou` -> `` [content_mismatch] This sentence is about the weather, not the speaker's location.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `comprei` -> `Penso` [content_mismatch] The answer is about working from home, not buying a bicycle.

### a2-english-coffee (not_portuguese, a2)

- Task: Do you like coffee?
- Answer: Yes, I like coffee a lot.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma3:4b / one_shot_correct

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `bebo` -> `bebo` [verb_form] The verb 'beber' needs to be in the present tense to match 'eu'
- Missed: `manha`
- Missed: `cafe`

### a1-agreement-trousers (erroneous, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: Marina veste uma camiseta azul e uma calça pretos.
- Reference: `pretos` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pretos`

### a1-agreement-plural-adjective (erroneous, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos. Eles são muito simpático.
- Reference: `simpático` -> simpáticos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eles são` -> `Eles são simpáticos` [agreement] The correct form is 'Eles são simpáticos', not 'Eles são simpático'.
- Missed: `simpático`

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `estou brasileiro`

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `moro Toronto`

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
- Found (false positive): `A minha amiga` -> `Minha amiga` [word_choice] The phrase 'A minha amiga' is not needed here.
- Found (false positive): `e ela` -> `` [word_choice] The pronoun 'ela' is redundant because 'Maria' is already known.
- Missed: `chama`

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
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective 'vermelha' is feminine singular and doesn't agree with 'camisa' (which is masculine singular).
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `gosto de café muito`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol` -> `Eu como` [word_choice] This answer is about football, not breakfast.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `estudei` -> `estudei` [not_brazilian] ‘Estudei’ is European Portuguese; ‘estudei’ should be ‘estudei’ in Brazilian Portuguese.
- Missed: `portugues`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `cafe`
- Missed: `onibus`
- Missed: `as nove`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `era cansado`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `engraçado` -> `engraçado` [agreement] The adjective 'engraçado' needs to be masculine to agree with 'ele'.
- Missed: `engraçado`
- Missed: `curtas`

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

### a2-pronoun-reciprocal (erroneous, a2)

- Task: Describe your relationship with your neighbors.
- Answer: Eu me dou bem com meus vizinhos e sempre nós ajudamos quando precisamos.
- Reference: `nós ajudamos` -> nos ajudamos [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `nós ajudamos`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pequeno-almoço`

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A gente` -> `Eu` [word_choice] ‘A gente’ is informal and not appropriate for this level.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu` -> `Eles` [pronoun] This refers to the speaker, not the meal.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-verb-form-subjunctive-present (erroneous, b1)

- Task: What do you hope for the future?
- Answer: Espero que meus filhos são felizes e que eles tenham uma boa educação.
- Reference: `são` -> sejam [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `são`

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Se eu ter`

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `ganhava`

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `morro` -> `moro` [verb_form] The verb 'morar' needs to be conjugated in the present tense.
- Missed: `tenho visitado`

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `estuda`
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sobre do`

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu preocupo`

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
- Found (false positive): `ontem` -> `na` [word_choice] The correct word to use here is 'na', indicating the day of the week.
- Missed: `fiz um erro`

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `é doente`

### b1-verb-form-para-que (erroneous, b1)

- Task: Why do you study every day?
- Answer: Eu estudo todos os dias para que eu passo no exame.
- Reference: `eu passo` -> eu passe [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `que` -> `para` [word_choice] The ‘que’ is unnecessary and incorrect in this context.
- Missed: `eu passo`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sempre fui`

### a2-not-brazilian-phone (erroneous, a2)

- Task: Where is your phone?
- Answer: O meu telemóvel está em cima do sofá.
- Reference: `telemóvel` -> celular [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `telemóvel`

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pra` -> `para` [not_brazilian] ‘Pra’ is European Portuguese for ‘to’, but ‘para’ is correct in Brazilian Portuguese.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Trabalhei` -> `Eu trabalhei` [verb_form] ‘Trabalhei’ is the 3rd person singular, it needs to be ‘Eu trabalhei’

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje` -> `` [word_choice] This word means 'today' and is not relevant to the question.
- Found (false positive): `eu estou` -> `` [word_choice] This phrase is not relevant to the question about the learner's name.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `comprei` -> `penso` [content_mismatch] This refers to Saturday, not working from home.

### a2-english-coffee (not_portuguese, a2)

- Task: Do you like coffee?
- Answer: Yes, I like coffee a lot.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma3:4b / one_shot_mistake

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A Marina está` -> `Marina usa` [ser_estar] The verb ‘estar’ is not needed here, ‘Marina usa’ or ‘Marina tem’ would be more appropriate.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô` -> `Estou` [verb_form] This is a colloquial form of 'estou' and is not appropriate for this level.

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu me chamo` -> `Meu nome é` [word_choice] This phrase is not appropriate for answering the question 'What is your name?'

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `feijão` -> `feijão` [spelling] The word needs an accent on the i: feijão.
- Missed: `fejão`

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha` -> `De manhã` [not_brazilian] This is European Portuguese for 'In the morning'.
- Missed: `cafe`

### a1-agreement-trousers (erroneous, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: Marina veste uma camiseta azul e uma calça pretos.
- Reference: `pretos` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `veste` -> `usa` [word_choice] The verb 'vestir' is not appropriate here; 'usar' is more common.
- Missed: `pretos`

### a1-agreement-plural-adjective (erroneous, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos. Eles são muito simpático.
- Reference: `simpático` -> simpáticos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `são` -> `são` [verb_form] The verb needs to agree with the subject: eles são.
- Missed: `simpático`

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `do` -> `de` [article_preposition] The preposition ‘de’ is needed before ‘Canadá’
- Missed: `moro Toronto`

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A minha amiga` -> `Minha amiga` [word_choice] The correct phrase is 'Minha amiga' when referring to a single friend.
- Missed: `chama`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No` -> `No` [spelling] The correct preposition is ‘no’ (with no accent mark).
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective needs to agree in gender with camisa: vermelha, not vermelha.
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `eu` -> `` [pronoun] The pronoun is not needed here.
- Missed: `gosto de café muito`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol` -> `Eu comi...` [content_mismatch] This is about football, not breakfast.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `trabalhei` -> `trabalhei` [verb_form] The verb needs to be in the past tense: trabalhei.
- Missed: `portugues`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de onibus` -> `em ônibus` [word_choice] The correct preposition is ‘em’ before ‘ônibus’
- Missed: `cafe`
- Missed: `as nove`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `visitei` -> `visitei` [verb_form] The verb needs to be in the present tense: visitei.
- Missed: `volti`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective needs to agree in gender with the noun: engraçada.
- Missed: `curtas`

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `eu` -> `` [pronoun] The pronoun 'eu' is redundant when 'de manhã' is used.
- Missed: `lavo os dentes`

### a2-pronoun-reciprocal (erroneous, a2)

- Task: Describe your relationship with your neighbors.
- Answer: Eu me dou bem com meus vizinhos e sempre nós ajudamos quando precisamos.
- Reference: `nós ajudamos` -> nos ajudamos [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `me dou` -> `me relaciono` [word_choice] The correct reflexive construction is ‘me dou’ but it’s better to use ‘me relaciono’ or ‘me dou bem’
- Missed: `nós ajudamos`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tomo` -> `faço` [word_choice] The verb 'tomar' is not appropriate for 'have' in this context; 'tomar' means 'to drink'.
- Missed: `pequeno-almoço`

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para o Rio de Janeiro` -> `ao Rio de Janeiro` [article_preposition] The preposition ‘para’ is not needed here.

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `no sábado` -> `no sábado` [spelling] The correct preposition is 'no' (with no accent mark).

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `o seu` -> `seu` [article_preposition] The pronoun ‘o’ is unnecessary and incorrect here.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A gente` -> `Nós` [not_brazilian] This is a European Portuguese construction, not standard Brazilian Portuguese.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> `` [word_choice] The pronoun 'dele' is not needed here.

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Em` [article_preposition] The correct preposition is ‘Em’.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro` -> `Eles estão` [verb_form] This sentence is irrelevant to the task and uses the wrong verb form.

### b1-verb-form-subjunctive-present (erroneous, b1)

- Task: What do you hope for the future?
- Answer: Espero que meus filhos são felizes e que eles tenham uma boa educação.
- Reference: `são` -> sejam [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `são`

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `morro` -> `morei` [verb_form] The verb ‘morro’ is incorrect; the correct form is ‘morei’ in the past tense.
- Missed: `tenho visitado`

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `você possa` -> `possa` [verb_form] The correct form is ‘possa’ without the ‘você’

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The correct form is 'ganhasse' (conditional), not 'ganhasse'.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `eu` [pronoun] The pronoun is incorrect here; it should be 'eu'.

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `gente` -> `pessoas` [not_brazilian] The use of ‘gente’ is not appropriate for this level.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Sr. Souza` -> `Senhor Souza` [word_choice] It's more formal to use 'Senhor Souza' instead of 'Sr. Souza'.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `você` -> `estuda` [pronoun] The pronoun is not needed here, it's more natural to say 'estuda'
- Missed: `estuda`
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `do meu pedido` -> `do meu pedido` [word_choice] The correct expression is 'do meu pedido'.
- Missed: `sobre do`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajará` [verb_form] The verb needs to be in the present tense: viajará.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `cheguei` -> `cheguei` [verb_form] The verb needs to be in the present perfect: cheguei.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `no dia 10` [word_choice] The correct expression is ‘no dia 10’ (day ten).

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `a` [pronoun] The correct pronoun to use here is 'a' or 'ao', not 'lhe'.

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu diverti`

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `é doente`

### a1-control-accents-present (control, a1)

- Task: What do you eat for breakfast?
- Answer: De manhã eu como pão com queijo e bebo café com leite.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `eu como` -> `Eu como` [verb_form] The verb 'comer' needs to be in the present tense, not the present continuous.

### a1-control-where-live (control, a1)

- Task: Where do you live?
- Answer: Moro no Brasil, numa cidade perto do mar.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Moro` -> `Morro` [word_choice] The verb 'morar' is more common in this context, 'morro' is also correct.

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô` -> `Estou` [verb_form] The learner should use 'Estou' instead of 'Tô' in this context.

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Na minha` -> `Em minha` [word_choice] The correct preposition is ‘na’

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está` -> `usa` [verb_form] The verb ‘estar’ is not needed here.

### a2-control-pet (control, a2)

- Task: Describe your pet.
- Answer: Eu tenho um cachorro. O meu cachorro se chama Bolt e ele adora correr.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ele adora` -> `ele adora` [spelling] The word needs an accent on the i: ele adora.

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está` -> `é` [ser_estar] The correct verb to use here is 'é' because it's a statement about his state.

### a2-control-family-meal (control, a2)

- Task: Describe the meal these people are eating.
- Answer: A família está comendo arroz, feijão e frango grelhado. Eles bebem suco de laranja.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `família` -> `família` [spelling] The word needs an accent on the i: família.

### a2-control-phone (control, a2)

- Task: Where is your phone?
- Answer: O meu celular está em cima do sofá.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `em cima` -> `em` [word_choice] The correct preposition is 'em': em cima do sofá.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para que eu passe` -> `a fim de que eu passe` [word_choice] The phrase ‘para que’ is often replaced by ‘a fim de’ or ‘com o objetivo de’ at this level.

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `eu sempre ia` -> `eu ia` [word_choice] The expression ‘eu sempre ia’ is overly emphatic and not natural for this context.

### b1-control-weather-hot (control, b1)

- Task: Describe the weather today.
- Answer: Hoje faz muito calor e o céu está sem nuvens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `sem nuvens` -> `céu limpo` [word_choice] The expression ‘sem nuvens’ is correct, but it is more natural to say ‘céu limpo’ in this context.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Trabalhei` -> `Trabalhei` [verb_form] The verb needs to be in the present tense to answer the question 'How was your day?','kind':'verb_form','replacement':'Trabalhei'}]} 1```jsonRepairing the response to accurately answer the question.```

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `convidei-a` -> `eu a convidei` [pronoun] The verb 'convidar' needs the subject pronoun 'eu': eu a convidei.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `está chovendo` -> `Meu nome é...` [content_mismatch] This is a description of the weather, not a response to the question.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `comprei` -> `penso` [verb_form] This sentence is about opinions, not a past action.

## gemma3:4b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A Marina está` -> `Marina está` [article_preposition] The use of 'A' before Marina is incorrect.

### a1-control-coffee (control, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Sim` -> `Eu` [word_choice] ‘Sim’ is not needed at the beginning of the answer.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô` -> `Estou` [not_brazilian] ‘Tô’ is European Portuguese for ‘I am’, but ‘I’m’ is correct in Brazilian Portuguese.
- Found (changes nothing): `e você?` -> `e você?` [word_choice] The word order is correct, but the question is a bit unnatural.

### a1-control-daily-routine (control, a1)

- Task: What do you do every day?
- Answer: Todo dia eu acordo cedo, tomo café e vou para o trabalho.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `eu` -> `Todo dia eu acordo cedo, tomo café e vou para o trabalho.` [word_choice] The pronoun 'eu' is redundant after 'Todo dia'.

### a1-control-order-meal (control, a1)

- Task: You are at a restaurant. Order your meal.
- Answer: Eu queria uma salada e um suco de laranja, por favor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu queria` -> `Eu quero` [word_choice] This is a more formal way to say 'I want' than what would be typical for an A1 learner.

### a1-control-family (control, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos e uma irmã. Meu pai é médico e minha mãe é professora.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `irmã` -> `irmão` [spelling] The learner should use 'irmão' to describe a male sibling.

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu` -> `Me` [word_choice] The question asks for a name, not a self-introduction.

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `fejão` -> `feijão` [spelling] ‘Feijão’ is the correct spelling of ‘bean’ in Brazilian Portuguese.
- Found (false positive): `é` -> `gosto` [verb_form] The verb ‘ser’ is used incorrectly here; ‘é’ is the third person singular of ‘ser’ and is not appropriate for expressing a preference.

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `De` -> `De manhã` [word_choice] The correct expression is 'De manhã'.
- Missed: `manha`
- Missed: `cafe`

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estou` -> `sou` [verb_form] ‘Estou’ is incorrect here; ‘Sou’ is needed to state origin.
- Found (false positive): `em Recife` -> `Recife` [article_preposition] ‘Em’ is an unnecessary preposition here.

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu sou` -> `Eu sou de` [word_choice] The correct response is 'Eu sou de...' not 'Eu sou do...'
- Missed: `moro Toronto`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `Eu chamo` -> `Qual é o seu nome?` [word_choice] The correct phrase to ask for someone's name is 'Qual é o seu nome?'
- Found (false positive): `e tenho vinte anos.` -> `` [content_mismatch] The question asks for the name, not the age.

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A` -> `Minha` [spelling] The demonstrative adjective 'A' is incorrect here; it should be 'Minha'.
- Missed: `chama`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Eu` [article_preposition] ‘No’ is not the correct way to start this sentence.
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective 'vermelha' is feminine singular, but 'camisa' is masculine singular.
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Sim` -> `Eu` [word_choice] ‘Sim’ is not needed at the beginning of the answer.
- Missed: `gosto de café muito`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu gosto de futebol` -> `Eu comi` [content_mismatch] This response answers a question about breakfast, not a preference for football.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `trabalhei` -> `trabalhei` [not_brazilian] The verb 'trabalhar' is used in European Portuguese, while 'trabalhei' is used in Brazilian Portuguese.
- Missed: `portugues`

### a2-spelling-agua (erroneous, a2)

- Task: What did you buy at the market?
- Answer: No mercado eu comprei maçãs, dois quilos de arroz e uma garrafa de agua.
- Reference: `agua` -> água [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No mercado` -> `Eu` [word_order] The sentence needs an introductory verb.
- Found (matched): `agua` -> `água` [spelling] The correct word for 'water' in Brazilian Portuguese is 'água'.

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de onibus` -> `em ônibus` [word_choice] The correct preposition to use with 'ônibus' is 'em'.
- Missed: `cafe`
- Missed: `as nove`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Finalmente` [article_preposition] ‘No’ is not the correct way to start this sentence in Brazilian Portuguese.
- Missed: `volti`

### a2-verb-form-subject-agreement (erroneous, a2)

- Task: What did your family do on Sunday?
- Answer: No domingo minha família almoçaram na casa da minha avó.
- Reference: `almoçaram` -> almoçou [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `almoçaram` -> `almoçaram` [verb_form] The verb 'almoçar' needs to agree with the subject 'minha família' (plural).
- Missed: `almoçaram`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `Eu era cansado` -> `Eu estava cansado` [verb_form] The past tense 'cansado' is needed here, not the present tense 'era'.
- Found (false positive): `e dormi o dia todo` -> `e dormi o dia inteiro` [word_choice] The correct way to express 'slept all day' is 'dormi o dia inteiro'.

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `engraçado` -> `engraçado` [agreement] The adjective 'engraçado' needs to be masculine to agree with 'amiga'.
- Found (matched): `cabelos curtas` -> `cabelos` [spelling] The plural form of 'cabelo' is 'cabelos', not 'cabelos curtas'.
- Missed: `engraçado`

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `eu` -> `De manhã eu` [word_choice] The pronoun 'eu' is redundant when 'De manhã' is used.
- Missed: `lavo os dentes`

### a2-pronoun-reciprocal (erroneous, a2)

- Task: Describe your relationship with your neighbors.
- Answer: Eu me dou bem com meus vizinhos e sempre nós ajudamos quando precisamos.
- Reference: `nós ajudamos` -> nos ajudamos [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Eu me dou` -> `Eu me dou` [verb_form] The reflexive pronoun 'me' needs to be combined with 'ar' or 'er' to form the reflexive verb 'me dou'.
- Missed: `nós ajudamos`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu tomo` -> `Eu como` [word_choice] ‘Eu tomo’ is overly formal for this level; ‘Eu como’ or ‘Eu faço’ are more natural.
- Missed: `pequeno-almoço`

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No` -> `No` [article_preposition] ‘No’ is not the correct way to start the sentence in this context.

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Na` [word_choice] The correct word to start the sentence is 'Na'.

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `o seu` -> `o meu` [agreement] The pronoun 'o seu' is incorrect here; it should be 'o meu'.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Siga` -> `Vá` [word_choice] ‘Siga’ is a more formal command, ‘Vá’ is more appropriate for a friend.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A gente` -> `Nós` [word_choice] ‘A gente’ is not appropriate in this context; ‘Nós’ is more natural.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> `do filme` [word_choice] The pronoun 'dele' is unnecessary and redundant in this context.

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `No meu tempo` [word_choice] The correct introductory phrase is 'No meu tempo'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu moro` -> `Eles` [word_choice] This phrase means 'I live' and is irrelevant to the task.

### b1-verb-form-subjunctive-present (erroneous, b1)

- Task: What do you hope for the future?
- Answer: Espero que meus filhos são felizes e que eles tenham uma boa educação.
- Reference: `são` -> sejam [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `eles` -> `eles` [word_choice] The pronoun 'eles' is redundant as 'meus filhos' already refers to the children.
- Missed: `são`

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu moro aqui` -> `Eu moro aqui há` [word_choice] The correct phrase to use when asking about length of time living somewhere is 'há' + time period.
- Missed: `tenho visitado`

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `você` -> `tu` [word_choice] The pronoun 'você' is overly formal for a B1 level invitation.

### b1-control-future-subjunctive (control, b1)

- Task: What will you do when you finish the course?
- Answer: Quando eu terminar o curso, vou procurar um emprego novo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `terminar` -> `concluir` [verb_form] The verb 'terminar' is used incorrectly here; 'concluir' is more appropriate.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The conditional form of 'ganhar' (to win) needs the subjunctive mood after 'se'.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `eu` [pronoun] The correct pronoun to use in this context is 'eu'.

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a gente` -> `eu` [word_choice] ‘A gente’ is informal and not appropriate for a B1 level response.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `na próxima sexta-feira` -> `na próxima semana` [word_choice] It's more natural to say 'na próxima semana' (next week) instead of 'no próximo dia sexta-feira' (on the next Friday).

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `você` -> `tu` [word_choice] The pronoun 'você' is generally considered too formal for most contexts, especially when recommending something to a friend.
- Missed: `estuda`
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `do atraso` -> `do atraso` [word_choice] ‘Do atraso’ is not a correct phrase in this context; ‘do atraso’ is a noun, not a correct way to express ‘of the delay’.
- Missed: `sobre do`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajará` [verb_form] The future tense is incorrect; 'viajará' is needed.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `apagar vários incêndios` -> `combater` [word_choice] ‘Apagar’ is not the most natural expression for ‘putting out’ fires in this context.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `o décimo dia` [word_choice] The correct way to express 'the tenth' is 'o décimo dia'.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `a ele` [word_choice] ‘Lhe’ is overly formal; ‘a ele’ is more appropriate in this context.

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `eu` -> `eu` [word_choice] The pronoun 'eu' is not needed after 'diverti'.
- Missed: `eu diverti`

### a1-short-verb-morar (erroneous, a1)

- Task: Where do you live?
- Answer: Eu mora em Lisboa.
- Reference: `mora` -> moro [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Eu` -> `Eu` [pronoun] The correct pronoun to use is 'Eu' when stating where you live.
- Missed: `mora`

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Ontem` -> `Na` [word_choice] The question asks about 'your test', so 'yesterday' is not appropriate.
- Missed: `fiz um erro`

### a2-not-brazilian-phone (erroneous, a2)

- Task: Where is your phone?
- Answer: O meu telemóvel está em cima do sofá.
- Reference: `telemóvel` -> celular [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `O` -> `O meu` [agreement] The correct article for 'meu' is 'o'.
- Found (matched): `telemóvel` -> `celular` [word_choice] While understandable, 'telemóvel' is a loanword and less common than 'celular' in Brazil.

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tenho` -> `tenho` [verb_form] The verb 'ter' needs to be conjugated in the present tense for 'I'.

### a1-control-accents-present (control, a1)

- Task: What do you eat for breakfast?
- Answer: De manhã eu como pão com queijo e bebo café com leite.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `eu` -> `como` [word_choice] The pronoun 'eu' is unnecessary and redundant in this context.

### a1-control-where-live (control, a1)

- Task: Where do you live?
- Answer: Moro no Brasil, numa cidade perto do mar.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Moro` -> `Vivo` [verb_form] ‘Moro’ is the correct form of ‘vivar’ (to live) for ‘I’ in this context.

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pra` -> `para` [article_preposition] The correct preposition is 'para'.

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Na` -> `Em` [word_choice] The correct preposition is 'Em'.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A Camila` -> `Camila` [word_choice] The pronoun 'A' is not needed at the beginning of the sentence.

### a2-control-preterite-sunday (control, a2)

- Task: What did you do on Sunday?
- Answer: No domingo eu acordei tarde, fiz o almoço e depois assisti a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Na` [article_preposition] The correct pronoun to use here is 'Na'.

### a2-control-imperfect-grandmother (control, a2)

- Task: Describe your grandmother's house.
- Answer: A casa da minha avó era pequena, mas tinha um jardim grande com muitas flores.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A casa` -> `Casa` [article_preposition] The article 'A' is unnecessary here as it's the first mention of the house.

### a2-control-reflexive-party (control, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu me diverti muito.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `A festa` -> `A festa` [article_preposition] The article 'A' is unnecessary here as it's referring to a general party.

### a2-control-pet (control, a2)

- Task: Describe your pet.
- Answer: Eu tenho um cachorro. O meu cachorro se chama Bolt e ele adora correr.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ele adora correr` -> `ele gosta de correr` [word_choice] The correct phrase is 'ele gosta de correr' (he likes to run).

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está` -> `é` [verb_form] The verb 'estar' is used incorrectly; 'está' is the third person singular form of 'estar', but the subject is 'irmão' (brother), which is masculine singular.

### a2-control-family-meal (control, a2)

- Task: Describe the meal these people are eating.
- Answer: A família está comendo arroz, feijão e frango grelhado. Eles bebem suco de laranja.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A família` -> `A família está` [word_choice] ‘A família’ is incorrect; it should be ‘A família’ or ‘A família está’.
- Found (false positive): `bebem` -> `bebe` [verb_form] The verb ‘bebem’ is incorrect; it should be ‘bebe’ to agree with ‘eles’.

### a2-control-phone (control, a2)

- Task: Where is your phone?
- Answer: O meu celular está em cima do sofá.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `O` -> `O meu` [spelling] The correct article for 'celular' is 'o' because it starts with a consonant.

### a2-control-cinema-contraction (control, a2)

- Task: Where did you go on Saturday?
- Answer: No sábado eu fui ao cinema com meus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Na` [word_choice] The correct introductory word is 'Na'.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para que eu passe` -> `para passar` [word_choice] This phrase is overly formal and not typical for a B1 level learner.

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `eu sempre ia` -> `eu ia` [word_choice] The use of 'eu sempre ia' is overly emphatic and not natural in this context.

### b1-control-gostar-de (control, b1)

- Task: What do you like about the city?
- Answer: Eu gosto da cidade porque é tranquila e as pessoas são simpáticas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `é` -> `e` [spelling] The pronoun 'é' is incorrect here; it should be 'e'.

### b1-control-weather-hot (control, b1)

- Task: Describe the weather today.
- Answer: Hoje faz muito calor e o céu está sem nuvens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `sem nuvens` -> `nublado` [word_choice] ‘Sem nuvens’ is a literal translation of ‘without clouds’ and doesn’t sound natural in this context.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Trabalhei` -> `Eu trabalhei` [word_choice] ‘Trabalhei’ is too formal for this level and context.

### b2-control-news (control, b2)

- Task: Comment on the news.
- Answer: As notícias de hoje são preocupantes, mas acredito que a situação possa melhorar.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `notícias` -> `a notícia` [word_choice] The correct word is ‘notícias’ (plural), but in this context, ‘a notícia’ (the news) would be more appropriate.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `convidei-a` -> `convidei-a para` [verb_form] The past perfect tense is not needed here.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está` -> `é` [ser_estar] ‘Está’ is used for location, not for describing a state of being.
- Found (false positive): `eu estou` -> `estou` [word_choice] This is an unnecessary and slightly awkward phrasing in this context.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu comprei` -> `Eu` [content_mismatch] This is about working from home, not buying a bicycle.
- Found (false positive): `uma bicicleta nova no sábado` -> `` [content_mismatch] This is irrelevant to the question about working from home.

### a2-english-coffee (not_portuguese, a2)

- Task: Do you like coffee?
- Answer: Yes, I like coffee a lot.
- Flags: written_in_portuguese=true answers_the_question=true

## qwen3.5:latest / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está` -> `` [word_choice] To describe what someone is wearing, the preposition 'de' is used without a verb.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cansado` -> `cansada` [agreement] The adjective must agree in gender with the implied speaker (usually 'eu' is male or female, but 'cansado' is masculine; if the learner implies being female, it should be 'cansada'. However, without explicit gender context, 'cansado' is often accepted for males. But strictly speaking, if the learner is female, this is an agreement error. Given A1 level and lack of gender specification in the prompt, we assume standard male default or that the user might be female. Let's look closer: 'Só um pouco cansado'. If the speaker is female, it's wrong. If male, it's right. Since gender isn't specified, we can't definitively call it a mistake unless we assume a specific gender. However, often in these tasks, if the user doesn't specify gender, 'cansado' is treated as correct for a generic male speaker or the learner is assumed male. Wait, actually, looking at previous examples, agreement errors are reported. Is there a missing pronoun? No. Is 'tô' allowed? Yes (spoken form). Is 'e você?' okay? Yes. The only potential issue is gender agreement on 'cansado'. Without knowing the learner's gender, we cannot flag it as an error definitively. BUT, often in these datasets, if the answer doesn't match a specific persona provided in facts (none here), we might skip. Let's re-read the rules: 'Do not report a mistake you are not sure about.' Since I don't know the learner's gender, I shouldn't report 'cansado' as an error. Is there any other mistake? No. So the list should be empty? Or is 'Só um pouco cansado' implying the speaker is male? Usually, yes. Let's assume it's correct for a male speaker. If the task implies describing oneself and no gender is given, standard practice is to accept masculine as default or not flag it. Wait, looking at the previous turn where 'irmãs são muito alta' was flagged. That was clear. Here, without facts about the learner's gender, I cannot assume it's wrong. So mistakes: [].

### a1-control-order-meal (control, a1)

- Task: You are at a restaurant. Order your meal.
- Answer: Eu queria uma salada e um suco de laranja, por favor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `queria` -> `quero` [verb_form] In a restaurant context, the present tense 'quero' is used to make an order, not the imperfect 'queria'.

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
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent on the e: café.
- Missed: `manha`

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

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chama`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `muito` -> `muito` [word_order] The adverb 'muito' is misplaced; it should come before the verb or after 'gosto'.
- Missed: `gosto de café muito`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `comi` [verb_form] The verb tense is wrong; the question asks about the past (breakfast), but 'gosto' is present tense.
- Found (false positive): `futebol` -> `pão` [word_choice] This word does not answer the question about what was eaten for breakfast.

### a1-english-lunch (not_portuguese, a1)

- Task: What did you eat for lunch?
- Answer: I ate rice and chicken for lunch.
- Flags: written_in_portuguese=false answers_the_question=false

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `avós` -> `avôs` [spelling] The noun 'avô' is masculine, so the plural must be 'avôs', not 'avós'.
- Missed: `volti`

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
- Found (matched): `fazendo` -> `está` [word_choice] The verb 'fazer' is not used to describe weather in Brazilian Portuguese; use 'está'.
- Found (duplicate): `sol` -> `chovendo` [content_mismatch] The scene says it is raining, but the answer says it is sunny.
- Found (duplicate): `calor` -> `frio` [content_mismatch] The scene says it is cold, but the answer says it is hot.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `comeu` -> `comemos` [verb_form] The verb must agree with the plural subject 'A gente' (which functions as 'we').

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `moro` -> `Eles comem` [content_mismatch] The sentence does not describe the meal being eaten as requested.
- Found (false positive): `São Paulo` -> `espaguete` [content_mismatch] The sentence does not describe the meal being eaten as requested.
- Found (false positive): `cinco anos` -> `vinho tinto` [content_mismatch] The sentence does not describe the meal being eaten as requested.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `tenho visitado` -> `tive visitado` [verb_form] The present perfect tense is not used in Brazilian Portuguese; the simple past (pretérito perfeito) should be used instead.
- Found (duplicate): `nunca tenho visitado o Nordeste` -> `` [content_mismatch] This statement contradicts the implied context of living there since 2019 without mentioning travel, but strictly based on the task 'How long have you lived here?', this sentence is irrelevant content.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `loteria` -> `loteria` [spelling] The word needs an accent on the i: loteria.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `` [pronoun] The subject pronoun 'nós' is unnecessary and creates a redundancy with the verb form.

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `viajaria` -> `viajaria` [verb_form] The future tense 'viagem' is needed to match the original statement's future meaning, not the conditional 'viajaria'.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morto` -> `cansado` [agreement] The adjective 'cansado' must agree in gender with the subject 'eu' (male).

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `para ele` [not_brazilian] In spoken Brazilian Portuguese, the indirect object pronoun 'lhe' is often replaced by 'para ele' or omitted in this context to avoid ambiguity with the direct object 'me'. While not strictly ungrammatical in formal writing, it is considered a European Portuguese influence or overly formal for A2/B2 learners describing casual speech. However, a more common error pattern here is actually the use of 'lhe' when 'para ele' is preferred in Brazil for clarity, but strictly speaking, 'lhe' is not a *mistake* in standard grammar unless we enforce spoken norms. Let's re-evaluate based on strict Brazilian norms: 'Eu disse para ele que...' is much more common than 'Eu lhe disse...'. But is it an error? Actually, the bigger issue might be the subjunctive 'esperasse' which is correct for past tense indirect speech. Wait, looking at the previous examples, the model seems to flag Europeanisms or agreement errors. Is 'lhe' considered a mistake here? In Brazil, 'lhe' is often avoided in favor of 'para ele' or rephrasing. Let's look closer. Actually, there is no hard grammatical error here for B2 level, but 'lhe' can be seen as formal/European. However, the most likely intended correction for a learner is often about word choice or clarity. Let's reconsider if there are NO mistakes. In Brazil, saying 'Eu lhe disse' is acceptable in writing and formal speech. Is it a mistake? Maybe not. What about 'esperasse'? Correct subjunctive. 'ouviu'? Correct past tense. Maybe the answer has no mistakes? But usually these tasks have one. Let's look at 'lhe' again. In Brazil, using 'lhe' as an indirect object pronoun is standard in formal contexts but often replaced by 'para ele' in informal. The prompt doesn't specify register. However, if I must find a mistake, 'lhe' is the only candidate for being 'not_brazilian' (European preference) or just less common. But wait, is there a simpler error? No. Let's assume 'lhe' is acceptable. Is there another error? 'Minhas irmãs são muito alta' in the previous example was agreement. Here everything seems correct grammatically. Perhaps the system expects 'para ele'. Let's check the rules: 'not_brazilian: a form or construction used only in European Portuguese'. 'Lhe' is not *only* European, but its usage frequency differs. Actually, I might be overthinking. Let's look at the sentence again. 'Eu lhe disse que não me esperasse'. This is perfectly valid Portuguese. Maybe there are no mistakes? But the previous examples always had one. Let's reconsider 'lhe'. In Brazil, it is very common to say 'Eu disse para ele' instead of 'Eu lhe disse'. Using 'lhe' might be flagged as 'not_brazilian' in the context of learner correction focusing on spoken norms or strong Brazilian preference. I will flag 'lhe' as not_brazilian because in Brazil, the construction with 'para + pronoun/name' is strongly preferred over 'lhe' for clarity and informality, whereas 'lhe' is more formal/European style in this specific indirect object context.

### b2-pronoun-clitic-gender (erroneous, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, eu falei com a Ana e convidei-o para a festa.
- Reference: `convidei-o` -> convidei-a [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `convidei-o` -> `convidei-o` [spelling] The pronoun 'o' should be attached with a hyphen to the verb in written Portuguese.
- Missed: `convidei-o`

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Ontem` -> `(none)` [content_mismatch] The task asks about the past event on 'your test', but the answer describes a different event ('yesterday') that contradicts the implied context of answering a specific question.
- Missed: `fiz um erro`

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
- Found (false positive): `pequeno` -> `pequenos` [agreement] The speaker uses 'meus avós' (grandparents), implying they are male, so the adjective should be masculine plural.
- Missed: `sempre fui`

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `marrons` -> `marrom` [spelling] The adjective 'marrom' is invariable and does not take a plural form.

### a2-control-imperfect-grandmother (control, a2)

- Task: Describe your grandmother's house.
- Answer: A casa da minha avó era pequena, mas tinha um jardim grande com muitas flores.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `era` -> `é` [verb_form] The scene implies a current description, so the present tense 'é' is needed instead of the past tense 'era'.

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pequeno` -> `pequena` [agreement] The adjective must agree in gender with the subject 'eu' (implied male or female, but 'pequena' is standard for a generic female speaker or if the learner implies female; however, without context of gender, 'pequeno' is often accepted for males. But wait, looking at the previous examples, strict agreement is expected. If the user doesn't specify gender, usually 'pequeno' is fine for male, 'pequena' for female. Let's look closer. Is there a mistake? Actually, without knowing the speaker's gender, 'pequeno' might be assumed male. But often in exercises, if gender isn't specified, it's safer to assume the learner might have made a gender error or the system expects 'pequena' if the default is female? No, that's not a rule. Let's re-evaluate. Is there another mistake? 'ia à praia' is correct. 'com meus avós' is correct. Maybe the mistake is assuming the speaker was male when they might be female? Or is it actually correct? Wait, if the answer is from a generic learner, maybe the system expects 'pequena' by default? No. Let's look for other errors. Ah, perhaps there are no mistakes? But I must find one if possible. Let's reconsider 'pequeno'. If the speaker is female, it's wrong. If male, it's right. Since gender isn't stated, maybe it's not a mistake. Is there a spelling error? No. Verb form? 'ia' (imperfect) is correct for childhood habits. Maybe the task implies describing *your* childhood, and if the learner wrote 'pequeno', it's fine for a male. If the system considers the default persona female, then 'pequena'. But usually, we don't assume gender. Let's look at the previous examples. Example 3 had 'irmãs são muito alta' -> 'altas'. So agreement is key. Here, 'pequeno' vs 'pequena'. Without context, I cannot be sure it's a mistake. However, often in these datasets, if the gender isn't specified, and the word is an adjective describing a person, sometimes the 'female' form is used as a default or the user is assumed female? No, that's not standard. Let's assume there are NO mistakes unless I am sure. But the instruction says 

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está chovendo muito e eu estou em casa.` -> `Meu nome é [Name].` [content_mismatch] The answer describes the weather and location instead of stating the name.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei uma bicicleta nova no sábado.` -> `` [content_mismatch] The sentence describes a past action unrelated to the question about opinions on working from home.

## qwen3.5:latest / few_shot_in_system

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cansado` -> `cansada` [agreement] The adjective must agree in gender with the speaker (implied 'eu'), so it should be feminine.

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
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent on the e: café.
- Missed: `manha`

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

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chama`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `comi` [verb_form] The task asks about the past (what did you eat), but 'gosto' is present tense.
- Found (false positive): `futebol` -> `pão` [word_choice] This word does not answer the question about what was eaten for breakfast.

### a1-english-lunch (not_portuguese, a1)

- Task: What did you eat for lunch?
- Answer: I ate rice and chicken for lunch.
- Flags: written_in_portuguese=true answers_the_question=false

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent mark: café.
- Found (matched): `onibus` -> `ônibus` [spelling] The word needs an accent mark: ônibus.
- Missed: `as nove`

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
- Found (matched): `engraçado` -> `engradada` [agreement] The adjective must agree in gender with 'amiga' (feminine).
- Missed: `curtas`

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

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `pequeno-almoço` -> `pequeno-almoço` [spelling] The word needs a til on the 'a': pequeno-almoço.
- Missed: `pequeno-almoço`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo` -> `chovendo` [word_choice] The verb 'fazer' is not used to describe weather in Brazilian Portuguese; 'está chovendo' should be used instead.
- Found (duplicate): `sol` -> `chuva` [content_mismatch] The scene states it is raining, but the answer says there is sun.
- Found (duplicate): `calor` -> `frio` [content_mismatch] The scene states it is cold, but the answer says it is hot.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `comeu` -> `comemos` [verb_form] The verb must agree with the plural subject 'a gente' (we).

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `` [content_mismatch] The answer describes the speaker's life in a city instead of describing the meal being eaten by two people at a restaurant.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=true answers_the_question=false

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `` [pronoun] The subject pronoun 'nós' is unnecessary and creates a redundancy with the verb form.

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `tinha começado` -> `tinha começado` [verb_form] The past perfect tense requires the auxiliary verb 'ter' followed by the past participle 'começado'.
- Missed: `tinha começou`

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] After 'recomendo que', the verb must be in the subjunctive mood.
- Found (matched): `assiste` -> `assistas` [verb_form] After 'recomendo que', the verb must be in the subjunctive mood.
- Found (changes nothing): `exige` -> `exige` [verb_form] The subject 'isso' is singular, so the verb should be singular to agree with it.
- Missed: `exige`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viagem` [verb_form] The past tense 'viajaria' implies uncertainty or a conditional mood, but the scene states she definitely will travel.
- Found (false positive): `tinha comprado` -> `comprou` [verb_form] The imperfect 'tinha' suggests an ongoing action or lack of completion, whereas the scene states the tickets were already bought (completed action).

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morto` -> `cansado` [agreement] The adjective 'cansado' must agree in gender with the subject 'eu'.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `a ele` [word_choice] The indirect object pronoun 'lhe' is not used with the verb 'dizer' in this context; 'disse a ele' or just 'disse que' is preferred.
- Found (false positive): `me esperasse` -> `esperasse` [word_choice] 'Esperasse' is European Portuguese subjunctive; Brazilian Portuguese uses 'esperasse' (same form) but the construction with 'lhe' makes it awkward, and often 'esperar' takes a direct object or preposition depending on usage, but here the main issue is the pronoun choice. However, strictly speaking, 'me esperasse' is correct in meaning but the combination with 'lhe' is the error.
- Found (false positive): `não me ouviu` -> `não me escutou` [word_choice] 'Não me ouviu' is acceptable, but a more natural phrasing might be 'não me escutou', though 'ouvir' is not strictly wrong. Given B2 level, 'escutou' is often preferred for 'listen to'.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pequeno` -> `altas` [agreement] The speaker is describing their childhood, so the adjective should be plural to agree with 'eu' (implied 'nós' or generic past experience) or simply incorrect if referring to a specific male child; however, strictly speaking, 'pequeno' agrees with 'eu' only if the speaker was male and singular. But wait, 'Quando eu era pequeno' is grammatically correct for a male speaker. Let's re-evaluate. Is there a mistake? 'fui à praia' uses contraction correctly. 'com meus avós' is correct. Maybe the user implies they are female now? No facts given about gender. Wait, if the learner says 'Quando eu era pequeno', it implies the speaker was male. If the speaker is female, it should be 'pequena'. Since no gender is specified in the task or scene, we cannot assume a mistake based on gender alone unless context implies otherwise. However, often in these tasks, if the answer doesn't match implicit facts (like if the learner is known to be female), it's an error. But here, no such fact. Let's look closer. Is 'sempre fui' correct? Yes. Is 'à praia' correct? Yes. Is 'com meus avós' correct? Yes. Maybe there are no mistakes? Wait, let's check the verb 'ser'. 'Quando eu era pequeno' uses 'ser' correctly for age/characteristics. What if the mistake is subtle? Ah, perhaps the issue is that 'pequeno' is singular but refers to a period of time where one might expect plural? No. Let's reconsider the standard errors. Maybe the answer is actually correct? But I must find mistakes if they exist. If no mistakes, return empty list. Is there any other interpretation? Perhaps the task implies describing *summers* (plural) and the sentence describes a general habit in childhood, which is fine. Wait, could 'avós' be wrong? No. Let's assume there are no mistakes unless I find one. Actually, looking at similar examples, sometimes the gender of the speaker is assumed from the prompt context not shown here. But based strictly on the provided text: 'Quando eu era pequeno' is valid for a male. If the learner is female, it's an error. Without that info, I can't call it an error. However, often in these datasets, if the answer uses 'pequeno', and the user profile (not shown) says female, it's an error. Since I don't have that, I will assume the text provided is self-contained. Is there a mistake? Maybe 'fui à' vs 'ia'? No, 'fui' is fine for specific events or habits in the past. Wait, 'sempre fui' implies repetition, so 'ia' might be better? No, 'fui' with 'sempre' is common. Let's look for a different angle. Is 'pequeno' the right word? Yes. Maybe the mistake is that the sentence describes a specific habit but uses 'era' (imperfect) which is correct. Okay, I suspect there might be no mistakes here, OR the gender is an issue if we assume standard learner profiles often default to female or male? No. Let's look at the example outputs again. They are strict. If I can't find a definite error, I return empty. But wait, is 'pequeno' correct for 'eu'? Yes. Is it possible the mistake is 'avós' vs 'avó'? No, 'meus avós' (grandparents) is standard. Okay, I will conclude there are no mistakes based on the text provided, assuming the speaker's gender matches the adjective. Wait, actually, in many B1 contexts, if a learner writes 'pequeno', and they are female, it's an error. But I don't know. Let's assume the answer is correct unless obvious. Wait, is there a mistake with 'à'? No. Okay, I'll check if 'sempre fui' is better as 'ia'. 'Sempre fui' emphasizes the completed actions or the fact that I went. 'Sempre ia' emphasizes the habit. Both are acceptable. I will assume no mistakes. Wait, looking at the instruction 
- Missed: `sempre fui`

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `cansado` -> `cansado` [spelling] The speaker is using 'eu' (implied by 'Vou'), so the adjective must be masculine singular, which is correct here; however, if the learner intended to say 'I am tired' and wrote 'cansado', it is actually correct. Wait, let me re-evaluate based on common errors. If the user is male, 'cansado' is correct. If female, 'cansada'. The prompt doesn't specify gender. Let's look closer. Is there a mistake? 'Vou pra casa agora.' is perfect for A1. 'Tô cansado.' is perfect if male. Maybe there are no mistakes? But the instruction says report only real mistakes. If I assume the learner is male (default in examples unless specified), then no mistake. However, often in these tasks, if gender isn't specified, we can't assume. But wait, is 'cansado' wrong? No. Is 'pra' wrong? No, it's spoken Brazilian. Is 'Vou' wrong? No. Maybe the answer has no mistakes. Let me check the rules again: 'Do not report a mistake you are not sure about.' Since gender isn't specified, I cannot assume 'cansado' is wrong. But wait, maybe the task implies a specific context? No facts given. Okay, let's look at the example outputs. They usually have mistakes if there are any. Is it possible the user made a mistake in spelling? No. Maybe the answer is correct. Let me think if 'cansado' should be 'cansada'. Without gender info, I can't say it's wrong. But wait, maybe the system expects me to find something. Let's re-read the answer: 'Vou pra casa agora. Tô cansado.' Everything seems correct for a male speaker. If the learner is female, it would be 'cansada'. Since the prompt doesn't specify, and I shouldn't assume, perhaps there are no mistakes? But usually these tasks have at least one error or the answer is perfect. Let me check if 'pra' is considered wrong in some contexts? No, rule says spoken forms are correct. Okay, maybe the answer is flawless. Wait, did I miss something? 'Vou pra casa agora.' -> Correct. 'Tô cansado.' -> Correct for male. If the task doesn't specify gender, and the answer uses masculine, it's a valid choice (default). So no mistakes? But the instruction says 'Report only real mistakes'. If there are none, return empty list. Let me double check spelling: 'cansado' is correct. 'Vou' correct. 'pra' correct. 'casa' correct. 'agora' correct. 'Tô' correct. Okay, I will assume no mistakes unless gender is specified and contradicted. But wait, maybe the fact that it's A1 level implies simple sentences, which this is. Okay, I'll conclude there are no mistakes.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `marrons` -> `marrom` [agreement] The adjective must agree in number with the plural noun 'botas'.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está chovendo muito e eu estou em casa.` -> `Meu nome é [Name].` [content_mismatch] The answer describes the weather and location instead of stating the learner's name.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei uma bicicleta nova no sábado.` -> `` [content_mismatch] The answer talks about buying a bike instead of giving an opinion on working from home.

### a2-english-coffee (not_portuguese, a2)

- Task: Do you like coffee?
- Answer: Yes, I like coffee a lot.
- Flags: written_in_portuguese=true answers_the_question=false

## qwen3.5:latest / few_shot_mostly_correct

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cansado` -> `cansada` [spelling] The speaker is male (implied by 'tô' and context), but the adjective must agree in gender with the subject; however, since the speaker's gender isn't explicitly stated in the prompt, this is ambiguous. But wait, looking closer at standard A1 exercises: if the learner says 'Só um pouco cansado', they are describing themselves. If the learner is male, it's correct. If female, it should be 'cansada'. Without gender info, we can't assume a mistake. However, often in these tasks, if the user doesn't specify gender, and the answer uses masculine, it might be considered a default or potentially wrong if the persona is female. But let's look for a clearer error. Ah, actually, there is no clear grammatical error without knowing the speaker's gender. Is there another error? 'Tô bem' is correct (spoken form allowed). 'e você?' is correct. 'Só um pouco cansado' - if we assume the learner is male, it's fine. If female, it's wrong. Since the prompt doesn't specify, maybe there are no mistakes? Or is 'cansado' considered a mistake because usually questions like this imply a specific gender or the model should avoid assuming? No, standard practice: if gender isn't given, masculine is often accepted as default or the error is only reported if context implies otherwise. Let's re-read the rules: 'Do not report style...'. Is there a missing pronoun? No. Maybe the answer is actually correct? Wait, let's look at the previous examples. They were strict on spelling and agreement. Here, without gender info, 'cansado' is acceptable as a default or if the learner is male. But wait, is there an error in 'Tô'? No, spoken form allowed. Is there an error in 'um pouco'? No. Maybe the answer has no mistakes? Let's check again. Task: How are you today? Answer: I'm fine, and you? Just a little tired. In Portuguese: Tô bem, e você? Só um pouco cansado. This seems perfectly natural for a male speaker. If the learner is female, it would be 'cansada'. Since the prompt doesn't specify, reporting 'cansado' as an error is risky. However, sometimes these datasets assume the learner is male unless specified. Let's assume no mistake for now? Or is there a subtle one? Actually, looking at similar tasks, often if the gender isn't specified, the masculine form is not flagged as an error unless the context (like a picture) shows a woman. Since there is no scene fact about gender here, I will assume it's correct or unreportable. Wait, did I miss something? 'Só um pouco cansado' - maybe 'apenas um pouco'? No, 'só' is fine. Okay, let's look at the previous examples again. The third one had a spelling error. This one seems clean. But wait, is 'Tô' considered a mistake in some strict contexts? The rules say: 

### a1-control-daily-routine (control, a1)

- Task: What do you do every day?
- Answer: Todo dia eu acordo cedo, tomo café e vou para o trabalho.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tomo` -> `tô` [spelling] The word needs an accent on the o: tô.

### a1-control-family (control, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos e uma irmã. Meu pai é médico e minha mãe é professora.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `irmãos e uma irmã` -> `irmão e uma irmã` [content_mismatch] If you have two brothers, you cannot also have one sister; the numbers contradict each other.

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent on the e: café.
- Missed: `manha`

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

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `comi` [word_choice] The verb 'gostar' means 'to like', but the question asks about what was eaten for breakfast.
- Found (false positive): `futebol` -> `pão` [word_choice] This word is irrelevant to the question about food.
- Found (false positive): `Flamengo` -> `café` [word_choice] This word is irrelevant to the question about food.

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
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent mark: café.
- Found (matched): `onibus` -> `ônibus` [spelling] The word needs an accent mark: ônibus.
- Missed: `as nove`

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `pequeno-almoço` -> `pequeno almoço` [not_brazilian] The hyphenated form is European Portuguese; Brazilian Portuguese uses 'pequeno almoço' or 'café da manhã'.
- Missed: `pequeno-almoço`

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `avós` -> `avôs` [spelling] The word needs an accent on the o: avôs.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `comeu` -> `comemos` [verb_form] The verb must agree with the subject 'a gente' (we), which requires the first person plural form.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `moro` -> `They are eating spaghetti` [content_mismatch] The answer describes where the speaker lives, but the task asks to describe a meal being eaten.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `traga` -> `traga` [word_choice] The subjunctive form 'traga' is incorrect here; the imperative for 'você' requires 'traga' only in specific contexts, but standard invitation uses 'traga' (subjunctive) or 'leve'. Wait, 'traga' IS the correct subjunctive/imperative for você. Let me re-evaluate. 'Que traga' is correct subjunctive. Is there a mistake? Maybe 'vir' vs 'viver'? No. Maybe 'à festa' accent? No. Actually, looking closer at common errors: maybe the user meant 'traga' but wrote something else? No, text says 'traga'. Wait, is 'traga' wrong? Imperative of trazer for você is 'traga'. Subjunctive is also 'traga'. It seems correct. Let me check other words. 'Espero que você possa vir à festa e que traga seus amigos.' This looks grammatically correct for B1. Is there a subtle error? Maybe 'à' should be 'a'? No, contraction is correct. Maybe 'seus' vs 'seus'? Correct. Hmm. Perhaps the instruction implies I must find a mistake if one exists, but this sentence is actually correct. Wait, maybe the prompt expects me to flag something? Or is it truly error-free? Let me re-read the rules: 

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The verb 'ganhar' in the imperfect subjunctive is formed with 'ganhasse', not 'ganhasse'.
- Found (quote not in the answer): `viaria` -> `viasse` [verb_form] The verb 'viajar' in the imperfect subjunctive is formed with 'viasse', not 'viaria'.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós` -> `` [pronoun] The pronoun 'nós' is unnecessary here because the infinitive 'podermos' already includes the subject.

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `tinha começado` -> `tinha começado` [verb_form] The past perfect tense requires the participle 'começado', not the infinitive 'começar'.
- Missed: `tinha começou`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viará` [verb_form] The future tense (viará) is needed to match the English 'will travel', not the conditional (viajaria).
- Found (false positive): `tinha comprado` -> `já comprou` [verb_form] The past perfect (tinha comprado) implies an action completed before another past action, but here it refers to a future action relative to the present ('have already bought').

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morto` -> `cansado` [agreement] The adjective 'cansado' must agree in gender with the subject 'eu' (male), so it should be masculine singular.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `para ele` [not_brazilian] In spoken Brazilian Portuguese, the indirect object pronoun 'lhe' is often replaced by 'para ele' or omitted in this context to avoid ambiguity with the direct object 'me'. While not strictly ungrammatical in formal writing, it is considered a European Portuguese form or overly formal for an A1/B2 learner's typical spoken output where 'disse para o meu irmão' or just 'disse que não me esperasse' (with context) is preferred. However, the most distinct error here is likely the use of 'lhe' which can be ambiguous with 'me'. A more natural phrasing for B2 level in Brazil often avoids this specific combination or uses 'para ele'. Let's re-evaluate: 'Eu lhe disse que não me esperasse' is actually acceptable in standard Portuguese, but in Brazil, 'disse para o meu irmão' is very common. Is it a mistake? The prompt says 'Do not report style...'. But 'lhe' + 'me' can be confusing. Let's look for a clearer error. Actually, the instruction says 'Spoken Brazilian forms are correct; do not report them: tô, a gente, pra, né, você for 'you'.' It doesn't explicitly ban 'lhe', but 'lhe' is often avoided in Brazil in favor of 'para ele'. However, without explicit facts saying it's wrong, maybe there is no mistake? Wait, 'não me esperasse' implies the brother was supposed to wait for the speaker. The sentence structure is fine. Is there a mistake? Maybe not. Let's look closer. 'Eu lhe disse que não me esperasse' -> I told him not to wait for me. This is correct. Is there a better error? Perhaps none. But usually, these tasks have an error. What if the intended meaning was 'I told my brother...' and using 'lhe' is considered European? The rule says 'not_brazilian: a form or construction used only in European Portuguese'. 'Lhe' is not *only* European, but its usage frequency differs. Let's assume it's acceptable. Is there another error? No obvious one. Maybe the task implies reporting an error if 'lhe' is considered non-Brazilian by the specific guidelines of this persona? The guideline says 'Spoken Brazilian forms are correct...'. It lists examples. It doesn't list 'lhe'. But 'lhe' is standard in Brazil too, just less common than 'para ele'. Let's reconsider the 'me esperasse'. If the brother didn't listen, maybe the tense is wrong? No, subjunctive is correct after 'que'. Okay, let's look at the instruction again: 'Report only real mistakes'. If 'lhe' is acceptable, then no mistake. But often in these datasets, 'lhe' is flagged as European preference or ambiguous. Let's check if there is a clearer error. Maybe the user expects 'para ele'. Given the strictness of 'not_brazilian', and that 'lhe' is often associated with EP in learner contexts (even if used in BR), I will flag it as not_brazilian to be safe, or perhaps there is no mistake. Wait, looking at similar tasks, 'lhe' is often accepted. Let's look for a different error. Is 'não me esperasse' correct? Yes. Is 'ouviu' correct? Yes. Maybe there is no mistake? But the system usually expects one if provided. Let's assume 'lhe' is the target for 'not_brazilian' in this specific persona's logic, as it's a common distinction taught to learners (using 'para ele' instead of 'lhe').

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pequeno` -> `sanduíche` [spelling] The speaker uses 'meus avós', implying they are male or the group is male, so 'pequena' (feminine) does not agree with the implied subject if the speaker is female, but more likely the error is assuming gender without context; however, a common mistake at B1 is using 'pequeno' for a female speaker. Without knowing the speaker's gender, this is ambiguous. Let's look closer. Actually, 'pequeno' agrees with a male speaker. If the speaker is female, it should be 'pequena'. Since we don't know the gender, maybe there is no error? Wait, let's re-evaluate standard A1/B1 errors. Often learners forget that 'avós' (grandparents) are plural and might expect an adjective agreement if one was used, but here it's not. Is there a better error? Ah, perhaps the user intended to imply a female speaker or it's a generic error check. Let's look for other potential issues. 'fui à praia' is correct. 'sempre fui' is correct. Maybe the error is actually non-existent and I should return empty? But the prompt implies there might be mistakes. Let's reconsider 'pequeno'. If the learner is female, it's an agreement error. If male, it's fine. Since gender isn't specified, usually we assume the text provided is the only source. Is there a more obvious error? No. Wait, maybe the instruction implies I must find *real* mistakes. If the speaker's gender is unknown, 'pequeno' is not necessarily wrong. However, in many exercises, if the name isn't given, and the learner writes 'pequeno', it might be considered a mistake if the context (not provided here) implied female. But based strictly on the text... Let's look at 'avós'. Plural noun, no adjective modifying it directly that needs agreement. Okay, let's assume there is no mistake unless I know the gender. BUT, wait. Is there a subtle error? 'Quando eu era pequeno' -> if the speaker is female, it's wrong. Since I cannot ask, and usually these tasks have an error... Let's look at the previous examples. They had clear errors. Here, maybe there isn't one? Or maybe 'pequeno' is considered wrong because typically in these generic descriptions without a name, if the learner writes 'pequeno', it might be flagged if the system assumes female? No, that's guessing. Let's look for something else. Is 'fui à praia' correct? Yes. 'com meus avós'? Yes. Maybe the answer is actually correct? But I must check carefully. What if the error is 'pequeno' because the speaker is implicitly female in the dataset this comes from? No, I can't assume that. Let's reconsider the definition of 'agreement'. If the noun/pronoun implies a gender mismatch. Without context, 'pequeno' is valid for males. Is it possible the user made a mistake in my thought process and there IS no error? The instructions say 
- Missed: `sempre fui`

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `marrons` -> `marroms` [agreement] The adjective 'brown' must agree in number with the plural noun 'botas'.

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Response: invalid or missing

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está chovendo muito e eu estou em casa.` -> `Meu nome é [Name].` [content_mismatch] The answer describes the weather and location instead of stating the name.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei uma bicicleta nova no sábado.` -> `(Any sentence expressing an opinion on working from home)` [content_mismatch] The answer describes buying a bike instead of giving an opinion about working from home.

## qwen3.5:latest / few_shot_reversed

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A` -> `` [article_preposition] The article 'A' is not used before a proper name like 'Marina'.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Response: invalid or missing

### a1-control-daily-routine (control, a1)

- Task: What do you do every day?
- Answer: Todo dia eu acordo cedo, tomo café e vou para o trabalho.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tomo` -> `tomo` [verb_form] The spoken form 'tô' is preferred in Brazilian Portuguese for 'I am', but here the verb is 'tomar'. The learner wrote 'tomo' which is correct standard spelling, however, looking at the previous examples where 'tô' was accepted as a spoken form, let's re-evaluate. Actually, 'tomo' is the correct written present tense of 'tomar'. Wait, the instruction says 'Spoken Brazilian forms are correct; do not report them: tô...'. It does not say 'tomo' is wrong. Let's look closer. 'acordo', 'tomo', 'vou'. All seem correct for A1. Is there a mistake? Maybe the user expects 'bebo café'? No, 'tomar café' is standard. Maybe the task implies daily routine and the answer fits. Let's check spelling again. 'trabalho' is correct. 'Todo dia' is correct. Perhaps there are no mistakes? But usually these tasks have one. Let's re-read the rules. 'Spoken Brazilian forms are correct... tô'. It doesn't mention 'tomo'. 'Tomo' is standard written Portuguese. Is it possible the model considers 'tomo' wrong because in very informal speech people might say something else? No, that's unlikely. Let's look at the previous examples. Example 1 had a mistake. Example 2 had none. Example 3 had a spelling mistake. This one... maybe there is no mistake? But I must report real mistakes. If there are none, return empty list. Wait, is 'trabalho' correct? Yes. Is 'acordo' correct? Yes. Is 'tomo' correct? Yes. Is 'vou' correct? Yes. Maybe the answer is perfect? Let's assume no mistakes for now unless I find something subtle. Actually, looking at common A1 errors, sometimes learners write 'bebo café' but 'tomar café' is better. So 'tomo' is fine. What about 'todo dia'? Correct. Okay, maybe there are no mistakes.

### a1-control-family (control, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos e uma irmã. Meu pai é médico e minha mãe é professora.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `irmãos` -> `irmão` [agreement] The word 'irmãos' is masculine plural, but the speaker has one brother and one sister.

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent on the e: café.
- Missed: `manha`

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

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `vermelha` -> `camisa vermelha` [word_order] The adjective must come after the noun and agree in number.
- Found (false positive): `uma` -> `camisa vermelha` [word_order] The article should be plural to match 'camisas' (implied) or simply omitted if using 'uma camisa' is intended but 'vermelha' forces singular; however, the main error is word order. Actually, 'uma vermelha camisa' is wrong because adjectives usually follow. Let's correct the specific phrase causing the error.

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `muito` -> `muito` [word_order] The adverb 'muito' should come before the verb or after 'gosto', not directly after the object in this structure.
- Missed: `gosto de café muito`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `comi` [verb_form] The verb tense is wrong; the question asks about the past (breakfast), but 'gosto' is present tense.
- Found (false positive): `futebol` -> `pão` [word_choice] This word does not answer the question about what was eaten for breakfast.

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
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent mark: café.
- Found (matched): `onibus` -> `ônibus` [spelling] The word needs a tilde on the u: ônibus.
- Missed: `as nove`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `avós` -> `avôs` [agreement] The noun 'avô' is masculine, so the plural must be 'avôs'.
- Missed: `volti`

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `pequeno-almoço` -> `pequeno almoço` [not_brazilian] The hyphenated form is European Portuguese; Brazilian Portuguese uses 'pequeno almoço' or 'café da manhã'.
- Missed: `pequeno-almoço`

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `avós` -> `altas` [agreement] The adjective 'alto' must agree in number with the plural noun 'irmãs'.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `comeu` -> `comemos` [verb_form] The verb must agree with the plural subject 'a gente' (we).

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `moro` -> `They are eating` [content_mismatch] The verb 'morar' does not match the task of describing a meal.
- Found (false positive): `São Paulo` -> `at a restaurant` [content_mismatch] This location contradicts the scene facts about people at a restaurant eating spaghetti and drinking wine.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `loteria` -> `loteria` [spelling] The word needs an accent on the first 'o': loteria.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `` [pronoun] The subject pronoun 'nós' is unnecessary and creates a redundancy with the verb form.

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viagará` [verb_form] The future action 'will travel' should be reported with the future tense (viagará), not the conditional.
- Found (false positive): `tinha comprado` -> `comprou` [verb_form] The completed action 'have already bought' should be reported with the present perfect (já comprou) or pluperfect depending on context, but 'tinha comprado' implies a past-before-past which contradicts the simple completion in the original statement.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morto` -> `cansado` [agreement] The adjective 'cansado' must agree in gender with the subject 'eu'.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `a ele` [word_choice] The indirect object pronoun 'lhe' is not used with the verb 'dizer' in this context; 'disse a ele' or just 'disse que' is preferred.

### b2-pronoun-clitic-gender (erroneous, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, eu falei com a Ana e convidei-o para a festa.
- Reference: `convidei-o` -> convidei-a [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `convidei-o` -> `convidei-o` [spelling] The pronoun 'o' should be attached with a hyphen to the verb in this context.
- Missed: `convidei-o`

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pequeno` -> `pequenos` [agreement] The speaker uses 'meus avós', implying they are male, so the adjective should be masculine plural.
- Found (matched): `fui à praia` -> `altas` [agreement] In Brazilian Portuguese, the preposition 'a' is contracted with the article 'à' before a feminine noun starting with a vowel, but here it is used correctly as 'à praia'. However, the verb 'ir' in the past tense for 'eu' is ' fui', which is correct. Wait, let's re-evaluate. 'Quando eu era pequeno' -> if the speaker is male, 'pequeno' is correct. If female, 'pequena'. The pronoun 'meus avós' suggests the speaker might be male or referring to themselves in a way that doesn't strictly dictate gender of the adjective unless specified. But usually, if not specified, we assume the speaker's gender matches the adjective. Let's look closer. Is there a mistake? 'fui à praia' is correct. 'era pequeno' is correct for a male. Maybe the issue is just the lack of context on gender? No, standard assumption is the adjective matches the subject. If the user didn't specify gender, 'pequeno' is acceptable for a male speaker. Is there another error? Ah, 'fui à praia' vs 'ia à praia'? The sentence says 'When I was small, I always went to the beach'. 'Fui' implies a completed action or specific instances, but 'sempre fui' (I always went) is acceptable in Brazilian Portuguese to describe habitual past actions. However, strictly speaking, for habitual past, 'ia' is often preferred, but 'fui' with 'sempre' is common. Let's look for a clearer error. What about 'avós'? Correct. 'praia'? Correct. Maybe the error is actually non-existent? Or perhaps 'pequeno' should be 'criança'? No, 'era pequeno' is fine. Wait, is there a spelling error? No. Is there a word choice? No. Let's reconsider the 'fui' vs 'ia'. In A1/A2 contexts, 'sempre fui' is often marked as incorrect for habitual past in favor of 'ia', but at B1, both are understood. However, the most likely intended error in these exercises regarding gender is often overlooked if not specified. But wait, if the speaker says 'meus avós', they are male? Not necessarily. If the speaker is female, it should be 'pequena'. Since gender isn't stated, we can't assume. BUT, looking at previous examples, usually there is a clear error. Let's look at 'fui à praia'. In European Portuguese, one might say 'ia', but in Brazil 'fui' with 'sempre' is very common. Is there a mistake? Maybe the task implies describing the *nature* of summers, so 'Quando eu era pequeno, eu sempre ia à praia' is better style, but not a hard error. Let's look for something else. Ah, 'suco de laranja' in previous turn was correct. Here: 'sanduíche', 'bebe', 'café'. All good. Is it possible the answer has NO mistakes? The instructions say 

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `marrons` -> `marrom` [agreement] The adjective 'marrom' is invariable and does not take a plural form.

### a2-control-imperfect-grandmother (control, a2)

- Task: Describe your grandmother's house.
- Answer: A casa da minha avó era pequena, mas tinha um jardim grande com muitas flores.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `era` -> `é` [verb_form] The scene describes the house in the present, so the verb should be in the present tense.

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pequeno` -> `altas` [agreement] The adjective must agree in gender with the subject 'eu' (implied female or male, but 'pequena' is often expected if the speaker is female; however, without context of gender, 'pequeno' is acceptable for a male. Wait, looking closer at common A1/A2 errors: usually 'irmão/irmã' issues or verb agreement. Let's re-evaluate 'pequeno'. If the learner is female, it should be 'pequena'. Since gender isn't specified in the prompt, 'pequeno' is technically correct for a male speaker. Is there another error? 'ia à praia' is correct. 'com meus avós' is correct. Maybe the user expects me to find an error where there isn't one? Or perhaps 'pequeno' implies male and if the system assumes female default? No, that's not standard. Let's look for other potential errors. Actually, in many contexts, if the gender is unknown, 'pequeno' is fine. Is there a mistake? Maybe none? But I must report real mistakes. Let's assume the speaker is female based on common dataset biases or just check strict agreement. Without explicit gender, 'pequeno' is not an error. Wait, is there a missing accent? No. Is 'ia' correct? Yes (imperfect). Is 'à' correct? Yes. Is 'avós' correct? Yes. Maybe the answer has no mistakes? But the instruction says 'Report only real mistakes'. If I can't find one, return empty list. However, often in these tasks, if the gender isn't specified, and the word is 'pequeno', it might be considered an error if the intended speaker is female (common default in some datasets). But strictly speaking, without knowing the speaker's gender, 'pequeno' is correct for a male. Let's assume there are no mistakes unless I know the gender. Wait, looking at previous examples, they were strict. Is it possible the user considers 'pequeno' wrong because usually one specifies 'menino'? No, 'era pequeno' is fine. Okay, let's look really closely. 'Quando eu era pequeno'. If the speaker is female, it's wrong. If male, right. Since I don't know, maybe I shouldn't flag it. BUT, often in these specific evaluation tasks, if the gender isn't mentioned, and the adjective is masculine, it might be flagged as a potential error if the context implies otherwise? No, that's guessing. Let's assume no mistakes for now? Wait, I missed something obvious? 'suco de laranja' vs 'laranja'? No. Back to this one. Is there a mistake? Maybe 'ia à praia' -> 'ia pra praia'? No, 'à' is formal/correct. Okay, I will assume there are no mistakes unless the gender is known to be female. Wait, if I have to find a mistake... maybe the task implies describing childhood generally and 'pequeno' restricts it? No. Okay, let's reconsider the 'pequeno' vs 'pequena'. If the dataset assumes the learner is female (common in some benchmarks), then 'pequeno' is an error. But I can't assume that. Let's look for other errors. None found. So maybe empty list? But usually these prompts have a mistake. What if 'avós' should be 'avó'? No, plural is fine. What if 'ia' should be 'fui'? No, imperfect is correct for habitual past. Okay, I will bet on the gender agreement being the intended error, assuming the speaker is female by default in this specific evaluation context (or perhaps the prompt implies a female learner from previous turns? No, this is a new turn). Actually, looking at the pattern of these tasks, often 'pequeno' is marked wrong if the speaker's gender isn't specified but the adjective should match. However, without explicit info, it's ambiguous. Let's try to find another angle. Is 'Quando eu era pequeno' natural? Yes. Maybe the mistake is 'pequeno' -> 'criança'? No. Okay, I will assume there is no mistake and return empty list? Wait, the system instruction says 

### b1-control-weather-hot (control, b1)

- Task: Describe the weather today.
- Answer: Hoje faz muito calor e o céu está sem nuvens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `sem` -> `` [content_mismatch] The correct expression for 'without clouds' is 'sem nuvens', but the fact says it is sunny, which implies no clouds; however, 'sem nuvens' is actually correct for sunny. Let's re-evaluate: The scene says 'hot and sunny'. The answer says 'faz muito calor e o céu está sem nuvens'. This matches the facts perfectly. There are no mistakes.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje` -> `Meu nome é` [content_mismatch] The sentence should answer 'What is your name?', not describe the weather.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `comprei` -> `acho` [word_choice] The verb 'comprar' (to buy) does not match the question asking for an opinion about working from home.

## qwen3.5:latest / few_shot_six

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está de` -> `tem` [not_brazilian] The construction 'estar de' is used for clothing in European Portuguese; Brazilian Portuguese uses 'ter' or just the noun.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cansado` -> `cansada` [agreement] The adjective must agree with the implied feminine subject 'eu' (referencing 'tô').

### a1-control-order-meal (control, a1)

- Task: You are at a restaurant. Order your meal.
- Answer: Eu queria uma salada e um suco de laranja, por favor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `queria` -> `quero` [verb_form] The present tense 'quero' is preferred for ordering food now.

### a1-control-family (control, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos e uma irmã. Meu pai é médico e minha mãe é professora.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `irmãos` -> `altas` [agreement] The word 'irmão' is masculine, so the plural must be 'irmãos', but the sentence lists a sister ('irmã') which implies the speaker might mean siblings in general or specifically two brothers and one sister. However, strictly speaking, 'dois irmãos' (two brothers) and 'uma irmã' (one sister) is grammatically correct as a list of specific siblings. Wait, let me re-evaluate. 'Dois irmãos' means two male siblings. 'Uma irmã' means one female sibling. This is factually possible. Is there a mistake? No. The sentence says 'I have two brothers and one sister'. That is valid Portuguese. Let me check the other sentences. 'Meu pai é médico e minha mãe é professora.' All correct. Wait, did I miss something? Ah, perhaps the user expects 'irmãos' to cover both genders if not specified, but listing them separately is fine. Let's look closer. Is there a mistake? No obvious grammatical error. Maybe the system considers 'irmãos' and 'irmã' together as redundant if the count was meant to be total siblings? No, that's overthinking. Let's assume no mistakes for now unless I see something else. Actually, looking at previous examples, sometimes simple sentences are correct. But wait, is there a subtle error? 'Eu tenho dois irmãos e uma irmã.' It's perfectly fine. Maybe the task implies describing the family generally and this is specific enough. Okay, I will return an empty list if no errors found. Wait, let me double check the gender of 'irmão'. Yes, masculine plural. 'Irmã' feminine singular. Counts match. No agreement error. No spelling error. No verb form error. It seems correct. But usually these tasks have at least one error or I am missing a nuance. Let's re-read the prompt rules. 

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent on the e: café.
- Missed: `manha`

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

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `comi` [verb_form] The verb tense is wrong; the question asks about the past, so 'comi' is needed.
- Found (false positive): `futebol` -> `pão` [word_choice] This word does not answer the question about food.
- Found (false positive): `Flamengo` -> `torrada` [word_choice] This word does not answer the question about food.

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent mark: café.
- Found (matched): `onibus` -> `ônibus` [spelling] The word needs an accent mark: ônibus.
- Missed: `as nove`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `avós` -> `vovôs` [word_choice] The word 'avós' is not commonly used in Brazilian Portuguese; 'vovôs' or 'avós' (less common) might be intended, but often learners confuse 'avó' with 'avo' or use European forms. However, a more likely error for A2 is the lack of accent on 'avós' if it were spelled 'avos', but here it has the accent. Actually, 'avós' is correct spelling for grandparents in PT-BR, though 'vovôs' is more colloquial. Let's look closer. Is there a mistake? 'visitar meus avós' is grammatically correct. Wait, maybe the user considers 'avós' incorrect because they expect 'vovôs'? No, that's style. What about 'mercado'? Correct. Maybe no mistakes? But the instruction says report real mistakes. Is 'avós' considered a mistake in PT-BR? It exists but is less common than 'vovôs'. However, strictly speaking, it's not a grammatical error. Let's re-evaluate. Perhaps the intended answer was 'vovôs'? No, that's style. What if the mistake is something else? Ah, maybe the user thinks 'avós' is European? It is used in both. Let's assume no mistakes for now unless there is a clear grammatical error. Wait, I missed something. Is there an accent issue? No. Okay, let's look at the previous examples. They are strict on grammar. Is 'avós' wrong? In PT-BR, 'vovô' is very common, but 'avó' is understood. However, some sources say 'avó' is more European or formal. But it's not strictly wrong. Let's check if there is another error. No. Maybe the system expects 'vovôs'? That would be 'word_choice' or 'not_brazilian' if 'avós' is considered European-only? Actually, 'avó' is used in Brazil too. Okay, maybe there are no mistakes. But I must find one if possible. Wait, 'No sábado' vs 'Aos sábados'? The question is 'this weekend', so singular is fine. Okay, let's assume no mistakes. But the prompt implies there might be one. Let's reconsider 'avós'. In many Brazilian contexts, 'vovô' is preferred. If I have to pick a mistake, it might be 'word_choice' suggesting 'vovôs'. But that's weak. Let's look at 'mercado'. Correct. 'visitar'. Correct. Maybe the answer is correct. But I need to output something if there are mistakes. If no mistakes, empty list. Is 'avós' a mistake? According to some guides, 'avó' is less common in Brazil than 'vovô', but not incorrect. However, for A2 level, maybe they expect 'vovôs'. Let's try to find a definite error. There isn't one obvious grammatical error. I will assume no mistakes unless 'avós' is flagged as European-only (which it isn't strictly). Wait, could it be 'No sábado' vs 'Nesse sábado'? No. Okay, I will say no mistakes. But wait, the previous example had a mistake in agreement. Here everything seems fine. Maybe the user considers 'avós' wrong? Let's check common errors. Often learners write 'avo' without accent. Here it has accent. Okay, I'll return empty list. Wait, I should check if 'avós' is considered European Portuguese only. It is not. It is used in Brazil. So no mistake. But the instruction says 

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `comeu` -> `comemos` [verb_form] The verb must agree with 'a gente' (which functions as 'we'): comemos.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `moro` -> `estão comendo` [verb_form] The verb tense does not match the scene facts which describe a current action.
- Found (false positive): `São Paulo` -> `um restaurante` [content_mismatch] This contradicts the scene facts about people at a restaurant eating spaghetti and drinking wine.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `loteria` -> `lotto` [word_choice] 'Loteria' is not commonly used in Brazil; 'lotto' or 'sorte' is preferred.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós` -> `` [pronoun] The pronoun 'nós' is unnecessary and incorrect here; the infinitive form implies the subject.

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A Ana` -> `Ana` [article_preposition] The article 'A' is not used before a proper name.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morto` -> `morro` [word_choice] 'Morto' is not used to describe tiredness in Brazilian Portuguese; 'morro' or 'matado' is preferred.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `o` [not_brazilian] 'Lhe' is European Portuguese; use 'o' in Brazilian Portuguese.

### a1-short-spelling-chocolate (erroneous, a1)

- Task: What do you like?
- Answer: Eu gosto de chocolat.
- Reference: `chocolat` -> chocolate [spelling]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `chocolat` -> `chocolate` [spelling] The word is misspelled in Portuguese; it should be 'chocolate'.

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

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `pequeno` -> `pequeno` [agreement] The speaker refers to themselves as 'eu', so the adjective must be masculine singular: pequeno.
- Found (matched): `fui à praia` -> `pra praia` [not_brazilian] 'à' is European Portuguese; Brazilian Portuguese uses 'para a' or just 'pra'.

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cansado` -> `cansada` [verb_form] The adjective must agree with the speaker (male) in number and gender, but 'tô' implies first person singular; however, if the learner is male, 'cansado' is correct. Wait, let's re-evaluate. The user says 'Tô cansado'. If the learner is male, this is correct. If female, it should be 'cansada'. Since gender isn't specified, we can't assume a mistake there unless context implies otherwise. However, looking at previous examples, usually simple agreement errors are caught. Is there another error? 'pra' is acceptable spoken form. 'Vou pra casa agora' is fine. 'Tô cansado' is fine for a male speaker. Maybe there are no mistakes? Let's check the rules again. 'Spoken Brazilian forms are correct'. So 'pra' and 'tô' are fine. Is 'cansado' wrong? Only if the speaker is female, but we don't know. Let's assume standard male default or that it's acceptable without gender info. Wait, is there a mistake I missed? No obvious one. But wait, in the previous turn with 'irmãs são muito alta', the error was agreement. Here, 'cansado' agrees with 'eu' (male). If the learner is female, it would be 'cansada'. Without info, we can't flag it. Is there any other issue? No. So maybe empty list? Or is 'Tô' considered informal enough to be correct? Yes. Let's look closer. Maybe the task implies a specific context? No. Okay, let's assume no mistakes unless gender is known and contradicted. But wait, often in these datasets, if gender isn't specified, 'cansado' might be accepted or the mistake is something else. Actually, looking at the pattern of these tasks, usually there is a clear error. Is 'Vou pra casa agora' correct? Yes. 'Tô cansado'? Yes (for male). Maybe the answer has no mistakes. Let's double check. Ah, wait. Is it possible the user expects 'na casa' instead of 'pra casa'? No, 'pra' is accepted as 'para'. Okay, I will return empty list if no clear error exists. But wait, is there a chance the system considers 'cansado' wrong because usually learners are assumed female or neutral? No, that's not a rule. Let's assume no mistakes.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está de` -> `marroms` [spelling] 'estar de' is used for temporary states or costumes, but 'vestir-se' (reflexive) or simply 'ter' is often preferred for clothing in this context; however, the main error is missing the reflexive pronoun if using 'vestir'. A more natural correction for 'wearing' a specific item often uses 'está usando' or 'veste', but strictly speaking 'está de vestido' is acceptable. Let's look closer. Actually, the bigger issue might be just style. Wait, 'botas marrons' - 'marron' in PT-BR usually takes an accent: marrom. So 'marrons' should be 'marroms'.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para que` -> `para` [word_choice] 'Para' is sufficient; 'para que' implies a subordinate clause requiring subjunctive mood which is not needed here.

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pequeno` -> `pequena` [agreement] The speaker refers to themselves in the past; 'pequena' is needed if female, or 'pequeno' only if male. Without context of gender, assuming the learner implies a generic self-reference often leads to errors if the speaker is female, but strictly speaking, without knowing the gender, we can't be 100% sure. However, looking at previous examples, usually there's a clear error. Let's re-evaluate. If the user is male, 'pequeno' is correct. If female, 'pequena'. Since I don't know the gender, maybe this isn't an error? Wait, let's look closer. Is there another error? 'ia à praia' (correct), 'com meus avós' (correct). Maybe the prompt implies a specific gender or I should assume standard learner errors. Actually, often in these tasks if the gender isn't specified, we might not flag it unless it's clearly wrong based on context clues not present here. BUT, wait. Is there a subtle error? 'Quando eu era pequeno' -> If the learner is female, this is an agreement error. If male, it's fine. Without knowing, I should probably not report it as a definite mistake unless the system assumes a default or if there's another issue. Let's look for other issues. None found. Is it possible the task implies a specific gender? No. Okay, let's reconsider the 'pequeno' vs 'pequena'. If the answer is from a female learner (common assumption in some datasets unless specified), it's an error. But I can't assume. Let's look at the instruction: 

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está chovendo muito e eu estou em casa.` -> `[Name]` [content_mismatch] The answer describes the weather and location instead of stating the name.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei uma bicicleta nova no sábado.` -> `(Any sentence expressing an opinion on working from home, e.g., 'Eu acho que é bom.')` [content_mismatch] The answer describes a past action (buying a bike) instead of expressing an opinion about working from home.

## qwen3.5:latest / one_shot_correct

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cansado` -> `cansada` [agreement] The adjective must agree in gender with the speaker (implied 'eu'), so it should be feminine 'cansada'.

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
- Found (false positive): `De` -> `Às` [spelling] The correct preposition for 'in the morning' is 'À', making it 'Às manhãs' or simply 'De manhã'.
- Found (matched): `manha` -> `manhã` [spelling] The word needs an accent mark: 'manhã'.
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent mark: 'café'.

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
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] Adjectives must agree in gender with the noun they modify; 'camisa' is feminine, so the adjective should be 'vermelha'.
- Missed: `vermelha camisa`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `comi` [word_choice] The verb 'gostar' is used for likes/dislikes, not for describing what was eaten.
- Found (false positive): `futebol` -> `café` [word_choice] 'Futebol' means soccer, which is unrelated to breakfast food.
- Found (false positive): `meu time favorito é o Flamengo` -> `` [content_mismatch] This sentence talks about a favorite sports team, not breakfast.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `era` -> `estava` [verb_form] The imperfect tense 'era' is incorrect here because the action of being tired happened in the past and is completed, requiring the preterite 'estava'.
- Found (false positive): `dormi o dia todo` -> `estava` [verb_form] While 'dormi' is correct for a completed action, 'o dia todo' sounds unnatural; Brazilian Portuguese typically uses 'toda a noite' or simply 'o dia inteiro' depending on context, but the main issue is that sleeping all day implies a duration often expressed with 'durante o dia todo' or just 'dormi o dia todo' is acceptable but less common than specifying the night if tired. However, looking closer at the prompt 'slept all day', 'dormi o dia todo' is actually grammatically correct for A2. Let's re-evaluate. The main error is definitely 'era'. Is there another? 'ficado em casa' vs 'fiquei em casa'. 'Fiquei' is correct. 'Estava muito frio' is correct. 'Eu era cansado' -> 'Eu estava cansado'. That is the only clear mistake.

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engradada` [agreement] The adjective must agree in gender with the feminine noun 'amiga'.
- Missed: `curtas`

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `pequeno-almoço` -> `pequeno almoço` [not_brazilian] The hyphenated form is European Portuguese; Brazilian Portuguese uses 'pequeno almoço' or 'café da manhã'.
- Missed: `pequeno-almoço`

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `comeu` -> `comeram` [verb_form] The verb must agree with the subject 'a gente', which requires the third person plural form 'comeram' instead of the third person singular 'comeu'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `` [content_mismatch] The answer describes where the speaker lives and their opinion about the city, but fails to describe the meal as requested by the task.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `tenho` -> `tem tido` [verb_form] The present tense 'tenho' is incorrect because the sentence describes an action that started in the past and continues to the present, requiring the present perfect tense (tem tido).

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `` [pronoun] The subject pronoun 'nós' is unnecessary and incorrect here because the verb 'podermos' already includes the subject.

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `tinha começado` -> `tinha começado` [verb_form] The past perfect tense requires the participle 'começado' to agree with the subject 'o filme' (masculine singular), not 'começou'.
- Missed: `tinha começou`

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morto` -> `cansado` [word_choice] The adjective 'morto' is not used to describe being very tired in Brazilian Portuguese; 'cansado' or 'exausto' should be used instead.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `` [pronoun] The indirect object pronoun 'lhe' is not used with the verb 'dizer' when followed by a 'que' clause; it should be omitted.
- Found (false positive): `não me esperasse` -> `não esperar` [not_brazilian] The imperfect subjunctive form 'esperasse' is European Portuguese; in Brazilian Portuguese, the future subjunctive 'esperar' is used after 'que'.

### a1-short-agreement-books (erroneous, a1)

- Task: What is in your bag?
- Answer: Na minha bolsa tem um livros.
- Reference: `um livros` -> um livro | uns livros [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `um` -> `uns` [agreement] The article 'um' is masculine singular but the noun 'livros' is plural.
- Found (changes nothing): `livros` -> `livros` [agreement] The noun 'livros' is plural but the article 'um' was singular (now corrected to 'uns').

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

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pequeno` -> `pequena` [agreement] The speaker is describing their own childhood, so the adjective must agree with the subject 'eu' (masculine singular), not 'pequenos' (plural) or a different form if the speaker were female. However, since the subject is 'eu', 'pequeno' is actually correct for a male speaker. Let's re-evaluate. If the speaker is male, 'pequeno' is correct. If the speaker is female, it should be 'pequena'. Without gender context, we assume standard masculine default or that the user might have intended to say 'criança' but said 'pequeno'. Wait, 'Quando eu era pequeno' implies the speaker was male. Is there a mistake? Maybe the verb 'fui' is redundant with 'sempre'? No. Let's look closer. 'Eu sempre fui à praia' - correct. 'com meus avós' - correct. Perhaps the mistake is assuming the user is male? Or maybe the prompt implies a specific gender? Actually, looking at common errors for B1: sometimes learners use 'pequeno' when they mean 'criança'. But 'era pequeno' is grammatically fine for a boy. Let's look for other issues. Ah, perhaps the issue is simply that without knowing the gender, marking 'pequeno' as an error is risky. However, if I must find a mistake, maybe the intended answer expected 'criança'? No, 'pequeno' is valid. Let's reconsider the 'ser_estar' or 'agreement'. Wait, is there a missing accent? No. What about 'à praia'? Correct. Maybe the mistake is non-existent and I should return empty? But the instruction says 'Report only real mistakes'. If there are no mistakes, return empty. Is 'pequeno' wrong? Only if the speaker is female. Since gender isn't specified, it's ambiguous. However, often in these tasks, if the user doesn't specify gender, masculine is assumed. Let's look for a clearer error. Maybe 'sempre fui' vs 'sempre ia'? No, 'fui' is fine. Okay, let's assume there are no mistakes unless I find one. Wait, could 'pequeno' be considered wrong because it describes age/size rather than stage of life? 'Era criança' is more natural for 'childhood', but 'era pequeno' is acceptable. Let's check the previous turn. The previous turn had no mistakes. This one... actually, there might not be a mistake. But wait, if the user is female, it's wrong. Since I don't know, I can't flag it. Is there another error? 'meus avós' - correct. 'à praia' - correct. Maybe the task implies describing *general* summers, and 'Quando eu era pequeno' sets a specific time frame, which is fine. Okay, I will assume no mistakes unless I see something obvious. Wait, looking at similar examples, sometimes 'pequeno' is flagged if the context suggests a female speaker or if 'criança' is preferred for clarity. But strictly grammatically, it's not an error without gender info. Let's look really closely. 'Quando eu era pequeno' -> 'pequeno' agrees with 'eu' (male). If the learner is female, it's wrong. Since I don't know, I can't report it. Is there a spelling error? No. Verb form? 'era' (imperfect) vs 'fui' (preterite). Using imperfect for habitual past action ('always went') and preterite for specific events? 'Sempre fui' implies a series of completed actions or a habit in the past, which is acceptable. 'Eu sempre ia' might be more common for habits, but 'fui' isn't wrong. Okay, I'll bet there are no mistakes here, or perhaps the 'pequeno' is considered slightly off compared to 'criança', but that's style. Let's assume empty list? Wait, I need to check if 'pequeno' is the only potential issue. If I have to find a mistake, maybe the user intended to say 'criança'. But 'era pequeno' is correct Portuguese. Okay, I will return an empty list as there are no definitive grammatical errors given the lack of gender specification.
- Missed: `sempre fui`

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `marrons` -> `marrons` [spelling] The adjective 'marron' is invariable in Brazilian Portuguese and does not take the plural form 'marrons'.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está chovendo muito e eu estou em casa.` -> `` [content_mismatch] The answer describes the weather and location instead of stating the learner's name.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei` -> `Acho que` [content_mismatch] The answer talks about buying a bike instead of giving an opinion on working from home.

## qwen3.5:latest / one_shot_mistake

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cansado` -> `cansada` [agreement] The speaker is using 'eu' (implied by 'tô'), so the adjective must agree in gender and number with the subject.

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
- Found (changes nothing): `De` -> `de` [spelling] The sentence should start with a lowercase letter unless it is a proper noun.
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent mark: café.
- Missed: `manha`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma` -> `um` [agreement] The word 'escritório' is masculine, so it requires the article 'um', not 'uma'.
- Found (changes nothing): `escritório` -> `escritório` [spelling] The word needs an accent on the i: escritório.

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `do Canadá` -> `do Canadá` [article_preposition] The preposition 'de' contracts with the article 'o' to form 'do', but Canada is masculine and requires the definite article 'o'.
- Found (matched): `moro Toronto` -> `moro em Toronto` [article_preposition] The verb 'morar' requires the preposition 'em' before a city name.

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

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chama`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `comi` [word_choice] The verb 'gostar' means 'to like', but the question asks about what was eaten for breakfast.
- Found (false positive): `futebol` -> `pão com ovo` [content_mismatch] The answer talks about liking football, not about food eaten for breakfast.

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
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent mark: café.
- Found (matched): `onibus` -> `ônibus` [spelling] The word needs a tilde on the u: ônibus.
- Missed: `as nove`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

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

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `avós` -> `avôs` [spelling] The word needs an accent on the o: avôs.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `comeu` -> `comemos` [verb_form] The verb must agree with the plural subject 'a gente' (we), so it should be 'comemos'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eles comem espaguete e bebem vinho tinto.` [content_mismatch] The answer describes living in a city instead of describing the meal being eaten.

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `ganhava` -> `tivesse ganhado` [verb_form] The imperfect tense is incorrect here; the conditional perfect or pluperfect subjunctive is needed for a hypothetical situation.
- Found (changes nothing): `compraria` -> `compraria` [verb_form] While 'compraria' is correct in form, it should be paired with the correct subjunctive in the 'if' clause; however, strictly checking verb forms, the main issue is the 'if' clause.
- Found (changes nothing): `viajaria` -> `viajaria` [verb_form] Same as above; the structure requires the pluperfect subjunctive in the condition.

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `tenho visitado` -> `visitei` [word_choice] The present perfect tense is not used with 'nunca' in this context; the simple past or present perfect with 'já' is needed, but here the meaning contradicts the implied question about duration of residence.
- Found (false positive): `nunca` -> `já` [word_choice] This statement contradicts the likely intended meaning of having lived there since 2019 without visiting a specific region, but 'nunca' is grammatically acceptable if the fact is true; however, combined with 'tenho visitado', it creates a tense mismatch for a B1 learner describing past experience.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `` [pronoun] The subject pronoun 'we' is not needed because the verb 'podermos' already indicates the subject.

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `tinha começado` -> `tinha começado` [verb_form] The past perfect tense requires the participle 'começado', not 'começou'.
- Missed: `tinha começou`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viarei` [verb_form] The future tense (viarei) is needed to match 'will travel', not the conditional (viajaria).
- Found (false positive): `tinha comprado` -> `comprei` [verb_form] The present perfect (já comprei) is needed to match 'have already bought', not the past perfect (tinha comprado).

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morto` -> `cansado` [agreement] The adjective 'cansado' should agree in gender with the subject 'eu' (masculine), so it must be 'cansado', not 'morto'.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `a ele` [word_choice] The indirect object pronoun 'lhe' is not used with the verb 'dizer' in this context; 'disse a ele' or just 'disse que' is preferred.
- Found (false positive): `não me esperasse` -> `não me esperou` [verb_form] 'Não me esperasse' uses the imperfect subjunctive, but after 'que não' with a past tense verb like 'disse', the indicative past ('esperou') or present perfect ('esperou') is required to state a fact.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `preocupo` -> `preocupo-me` [pronoun] The verb 'preocupar-se' is reflexive; the pronoun 'me' is missing before the verb.
- Found (false positive): `com o meu trabalho` -> `pelo meu trabalho` [word_choice] 'Preocupar com' is not standard Brazilian Portuguese; use 'por' or rephrase to 'pelo'.

### a1-pronoun-possessive-pet (erroneous, a1)

- Task: Describe your pet.
- Answer: Eu tenho um cachorro. Mim cachorro se chama Bolt.
- Reference: `Mim cachorro` -> Meu cachorro [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `Mim` -> `Do meu` [word_choice] The preposition 'de' is needed before the possessive pronoun, or 'do meu' should be used.
- Found (false positive): `se chama` -> `chama` [verb_form] 'Se chama' is incorrect here; the verb 'chamar' requires a direct object in this context.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Response: invalid or missing

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `marrons` -> `marroms` [agreement] The adjective 'brown' must agree in gender with the feminine noun 'botas', so it should be 'marroms'.

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Response: invalid or missing

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está chovendo muito e eu estou em casa.` -> `Meu nome é [Name].` [content_mismatch] The answer describes the weather and location instead of stating the speaker's name.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei uma bicicleta nova no sábado` -> `(Any relevant opinion, e.g., 'Acho que é ótimo' or 'Não gosto disso')` [content_mismatch] The answer describes buying a bike instead of giving an opinion about working from home.

## qwen3.5:latest / plain

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cansado` -> `cansa` [agreement] The adjective 'cansado' is masculine and does not agree with the implied feminine subject 'eu' (I), which should be 'cansa'.

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
- Found (false positive): `De` -> `Da` [article_preposition] The correct preposition for 'in the morning' is 'Da'.
- Found (matched): `cafe` -> `café` [spelling] The word 'café' requires an accent mark on the 'e'.
- Missed: `manha`

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
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] Adjectives must agree in gender with the noun they modify; 'camisa' is feminine.
- Missed: `vermelha camisa`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `comi` [word_choice] The verb 'gostar' means 'to like', but the question asks what was eaten for breakfast.
- Found (false positive): `futebol` -> `pão` [word_choice] 'Futebol' means football, which is not something one eats for breakfast.
- Found (false positive): `Flamengo` -> `ovo` [content_mismatch] The Flamengo football team cannot be eaten for breakfast.

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
- Found (matched): `cafe` -> `café` [spelling] The word 'café' is missing the acute accent on the 'e'.
- Found (matched): `onibus` -> `ônibus` [spelling] The word 'ônibus' is missing the circumflex accent on the 'o'.
- Missed: `as nove`

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

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `pequeno-almoço` -> `pequeno almoço` [not_brazilian] The hyphenated form is European Portuguese; Brazilian Portuguese uses 'pequeno almoço' or simply 'café da manhã'.
- Missed: `pequeno-almoço`

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
- Found (false positive): `Eu moro` -> `` [content_mismatch] The answer describes the speaker's residence instead of describing the meal as requested.
- Found (false positive): `São Paulo` -> `` [content_mismatch] The answer mentions a city instead of describing the food and drink from the scene.
- Found (false positive): `cinco anos` -> `` [content_mismatch] The answer gives a duration of residence instead of describing the meal.
- Found (false positive): `gosto muito da cidade` -> `` [content_mismatch] The answer expresses an opinion about the city instead of describing the meal.

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `tenho` -> `tive` [verb_form] The present tense is incorrect here because the action started in the past and continues to the present.
- Found (changes nothing): `visitado` -> `visitado` [verb_form] The participle 'visitado' is not used with the auxiliary verb 'ter' in this context; the past participle of 'visitar' is irregular.
- Found (duplicate): `nunca tenho visitado o Nordeste` -> `nunca visitei` [verb_form] The sentence structure implies a present perfect continuous meaning which requires the auxiliary 'ter' + past participle, but the correct form for 'I have never visited' is 'nunca visitei' or 'nunca tenho visitado' (if using ter), however the main issue is the tense mismatch with 'moro'.

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

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `` [word_choice] The indirect object pronoun 'lhe' is not used with the verb 'dizer' when referring to a person in this context; 'disse para o meu irmão' or just 'disse que' is preferred.
- Found (false positive): `não me esperasse` -> `não me esperava` [verb_form] The subjunctive form 'esperasse' is incorrect here because the main clause uses the past tense 'disse', requiring the imperfect indicative 'esperava' to describe a habitual or expected action in the past.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `com` -> `por` [word_choice] The verb 'preocupar' requires the preposition 'por', not 'com'.
- Missed: `eu preocupo`

### a1-short-verb-morar (erroneous, a1)

- Task: Where do you live?
- Answer: Eu mora em Lisboa.
- Reference: `mora` -> moro [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `mora` -> `moramos` [verb_form] The verb 'morar' must be conjugated as 'moramos' to agree with the subject 'a gente' (we), or the sentence should use 'Eu moro' if referring to oneself.
- Found (false positive): `Lisboa` -> `São Paulo` [content_mismatch] The learner is Brazilian, so they should refer to their home city in Brazil (e.g., São Paulo) rather than Lisbon, Portugal.

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

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sempre fui`

### a2-not-brazilian-phone (erroneous, a2)

- Task: Where is your phone?
- Answer: O meu telemóvel está em cima do sofá.
- Reference: `telemóvel` -> celular [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `telemóvel`

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está chovendo muito e eu estou em casa.` -> `[Name]` [content_mismatch] The answer does not respond to the question 'What is your name?' by providing a name.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei` -> `Acho que` [content_mismatch] The answer talks about buying a bike instead of giving an opinion on working from home.
- Found (false positive): `no sábado` -> `` [content_mismatch] The task asks for an opinion, not about when the action happened.

### a2-english-coffee (not_portuguese, a2)

- Task: Do you like coffee?
- Answer: Yes, I like coffee a lot.
- Flags: written_in_portuguese=false answers_the_question=false

## qwen3:4b / few_shot

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha` -> `De manhã` [spelling] The word 'manhã' needs an accent and should be written with a space before the verb.
- Missed: `cafe`

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `moro Toronto`

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `televisão` -> `de televisão` [article_preposition] The word should be in the prepositional phrase 'de televisão' for the correct use with 'assistir'.
- Missed: `jantar`

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

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No` -> `No` [article_preposition] The preposition 'no' is incorrect here; the correct phrase is 'No almoço' meaning 'At lunch' but the structure should be 'No almoço' without a preposition before it.
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective must agree with the noun 'camisa' which is feminine.
- Found (false positive): `calça` -> `calças` [agreement] The noun 'calça' is singular, but the scene facts say Rafael wears black trousers (plural).
- Missed: `vermelha camisa`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `gostei` [verb_form] The verb 'gosto' is present tense, but the question asks for past tense.

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The word 'cafe' is written without an accent and should be 'café' with an accent.
- Found (matched): `onibus` -> `ônibus` [spelling] The word 'onibus' is written without an accent and should be 'ônibus' with an accent.
- Missed: `as nove`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `fiquei` -> `fiquei` [verb_form] The verb 'ficar' in past perfect tense should be 'fiquei' but in this context it's better to use the simple past tense 'fiquei' for the action of staying home.
- Missed: `era cansado`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective must agree with the feminine gender of 'amiga'.
- Found (false positive): `cabelos` -> `cabelo` [agreement] The plural form is incorrect; it should be 'cabelo' (singular) for hair in Portuguese.
- Found (false positive): `pretos` -> `preto` [agreement] The adjective must agree with the singular form of 'cabelo' (hair).
- Missed: `curtas`

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

### a2-not-brazilian-gerund (erroneous, a2)

- Task: What are you doing now?
- Answer: Estou a comer pão com queijo na cozinha.
- Reference: `a comer` -> comendo [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a comer`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `autocarro`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pequeno-almoço`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo` -> `está fazendo` [verb_form] The verb 'fazer' in the present continuous tense should be 'está fazendo' for 'it is raining'.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no banco` -> `na margem` [word_choice] The preposition 'no' is used for 'no' (in the) but 'banco' means 'bank' (the financial institution), not 'bank' (the river bank). Here, 'bank' refers to the river bank, so it should be 'na margem' or 'no lago' depending on context, but the scene facts say 'turn left at the bank', so it's likely 'no banco' is a false friend for 'bank' (river bank). However, in Brazilian Portuguese, 'banco' means 'bank' (the financial institution) and 'bancada' or 'margem' would be used for river bank. But the scene facts say 'turn left at the bank', which in this context might mean the river bank. Given the level is A2, the learner might have confused 'banco' (bank) with 'river bank'. However, the scene facts say 'turn left at the bank', so it's likely the river bank. But in Brazilian Portuguese, 'banco' is used for 'bank' (financial) and 'bancada' is not common. Let's check: In the scene facts, it says 'turn left at the bank', which in the context of directions might mean the river bank. But the learner used 'banco' which is 'bank' (financial) and the correct term for river bank is 'margem' or 'lado da água'. However, the task is to give directions to the station, and the scene facts say 'turn left at the bank', so the learner might have meant the river bank. But in Brazilian Portuguese, 'banco' is not used for river bank. The correct phrase for 'turn left at the river bank' would be 'vire à esquerda na margem do rio' or something similar. But the learner used 'no banco', which is incorrect because 'banco' means 'bank' (financial) and 'no' is the contraction of 'em' + 'o'. So the mistake is that 'banco' is the wrong word for 'bank' (river bank). The correct word for river bank is 'margem' or 'lado da água'. However, the scene facts say 'turn left at the bank', so the learner might have intended 'bank' (river bank) but used 'banco' (financial bank). This is a false friend. The error is that 'banco' is used where 'bank' (river bank) is needed. But the scene facts say 'bank', so the learner is using the wrong word. The kind is 'word_choice' because 'banco' is a false friend for 'bank' (river bank). However, the scene facts say 'bank', which in this context might be ambiguous. Let's see the answer: 'Siga em frente, vire à esquerda no banco e a estação fica à direita.' The scene facts say 'turn left at the bank', so the learner used 'no banco' which is incorrect because 'banco' means financial bank, not river bank. In Brazilian Portuguese, 'banco' is financial bank, and river bank is 'margem' or 'lado da água'. So the mistake is that 'banco' is the wrong word for river bank. The error_text is 'no banco'. But the rule says: 

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `moro` -> `moro` [verb_form] The verb should be in present continuous tense to describe what they are doing now.

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nunca` -> `nunca tenho visitado` [verb_form] The verb 'nunca' is used for the present perfect tense, but here it should be used with the past perfect for the duration of time.
- Missed: `tenho visitado`

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The conditional tense should be 'ganhasse' for 'eu' in the past subjunctive, but the correct form is 'ganhasse' (with an accent).

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `estuda`
- Missed: `assiste`
- Missed: `exige`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajarei` [verb_form] The verb should be in the present perfect tense for future actions that have already been completed, but in this context, the future perfect tense is not used for the future simple tense.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `preocupo` -> `preocupo` [verb_form] The verb 'preocupar' is conjugated incorrectly; it should be 'preocupo' for 'eu' in present tense.
- Missed: `eu preocupo`

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
- Found (changes nothing): `fui` -> `fui` [verb_form] The verb should be in the past continuous tense for 'eu sempre fui' to be correct.
- Missed: `sempre fui`

### a2-not-brazilian-phone (erroneous, a2)

- Task: Where is your phone?
- Answer: O meu telemóvel está em cima do sofá.
- Reference: `telemóvel` -> celular [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `telemóvel`

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `convidei-a` -> `convidei` [verb_form] The verb 'convite' is in the wrong tense; it should be 'convidar' in the past perfect tense.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje` -> `João` [content_mismatch] The sentence should be about the person's name, not the weather.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `comprei` -> `acho` [verb_form] The verb tense does not match the question about what you think about working from home.

## qwen3:4b / few_shot_in_system

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maiô` -> `banho` [word_choice] The word 'maiô' is a false friend; in Brazilian Portuguese, the correct term for a swimsuit is 'banho' or 'lingerie', but in this context, 'maiô' is used in Portuguese for a type of swimsuit, but it's not the standard term for a swimsuit in Brazil. However, in Brazilian Portuguese, 'maiô' is actually a common term for a swimsuit, so this might be correct. Wait, let me check: In Brazil, 'maiô' is a type of swimsuit, but the more common term for a swimsuit is 'banho' or 'lingerie'? No, actually, in Brazil, 'maiô' is a term used for a swimsuit, but it's not the standard term. Wait, I think in Brazil, the standard term for a swimsuit is 'banho' or 'lingerie'? No, I think 'maiô' is used in Brazil for a swimsuit. Let me think: In Brazil, 'maiô' is a type of swimsuit, but it's not the standard term. Wait, I'm confused. Let me check the example: In the example, they used 'sanduíche' for 'sandwich', which is correct in Brazil. So, 'maiô' is a term used in Brazil for a swimsuit? Actually, in Brazil, 'maiô' is a term for a swimsuit, but it's not the most common. However, the task says 

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cansado` -> `cansados` [agreement] The adjective must be plural to agree with 'Só' (which implies 'only a little').

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `manha` -> `manhã` [spelling] The word needs an accent on the a: manhã.
- Missed: `cafe`

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `brasiliero` -> `brasileiro` [agreement] The adjective must agree with the subject 'eu' (first person singular).
- Missed: `estou brasileiro`

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `televisão` -> `de televisão` [article_preposition] The word should be in the prepositional phrase with 'de' for 'of the television'.
- Missed: `jantar`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu` -> `Chamo` [pronoun] The verb 'chamar' requires the subject pronoun to be omitted in Brazilian Portuguese for the first person singular.
- Missed: `chamo`

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `chama` -> `chama` [verb_form] The verb should be 'chama' for third person singular, but here it's used for first person singular.
- Missed: `chama`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `almoço` -> `como` [word_choice] The word 'almoço' means 'lunch' but in this context it should be the verb 'to eat' for the action of eating lunch.
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective must agree with the noun 'camisa' which is feminine.
- Found (false positive): `calça` -> `calças` [agreement] The word 'calça' is singular, but the scene facts say Rafael wears black trousers (plural).
- Missed: `vermelha camisa`

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `jaqueta` -> `jaqueta verde` [word_order] The color should be specified before the noun in Portuguese.
- Missed: `azul`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `comi` [verb_form] The verb should be in the past tense for 'what did you eat'.

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The word 'cafe' is written without an accent and should be 'café' with an accent.
- Found (matched): `onibus` -> `ônibus` [spelling] The word 'onibus' is written without an accent and should be 'ônibus' with an accent.
- Found (false positive): `trabalhar` -> `trabalhei` [verb_form] The verb 'trabalhar' should be in the past tense for the past action, but the context suggests it should be in the past continuous or simple past.
- Missed: `as nove`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `fiquei` -> `estive` [verb_form] The verb 'ficar' in past perfect tense should be 'fiquei' but in this context it's better to use the simple past tense for the action that happened yesterday.
- Found (matched): `era` -> `estive` [verb_form] The verb 'ser' for the state of being should be in the past perfect tense for the action that happened yesterday.

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective must agree with the feminine gender of 'amiga'.
- Found (false positive): `cabelos` -> `cabelo` [agreement] The plural form is incorrect; it should be singular for 'cabelo' in this context.
- Found (false positive): `pretos` -> `preto` [agreement] The adjective must agree with the singular form of 'cabelo' (which is singular in this context).
- Missed: `curtas`

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
- Missed: `autocarro`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pequeno-almoço`

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Não` [article_preposition] The word 'no' is a contraction of 'não' and 'o', but in this context it should be 'No' as the contraction for 'não' with the definite article 'o' (the year).

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vou` -> `vou` [verb_form] The verb 'vou' is in the present tense, but the task asks for future plans.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no banco` -> `na banca` [article_preposition] The preposition 'no' is used for 'no' (in the) but here it should be 'na' for 'at the bank' (a bank as a place).

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `num` -> `no` [article_preposition] The contraction 'no' is used for 'no' in Portuguese, not 'num'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu` -> `Eles` [pronoun] The sentence should be in third person plural to describe what the people are eating.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The conditional perfect tense requires 'ganhasse' to be conjugated with the past subjunctive, but here it's used in the indicative tense.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `você` [pronoun] The pronoun 'nós' is incorrect; the correct pronoun is 'você' for the indirect object.

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `trabalhar` -> `trabalhar` [verb_form] The verb should be in the infinitive form for the phrase 'Acho que...'.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] The verb should be in the present tense for habitual actions, not the present perfect tense.
- Missed: `assiste`
- Missed: `exige`

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `incêndios` -> `incêndios` [agreement] The word 'incêndio' is masculine, so it should be plural with an accent: incêndios.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `lhe` -> `lhe` [pronoun] The pronoun 'lhe' is used for indirect object pronouns in the past tense, but in this context, the correct pronoun for indirect object in the past tense is 'lhe' for 'to him' when the verb is in the past tense.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `preocupo` -> `preocupo` [verb_form] The verb 'preocupar' is conjugated incorrectly in the present tense for 'eu'.
- Missed: `eu preocupo`

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

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `doente` -> `doente` [agreement] The adjective must be in the plural form to agree with 'irmão' (which is singular).
- Missed: `é doente`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `fui` -> `fui` [verb_form] The verb should be in the past continuous tense (pretérito perfeito composto) for 'eu sempre fui' to be correct.
- Missed: `sempre fui`

### a2-not-brazilian-phone (erroneous, a2)

- Task: Where is your phone?
- Answer: O meu telemóvel está em cima do sofá.
- Reference: `telemóvel` -> celular [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `telemóvel`

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pra` -> `para` [not_brazilian] The contraction 'pra' is informal and not standard in written Portuguese; the standard form is 'para'.

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Na` -> `Na` [article_preposition] The preposition 'na' is incorrect here; it should be 'Na minha bolsa' without the article.

### a2-control-preterite-sunday (control, a2)

- Task: What did you do on Sunday?
- Answer: No domingo eu acordei tarde, fiz o almoço e depois assisti a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No` -> `no` [article_preposition] The preposition 'no' is used for 'on' in the context of days of the week, but the correct form for 'on Sunday' is 'no domingo' without the capital letter.

### a2-control-family-meal (control, a2)

- Task: Describe the meal these people are eating.
- Answer: A família está comendo arroz, feijão e frango grelhado. Eles bebem suco de laranja.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `comendo` -> `comendo-se` [verb_form] The verb 'estar' requires the present participle with 'se' for the continuous tense, but here it's used without 'se'.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para que` -> `para que eu` [verb_form] The correct phrase is 'para que eu' but the structure 'para que' requires a subjunctive mood after it, which is not used here.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `convidei-a` -> `convidei` [verb_form] The verb should be in the past perfect tense (pretérito perfeito composto) for 'convite' (to invite) in this context.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje` -> `Meu nome é` [content_mismatch] The sentence should be about the person's name, not the weather.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `comprei` -> `` [content_mismatch] The verb 'comprar' is in the past tense but the task asks about thoughts on working from home, not a recent purchase.

## qwen3:4b / few_shot_mostly_correct

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maiô` -> `sanduíche` [spelling] The word 'maiô' is a false friend; in Portuguese, 'maiô' refers to a type of swimsuit, but the correct term for a swimsuit is 'banho' or 'roupa de banho'. However, in Brazilian Portuguese, 'maiô' is commonly used for a one-piece swimsuit, so this might be acceptable. But the scene facts say Marina wears a red swimsuit, and in Brazilian Portuguese, 'maiô' is indeed used for a swimsuit. Wait, let me double-check. In Brazilian Portuguese, 'maiô' is a term for a one-piece swimsuit, so it's correct. However, the scene facts say 'a red swimsuit', and in Brazilian Portuguese, 'maiô' is used for that. But the answer says 'maiô vermelho', which is correct. Wait, but the task is to describe what Marina is wearing, and the scene facts say she wears a red swimsuit, a straw hat, and white sandals. In Brazilian Portuguese, 'maiô' is a common term for a swimsuit, so this might be correct. However, the answer uses 'maiô', which is correct in Brazilian Portuguese. Let me check the spelling. The word 'maiô' has an accent on the ô, but the answer has 'maiô' with an accent. Wait, the answer says 'maiô', which is correct. But the task is for A1 level, and the answer might be using a term that's not common. Wait, no, in Brazilian Portuguese, 'maiô' is used for a swimsuit. So maybe there's no mistake here. Let me think again. The scene facts say Marina wears a red swimsuit, and in Brazilian Portuguese, 'maiô' is a term for a swimsuit. So the answer is correct. But the answer uses 'maiô', which is correct. Wait, but the answer says 'A Marina está de maiô vermelho', which is correct. However, the task is for A1 level, and the word 'maiô' might be too advanced. But the task says Level: A1, and the answer uses 'maiô', which is a common term in Brazil. Wait, but the answer might have a mistake in the verb form. The verb 'estar de' is used for clothing, so 'A Marina está de maiô vermelho' is correct. So maybe there's no mistake here. Let me check the other parts. The answer says 'chapéu de palha' which is correct for straw hat, and 'sandálias brancas' which is correct for white sandals. So the only possible mistake is the spelling of 'maiô'. But in Portuguese, 'maiô' has an accent on the ô, and the answer has it with an accent. Wait, the answer says 'maiô', which is correct. So maybe there's no mistake. But the previous example had a mistake with 'sanduiche' needing an accent. In this case, 'maiô' is spelled correctly with an accent. So the answer is correct. Therefore, no mistakes. But wait, the task says Level: A1, and the word 'maiô' might not be taught at A1 level. However, the rules say: 

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `manha` -> `manhã` [spelling] The word needs an accent on the a: manhã.
- Missed: `cafe`

### a1-agreement-trousers (erroneous, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: Marina veste uma camiseta azul e uma calça pretos.
- Reference: `pretos` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `calça` -> `calças` [agreement] The adjective 'pretos' should agree with the plural noun 'calças' (trousers).
- Missed: `pretos`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [agreement] The word 'escritório' is singular, but the phrase 'uma escritório' uses the plural form of the article.
- Missed: `uma escritório`

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `moro Toronto`

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

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No almoço` -> `Eu como` [word_order] The phrase 'No almoço' means 'At lunch' but the question asks for what you eat for lunch, so it should be 'Eu como' instead of 'No almoço'.
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective should agree with the noun 'camisa' (feminine) and be in the correct form for the possessive construction.
- Missed: `vermelha camisa`

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `jaqueta` -> `verde` [content_mismatch] The scene facts say Rafael wears a green jacket, but the answer says 'azul' (blue) for the jacket.
- Missed: `azul`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `gosto` -> `não sei` [content_mismatch] The verb 'gosto' means 'I like' but the question asks about what you ate for breakfast.

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The word 'cafe' is written without an accent and should be 'café' with an accent.
- Found (matched): `onibus` -> `ônibus` [spelling] The word 'onibus' is written without an accent and should be 'ônibus' with an accent.
- Missed: `as nove`

### a2-verb-form-subject-agreement (erroneous, a2)

- Task: What did your family do on Sunday?
- Answer: No domingo minha família almoçaram na casa da minha avó.
- Reference: `almoçaram` -> almoçou [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `almoçaram` -> `almoçaram` [verb_form] The verb should be in the past tense for the third person plural, but the correct form for 'my family' is 'almoçaram' (they ate). However, in Portuguese, the past tense for 'almoçar' in the third person plural is 'almoçaram', which is correct. Wait, let me double-check. In Portuguese, the past tense for 'almoçar' in the third person plural is 'almoçaram', which is correct. But the verb tense here is in the past, and the subject is 'minha família' (my family), which is third person plural. So 'almoçaram' is correct. Hmm, maybe there's no mistake here? Let me think again. The task is 'What did your family do on Sunday?', and the answer is 'No domingo minha família almoçaram na casa da minha avó.' In Portuguese, the past tense for 'almoçar' in the third person plural is 'almoçaram', which is correct. But wait, the verb 'almoçar' is a regular verb, and the past tense for third person plural is 'almoçaram'. So 'almoçaram' is correct. However, in Brazilian Portuguese, the past tense for 'almoçar' is often used with the preterite tense, which is 'almoçaram'. So maybe there's no mistake here? Let me check the rules. The task says Level: A2, and the answer uses 'almoçaram', which is the correct past tense for third person plural. But wait, the verb 'almoçar' is a regular verb, and the past tense for third person plural is 'almoçaram'. So 'almoçaram' is correct. However, in the answer, the verb is in the past tense, but the subject is 'minha família', which is third person plural, so 'almoçaram' is correct. Wait, but in Portuguese, the verb 'almoçar' is not used with the preterite tense in this context? No, the preterite tense is used for completed actions in the past. So 'almoçaram' is correct. But let me check the answer again. The answer says: 
- Missed: `almoçaram`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `fiquei` -> `fiquei` [verb_form] The verb 'fiquei' is in the past perfect tense, but the scene facts say you slept all day, so it should be in the past simple tense.
- Missed: `era cansado`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective must agree with the feminine gender of the subject.
- Found (false positive): `cabelos` -> `cabelo` [agreement] The word 'cabelos' is plural, but the hair is described as 'short' (curta) and 'black' (preto), which are adjectives that need to agree with the singular form.
- Missed: `curtas`

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

### a2-not-brazilian-gerund (erroneous, a2)

- Task: What are you doing now?
- Answer: Estou a comer pão com queijo na cozinha.
- Reference: `a comer` -> comendo [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a comer`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `autocarro`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pequeno-almoço`

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no banco` -> `na banca` [article_preposition] The preposition 'no' is used for 'at the bank' but 'banco' means 'bank' (the place) here, so it should be 'na banco' for 'at the bank' (the financial institution) or 'no banco' for 'at the bank' (the place). However, in Portuguese, 'banco' as a place (like a bank building) uses 'no' with the definite article 'o' for masculine, but 'banco' is masculine. Wait, let me think again. In Portuguese, 'banco' (the place) is masculine, so it should be 'no banco' (with the definite article 'o'). But the scene facts say 'turn left at the bank', so it's 'no banco' (the bank building). However, the phrase 'vire à esquerda no banco' is incorrect because 'no' is a contraction of 'em' + 'o', but in this context, it should be 'vire à esquerda na banca' or 'vire à esquerda no banco'? Wait, no. Let me check: In Portuguese, 'banco' (the place) is masculine, so it should be 'no banco' (with the definite article 'o'). But the phrase 'vire à esquerda no banco' is incorrect because 'no' is a contraction of 'em' + 'o', but in this context, it's 'at the bank' (the place), so it should be 'vire à esquerda no banco'. Wait, but the scene facts say 'turn left at the bank', so it's 'vire à esquerda no banco'. However, the answer says 'no banco', which is correct? Wait, no. Let me think again. In Portuguese, when you say 'at the bank', it's 'no banco' (the bank building). But the phrase 'vire à esquerda no banco' is incorrect because 'no' is a contraction of 'em' + 'o', but in this context, it's 'at the bank', so it should be 'vire à esquerda no banco'. Wait, but the answer says 'no banco', which is correct? Wait, no. The scene facts say 'turn left at the bank', so it's 'vire à esquerda no banco'. But the answer says 'no banco', which is correct? Wait, I think the mistake is that 'banco' here means 'bank' (the financial institution), so it should be 'no banco' (with the definite article 'o'). But the answer says 'no banco', which is correct? Wait, no. Let me check the rules: The answer is 

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `moro` -> `morerei` [verb_form] The present perfect continuous tense is not used in Portuguese for this question. The correct tense is present perfect (perfeito) or simple past (pretérito), but for 'how long have you lived here' the correct tense is present perfect continuous (perfeito composto) or simple present (presente) for duration.
- Missed: `tenho visitado`

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Espero` -> `Espero que você possa vir à festa e que traga seus amigos.` [verb_form] The verb 'esperar' is used in the present tense, but the context requires a future tense for an invitation.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The conditional tense should be 'ganhasse' but the verb form is incorrect for the conditional mood.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `podermos` -> `podéssemos` [verb_form] The verb form should be in the past perfect subjunctive, but 'podermos' is in the present subjunctive.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `estuda`
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `do` -> `do` [article_preposition] The preposition 'de' is missing the article 'o' before 'atraso'.
- Missed: `sobre do`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajarei` [verb_form] The verb should be in the present perfect tense for future actions in the past, but in this context, it's a simple future tense that's correct for a direct report of a future action.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `incêndios` -> `incêndios` [spelling] The word 'incêndios' is spelled with a cedilla on the 'c' in Portuguese, but the correct spelling is 'incêndios' with a cedilla.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `no dia 10` [article_preposition] In Portuguese, numbers from 21 to 99 are written with a space and the word 'de' before the number, but for numbers 1 to 20, it's written as 'dez' without 'de'.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `lhe` -> `lhe` [pronoun] The pronoun 'lhe' is used for indirect object pronouns in the past tense, but in this context, the correct pronoun for indirect object in the past tense is 'lhe' for 'to him' when the verb is in the past tense. However, in Brazilian Portuguese, the indirect object pronoun 'lhe' is used correctly here. Wait, let me double-check.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `preocupo` -> `preocupo` [verb_form] The verb 'preocupar' is conjugated incorrectly; it should be 'preocupo' for 'I worry' in present tense.
- Missed: `eu preocupo`

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu diverti`

### a1-short-verb-morar (erroneous, a1)

- Task: Where do you live?
- Answer: Eu mora em Lisboa.
- Reference: `mora` -> moro [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `mora`

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

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `fui` -> `fui` [verb_form] The past tense of 'ser' is 'fui' but here it should be the past tense of 'ir' (to go).
- Missed: `sempre fui`

### a2-not-brazilian-phone (erroneous, a2)

- Task: Where is your phone?
- Answer: O meu telemóvel está em cima do sofá.
- Reference: `telemóvel` -> celular [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `telemóvel`

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Estudo` -> `Estudo` [verb_form] The verb should be in the present tense for the first person singular, but 'Estudo' is the present tense form. However, in Portuguese, the present tense for 'estudar' in first person singular is 'Estudo', so this is correct.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `convidei-a` -> `convidei` [verb_form] The verb 'convidar' requires the preterite tense for past actions, but 'convidei' is in the infinitive form.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=true

## qwen3:4b / few_shot_reversed

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maiô` -> `banho` [word_choice] The word 'maiô' is used in Brazil for a swimsuit, but it's more common to say 'banho' or 'roupa de banho' for swimsuit in this context. However, 'maiô' is acceptable in Brazil for a one-piece swimsuit.

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `manha` -> `manhã` [spelling] The word 'manhã' needs an accent on the a.
- Missed: `cafe`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [agreement] The word 'escritório' is singular, but the context requires a plural form because 'uma escritório' is incorrect; it should be 'um escritório' for singular.
- Missed: `uma escritório`

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `jantar` -> `jantar` [word_choice] The verb 'jantar' is used for the action of eating dinner, but in this context, the learner meant 'dinner' as a noun.
- Missed: `jantar`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chamo`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `almoço` -> `meu almoço` [word_choice] The word 'almoço' means 'lunch' but in this context it's used as a noun for the meal, which is incorrect here. The correct phrase should be 'No meu almoço' or 'Para o almoço' to indicate what you eat for lunch.
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective must be in the correct form for the noun 'camisa' (feminine).
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `muito` -> `muito` [word_order] The adverb 'muito' should be placed after the noun in this context.
- Missed: `gosto de café muito`

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `jaqueta` -> `verde` [content_mismatch] The color should be 'green' (verde) not 'blue' (azul) as per scene facts.
- Missed: `azul`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `gostei` [verb_form] The verb 'gosto' is present tense, but the question asks for past tense.

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The word 'cafe' should be 'café' with an accent.
- Found (matched): `onibus` -> `ônibus` [spelling] The word 'onibus' should be 'ônibus' with an accent and the correct spelling.
- Missed: `as nove`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `fiquei` -> `fiquei` [verb_form] The verb 'fiquei' is in the past perfect tense, but the scene facts say you were tired and slept all day, so the correct tense is past simple.
- Missed: `era cansado`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective must agree with the feminine gender of 'amiga'.
- Found (false positive): `cabelos` -> `cabelo` [agreement] The plural form is incorrect; it should be 'cabelo' (singular) for hair in Portuguese.
- Found (false positive): `pretos` -> `preto` [agreement] The adjective must agree with the singular form of 'cabelo' (hair).
- Missed: `curtas`

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

### a2-not-brazilian-gerund (erroneous, a2)

- Task: What are you doing now?
- Answer: Estou a comer pão com queijo na cozinha.
- Reference: `a comer` -> comendo [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a comer`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `autocarro`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pequeno-almoço`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo` -> `chovendo` [content_mismatch] The weather is rainy and cold, but the answer says it's sunny and hot.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no banco` -> `na banco` [article_preposition] The preposition 'no' is used for 'no' (in the) but 'banco' means 'bank' (the financial institution), not 'bank' (the river bank).

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `num` -> `no` [article_preposition] The contraction 'no' is used for 'no' in Portuguese, not 'num'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `moro` -> `vivo` [verb_form] The verb 'morrer' is used for death, but here it should be 'vivo' for 'live'.

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `você` -> `você` [spelling] In Brazilian Portuguese, the pronoun 'você' is used for 'you' in informal contexts, but the verb 'pode' requires the pronoun to be in the informal form. However, in this context, the sentence is correct as it uses the informal 'você' for the second person singular. Wait, let me think again. The sentence is: 'Espero que você possa vir à festa e que traga seus amigos.' This is correct in Brazilian Portuguese. The verb 'pode' is conjugated correctly for the informal 'você'. So there's no mistake here.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The conditional tense should be 'ganhasse' for 'eu' (first person singular), but the verb form is correct for the past subjunctive.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `você` [pronoun] The pronoun should be 'você' for the indirect object pronoun in this context.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `folga` -> `dia de folga` [word_choice] In Brazilian Portuguese, 'folga' means 'leave' in the context of work, but 'dia de folga' is the correct phrase for 'day off'.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] The verb should be in the present tense for habitual actions, not the present perfect tense.
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `do` -> `do` [article_preposition] The preposition 'de' is missing the article 'o' before 'atraso'.
- Missed: `sobre do`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajarei` [verb_form] The verb should be in the present perfect tense for future actions in the past, but in this context, it's a simple future tense that's correct for a direct statement of future action.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `incêndios` -> `incêndios` [spelling] The word 'incêndio' is masculine, so it should be plural with an accent: incêndios.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `no` -> `no` [spelling] The contraction 'no' is incorrect; it should be 'no' (with accent) for 'no dia'.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `preocupo` -> `preocupo` [verb_form] The verb 'preocupar' is conjugated with -u instead of -o for the first person singular present tense.
- Missed: `eu preocupo`

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `diverti` -> `diverti` [verb_form] The verb should be in the past perfect tense (pretérito perfeito) for the action of having fun.
- Missed: `eu diverti`

### a1-short-verb-morar (erroneous, a1)

- Task: Where do you live?
- Answer: Eu mora em Lisboa.
- Reference: `mora` -> moro [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `mora`

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
- Found (changes nothing): `doente` -> `doente` [verb_form] The adjective must be in the present participle form for the verb 'ser' in this context.
- Missed: `é doente`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `fui` -> `fui` [verb_form] The past tense of 'ser' is used for age, not for actions in the past.
- Missed: `sempre fui`

### a2-not-brazilian-phone (erroneous, a2)

- Task: Where is your phone?
- Answer: O meu telemóvel está em cima do sofá.
- Reference: `telemóvel` -> celular [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `telemóvel`

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de` -> `com` [article_preposition] The preposition 'de' is used for clothing, but in this context, it's better to use 'com' for 'wearing'.

### a2-control-family-meal (control, a2)

- Task: Describe the meal these people are eating.
- Answer: A família está comendo arroz, feijão e frango grelhado. Eles bebem suco de laranja.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `frango` -> `frangos` [agreement] The word 'frango' is singular, but the scene facts say 'grilled chicken' (plural).

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `trabalhei` -> `trabalhei` [verb_form] The past tense of 'trabalhar' should be 'trabalhei' for the preterite tense, but in Brazilian Portuguese, the preterite is often used with 'fazer' for completed actions, and 'trabalhar' uses the preterite correctly here. Wait, no, 'trabalhei' is correct for the preterite tense. Let me check again.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `convidei-a` -> `convidei` [verb_form] The verb 'convite' is in the wrong tense; it should be 'convidar' in the past perfect tense.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje` -> `Meu nome é` [word_order] The sentence should start with 'Meu nome é...' for the question 'What is your name?'

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `comprei` -> `não sei` [content_mismatch] The verb 'comprar' is in the past tense, but the question is about opinions on working from home, not a recent purchase.

## qwen3:4b / few_shot_six

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha` -> `De manhã` [article_preposition] The word 'manhã' needs an accent and should be written with a contraction for 'de' and 'a'.
- Missed: `cafe`

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

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No` -> `No` [article_preposition] The preposition 'no' is incorrect; it should be 'No' for 'na' (in the) but the correct phrase is 'No almoço' meaning 'At lunch'.
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective must agree with 'camisa' (feminine).
- Found (false positive): `calça` -> `calças` [agreement] The noun must be plural to agree with 'uma' (one).
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `muito` -> `muito` [word_order] The adverb 'muito' should be placed after the noun in this context.
- Missed: `gosto de café muito`

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `jaqueta` -> `jaqueta verde` [word_choice] The color should be 'green' (verde) for the jacket, not 'blue' (azul).
- Missed: `azul`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `comi` [verb_form] The verb must be in the past tense for 'what did you eat' (past simple).

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The word 'cafe' is written without an accent: café.
- Found (matched): `onibus` -> `ônibus` [spelling] The word 'onibus' is written without an accent: ônibus.
- Missed: `as nove`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Response: invalid or missing

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective must agree with 'amiga' (feminine).
- Found (changes nothing): `cabelos` -> `cabelos` [agreement] The noun must be plural to agree with 'cabelos' (plural).
- Found (changes nothing): `pretos` -> `pretos` [agreement] The adjective must agree with 'cabelos' (plural).
- Missed: `curtas`

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
- Missed: `nós ajudamos`

### a2-not-brazilian-gerund (erroneous, a2)

- Task: What are you doing now?
- Answer: Estou a comer pão com queijo na cozinha.
- Reference: `a comer` -> comendo [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a comer`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `autocarro`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pequeno-almoço`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo` -> `está chovendo e frio` [content_mismatch] The weather is rainy and cold, but the answer says it's sunny and hot.

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vou` -> `vou` [verb_form] The verb 'vou' is in the present tense, but the question asks for future plans.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no banco` -> `na banca` [article_preposition] The preposition 'no' is used for 'no banco' (in the bank), but the scene facts say 'turn left at the bank', which requires 'na' for 'na banca' (at the bank).

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ao` -> `no` [article_preposition] The preposition 'a' contracts with the definite article 'o' to 'ao' but in this context, it should be 'no' for 'in the cinema'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `moro` -> `They are eating spaghetti and drinking red wine` [content_mismatch] The verb must agree with 'eu': moro is correct for first person singular, but the sentence is about what they are eating, not where they live.

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nunca` -> `nunca tenho visitado` [verb_form] The verb 'visitar' should be in the present perfect tense, but 'nunca' is an adverb that doesn't need to be conjugated.
- Missed: `tenho visitado`

### b1-content-date (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1850 e restaurada em 2005.
- Reference: `1850` -> 1750 [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `1850`

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The verb must be in the conditional tense for the past subjunctive: ganhasse.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `estuda`
- Missed: `assiste`
- Missed: `exige`

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

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `doente` -> `doente` [agreement] The adjective must agree with 'irmão' (masculine singular).
- Missed: `é doente`

### a2-not-brazilian-phone (erroneous, a2)

- Task: Where is your phone?
- Answer: O meu telemóvel está em cima do sofá.
- Reference: `telemóvel` -> celular [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `telemóvel`

### a2-control-preterite-sunday (control, a2)

- Task: What did you do on Sunday?
- Answer: No domingo eu acordei tarde, fiz o almoço e depois assisti a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No` -> `No` [article_preposition] The article 'o' is missing before 'domingo' in this context.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Trabalhei` -> `Trabalhei` [verb_form] The verb must agree with 'eu': trabalhei is correct for 'I worked', but the tense is past perfect which is not used here. For 'I worked' in the past, it should be 'Trabalhei' (simple past), but the context suggests a simple past tense is correct. Wait, let me check again. In Portuguese, the simple past tense for 'I worked' is 'Trabalhei' (pretérito perfeito), which is correct. So maybe there's no mistake here?

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `convidei-a` -> `convidei` [verb_form] The verb 'convite' is in the wrong form; it should be 'convidei' for the past tense.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje` -> `Meu nome é` [content_mismatch] The question 'What is your name?' does not require the time expression 'today'.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `comprei` -> `acho` [verb_form] The verb must be in the present tense to answer 'what do you think', not past tense.

## qwen3:4b / one_shot_correct

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A` -> `Marina` [article_preposition] The definite article 'a' is used for feminine nouns in Portuguese, but 'Marina' is a proper noun and doesn't require an article.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô` -> `Estou` [spelling] Spoken Brazilian forms are correct; do not report them

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha` -> `de manhã` [spelling] The correct spelling is 'de manhã' with an accent on the 'a'
- Missed: `cafe`

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estou` -> `sou` [ser_estar] The verb 'estar' is used for temporary states, but 'ser' is needed for nationality
- Found (quote not in the answer): `brasiliero` -> `brasileiro` [word_choice] The word 'brasiliero' is a false friend from Italian, the correct Portuguese word is 'brasileiro'

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `moro Toronto`

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `jantar` -> `comer` [word_choice] The verb for 'to eat dinner' is 'comer', not 'jantar' (which means 'to dinner' as a noun)
- Found (changes nothing): `televisão` -> `televisão` [spelling] The correct word for 'television' in Portuguese is 'televisão' (with an accent), but the learner wrote it without the accent

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

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No almoço` -> `No almoço` [article_preposition] The preposition 'no' is incorrect; it should be 'no' for 'in the' but in Portuguese, the correct preposition for 'at lunch' is 'no almoço' (with 'no' being the contraction of 'não' and 'o'?) Wait, no, in Portuguese, 'no almoço' is correct for 'at lunch'. Let me check: In Brazilian Portuguese, 'no almoço' is the correct phrase for 'at lunch'. However, the learner wrote 'No almoço' with a capital 'N' which is incorrect because in Portuguese, the first word of a sentence is capitalized, but here the phrase starts with 'No' which is a preposition and should be lowercase. Wait, but the task is about what do you eat for lunch, so the answer is a sentence starting with 
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective should be in the same gender as the noun it describes. 'Camisa' is feminine, so it should be 'vermelha' (feminine) but the adjective is written in masculine form here.
- Found (false positive): `calça` -> `preta` [agreement] The adjective should be in the same gender as the noun it describes. 'Calça' is feminine, so it should be 'preta' (feminine) but the adjective is written in masculine form here.
- Missed: `vermelha camisa`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu gosto de futebol` -> `Eu como pão com manteiga` [content_mismatch] The question asks about breakfast, but the answer talks about football

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] In Brazilian Portuguese, coffee is 'café' with an accent
- Found (matched): `onibus` -> `ônibus` [spelling] In Brazilian Portuguese, the word for bus is 'ônibus' with a tilde and a capital letter for the first letter of the word in some contexts
- Missed: `as nove`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `fiquei` -> `fiquei` [verb_form] The verb 'fiquei' is in the past perfect tense, but the scene facts say you were tired and slept all day, so the correct tense for 'to stay home' is the simple past (pretérito perfeito) or imperfect (imperfective) tense.
- Found (matched): `era` -> `estava` [verb_form] The verb 'era' is in the imperfect tense, but the scene facts say you were tired and slept all: the correct tense for 'to be tired' is the simple past (pretérito perfeito) or imperfect (imperfective) tense.
- Found (changes nothing): `dormi` -> `dormi` [verb_form] The verb 'dormi' is in the simple past tense, but the scene facts say you slept all day, so the correct tense for 'to sleep' is the imperfect tense (imperfective) for actions that happened over a period of time.

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective must agree with the feminine gender of the friend
- Found (false positive): `cabelos` -> `cabelo` [agreement] The plural form is incorrect; the correct term for 'hair' in Portuguese is 'cabelo' (masculine singular)
- Missed: `curtas`

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

### a2-not-brazilian-gerund (erroneous, a2)

- Task: What are you doing now?
- Answer: Estou a comer pão com queijo na cozinha.
- Reference: `a comer` -> comendo [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a comer`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `autocarro`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pequeno-almoço`

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no banco` -> `no banco do rio` [word_choice] In Brazilian Portuguese, 'banco' means 'bank' (the financial institution), but the scene facts say 'bank' as in the river bank or riverbank

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `moro` -> `morerei` [verb_form] The present perfect tense is needed for 'how long have you lived here'
- Found (false positive): `nunca` -> `nunca visitei` [verb_form] The phrase 'nunca tenho visitado' is incorrect because 'nunca' should be used with the present perfect tense for actions that have happened at some point in the past
- Missed: `tenho visitado`

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós podermos` -> `para que nós possamos` [verb_form] The pronoun 'nós' should be in the third person plural form for the verb 'poder' to match the subject 'nós' (we).

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `estuda`
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `do` -> `da` [article_preposition] The preposition 'de' is missing the contraction with 'a' to form 'da'
- Found (changes nothing): `no` -> `no` [article_preposition] The preposition 'no' is incorrect; it should be 'no' for 'no dia' but the correct contraction is 'no' for 'no' (in the) but here it's 'no dia' so it should be 'no' (in the) but the correct form is 'no dia' without contraction

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `incêndios` -> `incêndio` [word_choice] The word 'incêndios' is plural but the verb 'apagar' requires a singular object for this context

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `no dia 10` [article_preposition] In Brazilian Portuguese, numbers from 1 to 20 are written with cardinal numbers (dez) without the article and without the preposition 'no' for the day.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `preocupo` -> `preocupo` [spelling] The verb 'preocupar' is conjugated with -o for first person singular, but the correct form is 'preocupo' with an 'o' instead of 'u'.
- Missed: `eu preocupo`

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

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Response: invalid or missing

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `fui` -> `fui` [verb_form] The past tense of 'ser' is used for age, not for actions
- Missed: `sempre fui`

### a2-not-brazilian-phone (erroneous, a2)

- Task: Where is your phone?
- Answer: O meu telemóvel está em cima do sofá.
- Reference: `telemóvel` -> celular [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `telemóvel`

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para que` -> `para que eu passe` [verb_form] In Brazilian Portuguese, 'para que' is used with the subjunctive mood, but here it's used with the indicative mood

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `convidei-a` -> `convidei` [verb_form] The verb 'convite' is used in the infinitive form, but here it should be the past tense (pretérito perfeito) for the action of inviting.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Hoje está chovendo muito e eu estou em casa.` -> `João` [content_mismatch] The answer does not answer the question 'What is your name?'

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu comprei` -> `Eu acho que trabalhar em casa é bom` [content_mismatch] The question is about what the person thinks about working from home, but the answer talks about buying a bike on Saturday, which is unrelated.

## qwen3:4b / one_shot_mistake

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maiô` -> `banho` [word_choice] In Brazilian Portuguese, 'swimsuit' is 'banho' or 'roupa de banho', not 'maiô'.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô` -> `Estou` [not_brazilian] This is a spoken form that is acceptable in Brazilian Portuguese for 'I am' in informal contexts.

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha` -> `de manhã` [spelling] The correct preposition for 'in the morning' is 'de manhã' with an accent on the 'a' and a space before 'eu'.
- Missed: `cafe`

### a1-agreement-trousers (erroneous, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: Marina veste uma camiseta azul e uma calça pretos.
- Reference: `pretos` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `calça` -> `calças` [agreement] The plural form of 'calça' is 'calças', but here it's used in the context of 'uma calça' (singular) for trousers.
- Missed: `pretos`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [spelling] In Brazilian Portuguese, the correct term for 'office' is 'escritório' (with a circumflex accent on the o).
- Missed: `uma escritório`

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `brasiliero` -> `brasileiro` [spelling] The word needs to be 'brasileiro' with an accent on the e.
- Missed: `estou brasileiro`

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Canadá` -> `canadá` [spelling] In Brazilian Portuguese, the country names are not capitalized unless they start a sentence or are part of a proper noun.
- Missed: `moro Toronto`

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `jantar` -> `comer` [word_choice] The verb 'to eat dinner' is 'comer' in Portuguese, not 'jantar'.
- Found (changes nothing): `televisão` -> `televisão` [spelling] The correct word for 'television' in Portuguese is 'televisão' (with a tilde), but the answer has it without the tilde.

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `amiga` -> `amigo` [content_mismatch] The question asks for your friend's name, but the answer refers to a female friend.
- Missed: `chama`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `feijão` [word_choice] The word 'no' is incorrect here; it should be 'No' as in 'At lunch' but in Portuguese, the correct phrase is 'No almoço' meaning 'At lunch' but the word 'no' is a contraction of 'n' (preposition) and 'o' (article). However, in this context, the phrase 'No almoço' is correct. Wait, let me think again. In Portuguese, 'no' is a contraction of 'em' + 'o' for 'in the', but in this context, the phrase 'No almoço' is correct. Actually, the problem might be that the word 'No' is capitalized incorrectly. But in Portuguese, the first word of a sentence is capitalized, so 'No' is correct. Wait, the answer is: 'No almoço eu como arroz, beans e frango.' The word 'No' is capitalized, which is correct for the start of the sentence. But the problem might be that 'No' is a contraction of 'em' + 'o', but in this context, it's correct. Let me check the rules. The rules say: 'Do not report style, correct but unusual wording, or punctuation.' So maybe there's no mistake here? Wait, the task is: 'What do you eat for lunch?' The answer is: 'No almoço eu como arroz, beans e fr. Wait, the answer has 'beans' which is English. The task says: Level: A1. In A1, learners might use English words. But the rules say: 'Do not report style, correct but unusual wording, or punctuation.' So maybe the mistake is that 'beans' is in English, but the task is in English, so the answer should be in Portuguese. Wait, the answer has 'beans' which is English, so that's a word choice mistake. But the rules say: 'word_choice: a wrong word for the meaning, a false friend, or a word from another language'. So 'beans' is a word from English, so it's a word choice mistake. But the answer has 'beans' which is English, so it's a wrong word for the meaning. Let me see the rules again. The task is in English, but the answer is in Portuguese. The answer has 'beans' which is English, so it's a word from another language. So the error_text is 'beans'. But the answer says: 'No almoço eu como arroz, beans e frango.' So the word 'beans' is in English. The correct Portuguese word for beans is 'feijão'. So the mistake is 'beans' is a wrong word for the meaning. So the error_text is 'beans', explanation: 
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective should agree with the noun 'camisa' (feminine) and be in the correct form for the context.
- Found (false positive): `calça` -> `calças` [agreement] The word 'calça' is singular, but the scene facts say Rafael wears black trousers (plural).
- Missed: `vermelha camisa`

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `jaqueta` -> `verde` [content_mismatch] The color should be 'green' (verde), not 'blue' (azul).
- Missed: `azul`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `comi` [verb_form] The verb 'gosto' is present tense, but the question asks for past tense (what did you eat).

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `trabalhei` -> `trabalhei` [verb_form] The past tense of 'trabalhar' should be 'trabalhei' for the preterite tense, but in Brazilian Portuguese, the preterite is often used with 'fazer' for actions that happened in the past, so 'trabalhei' is correct for the preterite tense.
- Found (changes nothing): `estudei` -> `estudei` [verb_form] The past tense of 'estudar' should be 'estudei' for the preterite tense, but in Brazilian Portuguese, the preterite is often used with 'fazer' for actions that happened in the past, so 'estudei' is correct for the preterite tense.
- Missed: `portugues`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] In Brazilian Portuguese, coffee is 'café' with an accent.
- Found (matched): `onibus` -> `ônibus` [spelling] The correct spelling is 'ônibus' with an accent and a tilde.
- Found (changes nothing): `cheguei` -> `cheguei` [verb_form] The verb 'cheguei' is in the past tense, but the correct form for 'I arrived' in the past tense is 'cheguei' (with a 'u' instead of 'e').
- Missed: `as nove`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `visitei` -> `vi` [verb_form] The past tense of visitar is 'visitei' but in Brazilian Portuguese, the correct past tense for 'to visit' is 'vi' (pretérito perfeito), not 'visitei' (pretérito perfeito composto).
- Found (changes nothing): `almoçamos` -> `almoçamos` [verb_form] The verb 'almoçar' is in the first person plural, but the subject is 'nós' (we), so it should be 'almoçamos' (correct).
- Found (matched): `volti` -> `voltei` [spelling] The verb 'voltar' in the past tense should be 'voltei' (pretérito perfeito), not 'volti' (a misspelling).

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `fiquei` -> `fiquei` [verb_form] The verb 'fiquei' is in the past perfect tense, but the scene facts say you slept all day, so it should be in the past simple tense.
- Missed: `era cansado`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective must agree with the feminine subject 'amiga' (it should be feminine).
- Found (false positive): `cabelos` -> `cabelo` [agreement] The plural form is incorrect; it should be singular for 'cabelo' (the hair is singular in Portuguese).
- Found (false positive): `pretos` -> `preto` [agreement] The adjective must agree with the singular noun 'cabelo' (it should be singular).
- Missed: `curtas`

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pequeno-almoço`

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no banco` -> `na banco` [article_preposition] The preposition 'no' is incorrect; it should be 'na' for 'bank' (feminine).

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `num` -> `no` [article_preposition] The contraction 'no' is used for 'no' in Portuguese, not 'num'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `moro` -> `tenho morado` [verb_form] The present perfect tense is needed for 'how long have you lived here', but 'moro' is present simple.
- Missed: `tenho visitado`

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Espero` -> `Espero que você possa vir à festa e que traga seus amigos.` [verb_form] The verb 'esperar' is used in the present tense, but the context requires a future tense for an invitation.

### b1-control-future-subjunctive (control, b1)

- Task: What will you do when you finish the course?
- Answer: Quando eu terminar o curso, vou procurar um emprego novo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `emprego` -> `trabalho` [word_choice] In Brazilian Portuguese, the word for 'job' is 'emprego' but it's more common to use 'trabalho' in this context.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The conditional tense should be 'ganhar' for the first person singular, but the verb form is incorrect for the conditional tense.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós podermos` -> `para que nós possamos` [verb_form] The phrase 'para nós podermos' is incorrect; it should be 'para que nós possamos' for the subjunctive mood after 'para que'.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `folga` -> `férias` [word_choice] In Brazilian Portuguese, 'folga' means 'time off' but in this context, 'dia de folga' is not the correct phrase for 'day off'. The correct phrase is 'dia de folga' or 'férias' for vacation, but 'folga' alone is not used for a day off.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `estuda`
- Missed: `assiste`
- Missed: `exige`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajarei` [verb_form] The verb should be in the present perfect tense (pretérito perfeito) for the past action, but the correct form for 'I will travel' in the past is 'viajarei' (future) or 'viajou' (past) depending on context.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `incêndios` -> `incêndios` [word_choice] The word 'incêndio' is singular in the context of the sentence, but the verb 'apagar' requires a plural form for multiple fires.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `preocupo` -> `preocupo` [verb_form] The verb 'preocupar' is conjugated incorrectly; it should be 'preocupo' for first person singular.
- Missed: `eu preocupo`

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu diverti`

### a1-short-verb-morar (erroneous, a1)

- Task: Where do you live?
- Answer: Eu mora em Lisboa.
- Reference: `mora` -> moro [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `mora`

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `doente` -> `doente` [not_brazilian] In Brazilian Portuguese, 'doente' is not commonly used; 'doente' is more European Portuguese. The correct term for 'sick' in Brazilian Portuguese is 'doente' but it's less common. However, in A2 level, 'doente' is acceptable. Wait, let me check again.
- Missed: `é doente`

### b1-verb-form-para-que (erroneous, b1)

- Task: Why do you study every day?
- Answer: Eu estudo todos os dias para que eu passo no exame.
- Reference: `eu passo` -> eu passe [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `passo` -> `passo` [verb_form] The verb 'passar' in the present tense for 'eu' should be 'paso' in Brazilian Portuguese, but in this context, the correct form is 'passo' (the verb 'passar' is conjugated as 'passo' for 'eu' in Brazilian Portuguese).
- Missed: `eu passo`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `fui` -> `fui` [verb_form] The past tense of 'ser' is 'fui' but here it should be the past tense of 'ir' (to go).
- Missed: `sempre fui`

### a2-not-brazilian-phone (erroneous, a2)

- Task: Where is your phone?
- Answer: O meu telemóvel está em cima do sofá.
- Reference: `telemóvel` -> celular [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `telemóvel`

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pra` -> `para` [not_brazilian] In Brazilian Portuguese, 'pra' is informal and not used in formal writing. The task is for a learner at A1 level, but the answer uses 'pra' which is acceptable in spoken language. However, the rule says: 'Spoken Brazilian forms are correct; do not report them: tô, a gente, pra, né, você for 'you'.'

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Na minha bolsa` -> `minha bolsa` [article_preposition] The preposition 'na' is incorrect; it should be 'na minha bolsa' without the preposition 'na' because 'bolsa' is the direct object.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A` -> `ela` [pronoun] The article 'a' is incorrect; it should be 'ela' for the subject.

### a2-control-family-meal (control, a2)

- Task: Describe the meal these people are eating.
- Answer: A família está comendo arroz, feijão e frango grelhado. Eles bebem suco de laranja.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `frango grelhado` -> `frango grelhado` [agreement] The phrase 'frango grelhado' should be written with the adjective agreeing in gender and number with the noun 'frango'.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Estudo` -> `Estudo` [verb_form] The verb should be in the present tense for the question 'Why do you study every day?'

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `convidei-a` -> `convidado-a` [verb_form] The verb 'convidar' should be in the past perfect tense (convidado) for the correct past perfect tense construction.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está chovendo muito e eu estou em casa.` -> `João` [content_mismatch] The answer does not answer the question 'What is your name?'

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei` -> `Nada` [content_mismatch] The question is about what you think about working from home, but the answer talks about buying a bicycle.

## qwen3:4b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maiô` -> `banho` [word_choice] In Brazilian Portuguese, 'swimsuit' is 'banho' or 'roupa de banho', not 'maiô'.
- Found (false positive): `chapéu` -> `palha` [word_choice] In Brazilian Portuguese, 'hat' is 'chapéu' but it's not used for a straw hat in this context. The correct term for a straw hat is 'palha' or 'chapéu de palha'.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô` -> `Estou` [not_brazilian] Tô is a spoken Brazilian form; the task says to use standard Portuguese
- Found (false positive): `cansado` -> `cansada` [agreement] cansado is masculine; the subject is feminine (you) so it should be cansada

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha` -> `de manhã` [spelling] The correct phrase for 'in the morning' is 'de manhã' with an accent on the 'a'.
- Found (changes nothing): `bebo` -> `bebo` [verb_form] The verb should be in the present tense for 'I drink' but 'bebo' is the first person present of 'beber' (to drink), so it's correct. Wait, no, let me check again. In Portuguese, the verb 'beber' in the present tense for 'I' is 'bebo', so this is correct. Maybe this is not a mistake.
- Missed: `cafe`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `escritório` -> `o escritório` [agreement] In Brazilian Portuguese, the word for 'office' is 'escritório' but it should be used with the definite article 'o' when referring to a specific office.
- Found (changes nothing): `no` -> `no` [article_preposition] The contraction 'no' is not used in Brazilian Portuguese; it should be 'no' as a contraction of 'em + o' but in this context, it's better to use 'no' as a contraction for 'em o' which is correct in Brazilian Portuguese.

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `brasiliero` -> `brasileiro` [spelling] The adjective for 'Brazil' in Portuguese is 'brasileiro' (not 'brasiliero')
- Missed: `estou brasileiro`

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `sou` -> `estou` [verb_form] The verb 'ser' is used for permanent states, but 'cansado' is a temporary state after work.
- Found (false positive): `com` -> `e` [word_order] The correct phrase for 'feeling hungry' is 'com fome' but the structure here is incorrect because 'cansado' should be followed by 'e' without a preposition.

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `moro` -> `morava` [verb_form] The verb 'moro' is in the present tense but should be in the past tense for the location where you currently live
- Found (changes nothing): `Toronto` -> `Toronto` [content_mismatch] Toronto is a city in Canada, not in Brazil

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `jantar` -> `jantar` [word_choice] jantar is a verb meaning 'to eat dinner', but here it's used as a noun (dinner).
- Found (quote not in the answer): `assistos` -> `assisto` [verb_form] The verb 'assistir' should be in the present tense, not past tense.
- Missed: `jantar`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `chamo` -> `chamo` [verb_form] The verb 'chamar' is used in the present tense for 'I call', but here it's used in the past tense for 'I am called'.
- Found (changes nothing): `vinte` -> `vinte-` [spelling] The number 'twenty' in Portuguese is 'vinte' but it should be written with a hyphen as 'vinte-'.
- Missed: `chamo`

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
- Found (false positive): `No` -> `Não` [article_preposition] The word 'no' is a contraction of 'não' in Portuguese, but here it's used as a preposition meaning 'at' or 'on', which is incorrect. In this context, 'não' is the negative form of 'ser' and should be used with a space for negation.
- Found (changes nothing): `almoço` -> `almoço` [article_preposition] The word 'almoço' is the direct object of the verb 'comer' (to eat), but in this context, it should be the time of day (lunch) and thus should be in the prepositional phrase 'no almoço' (at lunch). However, the phrase 'no almoço' is used here as a prepositional phrase, so 'almoço' is correct but the structure is incorrect because 'no' is a contraction of 'não' and should be 'no' (with a space) for the preposition 'em' (at) or 'no' (at) in this context.
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective must agree with the noun 'camisa' which is feminine, but 'vermelha' is masculine form
- Found (changes nothing): `calça` -> `calça` [agreement] The noun 'calça' is feminine, but the article 'uma' is used for masculine nouns
- Missed: `vermelha camisa`

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `azul` -> `verde` [content_mismatch] The scene says Rafael wears a green jacket, not blue.
- Found (false positive): `calça` -> `jeans` [agreement] In Brazilian Portuguese, jeans are called 'jeans' without an article, and 'calça' is the singular form for pants, but the scene says 'jeans' (plural).

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `onibus` -> `ônibus` [spelling] The word 'ônibus' is written without the accent in Brazilian Portuguese
- Found (false positive): `cheguei` -> `tinha chegado` [verb_form] The verb 'chegar' should be in the past perfect tense for the time expression 'as nove'
- Missed: `cafe`
- Missed: `as nove`

### a2-verb-form-imperfect (erroneous, a2)

- Task: Describe your childhood.
- Answer: Quando eu era criança, eu moro numa cidade pequena e brincava na rua.
- Reference: `moro` -> morava [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `moro` -> `morava` [verb_form] The verb 'moro' is present tense, but the sentence is in the past tense (when I was a child).
- Found (false positive): `brincava` -> `brincava na rua` [word_order] The verb 'brincava' is correct for past tense, but the sentence structure needs a comma after 'cidade pequena' for clarity.

### a2-verb-form-subject-agreement (erroneous, a2)

- Task: What did your family do on Sunday?
- Answer: No domingo minha família almoçaram na casa da minha avó.
- Reference: `almoçaram` -> almoçou [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `almoçaram` -> `almoçaram` [verb_form] The verb should be in the past tense for the third person plural, but 'almoçaram' is the past tense for the third person singular.
- Missed: `almoçaram`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `fiquei` -> `fiquei` [verb_form] The verb 'fiquei' is in the past perfect tense, but the scene says you slept all day, so it should be past simple.
- Found (matched): `era` -> `estava` [verb_form] The verb 'era' is in the past imperfect tense, but the scene says you were tired, so it should be past perfect tense.
- Found (changes nothing): `dormi` -> `dormi` [verb_form] The verb 'dormi' is in the past perfect tense, but the scene says you slept all day, so it should be past simple.

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective 'engraçado' is masculine, but the friend is a woman.
- Found (false positive): `cabelos` -> `cabelo` [word_choice] The plural form 'cabelos' is incorrect for 'short black hair' in this context.
- Missed: `curtas`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `assisti` [word_choice] The verb 'atender' is used for 'to attend' (e.g., a call), but here it should be 'assistir' for 'to watch a movie'.
- Found (changes nothing): `filme` -> `filme` [agreement] In Brazilian Portuguese, 'filme' is masculine, so it should be 'filmes' for plural (since 'um filme novo' implies a specific movie, but the context suggests a general action of watching a movie, so it should be singular).

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

### a2-not-brazilian-gerund (erroneous, a2)

- Task: What are you doing now?
- Answer: Estou a comer pão com queijo na cozinha.
- Reference: `a comer` -> comendo [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Estou` -> `Sou` [verb_form] The verb 'estar' is used for temporary states, but 'estou' is the present tense of 'estar' for first person singular. Here, the action is ongoing and not a temporary state, so 'ser' should be used for habitual actions.
- Found (matched): `a` -> `com` [article_preposition] The preposition 'a' is used for the preposition 'a' in Portuguese, but in this context, it should be 'com' for 'with'.

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pequeno-almoço`

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vou ao mercado` -> `vou ao mercado` [verb_form] The verb 'vou' is in the present tense, but the context requires future tense for 'this weekend'.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `banco` -> `banco` [word_choice] The word 'banco' means 'bank' (the place where you put money) in Portuguese, but the scene says 'turn left at the bank' meaning the financial institution.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `num` -> `no` [article_preposition] The contraction 'no' is missing the 'o' from 'não' in the contraction for 'no'
- Found (false positive): `perto` -> `perto do` [word_order] The adverb 'perto' should be in the correct form for the location of the restaurant

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `não` [spelling] The word 'no' is a contraction of 'não' in Portuguese, but here it's written as 'No' with capital letter, which is incorrect.
- Found (changes nothing): `cachorro` -> `cachorro` [spelling] The word 'cachorro' is correct for 'dog' in Portuguese, but the phrase 'passear com o meu cachorro' is grammatically correct. However, the error is in the capitalization of 'No' which is a spelling mistake.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `ter` -> `tendo` [verb_form] The verb 'ter' is used for possession, not for future tense. The correct future tense verb is 'ter' conjugated as 'tendo' for the future perfect tense, but here it's a simple future tense construction.
- Found (changes nothing): `vou` -> `vou` [verb_form] The verb 'vou' is in the present tense, but it should be in the future tense for the main clause.

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `moro` -> `morerei` [verb_form] The present perfect continuous tense is not used in Portuguese for 'how long have you lived here'.
- Found (changes nothing): `nunca` -> `nunca` [word_choice] The phrase 'nunca tenho visitado' is incorrect because 'nunca' means 'never' and the verb 'visitar' is in the present perfect tense, which is not appropriate here.
- Missed: `tenho visitado`

### b1-agreement-passive (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1750 e foi restaurado em 2005.
- Reference: `restaurado` -> restaurada [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `foi restaurado` -> `foi restaurado` [verb_form] The verb 'restaurar' is used in the past tense, but the correct past tense for 'restaurar' in this context is the past perfect (pretérito perfeito) or past continuous (pretérito imperfeito) depending on the context. However, in Portuguese, the correct past tense for 'restaurar' in this context is the pretérito perfeito (passado perfeito) for the simple past, but the verb 'restaurar' is not used in the past tense here. The correct verb form for 'restored' in Portuguese is 'reformado' or 'restaurado' in the pretérito perfeito (passado perfeito) tense.
- Missed: `restaurado`

### b1-control-future-subjunctive (control, b1)

- Task: What will you do when you finish the course?
- Answer: Quando eu terminar o curso, vou procurar um emprego novo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `novo` -> `novo` [agreement] The adjective 'novo' should agree with the noun 'emprego' which is masculine, so it should be 'novo' (not 'nova').

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ganhasse` -> `ganhar` [verb_form] The conditional tense should be 'ganhar' for the past conditional, but 'ganhasse' is the imperfect subjunctive
- Found (false positive): `compraria` -> `comprar` [verb_form] The conditional tense should be 'comprar' for the past conditional, but 'compraria' is the imperfect subjunctive
- Found (false positive): `viajaria` -> `viajar` [verb_form] The conditional tense should be 'viajar' for the past conditional, but 'viajaria' is the imperfect subj. tense

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós` -> `nos` [article_preposition] The preposition 'para' is used with 'nós' (we) in this context, but 'para' should be followed by the indirect object pronoun 'nos' for the indirect object, not 'nós'.

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `trabalhar` -> `trabalhar` [spelling] The verb 'trabalhar' is used for the action of working, but in this context, the correct verb for 'to work' in the sense of 'to have a job' is 'trabalhar' (it's correct here).
- Found (false positive): `a gente` -> `nós` [not_brazilian] The phrase 'a gente' is a colloquial expression used in Brazil for 'we', but it's not standard in formal writing. However, the task specifies that spoken Brazilian forms are correct, so this is acceptable.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `folga` -> `férias` [word_choice] In Brazilian Portuguese, 'folga' means 'free time' or 'time off', but in this context, 'dia de folga' is not the correct term for 'day off'. The correct term is 'dia de folga' for 'day off' in the context of work.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] The verb 'estudar' should be in the present tense for habitual actions, but 'estuda' is the third person singular present tense of 'estudar' (he/she/it studies). The correct form for 'you' (você) in the present tense is 'estuda' for third person singular, but in this context, it's a recommendation for a friend, so it should be 'estude' (subjunctive mood).
- Found (matched): `assiste` -> `assista` [verb_form] The verb 'assistir' should be in the present tense for habitual actions, but 'assiste' is the third person singular present tense of 'assistir' (he/she/it watches). The correct form for 'you' (você) in the present tense is 'assiste' for third person singular, but in this context, it's a recommendation for a friend, so it should be 'assista' (subjunctive mood).
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `do` -> `da` [article_preposition] The preposition 'de' is missing the contraction with 'a' to form 'da' (from the)
- Found (false positive): `que` -> `o` [article_preposition] The relative pronoun 'que' should be 'que' (correct) but in this context, it's better to use 'o' for specificity, but the main issue is the contraction of 'de' with 'a'

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `incêndios` -> `incêndio` [agreement] The word 'incêndios' is plural but the verb 'tive' is singular, so it should be singular.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no` -> `em o` [article_preposition] The word 'no' is a contraction of 'em' + 'o' and should be written as 'em o' in formal Portuguese

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `lhe` -> `lhe` [pronoun] The pronoun 'lhe' is used for indirect object pronouns in formal contexts, but in Brazilian Portuguese, the informal 'lhe' is not used here. The correct pronoun for indirect object in informal contexts is 'lhe' but in this context, the correct pronoun is 'lhe' for indirect object. Wait, this is confusing. Let me think again.
- Found (changes nothing): `não` -> `não` [spelling] The word 'não' is used correctly here as a negation.
- Found (false positive): `esperasse` -> `esperou` [verb_form] The verb 'esperasse' is in the imperfect subjunctive, but in this context, the correct tense is the past perfect (pretérito perfeito) or the simple past (pretérito imperfeito). Wait, the sentence is in the past tense, so the correct verb form is 'esperasse' for the imperfect subjunctive. Let me check.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `preocupo` -> `preocupo` [verb_form] The verb 'preocupar' is conjugated with -u for first person singular in present tense, but here it's written with -o
- Missed: `eu preocupo`

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu diverti`

### a1-short-verb-morar (erroneous, a1)

- Task: Where do you live?
- Answer: Eu mora em Lisboa.
- Reference: `mora` -> moro [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `mora`

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-article-contraction-a-o (erroneous, a1)

- Task: Where did you go on Saturday?
- Answer: No sábado eu fui a o cinema com meus amigos.
- Reference: `a o` -> ao [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `a` -> `no` [article_preposition] The preposition 'a' is missing before 'cinema' in this context
- Found (changes nothing): `com` -> `com` [article_preposition] The preposition 'com' is missing before 'meus amigos' in this context

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `doente` -> `doente` [word_choice] In Brazilian Portuguese, 'doente' is used for 'sick' but the context here is about work, so 'doente' is not the right term for 'ill' in this context.
- Missed: `é doente`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `fui` -> `fui` [verb_form] The verb 'fui' is past perfect tense, but the context requires simple past tense for habitual actions in the past.
- Missed: `sempre fui`

### a2-not-brazilian-phone (erroneous, a2)

- Task: Where is your phone?
- Answer: O meu telemóvel está em cima do sofá.
- Reference: `telemóvel` -> celular [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `telemóvel`

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Vou` -> `Vou` [verb_form] Vou is present tense, but the task is asking where you are going (future), so it should be a future tense form
- Found (false positive): `Tô` -> `Estou` [pronoun] Tô is informal for 'estou', but the task is asking where you are going (present), so it should be a present tense form

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Na` -> `minha` [article_preposition] The preposition 'na' is incorrect here; it should be 'na minha' without the preposition 'na' because 'minha' is a possessive pronoun that doesn't require a preposition.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A` -> `Está` [article_preposition] The article 'a' is incorrect; in Brazilian Portuguese, the definite article for 'camila' (feminine) is 'a' but here it's used as a preposition, so it should be 'Está' without the article.
- Found (changes nothing): `vestido` -> `vestido` [spelling] The word 'vestido' is correct for 'dress' in Brazilian Portuguese.
- Found (changes nothing): `amarelo` -> `amarelo` [spelling] The color 'amarelo' is correct for 'yellow' in Brazilian Portuguese.
- Found (changes nothing): `e` -> `e` [spelling] The conjunction 'e' is correct for 'and' in Brazilian Portuguese.
- Found (changes nothing): `botas` -> `botas` [spelling] The word 'botas' is correct for 'boots' in Brazilian Portuguese.
- Found (changes nothing): `marrons` -> `marrons` [spelling] The color 'marrons' is correct for 'brown' in Brazilian Portuguese.

### a2-control-preterite-sunday (control, a2)

- Task: What did you do on Sunday?
- Answer: No domingo eu acordei tarde, fiz o almoço e depois assisti a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `não` [spelling] The word 'no' is a contraction of 'não' in Portuguese, but here it's written as 'No' with capital letter, which is incorrect because the first word of a sentence should be capitalized, but 'não' is not capitalized in Portuguese unless it's the start of a sentence. However, in Portuguese, the word 'não' is written with a lowercase letter unless it's the first word of the sentence. But in this context, the learner wrote 'No' with capital N, which is incorrect because 'não' is not capitalized in the middle of a sentence.
- Found (changes nothing): `almoço` -> `almoço` [spelling] The word 'almoço' is correct for 'lunch' in Portuguese, but the learner wrote it with a lowercase letter. However, in Portuguese, nouns are capitalized when they are proper nouns or at the beginning of a sentence, but 'almoço' is a common noun and should be lowercase unless it's the first word of the sentence. But in this context, the learner wrote 'almoço' with lowercase, which is correct. Wait, the problem says: 'Do not report style, correct but unusual wording, or punctuation.' So this is not a mistake.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para que` -> `para que eu possa` [verb_form] In Brazilian Portuguese, 'para que' is used with the subjunctive mood, but here it's used with the indicative mood (passar).
- Found (quote not in the answer): `passar` -> `passe` [verb_form] The verb 'passar' is in the infinitive, but it should be in the subjunctive mood to match the purpose clause.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `cheguei` -> `cheguei` [verb_form] The verb 'cheguei' is in the past tense, but the correct verb form for 'I arrived' in the past perfect tense is 'cheguei' (pretérito perfeito), which is correct here. Wait, no, in Portuguese, 'cheguei' is the correct past tense form for 'I arrived'. Let me double-check.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `falei` -> `falei` [verb_form] The verb 'falei' is past perfect tense, but the context requires simple past tense for the action of inviting.
- Found (changes nothing): `convidei-a` -> `convidei-a` [spelling] The verb 'convidei-a' is incorrect because the correct form should be 'convidei-a' (with a 'c' instead of 'v') for the past tense of 'convidar'.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu` -> `Eu acho que trabalhar em casa é bom` [content_mismatch] The question is about what you think about working from home, but the answer talks about buying a bike.

## qwen3:8b / few_shot

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cansado` -> `cansada` [agreement] The adjective must agree in gender with 'você' (feminine).

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha` -> `De manhã` [spelling] 'De manha' is incorrect; the correct phrase is 'De manhã' with the accent on the 'a'
- Missed: `cafe`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [article_preposition] The word needs an article before it: 'um escritório' or 'uma escritório' (but 'escritório' is masculine, so 'um').
- Missed: `uma escritório`

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `sou` -> `estou` [ser_estar] 'Sou' is the conjugation of 'ser', but 'estar' is needed to describe location.

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `assisto televisão` -> `assistir à televisão` [article_preposition] The preposition 'a' is missing before 'televisão'.
- Missed: `jantar`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `chamo` -> `me chamo` [word_choice] The verb 'chamar' should be in the first person singular present tense: 'chamo' is correct, but the sentence structure is incorrect for answering 'What is your name?'

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `chama` -> `chama` [not_brazilian] The correct form is 'chama' for 'chamar' (to call), but in Brazilian Portuguese, it is more common to use 'se chama' for formal or neutral contexts. However, at A1 level, 'chama' is acceptable. No mistake here.
- Missed: `chama`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective must agree in gender and number with 'camisa' (feminine singular).
- Found (changes nothing): `preta` -> `preta` [agreement] The adjective must agree in gender and number with 'calça' (feminine singular).
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `muito` -> `muito` [word_choice] 'Muito' is not used to describe the degree of liking something in this context. Use 'muito' with a verb or adjective that can take it, or use 'gosto muito de café' with a different structure.
- Missed: `gosto de café muito`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `gostei` [verb_form] The verb should be in the past tense to match the question about what was eaten.

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `era` -> `tinha sido` [verb_form] The verb should be in the past perfect tense (pretérito mais-que-perfeito) to match the context of a completed action in the past.
- Found (false positive): `o dia todo` -> `todo o dia` [word_order] In Brazilian Portuguese, 'todo o dia' is the correct phrase, not 'o dia todo'.

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

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
- Found (changes nothing): `pequeno-almoço` -> `pequeno-almoço` [not_brazilian] This is the European Portuguese spelling; the Brazilian Portuguese spelling is 'pequeno-almoço' without the hyphen.
- Missed: `pequeno-almoço`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `fazendo chuva` [content_mismatch] This contradicts the scene fact that it is raining.
- Found (duplicate): `muito calor` -> `muito frio` [content_mismatch] This contradicts the scene fact that it is cold.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu` -> `Nós` [pronoun] The subject should be plural to match the scene facts about two people.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Espero que você possa vir à festa e que traga seus amigos.` -> `Vou fazer uma festa e quero que você venha.` [word_choice] The sentence is grammatically correct, but it is a suggestion rather than an invitation. An invitation should be more direct.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `` [word_choice] The subject should be 'nós' but it is not needed here because the verb 'podermos' already implies the subject 'nós' is included.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Prezado Sr. Souza` -> `Senhor Souza` [word_choice] 'Prezado' is used for formal letters, but 'Sr.' is not commonly used in Brazilian Portuguese for addressing someone in a letter. Use 'Senhor' instead.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estudar` [verb_form] The verb should be in the infinitive form after 'recomendo que'
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `reclamar` -> `reclamar` [word_choice] The verb 'reclamar' should be in the infinitive form 'reclamar' after 'por meio desta' in formal writing.
- Found (matched): `do` -> `` [article_preposition] 'Do' is not used in this context; it should be omitted.

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `vou viajar` [verb_form] The verb should be in the future tense (vou viajar) instead of the conditional (viajaria) for reported speech in Brazilian Portuguese.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `morta de cansaço` -> `morta de cansaço` [agreement] The adjective must agree in gender with 'morte' (feminine).

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dia dez` -> `10` [not_brazilian] In formal Brazilian Portuguese, 'dez' should be written as '10' without the article 'dia' in this context.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `preocupo` -> `preocupo` [verb_form] The verb must be in the first person singular form: preocupo -> preocupo (correct), but the correct form is 'preocupo' (first person singular).
- Missed: `eu preocupo`

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `diverti` -> `diverti` [verb_form] The verb should be in the past participle form to agree with 'eu' in the sentence structure.
- Missed: `eu diverti`

### a1-word-choice-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu sou vinte anos velho.
- Reference: `sou vinte anos velho` -> tenho vinte anos [word_choice]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `vinte anos velho` -> `tenho vinte anos` [word_choice] The phrase is incorrect; Portuguese uses 'tenho' + age instead of 'sou' + age.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a2-not-brazilian-phone (erroneous, a2)

- Task: Where is your phone?
- Answer: O meu telemóvel está em cima do sofá.
- Reference: `telemóvel` -> celular [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `telemóvel`

### a1-word-order-article-noun (erroneous, a1)

- Task: Where do you live?
- Answer: Eu moro com meus pais em casa uma pequena.
- Reference: `casa uma pequena` -> uma casa pequena [word_order]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `uma pequena` -> `em uma pequena casa` [word_order] This phrase is incomplete and grammatically incorrect. It should be 'em uma pequena casa' or similar.

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô` -> `Estou` [not_brazilian] 'Tô' is a colloquial form of 'estou' and is not considered standard Brazilian Portuguese for this level.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de` -> `` [article_preposition] The preposition 'de' is not used in this context. The correct structure is 'está de vestido' without 'de' before the noun.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (quote not in the answer): `Hojе` -> `Hoje` [spelling] The word 'Hoje' is incorrectly spelled without the accent.
- Found (false positive): `estou` -> `so` [verb_form] The verb 'estar' is incorrect for the question 'What is your name?' which requires 'ser' for identity.
- Found (false positive): `em casa` -> `na casa` [article_preposition] The preposition 'em' is incorrect; the correct preposition is 'na' for 'em casa' when referring to a place.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `comprei` -> `tenho uma opinião sobre trabalhar em casa` [content_mismatch] The verb does not match the question about working from home.

## qwen3:8b / few_shot_in_system

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de` -> `com` [article_preposition] The preposition 'de' is not used with 'maiô' in this context. Use 'com' instead.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `cansado` -> `cansado` [agreement] The adjective must agree in gender with 'você' (masculine).

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [spelling] The word needs an accent on the o: escritório.
- Missed: `uma escritório`

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `assisto televisão` -> `assistir à televisão` [article_preposition] The preposition 'a' is missing before 'televisão'
- Missed: `jantar`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective must agree in gender and number with 'camisa' (feminine singular).
- Found (changes nothing): `preta` -> `preta` [agreement] The adjective must agree in gender and number with 'calça' (feminine singular).
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `muito` -> `muito` [word_choice] 'Muito' is used for quantity, not to express degree of liking. Use 'muito' with a noun or 'muito' as an adverb. Use 'muito' as an adverb to express degree of liking, but it should be placed differently.
- Missed: `gosto de café muito`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `gostei` [verb_form] The verb should be in the past tense to match the question about what was eaten.
- Found (false positive): `é` -> `foi` [verb_form] The verb should be in the past tense to match the question about what was eaten.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `estudei` -> `estudei` [verb_form] The verb should be in the past historic tense (pretérito perfeito) for a question about yesterday, not the pretérito imperfeito.
- Missed: `portugues`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tomei cafe` -> `tomei café` [spelling] The word 'cafe' needs an accent on the 'e': café.
- Found (matched): `de onibus` -> `de ônibus` [spelling] The word 'onibus' needs an accent on the 'i': ônibus.
- Found (changes nothing): `cheguei` -> `cheguei` [verb_form] The verb 'chegar' should be in the past perfect tense (pretérito perfeito) for 'eu', which is 'cheguei'. However, the sentence structure is correct, so this is not a mistake.
- Missed: `as nove`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `era` -> `tinha sido` [verb_form] The verb should be in the past perfect tense (pretérito mais-que-perfeito) to match the context of a completed action in the past.
- Found (false positive): `dormi o dia todo` -> `dormi o dia inteiro` [word_choice] The phrase is incorrect; in Brazilian Portuguese, it is more natural to say 'dormi o dia inteiro' with 'inteiro' instead of 'todo'.

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
- Found (matched): `nós` -> `nos` [article_preposition] In Brazilian Portuguese, 'nós' is not used in this context; the pronoun 'nós' is not needed when referring to a general situation.
- Found (false positive): `precisamos` -> `precisar` [verb_form] 'Precisamos' is incorrect in this context; the correct form is 'precisar' when used with 'quando' in a conditional sentence.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `fazendo chuva` [content_mismatch] This contradicts the scene fact that it is raining.

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `do que` -> `do que` [not_brazilian] The correct preposition is 'do que' for comparison, but it should be 'do que' when comparing qualities, which is correct here. No mistake.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `à direita` -> `direita` [article_preposition] The preposition 'a' is not needed before 'direita' in this context.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `comeu` -> `comeu` [verb_form] The verb should be in the past perfect (pretérito perfeito) to match the context of a completed action in the past.

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `cozinhar` -> `cozinhar` [word_choice] The verb form should be in the infinitive, not the gerund form.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu` -> `` [content_mismatch] The answer does not describe the meal, it talks about the speaker's residence.

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Espero` -> `Vou` [word_choice] The verb 'esperar' is not appropriate for inviting someone to a party.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós podermos` -> `para você podermos` [pronoun] The pronoun 'nós' should be replaced with 'você' to match the subject 'você' in the main clause.

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a gente` -> `nós` [not_brazilian] 'A gente' is considered informal and is not typically used in written Portuguese at B1 level.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `folga` -> `dia de folga` [word_choice] 'Folga' is not commonly used in Brazilian Portuguese for 'day off'; 'dia de folga' is more natural.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] The verb form should be in the infinitive after 'recomendo que' to match the subjunctive mood.
- Found (matched): `exige` -> `exija` [verb_form] The verb 'exige' is in the indicative mood, but it should be in the subjunctive mood after 'embora' to match the context.
- Missed: `assiste`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `reclamar` -> `reclamar` [verb_form] The verb 'reclamar' is incorrect in this context; the correct verb is 'reclamar' (same spelling) but it should be in the infinitive form 'reclamar' after 'por meio desta'
- Missed: `sobre do`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajarei` [verb_form] The verb should be in the future tense (viajarei) instead of the conditional (viajaria) because Ana is talking about a future action.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cheguei` -> `tinha chegado` [verb_form] The verb should be in the past perfect (pretérito mais-que-perfeito) to match the context of a day that has already happened and is being described in a narrative.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dez` -> `décimo` [not_brazilian] In formal writing, the ordinal number 'dia dez' should be written as 'dia décimo' in Brazilian Portuguese.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `esperasse` -> `esperar` [word_choice] The verb should be in the infinitive form after 'disse que' in this context.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `preocupo` -> `preocupo` [verb_form] The verb must be in the first person singular form: preocupo -> preocupo.
- Missed: `eu preocupo`

### a1-short-agreement-books (erroneous, a1)

- Task: What is in your bag?
- Answer: Na minha bolsa tem um livros.
- Reference: `um livros` -> um livro | uns livros [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tem` -> `temos` [verb_form] The verb 'tem' should be 'temos' to agree with the plural subject 'livros'.
- Missed: `um livros`

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `prova` -> `prova` [not_brazilian] The word 'prova' is singular, but the context implies a test, which is usually referred to as 'prova' in Brazilian Portuguese, and the sentence is correct as is.
- Missed: `fiz um erro`

### a2-not-brazilian-phone (erroneous, a2)

- Task: Where is your phone?
- Answer: O meu telemóvel está em cima do sofá.
- Reference: `telemóvel` -> celular [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `telemóvel`

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô` -> `Estou` [not_brazilian] 'Tô' is a colloquial form of 'estou' and is not considered standard Brazilian Portuguese for this level.

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tem` -> `há` [word_choice] In Brazilian Portuguese, 'há' is used for the existence of things, not 'tem' which is used for possession.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está` -> `é` [ser_estar] The verb 'estar' is not needed here because the sentence is describing a permanent or habitual state.

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `doente` -> `doente` [agreement] The adjective should agree in number with 'irmão' (singular).

### a2-control-family-meal (control, a2)

- Task: Describe the meal these people are eating.
- Answer: A família está comendo arroz, feijão e frango grelhado. Eles bebem suco de laranja.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `frango grelhado` -> `frango grelhado` [word_order] The correct word order is 'frango grelhado' (correct), but the phrase should be 'frango grelhado' (no mistake).

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `passe` -> `passar` [word_choice] The verb 'passar' should be in the infinitive form after 'para que' in this context.

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `pequeno` -> `pequeno` [agreement] The adjective must agree in number with 'eu' (singular).

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `falei` -> `tinha falado` [verb_form] The verb should be in the past perfect (pretérito mais-que-perfeito) to match the context of inviting someone.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (quote not in the answer): `Hojé` -> `Hojé` [spelling] The word needs an accent on the e: Hoje.
- Found (false positive): `estou` -> `me chamo` [content_mismatch] The verb form does not match the question. The question is asking for a name, not an activity.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `comprei` -> `tenho opinião sobre trabalhar em casa` [content_mismatch] The verb form does not match the question about working from home.

## qwen3:8b / few_shot_mostly_correct

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maiô` -> `traje de banho` [word_choice] 'Maiô' is not commonly used in Brazilian Portuguese to refer to a swimsuit; the more common term is 'biquini' or 'traje de banho' at this level.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Só um pouco cansado.` -> `Só um pouco cansado.` [spelling] 'Cansado' should be 'cansado' with an accent on the 'a' for masculine singular agreement.

### a1-control-family (control, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos e uma irmã. Meu pai é médico e minha mãe é professora.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `irmãos` -> `irmãs` [agreement] 'Irmãos' is masculine, but the subject is 'irmã' (female), so the adjective should be feminine.

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha` -> `De manhã` [spelling] 'De manha' is incorrect; the correct phrase is 'De manhã' with an accent on the 'a'
- Missed: `cafe`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [spelling] The word needs an accent on the o: escritório.
- Missed: `uma escritório`

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `assisto televisão` -> `assistir a televisão` [article_preposition] The preposition 'a' is missing before 'televisão'
- Missed: `jantar`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo.` -> `Eu comi pão com manteiga e ovos para o café da manhã.` [content_mismatch] The answer does not answer the question about what was eaten for breakfast.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `estudei` -> `estudei` [verb_form] The verb 'estudar' should be in the past tense 'estudei' for 'I studied', but the correct form for 'I studied' in Portuguese is 'estudei' (correct). No mistake here.
- Missed: `portugues`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tomei cafe` -> `tomei café` [spelling] 'Cafe' needs an accent: 'café'
- Found (matched): `de onibus` -> `de ônibus` [spelling] 'Onibus' needs an accent: 'ônibus'
- Missed: `as nove`

### a2-verb-form-subject-agreement (erroneous, a2)

- Task: What did your family do on Sunday?
- Answer: No domingo minha família almoçaram na casa da minha avó.
- Reference: `almoçaram` -> almoçou [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `almoçaram` -> `almoçaram` [verb_form] The verb 'almoçar' should be in the preterite tense for 'they' (eles/elas), which is 'almoçaram' is correct. However, the verb form is incorrect because 'minha família' is plural, so the verb should be 'almoçaram' (correct). No mistake here.
- Missed: `almoçaram`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `era` -> `estava` [ser_estar] The verb 'ser' should be 'estar' to describe a temporary state of being tired.
- Found (false positive): `o dia todo` -> `todo o dia` [word_order] In Brazilian Portuguese, 'todo o dia' is the correct phrase, not 'o dia todo'.

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `pequeno-almoço` -> `pequeno-almoço` [not_brazilian] This is the European Portuguese spelling; the Brazilian Portuguese spelling is pequeno-almoço without the hyphen.
- Missed: `pequeno-almoço`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `fazendo chuva` [content_mismatch] This contradicts the scene fact that it is raining.
- Found (duplicate): `muito calor` -> `muito frio` [content_mismatch] This contradicts the scene fact that it is cold.

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `do que` -> `do que` [article_preposition] The correct preposition is 'do que' for comparison, but in Brazilian Portuguese, it is commonly replaced with 'do que' in spoken language. However, in written form, it should be 'do que' (with the 'o' before 'que').

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `à direita` -> `direita` [article_preposition] The preposition 'a' is not needed before 'direita' in this context.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vi` -> `vi` [verb_form] The past tense of 'ver' for 'I saw' should be 'vi' (correct), so this is actually correct.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Os dois comem macarrão e bebem vinho tinto.` [content_mismatch] This answer does not describe the meal the people are eating, but instead talks about where the speaker lives and their feelings about the city.

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Espero que você possa vir à festa e que traga seus amigos.` -> `Vou te convidar para a festa.` [word_choice] The sentence is grammatically correct, but it is more natural to use 'Vou te convidar para a festa' or similar phrasing for an invitation.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós podermos` -> `para que possamos` [word_choice] The pronoun 'nós' is incorrect here; the correct pronoun is 'nós' but it should be 'para que possamos' or 'para nós podermos' is not standard in Brazilian Portuguese.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `folga` -> `dia de folga` [word_choice] 'Folga' is not commonly used in Brazilian Portuguese for 'day off'; 'dia de folga' is more natural.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estudar` [verb_form] The verb form should be in the infinitive, not the third person singular.
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `reclamar` -> `reclamar de` [word_choice] The verb 'reclamar' is incorrect in this context; the correct form is 'reclamar de' when complaining about something.
- Found (matched): `do` -> `de` [article_preposition] 'Do' is not used in this structure; it should be 'do' as part of 'de' in 'reclamar de'

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajarei` [verb_form] The verb should be in the future tense (viajarei) instead of the conditional (viajaria) for direct speech reported in Brazilian Portuguese.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `apagar vários incêndios` -> `extinguir vários incêndios` [word_choice] 'Apagar incêndios' is not a common expression in Brazilian Portuguese for 'put out fires'; the correct expression is 'extinguir incêndios' or 'apagar chamas' in some contexts.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `deveria` -> `deveria` [not_brazilian] In formal writing, 'deveria' is considered too informal; the correct formal form is 'deveria' (same spelling, but contextually it's more formal in written complaints).

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô` -> `Estou` [not_brazilian] 'Tô' is a spoken form and is not correct in written Portuguese.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de vestido` -> `vestido amarelo` [article_preposition] The preposition 'de' is not needed here. The correct phrase is 'vestido amarelo' without 'de'.

### a2-control-family-meal (control, a2)

- Task: Describe the meal these people are eating.
- Answer: A família está comendo arroz, feijão e frango grelhado. Eles bebem suco de laranja.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `comendo` -> `estão comendo` [article_preposition] The verb 'estar' is missing, so the correct form is 'estão comendo' for third person plural.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para que eu passe` -> `para passar` [word_choice] The phrase should be 'para passar' instead of 'para que eu passe' because 'para que' is used with subjunctive mood, which is not needed here.

### b1-control-weather-hot (control, b1)

- Task: Describe the weather today.
- Answer: Hoje faz muito calor e o céu está sem nuvens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `sem nuvens` -> `sem nuvem` [agreement] 'Sem nuvens' is incorrect; the correct expression is 'sem nuvem' (singular).

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cheguei` -> `eu cheguei` [pronoun] The verb 'chegar' should be in the first person singular form 'cheguei' is correct, but the sentence is missing a subject.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `falei` -> `falo` [verb_form] The verb 'falei' is in the past tense, but the question is about a general action, so the present tense 'falo' is more appropriate.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (quote not in the answer): `Hojé está chovendo muito e eu estou em casa.` -> `Meu nome é [your name].` [content_mismatch] The answer does not answer the question 'What is your name?'

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei uma bicicleta nova no sábado.` -> `Eu acho que trabalhar em casa é bom porque você pode economizar tempo e dinheiro.` [content_mismatch] The answer does not address the question about working from home.

### a2-english-coffee (not_portuguese, a2)

- Task: Do you like coffee?
- Answer: Yes, I like coffee a lot.
- Flags: written_in_portuguese=true answers_the_question=true

## qwen3:8b / few_shot_reversed

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `maiô` -> `maiô` [spelling] The correct spelling is 'maiô' with an accent on the o.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cansado` -> `cansada` [agreement] 'Cansado' is masculine, but the subject is 'você', which is neutral in Portuguese and typically takes the feminine form in colloquial speech.

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha` -> `De manhã` [spelling] 'De manha' is incorrect; the correct phrase is 'De manhã' with an accent on the 'a'
- Missed: `cafe`

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `assisto televisão` -> `assistir à televisão` [article_preposition] The preposition 'a' is missing before 'televisão'
- Missed: `jantar`

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `chama` -> `chama` [not_brazilian] The correct form is 'chama' for 'chamar' (to call), but in Brazilian Portuguese, it is more common to use 'se chama' for formal or neutral contexts. However, at A1 level, 'chama' is acceptable. No mistake here.
- Missed: `chama`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective must agree in gender with 'camisa' (feminine).
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `muito` -> `muito` [word_choice] 'Muito' is not used to describe the degree of liking in this context; 'muito' should be replaced with 'muito' (correct) but the structure is incorrect.
- Missed: `gosto de café muito`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `gostei` [verb_form] The verb should be in the past tense to match the question about what was eaten.

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tomei cafe` -> `tomei café` [spelling] 'Cafe' needs an accent: 'café'
- Found (matched): `de onibus` -> `de ônibus` [spelling] 'Onibus' needs an accent: 'ônibus'
- Missed: `as nove`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `era` -> `tinha sido` [verb_form] The verb should be in the past perfect tense (pretérito mais-que-perfeito) to match the context of a completed action in the past.
- Found (false positive): `o dia todo` -> `todo o dia` [word_order] In Brazilian Portuguese, 'todo o dia' is the correct phrase, not 'o dia todo'.

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
- Found (matched): `nós` -> `nos` [pronoun] 'Nós' is incorrect in this context; the correct pronoun is 'nós' for 'we', but it is not needed here.
- Found (false positive): `precisamos` -> `precisam` [verb_form] 'Precisamos' is incorrect in this context; the correct form is 'precisam' to agree with 'vizinhos' (plural).

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `pequeno-almoço` -> `pequeno-almoço` [not_brazilian] This is the European Portuguese spelling; the Brazilian Portuguese spelling is 'pequeno-almoço' without the hyphen.
- Missed: `pequeno-almoço`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `fazendo chuva` [content_mismatch] This describes sunny weather, which contradicts the scene facts.
- Found (duplicate): `muito calor` -> `muito frio` [content_mismatch] This describes hot weather, which contradicts the scene facts.

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `do que` -> `do que` [not_brazilian] The correct preposition is 'do que' for comparison, but it should be 'do que' when comparing two things. However, the sentence is correct as it is.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `à direita` -> `direita` [article_preposition] The preposition 'a' is not needed before 'direita' in this context.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Dois pessoas estão comendo espagueti e bebendo vinho tinto em um restaurante.` [content_mismatch] The answer does not describe the meal being eaten.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Espero` -> `Vou` [word_choice] The verb 'esperar' is not appropriate for inviting someone to a party.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós podermos` -> `para você podermos` [pronoun] The pronoun 'nós' is incorrect; the correct pronoun is 'você' in this context.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Prezado Sr. Souza` -> `Senhor Souza` [word_choice] 'Prezado' is used for formal letters, but 'Sr.' is not appropriate in Brazilian Portuguese for addressing someone in a letter. Use 'Senhor' instead.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estudar` [verb_form] The verb should be in the infinitive form after 'recomendo que'
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `reclamar` -> `reclamar` [word_choice] The verb 'reclamar' should be in the infinitive form 'reclamar' after 'por meio desta' in formal writing.
- Found (matched): `do` -> `` [article_preposition] 'Do' is not used in this context; it should be omitted.
- Found (false positive): `no` -> `no dia dez` [not_brazilian] In formal writing, 'no' should be replaced with 'no' (correct) but the phrase 'no dia dez' is acceptable. However, in more formal contexts, 'no dia dez' is correct.

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `vou viajar` [verb_form] The verb should be in the future tense (vou viajar) instead of the conditional (viajaria) for direct speech reported in the past.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `apagar vários incêndios` -> `extinguir vários incêndios` [word_choice] 'Apagar incêndios' is not a common expression in Portuguese for 'put out fires'; the correct expression is 'extinguir incêndios' or 'apagar incêndios' is not idiomatic in this context.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `no dia dezesseis` [not_brazilian] In formal writing, the ordinal number should be written out in words, not as a numeral.

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `diverti` -> `diverti` [verb_form] The verb should be in the past participle form to agree with 'eu' in the sentence structure.
- Missed: `eu diverti`

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a2-not-brazilian-phone (erroneous, a2)

- Task: Where is your phone?
- Answer: O meu telemóvel está em cima do sofá.
- Reference: `telemóvel` -> celular [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `telemóvel`

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô` -> `Estou` [not_brazilian] 'Tô' is a colloquial form of 'estou' and is not considered standard Brazilian Portuguese for this level.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de` -> `` [article_preposition] The preposition 'de' is not needed here. The correct structure is 'está de vestido' without 'de' when describing clothing.

### a2-control-family-meal (control, a2)

- Task: Describe the meal these people are eating.
- Answer: A família está comendo arroz, feijão e frango grelhado. Eles bebem suco de laranja.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `comendo` -> `estão comendo` [article_preposition] The verb 'estar' is missing, so the correct form is 'estão comendo' for third person plural.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `falei` -> `falo` [verb_form] The verb 'falei' is in the past tense, but the question is about a general action, so the present tense 'falo' is more appropriate.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (quote not in the answer): `Hojе` -> `Hoje` [spelling] The word 'Hoje' is incorrectly spelled with an accent on the e.
- Found (false positive): `estou` -> `me chamo` [content_mismatch] The verb form 'estou' is correct, but the question asks for your name, not the weather or where you are.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei uma bicicleta nova no sábado.` -> `Não` [content_mismatch] The answer does not address the question about working from home.

### a2-english-coffee (not_portuguese, a2)

- Task: Do you like coffee?
- Answer: Yes, I like coffee a lot.
- Flags: written_in_portuguese=true answers_the_question=true

## qwen3:8b / few_shot_six

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cansado` -> `cansada` [agreement] The adjective must agree with 'você' (a singular noun) in gender and number. Since 'você' is neutral in Portuguese, 'cansado' is correct, but in Brazilian Portuguese, it's more natural to use 'cansado' for 'você' regardless of gender. However, if the speaker is male, 'cansado' is correct; if female, 'cansada' is needed. Since the gender is not specified, this is not a mistake in Brazilian Portuguese.

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha` -> `De manhã` [spelling] The correct phrase is 'De manhã' with an accent on the 'a'
- Missed: `cafe`

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `sou` -> `estou` [verb_form] 'Sou' is used for 'to be' in the sense of 'to be from' or 'to be a', not for location. Use 'estou' for 'I am'.

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `assisto televisão` -> `assistô televisão` [article_preposition] The preposition 'a' is missing before 'televisão'.
- Found (matched): `jantar` -> `janto` [verb_form] The verb must agree with 'eu': janto.

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `gostei` [verb_form] The verb must be in past tense to match the question about the past.

### a1-english-lunch (not_portuguese, a1)

- Task: What did you eat for lunch?
- Answer: I ate rice and chicken for lunch.
- Flags: written_in_portuguese=false answers_the_question=false

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `era` -> `tinha` [verb_form] The verb must be in the past perfect tense 'tinha' to match the context of 'ontem'.
- Found (false positive): `o dia todo` -> `todo o dia` [word_order] In Brazilian Portuguese, 'todo o dia' is the correct phrase, not 'o dia todo'.

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
- Found (changes nothing): `nós` -> `nós` [pronoun] 'Nós' is incorrect in this context; the correct pronoun is 'nós' for 'we', but it should be 'nós' or 'a gente' in informal speech. However, the phrase 'nós ajudamos' is grammatically correct in Brazilian Portuguese. No mistake here.
- Missed: `nós ajudamos`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `fazendo chuva` [content_mismatch] This describes sunny weather, but the scene says it is raining.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Dois pessoas estão comendo espagueti e bebendo vinho tinto em um restaurante.` [content_mismatch] The answer does not describe the meal the people are eating.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós podermos` -> `para podermos` [word_choice] 'Para' is followed by an infinitive without 'nos'.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estudar` [verb_form] The verb must be in the infinitive form after 'recomendo que'
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `reclamar` -> `reclamar de` [article_preposition] The verb 'reclamar' requires the preposition 'de' after it, so it should be 'reclamar de'
- Found (matched): `sobre do` -> `sobre o` [article_preposition] 'sobre' and 'do' are not used together; the correct phrase is 'sobre o'

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajarei` [verb_form] The verb should be in the future tense, not the conditional.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `apagar vários incêndios` -> `extinguir vários incêndios` [word_choice] 'Apagar incêndios' is not a common expression in Portuguese; the correct expression is 'extinguir incêndios' or 'apagar incêndios' is not idiomatic in this context.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `do atraso` -> `o atraso` [article_preposition] The preposition 'de' is not needed before 'atraso' in this context.
- Found (false positive): `no dia dez` -> `no dia 10` [not_brazilian] In formal writing, 'dez' should be written as '10' or '10º' for clarity and formality.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `preocupo` -> `preocupo` [verb_form] The verb must agree with 'eu': preocupo -> preocupo (correct form for 'eu').
- Missed: `eu preocupo`

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu diverti`

### a1-word-choice-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu sou vinte anos velho.
- Reference: `sou vinte anos velho` -> tenho vinte anos [word_choice]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `vinte anos velho` -> `tenho vinte anos` [word_choice] The phrase is incorrect; Portuguese uses 'tenho' with age.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sempre fui`

### a2-not-brazilian-phone (erroneous, a2)

- Task: Where is your phone?
- Answer: O meu telemóvel está em cima do sofá.
- Reference: `telemóvel` -> celular [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `telemóvel`

### a1-word-order-article-noun (erroneous, a1)

- Task: Where do you live?
- Answer: Eu moro com meus pais em casa uma pequena.
- Reference: `casa uma pequena` -> uma casa pequena [word_order]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `uma pequena` -> `em uma pequena casa` [content_mismatch] This phrase is incomplete and does not make sense in this context.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (quote not in the answer): `Hojе` -> `Hoje` [spelling] The word 'Hoje' should not have an accent mark in Brazilian Portuguese.
- Found (false positive): `estou` -> `me chamo` [content_mismatch] The verb must agree with 'eu': estou is correct, but the question asks for your name, not the weather.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (quote not in the answer): `comprou` -> `tenho uma opinião sobre trabalhar em casa` [content_mismatch] The verb does not match the question about working from home.

## qwen3:8b / one_shot_correct

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de maiô` -> `em maiô` [article_preposition] 'Maiô' is a noun and should be preceded by 'em' for 'in a swimsuit'

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Só um pouco cansado` -> `um pouco cansado` [not_brazilian] 'Cansado' is incorrect for 'a little tired' in this context; the correct form is 'cansado' for masculine singular, but the phrase should use 'um pouco cansado' for a person, which is correct. However, the phrase is not natural in Brazilian Portuguese.

### a1-control-family (control, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos e uma irmã. Meu pai é médico e minha mãe é professora.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `dois irmãos e uma irmã` -> `dois irmãos e uma irmã` [agreement] 'dois irmãos' and 'uma irmã' are not compatible in number and gender

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `me chamo` -> `chamo` [word_choice] 'Me chamo' is a colloquial form that is not standard in Brazilian Portuguese at A1 level.
- Found (false positive): `moro` -> `morando` [word_order] 'Moro' is correct, but the sentence structure could be more formal and standard for A1 level.

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha` -> `De manhã` [spelling] The correct phrase is 'De manhã' with the accent on the 'a'.
- Missed: `cafe`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [article_preposition] needs the definite article 'o' before it
- Missed: `uma escritório`

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `assisto televisão` -> `assistir à televisão` [article_preposition] The preposition 'a' is missing before 'televisão'.
- Missed: `jantar`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma vermelha` -> `uma camisa vermelha` [word_order] 'vermelha' should be 'vermelha' (correct spelling), but the article 'uma' is not needed here because 'camisa' is a feminine noun and the color adjective should come before the noun without an article.
- Found (changes nothing): `uma calça preta` -> `uma calça preta` [word_order] 'calça' is a feminine noun, so 'preta' is correct, but the article 'uma' is not needed here because the color adjective should come before the noun without an article.

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `muito` -> `muito` [word_choice] 'Muito' is not used in this way with 'gosto de' in Brazilian Portuguese.
- Found (changes nothing): `muito` -> `muito` [word_choice] 'Muito' is not used in this way with 'gosto de' in Brazilian Portuguese.
- Missed: `gosto de café muito`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo.` -> `Eu comi pão com manteiga e ovos.` [content_mismatch] The answer does not address the question about what was eaten for breakfast.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `estudei` -> `estudei` [verb_form] The verb 'estudar' should be in the past historic tense for a narrative context.
- Found (matched): `portugues` -> `português` [spelling] The word 'portugues' is missing an accent mark.

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tomei cafe` -> `tomei café` [spelling] 'cafe' is incorrect; the correct spelling is 'café' with an accent mark.
- Found (matched): `de onibus` -> `de ônibus` [article_preposition] 'de onibus' is incorrect; the correct preposition is 'de ônibus' with an accent mark.
- Missed: `as nove`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `era` -> `estava` [verb_form] The past tense 'era' is used for permanent characteristics, not for temporary states like being tired.
- Found (false positive): `o dia todo` -> `todo o dia` [word_order] In Brazilian Portuguese, 'todo o dia' is the correct phrase, not 'o dia todo'.

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `pequeno-almoço` -> `pequeno-almoço` [not_brazilian] This is the European Portuguese spelling; in Brazilian Portuguese, it is written as 'pequeno-almoço' with a hyphen and no accent.
- Missed: `pequeno-almoço`

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vou` -> `irei` [verb_form] The future tense 'vou' is not commonly used in Brazilian Portuguese for planned future actions at A2 level; 'irei' is more formal and less common in spoken language.

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `do que` -> `do que` [not_brazilian] The correct preposition after 'mais' is 'do que' when comparing two things, but the structure 'mais novo do que' is correct.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `à direita` -> `direita` [article_preposition] The preposition 'a' is not needed with 'direita' in this context.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A gente` -> `Nós` [not_brazilian] 'A gente' is considered informal and is not typically used in written Portuguese, especially at A2 level.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `gostei muito dele` -> `gostei muito do filme` [pronoun] 'dele' is incorrect because 'filme' is masculine, but the pronoun should refer to the film, not the person.

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No meu tempo livre` -> `No meu tempo livre` [article_preposition] 'No' is not used with 'meu' in Brazilian Portuguese.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Os dois comem spaghetti e bebem vinho tinto.` [content_mismatch] The answer does not describe the meal being eaten in the scene.

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Espero que você possa vir à festa e que traga seus amigos.` -> `Convidei meu amigo para a festa.` [content_mismatch] The sentence is a wish, but the task is to invite a friend, not to express a hope.

### b1-control-future-subjunctive (control, b1)

- Task: What will you do when you finish the course?
- Answer: Quando eu terminar o curso, vou procurar um emprego novo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vou` -> `vou` [verb_form] The future tense should be formed with 'vou' for 'I will', which is correct here.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Se eu ganhasse na loteria` -> `Se eu ganhar na loteria` [word_order] The conditional structure 'se' is not correctly used with the future tense in this context.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós podermos` -> `para você podermos` [pronoun] The pronoun 'nós' is incorrect in this context; the correct pronoun is 'você' or 'vocês'

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a gente` -> `nós` [word_choice] 'A gente' is considered informal and is not typically used in B1 level writing.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Prezado Sr. Souza` -> `Prezado Sr. Souza` [not_brazilian] The form 'Prezado' is used in European Portuguese, not Brazilian Portuguese.
- Found (false positive): `folga` -> `dia de descanso` [word_choice] 'Folga' is not commonly used in Brazilian Portuguese to refer to a day off; 'dia de folga' is correct but 'dia de descanso' or 'feriado' may be more natural.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estudar` [verb_form] The verb form should be in the infinitive after 'recomendo que' when giving advice.
- Found (changes nothing): `exige` -> `exige` [verb_form] The verb should be in the third person singular form to agree with 'isso'.
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `reclamar sobre do` -> `reclamar sobre` [word_choice] The phrase 'reclamar sobre do' is incorrect; the correct preposition is 'sobre' without 'do'.
- Found (false positive): `que deveria ter chegado no dia dez` -> `que deveria ter chegado no dia décimo` [word_choice] In formal writing, it is more appropriate to use the ordinal form 'décimo' instead of the cardinal form 'dez'.

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajou` [verb_form] The future tense is not used in reported speech in Brazilian Portuguese; the simple past is used instead.
- Found (false positive): `tinha comprado` -> `comprado` [verb_form] The past perfect tense is not used in reported speech in Brazilian Portuguese; the simple past is used instead.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cheguei em casa morto de cansaço` -> `morta de cansaço` [word_choice] The expression 'morta de cansaço' is more natural and commonly used in Brazilian Portuguese.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `no décimo dia` [word_choice] In Brazilian Portuguese, it is more natural to use the ordinal form 'décimo' instead of 'dez' in this context.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `não me esperasse` -> `não me esperar` [word_choice] The subjunctive form 'esperasse' is not used in this context; the indicative form 'esperar' is more natural in spoken Brazilian Portuguese.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `preocupo` -> `preocupo` [verb_form] The verb 'preocupar' should be in the first person singular form 'preocupo' is incorrect; the correct form is 'preocupo' (correct) or 'preocupo' (correct). Wait, the verb 'preocupar' is irregular and the correct first person singular form is 'preocupo'. However, the sentence structure is incorrect.
- Found (changes nothing): `muito` -> `muito` [word_choice] The word 'muito' is used incorrectly here; the correct word is 'muito' (correct) but the structure is wrong.
- Missed: `eu preocupo`

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `prova de matemática` -> `prova de matemática` [word_choice] 'Prova' is not commonly used in Brazilian Portuguese for a test, especially at the A2 level.
- Found (matched): `fiz um erro` -> `errrei` [word_choice] 'Fiz um erro' is not natural in Brazilian Portuguese for this context; 'errrei' is more common.

### a1-control-where-live (control, a1)

- Task: Where do you live?
- Answer: Moro no Brasil, numa cidade perto do mar.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `numa` -> `numa` [not_brazilian] 'numa' is incorrect for masculine nouns; the correct form is 'numa' for feminine, but 'cidade' is feminine, so it is correct. However, in Brazilian Portuguese, 'numa' is considered informal and is often replaced by 'numa' in spoken language. However, in written A1 level, it's better to use 'numa' for feminine nouns. No mistake here.

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô` -> `Estou` [not_brazilian] 'Tô' is a colloquial form of 'estou' and is not considered standard Brazilian Portuguese for this level.

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tem` -> `tem` [not_brazilian] 'Tem' is used in European Portuguese, but in Brazilian Portuguese, 'tem' is considered incorrect in this context; the correct form is 'tem' in Brazilian Portuguese for this usage.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está` -> `veste` [not_brazilian] 'está' is used for temporary states, but here the description is permanent.

### a2-control-preterite-sunday (control, a2)

- Task: What did you do on Sunday?
- Answer: No domingo eu acordei tarde, fiz o almoço e depois assisti a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `fiz` -> `fiz` [verb_form] The verb 'fazer' should be in the past perfect (pretérito perfeito) to match the context of a completed action in the past.

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `não vai trabalhar hoje` -> `não vai trabalhar hoje` [word_choice] The verb 'trabalhar' should be in the infinitive form after 'não vai' in this context

### a2-control-family-meal (control, a2)

- Task: Describe the meal these people are eating.
- Answer: A família está comendo arroz, feijão e frango grelhado. Eles bebem suco de laranja.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `frango grelhado` -> `frango à grelha` [word_choice] 'Grelhado' is more commonly used in Brazilian Portuguese as an adjective, but it is not the most natural way to say 'grilled chicken' in everyday speech.
- Found (changes nothing): `suco de laranja` -> `suco de laranja` [word_choice] 'Suco de laranja' is correct, but in Brazilian Portuguese, it is more common to say 'suco de laranja' or 'suco de laranja fresco' for 'orange juice'.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para que eu passe no exame` -> `para passar no exame` [word_choice] The phrase is grammatically correct, but it is more natural to use 'para passar no exame' without the pronoun 'eu' in this context.

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ia` -> `fui` [verb_form] The verb 'ia' is the imperfect tense of 'ir', but the sentence refers to a repeated action in the past, which should use the past historic tense.

### b1-control-weather-hot (control, b1)

- Task: Describe the weather today.
- Answer: Hoje faz muito calor e o céu está sem nuvens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `sem nuvens` -> `limpo` [word_choice] 'Sem nuvens' is not commonly used in Brazilian Portuguese to describe a sunny day; 'limpo' or 'claro' are more natural choices.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `cheguei` -> `cheguei` [verb_form] The verb 'chegar' should be in the past perfect (pretérito perfeito) to match the context of the day already finished.
- Found (false positive): `em casa` -> `na casa` [article_preposition] The preposition 'em' is incorrect here; the correct preposition is 'na' for 'in the house' in Brazilian Portuguese.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `falei` -> `falo` [verb_form] The verb 'falei' is in the past tense, but the question is about a general habit, not a specific past action.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está chovendo muito e eu estou em casa.` -> `Meu nome é [nome].` [content_mismatch] The answer does not answer the question about the user's name.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei uma bicicleta nova no sábado.` -> `Eu acho que trabalhar em casa é bom porque posso economizar tempo.` [content_mismatch] The answer does not address the question about working from home.

## qwen3:8b / one_shot_mistake

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maiô` -> `biquini` [word_choice] The word 'maiô' is not commonly used in Brazilian Portuguese; the correct term is 'biquini' or 'traje de banho' for a swimsuit.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Só um pouco cansado.` -> `um pouco cansado` [content_mismatch] 'Cansado' is incorrect for 'a little tired' at A1 level; the correct form is 'cansado' for masculine singular, but the phrase is not appropriate for an A1 learner.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [spelling] The word needs an accent on the o: escritório.
- Missed: `uma escritório`

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `sou` -> `estou` [ser_estar] 'Sou' is the conjugation of 'ser' for 'eu', but the correct verb here is 'estar' for location.

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `assisto televisão` -> `assistir à televisão` [article_preposition] The preposition 'a' is missing before 'televisão'
- Missed: `jantar`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `chamo` -> `me chamo` [word_choice] The correct form is 'chamo' for 'I am called', but the question asks for your name, not how you are called.

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `vermelha` -> `camisa vermelha` [agreement] The adjective 'vermelha' needs to agree in gender and number with the noun 'camisa' (feminine singular), so it should be 'vermelha' (correct, but the article is missing).
- Found (false positive): `preta` -> `calça preta` [agreement] The adjective 'preta' needs to agree in gender and number with the noun 'calça' (feminine singular), so it should be 'preta' (correct, but the article is missing).

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo.` -> `Eu comi pão com manteiga e ovos.` [content_mismatch] The answer does not address the question about what was eaten for breakfast.

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tomei cafe` -> `tomei café` [spelling] 'Cafe' needs an accent: 'café'
- Found (matched): `de onibus` -> `de ônibus` [spelling] 'Onibus' needs an accent: 'ônibus'
- Missed: `as nove`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `era` -> `estava` [ser_estar] The verb 'ser' is used instead of 'estar' for temporary states like being tired.
- Found (false positive): `o dia todo` -> `todo o dia` [word_order] In Brazilian Portuguese, 'todo o dia' is the correct phrase, not 'o dia todo'.

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `vi` [word_choice] The verb 'atender' is not used for 'to watch a movie' in Brazilian Portuguese.
- Found (changes nothing): `no cinema` -> `no cinema` [article_preposition] The preposition 'no' is incorrect here; the correct preposition is 'no' for 'in the cinema', but the verb 'atender' is wrong.

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
- Found (changes nothing): `nós` -> `nós` [word_choice] In Brazilian Portuguese, 'nós' is not used in this context with 'ajudamos' at A2 level.
- Missed: `nós ajudamos`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `pequeno-almoço` -> `pequeno-almoço` [not_brazilian] This is the European Portuguese spelling; the Brazilian Portuguese spelling is pequeno-almoço without the hyphen.
- Missed: `pequeno-almoço`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `chovendo` [content_mismatch] This phrase contradicts the scene fact that it is raining.
- Found (duplicate): `muito calor` -> `muito frio` [content_mismatch] This phrase contradicts the scene fact that it is cold.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `à direita` -> `direita` [article_preposition] In Brazilian Portuguese, 'à direita' is correct, but the preposition 'a' is not needed here.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Dois pessoas estão comendo espagueti e bebendo vinho tinto em um restaurante.` [content_mismatch] The answer does not describe the meal the people are eating.

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `à` -> `a` [article_preposition] The preposition 'a' should be used without the accent in this context.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós podermos` -> `para você podermos` [pronoun] The pronoun 'nós' is incorrect in this context; the correct pronoun is 'você' or 'vocês' depending on the audience.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estudar` [verb_form] The verb form should be in the infinitive after 'recomendo que'
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `reclamar` -> `reclamar do atraso` [word_choice] The verb 'reclamar' is transitive and requires a direct object. Here, it should be 'reclamar sobre o atraso' or 'reclamar do atraso'
- Found (false positive): `que deveria ter chegado` -> `que deveria ter chegado no dia dez` [word_order] The phrase should be 'que deveria ter chegado' (correct), but the sentence structure is not fully formal. A more formal structure would be 'que deveria ter chegado no dia dez'
- Missed: `sobre do`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `viajaria` -> `viajaria` [verb_form] The verb form should be in the future tense, not the conditional.
- Found (changes nothing): `tinha` -> `tinha` [verb_form] The verb form should be in the future tense, not the past perfect.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `apagar vários incêndios` -> `lidar com vários incêndios` [word_choice] 'Apagar incêndios' is not a common expression in Brazilian Portuguese for 'put out fires' in a stressful context; a more natural expression would be 'extinguish fires' or 'deal with fires' in a metaphorical sense.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dia dez` -> `décimo dia` [word_choice] In formal Brazilian Portuguese, it is more common to use the ordinal form 'décimo' instead of 'dia dez' in this context.

### a1-word-choice-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu sou vinte anos velho.
- Reference: `sou vinte anos velho` -> tenho vinte anos [word_choice]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `vinte anos velho` -> `vinte anos` [word_choice] The phrase is incorrect; Portuguese does not use 'velho' with age.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `prova` -> `teste` [word_choice] 'Prova' is used for exams or tests in a formal context, but at A2 level, 'teste' is more commonly used.
- Missed: `fiz um erro`

### a2-not-brazilian-phone (erroneous, a2)

- Task: Where is your phone?
- Answer: O meu telemóvel está em cima do sofá.
- Reference: `telemóvel` -> celular [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `telemóvel`

### a1-word-order-article-noun (erroneous, a1)

- Task: Where do you live?
- Answer: Eu moro com meus pais em casa uma pequena.
- Reference: `casa uma pequena` -> uma casa pequena [word_order]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `uma pequena` -> `uma pequena casa` [content_mismatch] This phrase is incomplete and does not make sense in Portuguese.

### a1-control-where-live (control, a1)

- Task: Where do you live?
- Answer: Moro no Brasil, numa cidade perto do mar.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Moro` -> `Morar` [verb_form] A1 learners should use the infinitive form 'morar' instead of the conjugated verb 'moro'.

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô` -> `Estou` [not_brazilian] 'Tô' is a colloquial form of 'estou' and is not considered standard Brazilian Portuguese for this level.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está` -> `estar` [verb_form] A1 learners should use the infinitive form 'estar' instead of the conjugated form.

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `doente` -> `doado` [word_choice] 'Doente' is not commonly used in Brazilian Portuguese to mean 'sick' in this context. The more natural word is 'doado' or 'doído' depending on the context.

### a2-control-family-meal (control, a2)

- Task: Describe the meal these people are eating.
- Answer: A família está comendo arroz, feijão e frango grelhado. Eles bebem suco de laranja.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `comendo` -> `estão comendo` [article_preposition] The verb 'estar' is missing, so the correct form is 'estão comendo' for third person plural.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para que eu passe no exame` -> `para passar no exame` [word_choice] The phrase is grammatically correct, but it is more natural to say 'para passar no exame' without the pronoun 'eu' in this context.

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `avós` -> `avôs` [agreement] In Brazilian Portuguese, 'avós' is incorrect; the correct form is 'avôs' (plural of 'avô').

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cheguei` -> `chegamos` [verb_form] The verb should be in the first person plural to match 'nós' (we) in the previous sentence.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `falei` -> `falo` [verb_form] The verb 'falei' is in the past tense, but the question is about a general action, so the present tense 'falo' is more appropriate.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei uma bicicleta nova no sábado.` -> `Eu acho que trabalhar em casa é bom porque você pode economizar tempo e dinheiro.` [content_mismatch] The answer does not address the question about working from home.

### a2-english-coffee (not_portuguese, a2)

- Task: Do you like coffee?
- Answer: Yes, I like coffee a lot.
- Flags: written_in_portuguese=true answers_the_question=true

## qwen3:8b / plain

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [article_preposition] The definite article 'o' is missing.
- Missed: `uma escritório`

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estou` -> `sou` [ser_estar] The verb 'estar' is not used to indicate origin.
- Found (changes nothing): `brasileiro` -> `brasileiro` [word_choice] The word 'brasileiro' is not used in Brazilian Portuguese to refer to someone from Brazil.

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `sou` -> `estou` [ser_estar] The verb 'ser' is not used to describe temporary states like being tired.
- Found (changes nothing): `cansado` -> `cansado` [not_brazilian] The adjective 'cansado' is masculine, but the subject is 'eu' (masculine), so it is correct. However, in Brazilian Portuguese, 'cansado' is more commonly used with 'eu' than 'cansada'.

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `assisto televisão` -> `assistō à televisão` [article_preposition] The preposition 'a' is missing before 'televisão'
- Missed: `jantar`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective 'vermelha' should agree in gender and number with the noun 'camisa', which is feminine singular. Therefore, it should be 'vermelha' (correct form).
- Found (changes nothing): `calça` -> `calça` [not_brazilian] The noun 'calça' is feminine, but the article 'uma' is used, which is correct. However, in Brazilian Portuguese, 'calça' is often used with the masculine article 'um' in informal contexts, though it's not a strict rule. This is not a mistake but a style choice.
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `muito` -> `muito` [word_order] The word 'muito' is not used correctly in this context. It should be 'muito' as an adverb, but it is not properly placed.
- Missed: `gosto de café muito`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo.` -> `Eu comi pão e café da manhã.` [content_mismatch] The answer does not address the question about what was eaten for breakfast.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `estudei` -> `estudei` [verb_form] The verb should be in the past historic tense for a narrative context.
- Found (matched): `portugues` -> `português` [spelling] The word is missing an accent mark.

### a2-verb-form-imperfect (erroneous, a2)

- Task: Describe your childhood.
- Answer: Quando eu era criança, eu moro numa cidade pequena e brincava na rua.
- Reference: `moro` -> morava [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `moro` -> `morava` [verb_form] The verb should be in the past tense because the sentence is about a childhood memory.
- Found (changes nothing): `brincava` -> `brincava` [verb_form] The verb should be in the past tense to match the rest of the sentence.

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `era` -> `estava` [ser_estar] The verb 'ser' is used incorrectly; 'estar' should be used for temporary states like being tired.
- Found (false positive): `fiquei` -> `fui` [word_choice] The verb 'ficar' is used incorrectly; 'ficar em casa' is not natural in this context. Use 'fui' or 'estava em casa' instead.
- Found (false positive): `dormi o dia todo` -> `todo o dia` [word_order] The phrase 'o dia todo' is not natural in Brazilian Portuguese. Use 'todo o dia' instead.

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
- Found (changes nothing): `nós` -> `nós` [word_choice] nós is not used in Brazilian Portuguese in this context with the verb 'ajudamos' when referring to a general habit.
- Found (matched): `ajudamos` -> `ajudam` [verb_form] ajudamos is incorrect in this context; the correct form is 'ajudam' for third person plural.

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
- Found (matched): `exige` -> `exigir` [verb_form] The verb form should be in the infinitive after 'embora'
- Missed: `assiste`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `reclamar sobre do` -> `reclamar por` [article_preposition] The preposition 'sobre' is incorrectly used with 'reclamar'; the correct preposition is 'por' or 'contra'.
- Found (false positive): `deveria ter chegado no dia dez` -> `no dia dez` [article_preposition] In Brazilian Portuguese, it is more natural to say 'no dia dez' without the preposition 'no' when referring to a specific date.

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
- Found (false positive): `cheguei em casa morto de cansaço` -> `morrer de cansaço` [word_order] The phrase is grammatically correct, but 'morto de cansaço' is more commonly expressed as 'morrer de cansaço' in Brazilian Portuguese.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `do atraso na entrega do meu pedido` -> `no dia dez` [article_preposition] The preposition 'do' is incorrect in this context; the correct preposition is 'do' for 'do atraso', but the phrase should be 'do atraso na entrega do meu pedido' is correct. However, the issue is with the use of 'do' in the phrase 'reclamar do atraso' which is correct. The mistake is actually in the use of 'no dia dez' which is incorrect.
- Found (false positive): `no dia dez` -> `no dia 10` [word_choice] In Brazilian Portuguese, it is more natural to say 'no dia 10' rather than 'no dia dez' when referring to a specific date.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `preocupo` -> `preocupo` [verb_form] The verb form is incorrect; the subject 'eu' requires the first person singular form of the verb 'preocupar'.
- Missed: `eu preocupo`

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu diverti`

### a1-short-verb-morar (erroneous, a1)

- Task: Where do you live?
- Answer: Eu mora em Lisboa.
- Reference: `mora` -> moro [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `mora` -> `mora` [verb_form] The verb 'mora' is incorrect; the correct form for 'live' in third person singular is 'morando' or 'mora' (but in Brazilian Portuguese, the present tense is 'mora' for third person singular).
- Missed: `mora`

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
- Found (matched): `doente` -> `do adoecido` [word_choice] doente is not commonly used in Brazilian Portuguese to mean 'sick' in this context
- Found (changes nothing): `não vai trabalhar` -> `não vai trabalhar` [verb_form] the verb form is incorrect for the context and level

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sempre fui`

### a2-not-brazilian-phone (erroneous, a2)

- Task: Where is your phone?
- Answer: O meu telemóvel está em cima do sofá.
- Reference: `telemóvel` -> celular [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `telemóvel`

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô` -> `Estou` [not_brazilian] Tô is a colloquial form of 'estou' which is not considered standard Brazilian Portuguese in formal contexts.
