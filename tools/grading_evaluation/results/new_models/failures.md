# Grading failures

Every call with a missed error, a false alarm, a quote not in the answer, a wrong flag, or an invalid response.

## gemma3n:e2b / few_shot

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
- Flags: written_in_portuguese=true answers_the_question=true
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
- Found (changes nothing): `moro` -> `moro` [verb_form] The verb 'moro' is in the infinitive form. It should be conjugated to the present tense, like 'moro'.
- Missed: `moro Toronto`

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `às oito` -> `às oito` [article_preposition] The preposition 'às' is incorrect. It should be 'às oito'.
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
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective should be 'vermelha' instead of 'vermelha' to agree with 'camisa'.
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
- Found (false positive): `futebol` -> `café` [word_choice] The word 'futebol' is not the correct word to describe what you eat for breakfast.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `portugues`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent on the e: café.
- Found (matched): `onibus` -> `ônibus` [spelling] The word needs an accent on the u: ônibus.
- Missed: `as nove`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `eu` [verb_form] The pronoun 'nós' is incorrect here. The verb 'almoçamos' needs to agree with the subject 'eu'.
- Missed: `volti`

### a2-verb-form-subject-agreement (erroneous, a2)

- Task: What did your family do on Sunday?
- Answer: No domingo minha família almoçaram na casa da minha avó.
- Reference: `almoçaram` -> almoçou [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `almoçaram` -> `almoçaram` [verb_form] The verb should be in the third person singular form, which is 'almoçaram'.
- Missed: `almoçaram`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective needs to agree in number with the noun 'inteligente'.
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
- Found (changes nothing): `nós` -> `nós` [pronoun] The pronoun 'nós' is incorrect here. The correct form is 'nós' when referring to a group of people.
- Missed: `nós ajudamos`

### a2-not-brazilian-gerund (erroneous, a2)

- Task: What are you doing now?
- Answer: Estou a comer pão com queijo na cozinha.
- Reference: `a comer` -> comendo [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a comer`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `há cinco anos` -> `sou morador(a) de São Paulo há cinco anos` [word_order] The correct expression is 'sou morador(a) de São Paulo há cinco anos'.

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

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ganhasse` -> `ganhassei` [verb_form] The verb form is incorrect; it should be 'ganhassei' (past subjunctive) to express a hypothetical situation.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `a gente` [pronoun] The pronoun 'nós' is unnecessary and incorrect here.

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `tinha começou`

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] The verb should be conjugated in the present tense, third person singular.
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

### b2-pronoun-clitic-gender (erroneous, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, eu falei com a Ana e convidei-o para a festa.
- Reference: `convidei-o` -> convidei-a [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `convidei-o` -> `convidei-o` [verb_form] The verb should be conjugated to agree with the pronoun 'eu'.
- Missed: `convidei-o`

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
- Found (changes nothing): `doente` -> `doente` [word_choice] The adjective should be 'doente' (sick) instead of 'doente'.
- Missed: `é doente`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `à` -> `para` [article_preposition] The preposition 'a' should be used with the verb 'ir' to indicate location.
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
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `pequena` -> `pequena` [agreement] The adjective needs to agree in number with the noun 'casa'.
- Missed: `casa uma pequena`

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô` -> `Estou` [verb_form] The verb 'estar' is used with 'tô' to indicate a temporary state, but it's better to use 'estou' for a more formal or precise statement.

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pequeno` -> `criança` [word_choice] The adjective 'pequeno' is not the best choice here. 'Criança' is more appropriate to describe a child.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `estou` -> `estou` [ser_estar] The verb 'estar' is used to describe a temporary state, while 'estou' is used to describe a state that is ongoing.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `comprei` -> `comprei` [verb_form] The verb form is incorrect. It should be 'comprei' to agree with the past tense.

## gemma3n:e2b / few_shot_six

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
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [spelling] The word needs an accent on the i: escritório.
- Missed: `uma escritório`

### a1-agreement-plural-adjective (erroneous, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos. Eles são muito simpático.
- Reference: `simpático` -> simpáticos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `simpático`

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `estou brasileiro`

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `cansado` -> `cansado` [agreement] The adjective needs to be in the masculine form to agree with 'eu'.
- Missed: `sou cansado`

### a1-article-contraction-em-o (erroneous, a1)

- Task: Where do you live?
- Answer: Eu moro em o Brasil, em São Paulo.
- Reference: `em o` -> no [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `em o` -> `em o` [article_preposition] The preposition 'em' is missing the article 'o'.
- Missed: `em o`

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `moro` -> `moro` [verb_form] The verb should be 'moro' (live) instead of 'moro' (I live).
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
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `gosto de café muito`

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `azul`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `à tarde` -> `à tarde` [article_preposition] The preposition 'à' is incorrect. It should be 'à tarde'.
- Missed: `portugues`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The word needs to be 'café' with the accent mark.
- Missed: `onibus`
- Missed: `as nove`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `eu` [pronoun] The pronoun should be 'eu' because the subject is 'eu'.
- Missed: `volti`

### a2-verb-form-subject-agreement (erroneous, a2)

- Task: What did your family do on Sunday?
- Answer: No domingo minha família almoçaram na casa da minha avó.
- Reference: `almoçaram` -> almoçou [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `almoçaram` -> `almoçaram` [verb_form] The verb must agree with 'minha família': almoçaram.
- Missed: `almoçaram`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective needs to agree in number with the noun 'inteligente'.
- Missed: `curtas`

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `por passar`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `à noite` -> `na` [article_preposition] The preposition 'à' is incorrect. It should be 'na'.
- Missed: `atendi`

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
- Found (changes nothing): `nós` -> `nós` [pronoun] The pronoun should be 'nós' when referring to a group of people.
- Missed: `nós ajudamos`

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
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fazendo sol e muito calor`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `há cinco anos` -> `sou morador há cinco anos` [word_order] The correct expression is 'sou morador há cinco anos'.

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

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The verb should be in the subjunctive mood, but the form is incorrect.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `nós` -> `nós` [pronoun] The pronoun should be 'nós' (we) not 'nós' (us).

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] The verb should be in the imperative mood, which is 'estude'.
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

### b2-pronoun-clitic-gender (erroneous, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, eu falei com a Ana e convidei-o para a festa.
- Reference: `convidei-o` -> convidei-a [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `convidei-o` -> `convidei-o` [verb_form] The verb should be conjugated to agree with the pronoun 'eu'.
- Missed: `convidei-o`

### a1-short-agreement-books (erroneous, a1)

- Task: What is in your bag?
- Answer: Na minha bolsa tem um livros.
- Reference: `um livros` -> um livro | uns livros [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `livros` -> `livros` [spelling] The noun needs to be in the plural form.
- Missed: `um livros`

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
- Found (false positive): `à` -> `a` [article_preposition] The preposition 'a' should be 'a'
- Missed: `sempre fui`

### b2-agreement-news (erroneous, b2)

- Task: Comment on the news.
- Answer: As notícias de hoje são preocupante.
- Reference: `preocupante` -> preocupantes [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `preocupante` -> `preocupante` [agreement] The adjective needs to be in the feminine form to agree with 'notícias'.
- Missed: `preocupante`

### a2-not-brazilian-phone (erroneous, a2)

- Task: Where is your phone?
- Answer: O meu telemóvel está em cima do sofá.
- Reference: `telemóvel` -> celular [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `telemóvel`

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `era pequeno` -> `eu era` [verb_form] The verb 'era' should be 'eu era' to indicate the subject.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `comprei` -> `comprei` [verb_form] The verb must agree with 'eu': comprei.

## gemma3n:e2b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de` -> `` [article_preposition] The preposition 'de' is unnecessary in this sentence.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `cansado` -> `cansado` [agreement] The adjective 'cansado' needs to agree with the subject 'eu' (I) in number and gender. Since 'eu' is singular and neuter, the correct form is 'cansado'.

### a1-control-daily-routine (control, a1)

- Task: What do you do every day?
- Answer: Todo dia eu acordo cedo, tomo café e vou para o trabalho.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `cedo` -> `cedo` [word_choice] The adverb 'cedo' (early) is correct, but the sentence could be more natural with a more common word for 'wake up'.

### a1-control-order-meal (control, a1)

- Task: You are at a restaurant. Order your meal.
- Answer: Eu queria uma salada e um suco de laranja, por favor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `suco` -> `refrigerante` [word_choice] The word 'suco' is used for juice, but 'refrigerante' is more appropriate for a soft drink.

### a1-control-family (control, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos e uma irmã. Meu pai é médico e minha mãe é professora.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `irmã` -> `irmã` [agreement] The word 'irmã' is correct, but the sentence is missing the article 'a'.

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `moro` -> `moro` [verb_form] The verb 'moro' is the third person singular of the present indicative of 'morar' (to live). The correct form for 'I' is 'eu moro'.

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `feijão` -> `feijão` [word_choice] The word 'feijão' is a common term for beans in Brazilian Portuguese, but it's not the most specific or natural way to say 'beans' in a general sense.
- Missed: `fejão`

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [word_choice] The word 'cafe' is not the correct word for coffee in Brazilian Portuguese.
- Missed: `manha`

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cansado` -> `estou cansado` [word_order] The adjective 'cansado' is correct, but the sentence structure is not typical for expressing feelings after work in Brazilian Portuguese.
- Found (false positive): `com fome` -> `estou com fome` [word_choice] While 'com fome' is correct, it's a bit informal. A more common way to express being hungry after work would be to use a more descriptive phrase.

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `Toronto` -> `Eu sou do Canadá e moro em Toronto.` [word_choice] Toronto is a city, not a place where someone lives.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `chamo` -> `chamo` [verb_form] The verb 'chamar' (to call) needs to be conjugated to the first person singular present tense, which is 'chamo'.
- Missed: `chamo`

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `chama` -> `chama` [verb_form] The verb 'chamar' (to call) needs to be conjugated to agree with the subject 'Maria'.
- Missed: `chama`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `muito` -> `muito` [word_choice] While 'muito' is correct, 'muito' is often used to emphasize a feeling, and in this case, 'muito' is redundant.
- Missed: `gosto de café muito`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `gosto de futebol` -> `como` [word_choice] The verb 'gostar' (to like) is not the correct verb to use when talking about what you eat for breakfast. You should use a verb like 'comer' (to eat).

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [word_choice] The word 'cafe' is not the correct word for coffee in Brazilian Portuguese.
- Found (matched): `onibus` -> `ônibus` [word_choice] The word 'onibus' is not the correct word for bus in Brazilian Portuguese.
- Missed: `as nove`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `nós comemos` [agreement] The pronoun 'nós' (we) is used with the verb 'almoçamos' (we ate) in the third person plural, which is incorrect. The correct form is 'nós comemos' (we ate).
- Missed: `volti`

### a2-verb-form-subject-agreement (erroneous, a2)

- Task: What did your family do on Sunday?
- Answer: No domingo minha família almoçaram na casa da minha avó.
- Reference: `almoçaram` -> almoçou [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `almoçaram` -> `almoçaram` [verb_form] The verb 'almoçar' needs to be conjugated to the third person plural to agree with 'minha família'.
- Missed: `almoçaram`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The word 'engraçado' means 'funny' but is masculine. The adjective needs to agree with the noun 'amiga' (feminine).
- Missed: `curtas`

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `muito` -> `muito` [word_choice] The adverb 'muito' is redundant in this context; 'muito' already implies a large amount of studying.
- Missed: `por passar`

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tomo` -> `tomo banho` [verb_form] The verb 'tomar' means 'to take' or 'to drink'. The correct verb for 'to take a bath' is 'tomar banho'.
- Missed: `lavo os dentes`

### a2-pronoun-reciprocal (erroneous, a2)

- Task: Describe your relationship with your neighbors.
- Answer: Eu me dou bem com meus vizinhos e sempre nós ajudamos quando precisamos.
- Reference: `nós ajudamos` -> nos ajudamos [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `nós` -> `nós` [agreement] The pronoun 'nós' (we) is incorrect here. The verb 'ajudamos' needs to agree with the subject, which is 'meus vizinhos' (my neighbors).
- Missed: `nós ajudamos`

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `num` -> `em` [article_preposition] The preposition 'em' is needed with 'num' to indicate location in a hotel.
- Found (false positive): `nós` -> `nós ficamos` [verb_form] The verb 'ficamos' is in the third person plural, but the subject is 'nós' (we). The correct form should be 'nós ficamos'.

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `sábado` -> `sábado` [spelling] The word 'sábado' is correct.
- Found (changes nothing): `meus avós` -> `meus avós` [agreement] The possessive adjective 'meus' is correct.
- Found (false positive): `ao mercado` -> `no mercado` [article_preposition] The preposition 'a' is needed before 'mercado' to indicate going to a place.

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `mais novo` -> `mais novo` [agreement] The adjective 'novo' (new) needs to agree in number with the noun 'celular' (phone).

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `à` -> `no` [article_preposition] The preposition 'a' is used with the verb 'vire' (turn) in this context, but it should be 'no'.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `num` -> `em` [article_preposition] The preposition 'em' is needed before 'restaurante' to indicate location.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `muito` -> `gostei` [word_choice] While 'muito' is correct, 'gostei' is a more natural way to express liking a film in Portuguese.

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cozinhar` -> `cozinhando` [verb_form] The verb 'cozinhar' is the infinitive form of the verb 'to cook'. The gerund form 'cozinhando' is needed here.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (changes nothing): `moro` -> `moro` [verb_form] The verb 'morar' (to live) needs to agree with the subject 'eu' (I) in the present tense. The correct form is 'moro'.

### b1-verb-form-subjunctive-present (erroneous, b1)

- Task: What do you hope for the future?
- Answer: Espero que meus filhos são felizes e que eles tenham uma boa educação.
- Reference: `são` -> sejam [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `são` -> `são` [agreement] The verb 'ser' (to be) is used with people, but 'são' is the third-person plural form of 'ser' which is incorrect here.
- Missed: `são`

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ter` -> `ter` [verb_form] The verb 'ter' (to have) is not the correct verb to use with 'tempo' (time) in this context. 'Ter tempo' is not the correct way to express having free time.
- Missed: `Se eu ter`

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tenho` -> `tenho` [verb_form] The verb 'ter' (to have) is not the correct verb to use with 'nunca' (never) in this context. 'Tenho' is the present tense of 'ter' and doesn't express a habitual action.
- Missed: `tenho visitado`

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `possa` -> `possa` [verb_form] The verb 'poder' needs to be conjugated to the subjunctive mood to express possibility, but the form 'possa' is incorrect.
- Found (false positive): `traga` -> `traça` [verb_form] The verb 'trazer' needs to be conjugated to the subjunctive mood to express possibility, but the form 'traga' is incorrect.

### b1-control-future-subjunctive (control, b1)

- Task: What will you do when you finish the course?
- Answer: Quando eu terminar o curso, vou procurar um emprego novo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `novo` -> `novos` [word_choice] The adjective 'novo' (new) doesn't fit well with 'emprego' (job) in this context. A better word would be 'novas'.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The verb 'ganhar' needs to be conjugated to the subjunctive mood in the conditional sentence.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós` -> `a fim de que` [word_choice] The preposition 'para' is incorrect here; 'a fim de que' is more appropriate to express purpose.

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a gente` -> `nós` [pronoun] While 'a gente' is common in spoken Brazilian Portuguese, it's generally better to use 'nós' (we) in formal or more precise writing.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Prezado` -> `Caro` [word_choice] The word 'Prezado' is a bit formal for a request to your boss. 'Caro' or 'Prezado' (less formal) would be more appropriate.

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `começou` -> `começou` [verb_form] The verb 'começar' (to start) is needed here, not 'começou' (started).
- Missed: `tinha começou`

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `estuda` -> `estuda` [verb_form] The verb 'estudar' (to study) needs to be conjugated to the present tense, third person singular. The correct form is 'estuda'.
- Missed: `estuda`
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dez` -> `décimo` [word_choice] The number 'dez' is used for the month, not for the day.
- Missed: `sobre do`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `irei` [verb_form] The verb 'ir' (to go) needs to be conjugated with the future tense, which is 'ir + infinitive'.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morto` -> `exausto` [word_choice] The word 'morto' means 'dead', not 'exhausted'.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dez` -> `10` [word_choice] The number 'dez' is used for the month, not for the day.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `a ele` [article_preposition] The pronoun 'lhe' is not needed in this sentence; 'a ele' is more natural.

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `doente` -> `doente` [word_choice] The adjective 'doente' (sick) is better used with 'ele' (he) or 'ela' (she) to describe a person, not 'irmão' (brother).
- Missed: `é doente`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `à` -> `a` [article_preposition] The preposition 'a' should be 'a' when followed by a verb like 'ir' (to go).
- Missed: `sempre fui`

### a1-word-order-article-noun (erroneous, a1)

- Task: Where do you live?
- Answer: Eu moro com meus pais em casa uma pequena.
- Reference: `casa uma pequena` -> uma casa pequena [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `pequena` -> `pequena` [word_choice] The adjective 'pequena' doesn't fit well with 'casa' in this context. It's better to use a word that describes the size of the house.
- Missed: `casa uma pequena`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `irmã` -> `irmã` [agreement] The word 'irmã' is correct.

### a1-control-accents-present (control, a1)

- Task: What do you eat for breakfast?
- Answer: De manhã eu como pão com queijo e bebo café com leite.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `com leite` -> `com leite` [article_preposition] The preposition 'com' is redundant when describing what is consumed with something else.

### a1-control-where-live (control, a1)

- Task: Where do you live?
- Answer: Moro no Brasil, numa cidade perto do mar.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no` -> `em` [article_preposition] The preposition 'no' is not needed before 'Brasil' when referring to the country.

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pra` -> `para` [article_preposition] The preposition 'pra' is not correct; 'para' is needed.
- Found (false positive): `Tô` -> `Estou` [verb_form] The verb 'tô' is a spoken form and not appropriate for written answers.

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tem` -> `tenho` [verb_form] The verb 'ter' (to have) needs to agree with the subject 'minha bolsa' (my bag) in number. The correct form is 'tenho'.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `marrons` -> `marrom` [word_choice] The adjective 'marrom' means brown, but 'marrom' is used for color in Brazilian Portuguese. 'Marrons' is the plural form of 'marrom' and is not the correct adjective to describe the color of the boots.

### a2-control-preterite-sunday (control, a2)

- Task: What did you do on Sunday?
- Answer: No domingo eu acordei tarde, fiz o almoço e depois assisti a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `acordei` -> `acordei` [verb_form] The verb 'acordar' needs to agree with the subject 'eu' in number. The correct form is 'acordei'.

### a2-control-imperfect-grandmother (control, a2)

- Task: Describe your grandmother's house.
- Answer: A casa da minha avó era pequena, mas tinha um jardim grande com muitas flores.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `grande` -> `grande` [agreement] The adjective 'grande' is used with 'jardim', which is masculine. 'Grande' should be used with a feminine noun.

### a2-control-reflexive-party (control, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu me diverti muito.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ótima` -> `ótima` [word_choice] The adjective 'ótima' is used correctly to describe 'festa'.

### a2-control-pet (control, a2)

- Task: Describe your pet.
- Answer: Eu tenho um cachorro. O meu cachorro se chama Bolt e ele adora correr.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `meu` -> `meu` [word_choice] The possessive adjective 'meu' is redundant when describing a pet. It's implied.

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `doente` -> `doente` [word_choice] The adjective 'doente' is correct, but the sentence could be improved for clarity.

### a2-control-family-meal (control, a2)

- Task: Describe the meal these people are eating.
- Answer: A família está comendo arroz, feijão e frango grelhado. Eles bebem suco de laranja.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `está` -> `está` [ser_estar] The verb 'estar' is used to describe a temporary state, while 'estar' is used to describe a state of being. In this case, 'estar' is not the correct verb to use.

### a2-control-phone (control, a2)

- Task: Where is your phone?
- Answer: O meu celular está em cima do sofá.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `em cima` -> `sobre` [word_choice] While grammatically correct, 'em cima' is a very common and informal way to say 'on top of'. A more formal and precise option would be 'sobre'.

### a2-control-cinema-contraction (control, a2)

- Task: Where did you go on Saturday?
- Answer: No sábado eu fui ao cinema com meus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No` -> `no` [article_preposition] The preposition 'no' is incorrect; 'no' should be 'no' in this context.
- Found (false positive): `meus` -> `meus amigos` [agreement] The possessive adjective 'meus' is incorrect; it should be 'meus amigos' to indicate 'my friends'.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `todos os dias` -> `diariamente` [word_choice] The phrase 'todos os dias' is redundant; 'diariamente' is more concise and natural.

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pequeno` -> `criança` [word_choice] The word 'pequeno' means 'small', not 'childhood'.

### b1-control-gostar-de (control, b1)

- Task: What do you like about the city?
- Answer: Eu gosto da cidade porque é tranquila e as pessoas são simpáticas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tranquila` -> `calma` [word_choice] The adjective 'tranquila' is correct, but it's a bit informal. A more standard word for 'quiet' would be 'calma'.

### b1-control-weather-hot (control, b1)

- Task: Describe the weather today.
- Answer: Hoje faz muito calor e o céu está sem nuvens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `sem nuvens` -> `sem nuvens` [word_choice] The phrase 'sem nuvens' is not the most natural way to say 'without clouds' in Portuguese.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `em` -> `a` [article_preposition] The preposition 'em' is not appropriate here; 'a' is needed to indicate arrival at a place.

### b2-control-news (control, b2)

- Task: Comment on the news.
- Answer: As notícias de hoje são preocupantes, mas acredito que a situação possa melhorar.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `preocupantes` -> `alarmantes` [word_choice] The adjective 'preocupantes' is not the best choice here. 'Preocupantes' implies something is causing worry, while 'alarmantes' is more appropriate for news.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `convidei-a` -> `convidei-a` [agreement] The pronoun 'a' is unnecessary after 'convidei' and the verb 'invitar' is in the imperative mood.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `estou` -> `estou` [ser_estar] The verb 'estar' is needed here to describe the current state of being at home.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `comprei` -> `comprá` [verb_form] The verb 'comprar' (to buy) is in the past tense, but the sentence is about a present action (buying a new bike).

## gemma3n:e4b / few_shot

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
- Found (false positive): `e` -> `e eu gosto de futebol` [word_order] The sentence is missing a verb to connect the two clauses.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `portugues`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tomei cafe` -> `tomei café` [word_choice] The verb 'tomar' is not used with 'café'.
- Missed: `onibus`
- Missed: `as nove`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `nós almoçamos` -> `nós almoçamos` [verb_form] The verb 'almoçar' does not agree with the pronoun 'nós'.
- Missed: `volti`

### a2-verb-form-subject-agreement (erroneous, a2)

- Task: What did your family do on Sunday?
- Answer: No domingo minha família almoçaram na casa da minha avó.
- Reference: `almoçaram` -> almoçou [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `almoçaram` -> `almoçaram` [verb_form] The verb conjugation is incorrect. It should be 'almoçaram'.
- Missed: `almoçaram`

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
- Found (changes nothing): `nós ajudamos` -> `nós ajudamos` [verb_form] The verb 'ajudar' needs to agree with the subject 'nós', so it should be 'nós ajudamos'.
- Missed: `nós ajudamos`

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
- Found (matched): `fazendo sol` -> `Está chovendo` [content_mismatch] The weather is not sunny, it is raining.

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `num` -> `em` [article_preposition] The contraction should be 'em', not 'num'.

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ao mercado` -> `para o mercado` [article_preposition] The preposition 'a' is missing before 'mercado'.

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `ganhava`

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
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The correct form is 'ganhasse'.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estudar` [verb_form] The verb should be in the infinitive form after 'que'.
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sobre do`

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morto de cansaço` -> `exausto` [word_choice] The expression is not idiomatic in Brazilian Portuguese.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `ele` [pronoun] 'lhe' is too formal; 'ele' is more appropriate.

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

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para que eu passe` -> `para passar` [word_choice] The correct expression is 'para passar', not 'para que eu passe'.

## gemma3n:e4b / few_shot_six

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
- Missed: `gosto de café muito`

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `à tarde` -> `à tarde` [article_preposition] The preposition 'de' is needed before 'à tarde'.
- Missed: `portugues`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tomei cafe` -> `tomei café que` [word_choice] The verb 'tomar' needs to be followed by the preposition 'que' when referring to a drink: tomei café que.
- Missed: `onibus`
- Missed: `as nove`

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
- Found (changes nothing): `almoçaram` -> `almoçaram` [verb_form] The verb must agree with 'minha família': almoçaram.
- Missed: `almoçaram`

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
- Found (changes nothing): `nós ajudamos` -> `nós ajudamos` [verb_form] The verb 'ajudar' needs to agree with the subject 'nós', so it should be 'nós ajudamos'.
- Missed: `nós ajudamos`

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
- Found (matched): `fazendo sol` -> `Está chovendo` [content_mismatch] The weather is raining, not sunny.

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ao mercado` -> `o mercado` [article_preposition] The preposition 'a' is not needed here.

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No meu tempo livre` -> `No meu tempo livre eu` [article_preposition] The correct expression is 'No meu tempo livre eu...'.

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `ganhava`

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The correct form is 'ganhasse'.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós podermos` -> `a fim de podermos` [word_order] The construction 'para nós podermos' is not the most natural way to express the reason.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estudar` [verb_form] The verb should be in the infinitive form after 'que'.
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `por meio desta` -> `gostaria de` [word_choice] This phrase is too formal and sounds unnatural in a complaint.
- Missed: `sobre do`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria na` -> `viajar` [verb_form] The verb 'viajar' is not used in the conditional tense.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `em casa morto` -> `morto de cansaço` [word_choice] The correct expression is 'morto de cansaço', not 'morto em cansaço'.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `a ele` [pronoun] 'a ele' is more common in Brazilian Portuguese.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu preocupo`

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

### a1-word-order-article-noun (erroneous, a1)

- Task: Where do you live?
- Answer: Eu moro com meus pais em casa uma pequena.
- Reference: `casa uma pequena` -> uma casa pequena [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `uma pequena` -> `uma pequena` [agreement] The adjective 'pequena' needs to agree in gender and number with 'casa', so it should be 'uma pequena'.
- Missed: `casa uma pequena`

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `para que eu passe` -> `para que eu passe` [verb_form] The correct expression is 'para que eu passe' or 'para passar'.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `em` -> `à` [article_preposition] The preposition 'em' is not needed here.

## gemma3n:e4b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de maiô` -> `em` [article_preposition] The preposition 'de' is not needed here; 'em' is more appropriate.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Só um pouco cansado` -> `Estou um pouco cansado` [word_order] The phrase is grammatically correct, but it's more natural to say 'Estou um pouco cansado' or 'Estou só um pouco cansado'.

### a1-control-daily-routine (control, a1)

- Task: What do you do every day?
- Answer: Todo dia eu acordo cedo, tomo café e vou para o trabalho.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para o trabalho` -> `ao trabalho` [article_preposition] The preposition 'para' is not needed when indicating the destination of a work activity.

### a1-control-order-meal (control, a1)

- Task: You are at a restaurant. Order your meal.
- Answer: Eu queria uma salada e um suco de laranja, por favor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `uma salada` -> `a salada` [agreement] The word 'salada' is feminine, so it needs a feminine article 'a'.
- Found (false positive): `um suco` -> `o suco` [agreement] The word 'suco' is masculine, so it needs a masculine article 'o'.

### a1-control-family (control, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos e uma irmã. Meu pai é médico e minha mãe é professora.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `tenho dois irmãos e uma irmã` -> `Tenho dois irmãos e uma irmã.` [word_choice] The verb 'ter' is not typically used to describe family members. 'Ter' means 'to have'.

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `moro em` -> `moro em` [article_preposition] The verb 'morar' (to live) is not used with 'em' when indicating residence; 'morar em' is grammatically incorrect.

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de manha` -> `de manhã` [spelling] The correct expression is 'de manhã', not 'de manha'.
- Missed: `cafe`

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `moro Toronto` -> `moro em Toronto` [article_preposition] The verb 'moro' (I live) needs a preposition to indicate the city.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `e tenho` -> `e eu tenho` [word_order] The phrase is grammatically incorrect; it should be 'e eu tenho'.
- Missed: `chamo`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu gosto de futebol e` -> `Eu comi...` [content_mismatch] This sentence talks about liking football, not what was eaten for breakfast.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `à tarde` -> `à tarde` [article_preposition] The preposition 'a' is not needed before 'tarde' in this context.
- Missed: `portugues`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `tomei cafe` -> `bebi café` [word_choice] The verb 'tomar' is not typically used with 'cafe' to mean 'drink coffee'.
- Found (matched): `de onibus` -> `em ônibus` [article_preposition] The preposition 'de' is not correct here; 'em' is needed.
- Found (matched): `as nove` -> `às nove` [article_preposition] The correct way to say 'at nine' is 'às nove'.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `nós almoçamos` -> `nós almoçamos` [verb_form] The verb 'almoçar' is not used with the pronoun 'nós'; 'nós almoçamos' is grammatically incorrect.
- Missed: `volti`

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tomo banho` -> `faço banho` [word_choice] While not strictly incorrect, 'tomar banho' is less common in Brazilian Portuguese than 'fazer banho'.
- Missed: `lavo os dentes`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para o trabalho` -> `ao trabalho` [article_preposition] The preposition 'para' is not needed here; 'a' is more appropriate.
- Missed: `autocarro`

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `num` -> `em` [article_preposition] The contraction 'num' is not used in formal writing; 'em' is more appropriate.

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ao mercado` -> `no mercado` [article_preposition] The preposition 'a' is not needed before 'mercado' in this context.

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `do que` -> `que` [word_choice] While not strictly incorrect, 'do que' is less common and sounds slightly less natural than 'que' in this context.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `à direita` -> `direita` [article_preposition] The preposition 'a' is not needed here; 'à' is redundant.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `num` -> `em` [article_preposition] The contraction 'num' is not used in Brazilian Portuguese; 'em' should be used instead.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> `` [word_choice] The pronoun 'dele' is redundant because 'o filme' is already the object.

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No meu tempo livre` -> `No meu tempo livre eu gosto` [word_order] The phrase 'No meu tempo livre' is not incorrect, but it's not the most natural way to say 'In my free time' in Brazilian Portuguese. A more common phrasing would be 'No meu tempo livre eu gosto...' or 'No meu tempo livre, eu gosto...' 

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eles estão comendo espaguete.` [content_mismatch] The answer describes where the speaker lives, not the meal.

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `tenho visitado` -> `já visitei` [verb_form] The verb tense is incorrect; it should be in the past participle to form the perfect infinitive.

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `que traga` -> `que traga` [verb_form] The subjunctive 'traga' is not needed after 'que' in this context; the indicative 'traga' is more appropriate.

### b1-control-future-subjunctive (control, b1)

- Task: What will you do when you finish the course?
- Answer: Quando eu terminar o curso, vou procurar um emprego novo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `um emprego novo` -> `um emprego` [word_choice] The word 'novo' is redundant because 'emprego' implies something new.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The correct form of the verb 'ganhar' in the conditional tense is 'ganhasse'.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós podermos` -> `a fim de podermos` [article_preposition] The preposition 'para' is not needed here; 'para' indicates a destination, not purpose.

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a gente economiza tempo` -> `nós economizamos tempo` [pronoun] While 'a gente' is common in spoken Brazilian Portuguese, it's less formal and not ideal for written answers. A more appropriate alternative would be 'nós' or 'você' (if addressing someone directly).

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `folga` -> `dia de descanso` [word_choice] While not strictly incorrect, 'folga' is less common and sounds slightly less formal than 'dia de descanso' or 'dia de férias' in this context.

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `tinha começou` -> `já tinha começado` [verb_form] The verb 'ter' is missing before 'começou', which is required for the past perfect tense.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda todos os dias` -> `estude todos os dias` [verb_form] The verb 'estudar' should be conjugated in the subjunctive mood after 'que' in a subordinate clause expressing a recommendation.
- Found (matched): `exige paciência` -> `exija paciência` [verb_form] The verb 'exigir' should be conjugated in the third person singular to agree with 'isso'.
- Missed: `assiste`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `deveria ter chegado no dia dez` -> `em o dia dez` [article_preposition] The preposition 'no' is not used before the day of the month in this context; 'em' is more appropriate.
- Missed: `sobre do`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `vou viajar` [verb_form] The original sentence is in the future, so the correct tense is 'vou viajar'.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morto de cansaço` -> `exausto` [word_choice] The expression is not incorrect, but 'exausto' is more common and natural in this context.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `no dia 10` [article_preposition] The correct phrasing is 'no dia 10' or 'no dia dezessete' (if it's the 17th).

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `ele` [pronoun] The pronoun 'lhe' is too formal for speaking to a brother; 'ele' is more appropriate.

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `à praia` -> `na praia` [article_preposition] The preposition 'a' requires the definite article 'a' when used with 'praia'.
- Missed: `sempre fui`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `uma irmã` -> `uma irmã` [agreement] The word 'irmã' is feminine, so it needs a feminine article 'uma'.

### a1-control-accents-present (control, a1)

- Task: What do you eat for breakfast?
- Answer: De manhã eu como pão com queijo e bebo café com leite.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `como` -> `eu como` [verb_form] The verb 'como' is in the third person singular, but the subject 'eu' is first person singular, so it should be 'eu como'.

### a1-control-where-live (control, a1)

- Task: Where do you live?
- Answer: Moro no Brasil, numa cidade perto do mar.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no Brasil` -> `em o Brasil` [article_preposition] The preposition 'em' is required before 'Brasil' when indicating location.

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pra` -> `para` [spelling] The preposition 'para' is misspelled.
- Found (false positive): `Tô` -> `Estou` [not_brazilian] The use of 'tô' is too informal for this level.

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tem` -> `há` [verb_form] The verb 'ter' is not the most natural choice here; 'haver' is more common when describing contents.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de vestido` -> `em` [article_preposition] The preposition 'de' is not needed here; 'em' is more appropriate to describe what someone is wearing.

### a2-control-preterite-sunday (control, a2)

- Task: What did you do on Sunday?
- Answer: No domingo eu acordei tarde, fiz o almoço e depois assisti a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `o almoço` -> `a almoço` [article_preposition] It is more common to say 'a' almoço when referring to a meal.

### a2-control-imperfect-grandmother (control, a2)

- Task: Describe your grandmother's house.
- Answer: A casa da minha avó era pequena, mas tinha um jardim grande com muitas flores.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tinha um jardim grande` -> `tinha um jardim grande` [verb_form] The verb 'ter' (to have) is more common than 'tinham' (they had) when describing possession.

### a2-control-reflexive-party (control, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu me diverti muito.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `me diverti` -> `diverti` [pronoun] The reflexive pronoun 'me' is not needed here; 'diverti' is sufficient.

### a2-control-pet (control, a2)

- Task: Describe your pet.
- Answer: Eu tenho um cachorro. O meu cachorro se chama Bolt e ele adora correr.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `se chama` -> `chama` [word_choice] The verb 'chamar' (to call) is not the correct verb to use when describing a pet's name; 'chamar' is used for people.

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `doente` -> `como está` [word_choice] While not strictly incorrect, 'doente' implies a more serious illness than simply 'how is your brother'.

### a2-control-family-meal (control, a2)

- Task: Describe the meal these people are eating.
- Answer: A família está comendo arroz, feijão e frango grelhado. Eles bebem suco de laranja.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está comendo` -> `comendo` [word_order] While not strictly incorrect, 'está comendo' is a bit redundant; 'comendo' alone is sufficient.

### a2-control-phone (control, a2)

- Task: Where is your phone?
- Answer: O meu celular está em cima do sofá.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `em cima do sofá` -> `sobre o sofá` [word_choice] While not strictly incorrect, 'em cima do' is a bit redundant; 'sobre o sofá' is more natural.

### a2-control-cinema-contraction (control, a2)

- Task: Where did you go on Saturday?
- Answer: No sábado eu fui ao cinema com meus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ao` -> `ao` [article_preposition] The preposition 'a' is not needed before 'cinema' in this context.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `para que eu passe` -> `para que eu passe` [verb_form] The verb 'passar' needs to be in the infinitive form after 'para que'.

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ia à` -> `ia` [article_preposition] The preposition 'a' is unnecessary here; 'ia' already implies going to the beach.

### b1-control-gostar-de (control, b1)

- Task: What do you like about the city?
- Answer: Eu gosto da cidade porque é tranquila e as pessoas são simpáticas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `da cidade` -> `da cidade` [article_preposition] The preposition 'de' is more appropriate here.

### b1-control-weather-hot (control, b1)

- Task: Describe the weather today.
- Answer: Hoje faz muito calor e o céu está sem nuvens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `sem nuvens` -> `limpo` [word_choice] The phrase 'sem nuvens' is not a natural way to describe a clear sky in Brazilian Portuguese; 'livre de nuvens' or 'limpo' would be better.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `em casa` -> `casa` [article_preposition] The preposition 'em' is not needed here; 'em casa' is redundant.

### b2-control-news (control, b2)

- Task: Comment on the news.
- Answer: As notícias de hoje são preocupantes, mas acredito que a situação possa melhorar.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `As notícias de hoje são preocupantes, mas` -> `As notícias de hoje são preocupantes, mas` [article_preposition] The conjunction 'mas' is missing a comma after the subordinate clause.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `convidei-a` -> `convidei ela` [pronoun] The pronoun 'a' is unnecessary and makes the sentence sound overly formal in Brazilian Portuguese.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `estou em` -> `Meu nome é` [content_mismatch] The question asks for the name, not the location.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei` -> `Eu acho ótimo.` [content_mismatch] The answer is about buying a bicycle, not about opinions on working from home.

## gemma4:12b-it-qat / few_shot

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=false answers_the_question=true
- Missed: `beans`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The word needs a circumflex accent: café.
- Found (matched): `as` -> `às` [article_preposition] Time expressions require a grave accent for the contraction: às.
- Missed: `onibus`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective must be feminine to agree with 'amiga'.
- Found (false positive): `cabelos` -> `cabelo` [word_choice] In Portuguese, 'cabelo' is usually used in the singular to describe the hair on someone's head.
- Found (matched): `curtas` -> `curto` [agreement] The adjective must be singular to agree with 'cabelo'.

### a2-pronoun-reciprocal (erroneous, a2)

- Task: Describe your relationship with your neighbors.
- Answer: Eu me dou bem com meus vizinhos e sempre nós ajudamos quando precisamos.
- Reference: `nós ajudamos` -> nos ajudamos [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `nós ajudamos`

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `tinha começou`

### a1-short-spelling-chocolate (erroneous, a1)

- Task: What do you like?
- Answer: Eu gosto de chocolat.
- Reference: `chocolat` -> chocolate [spelling]
- Flags: written_in_portuguese=false answers_the_question=true
- Missed: `chocolat`

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

## gemma4:12b-it-qat / few_shot_six

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=false answers_the_question=true
- Missed: `beans`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent mark: café.
- Found (matched): `as nove` -> `às nove` [article_preposition] Time expressions require a grave accent for the contraction: às nove.
- Missed: `onibus`

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `por passar`

### a1-short-spelling-chocolate (erroneous, a1)

- Task: What do you like?
- Answer: Eu gosto de chocolat.
- Reference: `chocolat` -> chocolate [spelling]
- Flags: written_in_portuguese=false answers_the_question=true
- Missed: `chocolat`

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

## gemma4:12b-it-qat / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A Marina` -> `Marina` [article_preposition] In Portuguese, you generally do not use a definite article before a person's name.

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fejão`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No almoço` -> `No almoço` [article_preposition] The preposition 'no' (em + o) is used for location; 'no' (na) should be used for time periods like lunch.
- Found (matched): `beans` -> `feijão` [word_choice] The word 'beans' is English and should be translated to Portuguese.

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `muito` -> `muito` [word_order] In Portuguese, adverbs of intensity like 'muito' usually come before the verb.
- Missed: `gosto de café muito`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo.` -> `` [content_mismatch] The answer does not address the question about what you ate for breakfast.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eles estão comendo espaguete e bebendo vinho tinto.` [content_mismatch] The answer describes where the speaker lives instead of describing the meal in the scene.

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A Ana` -> `Ana` [article_preposition] In Portuguese, it is generally incorrect to use a definite article before a person's name unless it is a very informal or specific regional context.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `disse para ele` [pronoun] In Brazilian Portuguese, 'te' is the standard pronoun for 'you' (informal) in this context, or 'disse para ele' should be used.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `eu preocupo` -> `eu me preocupo` [pronoun] The verb 'preocupar' is reflexive in this context and requires the pronoun 'me'.
- Found (changes nothing): `muito com` -> `muito com` [word_order] The verb 'preocupar-se' requires the preposition 'com', but it is more natural to use 'muito' before the verb or 'com o meu trabalho' directly.

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

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei uma bicicleta nova no sábado` -> `` [content_mismatch] The answer does not address the question about working from home.

## gemma4:12b / few_shot

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent on the e: café.
- Found (matched): `as nove` -> `às nove` [spelling] The time of day requires a grave accent: às nove.
- Missed: `onibus`

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

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `tinha começado` -> `tinha começado` [verb_form] The past participle of 'começar' is 'começado', not 'começou'.
- Missed: `tinha começou`

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

## gemma4:12b / few_shot_six

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent mark: café.
- Found (matched): `as nove` -> `às nove` [article_preposition] Time expressions require a grave accent for the contraction: às nove.
- Missed: `onibus`

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

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `comeu` -> `` [verb_form] The verb must agree with 'a gente': comeu is correct, but 'a gente' is often treated as a singular subject, however, in this context, the learner likely confused the conjugation for 'nós' or 'eles'. Wait, 'a gente' actually takes the 3rd person singular. Let me re-evaluate. 'A gente' + 'comeu' is actually correct. No mistake here.

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `tinha começado` -> `tinha começado` [verb_form] The past participle of 'começar' is 'começado'.
- Missed: `tinha começou`

### a1-short-spelling-chocolate (erroneous, a1)

- Task: What do you like?
- Answer: Eu gosto de chocolat.
- Reference: `chocolat` -> chocolate [spelling]
- Flags: written_in_portuguese=false answers_the_question=true
- Missed: `chocolat`

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

## gemma4:12b / plain

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo.` -> `Eu comi pão com café da manhã.` [content_mismatch] The answer discusses a favorite soccer team instead of what you ate for breakfast.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eles estão comendo espaguete e bebendo vinho tinto.` [content_mismatch] The answer describes where the person lives instead of describing the meal in the scene.

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `tinha começado` -> `tinha começado` [verb_form] The past participle of 'começar' is 'começado', not 'começou'.
- Missed: `tinha começou`

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `não me esperasse` -> `não me esperasse` [verb_form] The subjunctive mood is not required here because the action is a direct command or request in the present/future context.

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

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A Camila` -> `Camila` [article_preposition] In Portuguese, you generally do not use an article before a person's name.
- Found (false positive): `marrons` -> `marrom` [spelling] The word 'marrom' is an invariable adjective and does not take a plural suffix.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para que eu passe` -> `para eu passar` [word_choice] The construction 'para que' usually requires a subjunctive verb, but for expressing purpose with a simple action, 'para' + infinitive is more natural.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `convidei-a` -> `a convidei` [word_order] In Brazilian Portuguese, it is much more natural to place the pronoun before the verb (clitic placement) in this context.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está chovendo muito e eu estou em casa.` -> `Meu nome é [Name].` [content_mismatch] The answer describes the weather and location instead of stating your name as requested.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei uma bicicleta nova no sábado.` -> `Eu acho que trabalhar de casa é muito bom.` [content_mismatch] The answer does not address the question about working from home.

## gemma4:e2b-it-qat / few_shot

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Tô` -> `Estou` [word_choice] The informal contraction 'tô' is not appropriate for a general A1 response; 'Estou' is better.

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=false

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

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sou em casa`

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `estou brasileiro`

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sou cansado`

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
- Found (matched): `chamo` -> `me chamo` [word_choice] The verb 'chamar' is not used in this context; 'me chamo' is the correct form.

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `trinta anos` -> `trinta anos` [word_order] The correct way to express age is 'trinta anos' or 'trinta anos de idade', but the structure is slightly awkward here.
- Missed: `chama`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No almoço` -> `No almoço eu como` [word_order] The correct structure is 'No almoço eu como' or 'Para almoço eu como'.
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective must be masculine to agree with 'camisa'.
- Missed: `vermelha camisa`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent on the i: café.
- Found (matched): `onibus` -> `ônibus` [spelling] The word needs an accent on the i: ônibus.
- Missed: `as nove`

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
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective must agree with the feminine noun 'amiga'.
- Missed: `curtas`

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `por passar`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `atendi`

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

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol e muito calor` -> `está chovendo e está frio` [content_mismatch] The description contradicts the facts that it is cold and rainy.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `comeu` -> `comemos` [verb_form] The verb should be in the past tense (pretérito perfeito) for a completed action.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eles comem espaguete e bebem vinho tinto.` [content_mismatch] The answer does not describe the meal.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=true answers_the_question=false

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Se eu ter`

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `tenho visitado`

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `tinha começou`

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

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A Camila` -> `Camila` [article_preposition] The article 'A' is unnecessary before the proper name Camila.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `convidei-a` -> `convidei-a` [verb_form] The past tense of 'convidar' is 'convidei', and the pronoun should be 'a' (object pronoun).

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei uma bicicleta nova no sábado.` -> `I think about working from home.` [content_mismatch] This sentence does not answer the question about working from home.

## gemma4:e2b-it-qat / few_shot_six

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `maiô` -> `maiô` [spelling] The correct spelling is 'maiô'.

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
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent: café.
- Missed: `manha`

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `estou brasileiro`

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sou cansado`

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
- Found (changes nothing): `trinta anos` -> `trinta anos` [word_order] The number should be written as 'trinta anos' or 'trinta anos de idade', but the structure is slightly awkward. For A1, 'trinta anos' is acceptable, but the structure is fine. No mandatory correction needed based on the rules.
- Missed: `chama`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Não` [word_order] The correct negation is 'Não'.
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective must agree with 'camisa': vermelha.
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `gosto de café muito`

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
- Found (matched): `cafe` -> `café` [spelling] The word needs an accent: café.
- Missed: `onibus`
- Missed: `as nove`

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
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective must agree with 'amiga' (feminine): engraçada.
- Missed: `curtas`

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `por passar`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `atendi`

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
- Found (changes nothing): `a comer` -> `a comer` [verb_form] The structure should be 'estou a comer' or 'estou comendo'.
- Missed: `a comer`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `de autocarro` -> `de autocarro` [word_order] The correct preposition is 'de autocarro' or 'de autocarro' is acceptable, but 'de autocarro' is more common for transport. However, the main issue is the lack of an article or a slightly awkward phrasing. A more natural phrasing would be 'de autocarro' or 'de autocarro'. Given the context, the structure is acceptable, but I will check for a more precise error. The structure is fine, but 'de autocarro' is the standard way. I will check for a more definite error. The sentence is grammatically acceptable for A2. No mandatory correction found based on the rules.
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
- Found (matched): `está fazendo sol e muito calor` -> `está chovendo e está frio.` [content_mismatch] The description contradicts the facts that it is cold and rainy.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `no banco` -> `no banco` [word_order] The correct preposition for turning is 'no' or 'no', but 'à esquerda' is used for turning.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `comeu` -> `comemos` [verb_form] The verb must agree with 'a gente' (we): comemos.

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No` -> `No` [word_order] The correct contraction is 'No' (not 'No' as a standalone word in this context).

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=true answers_the_question=false

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Se eu ter`

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `tenho visitado`

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `tinha começou`

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

### a1-word-order-article-noun (erroneous, a1)

- Task: Where do you live?
- Answer: Eu moro com meus pais em casa uma pequena.
- Reference: `casa uma pequena` -> uma casa pequena [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `uma` -> `uma` [agreement] The noun 'casa' is feminine, so the article must be feminine.
- Missed: `casa uma pequena`

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `de vestido amarelo` -> `de vestido amarelo` [word_order] The correct preposition is 'de' followed by the noun, not 'de' before the adjective.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `convidei-a` -> `convidei-a` [pronoun] The past tense of 'convidar' is 'convidei', and the pronoun should be 'a' (not 'a').

## gemma4:e2b-it-qat / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de` -> `está com` [word_choice] The correct preposition to describe what someone is wearing is 'vestindo' or 'com' in this context.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô` -> `Estou` [word_choice] The informal contraction 'tô' is not appropriate for a general A1 response.

### a1-control-daily-routine (control, a1)

- Task: What do you do every day?
- Answer: Todo dia eu acordo cedo, tomo café e vou para o trabalho.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tomo` -> `tomo` [verb_form] The verb 'tomar' should be conjugated as 'tomo' for the first person singular present tense.

### a1-control-order-meal (control, a1)

- Task: You are at a restaurant. Order your meal.
- Answer: Eu queria uma salada e um suco de laranja, por favor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu queria` -> `Eu gostaria` [word_choice] The verb 'queria' is too informal for ordering in a restaurant.

### a1-control-family (control, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos e uma irmã. Meu pai é médico e minha mãe é professora.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `uma irmã` -> `uma irmã` [agreement] The word 'irmã' is feminine, so the article should be feminine.

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `feijão` -> `feijão` [spelling] The correct spelling for the food is 'feijão' with a tilde.
- Missed: `fejão`

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha` -> `De manhã` [spelling] The correct way to say 'In the morning' is 'De manhã' with a cedilla.
- Missed: `cafe`

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `trinta anos` -> `trinta anos` [word_choice] The correct way to express age in Portuguese is to use the verb 'ter' followed by the number and the noun, without the article 'trinta'.
- Missed: `chama`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Eu` [word_order] The correct way to start this sentence is with a subject pronoun or a simple statement.
- Missed: `beans`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo.` -> `Eu tomei pão com manteiga para o café da manhã.` [content_mismatch] The answer does not answer the question about what the person ate for breakfast.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `estudei` -> `estudei` [verb_form] The verb 'estudar' is conjugated incorrectly for the first person singular past tense.
- Missed: `portugues`

### a2-spelling-agua (erroneous, a2)

- Task: What did you buy at the market?
- Answer: No mercado eu comprei maçãs, dois quilos de arroz e uma garrafa de agua.
- Reference: `agua` -> água [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No mercado` -> `No mercado` [word_order] The correct word is 'No mercado' or simply 'No mercado' depending on context, but the structure is slightly awkward here.
- Missed: `agua`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `cafe` -> `café` [spelling] The correct spelling is 'café' with an accent.
- Missed: `onibus`
- Missed: `as nove`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `eu fiquei` -> `eu estava` [word_choice] The verb 'ficar' is not the most natural choice for describing being tired and sleeping all day.
- Missed: `era cansado`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective must agree in gender with the noun 'amiga' (feminine).
- Missed: `curtas`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Ontem à noite` -> `Ontem à noite` [word_order] The correct way to say 'last night' is 'ontem à noite' or simply 'ontem'.
- Missed: `atendi`

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tomo` -> `tomo` [verb_form] The verb 'tomar' needs to be conjugated in the first person singular, which is 'tomo' (I take), but the context suggests a habitual action, so the simple present tense is correct here, however, the structure is fine.
- Missed: `lavo os dentes`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `de autocarro` -> `de autocarro` [word_choice] The correct preposition for public transport is 'de autocarro' or 'de elétrico', but 'de autocarro' is more common in European Portuguese. In Brazilian Portuguese, 'de ônibus' is more common, but 'de autocarro' is not strictly wrong.
- Missed: `autocarro`

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No ano passado` -> `No ano passado` [word_order] The correct way to say 'last year' is 'no ano passado' or 'ano passado' in this context.

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `No sábado` [article_preposition] The preposition 'No' is incorrect here; 'No' is used for negation, but here a simple time marker is needed.

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `mais novo` -> `mais novo do que` [word_order] The correct comparative form for 'novo' is 'mais novo' or 'mais novo que' when comparing two things, but the structure here is slightly awkward.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vire` -> `vire` [verb_form] The correct imperative form for 'vire' (to turn) in this context is 'vire' or 'vire' (depending on the desired formality, but 'vire' is acceptable here).

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A gente` -> `Nós` [pronoun] The correct informal subject pronoun is 'nós' or 'a gente' is used, but in this context, 'nós' is more standard for a simple past action.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> `do filme` [word_choice] The pronoun 'dele' refers to a masculine singular noun, which is correct here, but 'do filme' is more natural when referring to the film itself.

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `No meu tempo livre` [word_order] The correct way to start this sentence is 'No meu tempo livre' or simply 'No meu tempo livre' without the extra 'No' if it's meant to be a direct answer.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eles estão comendo espaguete e bebendo vinho tinto.` [content_mismatch] The answer does not describe the meal mentioned in the task.

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nunca` -> `eu nunca` [word_choice] The word 'nunca' means 'never', which doesn't fit the context of visiting a place.
- Missed: `tenho visitado`

### b1-agreement-passive (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1750 e foi restaurado em 2005.
- Reference: `restaurado` -> restaurada [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `restaurado`

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `à` -> `a` [word_order] The preposition 'a' should not have an accent here.

### b1-control-future-subjunctive (control, b1)

- Task: What will you do when you finish the course?
- Answer: Quando eu terminar o curso, vou procurar um emprego novo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `procurar` -> `procurar um emprego` [word_choice] The correct verb form for 'to look for' in this context is 'procurar' or 'buscar', but 'procurar' is acceptable. However, a more natural phrasing might be 'procurar um emprego' or 'buscar um emprego'.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The conditional tense should be used here.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós podermos` -> `para podermos` [word_order] The structure 'para nós podermos' is awkward; a simpler structure is better.

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a gente` -> `nós` [word_choice] In this context, 'a gente' is informal and less appropriate for a general opinion statement.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `gostaria de pedir` -> `Gostaria de solicitar` [word_choice] The phrasing is too informal for a request to a boss.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] The verb should be conjugated in the subjunctive mood after 'recomendo que' when expressing a recommendation.
- Missed: `assiste`
- Missed: `exige`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajará` [verb_form] The future simple tense is not the most natural choice here; the simple future or present tense is better.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morto de cansaço` -> `exausto` [word_choice] The correct idiomatic expression is 'exausto' or 'esgotado' for being very tired.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `no dia dez` -> `no dia dez` [word_order] The correct preposition for dates in this context is 'no dia' followed by the number.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu lhe disse` -> `Eu disse a ele` [pronoun] The pronoun 'lhe' is used incorrectly here; 'te' or 'a ele' would be more natural in this context.

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `eu` -> `eu` [pronoun] The subject pronoun should be 'eu' or omitted in this context, but 'eu' is acceptable here.
- Missed: `eu diverti`

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Ontem` -> `Hoje` [word_choice] The correct word for 'yesterday' in this context is 'Hoje' or a more specific time reference, but 'Ontem' is grammatically acceptable.
- Missed: `fiz um erro`

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `doente` -> `doente` [word_choice] The correct adjective for 'sick' is 'doente' or 'doente', but in this context, 'doente' is correct, however, the structure is fine.
- Missed: `é doente`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `eu era pequeno` -> `eu era pequeno` [agreement] The adjective 'pequeno' should agree with the subject 'eu' (I) in the past tense, which is 'era' (was).
- Missed: `sempre fui`

### a1-control-accents-present (control, a1)

- Task: What do you eat for breakfast?
- Answer: De manhã eu como pão com queijo e bebo café com leite.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `queijo` -> `queijo` [word_choice] The word for cheese is 'queijo' in Portuguese, but the context suggests a more common breakfast item, though 'queijo' is grammatically correct.

### a1-control-where-live (control, a1)

- Task: Where do you live?
- Answer: Moro no Brasil, numa cidade perto do mar.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `numa` -> `uma` [article_preposition] The correct article before a feminine noun starting with a consonant is 'uma' or 'uma' is used before 'cidade' in this context, but 'numa' is incorrect here.

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pra` -> `para` [word_choice] The correct contraction for 'para' is 'pra' in informal spoken Brazilian Portuguese, but in a written context, 'para' is more appropriate.

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Na minha bolsa` -> `Na minha bolsa` [article_preposition] The correct preposition to use with 'tem' is 'Na' or 'Em' depending on the context, but 'Na' is acceptable here, however, 'Na' is more common for possession in this context.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A` -> `Camila` [article_preposition] The article 'A' is unnecessary before the name Camila in this context.

### a2-control-preterite-sunday (control, a2)

- Task: What did you do on Sunday?
- Answer: No domingo eu acordei tarde, fiz o almoço e depois assisti a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No domingo` -> `No domingo eu` [word_order] The correct word order for time expressions in Portuguese is usually 'No domingo' or 'No domingo eu' depending on context, but 'No domingo eu' is more natural here.

### a2-control-imperfect-grandmother (control, a2)

- Task: Describe your grandmother's house.
- Answer: A casa da minha avó era pequena, mas tinha um jardim grande com muitas flores.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `avó` -> `avó` [word_choice] The correct word for grandmother is 'avó' (grandmother), but the context implies the house belongs to the grandmother, so 'casa da minha avó' is correct, but the description is fine.

### a2-control-reflexive-party (control, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu me diverti muito.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `eu` -> `nós` [pronoun] The subject pronoun should be 'nós' or implied, as the context suggests a general feeling, but 'eu' is grammatically acceptable in this context.

### a2-control-pet (control, a2)

- Task: Describe your pet.
- Answer: Eu tenho um cachorro. O meu cachorro se chama Bolt e ele adora correr.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `O meu cachorro` -> `Meu cachorro` [word_order] The possessive pronoun 'meu' is redundant here.

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `doente` -> `doente` [word_choice] The correct adjective for 'sick' is 'doente' or 'doente' is correct, but the context implies a state, and 'doente' is acceptable here.

### a2-control-family-meal (control, a2)

- Task: Describe the meal these people are eating.
- Answer: A família está comendo arroz, feijão e frango grelhado. Eles bebem suco de laranja.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `suco` -> `suco de laranja` [word_choice] The correct word for orange juice is 'suco de laranja' or just 'laranja' in this context, but 'suco' alone is less specific.

### a2-control-phone (control, a2)

- Task: Where is your phone?
- Answer: O meu celular está em cima do sofá.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `em cima` -> `em cima do` [word_choice] The correct preposition for 'on top of' in this context is 'em cima do' or simply 'no', but 'em cima do' is more natural here.

### a2-control-cinema-contraction (control, a2)

- Task: Where did you go on Saturday?
- Answer: No sábado eu fui ao cinema com meus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No sábado` -> `No sábado` [word_order] The correct way to say 'on Saturday' is 'no sábado' (without the 'o' before 'sábado' in this context, or simply 'no sábado' is acceptable, but 'no sábado' is the standard form).

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Estudo` -> `Eu estudo` [verb_form] The verb should be conjugated in the present tense with the subject 'eu'.

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pequeno` -> `criança` [word_choice] The correct adjective for 'small' in this context is 'criança' or 'menino/menina' (child), but 'pequeno' is acceptable if referring to a boy.

### b1-control-gostar-de (control, b1)

- Task: What do you like about the city?
- Answer: Eu gosto da cidade porque é tranquila e as pessoas são simpáticas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Eu gosto da cidade porque é tranquila` -> `Eu gosto da cidade porque é tranquila` [agreement] The adjective 'tranquila' should agree with the feminine noun 'cidade'.

### b1-control-weather-hot (control, b1)

- Task: Describe the weather today.
- Answer: Hoje faz muito calor e o céu está sem nuvens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está` -> `faz` [word_choice] The verb 'estar' is used correctly here, but 'faz' is more common for describing the weather condition like heat.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `às` -> `às` [article_preposition] The correct preposition for time in this context is 'à' (with the grave accent) when indicating a specific time.

### b2-control-news (control, b2)

- Task: Comment on the news.
- Answer: As notícias de hoje são preocupantes, mas acredito que a situação possa melhorar.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `As` -> `As` [word_order] The definite article 'As' is incorrect; it should be 'As' or 'As' depending on context, but here it's fine, however, the structure is slightly awkward for a general comment.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `convidei-a` -> `convidei a` [word_order] The past tense of 'convidar' is 'convidei' (I invited), and the pronoun should be 'a' (to her) without the hyphen.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está chovendo muito e eu estou em casa.` -> `Meu nome é...` [content_mismatch] The answer does not answer the question 'What is your name?'

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei uma bicicleta nova no sábado.` -> `Eu acho que trabalhar de casa é bom.` [content_mismatch] The answer does not address the question about working from home.

## gemma4:e2b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `maiô` -> `maiô` [word_choice] The word for swimsuit is 'maiô'.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Só um pouco cansado` -> `Estou um pouco cansado` [word_choice] The structure for expressing tiredness is incorrect.

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `moro` -> `Meu nome é Ana.` [content_mismatch] The verb 'morar' is used for permanent residence, but 'eu sou' or 'eu me chamo' is more appropriate for introducing oneself. However, the main issue is the content mismatch as the question asks for a name.

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
- Found (matched): `De manha` -> `De manhã` [word_order] The correct expression for 'in the morning' is 'de manhã'.
- Missed: `cafe`

### a1-agreement-trousers (erroneous, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: Marina veste uma camiseta azul e uma calça pretos.
- Reference: `pretos` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `pretos` -> `preta` [agreement] The adjective must agree in gender and number with 'calça' (feminine singular).

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `sou` -> `estou` [ser_estar] The verb 'ser' is incorrect here; 'estar' should be used for location.

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `eu sou cansado` -> `eu estou cansado` [word_choice] The subject is 'you' (implied by the context), and the adjective should agree with the subject. Assuming the speaker is male, 'cansado' is correct, but the structure is slightly awkward. However, the main issue is the missing article/pronoun for 'you'. Given the context, 'eu estou cansado' or simply stating the condition is better. The main error is the lack of a proper structure for this simple statement. The most direct correction is to use the correct form for 'I am'.

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `Eu sou do Canadá e moro Toronto.` -> `Eu sou do Canadá e moro em Toronto.` [not_brazilian] The sentence is in European Portuguese, and the structure is slightly unnatural for a native Brazilian speaker. However, the main issue is the language used. The structure 'Eu sou do...' is common, but the context implies a need for more natural phrasing in Brazilian Portuguese. The main error is the use of 'sou' instead of 'sou' or 'venho de', and the lack of proper phrasing for location. The most direct correction for a Brazilian context would be to use 'sou de' or 'venho de'.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `tenho vinte anos` -> `tenho vinte anos` [word_order] The correct way to say 'I am twenty years old' is 'I am twenty years old' or 'I am twenty years old'. The structure used is incorrect.
- Missed: `chamo`

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `A minha amiga` -> `Minha amiga` [article_preposition] The definite article 'a' is incorrect here. It should be 'Minha amiga'.
- Missed: `chama`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No almoço eu` -> `Eu almoço` [word_order] The word order is incorrect for a simple statement.
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `vermelha` -> `camisa vermelha` [word_choice] The color should be used as an adjective, not a noun. It should be 'vermelha' or 'vermelha' is fine, but the structure is wrong. The correct way is 'camisa vermelha'.

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `azul` -> `verde` [content_mismatch] The jacket is green, not blue.

### a2-spelling-agua (erroneous, a2)

- Task: What did you buy at the market?
- Answer: No mercado eu comprei maçãs, dois quilos de arroz e uma garrafa de agua.
- Reference: `agua` -> água [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maçãs` -> `comprei` [verb_form] The word for apples is 'maçãs' (plural), but it should be 'maçãs' in this context as it is a list of items bought. However, the issue is the lack of the correct article or structure for a simple past tense statement. The main issue is the lack of the correct past tense conjugation for 'comprar'.
- Missed: `agua`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tomei` -> `tomei` [verb_form] The verb 'tomar' should be in the past tense (pretérito perfeito).
- Missed: `cafe`
- Missed: `onibus`
- Missed: `as nove`

### a2-verb-form-imperfect (erroneous, a2)

- Task: Describe your childhood.
- Answer: Quando eu era criança, eu moro numa cidade pequena e brincava na rua.
- Reference: `moro` -> morava [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `moro` -> `morava` [verb_form] The verb should be in the imperfect tense to describe a past habit.

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
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective must agree in gender with 'amiga' (feminine).
- Missed: `curtas`

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `por passar`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ontem` -> `na noite passada` [word_choice] The correct word for 'last night' is 'ontem à noite' or simply 'na noite passada'. 'Ontem' means yesterday.
- Missed: `atendi`

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
- Found (matched): `fazendo sol` -> `está chovendo e está frio` [content_mismatch] The description contradicts the facts that it is cold and rainy.

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `eu viajei` -> `eu viajei` [verb_form] The verb 'viajar' in the past tense should be conjugated correctly for the first person singular.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Response: invalid or missing

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `comeu` -> `comeu` [verb_form] The verb should be in the past tense (pretérito perfeito) for a completed action.
- Found (false positive): `num` -> `no` [article_preposition] The correct preposition for 'in' when referring to a place is 'no'.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Response: invalid or missing

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Não` [article_preposition] The correct negative form is 'Não'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro` -> `Eles estão com` [word_choice] The sentence does not describe the meal. The verb tense is also incorrect for the context.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

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
- Found (matched): `nunca tenho visitado` -> `nunca visitei` [verb_form] The verb tense is incorrect for this context. It should be in the present perfect or simple past depending on the intended meaning. For 'never visited', the simple past is more appropriate here.

### b1-content-date (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1850 e restaurada em 2005.
- Reference: `1850` -> 1750 [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `1850` -> `1750` [content_mismatch] The year given in the facts is 1750.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `nós` -> `nós` [word_order] The pronoun should be 'nós' or 'nós' is acceptable, but 'nós' is more natural here. However, the main issue is the missing accent on 'podermos'.

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a gente` -> `as pessoas` [word_choice] The correct pronoun for 'people' in this context is 'as pessoas' or simply omitting it when using a general statement. 'A gente' is more common in spoken Brazilian Portuguese but 'as pessoas' is more appropriate here. However, the main issue is the lack of proper subject agreement for a general statement. In this context, 'as pessoas' or a rephrasing is better. Given the context, 'as pessoas' is the most natural fit.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `Prezado` -> `Prezado` [word_order] The formal address should be capitalized at the beginning of a sentence.

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

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `no dia dez` -> `no dia dez` [article_preposition] The correct phrasing for 'on the tenth' is 'no dia dez' or 'no dia dez', but the structure is slightly awkward for formal writing. A more natural phrasing would be better. However, sticking to the most direct error: the preposition is slightly off for this context.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `lhe` -> `a ele` [word_choice] The correct pronoun for 'to him/brother' in this context is 'a ele' or simply omitting it if the context is clear, but 'lhe' is often used incorrectly here. For a direct speech, 'ele' or 'a ele' is more natural. However, the main issue is the structure. A more natural phrasing would be 'Eu disse a ele que não me esperasse'.

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

### a1-pronoun-possessive-pet (erroneous, a1)

- Task: Describe your pet.
- Answer: Eu tenho um cachorro. Mim cachorro se chama Bolt.
- Reference: `Mim cachorro` -> Meu cachorro [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Mim cachorro`

### a1-word-choice-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu sou vinte anos velho.
- Reference: `sou vinte anos velho` -> tenho vinte anos [word_choice]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `vinte` -> `vinte` [word_choice] The correct word for twenty is 'vinte'.
- Missed: `sou vinte anos velho`

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a2-preposition-gostar-de (erroneous, a2)

- Task: What do you like about the city?
- Answer: Eu gosto a cidade porque é tranquila.
- Reference: `gosto a cidade` -> gosto da cidade [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `gosto a cidade`

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `doente` -> `doente` [word_choice] The correct adjective for a person is 'doente' or 'doente', but the context implies a state, and the structure is slightly awkward. However, the main issue is the lack of agreement if 'doente' is used as an adjective describing a state. A more natural phrasing would be needed, but sticking to the direct error check: 'doente' is fine here, but the sentence structure is slightly off for A2. Let's check for a more concrete error. The main issue is the lack of a clear grammatical error based on the rules provided. Re-evaluating: the sentence is grammatically acceptable for A2, but I will flag it for potential word choice if it implies a permanent state. Given the constraints, I will check for a clear error. There is no clear error based on the rules provided.
- Missed: `é doente`

### b1-verb-form-para-que (erroneous, b1)

- Task: Why do you study every day?
- Answer: Eu estudo todos os dias para que eu passo no exame.
- Reference: `eu passo` -> eu passe [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para que` -> `para` [word_choice] The correct conjunction to express purpose is 'para que' or simply 'para'. Here, 'para' is more natural.
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
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `O meu` -> `Meu` [article_preposition] The definite article 'o' is incorrect before a possessive pronoun like 'meu'.
- Missed: `telemóvel`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `uma irmã` -> `algumas irmãs` [agreement] The sentence implies the speaker has only one sister, but the context doesn't specify the number of siblings. However, the main issue is the lack of plural agreement if the intent was to say 'I have siblings'. Since the task is just 'Do you have siblings?', the answer is grammatically acceptable but contextually incomplete. Given the constraints, no clear grammatical error is flagged here.

### a1-control-accents-present (control, a1)

- Task: What do you eat for breakfast?
- Answer: De manhã eu como pão com queijo e bebo café com leite.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `De manhã` -> `De manhã` [word_order] The correct expression for 'in the morning' is 'de manhã' or 'de manhã' is acceptable, but the structure is slightly unnatural. A better phrasing would be 'De manhã' or 'Para o café da manhã'. However, the main issue is the lack of a definite article or a more natural phrasing. Given the context, 'De manhã' is acceptable, but the structure is weak. Let's check for a more concrete error. The main issue is the missing article or a more natural phrasing for the time. Let's re-evaluate. The structure is acceptable for A1, but let's check for a more obvious error. The structure is fine for A1. No mistake found based on the rules.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A Camila está de vestido amarelo e botas marrons.` -> `Camila está de vestido amarelo e botas marrons.` [word_order] The sentence structure is unnatural for describing clothes. It should use 'vestido' and 'botas' without the article 'de' in this context.

### a2-control-preterite-sunday (control, a2)

- Task: What did you do on Sunday?
- Answer: No domingo eu acordei tarde, fiz o almoço e depois assisti a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `fiz o almoço` -> `almocei` [word_choice] The verb 'fazer' is used incorrectly here; it should be 'almocei' or 'fiz o almoço' (depending on context, but the structure is awkward). For this simple past action, a simple past tense is better. However, the main issue is the structure. A more natural phrasing would be needed.

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=false

### a2-control-phone (control, a2)

- Task: Where is your phone?
- Answer: O meu celular está em cima do sofá.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `O meu` -> `Meu` [article_preposition] The definite article 'o' is incorrect before 'meu'.

### b1-control-weather-hot (control, b1)

- Task: Describe the weather today.
- Answer: Hoje faz muito calor e o céu está sem nuvens.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `o céu está sem nuvens` -> `o céu está sem nuvens` [word_choice] The correct way to say the sky is clear is 'o céu está sem nuvens' or 'não há nuvens'. The structure used is slightly unnatural.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `cheguei` -> `cheguei` [verb_form] The correct past tense conjugation for 'eu' is 'cheguei'.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está chovendo muito e eu estou em casa.` -> `Meu nome é...` [content_mismatch] This answer does not address the question about the name.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei uma bicicleta nova no sábado.` -> `(No correction needed as the content is irrelevant to the task, but the answer is factually incorrect for the task.)` [content_mismatch] This sentence does not answer the question about working from home.

### a2-english-coffee (not_portuguese, a2)

- Task: Do you like coffee?
- Answer: Yes, I like coffee a lot.
- Flags: written_in_portuguese=false answers_the_question=false

## gemma4:e2b / few_shot_six

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `maiô` -> `maiô` [spelling] The word for swimsuit is 'maiô'.

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Response: invalid or missing

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `feijão` -> `feijão` [spelling] The word needs an accent on the 'a': feijão.
- Missed: `fejão`

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha` -> `De manhã` [word_order] The correct expression is 'De manhã'.
- Missed: `cafe`

### a1-agreement-trousers (erroneous, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: Marina veste uma camiseta azul e uma calça pretos.
- Reference: `pretos` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `pretos` -> `preta` [agreement] The adjective must agree with 'calça' (feminine singular).

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `eu sou` -> `eu estou` [ser_estar] The correct form for 'I am' is 'eu estou'.

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `moro` -> `moro` [verb_form] The verb 'morar' is conjugated incorrectly for the first person singular. It should be 'moro'.
- Missed: `moro Toronto`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `vinte` -> `vinte` [spelling] The correct spelling for twenty is vinte.
- Missed: `chamo`

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `trinta` -> `trinta` [word_choice] The correct way to say thirty is 'trinta'.
- Missed: `chama`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Eu` [word_choice] The correct word for 'I' is eu.
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `vermelha` -> `camisa vermelha` [word_choice] The color should be used as an adjective, not a noun. It should be 'vermelha' or 'vermelha' is fine, but the structure is wrong. The correct way is 'camisa vermelha'.

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `azul` -> `verde` [content_mismatch] The jacket is green, not blue.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `e à tarde` -> `Ontem eu trabalhei de manhã e à tarde eu estudei portugues.` [word_order] The correct preposition for time is 'e à tarde' or simply 'e à tarde', but the structure is slightly awkward. 'e à tarde' is acceptable, but the main issue is the lack of proper tense agreement in the context of a simple past narrative. However, focusing on the most obvious grammatical error for this level: the use of 'e' instead of 'e' for the second part of the sentence is not a clear error here. Let's re-evaluate. The structure is acceptable for A2, but the use of 'e' for the second part is common. Let's check for a clearer error. There is no clear grammatical error based on the rules provided. Returning empty list based on strict interpretation. Wait, the structure is fine. Let's check for 'not_brazilian'. 'Ontem' is fine.
- Missed: `portugues`

### a2-spelling-agua (erroneous, a2)

- Task: What did you buy at the market?
- Answer: No mercado eu comprei maçãs, dois quilos de arroz e uma garrafa de agua.
- Reference: `agua` -> água [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maçãs` -> `uma maçã` [article_preposition] The word for apples is 'maçãs' (plural), but the context implies a singular item was bought, or it should be 'uma maçã' if referring to one. However, the main issue is the lack of article for the first item. Given the context, the plural is acceptable but the structure is slightly off for a simple list. The most direct error is the missing article for the first item.
- Missed: `agua`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tomei` -> `tomei` [verb_form] The verb should be in the past tense (pretérito perfeito) for a completed action. It should be 'tomei'.
- Missed: `cafe`
- Missed: `onibus`
- Missed: `as nove`

### a2-verb-form-imperfect (erroneous, a2)

- Task: Describe your childhood.
- Answer: Quando eu era criança, eu moro numa cidade pequena e brincava na rua.
- Reference: `moro` -> morava [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `moro` -> `morava` [verb_form] The verb should be in the imperfect tense for a past habit.

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `estava` -> `estive` [ser_estar] The subject is 'eu', so the verb should be 'estive'.
- Missed: `era cansado`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective must agree with 'amiga' (feminine).
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
- Found (changes nothing): `tomo` -> `tomo` [verb_form] The verb must be conjugated for 'eu': tomo.
- Missed: `lavo os dentes`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para o trabalho` -> `de autocarro` [word_choice] The correct preposition is 'para o trabalho' or 'de autocarro'. 'para' is acceptable but 'de' is more natural here. However, the main issue is the missing article before 'trabalho' if it's general, but the main error is the missing article before 'trabalho' if it's a specific place. Given the context, 'para o trabalho' is acceptable, but let's check for a more obvious error. The structure is fine, but 'de autocarro' is more idiomatic. Let's re-evaluate. The structure is acceptable for A2. No mistake found based on strict rules.
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
- Found (matched): `fazendo sol e muito calor` -> `está chovendo e está frio` [content_mismatch] The description contradicts the facts that it is cold and rainy. Also, the structure is unnatural.

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No ano passado` -> `ano passado` [article_preposition] The correct way to say 'last year' is 'no ano passado' or 'ano passado'. 'No' is unnecessary here.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `em frente` -> `em frente` [word_choice] The correct expression is 'em frente' or 'reto', but 'em frente' is less natural here. 'Em frente' is acceptable but 'em frente' is more common. The main issue is the lack of a definite article or a more natural phrasing. However, sticking to the rule, 'em frente' is not strictly wrong, but the phrasing is slightly awkward. Let's check for a more direct error. The instruction is to report only real mistakes. The instruction is 'Go straight ahead'. 'Siga em frente' is acceptable. Let's re-evaluate. The instruction is 'Go straight ahead'. 'Siga em frente' is fine. Let's check the next part. 'vire à esquerda no banco'. This is fine. 'e a estação fica à direita'. This is fine. No clear grammatical error based on the rules provided. Re-checking the task: 'Go straight ahead'. 'Siga em frente' is okay. Let's assume no mistake for this one based on the strict rules. Wait, the instruction is 'Go straight ahead'. 'Siga em frente' is fine. Let's check the next part. 'turn left at the bank'. 'vire à esquerda no banco'. This is fine. Let's check the final part. 'the station is on the right'. 'a estação fica à direita'. This is fine. No mistakes found based on the rules provided. Returning empty list. Wait, I must report *real* mistakes. Let's assume 'em frente' is slightly awkward but not a mistake. I will stick to the initial assessment that there are no clear grammatical errors.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `comeu` -> `comeu` [verb_form] The verb must be in the past tense (pretérito perfeito) for 'a gente'.
- Found (false positive): `num` -> `no` [word_choice] The correct preposition for 'in' in this context is 'no'.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `dele` -> `dele` [word_choice] The correct pronoun for 'dele' in this context is 'dele' or 'dela', but 'dele' is acceptable here, however, the common usage is 'dele' or 'dela' depending on context. The main issue is the missing accent on 'gostei' if it was meant to be a specific structure, but here it's fine. Let's check for a more definite error. The sentence is grammatically correct for A2 level, but 'dele' is often replaced by 'dele' or 'dela' depending on the nuance. Given the constraints, I will check for a common error. The sentence is acceptable. No mistake found.

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Não` [article_preposition] The correct negative is 'Não'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro` -> `Eles estão com` [verb_form] The verb tense is incorrect for describing what people are eating. It should be present tense or a description of the meal.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

### b1-verb-form-subjunctive-present (erroneous, b1)

- Task: What do you hope for the future?
- Answer: Espero que meus filhos são felizes e que eles tenham uma boa educação.
- Reference: `são` -> sejam [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `são` -> `são` [ser_estar] The verb 'ser' should be used here, not 'estar'.
- Missed: `são`

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Se eu ter`

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `nunca tenho visitado` -> `nunca visitei` [verb_form] The structure for 'never' in this context is usually 'nunca visitei' or 'nunca tenho visitado'. The tense usage is slightly awkward for this context.

### b1-content-date (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1850 e restaurada em 2005.
- Reference: `1850` -> 1750 [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `em 1850` -> `em 1750` [content_mismatch] The year given in the facts is 1750, not 1850.

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `vir` -> `venha` [verb_form] The verb should be in the subjunctive after 'Espero que'.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós podermos` -> `para que nós possamos` [word_choice] The structure should be 'para que nós possamos' or simpler. 'podermos' is incorrect here.

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a gente` -> `as pessoas` [word_choice] The correct pronoun for 'people' in this context is 'as pessoas' or simply omitting it if the context is general. 'A gente' is very informal and often replaced by 'as pessoas' or 'as pessoas'.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Prezado` -> `Prezado Senhor` [word_choice] The correct formal address is 'Prezado Senhor'.

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

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `do` -> `deveria` [word_choice] The verb 'deveria' is correct for the conditional tense here. 'deveria' is the correct form.

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
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vinte` -> `vinte` [word_choice] The correct word for twenty is vinte.
- Missed: `sou vinte anos velho`

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Ontem` -> `Ontem` [word_order] The correct way to say 'yesterday' is 'ontem'.
- Missed: `fiz um erro`

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `doente` -> `doente` [verb_form] The adjective should be 'doente' or 'doente' is correct, but the context implies a state, and the structure is slightly awkward. However, 'doente' is grammatically correct for a description of a sick person. No mistake found based on the rules.
- Missed: `é doente`

### b1-verb-form-para-que (erroneous, b1)

- Task: Why do you study every day?
- Answer: Eu estudo todos os dias para que eu passo no exame.
- Reference: `eu passo` -> eu passe [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para que` -> `para` [word_choice] The correct conjunction is 'para que' or simply 'para'. Here, 'para' is sufficient.
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
- Found (changes nothing): `em cima` -> `em cima` [word_choice] The correct expression for 'on top of' is 'em cima'.
- Missed: `telemóvel`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `uma irmã` -> `uma irmã` [agreement] The sentence structure implies a general statement, but the context is about having siblings in general. If the intent is to state the number, it's fine, but the context is missing. However, based on the rules, this is not a clear grammatical error in this context. Re-evaluating: the sentence is grammatically correct for A1. No mistake reported.

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Na minha bolsa` -> `Na minha bolsa` [agreement] The correct structure is 'Na minha bolsa há' or 'Na minha bolsa tem'. The structure used is acceptable but 'tem' is more common for possession in this context. However, the main issue is the missing article before 'minha' in this context, or the structure itself. Let's focus on the common structure. The structure is acceptable, but the article usage is slightly off for a simple statement. A more natural phrasing would omit 'minha' or use a different structure. Given the constraints, we will check for a more direct grammatical issue. The structure is fine, but we will flag the article usage as potentially non-standard for A1. Let's re-evaluate. The sentence is grammatically acceptable for A1. No mistake found based on strict rules. Re-checking for a potential error: The structure is fine. No mistake found.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de` -> `com` [word_choice] The correct preposition for wearing is 'com'.

### a2-control-preterite-sunday (control, a2)

- Task: What did you do on Sunday?
- Answer: No domingo eu acordei tarde, fiz o almoço e depois assisti a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `assisti a um filme` -> `um filme` [word_choice] The verb 'assistir' is used with the preposition 'a' when referring to watching something, and the article 'um' is incorrect here. It should be 'um filme'.

### a2-control-phone (control, a2)

- Task: Where is your phone?
- Answer: O meu celular está em cima do sofá.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `em cima` -> `o meu celular` [article_preposition] The correct expression for location is 'em cima' or 'no topo'. 'em cima' is acceptable but 'no topo' is more natural here. However, the main issue is the missing article before 'meu'.

### b1-control-weather-hot (control, b1)

- Task: Describe the weather today.
- Answer: Hoje faz muito calor e o céu está sem nuvens.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `sem nuvens` -> `limpo` [word_choice] The correct expression for 'cloudless' is 'sem nuvens' or 'limpo'.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `cheguei` -> `cheguei` [verb_form] The correct past tense conjugation for 'eu' is 'cheguei'.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está chovendo muito e eu estou em casa.` -> `Meu nome é...` [content_mismatch] This answer does not address the question 'What is your name?'.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `comprei` -> `comprei` [verb_form] The verb tense is incorrect for a general statement about a past action. It should be in the simple past or present perfect depending on context, but here it sounds like a statement of fact. Assuming a simple past action is intended.

## gemma4:e2b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `A Marina está de` -> `Marina está usando` [word_choice] The correct way to describe what someone is wearing is by saying they are wearing something.
- Found (changes nothing): `maiô` -> `maiô` [word_choice] The correct word for swimsuit is 'maiô' or 'biquíni', but the structure is wrong.
- Found (changes nothing): `sandálias` -> `sandálias` [spelling] The correct spelling for sandals is 'sandálias'.

### a1-control-coffee (control, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `eu` -> `` [pronoun] The subject pronoun should be omitted in this context.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Tô` -> `Estou` [word_choice] The informal contraction 'tô' is too informal for a general response.
- Found (changes nothing): `você` -> `você` [not_brazilian] The correct form for 'you' in Brazilian Portuguese is 'você' or 'você' is acceptable in informal speech, but 'você' is often replaced by 'você' or 'você' is used in this context.

### a1-control-daily-routine (control, a1)

- Task: What do you do every day?
- Answer: Todo dia eu acordo cedo, tomo café e vou para o trabalho.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Todo dia` -> `Todo dia` [word_order] The correct way to say 'Every day' is 'Todo dia' or 'Todos os dias'.

### a1-control-order-meal (control, a1)

- Task: You are at a restaurant. Order your meal.
- Answer: Eu queria uma salada e um suco de laranja, por favor.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Eu queria` -> `Eu gostaria` [word_choice] Using 'eu queria' is too informal for ordering in a restaurant.

### a1-control-family (control, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos e uma irmã. Meu pai é médico e minha mãe é professora.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Eu tenho dois irmãos e uma irmã.` -> `Eu tenho dois irmãos e uma irmã.` [content_mismatch] The sentence structure is correct but the context implies a description of family members, not just a count.

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Eu me chamo` -> `Meu nome é` [word_choice] The correct way to say 'My name is' is 'Meu nome é' or 'Eu sou', not 'I call myself'.

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `é` -> `é` [verb_form] The verb 'ser' is used for permanent characteristics, while 'estar' is used for temporary states or conditions, and 'é' is used incorrectly here.
- Missed: `fejão`

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `De manha` -> `De manhã` [word_order] The correct way to say 'In the morning' is 'De manhã'.
- Missed: `cafe`

### a1-agreement-trousers (erroneous, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: Marina veste uma camiseta azul e uma calça pretos.
- Reference: `pretos` -> preta [agreement]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `veste` -> `veste` [verb_form] The verb 'vestir' is used for clothing, but the correct form for 'Marina' (third person singular) is 'veste'.
- Found (matched): `pretos` -> `preta` [agreement] The adjective 'preto' must agree in gender with 'calça', which is feminine.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `em uma escritório` -> `em um escritório` [article_preposition] The correct preposition for a place is 'em um escritório' and 'escritório' is masculine.

### a1-agreement-plural-adjective (erroneous, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos. Eles são muito simpático.
- Reference: `simpático` -> simpáticos [agreement]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `simpático` -> `simpáticos` [agreement] The adjective needs to agree in gender with the noun it describes.

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `eu sou` -> `estou` [ser_estar] The correct verb for location is 'estar' instead of 'ser'.

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `Eu estou` -> `Eu sou` [verb_form] The correct way to say 'I am' in this context is 'Eu sou' or 'Eu sou'.

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `eu sou` -> `eu estou` [verb_form] The correct way to express this feeling is using the adjective 'cansado' or the verb 'estar' with the adjective.
- Found (false positive): `com fome` -> `e com fome` [word_choice] The correct way to express hunger is using the noun 'fome' or the phrase 'com fome' is unnatural.

### a1-article-contraction-em-o (erroneous, a1)

- Task: Where do you live?
- Answer: Eu moro em o Brasil, em São Paulo.
- Reference: `em o` -> no [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `em o` -> `em` [article_preposition] The preposition 'em' is used without the article 'o' in this context.

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Eu sou do Canadá` -> `Eu sou do Canadá` [content_mismatch] The correct way to say where you are from is 'Eu sou do Canadá' or 'Eu sou do Canadá' (depending on context, but the structure is fine, the issue is the next part).
- Missed: `moro Toronto`

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `Eu jantar` -> `Eu janto` [verb_form] The verb 'jantar' needs a conjugated form to indicate an action.
- Found (false positive): `assisto televisão` -> `eu assisto televisão` [verb_form] The verb 'assistir' needs the correct conjugation for the first person singular.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `tenho vinte anos` -> `eu tenho vinte anos` [word_choice] The correct way to say you are a certain age is to use 'tenho' followed by the number, or to use 'eu tenho' for possession.
- Missed: `chamo`

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `A minha amiga` -> `Minha amiga` [article_preposition] The definite article 'a' is usually omitted when referring to a specific friend in this context.
- Missed: `chama`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `No almoço eu` -> `Eu almoço` [word_order] The word order is incorrect for a simple statement.
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `uma vermelha camisa` -> `uma camisa vermelha` [word_choice] The adjective 'vermelha' is not used to describe a shirt; you should use a color.
- Found (false positive): `uma calça preta` -> `calças pretas` [word_choice] The noun 'calça' is usually used for trousers, but the structure is slightly unnatural.

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `muito` -> `Sim, eu gosto de café.` [word_choice] The adverb 'muito' is used incorrectly here; an adjective or a different structure is needed.

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `azul` -> `verde` [word_choice] The jacket is green, not blue.

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Eu gosto de futebol` -> `Eu comi pão com manteiga.` [content_mismatch] This sentence does not answer the question about what was eaten for breakfast.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Ontem` -> `Ontem eu trabalhei de manhã e à tarde eu estudei português.` [word_order] The correct way to say 'yesterday' in this context is 'Ontem' or 'No dia anterior', but 'Ontem' is acceptable, however the structure is slightly unnatural for a simple past action.
- Missed: `portugues`

### a2-spelling-agua (erroneous, a2)

- Task: What did you buy at the market?
- Answer: No mercado eu comprei maçãs, dois quilos de arroz e uma garrafa de agua.
- Reference: `agua` -> água [spelling]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `No mercado` -> `No mercado` [article_preposition] The correct way to say 'at the market' is 'no mercado' without the article 'o' in this context.
- Missed: `agua`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `tomei` -> `bebi` [verb_form] The correct verb for drinking coffee in this context is 'beber' in the past tense.
- Found (matched): `de onibus` -> `de ônibus` [article_preposition] The correct preposition for transportation is 'de' or 'em', but 'de' is more natural here.
- Missed: `cafe`
- Missed: `as nove`

### a2-verb-form-irregular-fazer (erroneous, a2)

- Task: What did you do yesterday evening?
- Answer: Ontem à noite eu fazi o jantar e depois fui dormir cedo.
- Reference: `fazi` -> fiz [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `eu fazi` -> `eu fiz` [verb_form] The past tense for 'eu' in the first person singular is 'fiz'.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `eu visitei` -> `eu visitei` [verb_form] The verb 'visitei' is in the simple past tense, but the context implies a completed action in the past, and the structure is slightly awkward for a simple past narrative.
- Found (matched): `eu volti` -> `eu voltei` [verb_form] The verb 'volti' is incorrect; the correct past tense for 'voltar' (to return) is 'voltei'.

### a2-verb-form-imperfect (erroneous, a2)

- Task: Describe your childhood.
- Answer: Quando eu era criança, eu moro numa cidade pequena e brincava na rua.
- Reference: `moro` -> morava [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `eu moro` -> `eu morava` [verb_form] The verb 'morar' should be in the past tense for a past event.

### a2-verb-form-subject-agreement (erroneous, a2)

- Task: What did your family do on Sunday?
- Answer: No domingo minha família almoçaram na casa da minha avó.
- Reference: `almoçaram` -> almoçou [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `minha família almoçaram` -> `minha família almoçou` [verb_form] The verb 'almoçar' should be conjugated in the past tense for a singular subject or a plural subject with correct agreement.

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Ontem estava muito frio` -> `Foi muito frio` [verb_form] The correct way to say this in the past is using the simple past tense.
- Found (changes nothing): `eu fiquei em casa` -> `eu fiquei em casa` [word_choice] The verb 'fiquei' is correct, but the context implies a simple statement of being, not a specific action of staying.
- Missed: `era cansado`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective should agree in gender with 'amiga' (feminine).
- Missed: `curtas`

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Eu estudo muito` -> `Eu estudo muito` [word_order] The adverb 'muito' should be placed after the verb in this structure.
- Missed: `por passar`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Ontem à noite` -> `Ontem à noite` [word_order] The correct way to say 'last night' is 'last night' or 'yesterday night'.
- Missed: `atendi`

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `De manhã` -> `De manhã` [word_order] The correct way to say 'In the morning' is 'De manhã' or 'De manhã' is acceptable, but 'De manhã' is more natural in this context.
- Missed: `lavo os dentes`

### a2-pronoun-reciprocal (erroneous, a2)

- Task: Describe your relationship with your neighbors.
- Answer: Eu me dou bem com meus vizinhos e sempre nós ajudamos quando precisamos.
- Reference: `nós ajudamos` -> nos ajudamos [pronoun]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Eu me dou bem` -> `Eu tenho um bom relacionamento` [word_choice] The correct way to express this is 'I get along well' or 'I have a good relationship' in this context.
- Found (matched): `sempre nós ajudamos` -> `e sempre ajudamos` [pronoun] The pronoun 'nós' is unnecessary here; the subject is implied or the sentence structure needs adjustment.

### a2-not-brazilian-gerund (erroneous, a2)

- Task: What are you doing now?
- Answer: Estou a comer pão com queijo na cozinha.
- Reference: `a comer` -> comendo [not_brazilian]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Estou a comer` -> `Estou a comer` [verb_form] The correct structure for the present continuous tense in Portuguese is 'estar' + gerund.
- Missed: `a comer`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Eu vou para o trabalho` -> `Eu vou para o trabalho` [article_preposition] The correct way to express this is 'Eu vou para o trabalho' or 'Eu vou para o trabalho' without the article 'o' in this context.
- Missed: `autocarro`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `pequeno-almoço` -> `café da manhã` [word_choice] The correct term for breakfast is 'café da manhã'.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `está fazendo sol` -> `está chovendo e está frio` [content_mismatch] The weather is raining and cold, not sunny and hot.
- Found (duplicate): `e muito calor` -> `e está frio` [content_mismatch] The weather is raining and cold, not hot.

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `No ano passado` -> `No ano passado` [word_order] The correct way to say 'last year' is 'no ano passado' or 'no ano passado' is acceptable, but the phrasing is slightly unnatural.
- Found (changes nothing): `Cristo Redentor` -> `Cristo Redentor` [spelling] The correct name of the monument is Christ the Redeemer.

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `No sábado` -> `No sábado` [article_preposition] The correct way to say 'On Saturday' is 'No sábado' or 'No próximo sábado'.

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `O meu celular` -> `Meu celular` [agreement] The possessive pronoun 'meu' is incorrect when used with the definite article 'o' in this context.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `em frente` -> `reto` [word_choice] The correct way to say 'straight ahead' is 'reto' or 'em frente' is acceptable but less natural in this context.
- Found (changes nothing): `no banco` -> `no banco` [word_choice] The correct preposition for turning is 'no banco' or 'na esquina' depending on context, but 'no banco' is slightly awkward.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `A gente` -> `Nós` [pronoun] The correct way to say 'we' in this context is 'nós' or simply omitting the pronoun.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `dele` -> `dele` [word_choice] The correct pronoun to use after 'gostei' when referring to a film is 'dele' or 'disso'.

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `No` -> `No meu tempo livre` [article_preposition] The correct way to say 'In my free time' is 'No meu tempo livre' or simply 'No meu tempo livre' without 'No' at the start.
- Found (false positive): `com o meu cachorro` -> `passear com o cachorro` [article_preposition] The correct preposition to use with 'passear' is 'com' or 'passear' alone, and the article usage is incorrect.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `They are eating spaghetti and drinking red wine.` [content_mismatch] This sentence does not describe the meal.

### b1-verb-form-subjunctive-present (erroneous, b1)

- Task: What do you hope for the future?
- Answer: Espero que meus filhos são felizes e que eles tenham uma boa educação.
- Reference: `são` -> sejam [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `meus filhos são felizes` -> `meus filhos estejam felizes` [ser_estar] The verb 'ser' is used for permanent states, and 'estar' is used for temporary states like happiness.

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `Se eu ter tempo` -> `Se eu tiver tempo` [verb_form] The correct structure for this conditional is 'Se eu tiver tempo' in the present subjunctive.

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `Se eu ganhava na loteria` -> `Se eu ganhasse na loteria` [verb_form] The conditional tense is incorrect for this hypothetical situation.

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Eu moro` -> `Eu moro` [verb_form] The correct verb for expressing duration since a point in the past is 'Eu moro' or 'Eu moro' (present tense is used incorrectly for a past action/duration).
- Found (false positive): `desde 2019` -> `Eu moro desde 2019` [verb_form] The correct structure for expressing duration since a point in the past uses the present perfect tense.
- Found (matched): `nunca tenho visitado` -> `nunca visitei` [verb_form] The structure for 'never' in this context requires the present perfect tense.

### b1-agreement-passive (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1750 e foi restaurado em 2005.
- Reference: `restaurado` -> restaurada [agreement]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `foi` -> `foi` [verb_form] The subject 'a catedral' is feminine, so the verb should be 'foi' (singular past tense) or 'foi' (singular past tense) for the passive voice, but the agreement is wrong.
- Missed: `restaurado`

### b1-content-date (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1850 e restaurada em 2005.
- Reference: `1850` -> 1750 [content_mismatch]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `1850` -> `1750` [content_mismatch] The building year given in the facts is 1750.

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `vir` -> `venha` [verb_form] The verb 'vir' is incorrect in this context; 'venha' is needed for a polite invitation.

### b1-control-future-subjunctive (control, b1)

- Task: What will you do when you finish the course?
- Answer: Quando eu terminar o curso, vou procurar um emprego novo.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Quando eu terminar o curso, vou procurar um emprego novo.` -> `Quando eu terminar o curso, vou procurar um emprego.` [word_order] The sentence structure is slightly unnatural for this context.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Se eu ganhasse na loteria` -> `Se eu ganhasse na loteria` [verb_form] The conditional tense is incorrect for this type of hypothetical situation.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `para nós podermos` -> `para podermos` [word_order] The correct structure for expressing purpose with 'para' followed by an infinitive is needed.

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `a gente` -> `as pessoas` [word_choice] This is a colloquial expression and 'as pessoas' or 'as pessoas' is more appropriate here.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Prezado` -> `Caro` [word_choice] This is too formal for a typical email to a boss.
- Found (changes nothing): `gostaria de pedir` -> `gostaria de pedir` [word_choice] The phrasing is too indirect for a direct request.
- Found (changes nothing): `um dia de folga` -> `um dia de folga` [word_choice] The phrasing is slightly unnatural for requesting time off.
- Found (changes nothing): `Agradeço desde já.` -> `Agradeço desde já.` [word_choice] The phrasing is grammatically incorrect and unnatural for this context.

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `já tinha começou` -> `já tinha começado` [verb_form] The past perfect tense is used incorrectly here.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `todos os dias` -> `todos os dias` [word_choice] The correct phrasing for this general advice is 'every day' or 'daily', not 'all days'.
- Missed: `estuda`
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Venho por meio desta reclamar` -> `Eu gostaria de reclamar` [word_choice] This phrasing is too formal and unnatural for a simple complaint.
- Found (changes nothing): `no dia dez` -> `no dia dez` [article_preposition] The preposition 'no' is incorrect when referring to a specific date.
- Missed: `sobre do`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Ana disse que viajaria` -> `Ana disse que viajará` [verb_form] The future tense in Portuguese is usually expressed with 'vai' or 'irá' for this context.
- Found (false positive): `e que já tinha comprado as passagens.` -> `e que já comprou as passagens.` [verb_form] The past perfect tense is not the most natural translation here.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `morto de cansaço` -> `exaurido` [word_choice] This phrase is too literal and not a common idiomatic expression for describing a stressful day.
- Found (false positive): `cheguei em casa` -> `cheguei` [verb_form] The correct verb for arriving is 'chegar' in the past tense.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Response: invalid or missing

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Eu lhe disse` -> `Eu disse` [word_choice] The correct way to say this in this context is 'Eu disse' or 'Eu lhe disse' is too formal/old-fashioned.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `eu preocupo` -> `eu me preocupo` [verb_form] The verb 'preocupar' should be conjugated in the present tense with the correct subject pronoun.

### a2-pronoun-reflexive-party (erroneous, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu diverti muito.
- Reference: `eu diverti` -> eu me diverti [pronoun]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `eu diverti` -> `eu me diverti` [verb_form] The verb 'divertir' is incorrect in this context; the correct form is 'me diverti' or 'eu me diverti'.

### a1-pronoun-possessive-pet (erroneous, a1)

- Task: Describe your pet.
- Answer: Eu tenho um cachorro. Mim cachorro se chama Bolt.
- Reference: `Mim cachorro` -> Meu cachorro [pronoun]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `Mim` -> `Eu` [pronoun] This is an incorrect pronoun; the correct one is 'me' or it should be omitted.

### b2-pronoun-clitic-gender (erroneous, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, eu falei com a Ana e convidei-o para a festa.
- Reference: `convidei-o` -> convidei-a [pronoun]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `convidei-o` -> `convidei-la` [pronoun] The pronoun should be attached to the verb in the past tense.

### a1-short-agreement-house (erroneous, a1)

- Task: Describe your house.
- Answer: Minha casa é grande e bonito.
- Reference: `bonito` -> bonita [agreement]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `grande e bonito` -> `grande e bonita` [agreement] Adjectives must agree in gender and number with the noun they describe.

### a1-short-agreement-books (erroneous, a1)

- Task: What is in your bag?
- Answer: Na minha bolsa tem um livros.
- Reference: `um livros` -> um livro | uns livros [agreement]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `Na minha bolsa tem um livros` -> `Na minha bolsa tem um livro` [agreement] The correct article for a singular masculine noun is 'um' and the noun should be singular.

### a1-word-choice-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu sou vinte anos velho.
- Reference: `sou vinte anos velho` -> tenho vinte anos [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vinte` -> `vinte` [spelling] The correct word for twenty is 'vinte'.
- Found (matched): `velho` -> `eu tenho` [agreement] The adjective for old is 'velho', but it should be used with the correct article or structure for age.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Ontem` -> `ontem` [spelling] The correct word for 'yesterday' in this context is 'ontem'.
- Missed: `fiz um erro`

### a1-article-contraction-a-o (erroneous, a1)

- Task: Where did you go on Saturday?
- Answer: No sábado eu fui a o cinema com meus amigos.
- Reference: `a o` -> ao [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `a o` -> `o` [article_preposition] The article 'a' is incorrect before the definite article 'o'.

### a2-article-contraction-de-o (erroneous, a2)

- Task: Whose is the car?
- Answer: O carro é de o meu pai.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `de o` -> `do` [article_preposition] The preposition 'de' is used without the article 'o' in this context.

### a2-preposition-gostar-de (erroneous, a2)

- Task: What do you like about the city?
- Answer: Eu gosto a cidade porque é tranquila.
- Reference: `gosto a cidade` -> gosto da cidade [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `Eu gosto a cidade` -> `Eu gosto da cidade` [word_choice] The verb 'gostar' is not the correct way to express liking something in this context.

### a2-serestar-sick (erroneous, a2)

- Task: How is your brother?
- Answer: Meu irmão é doente e não vai trabalhar hoje.
- Reference: `é doente` -> está doente [ser_estar]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `doente` -> `doente` [word_choice] The correct adjective for a person is 'doente' or 'doente' in this context, but the structure is slightly unnatural for a simple statement about someone's health.
- Missed: `é doente`

### b1-verb-form-para-que (erroneous, b1)

- Task: Why do you study every day?
- Answer: Eu estudo todos os dias para que eu passo no exame.
- Reference: `eu passo` -> eu passe [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `todos os dias` -> `todos os dias` [word_order] The correct expression for 'every day' is 'todos os dias' or simply 'todos os dias' in this context, but the phrasing is slightly unnatural.
- Missed: `eu passo`

### b1-verb-form-habitual-past (erroneous, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre fui à praia com meus avós.
- Reference: `sempre fui` -> sempre ia [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `eu sempre fui` -> `eu sempre estava` [verb_form] The verb 'ser' is used incorrectly here; 'estar' is needed to describe a state or condition.

### b2-agreement-news (erroneous, b2)

- Task: Comment on the news.
- Answer: As notícias de hoje são preocupante.
- Reference: `preocupante` -> preocupantes [agreement]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `As notícias de hoje são preocupante` -> `As notícias de hoje são preocupantes.` [agreement] The adjective 'preocupante' should agree in gender with 'notícias', which is feminine.

### a2-not-brazilian-phone (erroneous, a2)

- Task: Where is your phone?
- Answer: O meu telemóvel está em cima do sofá.
- Reference: `telemóvel` -> celular [not_brazilian]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `O meu` -> `Meu` [agreement] The possessive pronoun 'meu' is incorrect when used with the definite article 'o' in this context.
- Missed: `telemóvel`

### a1-word-order-article-noun (erroneous, a1)

- Task: Where do you live?
- Answer: Eu moro com meus pais em casa uma pequena.
- Reference: `casa uma pequena` -> uma casa pequena [word_order]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Eu moro` -> `Eu moro` [verb_form] The correct conjugation for 'I live' in the present tense is 'Eu moro' or 'Eu moro' is acceptable, but the structure is slightly unnatural for this context.
- Found (matched): `em casa uma pequena` -> `em uma casa pequena` [article_preposition] The preposition and article usage for describing a place is incorrect.

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Sim` -> `Sim, eu tenho` [word_choice] The correct affirmative answer in this context is usually 'Sim, eu tenho...' or simply 'Sim, tenho...', but the structure is slightly unnatural for a simple yes/no question.

### a1-control-accents-present (control, a1)

- Task: What do you eat for breakfast?
- Answer: De manhã eu como pão com queijo e bebo café com leite.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `De manhã` -> `De manhã` [word_order] The correct way to express 'In the morning' is 'De manhã' or 'Para o café da manhã'. 'De manhã' alone sounds unnatural.

### a1-control-where-live (control, a1)

- Task: Where do you live?
- Answer: Moro no Brasil, numa cidade perto do mar.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Moro` -> `Eu moro` [verb_form] The correct conjugation for 'I live' in the present tense is 'Eu moro' or simply 'Eu moro' in this context, but the subject pronoun is missing.

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Vou pra casa agora.` -> `Eu vou para casa agora.` [word_choice] The use of 'pra' is too informal for this context.

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Na minha bolsa` -> `Minha bolsa tem` [article_preposition] The correct way to say this is 'In my bag' or 'My bag has', without the article 'a' or 'the' in this context.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Camila está de` -> `Camila está com` [word_choice] The correct way to describe what someone is wearing is by saying they are wearing something.

### a2-control-preterite-sunday (control, a2)

- Task: What did you do on Sunday?
- Answer: No domingo eu acordei tarde, fiz o almoço e depois assisti a um filme.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `No domingo` -> `No domingo` [article_preposition] The correct way to say 'On Sunday' is 'No domingo' or simply 'No domingo' without the article.

### a2-control-imperfect-grandmother (control, a2)

- Task: Describe your grandmother's house.
- Answer: A casa da minha avó era pequena, mas tinha um jardim grande com muitas flores.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `da` -> `a` [article_preposition] The definite article 'a' is needed here.

### a2-control-reflexive-party (control, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu me diverti muito.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `eu` -> `` [pronoun] The subject pronoun should be omitted in this context.

### a2-control-pet (control, a2)

- Task: Describe your pet.
- Answer: Eu tenho um cachorro. O meu cachorro se chama Bolt e ele adora correr.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `O meu cachorro` -> `Meu cachorro` [article_preposition] The possessive pronoun 'meu' is unnecessary here.

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `doente` -> `doente` [word_choice] The correct adjective for illness is 'doente' in this context, but the sentence structure is slightly unnatural for a simple status update.

### a2-control-family-meal (control, a2)

- Task: Describe the meal these people are eating.
- Answer: A família está comendo arroz, feijão e frango grelhado. Eles bebem suco de laranja.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `A família está comendo` -> `A família come` [verb_form] The correct way to describe what people are eating is usually in the simple present tense or a more direct description.

### a2-control-phone (control, a2)

- Task: Where is your phone?
- Answer: O meu celular está em cima do sofá.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `O meu celular` -> `Meu celular está` [article_preposition] The definite article 'o' is usually omitted before possessive pronouns like 'meu' in this context.

### a2-control-cinema-contraction (control, a2)

- Task: Where did you go on Saturday?
- Answer: No sábado eu fui ao cinema com meus amigos.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `No sábado` -> `No sábado` [article_preposition] The correct way to say 'On Saturday' is 'No sábado' or 'No sábado' depending on context, but the structure is slightly off.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `todos os dias` -> `todos os dias` [word_order] The correct way to say 'every day' is 'todos os dias' or 'todos os dias' in this context, but the phrasing is slightly unnatural.

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `eu sempre ia` -> `eu sempre ia` [verb_form] The verb tense is incorrect for describing a habitual past action.

### b1-control-gostar-de (control, b1)

- Task: What do you like about the city?
- Answer: Eu gosto da cidade porque é tranquila e as pessoas são simpáticas.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Eu gosto da cidade` -> `Eu gosto da cidade` [article_preposition] The correct structure is 'Eu gosto da cidade' or 'Eu gosto da cidade' (without the article 'a' in this context).

### b1-control-weather-hot (control, b1)

- Task: Describe the weather today.
- Answer: Hoje faz muito calor e o céu está sem nuvens.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `o céu está sem nuvens` -> `não há nuvens` [word_choice] The correct way to say the sky has no clouds is 'não há nuvens' or 'não tem nuvens'.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Trabalhei` -> `Trabalhei` [verb_form] The verb 'trabalhei' is in the past tense, but the context implies a description of the day, so the present tense or a more natural past construction is expected.

### b2-control-news (control, b2)

- Task: Comment on the news.
- Answer: As notícias de hoje são preocupantes, mas acredito que a situação possa melhorar.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `As notícias` -> `As notícias` [article_preposition] The definite article 'as' is unnecessary here.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `convidei-a` -> `convidei-la` [verb_form] The correct past tense conjugation for 'convidar' in this context is 'convidei-la' or simply 'convidei', and the pronoun placement is incorrect.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Response: invalid or missing

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Response: invalid or missing

## gemma4:e4b-it-qat / few_shot

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
- Found (matched): `De manha` -> `De manhã` [article_preposition] The phrase requires the definite article 'a' before 'manhã'.
- Missed: `cafe`

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `estou brasileiro`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Eu chamo Pedro` -> `Meu nome é Pedro` [word_choice] The correct way to state one's name is 'Meu nome é Pedro' or 'Eu sou Pedro'.

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
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `muito` -> `muito` [word_choice] The adverb 'muito' is used incorrectly here; 'muito' should be an adjective or used differently.
- Missed: `gosto de café muito`

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `azul` -> `verde` [content_mismatch] The jacket is green, not blue.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
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
- Missed: `almoçaram`

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

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Missed: `fazendo sol e muito calor`

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

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `tenho visitado`

### b1-content-date (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1850 e restaurada em 2005.
- Reference: `1850` -> 1750 [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `1850` -> `1750` [content_mismatch] The fact states the cathedral was built in 1750, not 1850.

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

## gemma4:e4b-it-qat / few_shot_six

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

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `chamo` -> `chamo` [verb_form] The correct verb form for 'eu' is 'chamo'.
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
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `muito` -> `muito` [word_order] The adverb 'muito' is misplaced and should follow the noun it modifies.
- Missed: `gosto de café muito`

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Missed: `azul`

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
- Missed: `nós ajudamos`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Missed: `fazendo sol e muito calor`

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `comeu` -> `comeu` [verb_form] The subject is 'a gente', which requires the verb in the third person singular, but the action is done by 'a gente' (we), so it should be 'comeu' if referring to 'a gente' as 'we', but since 'a gente' is used, the verb form is correct for the subject, however, the structure implies 'we ate', so 'a gente comeu' is correct. The issue is that 'a gente' is used, which is informal, but the verb form is correct for 'a gente'. Let's check the context. 'A gente foi' (we went). Then 'e depois comeu pizza' (and then ate pizza). This implies 'a gente' ate. So 'comeu' is correct for 'a gente'. No mistake found based on standard usage for A2 level informal speech.

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Se eu ter`

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `tenho visitado`

### b1-content-date (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1850 e restaurada em 2005.
- Reference: `1850` -> 1750 [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `1850` -> `1750` [content_mismatch] The year given in the facts is 1750, not 1850.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós podermos` -> `para podermos` [word_choice] The correct structure requires the infinitive form of the verb after 'para' when referring to the subject of the main clause.

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

## gemma4:e4b-it-qat / plain

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Ana` -> `Seu nome` [content_mismatch] The speaker is answering the question 'What is your name?' but uses a specific name instead of a general response.

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
- Found (matched): `De manha` -> `de manhã` [spelling] The correct way to say 'in the morning' in Portuguese is 'de manhã'.
- Missed: `cafe`

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `estou brasileiro`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Pedro` -> `seu nome` [content_mismatch] The speaker is answering the question 'What is your name?' but provides a name that is not their own.
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
- Found (matched): `uma vermelha camisa` -> `camisa vermelha` [word_order] The adjective should follow the noun in Portuguese.
- Found (false positive): `uma calça preta` -> `calça preta` [word_order] The adjective should follow the noun in Portuguese.

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `muito` -> `muito` [word_order] The adverb 'muito' is usually placed before the verb or after the object when modifying the verb 'gostar' in this context.
- Missed: `gosto de café muito`

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `azul` -> `verde` [content_mismatch] The jacket is green, not blue.
- Found (changes nothing): `tênis brancos` -> `tênis brancos` [content_mismatch] The sneakers are white, not white (the description is correct, but the fact states 'white sneakers', and the answer says 'tênis brancos', which is correct, however, if we strictly follow the facts, the color is white, and the answer uses the correct Portuguese translation for white). Re-evaluating: The fact says 'white sneakers', and the answer says 'tênis brancos'. This is a correct translation. No mistake here based on the facts provided.

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo.` -> `` [content_mismatch] The answer does not state what was eaten for breakfast.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `portugues`

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

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `sol` -> `chuva` [content_mismatch] The scene states it is raining, not sunny.
- Found (duplicate): `calor` -> `frio` [content_mismatch] The scene states it is cold, not hot.

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `num` -> `em um` [article_preposition] The contraction 'num' is informal; 'em um' is the standard form.

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vou ao mercado` -> `ir ao mercado` [article_preposition] The verb 'ir' (to go) requires the preposition 'a' before the destination, but when followed by a definite article, the contraction 'ao' is correct, however, the phrasing is slightly unnatural for this context.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `comeu` -> `comeu` [verb_form] The verb needs to be conjugated to agree with the subject 'a gente'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eles estão comendo espaguete e bebendo vinho tinto.` [content_mismatch] The answer does not describe the meal as requested by the task.

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Se eu ter`

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `moro` -> `more` [verb_form] The present tense of 'morar' does not correctly convey the duration of time since a specific past event.
- Missed: `tenho visitado`

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `` [pronoun] The pronoun 'nós' is unnecessary here because the verb 'podermos' already implies the subject.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] The verb should be in the subjunctive mood after 'recomendo que'.
- Found (matched): `assiste` -> `assista` [verb_form] The verb should be in the subjunctive mood after 'que'.
- Missed: `exige`

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

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A` -> `` [article_preposition] The definite article 'A' is unnecessary when describing what someone is wearing in this context.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está chovendo muito e eu estou em casa.` -> `Meu nome é [Your Name].` [content_mismatch] The answer does not state the name as requested by the task.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei uma bicicleta nova no sábado.` -> `` [content_mismatch] The answer does not address the question about working from home.

## gemma4:e4b / few_shot

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
- Found (matched): `De manha` -> `De manhã` [spelling] The correct Portuguese phrase is 'De manhã'.
- Missed: `cafe`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `uma escritório` -> `um escritório` [agreement] 'Escritório' is a masculine noun, so it requires 'um'.

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `moro Toronto` -> `moro em Toronto` [article_preposition] The verb 'morar' requires a preposition before the city name.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Eu chamo Pedro` -> `Eu me chamo` [word_choice] The correct way to state one's name is to use 'me chamo'.

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
- Found (changes nothing): `vermelha` -> `vermelha` [word_choice] The adjective must be masculine to agree with 'camisa' (which is feminine, but the color is usually described as masculine when referring to the shirt itself, or the noun should be masculine). However, the main error is that 'camisa' is feminine, so 'vermelha' is correct, but 'ele está com' is less natural than 'ele veste'. Given the A1 level, the main error is the use of 'está com' for permanent description. I will focus on the agreement of the noun/adjective pair. 'Camisa' is feminine, so 'vermelha' is correct. The error is in the use of 'está com' instead of 'veste'. I will report the less severe error related to the structure.
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `muito` -> `muito` [word_order] The adverb 'muito' is incorrectly placed after the noun 'café' in this structure.
- Missed: `gosto de café muito`

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `azul` -> `verde` [content_mismatch] The jacket color should be green according to the facts.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós almoçamos` -> `almoçamos` [pronoun] The subject pronoun 'nós' is redundant when the verb conjugation already implies it.
- Missed: `volti`

### a2-verb-form-subject-agreement (erroneous, a2)

- Task: What did your family do on Sunday?
- Answer: No domingo minha família almoçaram na casa da minha avó.
- Reference: `almoçaram` -> almoçou [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No domingo` -> `No domingo` [article_preposition] The preposition 'em' is needed before 'domingo'.
- Missed: `almoçaram`

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

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `chovendo` [content_mismatch] The weather is rainy, not sunny.
- Found (duplicate): `muito calor` -> `muito frio` [content_mismatch] The weather is cold, not hot.

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vou ao mercado` -> `(no mistake found)` [content_mismatch] The verb 'ir' (to go) requires the preposition 'a' before the place, but 'mercado' is masculine, so it should be 'ao'. However, the structure 'vou ao mercado' is correct for 'I am going to the market'. The mistake is that the structure 'vou ao mercado' is not wrong, but the previous sentence structure is slightly awkward for a native speaker, but I will not report it as a mistake. The structure is acceptable in spoken Brazilian Portuguese. I will re-evaluate. The sentence is grammatically correct in Brazilian Portuguese.

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `do que` -> `que` [article_preposition] When comparing with 'mais', the preposition 'que' is used without the article 'o'.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `banco` -> `banco` [word_choice] The word for 'bank' (financial institution) is 'banco', but the instruction implies a physical object, so 'banco' (bench) is more appropriate in this context.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `comeu` -> `pizza` [content_mismatch] The subject 'a gente' requires the verb to be conjugated in the third person singular, which is correct, but the structure is awkward; 'a gente comeu' is acceptable, but 'a gente comeu' is not the most natural phrasing for the action following 'e depois'. However, since 'a gente' is used, 'comeu' is technically correct for the subject, but the structure is slightly off for a natural flow. I will focus on the most direct error: the verb tense/agreement after 'e depois' when the subject is implied to be 'a gente'. The structure is acceptable in spoken Brazilian Portuguese, so I will not mark it as a mistake.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro` -> `Eles comem` [content_mismatch] The answer does not describe the meal.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=true answers_the_question=false

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `nunca tenho visitado` -> `nunca visitei` [verb_form] The tense used is incorrect for this context; the simple past or present perfect continuous is more appropriate, but the structure is awkward.

### b1-content-date (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1850 e restaurada em 2005.
- Reference: `1850` -> 1750 [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `1850` -> `1750` [content_mismatch] The fact states the cathedral was built in 1750.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `a gente` [pronoun] The pronoun 'nós' is incorrect in this context; it should be 'a gente' or 'eu' depending on the intended subject, but 'a gente' is more natural for this structure.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] The verb 'estudar' must be in the subjunctive mood after 'recomendo que'.
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

### a1-short-agreement-house (erroneous, a1)

- Task: Describe your house.
- Answer: Minha casa é grande e bonito.
- Reference: `bonito` -> bonita [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `bonito` -> `bonita` [agreement] The adjective must be feminine to agree with 'casa'.

### a1-word-choice-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu sou vinte anos velho.
- Reference: `sou vinte anos velho` -> tenho vinte anos [word_choice]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `vinte anos velho` -> `tenho vinte anos` [word_choice] The structure for stating age in Portuguese is different.

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
- Found (matched): `em casa uma pequena` -> `em uma casa pequena` [word_order] The adjective needs to be placed before the noun, and the article is missing.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está` -> `usa` [ser_estar] The verb 'estar' is used for temporary states, but 'vestir' (to wear) is a state that is usually described with 'ser' or a simple description in this context, though 'estar' is common in spoken Brazilian Portuguese, the description of what she wears is usually done with 'vestir' or 'usar' which implies a more permanent state or a description of the outfit. However, the main error is the lack of agreement in the description of the boots.
- Found (false positive): `marrons` -> `marronas` [agreement] The adjective 'marrom' (brown) is singular when describing the material/color of the boots, or it should agree with 'botas' (plural). Since 'botas' is plural, 'marrons' is technically correct in plural, but the description is slightly awkward. The main issue is that 'marrom' is often used as an invariable color adjective, but if we treat it as an adjective, it should agree. Given the context, the most direct correction is to ensure the color adjective agrees with the plural noun 'botas'.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está chovendo muito` -> `Meu nome é...` [content_mismatch] This answers the weather, not the question about the name.

## gemma4:e4b / few_shot_six

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
- Found (changes nothing): `muito` -> `muito` [word_order] The adverb should follow the noun it modifies in this structure.
- Missed: `gosto de café muito`

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `azul` -> `verde` [content_mismatch] The jacket is green, not blue.

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `(no answer)` [content_mismatch] The answer does not describe what was eaten for breakfast.

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
- Found (matched): `onibus` -> `ônibus` [spelling] The word needs an accent: ônibus.
- Found (matched): `as nove` -> `às nove` [article_preposition] The correct way to say 'at nine' is 'às nove'.
- Missed: `cafe`

### a2-verb-form-subject-agreement (erroneous, a2)

- Task: What did your family do on Sunday?
- Answer: No domingo minha família almoçaram na casa da minha avó.
- Reference: `almoçaram` -> almoçou [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No domingo` -> `No domingo` [article_preposition] The preposition 'em' is usually used before 'domingo' in Brazilian Portuguese.
- Missed: `almoçaram`

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

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `chovendo` [content_mismatch] The weather is rainy, not sunny.
- Found (duplicate): `muito calor` -> `muito frio` [content_mismatch] The weather is cold, not hot.

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `com minha irmã` -> `com a minha irmã` [agreement] The possessive adjective 'minha' must agree in gender with 'irmã'.

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vou ao mercado` -> `vou ao mercado` [article_preposition] The verb 'ir' to a place requires the preposition 'a' and the article 'o', which contracts to 'ao'.

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `do que` -> `que` [article_preposition] When comparing qualities, the preposition 'que' is used without the article 'o'.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `banco` -> `banco` [content_mismatch] The word for 'bank' (financial institution) is 'banco', but the instruction implies a physical object, so 'banco' (seat) is acceptable, but 'banco' (financial institution) is the most common translation for the context of directions. Given the ambiguity, I will check for a more direct error. The structure is fine for A2. No clear error found based on the provided facts and common usage.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `comeu` -> `comemos` [verb_form] The subject 'a gente' requires the verb 'comer' in the first person plural form, which is 'comemos'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro` -> `(no answer)` [content_mismatch] The answer does not describe the meal.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=true answers_the_question=false

### b1-content-date (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1850 e restaurada em 2005.
- Reference: `1850` -> 1750 [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `1850` -> `1750` [content_mismatch] The fact states the cathedral was built in 1750.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `a gente` [pronoun] The pronoun 'nós' is not needed here; the construction 'para podermos' is incorrect.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] The subjunctive mood is required after 'recomendo que'.
- Found (matched): `assiste` -> `assista` [verb_form] The subjunctive mood is required after 'que'.
- Missed: `exige`

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `morto` -> `morto` [agreement] The adjective must agree in gender with the speaker (implied 'eu').

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu preocupo`

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
- Found (matched): `em casa uma pequena` -> `em uma casa pequena` [word_order] The adjective needs to be placed before the noun, and the article is missing.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A Camila` -> `Camila` [article_preposition] The definite article 'A' is not used before a proper name like Camila.

### a2-control-preterite-sunday (control, a2)

- Task: What did you do on Sunday?
- Answer: No domingo eu acordei tarde, fiz o almoço e depois assisti a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No domingo` -> `No domingo` [article_preposition] The preposition 'em' is more common in spoken Brazilian Portuguese for days of the week.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está chovendo muito` -> `(No mistake found in the provided answer that relates to the question asked)` [content_mismatch] This answers the weather, not the question about the name.

## gemma4:e4b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A` -> `` [article_preposition] The definite article 'A' is not needed before the name 'Marina' in this context.

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha` -> `De manhã` [spelling] The correct Portuguese phrasing for 'in the morning' is 'De manhã'.
- Missed: `cafe`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Eu chamo Pedro` -> `Meu nome é` [word_choice] The speaker is stating their name, so the verb 'chamar' is used incorrectly.

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A minha amiga` -> `Minha amiga` [word_choice] The possessive adjective 'minha' is incorrect when referring to a friend in this context.
- Missed: `chama`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word 'beans' is English and should be translated to Portuguese.
- Found (false positive): `frango` -> `o frango` [article_preposition] The word 'frango' is correct, but it is better to use the definite article 'o' before it in this context.

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective 'vermelha' must agree in gender with the noun 'camisa', which is feminine, but the article 'uma' is used, and the adjective should be in the feminine form.
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `muito` -> `muito` [word_order] The adverb 'muito' is usually placed differently or a different word is needed here.
- Missed: `gosto de café muito`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo.` -> `` [content_mismatch] The answer does not state what was eaten for breakfast.

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `onibus` -> `ônibus` [spelling] The word for bus needs an accent mark.
- Missed: `cafe`
- Missed: `as nove`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós almoçamos` -> `almoçamos` [pronoun] The subject pronoun 'nós' is redundant when the verb conjugation already implies the first-person plural.
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
- Found (matched): `fazendo sol` -> `chovendo` [content_mismatch] The weather is rainy, not sunny.
- Found (duplicate): `muito calor` -> `muito frio` [content_mismatch] The weather is cold, not hot.

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No ano passado` -> `No` [article_preposition] The preposition 'Em' is needed before 'ano passado' to indicate time.

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vou ao mercado` -> `vou no mercado` [article_preposition] The verb 'ir' (to go) requires the preposition 'a' before the destination, but the article 'o' is not needed here.

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `do que` -> `que` [article_preposition] The comparison structure requires the preposition 'que' without the article 'o'.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `banco` -> `banco` [word_choice] The word 'banco' means 'bench' or 'bank' (financial institution), but the context implies a turning point like a specific building or landmark, and 'banco' is too general or potentially incorrect for a standard direction.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `comeu` -> `comeu` [verb_form] The subject 'a gente' requires the third-person singular form of the verb 'comer' in the past tense.

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No meu tempo livre` -> `No meu tempo livre` [article_preposition] The preposition 'em' is usually used before 'meu' in this context.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eles estão comendo espaguete e bebendo vinho tinto.` [content_mismatch] The answer does not describe the meal as requested by the task.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `` [pronoun] The pronoun 'nós' is not necessary here because the verb 'podermos' already implies the subject.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] The verb 'estudar' needs to be in the subjunctive mood after 'recomendo que'.
- Found (matched): `assiste` -> `assista` [verb_form] The verb 'assistir' in this context (to watch) requires the subjunctive mood after 'que'.
- Missed: `exige`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A Ana` -> `Ana` [article_preposition] The definite article 'A' is unnecessary before the proper name 'Ana'.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morto de cansaço` -> `exausto` [word_choice] The correct idiomatic expression is 'morto de sono' or 'morto de cansaço' is used in a less common way, but 'exausto' or 'muito cansado' is more natural.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe disse` -> `disse` [word_choice] The indirect object pronoun 'lhe' is not needed here because the direct object is 'que não me esperasse'.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `preocupo` -> `preocupo` [verb_form] The verb 'preocupar' is usually used reflexively in this context.
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

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Na minha bolsa tem` -> `Minha bolsa tem` [word_order] The structure 'Na minha bolsa tem' is unnatural in Portuguese.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A` -> `` [article_preposition] The definite article 'A' is unnecessary before the name 'Camila' in this context.

### a2-control-family-meal (control, a2)

- Task: Describe the meal these people are eating.
- Answer: A família está comendo arroz, feijão e frango grelhado. Eles bebem suco de laranja.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está comendo` -> `come` [verb_form] The verb 'comer' in the present continuous tense is usually used with 'estar' in Brazilian Portuguese, but the simple present is more common for describing a typical meal.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para que eu passe` -> `para passar` [verb_form] The structure 'para que' requires the subjunctive mood, which is not used here.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está chovendo muito e eu estou em casa.` -> `Meu nome é [Your Name]` [content_mismatch] The answer does not state the name as requested by the question.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei uma bicicleta nova no sábado.` -> `` [content_mismatch] The answer does not address the question about working from home.

## granite4.2:3b / few_shot

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
- Missed: `uma escritório`

### a1-agreement-plural-adjective (erroneous, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos. Eles são muito simpático.
- Reference: `simpático` -> simpáticos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `simpático`

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sou em casa`

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `estou brasileiro`

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sou cansado`

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
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `gosto de café muito`

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `azul`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=true

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

### a2-verb-form-irregular-fazer (erroneous, a2)

- Task: What did you do yesterday evening?
- Answer: Ontem à noite eu fazi o jantar e depois fui dormir cedo.
- Reference: `fazi` -> fiz [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
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
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `moro`

### a2-verb-form-subject-agreement (erroneous, a2)

- Task: What did your family do on Sunday?
- Answer: No domingo minha família almoçaram na casa da minha avó.
- Reference: `almoçaram` -> almoçou [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
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

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `atendi`

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
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fazendo sol e muito calor`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
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
- Missed: `tenho visitado`

### b1-agreement-passive (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1750 e foi restaurado em 2005.
- Reference: `restaurado` -> restaurada [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `restaurado`

### b1-content-date (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1850 e restaurada em 2005.
- Reference: `1850` -> 1750 [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `1850`

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `tinha começou`

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

### a1-pronoun-possessive-pet (erroneous, a1)

- Task: Describe your pet.
- Answer: Eu tenho um cachorro. Mim cachorro se chama Bolt.
- Reference: `Mim cachorro` -> Meu cachorro [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Mim cachorro`

### b2-pronoun-clitic-gender (erroneous, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, eu falei com a Ana e convidei-o para a festa.
- Reference: `convidei-o` -> convidei-a [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `convidei-o`

### a1-short-spelling-chocolate (erroneous, a1)

- Task: What do you like?
- Answer: Eu gosto de chocolat.
- Reference: `chocolat` -> chocolate [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chocolat`

### a1-short-verb-morar (erroneous, a1)

- Task: Where do you live?
- Answer: Eu mora em Lisboa.
- Reference: `mora` -> moro [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `mora`

### a1-short-agreement-house (erroneous, a1)

- Task: Describe your house.
- Answer: Minha casa é grande e bonito.
- Reference: `bonito` -> bonita [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `bonito`

### a1-word-choice-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu sou vinte anos velho.
- Reference: `sou vinte anos velho` -> tenho vinte anos [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sou vinte anos velho`

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

### a2-article-contraction-de-o (erroneous, a2)

- Task: Whose is the car?
- Answer: O carro é de o meu pai.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `de o`

### a2-preposition-gostar-de (erroneous, a2)

- Task: What do you like about the city?
- Answer: Eu gosto a cidade porque é tranquila.
- Reference: `gosto a cidade` -> gosto da cidade [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `gosto a cidade`

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
- Missed: `eu passo`

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
- Missed: `preocupante`

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
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `casa uma pequena`

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=true

## granite4.2:3b / few_shot_six

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
- Missed: `uma escritório`

### a1-agreement-plural-adjective (erroneous, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos. Eles são muito simpático.
- Reference: `simpático` -> simpáticos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `simpático`

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sou em casa`

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `estou brasileiro`

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sou cansado`

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
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `gosto de café muito`

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `azul`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=true

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

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `atendi`

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

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=true

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

### b1-agreement-passive (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1750 e foi restaurado em 2005.
- Reference: `restaurado` -> restaurada [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `restaurado`

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `tinha começou`

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

### a1-pronoun-possessive-pet (erroneous, a1)

- Task: Describe your pet.
- Answer: Eu tenho um cachorro. Mim cachorro se chama Bolt.
- Reference: `Mim cachorro` -> Meu cachorro [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Mim cachorro`

### b2-pronoun-clitic-gender (erroneous, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, eu falei com a Ana e convidei-o para a festa.
- Reference: `convidei-o` -> convidei-a [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `convidei-o`

### a1-short-spelling-chocolate (erroneous, a1)

- Task: What do you like?
- Answer: Eu gosto de chocolat.
- Reference: `chocolat` -> chocolate [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chocolat`

### a1-short-verb-morar (erroneous, a1)

- Task: Where do you live?
- Answer: Eu mora em Lisboa.
- Reference: `mora` -> moro [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `mora`

### a1-short-agreement-house (erroneous, a1)

- Task: Describe your house.
- Answer: Minha casa é grande e bonito.
- Reference: `bonito` -> bonita [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `bonito`

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

### a2-article-contraction-de-o (erroneous, a2)

- Task: Whose is the car?
- Answer: O carro é de o meu pai.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `de o`

### a2-preposition-gostar-de (erroneous, a2)

- Task: What do you like about the city?
- Answer: Eu gosto a cidade porque é tranquila.
- Reference: `gosto a cidade` -> gosto da cidade [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `gosto a cidade`

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
- Missed: `eu passo`

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
- Missed: `preocupante`

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
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `pequena` -> `pequena` [article_preposition] The word 'pequena' is missing an article or preposition; it should be 'uma casa pequena' or 'uma casa de pequeno porte'.
- Missed: `casa uma pequena`

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=true

## granite4.2:3b / plain

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
- Missed: `uma escritório`

### a1-agreement-plural-adjective (erroneous, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos. Eles são muito simpático.
- Reference: `simpático` -> simpáticos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `simpático`

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sou em casa`

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `estou brasileiro`

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sou cansado`

### a1-article-contraction-em-o (erroneous, a1)

- Task: Where do you live?
- Answer: Eu moro em o Brasil, em São Paulo.
- Reference: `em o` -> no [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `o Brasil` -> `Brasil` [article_preposition] The article 'o' is incorrect; in Brazilian Portuguese, 'o' is used for masculine nouns, but 'Brasil' is a proper noun and should not be preceded by 'o'.
- Found (quote not in the answer): `em o São Paulo` -> `em São Paulo` [article_preposition] The phrase 'em o São Paulo' is incorrect; the correct preposition is 'em' followed by the city name without an extra 'o'.

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
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `gosto de café muito`

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `usa` -> `` [word_order] The verb 'usar' is not needed; the sentence should state what Rafael is wearing without the verb.
- Found (matched): `jaqueta azul` -> `green jacket` [content_mismatch] The scene says Rafael wears a green jacket, not a blue one.
- Found (false positive): `calça jeans` -> `jeans` [word_order] The correct term is 'calça jeans' or simply 'jeans' in Portuguese; 'calça jeans' is acceptable but the phrase is redundant.
- Found (false positive): `tênis brancos` -> `white sneakers` [word_choice] The scene says white sneakers, not tênis brancos (which is correct but the term 'sneakers' is more common in this context).

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=true

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

### a2-verb-form-irregular-fazer (erroneous, a2)

- Task: What did you do yesterday evening?
- Answer: Ontem à noite eu fazi o jantar e depois fui dormir cedo.
- Reference: `fazi` -> fiz [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
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
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `moro`

### a2-verb-form-subject-agreement (erroneous, a2)

- Task: What did your family do on Sunday?
- Answer: No domingo minha família almoçaram na casa da minha avó.
- Reference: `almoçaram` -> almoçou [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
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

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `atendi`

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
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fazendo sol e muito calor`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=true

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
- Missed: `tenho visitado`

### b1-agreement-passive (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1750 e foi restaurado em 2005.
- Reference: `restaurado` -> restaurada [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `restaurado`

### b1-content-date (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1850 e restaurada em 2005.
- Reference: `1850` -> 1750 [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `1850`

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `tinha começou`

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

### a1-pronoun-possessive-pet (erroneous, a1)

- Task: Describe your pet.
- Answer: Eu tenho um cachorro. Mim cachorro se chama Bolt.
- Reference: `Mim cachorro` -> Meu cachorro [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Mim cachorro`

### b2-pronoun-clitic-gender (erroneous, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, eu falei com a Ana e convidei-o para a festa.
- Reference: `convidei-o` -> convidei-a [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `convidei-o`

### a1-short-spelling-chocolate (erroneous, a1)

- Task: What do you like?
- Answer: Eu gosto de chocolat.
- Reference: `chocolat` -> chocolate [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chocolat`

### a1-short-verb-morar (erroneous, a1)

- Task: Where do you live?
- Answer: Eu mora em Lisboa.
- Reference: `mora` -> moro [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `mora`

### a1-short-agreement-house (erroneous, a1)

- Task: Describe your house.
- Answer: Minha casa é grande e bonito.
- Reference: `bonito` -> bonita [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `bonito`

### a1-word-choice-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu sou vinte anos velho.
- Reference: `sou vinte anos velho` -> tenho vinte anos [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sou vinte anos velho`

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

### a2-article-contraction-de-o (erroneous, a2)

- Task: Whose is the car?
- Answer: O carro é de o meu pai.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `de o`

### a2-preposition-gostar-de (erroneous, a2)

- Task: What do you like about the city?
- Answer: Eu gosto a cidade porque é tranquila.
- Reference: `gosto a cidade` -> gosto da cidade [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `gosto` -> `gosto a cidade` [ser_estar] The verb 'gosto' is missing the article 'a' before 'cidade' (should be 'gosto a cidade').
- Found (false positive): `tranquila` -> `Eu gosto da cidade porque é tranquila.` [word_order] The adjective 'tranquila' is correct, but the sentence structure is awkward; it should be 'Eu gosto da cidade porque é tranquila.'

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
- Missed: `eu passo`

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
- Missed: `preocupante`

### a2-not-brazilian-phone (erroneous, a2)

- Task: Where is your phone?
- Answer: O meu telemóvel está em cima do sofá.
- Reference: `telemóvel` -> celular [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `telemóvel`

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-english-coffee (not_portuguese, a2)

- Task: Do you like coffee?
- Answer: Yes, I like coffee a lot.
- Flags: written_in_portuguese=true answers_the_question=true

## ministral-3:14b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de maiô` -> `usando um maiô` [article_preposition] The correct preposition for clothing is *usando* or *com* (not *de*).
- Found (false positive): `chapéu de palha` -> `boné de palha` [word_choice] For a beach hat, *boné* or *chapéu de sol* is more natural than *chapéu de palha* (straw hat).

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=false

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [article_preposition] The word needs a contraction with 'em' in spoken Brazilian Portuguese.
- Found (matched): `em uma escritório` -> `num escritório` [article_preposition] The correct contraction for 'em + uma' is 'num' or 'numa' (feminine).

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Missed: `chamo`

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
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
- Found (matched): `vermelha` -> `uma camisa vermelha` [word_order] The adjective should follow the noun in Brazilian Portuguese word order.
- Found (false positive): `calça` -> `calças` [agreement] The word for trousers in this context is 'calça' (singular) but should be plural 'calças' to match 'trousers'.

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `era` -> `estive` [verb_form] The verb must be in the past tense to describe yesterday's state.
- Found (changes nothing): `dormi o dia todo` -> `dormi o dia todo` [word_choice] The correct phrase is *dormi o dia inteiro* or *dormi o dia todo* (with no article).

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

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajei` -> `` [verb_form] The correct past tense for 'to travel' in Brazilian Portuguese is 'viajei' (from viajar), but the verb form here should be 'viajei' (which is correct), but the mistake is that the verb should be 'viajei' (which is correct). Actually, the verb is correctly conjugated. No mistake here.

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tenho visitado` -> `visto` [verb_form] The present perfect requires the auxiliary 'ter' + past participle ('tenho visitado' should be 'tenho **visto**').
- Found (false positive): `Nordeste` -> `o Nordeste` [article_preposition] The region name 'Nordeste' should be preceded by the definite article 'o' (since it starts with a vowel sound).

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] The subjunctive form of 'estudar' should be used after 'recomendo que'.
- Found (matched): `assiste` -> `assista` [verb_form] The subjunctive form of 'assistir' should be used after 'que'.
- Missed: `exige`

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu preocupo`

### a1-short-verb-morar (erroneous, a1)

- Task: Where do you live?
- Answer: Eu mora em Lisboa.
- Reference: `mora` -> moro [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `mora` -> `moro` [verb_form] The verb must agree with the subject 'eu' (first-person singular).
- Found (false positive): `Lisboa` -> `na Lisboa` [article_preposition] The city name should be preceded by the preposition 'em' or 'na' (since it's feminine) in this context.

### a1-short-agreement-books (erroneous, a1)

- Task: What is in your bag?
- Answer: Na minha bolsa tem um livros.
- Reference: `um livros` -> um livro | uns livros [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `um livros` -> `um livro` [agreement] The adjective must agree in gender and number with the noun.
- Found (changes nothing): `tem` -> `tem` [verb_form] The verb must agree with the subject 'um livro' (singular).

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

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tem` -> `tem` [verb_form] The verb must agree with 'bolsa' (feminine singular).

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A` -> `` [article_preposition] The article 'a' is unnecessary before a proper noun.

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `cheguei em casa` -> `cheguei em casa` [article_preposition] The correct preposition for arriving *at* home is 'à' or 'em casa' (without 'em' before 'casa').

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `-a` -> `a` [pronoun] The clitic pronoun for 'her' in this context is **-lhe** (indirect object) or **-a** (direct object), but the correct form here is **-a** (direct object) without a hyphen.

## ministral-3:14b / few_shot_six

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=false

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [spelling] The word needs an accent on the o: escritório.
- Missed: `uma escritório`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Missed: `chamo`

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Missed: `chama`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `era` -> `estive` [verb_form] The verb must be in the past tense to match 'ontem'.
- Found (changes nothing): `dormi o dia todo` -> `dormi o dia todo` [word_choice] The correct phrase is 'o dia inteiro' or 'o dia todo' (with no article).

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `passar` -> `passar no` [verb_form] The verb should be in the infinitive form to express purpose (to pass). The correct form is 'passar' but the context requires 'para passar' or 'a fim de passar'.
- Found (false positive): `no exame` -> `em` [article_preposition] The correct preposition for 'exam' in this context is 'em' (to take an exam).

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `lavo os dentes`

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ganhasse` -> `ganharia` [verb_form] The conditional form should be 'ganhasse' (subjunctive) for hypotheticals, but the correct conditional form for 'ganhar' is 'ganharia'.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós podermos` -> `podermos` [pronoun] The subject pronoun 'nós' is unnecessary before the infinitive in this context.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] The verb must be in subjunctive mood to match the recommendation: 'estude'.
- Found (matched): `assiste` -> `assista` [verb_form] The verb must be in subjunctive mood to match the recommendation: 'assista'.
- Found (duplicate): `a` -> `de` [article_preposition] The preposition before 'filmes' should be 'de' (assista **a** filmes **de** Brasil) or 'filmes brasileiros' (assista **a** filmes brasileiros).
- Missed: `exige`

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu preocupo`

### a1-short-verb-morar (erroneous, a1)

- Task: Where do you live?
- Answer: Eu mora em Lisboa.
- Reference: `mora` -> moro [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `mora` -> `moro` [verb_form] The verb must agree with 'eu' (first-person singular).
- Found (false positive): `Lisboa` -> `São Paulo` [not_brazilian] The answer should be in Brazilian Portuguese, not European Portuguese.

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

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tem` -> `tem` [verb_form] The verb must agree with 'bolsa' (feminine singular).

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `convidei-a` -> `convidei a ela` [pronoun] The pronoun '-a' is not used with indirect object pronouns in Brazilian Portuguese; use 'convidei **a ela**' or 'a convidei' instead.

## ministral-3:14b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de maiô` -> `usando um maiô` [article_preposition] The correct preposition for clothing items is *usando* or *com* (not *de*).

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `bem` -> `bom` [word_choice] In Brazilian Portuguese, 'bem' is an adverb and cannot stand alone as an adjective in this context.

### a1-control-daily-routine (control, a1)

- Task: What do you do every day?
- Answer: Todo dia eu acordo cedo, tomo café e vou para o trabalho.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Todo dia` -> `Todos os dias` [word_choice] The correct phrase is 'Todos os dias' when referring to a daily routine in this context.

### a1-control-order-meal (control, a1)

- Task: You are at a restaurant. Order your meal.
- Answer: Eu queria uma salada e um suco de laranja, por favor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu queria` -> `Eu quero` [verb_form] At A1 level, the learner should use the present tense for ordering in a restaurant.

### a1-control-family (control, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos e uma irmã. Meu pai é médico e minha mãe é professora.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `irmão` -> `irmãos` [agreement] The word *irmão* is masculine, but it should agree in gender with *uma irmã* (feminine).

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu me chamo Ana e moro em Curitiba.` -> `Ana` [content_mismatch] The question only asks for the name, not additional information.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [spelling] The correct spelling is 'escritório' with an accent on the 'o'.
- Found (matched): `em uma escritório` -> `num escritório` [article_preposition] The preposition 'em' requires the contraction 'num' before a masculine noun starting with a vowel.

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `Eu estou brasileiro` -> `Eu sou brasileiro` [ser_estar] The adjective *brasileiro* cannot follow *estar* to describe nationality; use *sou* instead.
- Found (changes nothing): `brasileiro` -> `brasileiro` [agreement] The adjective *brasileiro* does not agree in gender with the subject *eu* (masculine), but it is correct here as a noun meaning 'Brazilian person'. However, the correct phrasing for nationality is *sou brasileiro* (no article needed).

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `do` -> `de` [article_preposition] The correct preposition for 'from' in this context is *de*, not *do* (which is a contraction of *de o*).
- Found (matched): `moro Toronto` -> `moro em Toronto` [article_preposition] The verb *morar* requires the preposition *em* before the city name.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Eu chamo Pedro` -> `Meu nome é` [word_choice] The correct way to say 'My name is Pedro' in Portuguese is 'Meu nome é Pedro' or 'Eu me chamo Pedro'.

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `minha amiga` -> `N/A (remove this part)` [content_mismatch] The task asks for the name of the learner's friend, not a description of them.
- Missed: `chama`

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `azul` -> `verde` [content_mismatch] The jacket is green, not blue.
- Found (false positive): `calça jeans` -> `jeans` [agreement] The correct term for jeans is *jeans* (plural) or *calças jeans* (plural). 'Calça' is singular and incorrect here.

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo.` -> `N/A (answer should focus on breakfast)` [content_mismatch] The answer does not address the question about breakfast.

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `assisti` [verb_form] The correct past tense of *assistir* (to watch) in the *pretérito perfeito* is *assisti*, not *atendi* (which means 'answered').
- Found (false positive): `um filme novo` -> `novo filme` [agreement] The adjective *novo* should agree in gender with *filme* (masculine), so it should be *filme novo* (correct here), but the phrase *filme novo* is redundant since *filme* already implies 'new' in this context. The learner likely meant to say *um novo filme* (a new film).

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ficamos num` -> `ficamos em` [article_preposition] The correct preposition for 'stay' in this context is *em*, not *num* (which is a contraction of *em* + *um*).

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ao mercado` -> `` [article_preposition] The preposition *ao* is incorrect here because *mercado* is a general place (not a specific one starting with a vowel), so *ao* (a + o) should be *ao* only if referring to a specific place like *ao supermercado*. For a general *mercado*, use *ao* only if it's a direct object with *ir* (e.g., *ir ao mercado*), but in this case, the correct preposition is simply *ao* (which is correct here, but the mistake is in the phrasing—actually, *ao mercado* is correct in this context, so this is a false flag). **Correction: No mistake here.**

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `mais barato` -> `menos caro` [word_choice] The comparison should use *mais barato* only if comparing to another object. Here, it should be *menos caro* because the base form is *caro* (expensive).

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no banco` -> `na praia` [word_choice] The bank is not the building where money is handled, but the riverbank (beach) in this context.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `num` -> `em um` [not_brazilian] The correct contraction for 'em um' is 'num' only in European Portuguese; in Brazilian Portuguese, it is 'num' in informal speech but the standard written form is 'em um'.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ontem` -> `nada` [content_mismatch] The task asks for an opinion on the film itself, not when it was watched.

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `passear` -> `dar uma volta` [word_choice] The verb *passear* is not commonly used in Brazilian Portuguese for walking with a dog; *dar uma volta* or *caminhar* is more natural.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-verb-form-subjunctive-present (erroneous, b1)

- Task: What do you hope for the future?
- Answer: Espero que meus filhos são felizes e que eles tenham uma boa educação.
- Reference: `são` -> sejam [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `são` -> `estejam` [ser_estar] The verb *ser* is used incorrectly here for a future state (use *estar*).
- Found (changes nothing): `felizes` -> `felizes` [agreement] The adjective *felizes* does not agree with the subject *meus filhos* (plural masculine).

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `nunca tenho visitado` -> `nunca visitei` [verb_form] The present perfect is not used with *nunca* (never) in this context; the present tense is needed.
- Found (false positive): `o Nordeste` -> `N/A (remove or adjust to fit the question)` [content_mismatch] The answer does not address the question about living in the current location.

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `à` -> `para` [article_preposition] The correct preposition for 'to' in this context is *para*, not *à* (which is used for 'at' or 'to' in a location).

### b1-control-future-subjunctive (control, b1)

- Task: What will you do when you finish the course?
- Answer: Quando eu terminar o curso, vou procurar um emprego novo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `um emprego novo` -> `um novo emprego` [word_order] The adjective should follow the noun in Brazilian Portuguese.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The conditional tense should be used for hypothetical situations in Portuguese, but the verb form here is subjunctive (imperfect subjunctive).

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós podermos` -> `para eu poder` [pronoun] The subject of the verb *podermos* should match the implied subject of the main clause (*eu*), not *nós*.

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `sinto falta` -> `sinto falta **dos colegas**` [word_order] The verb *sentir falta* is used with a noun (e.g., *sinto falta de meus colegas*), not directly with a pronoun.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Prezado Sr.` -> `Prezado Sr.,` [word_order] In Brazilian Portuguese, 'Sr.' is typically followed by a comma, not a period, when used in formal addresses.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] The verb should be conjugated in the subjunctive mood for recommendations.
- Found (matched): `assiste` -> `assista` [verb_form] The verb should be conjugated in the subjunctive mood for recommendations.
- Found (changes nothing): `isso exige` -> `isso exige` [verb_form] The subject of the sentence is 'aprender' or 'isso' (neuter), so the verb should be conjugated in the third-person singular form.
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `por meio desta` -> `por meio deste` [agreement] The correct phrase for formal complaints is *por meio deste* (masculine) when referring to the letter itself.
- Found (matched): `sobre do atraso` -> `sobre o atraso` [article_preposition] The correct preposition for *atraso* is *sobre o atraso* (no article needed before *atraso* in this context).

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `na semana seguinte` -> `próxima semana` [word_choice] The correct phrase is *próxima semana* when referring to the next week in a future context.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `cheguei em casa` -> `cheguei em casa` [article_preposition] The correct preposition for arriving *at home* is *à* (or *em* in some contexts, but *em casa* is more natural).

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `por meio desta` -> `por meio deste` [agreement] The correct phrase for formal complaints is *por meio deste* (masculine) when referring to the letter itself.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `me esperasse` -> `esperasse` [verb_form] The subjunctive form is incorrect for a negative command in the past.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `muito` -> `bastante` [word_choice] At A1 level, 'muito' is often overused and can sound unnatural in this context. A more natural alternative is 'bastante' or 'um pouco' for a simple answer.
- Missed: `eu preocupo`

### a1-word-choice-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu sou vinte anos velho.
- Reference: `sou vinte anos velho` -> tenho vinte anos [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `velho` -> `de idade` [word_choice] The adjective 'velho' is not used to describe age in this context.
- Found (changes nothing): `vinte anos` -> `vinte anos` [content_mismatch] The correct form is 'vinte anos de idade' or simply 'vinte anos' when used with 'tenho'.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `um erro` -> `um erro / um equívoco / um deslize` [word_choice] The correct expression is *um erro* (a mistake) in the context of a test, but the verb *fazer* does not collocate with *erro* here. The correct collocation is *cometer um erro*.
- Found (false positive): `na prova` -> `na minha prova` [word_order] While not strictly incorrect, *na prova* is slightly less natural than *na minha prova* or simply *na prova de matemática* (without repetition). However, this is not a grammatical error but a stylistic choice.

### b1-verb-form-para-que (erroneous, b1)

- Task: Why do you study every day?
- Answer: Eu estudo todos os dias para que eu passo no exame.
- Reference: `eu passo` -> eu passe [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `passo` -> `passar` [verb_form] The correct verb form for 'pass' in this context is *passar* in the subjunctive mood (to express a future possibility).
- Found (false positive): `no exame` -> `em o exame` [article_preposition] The correct preposition for 'exam' in this context is *em* (meaning 'in the exam').

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `uma irmã` -> `irmã` [article_preposition] At A1 level, the indefinite article *uma* is not typically used with *irmã* when answering 'Do you have siblings?' in a simple yes/no context.

### a1-control-accents-present (control, a1)

- Task: What do you eat for breakfast?
- Answer: De manhã eu como pão com queijo e bebo café com leite.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `De manhã` -> `` [content_mismatch] The phrase is unnecessary in a direct answer to 'What do you eat for breakfast?'

### a1-control-where-live (control, a1)

- Task: Where do you live?
- Answer: Moro no Brasil, numa cidade perto do mar.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `numa` -> `em` [article_preposition] The correct preposition for 'in' when referring to a city is *em*, not *em uma* (which would require an article).

### a1-control-colloquial-pra (control, a1)

- Task: Where are you going?
- Answer: Vou pra casa agora. Tô cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `pra` -> `para` [not_brazilian] The contraction 'pra' is informal and not typically used in written A1-level Portuguese.

### a1-control-plural-bag (control, a1)

- Task: What do you have in your bag?
- Answer: Na minha bolsa tem dois livros, um caderno e uma garrafa de água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Na minha bolsa tem` -> `Na minha bolsa **está**` [verb_form] The verb *ter* (to have) requires the subject to be in the singular or plural form matching the noun, not the use of *tem* (third-person singular) after a first-person subject.

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de` -> `usando` [article_preposition] The preposition *de* is incorrect here for describing clothing; *usando* or *com* is needed.
- Found (false positive): `marrons` -> `marrom` [not_brazilian] The adjective *marrons* is European Portuguese; in Brazilian Portuguese, *marrons* is not used for 'brown' in this context.

### a2-control-preterite-sunday (control, a2)

- Task: What did you do on Sunday?
- Answer: No domingo eu acordei tarde, fiz o almoço e depois assisti a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `fiz o almoço` -> `preparei o almoço` [word_choice] The verb *fazer* is not used with *almoço* (lunch) in this context; *preparar* or *cozinhar* is more natural.

### a2-control-reflexive-party (control, a2)

- Task: How was the party?
- Answer: A festa foi ótima e eu me diverti muito.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `me diverti` -> `diverti-me` [word_order] The reflexive pronoun *me* should come before the verb in the past tense for emphasis or clarity in spoken Brazilian Portuguese, but in written form, it is more common to place it after the verb (e.g., *diverti-me*). However, the correct written form for past tense reflexive verbs is to place the pronoun before the verb (e.g., *Eu me diverti*). The mistake here is the **word order** of the reflexive pronoun in written form.
- Found (false positive): `me diverti` -> `diverti-me` [word_order] The correct written form for the past tense of *divertir-se* is *diverti-me* (with the reflexive pronoun attached to the verb). The learner's answer uses the correct spoken form but not the standard written form.

### a2-control-pet (control, a2)

- Task: Describe your pet.
- Answer: Eu tenho um cachorro. O meu cachorro se chama Bolt e ele adora correr.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `se chama` -> `chama` [verb_form] The correct verb form for 'to be called' in the present tense is *chama*, not *se chama*.

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `doente` -> `doente**m**` [agreement] The adjective 'doente' should agree in gender with the subject 'irmão' (masculine).

### a2-control-family-meal (control, a2)

- Task: Describe the meal these people are eating.
- Answer: A família está comendo arroz, feijão e frango grelhado. Eles bebem suco de laranja.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `comendo` -> `estão comendo` [verb_form] The present continuous tense for 'to eat' should be 'estão comendo' (they are eating), not just 'comendo' (eating) without the subject.

### a2-control-cinema-contraction (control, a2)

- Task: Where did you go on Saturday?
- Answer: No sábado eu fui ao cinema com meus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No sábado` -> `No sábado` [article_preposition] The correct preposition for 'on Saturday' in Brazilian Portuguese is 'no' only when referring to a day of the week in a general sense, but it should be contracted with 'o' (on Saturday) as 'no sábado' is correct. However, the mistake here is the **missing contraction**—the preposition 'em' is needed before 'sábado' when referring to a specific day (like 'on Saturday').

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para que eu` -> `para` [word_order] The phrase *para que eu* is unnecessary and unnatural in this context; the infinitive alone suffices.

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ia` -> `N/A (no mistake)` [verb_form] The correct past tense for 'ir' in this context (repeated actions in the past) is *ia*, but the verb should agree with the subject *eu* in the imperfect tense, which it does. However, the mistake here is the **verb_form** for the context of *always going* (repeated actions), which should use the imperfect tense correctly—but the verb itself is conjugated properly. **No mistake in verb form here.**

### b1-control-gostar-de (control, b1)

- Task: What do you like about the city?
- Answer: Eu gosto da cidade porque é tranquila e as pessoas são simpáticas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `da` -> `de` [article_preposition] The correct preposition for 'city' in this context is *de* (meaning 'of' or 'about').

### b1-control-weather-hot (control, b1)

- Task: Describe the weather today.
- Answer: Hoje faz muito calor e o céu está sem nuvens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `sem nuvens` -> `despejado` [word_choice] The correct phrase to describe a clear sky is 'despejado' or 'limpo' (not 'sem nuvens').

### a2-control-arrived-home (control, a2)

- Task: How was your day?
- Answer: Trabalhei até tarde e cheguei em casa às nove.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `em casa` -> `à casa` [article_preposition] The correct preposition for 'home' in this context is *em* only when referring to a general location (e.g., *estar em casa*). However, when specifying the time of arrival, the correct preposition is *à* (or *em* in some contexts, but *em casa às nove* is less common). The most natural phrasing here would be *cheguei **à** casa às nove* or *cheguei **em casa** às nove* (though the latter is less idiomatic).

### b2-control-news (control, b2)

- Task: Comment on the news.
- Answer: As notícias de hoje são preocupantes, mas acredito que a situação possa melhorar.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `As notícias` -> `As notícias` [article_preposition] The article 'as' is incorrect before a plural noun starting with a consonant.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `com a Ana` -> `com Ana` [article_preposition] The preposition *com* does not require the article *a* before a proper name.
- Found (false positive): `convidei-a` -> `convidei` [verb_form] The correct clitic pronoun for *Ana* (feminine) is *-a*, but the verb *convidei* already ends in *-i*, so the clitic should be attached without an extra *-a*.

## qwen3.5:4b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Maiô` -> `maio` [spelling] The word 'maiô' is misspelled; the correct spelling for swimsuit in Brazilian Portuguese is 'maio'.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `cansado` -> `cansado` [agreement] The adjective must agree in gender with the implied subject 'eu' (masculine).

### a1-control-daily-routine (control, a1)

- Task: What do you do every day?
- Answer: Todo dia eu acordo cedo, tomo café e vou para o trabalho.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tomo` -> `tomo o` [article_preposition] The verb 'tomar' requires the pronoun 'o' for coffee: tomar o café.

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `De` -> `À` [article_preposition] The word 'de' should be contracted with the following word 'manhã'.
- Missed: `manha`
- Missed: `cafe`

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Toronto` -> `Toronto` [spelling] Place names in Portuguese should be written with an accent on the o: Toronto.
- Missed: `moro Toronto`

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
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective must be in the feminine form to agree with 'camisa'.
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
- Found (false positive): `gosto` -> `comi` [word_choice] The verb 'gostar' is used incorrectly; the response should describe what was eaten for breakfast.
- Found (false positive): `futebol` -> `omelete` [content_mismatch] The word 'futebol' does not answer the question about food; it refers to a sport.
- Found (false positive): `time` -> `prato` [word_choice] The word 'time' is irrelevant to the context of breakfast food and creates a false friend with English 'time'.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `estudei` -> `estudei a` [article_preposition] The verb 'estudar' is intransitive and requires the preposition 'a' before the object.
- Found (matched): `portugues` -> `Português` [spelling] The name of the language must be capitalized.

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

### a2-verb-form-subject-agreement (erroneous, a2)

- Task: What did your family do on Sunday?
- Answer: No domingo minha família almoçaram na casa da minha avó.
- Reference: `almoçaram` -> almoçou [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `almoçaram` -> `almoçaram` [article_preposition] The verb 'almoçar' is intransitive and does not take a direct object pronoun like 'em', so the preposition 'em' is incorrect here.
- Missed: `almoçaram`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `era` -> `estava` [verb_form] The verb 'ser' is used incorrectly here; the past continuous tense requires 'estar'.
- Found (changes nothing): `cansado` -> `cansado` [agreement] The adjective must agree in gender with the subject 'eu' (masculine).

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engracada` [agreement] The adjective must be feminine to agree with 'minha melhor amiga'.
- Missed: `curtas`

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `proficiência` -> `nível` [word_choice] The word 'proficiência' is not commonly used in Brazilian Portuguese for this context; 'nível' or 'certificação' is more natural.
- Missed: `por passar`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `vi` [word_choice] The verb 'ver' is used for watching movies, not 'atender'.
- Found (false positive): `cinema` -> `vi` [word_choice] In Brazil, people usually say 'no cinema' to mean at the movie theater, but 'no cinema' is often redundant; however, the more natural phrasing for 'watching a movie' is simply 'no cinema' without specifying location again if context implies it. Actually, the main issue is that 'atendi um filme' is wrong. Let's re-evaluate. The phrase 'no cinema' is correct for location. The error is 'atendi'. So only one mistake? Wait, 'cinema' might be okay. Let's stick to the clear error: 'atendi'.

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tomo` -> `como` [word_choice] The verb 'tomar' is used for coffee, tea, or medicine, not for breakfast; the correct verb is 'comer'.
- Missed: `pequeno-almoço`

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ficamos` -> `ficávamos` [pronoun] The verb 'ficar' requires a reflexive pronoun when used to mean 'to stay'.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no banco` -> `em o banco` [article_preposition] The preposition 'em' is required before 'banco' to indicate location at the bank.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `They are eating spaghetti and drinking red wine at a restaurant.` [content_mismatch] The answer does not describe the meal or the people eating it; it talks about where the speaker lives and their feelings about the city.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=true answers_the_question=false

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Se eu ter`

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para` -> `de que` [article_preposition] The conjunction 'para' is incorrect here; the subjunctive clause requires 'de que'.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Prezado` -> `Prezado(a)` [word_choice] The formal address 'Prezado' is outdated and overly stiff for a modern email to a boss; 'Prezado(a)' or simply 'Olá, Sr. Souza' is more appropriate.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] The verb 'recomendar' requires the subjunctive mood for the clause following 'que'.
- Found (false positive): `filmes brasileiros` -> `estude` [verb_form] The adjective should precede the noun in this context, or use a different structure like 'de filmes brasileiros' to avoid ambiguity with 'filmes' as a direct object of 'assistir'. However, given the strict rule against reporting style, and since 'filmes brasileiros' is acceptable phrasing for 'watching Brazilian movies', I will not flag it. Let's re-evaluate. Actually, 'assistir a filmes' is correct. The issue might be subtle or non-existent. Let's look closer. 'Recomendo que você estuda' -> subjunctive needed. That's a clear error. Is there another? 'filmes brasileiros' - no error. 'isso exige paciência' - no error. So only one mistake.
- Missed: `assiste`
- Missed: `exige`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A` -> `` [article_preposition] The article 'a' is incorrect before the name 'Ana'; it should be omitted.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morto` -> `esgotado` [word_choice] The adjective 'morto' is used to mean 'dead', but here it should mean 'exhausted'.

### a1-pronoun-possessive-pet (erroneous, a1)

- Task: Describe your pet.
- Answer: Eu tenho um cachorro. Mim cachorro se chama Bolt.
- Reference: `Mim cachorro` -> Meu cachorro [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `Mim` -> `Eu` [pronoun] The word 'mim' is incorrect; the correct reflexive pronoun for 'I' is 'eu'.
- Found (false positive): `se chama` -> `chama` [verb_form] The verb 'chamar' requires the third-person singular conjugation when referring to the dog, not the first person.

### a1-word-choice-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu sou vinte anos velho.
- Reference: `sou vinte anos velho` -> tenho vinte anos [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `velho` -> `velho` [agreement] The adjective 'old' must be in the masculine singular form to agree with the implied subject 'eu'.
- Found (matched): `anos` -> `ter` [verb_form] In Portuguese, age is expressed using the verb 'ter' followed by a number and the unit of time (e.g., 'ter vinte anos'), not 'ser'.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `marrons` -> `marrons` [agreement] The adjective must be plural to agree with 'botas'.

### a2-control-preterite-sunday (control, a2)

- Task: What did you do on Sunday?
- Answer: No domingo eu acordei tarde, fiz o almoço e depois assisti a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `almoço` -> `a` [article_preposition] The verb 'fazer' requires the preposition 'a', not the noun 'almoço'.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `passe` -> `aprove` [verb_form] The verb 'passar' requires the subjunctive mood here, but 'passar' is not a subjunctive form; it should be 'aprove' or 'consiga', though 'passar' itself is incorrect in this context without proper conjugation.

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ia` -> `vou` [verb_form] The verb 'ir' must be in the future tense (vou) to express a habitual action in the past, not the imperfect tense (ia).

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está chovendo muito e eu estou em casa.` -> `Meu nome é...` [content_mismatch] The answer does not address the question about the name.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `comprei` -> `acho que é bom` [word_choice] The verb 'comprar' does not fit the context of an opinion about working from home; it means 'to buy'.
- Found (false positive): `uma bicicleta nova no sábado` -> `Eu acho que é bom trabalhar em casa.` [content_mismatch] This phrase describes a past action unrelated to the topic of opinions on remote work, creating a content mismatch.

## qwen3.5:4b / few_shot_six

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Maiô` -> `maio` [spelling] The word 'maiô' is misspelled; it should be 'maio'.
- Found (false positive): `vermelho` -> `vermelha` [agreement] The adjective must agree in gender with 'maio' (feminine).

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
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `um` -> `um` [agreement] The article 'um' is masculine, but 'escritório' is neuter (or feminine in this context).
- Found (matched): `em uma escritório` -> `no` [article_preposition] The preposition 'em' requires a definite article before the noun.

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
- Missed: `gosto de café muito`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `comi` [verb_form] The verb tense does not match the question asking about breakfast.
- Found (false positive): `futebol` -> `comi um ovo` [content_mismatch] The content contradicts the task, which asks about food, not sports.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `à` -> `a` [article_preposition] The preposition 'a' should not be contracted with the article 'o' in this context; it is used incorrectly before 'tarde'.
- Missed: `portugues`

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

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `era` -> `estava` [verb_form] The verb 'ser' is used incorrectly here; the past continuous of 'estar' ('estava') is needed to describe a temporary state like being tired.
- Found (changes nothing): `cansado` -> `cansado` [agreement] The adjective must agree in gender with the subject 'eu' (masculine singular).

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engracada` [agreement] The adjective must agree in gender with 'ela' (feminine).
- Missed: `curtas`

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `proficiência` -> `nível` [word_choice] The word 'proficiência' is not commonly used in Brazilian Portuguese for this context; 'nível' or 'certificação' is more natural.
- Missed: `por passar`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tomo` -> `como` [word_choice] The verb 'tomar' is used for drinks, not for food like 'pequeno-almoço'.
- Missed: `pequeno-almoço`

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `no banco` -> `no banco` [article_preposition] The preposition 'em' is needed before the noun 'banco' to indicate location.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `comeu` -> `comeram` [verb_form] The verb must agree with 'a gente' (third person): comeram.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eles estão comendo espagueti e bebendo vinho tinto no restaurante.` [content_mismatch] The answer does not describe the meal or the people eating it as specified in the task and scene facts.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=true answers_the_question=false

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Se eu ter`

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para` -> `que` [word_choice] The conjunction 'para' is incorrect here; 'para que' or 'de modo que' is needed to introduce a purpose clause.

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `começou` -> `começou` [verb_form] The verb 'começar' is intransitive and does not require the prefix 're-' for the past perfect tense.
- Missed: `tinha começou`

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] The verb must agree with 'você' in the third person singular: estude.
- Found (false positive): `filmes brasileiros` -> `filmes do Brasil` [word_order] The adjective should precede the noun for this type of description: filmes do Brasil.
- Missed: `assiste`
- Missed: `exige`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `na` -> `em` [article_preposition] The preposition 'em' is needed before the noun 'semana'.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morto` -> `muito cansado` [agreement] The adjective must agree in gender with the subject 'eu' (masculine).

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `deveria` -> `teria` [verb_form] The verb 'deveria' is in the conditional mood, but the context requires the past perfect tense ('teria') to indicate an action that was expected to happen in the past.

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu preocupo`

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-dress-boots (control, a1)

- Task: Describe the clothes Camila is wearing.
- Answer: A Camila está de vestido amarelo e botas marrons.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `marrons` -> `marronas` [agreement] The adjective must agree in gender and number with 'botas' (feminine plural).

### a2-control-preterite-sunday (control, a2)

- Task: What did you do on Sunday?
- Answer: No domingo eu acordei tarde, fiz o almoço e depois assisti a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `almoço` -> `o` [article_preposition] The verb 'fazer' requires the preposition 'o' before the noun 'almoço'.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `passe` -> `passe` [verb_form] The verb 'passar' must be in the subjunctive mood after 'para que'.

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ia` -> `vou` [verb_form] The verb 'ir' should be in the future tense (vou) or present perfect (fui), not the imperfect 'ia', which is incorrect for this context.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje está chovendo muito e eu estou em casa.` -> `Meu nome é [Name].` [content_mismatch] The answer does not address the question about the learner's name.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `comprei` -> `acho que é bom` [word_choice] The verb 'comprar' does not match the topic of working from home; it means 'bought'.

## qwen3.5:4b / plain

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `manha`
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
- Found (false positive): `do` -> `de` [article_preposition] The preposition 'de' is required to indicate origin, not the contraction 'do'.
- Found (changes nothing): `Toronto` -> `toronto` [spelling] City names in Portuguese do not require capital letters.
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
- Found (changes nothing): `vermelha` -> `vermelha` [agreement] The adjective must agree in gender with the noun 'camisa' (feminine), so it should be 'vermelha', but the learner wrote 'vermelha' which is correct; however, the original text says 'vermelha' which matches 'camisa'. Wait, let me re-read. The answer says 'uma vermelha camisa'. 'Camisa' is feminine, so 'vermelha' is correct. There is no mistake here.
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
- Found (false positive): `gosto` -> `comi` [content_mismatch] The verb 'gostar' is used incorrectly; the response should describe what was eaten for breakfast.
- Found (false positive): `futebol` -> `um pão` [content_mismatch] The word 'futebol' (soccer) is irrelevant to the question about breakfast food.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `à` -> `a` [spelling] The accent mark on 'a' is incorrect; it should be an acute accent (á) to indicate stress in this context.
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
- Found (false positive): `No` -> `Em` [word_choice] The word 'no' is a false friend for English 'on'; the correct preposition is 'em'.
- Missed: `volti`

### a2-verb-form-subject-agreement (erroneous, a2)

- Task: What did your family do on Sunday?
- Answer: No domingo minha família almoçaram na casa da minha avó.
- Reference: `almoçaram` -> almoçou [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `almoçaram` -> `almoçaram` [verb_form] The verb 'almoçar' must agree with the plural subject 'família', requiring the third-person plural form 'almoçaram'.
- Missed: `almoçaram`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `era` -> `estava` [ser_estar] The verb 'ser' is used incorrectly here; the past continuous tense requires 'estar'.
- Found (changes nothing): `cansado` -> `cansado` [agreement] Adjectives describing a state must agree in gender with the subject 'eu', which is masculine.

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engracada` [agreement] Adjectives describing a female subject must agree in gender.
- Missed: `curtas`

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `proficiência` -> `nível` [word_choice] The word 'proficiência' is not commonly used in Brazilian Portuguese for language exams; 'nível' or 'certificação' is more natural.
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
- Found (false positive): `tomo` -> `como` [word_choice] The verb 'tomar' is used incorrectly for eating; the correct verb is 'comer'.
- Missed: `pequeno-almoço`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo` -> `chovendo` [verb_form] The verb 'fazer' is incorrectly conjugated; it should be 'chovendo' to match the fact that it is raining.
- Found (duplicate): `sol` -> `chuva` [content_mismatch] The word 'sol' contradicts the scene facts stating that it is raining and cold.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos` -> `They are eating spaghetti and drinking red wine.` [content_mismatch] This sentence does not describe the meal being eaten.

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Se eu ter`

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `tinha começou`

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] The verb 'estudar' must be conjugated in the third person singular ('estuda') to agree with the implied subject 'você'.
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `deveria` -> `devia` [verb_form] The verb 'deveria' is in the wrong tense for a past obligation that was not fulfilled.
- Missed: `sobre do`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=false answers_the_question=true

### a1-pronoun-reflexive-worry (erroneous, a1)

- Task: Do you worry about work?
- Answer: Sim, eu preocupo muito com o meu trabalho.
- Reference: `eu preocupo` -> eu me preocupo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu preocupo`

### a1-pronoun-possessive-pet (erroneous, a1)

- Task: Describe your pet.
- Answer: Eu tenho um cachorro. Mim cachorro se chama Bolt.
- Reference: `Mim cachorro` -> Meu cachorro [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `Mim` -> `Meu` [word_choice] The word 'mim' means 'me', but the intended meaning is 'my'.
- Found (false positive): `se chama` -> `chama` [verb_form] The verb 'chamar' requires the third-person singular form 'chama' when referring to a dog.

### a1-short-spelling-chocolate (erroneous, a1)

- Task: What do you like?
- Answer: Eu gosto de chocolat.
- Reference: `chocolat` -> chocolate [spelling]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `chocolat` -> `chocolate` [word_choice] The word 'chocolat' is not a valid Portuguese noun; the correct term for chocolate is 'chocolate'.

### a1-short-verb-morar (erroneous, a1)

- Task: Where do you live?
- Answer: Eu mora em Lisboa.
- Reference: `mora` -> moro [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Lisboa` -> `São Paulo` [content_mismatch] Lisbon is the capital of Portugal, not Brazil.
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

### a2-control-preterite-sunday (control, a2)

- Task: What did you do on Sunday?
- Answer: No domingo eu acordei tarde, fiz o almoço e depois assisti a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a` -> `ao` [article_preposition] The preposition 'a' is incorrect; it should be the contraction 'ao' (a + o) before a masculine noun.

### b1-control-para-que (control, b1)

- Task: Why do you study every day?
- Answer: Estudo todos os dias para que eu passe no exame.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `passe` -> `confer` [word_choice] The verb 'passar' requires the prefix 'con-' to mean 'to pass (an exam)', forming 'confer'.

### b1-control-habitual-past (control, b1)

- Task: Describe your childhood summers.
- Answer: Quando eu era pequeno, eu sempre ia à praia com meus avós.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ia` -> `via` [verb_form] The verb 'ir' must be conjugated as 'vai' for the first person singular in the past tense.

### a1-off-topic-rain (off_topic, a1)

- Task: What is your name?
- Answer: Hoje está chovendo muito e eu estou em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Hoje` -> `Meu nome é...` [content_mismatch] The answer does not address the question about the learner's name.

### b1-off-topic-bicycle (off_topic, b1)

- Task: What do you think about working from home?
- Answer: Eu comprei uma bicicleta nova no sábado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu comprei` -> `Acho que` [content_mismatch] The verb tense does not match the context of a general opinion about working from home.
