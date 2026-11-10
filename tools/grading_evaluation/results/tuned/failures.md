# Grading failures

Every call with a missed error, a false alarm, a quote not in the answer, a wrong flag, or an invalid response.

## aya-expanse:8b / few_shot

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
- Flags: written_in_portuguese=false answers_the_question=true
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
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `azul` -> `verde` [word_choice] The adjective does not match the described color of the jacket, which is green.

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo.` -> `Eu comi cereal, pão com manteiga ou um café da manhã típico.` [content_mismatch] This answer does not describe what the person ate for breakfast.

### a1-english-lunch (not_portuguese, a1)

- Task: What did you eat for lunch?
- Answer: I ate rice and chicken for lunch.
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
- Found (matched): `fazi` -> `fiz` [verb_form] The verb needs to be conjugated correctly: fiz.
- Found (false positive): `fui dormir cedo` -> `deitei cedo` [word_order] The order of words is a bit off. A more natural way to say this would be: 'deitei cedo' or 'dormi cedo'.

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
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `está chovendo e faz frio` [word_choice] The correct phrase would be 'está chovendo' given the scene facts.
- Found (duplicate): `muito calor` -> `frio` [content_mismatch] The weather is described as cold, not hot.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eles estão jantando espaguete e bebendo vinho tinto.` [content_mismatch] This is not a description of the meal, but a personal statement about the speaker's residence and preferences.

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
- Found (quote not in the answer): `1750` -> `1750` [spelling] The year needs to be corrected to 1750.
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

## aya-expanse:8b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maiô` -> `maiô` [spelling] The correct word for 'swimsuit' in this context is 'maiô' (not 'maió' which is a mistake in conjugation).

### a1-control-coffee (control, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Sim, eu gosto de café.` -> `Sim, eu gosto de café.` [spelling] The sentence is grammatically correct.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Só um pouco cansado.` -> `Estou um pouco cansado.` [verb_form] While 'cansado' means 'tired', the phrase should use 'estou' (I am) for the first-person present tense.

### a1-control-daily-routine (control, a1)

- Task: What do you do every day?
- Answer: Todo dia eu acordo cedo, tomo café e vou para o trabalho.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `acordo cedo` -> `acordo` [verb_form] The correct verb conjugation should be 'acordo' (I wake up).

### a1-control-order-meal (control, a1)

- Task: You are at a restaurant. Order your meal.
- Answer: Eu queria uma salada e um suco de laranja, por favor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `uma salada e um suco de laranja` -> `uma salada e um suco de laranja natural` [word_order] This is correct, but for a more polite order, it's better to say 'uma salada e um suco de laranja natural' (a salad and a natural orange juice).

### a1-control-family (control, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos e uma irmã. Meu pai é médico e minha mãe é professora.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu tenho dois irmãos e uma irmã.` -> `Eu tenho dois irmãos e uma irmã.` [agreement] No A1 level, simple subject-verb agreement is expected. The correct form would be 'Eu tenho dois irmãos e uma irmã.'

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu me chamo Ana e moro em Curitiba.` -> `Meu nome é Ana e eu moro em Curitiba.` [word_order] The sentence is grammatically correct, but it's a bit formal for a casual conversation at the A1 level.

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `feijão com arroz` -> `meu prato favorito é feijão com arroz` [word_order] This is correct, but for a more natural expression, 'meu prato favorito' (my favorite dish) could be used.
- Missed: `fejão`

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha eu bebo cafe com leite.` -> `Eu bebo café com leite de manhã.` [word_order] The phrase is grammatically correct, but it can be simplified.
- Missed: `cafe`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `escritório` -> `escritório` [spelling] The correct spelling is 'escritório' (office).
- Missed: `uma escritório`

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `Eu sou do Canadá e moro Toronto.` -> `Eu sou do Canadá.` [content_mismatch] The sentence is in Portuguese, but it should be in response to a question asked in Portuguese.

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ela tem trinta anos` -> `ela tem trinta anos de idade.` [word_order] The correct way to say 'she is thirty years old' would be 'ela tem trinta anos de idade'.
- Missed: `chama`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `No almoço eu como arroz, feijão e frango.` -> `No almoço eu como arroz, feijão e frango.` [article_preposition] The correct preposition to use with 'frango' (chicken) in this context is 'e' (and), not 'feijão' (bean).
- Missed: `beans`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo.` -> `Comi uma torrada com manteiga e café.` [content_mismatch] This sentence does not answer the question 'What did you eat for breakfast?',

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de manhã e à tarde` -> `de manhã até à tarde` [word_order] This is acceptable, but 'de manhã' and 'à tarde' can be combined into 'de manhã até à tarde' for better flow.
- Missed: `portugues`

### a2-spelling-agua (erroneous, a2)

- Task: What did you buy at the market?
- Answer: No mercado eu comprei maçãs, dois quilos de arroz e uma garrafa de agua.
- Reference: `agua` -> água [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maçãs` -> `maçãs` [spelling] The correct word is 'maçãs' (apples).
- Found (matched): `dois quilos de arroz e uma garrafa de agua` -> `dois quilos de arroz, uma garrafa de água` [word_order] The order of the items should be consistent with the sentence structure.

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tomei cafe` -> `tomei café` [spelling] The correct phrase would be 'tomei café' (with an accent on the final 'e').
- Missed: `onibus`
- Missed: `as nove`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No fim de semana eu visitei meus avós e nós almoçamos juntos.` -> `No fim de semana eu visitei meus avós e eu almocei com eles.` [word_order] The phrase 'nós almoçamos juntos' is grammatically correct, but 'visitei meus avós e nós' is awkward. It should be 'visitei meus avós e eu almocei com eles.'
- Missed: `volti`

### a2-verb-form-imperfect (erroneous, a2)

- Task: Describe your childhood.
- Answer: Quando eu era criança, eu moro numa cidade pequena e brincava na rua.
- Reference: `moro` -> morava [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `morlo` -> `Quando eu era criança, eu morava numa cidade pequena e brincava na rua.` [verb_form] The correct verb conjugation should be 'morar', not 'morlo'.
- Missed: `moro`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The correct word should be 'engraçada' to agree with the gender of the subject.
- Missed: `curtas`

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `De manhã eu faço a cama, tomo banho e lavo os dentes.` -> `De manhã, eu faço a cama, tomo banho e lavo os dentes.` [word_order] The sentence is grammatically correct, but it can be improved for flow and naturalness.
- Missed: `lavo os dentes`

### a2-pronoun-reciprocal (erroneous, a2)

- Task: Describe your relationship with your neighbors.
- Answer: Eu me dou bem com meus vizinhos e sempre nós ajudamos quando precisamos.
- Reference: `nós ajudamos` -> nos ajudamos [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `meus vizinhos` -> `meus vizinhos(as)` [agreement] The correct form should be 'meus vizinhos(as)' to include both male and female neighbors.
- Missed: `nós ajudamos`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tomo` -> `como` [verb_form] The correct verb conjugation should be 'como' for 'eat'.
- Missed: `pequeno-almoço`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Hoje está fazendo sol e muito calor.` -> `Hoje está chovendo e faz frio.` [content_mismatch] The answer contradicts the given scene facts, stating that it is sunny and hot instead of rainy and cold.

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No ano passado eu viajei para o Rio de Janeiro com minha irmã.` -> `No ano passado, viajei para o Rio de Janeiro com minha irmã.` [verb_form] The correct verb conjugation should be 'Viajei' in the past tense.

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No sábado vou visitar meus avós e no domingo vou ao mercado.` -> `No sábado visito meus avós e no domingo vou ao mercado.` [word_order] The sentence is grammatically correct, but it could be more natural to say 'No sábado visito meus avós e no domingo vou ao mercado.' to separate the actions better.

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `o seu é mais barato` -> `seu é mais barato` [verb_form] The correct phrase should use 'é' instead of 'é mais'.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vire à esquerda no banco` -> `vire à esquerda *em* o banco` [article_preposition] The correct phrase should use 'vire à esquerda *em* o banco' to mean 'turn left at the bank'.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.` -> `Nós fomos ao cinema e depois comemos pizza num restaurante perto de casa.` [word_order] The sentence is grammatically correct, but it can be more natural to say 'Nós fomos' instead of 'A gente foi' to refer to a specific group including the speaker and their friends.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `gostei muito dele` -> `gostei muito dele.` [word_order] The correct expression would be 'gostei muito dele' (I liked it very much).

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.` -> `Nos meus momentos de lazer, eu gosto de ler, cozinhar e dar uma volta com o meu cachorro.` [word_order] The sentence is grammatically correct, but it can be improved for natural flow in Brazilian Portuguese.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Estou comendo spaghetti e bebendo vinho tinto.` [content_mismatch] This sentence does not describe the meal, but rather talks about the speaker's residence and liking for São Paulo.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-agreement-passive (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1750 e foi restaurado em 2005.
- Reference: `restaurado` -> restaurada [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `restorado` -> `restaurada` [verb_form] The correct verb form should be 'restaurada' (feminine singular past participle of 'restaurar') as it refers to the cathedral, a feminine noun.
- Missed: `restaurado`

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Espero que você possa vir à festa e que traga seus amigos.` -> `Venha à festa com seus amigos! Seria ótimo ter vocês lá.` [content_mismatch] The sentence is grammatically correct, but it can be more friendly and direct for an invitation.

### b1-control-future-subjunctive (control, b1)

- Task: What will you do when you finish the course?
- Answer: Quando eu terminar o curso, vou procurar um emprego novo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `procurar um emprego novo` -> `procurar um novo emprego` [word_order] The correct phrase would be 'procurar um novo emprego' to maintain grammatical correctness.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `e viajaria pelo mundo.` -> `e viajaria pelo mundo se tivesse a oportunidade.` [verb_form] The correct verb form should be 'viajaria' (I would travel) to match the conditional tone of the sentence.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós podermos` -> `para que pudéssemos escolher uma boa mesa.` [verb_form] This phrase is grammatically incorrect. The correct way to express the idea of 'so we could' would be 'para que pudéssemos'.

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a gente` -> `eu` [pronoun] The correct pronoun in this context should be 'eu' (I) or 'nós' (we), not 'a gente'.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `um dia de folga` -> `um dia de folga na próxima sexta-feira` [word_choice] The correct phrase is 'um dia de folga' (a day off).

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estude` [verb_form] The correct verb conjugation should be 'estude' (you study) to match the subject 'você' (you).
- Missed: `assiste`
- Missed: `exige`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `passagens` -> `bilhetes` [word_choice] The correct word here is 'bilhetes' (tickets).

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `apagar vários incêndios` -> `apagar vários incêndios no trabalho` [word_choice] The correct expression to describe putting out fires would be 'apagar incêndios' in this context.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `reclamar do atraso na entrega do meu pedido` -> `reclamar da demora na entrega do meu pedido` [verb_form] The correct verb conjugation should be 'reclamar da demora' (complain about the delay).

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe disse` -> `disse a ele` [article_preposition] The correct preposition to use with 'disse' (said) is 'a' (to), not 'lhe'.

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
- Found (false positive): `escritório` -> `escritório` [spelling] This word needs an accent mark.
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
- Found (false positive): `às` -> `às` [article_preposition] The correct preposition to use with the time is 'à'.

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
- Found (false positive): `muito` -> `muito` [spelling] The correct way to express 'very much' after 'gostar' is 'muito' without an accent.
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
- Found (false positive): `nós ajudamos` -> `nós ajudamos` [agreement] The verb 'to help' needs to agree with the subject 'we' in this context.
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

## gemma3:1b / few_shot

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
- Missed: `em o`

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

### a1-english-lunch (not_portuguese, a1)

- Task: What did you eat for lunch?
- Answer: I ate rice and chicken for lunch.
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

## gemma3:1b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `maiô` -> `roupa` [spelling] The word 'maiô' means 'swimsuit', not 'red swimsuit'.

### a1-control-coffee (control, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `gosto` -> `gosto` [verb_form] The verb 'gostar' is used for liking, not 'to like'.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `tô` -> `tô` [spelling] This is a misspelling of 'tô'.

### a1-control-daily-routine (control, a1)

- Task: What do you do every day?
- Answer: Todo dia eu acordo cedo, tomo café e vou para o trabalho.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `tomo` -> `tomar` [verb_form] The verb 'tomar' means 'to drink'. 'Vou' means 'I go'. You need to use 'tomar' to drink coffee.

### a1-control-order-meal (control, a1)

- Task: You are at a restaurant. Order your meal.
- Answer: Eu queria uma salada e um suco de laranja, por favor.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `queria` -> `quisem` [verb_form] The verb 'querer' means 'to want'. 'Eu queria' is the correct form of the verb to express desire.

### a1-control-family (control, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos e uma irmã. Meu pai é médico e minha mãe é professora.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `Eu` -> `Eu` [pronoun] The correct form of 'I' is 'eu'.

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `Eu` -> `Eu` [word_choice] The verb 'me chamar' (to call me) is used for a person, not a thing. 'Eu' is the subject of the sentence.

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (quote not in the answer): `feijão` -> `arroz` [word_choice] ‘Feijão’ is a type of bean, not a food.
- Missed: `fejão`

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `de` -> `de` [spelling] The word 'de' should be 'de' to indicate possession.
- Missed: `manha`
- Missed: `cafe`

### a1-agreement-trousers (erroneous, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: Marina veste uma camiseta azul e uma calça pretos.
- Reference: `pretos` -> preta [agreement]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `veste` -> `usa` [verb_form] The correct verb is 'usa' (uses). 'Vestir' means 'to wear'.
- Missed: `pretos`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (matched): `escritório` -> `ofice` [spelling] The word 'escritório' means 'office' in Portuguese, not 'office'.

### a1-agreement-plural-adjective (erroneous, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos. Eles são muito simpático.
- Reference: `simpático` -> simpáticos [agreement]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `Eu` -> `Eu` [pronoun] The correct form of 'I' is 'eu'.
- Missed: `simpático`

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `Agora` -> `Agora` [verb_form] This is the correct form of the verb 'to be' in Portuguese. 'Agora' means 'now'.
- Missed: `sou em casa`

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `eu` -> `eu` [verb_form] The verb 'estar' is used for states and conditions, not for the origin of a person.
- Missed: `estou brasileiro`

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `cansado` -> `cansado` [spelling] This word means 'tired' in Portuguese, not 'exhausted' or 'exhausted'.
- Missed: `sou cansado`

### a1-article-contraction-em-o (erroneous, a1)

- Task: Where do you live?
- Answer: Eu moro em o Brasil, em São Paulo.
- Reference: `em o` -> no [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (matched): `o` -> `em` [spelling] The word 'o' should be 'em' because it indicates a location.

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `sou` -> `Eu estou` [verb_form] The verb 'ser' is used for permanent characteristics, not temporary locations. 'Eu sou' means 'I am', not 'I am from'.
- Missed: `moro Toronto`

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (quote not in the answer): `à oito` -> `jantou` [verb_form] The verb 'jantar' (to eat) should be in the past tense, so 'jantou' (ate) instead of 'à oito' (at eight).
- Missed: `jantar`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (matched): `chamo` -> `eu` [verb_form] The verb 'chamar' (to call) is used for a person, not a thing. It should be 'eu chamo' (I call).

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `amiga` -> `amigo` [agreement] The word 'amiga' means 'friend' in Portuguese, but it’s used to mean 'woman'.
- Missed: `chama`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `almoço` -> `almoçar` [verb_form] The verb 'almoçar' is used for 'to eat lunch', not 'to eat'.
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `vermelha` -> `vermelho` [spelling] The word 'vermelha' means 'red'. 'Vermelho' is the correct word for 'red'.

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `muito` -> `muito` [word_choice] The verb 'gostar' is used for 'to like' but 'muito' is an excessive amount, which doesn’t fit the context.
- Missed: `gosto de café muito`

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `usa` -> `use` [verb_form] The verb 'use' is incorrect in this context.
- Missed: `azul`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `gosto` -> `gosto` [verb_form] The verb 'gostar' is used for liking, not 'to like'.

### a1-english-lunch (not_portuguese, a1)

- Task: What did you eat for lunch?
- Answer: I ate rice and chicken for lunch.
- Flags: written_in_portuguese=false answers_the_question=false

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `estudei` -> `estudii` [verb_form] The verb 'estudar' (to study) is used for a lesson or activity, not for simply studying.
- Missed: `portugues`

### a2-spelling-agua (erroneous, a2)

- Task: What did you buy at the market?
- Answer: No mercado eu comprei maçãs, dois quilos de arroz e uma garrafa de agua.
- Reference: `agua` -> água [spelling]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `comprei` -> `comprar` [verb_form] The verb 'comprei' (bought) is the correct form of the verb 'comprar' (to buy) in the past tense. 'No mercado eu comprei' is the correct way to say 'I bought at the market'.
- Missed: `agua`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (quote not in the answer): `tomar` -> `aprender` [word_choice] The verb 'tomar' means 'to drink'. 'Aprender' means 'to learn'.
- Missed: `cafe`
- Missed: `onibus`
- Missed: `as nove`

### a2-verb-form-irregular-fazer (erroneous, a2)

- Task: What did you do yesterday evening?
- Answer: Ontem à noite eu fazi o jantar e depois fui dormir cedo.
- Reference: `fazi` -> fiz [verb_form]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (matched): `fazi` -> `fez` [verb_form] The correct verb is 'fez' (past tense of 'fazer' - to do).

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `visitei` -> `visitamos` [spelling] The verb 'visitar' means 'to visit'. 'Ver
- Missed: `volti`

### a2-verb-form-imperfect (erroneous, a2)

- Task: Describe your childhood.
- Answer: Quando eu era criança, eu moro numa cidade pequena e brincava na rua.
- Reference: `moro` -> morava [verb_form]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (matched): `moro` -> `live` [verb_form] The verb 'morar' means 'to live', not 'to live in'.

### a2-verb-form-subject-agreement (erroneous, a2)

- Task: What did your family do on Sunday?
- Answer: No domingo minha família almoçaram na casa da minha avó.
- Reference: `almoçaram` -> almoçou [verb_form]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `no` -> `na` [spelling] The verb 'no' is used to indicate 'not'. It should be 'na' before 'casa'.
- Missed: `almoçaram`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `fiquei` -> `dormi` [verb_form] The correct verb is 'dormi'. 'Fiquei' means 'I stayed'.
- Missed: `era cansado`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `muito` -> `very` [spelling] It should be 'very'
- Missed: `engraçado`
- Missed: `curtas`

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `estudo` -> `estudar` [verb_form] The verb 'estudar' is used for 'to study', not 'to be studying'.
- Missed: `por passar`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (matched): `atendi` -> `estudei` [verb_form] This verb means 'to attend' or 'to watch'. It’s the wrong verb form for 'to watch a movie'.

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (quote not in the answer): `foco` -> `focus` [word_choice] This means 'focus'. It’s not what you’re doing.
- Missed: `lavo os dentes`

### a2-pronoun-reciprocal (erroneous, a2)

- Task: Describe your relationship with your neighbors.
- Answer: Eu me dou bem com meus vizinhos e sempre nós ajudamos quando precisamos.
- Reference: `nós ajudamos` -> nos ajudamos [pronoun]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `me` -> `eu` [verb_form] The verb 'me' should be 'eu' because it's used as a subject.
- Missed: `nós ajudamos`

### a2-not-brazilian-gerund (erroneous, a2)

- Task: What are you doing now?
- Answer: Estou a comer pão com queijo na cozinha.
- Reference: `a comer` -> comendo [not_brazilian]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `estou` -> `estou a comer` [verb_form] The verb 'estar' is used for temporary states, not continuous actions. 'Estou a comer' is the correct form.
- Missed: `a comer`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `autocarro` -> `autocarro` [word_choice] ‘Autocarro’ means ‘bus’ in Portuguese, not ‘coach’.
- Missed: `autocarro`

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (quote not in the answer): `tomar` -> `comer` [verb_form] This verb means 'to take' or 'to eat'. It’s not the correct verb for 'breakfast'.
- Missed: `pequeno-almoço`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (matched): `fazendo` -> `está` [verb_form] This verb is used to describe an action that is happening now, but it’s not the correct verb to use when describing the weather.

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (quote not in the answer): `viajamos` -> `viajamos` [verb_form] The verb 'viajar' (to travel) is used correctly, but 'viajamos' (we traveled) is incorrect.

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (quote not in the answer): `vai` -> `vouro` [verb_form] The verb 'ir' (to go) is used for a future action, but 'vai' is used to indicate a future action in the present tense. It should be 'vouro' or 'vou'.

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `seu` -> `o` [agreement] The word 'seu' is a possessive pronoun and should be 'o' before 'seu'.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `siga` -> `siga em frente` [verb_form] This is the wrong verb form to use. It should be 'siga em frente'.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `gente` -> `pessoas` [agreement] ‘Gente’ is a plural noun, so it should be ‘pessoas’ (people) to agree with ‘comer’ (to eat).

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `gostei` -> `gostei de` [verb_form] This verb is used to express a feeling or opinion, not a simple statement of liking.

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `gosto` -> `gosto` [verb_form] The verb 'gostar' means 'to like', so 'gosto' is incorrect.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (quote not in the answer): `morou` -> `vive` [verb_form] The verb 'morar' means 'to live', not 'to live in'.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

### b1-verb-form-subjunctive-present (erroneous, b1)

- Task: What do you hope for the future?
- Answer: Espero que meus filhos são felizes e que eles tenham uma boa educação.
- Reference: `são` -> sejam [verb_form]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `Espero` -> `Espero que` [word_choice] This is the beginning of a phrase, not a question.
- Missed: `são`

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (matched): `ter` -> `ir` [verb_form] The verb 'ter' means 'to have'. 'Viajar' means 'to travel'. They are not interchangeable here.

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `comprar` -> `ganhar` [verb_form] The verb 'comprar' means 'to buy'. 'Ganhar' means 'to win'. The correct verb to use is 'ganhar' to indicate winning.
- Missed: `ganhava`

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `eu` -> `Eu` [verb_form] The verb 'morar' (to live) is conjugated to 'morro' (I live) in the present tense. 'Eu' is the first person singular pronoun.
- Found (false positive): `desde` -> `desde` [word_choice] The word 'desde' is used to indicate a duration of time. It is not needed here.
- Found (false positive): `2019` -> `2019` [word_choice] The number 2019 is not a correct way to express the duration of time.
- Missed: `tenho visitado`

### b1-agreement-passive (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1750 e foi restaurado em 2005.
- Reference: `restaurado` -> restaurada [agreement]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `construída` -> `built` [verb_form] The verb 'construir' (to build) is needed here, not 'feita' (made).
- Missed: `restaurado`

### b1-content-date (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1850 e restaurada em 2005.
- Reference: `1850` -> 1750 [content_mismatch]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `construída` -> `built` [spelling] The verb 'construir' means 'to build'. 'Construir' is the correct verb for 'to build' a building.
- Missed: `1850`

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `Espero` -> `Espero que você possa vir` [verb_form] This is a verb used to express a wish or hope. It’s not the correct phrase to introduce an invitation.

### b1-control-future-subjunctive (control, b1)

- Task: What will you do when you finish the course?
- Answer: Quando eu terminar o curso, vou procurar um emprego novo.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `vou` -> `terminar` [verb_form] The verb 'vou' is used for 'going to' but 'terminar' is the correct verb for 'to finish'.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `comprar` -> `gain` [verb_form] The verb 'comprar' means 'to buy'. 'Ganhar' means 'to win'. The correct verb to use is 'ganhar' to indicate winning.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `podermos` -> `poderíamos` [agreement] The verb 'podermos' is the subjunctive form of 'poder' and is used to express a hypothetical or unreal situation. It’s not appropriate here.

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `gente` -> `pessoas` [agreement] The word 'gente' means 'people'. It’s a common word, but it’s not the correct word to use here.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `gostaria` -> `to like` [verb_form] The verb 'gostar' is used for 'to like'. 'Gostar' is used for 'to like' in English.

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `cheguei` -> `chegar` [verb_form] The verb 'chegar' means 'to arrive'. 'Chegar' is the correct verb to use when you arrive.
- Missed: `tinha começou`

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `recomendo` -> `recomendar` [verb_form] The verb 'recomendar' means 'to recommend'. 'Recomendar' is the correct verb.
- Missed: `estuda`
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (matched): `do` -> `de` [spelling] The word 'do' is used incorrectly in this context. It should be 'de' to indicate the time.

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `já` -> `já que` [verb_form] The verb 'já' means 'already' but is used to indicate completion. It should be 'já que' to indicate 'that' or 'because'.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `complicado` -> `simple` [spelling] The verb 'apagar' means 'to extinguish', not 'to complicate'.
- Found (false positive): `morto de cansaço` -> `chegar` [verb_form] The verb 'chegar' is used to describe a state of being, not to describe a feeling of exhaustion.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (quote not in the answer): `reclamer` -> `reclamar` [verb_form] The verb 'reclamer' is incorrect. It should be 'reclamar'.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `lhe` -> `ele` [agreement] The correct pronoun is 'ele' (he) instead of 'lhe' (him).

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
- Found (false positive): `escritório` -> `escritório` [spelling] The word needs an accent on the i: escritório.
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
- Found (false positive): `vermelha` -> `vermelha` [agreement] The adjective needs to be masculine singular to agree with 'camisa'.
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
- Found (false positive): `à tarde` -> `à tarde` [word_choice] The correct expression is 'à tarde'
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
- Found (false positive): `engraçado` -> `engraçado` [agreement] The adjective needs to agree in gender with 'amiga'.
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
- Found (false positive): `autocarro` -> `autocarro` [spelling] The word needs an accent on the ‘o’: autocarro.
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
- Found (false positive): `ganhasse` -> `ganhasse` [verb_form] The conditional form needs to be 'ganhasse'.

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
- Found (false positive): `do meu pedido` -> `do meu pedido` [word_choice] The phrase 'do meu pedido' is redundant and less formal in this context.
- Missed: `sobre do`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajaria` [verb_form] The future tense needs to be expressed correctly with 'viajaria'.

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
- Found (false positive): `e você?` -> `e você?` [word_choice] The word order is correct, but the question is a bit unnatural.

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
- Found (false positive): `vermelha` -> `vermelha` [agreement] The adjective 'vermelha' is feminine singular, but 'camisa' is masculine singular.
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
- Found (false positive): `trabalhei` -> `trabalhei` [not_brazilian] The verb 'trabalhar' is used in European Portuguese, while 'trabalhei' is used in Brazilian Portuguese.
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
- Found (false positive): `almoçaram` -> `almoçaram` [verb_form] The verb 'almoçar' needs to agree with the subject 'minha família' (plural).
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
- Found (false positive): `engraçado` -> `engraçado` [agreement] The adjective 'engraçado' needs to be masculine to agree with 'amiga'.
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
- Found (false positive): `Eu me dou` -> `Eu me dou` [verb_form] The reflexive pronoun 'me' needs to be combined with 'ar' or 'er' to form the reflexive verb 'me dou'.
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
- Found (false positive): `No` -> `No` [article_preposition] ‘No’ is not the correct way to start the sentence in this context.

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
- Found (false positive): `eles` -> `eles` [word_choice] The pronoun 'eles' is redundant as 'meus filhos' already refers to the children.
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
- Found (false positive): `ganhasse` -> `ganhasse` [verb_form] The conditional form of 'ganhar' (to win) needs the subjunctive mood after 'se'.

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
- Found (false positive): `do atraso` -> `do atraso` [word_choice] ‘Do atraso’ is not a correct phrase in this context; ‘do atraso’ is a noun, not a correct way to express ‘of the delay’.
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

## llama3.1:8b / few_shot

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `um pouco cansado` -> `` [word_order] The phrase is not necessary and the sentence is already complete.

### a1-control-daily-routine (control, a1)

- Task: What do you do every day?
- Answer: Todo dia eu acordo cedo, tomo café e vou para o trabalho.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `acorda` -> `acordo` [verb_form] The verb 'acordar' is in the wrong form. It should be 'acordo' in the present tense.

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `me` -> `` [verb_form] The pronoun 'me' is used as an object pronoun, but in this sentence, it should be 'chamo' without a pronoun, as it's a verb conjugation.

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
- Found (false positive): `calça` -> `calça` [spelling] The word needs an accent on the a: calça.
- Missed: `pretos`

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sou em casa`

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `com fome` -> `` [word_choice] You are tired, not hungry.
- Missed: `sou cansado`

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `do Canadá` -> `de` [article_preposition] The preposition 'do' is not used in this context, use 'de' instead.
- Missed: `moro Toronto`

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `jantar` -> `jantar` [word_choice] The verb 'jantar' is used for eating at night, but it's not the correct verb to use in this context. The correct verb is 'comer' or 'almoçar' (if it's a light meal), but in this case, 'jantar' is not wrong, but 'assistir televisão' is not the correct verb to use, 'ver televisão' is more common.
- Missed: `jantar`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vinte anos` -> `vinte anos` [word_choice] The correct phrase is 'vinte anos de idade' or simply 'vinte anos' without 'de idade'.
- Missed: `chamo`

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `chama` -> `chama` [verb_form] The verb 'chamar' is in the infinitive form, it should be in the present tense: chama or chama-se.
- Missed: `chama`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vermelha` -> `vermelha` [agreement] The adjective must agree with the noun 'camisa' in gender.
- Found (false positive): `uma` -> `` [article_preposition] The article 'uma' is not necessary before 'calça' because it is a countable noun.
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
- Found (false positive): `jaqueta` -> `verde` [word_choice] The color of Rafael's jacket is green, not blue.
- Missed: `azul`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `gostei` [verb_form] The verb 'gosto' is in the present tense, but the question is about the past.

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `trabalhei` -> `estou estudando` [verb_form] The verb 'trabalhei' is in the preterite tense, but 'estudei' is in the preterite tense too, so the verb 'estudei' should be in the present perfect tense, which is 'estou estudando' or 'estudei' is correct but the sentence is in the wrong tense, it should be 'Ontem eu trabalhei de manhã e à tarde eu estou estudando português'.
- Missed: `portugues`

### a2-spelling-agua (erroneous, a2)

- Task: What did you buy at the market?
- Answer: No mercado eu comprei maçãs, dois quilos de arroz e uma garrafa de agua.
- Reference: `agua` -> água [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `arroz` -> `arroz` [spelling] The word needs an accent on the 'o': arroz.
- Missed: `agua`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `tomei cafe` -> `tomei café` [verb_form] The verb 'tomar' is in the infinitive form, it should be conjugated to 'tomei' (first person singular of the preterite tense).
- Missed: `onibus`
- Missed: `as nove`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Response: invalid or missing

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `estava` -> `estava` [verb_form] The verb 'estava' is in the present tense, but the sentence is describing a past situation.
- Missed: `era cansado`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [agreement] The adjective must agree with the subject in gender: 'amiga' is feminine, so the adjective should be 'engraçada'.
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
- Missed: `pequeno-almoço`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `sol e muito calor` -> `` [content_mismatch] The weather is described as cold and rainy, so this statement contradicts the facts.

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No ano passado` -> `` [word_order] The phrase is not necessary and can be omitted.

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `meus avós` -> `meus avôs` [agreement] The correct phrase is 'meus avôs' because avós is a feminine noun and the adjective 'meus' agrees with it.

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `mais barato` -> `é mais barato` [word_order] The correct phrase is 'é mais barato' instead of 'é mais' before 'barato'.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no banco` -> `à esquerda` [article_preposition] The preposition 'no' is not needed here, use 'à esquerda' instead.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A gente` -> `nós` [not_brazilian] At this level, the learner should use the more formal 'nós' instead of the informal 'a gente'.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> `dela` [agreement] The pronoun 'dele' is masculine and should be 'dela' to agree with the feminine noun 'filme'.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=true answers_the_question=false

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Response: invalid or missing

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
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `restaurado`

### b1-content-date (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1850 e restaurada em 2005.
- Reference: `1850` -> 1750 [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `construída` -> `foi construída` [verb_form] The verb tense is incorrect; the correct tense for a past event is the preterite, but the correct year is also incorrect.
- Missed: `1850`

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ganhasse` -> `ganhasse` [verb_form] The verb 'ganhar' is in the wrong form for the subjunctive mood.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `` [verb_form] The pronoun 'nós' is not necessary in this sentence, and the verb conjugation is also incorrect.

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a gente` -> `nós` [word_choice] The pronoun 'gente' is not used in this context, use 'nós' instead.

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `começou` -> `começou` [verb_form] The verb must be in the third person singular, so it should be 'começou' instead of 'começou' (which is the first person singular).
- Missed: `tinha começou`

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `assistes` -> `assista` [verb_form] The verb 'assistir' is in the wrong form for the subject 'você'.
- Missed: `estuda`
- Missed: `assiste`
- Missed: `exige`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajarei` [verb_form] The verb tense is incorrect; the correct tense is the future perfect, which is already used with 'tinha'.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morto de cansaço` -> `exausto` [not_brazilian] The expression is not typically used in Brazilian Portuguese; a more common expression would be 'exausto' or 'exaurido'

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dez` -> `décimo` [word_choice] The correct way to say 'tenth' in Portuguese is 'décimo'.

## llama3.1:8b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Response: invalid or missing

### a1-control-coffee (control, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `gosto` -> `gosto` [verb_form] The verb should be in the present tense, but 'gosto' is the first person singular of the present perfect tense.
- Found (false positive): `de café` -> `café` [article_preposition] The preposition 'de' is not needed here.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Só um pouco cansado` -> `` [content_mismatch] This is not a complete answer to the question.

### a1-control-daily-routine (control, a1)

- Task: What do you do every day?
- Answer: Todo dia eu acordo cedo, tomo café e vou para o trabalho.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `acordo` -> `acordo` [verb_form] The verb 'acordar' is not in the correct form for the subject 'eu'.
- Found (false positive): `para` -> `` [article_preposition] The preposition 'para' is not needed here.

### a1-control-order-meal (control, a1)

- Task: You are at a restaurant. Order your meal.
- Answer: Eu queria uma salada e um suco de laranja, por favor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `suco de laranja` -> `suco de laranja natural` [word_choice] The correct phrase is 'suco de laranja' but in a restaurant, it's more common to order 'suco de laranja natural' or just 'suco de laranja' without 'de laranja'.

### a1-control-family (control, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos e uma irmã. Meu pai é médico e minha mãe é professora.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dois irmãos` -> `duas irmãs` [agreement] The word 'dois' is masculine and should be 'duas' for 'irrmãs'.
- Found (false positive): `médico` -> `um médico` [ser_estar] The verb 'ser' is used instead of 'estar' to describe a profession.
- Found (false positive): `professora` -> `uma professora` [ser_estar] The verb 'ser' is used instead of 'estar' to describe a profession.

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `me` -> `` [verb_form] The verb 'chamar' is an intransitive verb, so it doesn't take a reflexive pronoun.
- Found (false positive): `e` -> `` [article_preposition] The conjunction 'e' is not needed here.

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `De manha` -> `À manhã` [word_order] The phrase 'De manha' is not grammatically correct in this context.
- Found (false positive): `com leite` -> `com leite é` [word_choice] The phrase 'com leite' is not the correct way to say 'with milk' in this context.
- Missed: `cafe`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `escritório` -> `escritório` [not_brazilian] The correct word is 'escritório' is not used in Brazil, 'escritório' is used in Portugal.
- Found (false positive): `no` -> `do` [article_preposition] The preposition 'no' is not used in this context.
- Missed: `uma escritório`

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estou` -> `sou` [verb_form] The verb 'estou' is in the present continuous tense, which is not suitable for a permanent characteristic like nationality.
- Found (false positive): `moro` -> `moro` [verb_form] The verb 'moro' is in the present continuous tense, which is not suitable for a permanent characteristic like residence.

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `sou` -> `estou` [verb_form] The verb 'sou' is a linking verb and doesn't match the adjective 'cansado'.
- Found (false positive): `com fome` -> `faminto` [word_choice] The correct phrase is 'com fome de comida' or simply 'faminto'.

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `do` -> `de` [article_preposition] Incorrect preposition to indicate origin.
- Found (matched): `moro` -> `morei` [verb_form] Incorrect verb form for the present tense.

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `jantar` -> `comer` [word_choice] The verb 'jantar' is used for eating at a restaurant, not at home.
- Found (false positive): `às oito` -> `oito` [article_preposition] The correct preposition to indicate the time is 'às' is not necessary here, 'oito' is enough.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vinte` -> `vinte e um` [word_choice] The number should be written in words for this level.
- Missed: `chamo`

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `chama` -> `se chama` [verb_form] The verb 'chama' is in the wrong form for the subject 'Maria'.
- Found (false positive): `ela tem trinta anos` -> `tem trinta anos` [verb_form] The verb 'tem' is in the wrong form for the subject 'Maria'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `almoço` -> `comem` [verb_form] The verb 'almoço' is in the first person singular, but the subject is 'eu' which is also in the first person singular, so it should be in the third person singular.
- Found (matched): `beans` -> `feijão` [not_brazilian] The word 'beans' is in English.

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `vermelha` -> `vermelho` [agreement] The adjective should agree with the noun 'camisa' in gender.
- Found (false positive): `uma calça` -> `calça` [article_preposition] The article 'uma' is not necessary before 'calça'.

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `gosto` -> `gosto` [verb_form] The verb 'gosto' is in the present tense, but the correct form for a simple statement in the present tense is 'gosto' for 'eu' but 'gostamos' would be used for 'we' and 'gostam' for 'they'.
- Found (matched): `muito` -> `` [word_choice] The adverb 'muito' is not necessary in this sentence.

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo` -> `` [content_mismatch] This sentence does not answer the question about breakfast.

### a1-english-lunch (not_portuguese, a1)

- Task: What did you eat for lunch?
- Answer: I ate rice and chicken for lunch.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `estudei` -> `estudei` [verb_form] The verb 'estudiar' is in the wrong form for the subject 'eu'.
- Found (false positive): `trabalhei` -> `trabalhei` [verb_form] The verb 'trabalhar' is in the wrong form for the subject 'eu'.
- Missed: `portugues`

### a2-spelling-agua (erroneous, a2)

- Task: What did you buy at the market?
- Answer: No mercado eu comprei maçãs, dois quilos de arroz e uma garrafa de agua.
- Reference: `agua` -> água [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maçãs` -> `maçã` [agreement] The correct form is 'maçã' for one apple, and 'maçãs' for multiple apples.
- Found (false positive): `dois quilos de arroz` -> `dois quilos de arroz` [word_order] The correct order is 'dois quilos de arroz' instead of 'de dois quilos de arroz'.
- Found (matched): `uma garrafa de agua` -> `água` [spelling] The correct form is 'água' instead of 'agua'.

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tomei` -> `tomou` [verb_form] The verb 'tomar' is in the infinitive form instead of the correct conjugation for the subject 'eu' in the present tense.
- Found (matched): `de onibus` -> `em ônibus` [article_preposition] The preposition 'de' is not needed before 'ônibus'.
- Missed: `cafe`
- Missed: `as nove`

### a2-verb-form-irregular-fazer (erroneous, a2)

- Task: What did you do yesterday evening?
- Answer: Ontem à noite eu fazi o jantar e depois fui dormir cedo.
- Reference: `fazi` -> fiz [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `fazi` -> `faz` [verb_form] The verb conjugation is incorrect.
- Found (false positive): `jantar` -> `jantar` [word_choice] The word choice is incorrect; the correct word is 'jantar' is not used in this context.
- Found (false positive): `cedo` -> `cedo` [word_choice] The word choice is incorrect; the correct word is 'cedo' is not used in this context.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `visitei` -> `visitei` [verb_form] The verb is in the third person singular, but the subject is 'eu' (I).
- Found (false positive): `meus avós` -> `minhas avós` [agreement] The adjective 'meus' agrees with the masculine noun 'avós', but 'avós' is a feminine noun.
- Found (false positive): `nós` -> `eu` [verb_form] The subject is 'eu', so the verb should be in the first person singular.
- Found (false positive): `almoçamos` -> `almocei` [verb_form] The verb is in the first person plural, but the subject is 'eu'.
- Found (matched): `Depois eu volti` -> `Depois eu voltei` [verb_form] The verb 'volti' is in the first person singular, but it should be in the first person singular of the verb 'voltar' in the past tense.
- Found (false positive): `para casa` -> `` [article_preposition] The preposition 'para' is not necessary in this context.
- Found (false positive): `No fim de semana` -> `` [content_mismatch] The phrase 'No fim de semana' is not necessary, as the context is clear.

### a2-verb-form-imperfect (erroneous, a2)

- Task: Describe your childhood.
- Answer: Quando eu era criança, eu moro numa cidade pequena e brincava na rua.
- Reference: `moro` -> morava [verb_form]
- Response: invalid or missing

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `exame de proficiência` -> `português` [word_choice] The correct term is 'exame de português' or 'exame de proficiência em português'.
- Missed: `por passar`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `vi` [verb_form] The verb 'atender' is not used in this context.
- Found (false positive): `um filme novo` -> `um filme novo` [agreement] The adjective 'novo' agrees with the masculine singular noun 'filme', but the article 'um' is used instead of 'um filme'.

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `faço` -> `estou fazendo` [verb_form] The verb 'faço' is in the present tense, but it should be in the present continuous tense to describe a routine.
- Missed: `lavo os dentes`

### a2-not-brazilian-gerund (erroneous, a2)

- Task: What are you doing now?
- Answer: Estou a comer pão com queijo na cozinha.
- Reference: `a comer` -> comendo [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `a` -> `` [article_preposition] The verb 'estou' is in the correct form, but the word 'a' is not needed here.
- Found (false positive): `com queijo` -> `` [word_choice] The phrase 'com queijo' is not necessary here, as 'pão com queijo' is a common combination.

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `to` -> `tenho` [verb_form] Incorrect verb conjugation for 'I have'.
- Missed: `pequeno-almoço`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `sol` -> `` [content_mismatch] It is raining, so the sun is not shining.
- Found (duplicate): `muito calor` -> `` [content_mismatch] It is cold, so it is not hot.

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `` [word_order] The correct form is 'No ano passado eu viajei para o Rio de Janeiro com minha irmã.' should be 'No ano passado eu viajei para o Rio de Janeiro com minha irmã.'
- Found (false positive): `Nós` -> `Eu` [agreement] The verb 'ficamos' is in the plural form, but 'eu' is in the singular form, so the subject-verb agreement is incorrect.
- Found (false positive): `perto da praia` -> `na` [word_choice] The correct phrase is 'perto da praia' should be 'na praia' or 'perto da praia' is correct but 'na praia' is more common.

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `meus` -> `minhas` [agreement] The adjective 'meus' is masculine and should be 'minhas' because 'avós' is feminine.

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `mais barato` -> `` [content_mismatch] The correct comparison is between the two phones, not between the phone and its price.
- Found (false positive): `do que` -> `que` [word_choice] This phrase is used for comparisons between two things, but in this case, we are comparing the age of one phone to the price of the other.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `em frente` -> `em frente` [spelling] The correct phrase is 'em frente' without accent mark.
- Found (false positive): `no banco` -> `à esquerda no banco` [word_order] The correct phrase is 'à esquerda no banco' or simply 'à esquerda'.
- Found (false positive): `fica` -> `está` [verb_form] The correct verb form is 'está' in the present tense.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A gente` -> `Nós` [verb_form] The verb conjugation is wrong for 'we'.
- Found (false positive): `perto de casa` -> `perto da casa` [article_preposition] The preposition 'de' is not needed here.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `gostei muito dele` -> `gostei muito` [verb_form] The verb 'gostei' is in the wrong form for the subject 'eu'.

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cozinhar` -> `cozinho` [verb_form] The verb 'cozinhar' is in the infinitive form, it should be in the present tense.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `` [content_mismatch] The answer does not describe the meal.

### b1-verb-form-subjunctive-present (erroneous, b1)

- Task: What do you hope for the future?
- Answer: Espero que meus filhos são felizes e que eles tenham uma boa educação.
- Reference: `são` -> sejam [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `meus` -> `meu` [agreement] The adjective 'meus' is masculine plural, but the noun 'filhos' is masculine singular.
- Found (false positive): `eles` -> `ele` [verb_form] The verb 'tenham' agrees with the subject 'eles', which is masculine plural, but the correct verb form should be 'tenham' -> 'tenham' is correct, but the subject is 'meus filhos', which is masculine singular.
- Missed: `são`

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ter tempo` -> `ter tempo` [word_choice] The verb 'ter' is not the correct choice here.
- Found (false positive): `no mês que vem` -> `em um mês` [word_choice] The phrase 'no mês que vem' is not the correct way to say 'next month'.
- Found (false positive): `vou viajar` -> `viajaria` [verb_form] The verb 'vou viajar' is in the future tense, but the question is about a hypothetical situation.
- Found (false positive): `para o Nordeste` -> `ao` [article_preposition] The correct preposition to use here is 'ao' instead of 'para'.
- Found (matched): `eu` -> `` [pronoun] The pronoun 'eu' is not necessary here, as the subject is implied.
- Found (false positive): `mês que vem` -> `um mês` [word_choice] The phrase 'mês que vem' is not the correct way to say 'next month'.
- Found (false positive): `Nordeste` -> `o` [article_preposition] The correct article to use here is 'o' instead of no.

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nunca` -> `não` [verb_form] This verb form is incorrect for the present tense.
- Found (matched): `visitado` -> `visito` [verb_form] The verb 'visitado' is in the preterite tense, but the correct form for the present tense is 'visito' or 'visito o Nordeste'.

### b1-agreement-passive (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1750 e foi restaurado em 2005.
- Reference: `restaurado` -> restaurada [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `foi construída` -> `foi construída` [verb_form] The verb 'construir' is in the wrong form for the subject 'catedral'.
- Found (matched): `foi restaurado` -> `foi restaurada` [verb_form] The verb 'restaurar' is in the wrong form for the subject 'catedral'.

### b1-content-date (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1850 e restaurada em 2005.
- Reference: `1850` -> 1750 [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `construída` -> `foi construída` [verb_form] The cathedral was built, so the correct verb form is 'foi construída'.
- Found (matched): `em 1850` -> `em 1750` [content_mismatch] The correct year of construction is 1750.

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Espero` -> `Espero` [verb_form] The verb 'esperar' is in the future tense, but the correct tense for an invitation is the present tense.
- Found (false positive): `seus` -> `seus` [agreement] The adjective 'seu' should agree with the noun 'amigos' in number.

### b1-control-future-subjunctive (control, b1)

- Task: What will you do when you finish the course?
- Answer: Quando eu terminar o curso, vou procurar um emprego novo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `procurar` -> `buscar` [word_choice] The verb 'procurar' is not the best choice for 'look for a job'.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ganhasse` -> `ganhas` [verb_form] The verb should be in the conditional tense, not the future tense.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `você` [agreement] The subject is 'you', not 'we'.

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a gente` -> `nós` [not_brazilian] The phrase 'a gente' is informal and typically used with 'pra' or 'nós', not in formal writing.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `gostaria` -> `gostaria` [verb_form] The verb 'gostaria' is in the conditional mood, but it should be in the present tense to express a desire or request.
- Found (false positive): `de` -> `` [article_preposition] The preposition 'de' is not needed here.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estuda` -> `estudo` [verb_form] The verb 'estuda' is in the third person singular, but it should be in the first person singular because it's a recommendation from the speaker.
- Found (quote not in the answer): `assistes` -> `assiste` [verb_form] The verb 'assistes' is in the second person singular, but it should be in the third person singular because it's a recommendation to a friend.
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Venho` -> `Vieram` [verb_form] The verb 'venho' is in the present tense, but the situation is in the past.
- Found (matched): `do` -> `` [article_preposition] The word 'do' is not needed here.

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajará` [verb_form] The verb tense does not match the future tense of 'will travel'.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `complicado` -> `estressante` [word_choice] The word 'complicado' is not typically used to describe a stressful day.
- Found (false positive): `morto de cansaço` -> `exausto` [word_choice] The phrase 'morto de cansaço' is an idiomatic expression, but it's more common to use 'exausto' or 'exaustão' in this context.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Venho` -> `Vinha` [verb_form] The verb 'venho' is in the present tense, but the situation is in the past.
- Found (false positive): `meu` -> `minha` [agreement] The adjective 'meu' agrees with the masculine singular subject, but the noun 'pedido' is feminine.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `me esperasse` -> `esperasse` [verb_form] The verb 'esperar' is in the wrong form for the subject 'eu'.
- Found (false positive): `me ouviu` -> `ouviu` [verb_form] The verb 'ouvir' is in the wrong form for the subject 'eu'.

## llama3.2:3b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de maiô` -> `de banho` [content_mismatch] The correct phrase is 'de banho' or 'de nadar', as 'maiô' is a specific type of swimsuit.

### a1-control-daily-routine (control, a1)

- Task: What do you do every day?
- Answer: Todo dia eu acordo cedo, tomo café e vou para o trabalho.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `acordo` -> `acordo` [verb_form] The verb needs to be conjugated to the present tense: acordo, acordo, acordamos, acordou.

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `comida` -> `prato` [word_choice] The word should be 'prato' or 'sabor' to express preference.
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
- Found (false positive): `veste` -> `tem` [ser_estar] The verb 'to wear' is not needed in this context.
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
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `simpático` -> `simpáticos` [agreement] The adjective must agree with the noun 'irmãos'. It should be 'simpáticos'.

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sou em casa`

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
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Canadá` -> `Canadá` [spelling] The country is not in Portuguese, the correct term is 'Canadá' but it should be written with an accent on the 'a': 'Canadá'.
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
- Found (false positive): `chama` -> `chama` [verb_form] The verb 'chamar' is not used to say 'calls'.
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
- Found (false positive): `calça` -> `calça` [agreement] In Portuguese, 'calça' is a masculine noun, so the feminine article 'a' should be used.
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
- Found (false positive): `uma` -> `um` [agreement] The word should be 'um' because Rafael is a masculine subject.
- Missed: `azul`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `futebol` -> `futebol` [word_choice] The correct word is 'futebol' (the sport), not 'futebol' (the team).

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `trabalhei` -> `trabalhei` [verb_form] The verb conjugation is incorrect, it should be 'trabalhei' (I worked) instead of 'trabalhei' (I worked).
- Missed: `portugues`

### a2-spelling-agua (erroneous, a2)

- Task: What did you buy at the market?
- Answer: No mercado eu comprei maçãs, dois quilos de arroz e uma garrafa de agua.
- Reference: `agua` -> água [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `arroz` -> `arroz` [spelling] The word needs an accent on the i: arroz.
- Missed: `agua`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `acordei` -> `acordei` [verb_form] The verb conjugation is incorrect, it should be 'acordei' (I woke up) instead of 'acordei' (I woke up).
- Missed: `cafe`
- Missed: `onibus`
- Missed: `as nove`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `visitei` -> `visitado` [verb_form] The verb should be in the past participle form to agree with the present perfect tense.
- Missed: `volti`

### a2-verb-form-imperfect (erroneous, a2)

- Task: Describe your childhood.
- Answer: Quando eu era criança, eu moro numa cidade pequena e brincava na rua.
- Reference: `moro` -> morava [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `moro` -> `morava` [verb_form] The verb conjugation is incorrect. The correct form is 'morava'.

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
- Found (false positive): `fiquei` -> `fiquei` [verb_form] The verb should be conjugated to the present tense, as it describes a general fact about the speaker.
- Missed: `era cansado`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [ser_estar] The correct word is 'engraçada', the feminine form of 'engraçado'.
- Missed: `curtas`

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `proficiência` -> `passar no exame de proficiência` [content_mismatch] The correct word is 'proficiência' but the correct answer would be 'passar no exame de proficiência'.
- Missed: `por passar`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `atendi` -> `assisti` [verb_form] The verb should be in the past participle form to agree with 'on

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=false
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
- Found (matched): `fazendo` -> `está` [ser_estar] The weather is not sunny, so 'fazendo' is incorrect. It should be 'está' instead.

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ao` -> `a` [article_preposition] The preposition needs to be 'a' instead of 'ao'.

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `barato` -> `caro` [agreement] The adjective must be masculine to agree with 'celular'.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vire` -> `vire à esquerda` [article_preposition] The correct preposition to use is 'vire à esquerda' instead of 'vire à esquerda no banco'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `São Paulo` -> `eles` [pronoun] The people being described are not the speaker, so the correct answer should be 'eles' (they) instead of 'eu'.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

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
- Found (false positive): `catedral` -> `catedral` [spelling] The correct word is 'catedral' with an accent on the 'e'.
- Missed: `1850`

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `seus` -> `seu` [agreement] The correct word is 'seu' (possessive adjective), not 'seus' (plural possessive adjective).

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `ganasse` -> `ganharia` [verb_form] The verb conjugation is incorrect, it should be 'ganharia'.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `nós` -> `eu` [not_brazilian] The subject-verb agreement is incorrect. The correct form is 'eu' or 'você' (you) for the first person singular, but 'eu' is used here, so it's correct.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Sr.` -> `Senhor` [not_brazilian] In Brazil, the formal title is 'Senhor' or 'Senhora', not 'Sr.'.

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cheguei` -> `chegado` [verb_form] The verb should be in the past perfect tense to indicate that it happened before another action in the past.
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

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `depois` -> `daqui a` [word_choice] The correct word is 'daqui a' instead of 'do' for 'from now on'.

## llama3.2:3b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de` -> `usando` [ser_estar] should be 'usando' to indicate she is wearing the clothes

### a1-control-coffee (control, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `gosto` -> `beijo` [word_choice] should be 'beijo' for 'like' in this context

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cansado` -> `esgotado` [word_choice] should be 'esgotado'

### a1-control-daily-routine (control, a1)

- Task: What do you do every day?
- Answer: Todo dia eu acordo cedo, tomo café e vou para o trabalho.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `acordo` -> `acordar` [verb_form] should be 'acordar'
- Found (false positive): `cedo` -> `cedo` [spelling] should be 'cedo' (correct spelling), but 'cedo' is correct in this context
- Found (false positive): `tomo` -> `tomar` [verb_form] should be 'tomar'
- Found (false positive): `vou` -> `vou` [verb_form] should be 'ir' (to go) or 'deixar' (to leave) depending on the context, but 'vou' is correct in this context

### a1-control-order-meal (control, a1)

- Task: You are at a restaurant. Order your meal.
- Answer: Eu queria uma salada e um suco de laranja, por favor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `suco de laranja` -> `suco de laranja` [content_mismatch] should be 'suco de laranja' but 'suco de laranja' is correct

### a1-control-family (control, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos e uma irmã. Meu pai é médico e minha mãe é professora.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `irmã` -> `irmãos` [agreement] should be 'irmãos' because the subject is plural

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `moro` -> `morro` [word_choice] should be 'morro'

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `bebo` -> `beo` [verb_form] should be 'beo' or 'bebo' is not wrong but 'bebo' is not the correct form of the verb 'beber' in this context
- Missed: `manha`
- Missed: `cafe`

### a1-agreement-trousers (erroneous, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: Marina veste uma camiseta azul e uma calça pretos.
- Reference: `pretos` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `veste` -> `usa` [verb_form] should be 'usa' because Marina is the subject of the sentence
- Found (matched): `pretos` -> `pretas` [agreement] should be 'pretas' because trousers are typically feminine

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `trabalho` -> `trabalho` [spelling] should be 'trabalho' with an acute accent
- Missed: `uma escritório`

### a1-agreement-plural-adjective (erroneous, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos. Eles são muito simpático.
- Reference: `simpático` -> simpáticos [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `simpático` -> `simpáticos` [word_choice] should be 'simpáticos' because the subject is plural

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `brasiliero` -> `brasileiro` [word_choice] should be 'brasileiro'
- Found (false positive): `moro` -> `morro` [word_order] should be 'morro'
- Missed: `estou brasileiro`

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `trabalho` -> `trabalho` [spelling] should be 'trabalho' (no accent mark)
- Missed: `sou cansado`

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Canadá` -> `Canadá é um país` [content_mismatch] Canada is a country, not a city.
- Found (matched): `moro` -> `vivo` [verb_form] should be 'morar' (to live) or 'vivo' (I live) in the present tense.

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `jantar` -> `jantar` [word_choice] should be 'jantar' (to eat) instead of 'jantar' (to go out to eat)
- Missed: `jantar`

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `chama` -> `se chama` [word_choice] should be 'se chama'
- Found (false positive): `e` -> `e` [article_preposition] should be 'e' (no space)
- Found (false positive): `ela` -> `ela` [article_preposition] should be 'ela' (no space)
- Found (false positive): `tem` -> `tem` [article_preposition] should be 'tem' (no space)
- Found (false positive): `trinta` -> `trinta e anos` [content_mismatch] should be 'trinta e' (and) or 'trinta anos' (and 'anos' for 'years')

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `almoço` -> `lanche` [content_mismatch] should be 'lanche' for 'lunch'
- Found (false positive): `e` -> `e` [word_choice] should be 'e' (conjunction) or 'e' (and) without space
- Found (false positive): `frango` -> `frango` [word_choice] should be 'frango' (male chicken) or 'frango' (chicken meat) without the 'o' ending
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `vermelha` -> `roxa` [word_choice] should be 'roxa'
- Found (false positive): `calça` -> `calças` [agreement] should be 'calças'

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `azul` -> `verde` [word_choice] should be green
- Found (false positive): `jaqueta` -> `jaqueta` [word_choice] should be jacket
- Found (false positive): `calça` -> `jeans` [word_choice] should be jeans
- Found (false positive): `tênis` -> `sneakers` [word_choice] should be sneakers

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `futebol` -> `comida` [word_choice] football is not typically eaten for breakfast

### a1-english-lunch (not_portuguese, a1)

- Task: What did you eat for lunch?
- Answer: I ate rice and chicken for lunch.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `trabalhei` -> `trabalhei` [verb_form] should be 'trabalhei' (present perfect tense) instead of 'trabalhei' (simple past tense)
- Found (false positive): `à tarde` -> `à tarde` [article_preposition] should be 'à tarde' (preposition) instead of 'à tarde' (adverb)
- Found (false positive): `estudei` -> `estudei` [verb_form] should be 'estudei' (present perfect tense) instead of 'estudei' (simple past tense)
- Found (matched): `portugues` -> `português` [agreement] should be 'português' (masculine noun) instead of 'portugues' (feminine noun)

### a2-spelling-agua (erroneous, a2)

- Task: What did you buy at the market?
- Answer: No mercado eu comprei maçãs, dois quilos de arroz e uma garrafa de agua.
- Reference: `agua` -> água [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `arroz` -> `arroz` [word_choice] should be 'rice'
- Found (false positive): `maçãs` -> `maçãs` [word_choice] should be 'apples'
- Found (false positive): `dois quilos de` -> `de dois quilos` [word_order] should be 'de dois quilos'
- Found (false positive): `garrafa de` -> `de uma garrafa` [word_order] should be 'de uma garrafa'
- Found (matched): `agua` -> `água` [word_choice] should be 'água'

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tarde` -> `tardia` [word_choice] should be 'tardia' because 'tarde' is an adjective, not an adverb
- Found (false positive): `tomei` -> `tomei` [verb_form] should be 'tomei' because 'tomar' is an irregular verb in the present tense, but 'tomei' is the correct form
- Found (false positive): `de` -> `` [article_preposition] should be 'de' because 'de' is a preposition, but 'de' is not needed here
- Found (matched): `as` -> `às` [article_preposition] should be 'às' because 'as' is a preposition, but 'às' is the correct preposition to use here
- Missed: `cafe`
- Missed: `onibus`

### a2-verb-form-irregular-fazer (erroneous, a2)

- Task: What did you do yesterday evening?
- Answer: Ontem à noite eu fazi o jantar e depois fui dormir cedo.
- Reference: `fazi` -> fiz [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `fazi` -> `fiz` [verb_form] should be 'fiz'
- Found (quote not in the answer): `cego` -> `cedo` [spelling] should be 'cedo'

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `fim de semana` -> `fim de semana` [article_preposition] should be 'fim de semana' (possessive form) instead of 'fim de semana' (simple form)
- Found (false positive): `visitei` -> `visitei` [verb_form] should be 'visitei' (verb conjugation) instead of 'visitei' (verb conjugation with 'eu' subject)
- Found (false positive): `nós` -> `meus avós` [pronoun] should be 'meus avós' (possessive form) instead of 'nós' (subject pronoun)
- Found (false positive): `almoçamos` -> `almoçamos` [verb_form] should be 'almoçamos' (verb conjugation) instead of 'almoçamos' (verb conjugation with 'nós' subject)
- Found (matched): `volti` -> `voltei` [verb_form] should be 'voltei' (verb conjugation) instead of 'volti' (verb conjugation with 'eu' subject)

### a2-verb-form-imperfect (erroneous, a2)

- Task: Describe your childhood.
- Answer: Quando eu era criança, eu moro numa cidade pequena e brincava na rua.
- Reference: `moro` -> morava [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `moro` -> `morava` [not_brazilian] should be 'morava'
- Found (false positive): `pequena` -> `pequena` [spelling] should be 'pequena'
- Found (false positive): `brincava` -> `brincava` [spelling] should be 'brincava'
- Found (false positive): `na` -> `na` [article_preposition] should be 'na'
- Found (false positive): `rua` -> `rua` [spelling] should be 'rua'

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `fiquei` -> `fiquei` [verb_form] should be 'fiquei' (present perfect) instead of 'fiquei' (imperfect)
- Found (matched): `eu era` -> `eu estava` [ser_estar] should be 'eu estava' (present perfect) instead of 'eu era' (imperfect)

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `engraçado` -> `engraçada` [word_choice] should be 'engraçada' because the friend is a woman
- Found (false positive): `pretos` -> `pretos` [spelling] should be 'pretos' (no accent mark) because 'preto' is a color, not a hair color
- Missed: `curtas`

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `exame de proficiência` -> `exame de proficiência` [not_brazilian] should be 'exame de proficiência' (no contraction)
- Missed: `por passar`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `assisti` [verb_form] should be 'assisti' because 'atender' is the infinitive form of 'atender' and not the present perfect tense
- Found (false positive): `novo` -> `novo` [word_choice] should be 'novo' because 'novi' is the present perfect tense of 'novar' and not the adjective 'novo'
- Found (false positive): `no` -> `no` [article_preposition] should be 'no' because 'no' is a determiner and not a preposition

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cama` -> `cama` [spelling] should be 'cama' (bed) instead of 'cama' (bed) with an accent mark
- Missed: `lavo os dentes`

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajei` -> `viajei` [verb_form] should be 'viajei' (verb conjugation error)
- Found (false positive): `perto da praia` -> `próximo à praia` [article_preposition] should be 'próximo à praia' (correct preposition)

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `sábado` -> `sábado` [word_choice] should be 'sábado' (day of the week) instead of 'sábado' (day of the week and day of the month)
- Found (false positive): `vou visitar` -> `vou visitar` [verb_form] should be 'vou visitar' (future simple) instead of 'vou visitar' (future simple, but not the correct form for the verb 'visitar' in this context)
- Found (false positive): `meus avós` -> `avós` [agreement] should be 'avós' (grandparents, informal) instead of 'avós' (grandparents, formal)
- Found (false positive): `vou ao` -> `vou ao mercado` [article_preposition] should be 'vou ao mercado' (going to the market) instead of 'vou ao mercado' (going to the market, but the preposition 'ao' is not used in this context)

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `mais novo` -> `mais velho` [content_mismatch] should be 'mais velho' because your phone is newer
- Found (false positive): `mais barato` -> `mais caro` [content_mismatch] should be 'mais caro' because your friend's phone is cheaper

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vire` -> `vire à esquerda` [word_order] should be 'vire à esquerda'
- Found (false positive): `banco` -> `banco` [word_choice] should be 'banco'
- Found (false positive): `fica` -> `` [not_brazilian] should be 'encontra' or 'encontra-se' (depending on the context), but in this case 'fica' is not wrong

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `perto de casa` -> `perto do cinema` [word_order] should be 'perto do cinema'

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ontem` -> `hier` [content_mismatch] should be 'hier' or 'ontem' depending on the day of the week

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `meu` -> `meu` [not_brazilian] should be 'meu' (possessive adjective) instead of 'no meu' (no possessive adjective)

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto muito da cidade` -> `` [content_mismatch] The task is about the meal, not the city.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-verb-form-subjunctive-present (erroneous, b1)

- Task: What do you hope for the future?
- Answer: Espero que meus filhos são felizes e que eles tenham uma boa educação.
- Reference: `são` -> sejam [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `meus` -> `meus filhos` [agreement] should be 'meus filhos' (possessive adjective agreement)
- Missed: `são`

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `mês que vem` -> `mês que vem` [content_mismatch] should be 'mês que vem' (month that comes) instead of 'mês que vem' (month that is coming)
- Missed: `Se eu ter`

### b1-agreement-passive (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1750 e foi restaurado em 2005.
- Reference: `restaurado` -> restaurada [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `catedral` -> `catedral` [spelling] should be 'catedral' with an acute accent
- Missed: `restaurado`

### b1-content-date (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1850 e restaurada em 2005.
- Reference: `1850` -> 1750 [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `catedral` -> `catedral` [spelling] should be 'catedral' with an acute accent
- Found (false positive): `construída` -> `construída` [spelling] should be 'construída' with an acute accent
- Missed: `1850`

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `seus` -> `seus amigos` [agreement] should be 'seus amigos' (possessive pronoun agreement)

### b1-control-future-subjunctive (control, b1)

- Task: What will you do when you finish the course?
- Answer: Quando eu terminar o curso, vou procurar um emprego novo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `terminar` -> `concluir` [verb_form] should be 'concluir' (to finish) or 'completar' (to complete) in this context

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `na` -> `a` [agreement] should be 'a' because 'loteria' is a feminine noun

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `nós` [agreement] subject-verb agreement, 'nós' is a plural pronoun, but 'podermos' is a singular verb
- Found (false positive): `uma boa mesa` -> `uma boa` [agreement] the adjective 'boa' should be 'uma boa' because 'mesa' is a feminine noun

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `trabalhar em casa` -> `trabalhar de casa` [word_order] should be 'trabalhar de casa'

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Sr.` -> `Senhor` [not_brazilian] Informal address, use 'Senhor' or 'Senhora' for a formal request.
- Found (false positive): `na próxima` -> `sexta-feira` [word_order] Should be 'na próxima sexta-feira', 'na próxima' is not a correct preposition to use here.

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `quando` -> `ao` [word_order] should be 'ao' instead of 'quando'
- Found (false positive): `eu` -> `eu` [pronoun] should be 'eu' instead of 'eu cheguei'
- Found (false positive): `cheguei` -> `cheguei` [word_order] should be 'cheguei' instead of 'cheguei ao'
- Found (false positive): `ao` -> `ao` [word_order] should be 'ao' instead of 'ao cinema'
- Found (false positive): `o filme` -> `o filme já` [ser_estar] should be 'o filme já' instead of 'o filme já tinha'
- Found (matched): `já tinha` -> `já` [ser_estar] should be 'já' instead of 'já tinha'

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `Recomendo que você estuda todos os dias` -> `Recomendo que você estude todos os dias` [verb_form] should be 'estude' instead of 'estuda'
- Found (quote not in the answer): `assistir` -> `assistir` [verb_form] should be 'assistir' instead of 'assistir' (no change needed)
- Found (false positive): `a filmes brasileiros` -> `filmes brasileiros` [article_preposition] should be 'filmes brasileiros' instead of 'filmes brasileiros' (no change needed)
- Found (false positive): `exige paciência` -> `exige paciência` [ser_estar] should be 'exige paciência' instead of 'exige paciência' (no change needed)
- Missed: `assiste`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `do` -> `de` [word_choice] should be 'de' instead of 'do'
- Found (quote not in the answer): `d dia` -> `no dia` [word_order] should be 'no dia' instead of 'no dia'
- Found (false positive): `dez` -> `dezimo` [spelling] should be 'dezimo' instead of 'dez'

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `na` -> `a` [article_preposition] should be 'a' because 'semana' is a feminine noun
- Found (false positive): `passagens` -> `passagens` [verb_form] should be 'passagens' because 'comprado' is in the past tense, so the noun should also be in the past participle form

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `complicado` -> `estressful` [word_choice] should be 'estressful' or 'difficult' instead of 'complicated'

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `deveria` -> `deveria ter` [verb_form] should be 'deveria ter' (had to) instead of 'deveria' (should have)
- Found (quote not in the answer): `dois` -> `dois` [spelling] should be 'dois' (two) instead of 'dez' (ten)

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `me` [pronoun] should be 'me' because 'disse' is in the 2nd person singular

## phi4-mini / few_shot

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
- Missed: `em o`

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
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto de futebol` -> `gosto de futebol` [word_choice] The verb 'gostar' should be followed by 'de' when talking about liking something.

### a1-english-lunch (not_portuguese, a1)

- Task: What did you eat for lunch?
- Answer: I ate rice and chicken for lunch.
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
- Found (false positive): `acordei tarde` -> `acordei tarde` [verb_form] The verb should be in the past tense to match the context of the sentence.
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
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `está chuvoso e frio` [word_choice] The phrase should describe cold and rainy weather, not sunny and hot.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eu estou comendo espaguete e vinho tinto com uma pessoa na restaurante.` [content_mismatch] The answer does not describe the meal. It is about the speaker's residence and liking for the city.

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

## phi4-mini / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `maiô` -> `maio` [word_choice] The word 'maiô' is not used in Brazilian Portuguese; the correct word is 'maio'.
- Found (false positive): `chapéu de palha` -> `chapéu` [word_choice] The word 'palha' is not used in Brazilian Portuguese; the correct word is 'chapéu'.
- Found (false positive): `sandálias brancas` -> `sandais` [word_choice] The word 'sandálias' is not used in Brazilian Portuguese; the correct word is 'sandais'.

### a1-control-coffee (control, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `eu gosto` -> `eu gosto` [verb_form] The verb 'gostar' should be conjugated to match the subject 'eu'.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Tô` -> `Estou` [verb_form] The word 'tô' is not appropriate for an A1 level speaker; the expected answer would be 'Estou'.
- Found (false positive): `bem` -> `tô` [word_choice] The word 'bem' is too formal for an A1 level speaker; the expected answer would be 'tô' or 'estou'.
- Found (false positive): `e você?` -> `você está bem?` [word_order] The phrase 'e você?' is too complex for an A1 level speaker; the expected answer would be 'você está bem?' or simply 'você bem?'.
- Found (false positive): `Só um pouco cansado` -> `Um pouco cansado` [word_order] The phrase 'Só um pouco cansado' is too complex for an A1 level speaker; the expected answer would be 'Um pouco cansado' or simply 'Cansado'.

### a1-control-daily-routine (control, a1)

- Task: What do you do every day?
- Answer: Todo dia eu acordo cedo, tomo café e vou para o trabalho.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (quote not in the answer): `agrego` -> `agro` [spelling] It should be 'agro' which means 'I get up' in Brazilian Portuguese.

### a1-control-order-meal (control, a1)

- Task: You are at a restaurant. Order your meal.
- Answer: Eu queria uma salada e um suco de laranja, por favor.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu queria` -> `Eu quero` [verb_form] The verb 'querer' should be conjugated to match the subject 'Eu' in the first person singular.

### a1-control-family (control, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos e uma irmã. Meu pai é médico e minha mãe é professora.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Meu pai é médico e minha mãe é professora.` -> `My father is a doctor and my mother is a teacher.` [content_mismatch] The answer does not describe the family as asked in the task.
- Found (false positive): `Eu tenho dois irmãos e uma irmã.` -> `I have two siblings and a sister.` [word_order] The answer is in English, not in Portuguese.
- Found (false positive): `Meu pai é médico e minha mãe é professora.` -> `My father is a doctor and my mother is a teacher.` [word_order] The answer is in English, not in Portuguese.
- Found (false positive): `Eu tenho dois irmãos e uma irmã.` -> `I have two siblings and a sister.` [word_order] The answer is in English, not in Portuguese.
- Found (false positive): `Meu pai é médico e minha mãe é professora.` -> `My father is a doctor and my mother is a teacher.` [word_order] The answer is in English, not in Portuguese.

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `me chamo` -> `Eu me chamo Ana.` [word_choice] The verb 'chamar' is not used to mean 'to call' or 'to name oneself'. The correct verb is 'ter' or 'se nomear'.
- Found (quote not in the answer): `me moro` -> `Eu me chamo Ana.` [word_choice] The verb 'morar' means 'to live' but it is not used to mean 'to call oneself'. The correct verb is 'ter' or 'se nomear'.

### a1-spelling-feijao (erroneous, a1)

- Task: What is your favorite food?
- Answer: Minha comida favorita é fejão com arroz.
- Reference: `fejão` -> feijão [spelling]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fejão` -> `arroz` [word_choice] The word 'fejão' is not a food item; it means 'head'.

### a1-spelling-accents (erroneous, a1)

- Task: What do you drink in the morning?
- Answer: De manha eu bebo cafe com leite.
- Reference: `manha` -> manhã [spelling]
- Reference: `cafe` -> café [spelling]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `De manha` -> `Eu bebo de manha.` [ser_estar] It should be 'Eu bebo' instead of 'Eu bebo de manha'.
- Found (matched): `bebo cafe` -> `bebo café.` [word_choice] The word 'cafe' should be 'café' in Brazilian Portuguese.
- Found (false positive): `com leite` -> `com leite.` [spelling] The word 'leite' should be in lowercase.

### a1-agreement-trousers (erroneous, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: Marina veste uma camiseta azul e uma calça pretos.
- Reference: `pretos` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `veste` -> `vesteu` [verb_form] The verb 'veste' is incorrect; it should be 'vesteu' to match the subject 'Marina'.
- Found (matched): `uma calça pretos` -> `uma calça pretas` [agreement] The noun 'calça' is masculine, so the adjective should be 'pretos' instead of 'preta'.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `centro` -> `cidade` [word_choice] The word should be 'cidade', which means 'city' in English.
- Found (false positive): `escritório` -> `escritório` [spelling] The word should be ' escritório', which is the correct spelling.
- Found (false positive): `Eu trabalho` -> `Eu trabalho` [verb_form] The answer should be in the present tense to match the question.
- Found (false positive): `em` -> `na` [article_preposition] The preposition should be 'na' before a feminine singular noun.
- Found (false positive): `no centro` -> `no centro da cidade` [word_order] The phrase should be 'no centro da cidade'.
- Missed: `uma escritório`

### a1-agreement-plural-adjective (erroneous, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos. Eles são muito simpático.
- Reference: `simpático` -> simpáticos [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `simpático` -> `simpática` [agreement] The word should be 'simpática' to agree with the feminine gender of 'irmãs'.
- Found (false positive): `Eles` -> `nós` [pronoun] The pronoun should be 'nós' to match the first person plural subject 'Eu'.
- Found (false positive): `Eu tenho` -> `Tenho eu dois irmãos.` [word_order] The sentence structure is incorrect for a simple description of family members.
- Found (false positive): `Eles são muito` -> `Eles são muito.` [word_order] The sentence structure is incorrect for a simple description of family members.

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Agora eu sou` -> `Estou` [ser_estar] The verb 'ser' is used instead of the reflexive 'estar' which is needed in this context.

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Eu estou brasileiro e moro em Recife.` -> `Eu sou do Brasil e moro em Recife.` [content_mismatch] The answer does not directly answer where the person is from.

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `sou` -> `estou` [article_preposition] A missing contraction no for o
- Found (false positive): `com fome` -> `ter fome` [word_choice] A more common expression would be 'ter fome' or 'estar com fome'
- Found (false positive): `Depois do trabalho` -> `Depois do trabalho` [word_order] A more common expression would be 'Depois do trabalho' or 'Depois do trabalho' for A1 level
- Found (duplicate): `eu sou cansado` -> `estou cansado` [verb_form] A more common expression would be 'estou cansado' or 'sou cansado' for A1 level
- Found (false positive): `e` -> `e eu estou cansado` [word_order] A coordinating conjunction 'e' is not needed here
- Found (false positive): `e com fome` -> `e tenho fome` [word_order] A coordinating conjunction 'e' is not needed here
- Found (false positive): `eu` -> `estou` [pronoun] A subject pronoun 'eu' is not needed when using the verb in the first person singular
- Found (duplicate): `sou cansado` -> `estou cansado` [verb_form] A more common expression would be 'estou cansado' or 'sou cansado' for A1 level
- Found (false positive): `e` -> `e estou cansado` [word_order] A coordinating conjunction 'e' is not needed here
- Found (quote not in the answer): `e tenho fome` -> `entendo tenho fome` [word_order] A coordinating conjunction 'e' is not needed here

### a1-article-contraction-em-o (erroneous, a1)

- Task: Where do you live?
- Answer: Eu moro em o Brasil, em São Paulo.
- Reference: `em o` -> no [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `em o` -> `em o Brasil` [article_preposition] The article 'o' is not used with 'Brasil' in this context; it should be 'no' or just 'em'.
- Found (duplicate): `Eu moro em o` -> `Eu moro no Brasil` [ser_estar] The article 'o' is not used with 'morou'; it should be 'no' or just 'em'.
- Found (quote not in the answer): `Eu moro em São Paulo` -> `Eu moro em São Paulo` [word_order] The sentence is grammatically incorrect.

### a1-preposition-missing-em (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Canadá e moro Toronto.
- Reference: `moro Toronto` -> moro em Toronto [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `e moro` -> `moço` [agreement] The verb 'morar' should agree with the subject 'Eu' in the first person singular, which is 'moço'.
- Found (false positive): `e` -> `` [word_order] The contraction 'e' is not needed in this context.

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Eu jantar` -> `Eu comer` [word_choice] The verb 'jantar' is not used for 'to eat'; the correct verb is 'comer'.
- Found (quote not in the answer): `assistor de televisão` -> `assistir televisão` [word_choice] The verb 'assistir' is not used for watching TV; the correct verb is 'assistir'.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Eu chamo Pedro` -> `Meu nome é Pedro` [content_mismatch] The answer contains unnecessary information about the name.

### a1-pronoun-reflexive-friend (erroneous, a1)

- Task: What is your friend's name?
- Answer: A minha amiga chama Maria e ela tem trinta anos.
- Reference: `chama` -> se chama [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `ela tem trinta anos` -> `A minha amiga Maria tem trinta anos.` [content_mismatch] The sentence contains unnecessary information.
- Missed: `chama`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `No almoço` -> `em almoço` [article_preposition] No should be replaced with a contraction em
- Found (false positive): `como` -> `eu como` [verb_form] The verb should be in the first person singular present indicative, which is 'como' not 'como'
- Found (false positive): `frango` -> `frangos` [agreement] The word should be in its plural form to agree with the plural noun 'beans'
- Found (matched): `beans` -> `arroz` [agreement] The word should be in its plural form to agree with the plural noun 'arroz'
- Found (false positive): `arroz` -> `arroz` [agreement] The word should be in its plural form to agree with the plural noun 'beans'
- Found (duplicate): `e` -> `` [word_order] The word 'e' is not necessary in this context

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Ele está` -> `Ele está com uma vermelha camisa e uma calça preta.` [verb_form] The verb 'está' is not needed in this context.
- Found (false positive): `vermelha camisa` -> `vermelha camisa` [word_choice] The word is spelled correctly.
- Found (false positive): `uma calça preta` -> `uma calça preta` [word_choice] The word is spelled correctly.
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `eu gosto de` -> `gosto de` [verb_form] The verb 'gostar' should be conjugated to match the subject 'eu'.
- Found (quote not in the answer): `eu gosto de muito` -> `eu gosto de` [word_choice] The adverb 'muito' is not needed in this context.

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Response: invalid or missing

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol` -> `Eu comecei o café com pão e iogurte.` [word_choice] The answer talks about liking football instead of answering what was eaten for breakfast.

### a1-english-lunch (not_portuguese, a1)

- Task: What did you eat for lunch?
- Answer: I ate rice and chicken for lunch.
- Flags: written_in_portuguese=true answers_the_question=false

### a2-spelling-portugues (erroneous, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu trabalhei de manhã e à tarde eu estudei portugues.
- Reference: `portugues` -> português [spelling]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Ontem` -> `eu trabalhei de manhã e à tarde eu estudei português.` [verb_form] The word 'ontem' should be in the past tense to match the question about yesterday.
- Found (false positive): `eu trabalhei de manhã e à tarde eu estudei` -> `eu trabalhei de manhã e à tarde estudei.` [verb_form] The verb 'estudei' should be in the past tense to match the time frame of 'yesterday'.
- Found (false positive): `eu estudei` -> `eu estudiei.` [verb_form] The verb 'estudei' should be in the past tense to match the time frame of 'yesterday'.
- Missed: `portugues`

### a2-spelling-agua (erroneous, a2)

- Task: What did you buy at the market?
- Answer: No mercado eu comprei maçãs, dois quilos de arroz e uma garrafa de agua.
- Reference: `agua` -> água [spelling]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `comprei` -> `compra` [verb_form] The verb 'compri' is not used in Brazilian Portuguese; 'compra' should be used.
- Found (quote not in the answer): `duois` -> `dois` [spelling] The number 'two' should be written as 'dois' in Brazilian Portuguese.
- Missed: `agua`

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Response: invalid or missing

### a2-verb-form-irregular-fazer (erroneous, a2)

- Task: What did you do yesterday evening?
- Answer: Ontem à noite eu fazi o jantar e depois fui dormir cedo.
- Reference: `fazi` -> fiz [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazi` -> `faí` [verb_form] The verb 'fazer' should be in the past tense 'faí'.
- Found (quote not in the answer): `fiei` -> `faí` [verb_form] The verb 'fazer' should be in the past tense 'faí'.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `visitou` -> `visitei` [verb_form] The verb should be in the past tense to match the context of the sentence.
- Missed: `volti`

### a2-verb-form-imperfect (erroneous, a2)

- Task: Describe your childhood.
- Answer: Quando eu era criança, eu moro numa cidade pequena e brincava na rua.
- Reference: `moro` -> morava [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `eu moro` -> `eu morava` [verb_form] The verb 'morar' is used incorrectly. It should be 'eu morava' to match the past tense in the context of describing childhood.
- Found (quote not in the answer): `eu brincava` -> `eu brincava` [verb_form] The verb 'brincava' is used incorrectly. It should be 'eu brincava' to match the past tense in the context of describing childhood.

### a2-verb-form-subject-agreement (erroneous, a2)

- Task: What did your family do on Sunday?
- Answer: No domingo minha família almoçaram na casa da minha avó.
- Reference: `almoçaram` -> almoçou [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `almoçaram` -> `almoçaram` [verb_form] The verb should be in the past tense, 'almoçaram' is correct.
- Found (false positive): `na casa da minha avó` -> `na casa da minha avó` [article_preposition] The preposition 'na' should be 'na'.
- Found (false positive): `No domingo` -> `No domingo` [word_order] The sentence should start with 'No domingo'.
- Found (false positive): `Minha família` -> `Minha família` [word_order] The sentence should start with 'Minha família'.
- Found (false positive): `almoçaram` -> `almoçaram` [verb_form] The verb should be in the past tense, 'almoçaram' is correct.
- Found (false positive): `na casa da minha avó` -> `na casa da minha avó` [article_preposition] The preposition 'na' should be 'na'.
- Found (false positive): `No domingo` -> `No domingo` [word_order] The sentence should start with 'No domingo'.
- Found (false positive): `Minha família` -> `Minha família` [word_order] The sentence should start with 'Minha família'.
- Missed: `almoçaram`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Eu era cansado e dormi o dia todo.` -> `eu dormi todo o dia.` [word_order] The phrase 'dormi o dia todo' is not idiomatic in Portuguese. The correct expression should be 'eu dormi todo o dia.'
- Found (false positive): `Ontem estava muito frio e eu fiquei em casa.` -> `Ontem estava muito frio e fiquei em casa.` [word_order] The sentence structure is incorrect. The correct structure should be 'Ontem estava muito frio e fiquei em casa.'
- Found (quote not in the answer): `Eu era cansado.` -> `Eu estava cansado.` [verb_form] The verb 'era' does not agree with the subject 'Eu'. The correct form should be 'Eu estava cansado.'
- Found (quote not in the answer): `Eu dormi o dia todo.` -> `eu dormi todo o dia.` [word_order] The phrase 'dormi o dia todo' is not idiomatic in Portuguese. The correct expression should be 'eu dormi todo o dia.'

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Response: invalid or missing

### a2-preposition-por-para (erroneous, a2)

- Task: Why are you studying Portuguese so much?
- Answer: Eu estudo muito por passar no exame de proficiência.
- Reference: `por passar` -> para passar [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `passar no exame de proficiência` -> `passar no exame de proficiência` [word_choice] The phrase 'passar no exame de proficiência' is not a common expression in Brazilian Portuguese. A more common way to say this would be 'passar no exame de proficiência' or 'passar no teste de proficiência.'
- Missed: `por passar`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `ontem` -> `ontou` [verb_form] The word should be in the past tense to match the question about 'last night'.
- Found (matched): `atendi` -> `ateste` [verb_form] The verb should be in the past tense to match the question about 'last night'.

### a2-word-choice-lavar-dentes (erroneous, a2)

- Task: Describe your morning routine.
- Answer: De manhã eu faço a cama, tomo banho e lavo os dentes.
- Reference: `lavo os dentes` -> escovo os dentes [word_choice]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `tomo` -> `serei` [word_choice] The verb 'tomar' means 'to drink' in this context, not 'to take a bath'.
- Found (quote not in the answer): `serei` -> `serei` [ser_estar] The correct future tense of 'ser' for 'I will be' is 'erei' in this context.
- Missed: `lavo os dentes`

### a2-pronoun-reciprocal (erroneous, a2)

- Task: Describe your relationship with your neighbors.
- Answer: Eu me dou bem com meus vizinhos e sempre nós ajudamos quando precisamos.
- Reference: `nós ajudamos` -> nos ajudamos [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `me dou bem` -> `eu me conheço bem` [word_choice] This phrase is not commonly used in Brazilian Portuguese. The correct phrase would be 'eu me conheço bem' which means 'I am well-acquainted with'.
- Missed: `nós ajudamos`

### a2-not-brazilian-gerund (erroneous, a2)

- Task: What are you doing now?
- Answer: Estou a comer pão com queijo na cozinha.
- Reference: `a comer` -> comendo [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Estou a comer pão com queijo na cozinha.` -> `I'm currently eating bread with cheese in the kitchen.` [content_mismatch] The answer does not answer the question about what the learner is doing now.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Eu vou para o trabalho de autocarro todos os dias.` -> `Eu faço o transporte de carro todos os dias para ir ao trabalho.` [content_mismatch] The answer does not directly answer the question about how to go to work.
- Found (duplicate): `Eu vou para o trabalho de autocarro` -> `Eu vou para o trabalho` [word_choice] The phrase 'de autocarro' is not necessary and makes the sentence awkward.

### a2-not-brazilian-breakfast (erroneous, a2)

- Task: What time do you have breakfast?
- Answer: Eu tomo o pequeno-almoço às sete e meia.
- Reference: `pequeno-almoço` -> café da manhã [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Eu tomo o pequeno-almoço` -> `Eu tomo o café da manhã às sete e meia.` [content_mismatch] The answer is about lunch, not breakfast.
- Found (false positive): `às sete e meia` -> `às sete e meia.` [word_order] The time is not specified as being in the morning.
- Found (duplicate): `Eu tomo o pequeno-almoço` -> `Eu tenho o café da manhã às sete e meia.` [word_choice] The verb 'tomar' is not typically used for breakfast.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol e muito calor` -> `it is raining and cold.` [content_mismatch] The description is opposite to the scene facts, which state it is raining and cold.
- Found (false positive): `Hoje está` -> `Hoje está chovendo.` [word_choice] The phrase should indicate that it is raining, not sunny.
- Found (duplicate): `muito calor` -> `muito frio.` [word_choice] The description is opposite to the scene facts, which state it is cold.

### a2-control-trip (control, a2)

- Task: Tell me about your last trip.
- Answer: No ano passado eu viajei para o Rio de Janeiro com minha irmã. Nós ficamos num hotel perto da praia e visitamos o Cristo Redentor.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (quote not in the answer): `viagei` -> `viageei` [verb_form] The verb 'viagear' should be in the first person singular form 'viageei' to match the subject 'eu'.
- Found (quote not in the answer): `já` -> `` [word_choice] The word 'já' is not needed in this context.
- Found (false positive): `perto da praia` -> `próximo da praia` [word_choice] The phrase 'perto da praia' is not incorrect but less common; 'próximo da praia' is more natural in this context.
- Found (false positive): `visitamos` -> `visitamos` [verb_form] The verb 'visitarmos' should be in the first person plural form 'visitamos' to match the subject 'nós'.
- Found (false positive): `perto da praia` -> `próximo da praia` [word_choice] The phrase 'perto da praia' is not incorrect but less common; 'próximo da praia' is more natural in this context.

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `No` -> `Vou` [word_choice] No is not used in this context; 'Vou' should be used instead.

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `mais novo` -> `novíssimo` [word_choice] The word should be 'novíssimo' to match the comparative form used in the sentence.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `a estação fica à direita` -> `a estação fica à esquerda` [word_order] The direction should be to the left, not to the right.
- Found (false positive): `Siga em frente` -> `vire à esquerda` [word_order] The phrase should be 'vire à esquerda' instead of 'siga em frente'.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `A gente` -> `Nós` [ser_estar] The use of 'A gente' instead of 'Nós' is more common in European Portuguese. In Brazil, 'Nós' is preferred.
- Found (quote not in the answer): `fomos` -> `vamos` [verb_form] The verb 'fomos' is not used in the context of a group activity that happened recently. The correct form should be 'fomos' or 'vamos'.
- Found (quote not in the answer): `fui` -> `vamos` [verb_form] The verb 'fui' is not used in the context of a group activity that happened recently. The correct form should be 'vamos'.
- Found (false positive): `comeu` -> `comei` [agreement] The verb 'comeu' should be in the plural form to agree with 'nós'.
- Found (false positive): `restaurante` -> `restaurante` [word_choice] The word 'restaurante' is not commonly used in Brazil; 'restaurante' is more common in Portugal.
- Found (false positive): `perto de casa` -> `perto de casa` [word_choice] The phrase 'perto de casa' is not commonly used in Brazil; 'perto de casa' is more common in Portugal.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu vi o filme` -> `Eu vi o filme` [spelling] The word 'filme' should be in lowercase.
- Found (false positive): `e gostei muito dele` -> `do dele` [agreement] The word 'dele' should be 'do dele' to match the masculine singular form.

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cozinhar` -> `cozinhar` [spelling] The word should be in lowercase, 'cozinhar' instead of 'cozinhar'.
- Found (quote not in the answer): `passar com o meu cachorro` -> `passar com o meu cachorro` [spelling] The word should be in lowercase, 'passar com o meu cachorro' instead of 'passar com o meu cachorro'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eu moro em São Paulo há cinco anos e gosto muito da cidade. Esta noite, dois amigos estão comendo espaguete e bebendo vinho tinto.` [content_mismatch] The answer does not describe the meal.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=false answers_the_question=false

### b1-verb-form-subjunctive-present (erroneous, b1)

- Task: What do you hope for the future?
- Answer: Espero que meus filhos são felizes e que eles tenham uma boa educação.
- Reference: `são` -> sejam [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (quote not in the answer): `hope` -> `I hope that my children are happy and that they have a good education.` [word_choice] The question asked for a statement about what the future holds, not a hope for something.
- Missed: `são`

### b1-verb-form-future-subjunctive (erroneous, b1)

- Task: What will you do if you have free time next month?
- Answer: Se eu ter tempo no mês que vem, vou viajar para o Nordeste.
- Reference: `Se eu ter` -> Se eu tiver [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `se eu ter` -> `se eu tiver` [verb_form] The phrase should be in the future tense, 'se eu tiver' instead of 'se eu ter'
- Found (false positive): `o Nordeste` -> `` [content_mismatch] This is not a mistake in Portuguese; it's just a specific place name.

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Se eu ganhava` -> `If I won the lottery, ` [content_mismatch] The question asked for what you would do if you won the lottery, but the answer talks about winning the lottery.
- Found (false positive): `eu compraria uma casa` -> `compraria uma casa, ` [verb_form] The answer should be in the conditional tense to match the hypothetical situation.
- Found (quote not in the answer): `viajar pelo mundo` -> `viajará pelo mundo, ` [verb_form] The answer should be in the conditional tense to match the hypothetical situation.
- Found (duplicate): `Se eu ganhava na loteria` -> `Se ganhasse na loteria, ` [ser_estar] The answer should be in the conditional tense to match the hypothetical situation.
- Found (duplicate): `eu ganhava na loteria` -> `ganharia na loteria, ` [verb_form] The answer should be in the conditional tense to match the hypothetical situation.

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro aqui` -> `Eu moro aqui há` [word_choice] The question asked for the duration of living in the place, not the current residence.
- Found (false positive): `desde 2019` -> `desde 2019 há` [verb_form] The phrase should be in the past tense to match the question about how long the person has lived there.
- Missed: `tenho visitado`

### b1-agreement-passive (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1750 e foi restaurado em 2005.
- Reference: `restaurado` -> restaurada [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `A` -> `A catedral` [word_order] The word 'A' is unnecessary and not used in spoken Brazilian Portuguese.
- Found (false positive): `foi` -> `fue` [ser_estar] The word 'foi' is not needed in this context.
- Missed: `restaurado`

### b1-content-date (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1850 e restaurada em 2005.
- Reference: `1850` -> 1750 [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `1850` -> `1750` [content_mismatch] The year given is incorrect; it should be 1750 as per the scene facts.
- Found (false positive): `restaurada em 2005` -> `` [content_mismatch] This part of the sentence is correct.

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `traga` -> `vir` [word_choice] The word 'traga' is not used in this context; 'vir' is the correct word.
- Found (quote not in the answer): `pode` -> `pode` [word_choice] The word 'pode' is not used in this context; 'pode' is not a mistake, but the context of 'pode' is not used correctly here.

### b1-control-future-subjunctive (control, b1)

- Task: What will you do when you finish the course?
- Answer: Quando eu terminar o curso, vou procurar um emprego novo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (quote not in the answer): `vai` -> `vou` [word_choice] The word 'vai' is not used in Brazilian Portuguese for this context; the correct word is 'vou'.
- Found (false positive): `terminar` -> `concluir` [word_choice] The word 'terminar' is not used in Brazilian Portuguese for this context; the correct word is 'concluir'.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `ganhasse` -> `ganharia` [verb_form] The verb 'ganhar' should be conjugated to 'ganhar' in the first person singular in the conditional tense.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (quote not in the answer): `poderamos` -> `poderemos` [agreement] The word 'poderemos' is incorrect in this context; it should be 'poderemos' to match the subject 'nós'.

### b1-control-opinion (control, b1)

- Task: What do you think about working from home?
- Answer: Acho que trabalhar em casa é bom porque a gente economiza tempo, mas às vezes sinto falta dos colegas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `sentir falta` -> `sentir falta de` [word_choice] The word 'sentir' is not typically used in this context in Brazilian Portuguese.
- Found (false positive): `a gente economiza` -> `nós economizamos` [word_choice] The phrase 'a gente' is less common in formal contexts and might be considered less standard.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Prezado` -> `Prezado Sr. Souza` [not_brazilian] This is a formal title used in European Portuguese, not commonly used in Brazil.
- Found (false positive): `dia de folga` -> `dia de folga` [word_choice] A more common term in Brazil is 'dia de folga'.

### b2-verb-form-pluperfect-participle (erroneous, b2)

- Task: Say what had already happened when you arrived at the cinema.
- Answer: Quando eu cheguei ao cinema, o filme já tinha começou.
- Reference: `tinha começou` -> tinha começado [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `o filme` -> `o filme` [word_choice] The word 'filme' should be in lowercase when referring to a movie in a general context in Brazilian Portuguese.
- Missed: `tinha começou`

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.` -> `Eu recomendaria que seu amigo estude todos os dias, assista a filmes em português e pratique falar com nativos. Isso pode ajudar muito na aprendizagem.` [content_mismatch] The answer does not directly answer the question about what to recommend to a friend who wants to learn Portuguese.
- Missed: `estuda`
- Missed: `exige`

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `meio` -> `medo` [word_choice] The word 'meio' means 'way' or 'means', which is incorrect in this context. The correct word should be 'medo'.
- Found (quote not in the answer): `reclamando` -> `reclamando` [word_choice] The word 'reclamando' is not formal enough for a formal complaint. The correct word should be 'reclamando'.
- Found (matched): `do atraso` -> `del atraso` [word_choice] The phrase 'do atraso' is not formal enough for a formal complaint. The correct phrase should be 'del atraso'.
- Found (false positive): `meu pedido` -> `o seu pedido` [word_choice] The phrase 'meu pedido' is not formal enough for a formal complaint. The correct phrase should be 'o seu pedido'.
- Found (quote not in the answer): `deixaram` -> `deixaram` [word_choice] The phrase 'deixaram' is not formal enough for a formal complaint. The correct phrase should be 'deixaram'.
- Found (false positive): `no dia dez` -> `no dia dez` [word_choice] The phrase 'no dia dez' is not formal enough for a formal complaint. The correct phrase should be 'no dia dez'.

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `já tinha comprado` -> `já comprou` [word_choice] The word 'já' is not needed here.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `apagar` -> `passar por uma tempestade` [word_choice] An idiomatic expression for a stressful day should be used, not a literal phrase.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `meio` -> `mediante` [word_choice] The word 'meio' means 'way' or 'means', which is not appropriate in this context. The correct word should be 'mediante'.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu lhe disse` -> `Eu disse a ele que não me esperasse, mas ele não me ouviu.` [pronoun] The pronoun 'lhe' is used instead of 'ele' which is the brother being referred to.
- Found (false positive): `mas ele não me ouviu` -> `mas você não me ouviu.` [pronoun] The pronoun 'ele' is used instead of 'tu' which is the brother being referred to.

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
- Found (false positive): `muito` -> `muito` [word_order] The adverb 'muito' is misplaced; it should come before the verb or after 'gosto'.
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
- Found (false positive): `loteria` -> `loteria` [spelling] The word needs an accent on the i: loteria.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `` [pronoun] The subject pronoun 'nós' is unnecessary and creates a redundancy with the verb form.

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajaria` [verb_form] The future tense 'viagem' is needed to match the original statement's future meaning, not the conditional 'viajaria'.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morto` -> `cansado` [agreement] The adjective 'cansado' must agree in gender with the subject 'eu' (male).

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `para ele` [not_brazilian] In spoken Brazilian Portuguese, the indirect object pronoun 'lhe' is often replaced by 'para ele' or omitted in this context to avoid ambiguity with the direct object 'me'. While not strictly ungrammatical in formal writing, it is considered a European Portuguese influence or overly formal for A2/B2 learners describing casual speech. However, a more common error pattern here is actually the use of 'lhe' when 'para ele' is preferred in Brazil for clarity, but strictly speaking, 'lhe' is not a *mistake* in standard grammar unless we enforce spoken norms. Let's re-evaluate based on strict Brazilian norms: 'Eu disse para ele que...' is much more common than 'Eu lhe disse...'. But is it an error? Actually, the bigger issue might be the subjunctive 'esperasse' which is correct for past tense indirect speech. Wait, looking at the previous examples, the model seems to flag Europeanisms or agreement errors. Is 'lhe' considered a mistake here? In Brazil, 'lhe' is often avoided in favor of 'para ele' or rephrasing. Let's look closer. Actually, there is no hard grammatical error here for B2 level, but 'lhe' can be seen as formal/European. However, the most likely intended correction for a learner is often about word choice or clarity. Let's reconsider if there are NO mistakes. In Brazil, saying 'Eu lhe disse' is acceptable in writing and formal speech. Is it a mistake? Maybe not. What about 'esperasse'? Correct subjunctive. 'ouviu'? Correct past tense. Maybe the answer has no mistakes? But usually these tasks have one. Let's look at 'lhe' again. In Brazil, using 'lhe' as an indirect object pronoun is standard in formal contexts but often replaced by 'para ele' in informal. The prompt doesn't specify register. However, if I must find a mistake, 'lhe' might be flagged as 'not_brazilian' (European preference) or just 'word_choice'. But wait, is there a simpler error? No. Let's assume 'lhe' is acceptable but maybe the user expects 'para ele'. Actually, looking at similar datasets, 'lhe' is often accepted. Is there another error? 'Minhas irmãs são muito alta' in the previous turn was agreement. Here: 'Eu lhe disse que não me esperasse, mas ele não me ouviu.' Everything seems grammatically correct for B2. Perhaps the instruction implies finding *real* mistakes. If none, return empty. But I suspect 'lhe' might be flagged as 'not_brazilian' if the system considers it European. However, standard Brazilian Portuguese grammar allows 'lhe'. Let's check if 'esperasse' is wrong. No, past imperfect subjunctive after 'disse que'. Okay, maybe there are no mistakes? But I need to output something if there is a mistake. Let's assume the system considers 'lhe' as potentially European-influenced in this specific context of describing what you said (often informal). Or perhaps the mistake is simply that there isn't one? The instructions say 

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
- Found (false positive): `vermelha` -> `vermelha` [agreement] Adjectives must agree in gender with the noun they modify; 'camisa' is feminine.
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
- Found (false positive): `pequeno-almoço` -> `pequeno almoço` [not_brazilian] The hyphenated form is European Portuguese; Brazilian Portuguese uses 'pequeno almoço' or simply 'café da manhã'.
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
- Found (false positive): `visitado` -> `visitado` [verb_form] The participle 'visitado' is not used with the auxiliary verb 'ter' in this context; the past participle of 'visitar' is irregular.
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

## qwen3:1.7b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `de maiô vermelho` -> `com maiô vermelho` [word_choice] The preposition 'de' should be replaced with 'com' to correctly express 'wears' in Portuguese.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô` -> `Você` [not_brazilian] 'Tô' is a spoken form and is not appropriate for formal questions.

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
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `pretos` -> `pretas` [agreement] The adjective must be plural to agree with 'calça' (trousers).

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
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `sou em casa` -> `estou em casa` [ser_estar] The phrase 'sou em casa' is incorrect. The correct form is 'estou em casa' to match the present tense and the subject 'eu'.

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `estou brasileiro` -> `Sou brasileiro` [ser_estar] The phrase 'estou brasileiro' is incorrect. The correct form is 'Sou brasileiro' (I am Brazilian).
- Found (false positive): `moro em Recife` -> `morar` [article_preposition] The preposition 'em' is incorrect. The correct preposition for 

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
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `em o Brasil` -> `em Brasil` [article_preposition] The preposition 'em' should be followed by the country name without the article 'o'

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
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `trinta anos` -> `trinta e cinco anos` [content_mismatch] The phrase 'trinta anos' is incorrect; the correct phrase is 'trinta e cinco anos' for a specific age, but the task is to ask for a friend's name, not to provide age information.
- Missed: `chama`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `vermelha` -> `vermelhas` [agreement] The adjective must be plural to agree with 'camisa' (shirt).
- Found (false positive): `preta` -> `pretas` [agreement] The adjective must be plural to agree with 'calça' (trousers).

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `azul` -> `verde` [word_choice] The color should be 'verde' instead of 'azul' since the jacket is green.

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `toméi` -> `tomé` [verb_form] The verb 'tomar' should be conjugated as 'tomé' in the past tense for 'eu' (third person singular).
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
- Found (quote not in the answer): `engràcado` -> `engràcado` [spelling] The word needs an accent on the i: engraçado.
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
- Found (matched): `fazendo sol e muito calor` -> `está chuvendo e muito frio` [content_mismatch] The weather is raining and cold, not sunny and hot.

### a2-control-a-gente (control, a2)

- Task: What did you and your friends do on Saturday?
- Answer: A gente foi ao cinema e depois comeu pizza num restaurante perto de casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A gente` -> `Você` [not_brazilian] 'A gente' is a spoken form and is not appropriate for formal writing or structured answers.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `há cinco anos` -> `--` [content_mismatch] The sentence is about where Paulo is located, not about what he is eating.

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

### b1-content-date (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1850 e restaurada em 2005.
- Reference: `1850` -> 1750 [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `construída` -> `construída` [word_order] The verb 'construir' should be in the past participle form 'construída' for the past perfect tense. However, the answer uses the correct form, so this is not a mistake.
- Missed: `1850`

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós podermos` -> `eu podermos` [pronoun] The pronoun 'nós' does not agree with the verb 'podermos' in number and person. The correct form should be 'eu podermos' or 'não podermos' (if negative).

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

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `incêndios` -> `incêndio` [word_choice] The expression 'incêndio' is singular, but the answer uses plural. However, this is a correct usage in Brazilian Portuguese for 'fires' (plural).

## qwen3:1.7b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A Marina está de maiô vermelho` -> `A Marina está com um maiô vermelho` [spelling] The phrase 'de maiô' is not correct in Brazilian Portuguese. The correct form is 'com maiô' or 'em um maiô' when referring to wearing a swimsuit.
- Found (false positive): `chapéu de palha e sandálias brancas` -> `chapéu de palha e sandálias brancas` [content_mismatch] The phrase 'chapéu de palha' is correct, but 'sandálias brancas' is not correct. The correct form is 'sandálias brancas' (with a 'c' instead of a 'ç' in 'sandálias'), but in this context, the mistake is not a spelling error but a false friend. However, since the answer is written in Portuguese and the task is to describe the clothes, this is a content mismatch.

### a1-control-coffee (control, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `gosto de café` -> `gosto` [word_choice] The verb 'gostar' is used in the answer, but in Brazilian Portuguese, the correct form for 'like' is 'gosto' (third person singular), and the preposition 'de' is used with 'café' (feminine noun). However, the answer is correct in terms of meaning and structure.

### a1-control-colloquial-feeling (control, a1)

- Task: How are you today?
- Answer: Tô bem, e você? Só um pouco cansado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tô bem, e você? Só um pouco cansado.` -> `Tô bem, e você?` [word_choice] The phrase 'Tô bem' is a spoken Brazilian form and is considered correct. However, the answer includes a redundant phrase 'Só um pouco cansado' which is not necessary and does not answer the question directly.

### a1-control-daily-routine (control, a1)

- Task: What do you do every day?
- Answer: Todo dia eu acordo cedo, tomo café e vou para o trabalho.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `acorde` -> `acordar` [spelling] The correct spelling is 'acorde' (with an 'e' instead of 'c' in the middle), but in Brazilian Portuguese, 'acordar' is the correct verb. However, this is a spelling mistake since 'acorde' is not a valid spelling in Brazilian Portuguese.
- Found (false positive): `tomo` -> `tomar` [spelling] The correct conjugation for the first person singular of 'tomar' is 'tomo', but in this context, the correct verb is 'tomar' (to take), and the correct conjugation is 'tomo'. However, 'tomo' is not a valid spelling in Brazilian Portuguese for this verb.

### a1-control-order-meal (control, a1)

- Task: You are at a restaurant. Order your meal.
- Answer: Eu queria uma salada e um suco de laranja, por favor.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `uma salada e um suco de laranja` -> `uma salada e um suco de laranja` [agreement] The article 'um' is incorrect for 'salada' and 'suco de laranja' in this context. 'Um' is used for countable nouns, but 'salada' and 'suco de laranja' are uncountable nouns, so 'um' should be replaced with 'uma' or 'um' with the correct article for uncountable nouns.

### a1-control-family (control, a1)

- Task: Describe your family.
- Answer: Eu tenho dois irmãos e uma irmã. Meu pai é médico e minha mãe é professora.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu tenho dois irmãos e uma irmã.` -> `dois irmãos e uma irmã` [word_order] The phrase 'irmãos e uma irmã' is incorrect. The correct phrase is 'irmãos e uma irmã' (two brothers and a sister), but the answer uses 'irmãos e uma irmã' which is correct. However, the error here is not in the word itself, but in the structure. The correct structure is 'dois irmãos e uma irmã' (two brothers and a sister), but the answer is correct in terms of meaning.
- Found (false positive): `Meu pai é médico e minha mãe é professora.` -> `é médico` [word_order] The answer is correct in terms of meaning, but the phrase 'é médico' is not correct. The correct form is 'é médico' (is a doctor), but the answer is correct. However, the error here is not in the word itself, but in the structure. The correct structure is 'é médico' (is a doctor), but the answer is correct in terms of meaning.

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu me chamo Ana e moro em Curitiba.` -> `Resido em Curitiba.` [word_choice] The answer is correct in meaning, but the phrase 'moro' is not commonly used in Brazilian Portuguese. The more natural phrase would be 'resido' or 'livo em'.

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

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `brasileiro` -> `brasileiro` [word_choice] The correct form in Brazilian Portuguese is 'brasileiro' (adjective), but the answer uses 'brasileiro' as a noun. This is a false friend and incorrect usage.
- Found (false positive): `moro em Recife` -> `moro` [article_preposition] The preposition 'em' is used incorrectly here. 'Moro' is already a verb, and 'em' should be omitted when using the verb 'moro' in this context.
- Missed: `estou brasileiro`

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Depois do trabalho eu sou cansado e com fome.` -> `Depois do trabalho eu sou cansado e com fome.` [content_mismatch] The phrase 'Depois do trabalho' is correct, but the answer does not address the question directly. The question is about how you feel after work, but the answer provides a description of what you do (you are tired and hungry) rather than how you feel (you are tired and hungry). However, the mistake here is not in the wording but in the content mismatch. The answer does not directly answer the question about feeling, but it is close.
- Missed: `sou cansado`

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
- Found (false positive): `tenho vinte anos` -> `replace with the person's name` [content_mismatch] The phrase 'tenho vinte anos' is correct, but the task is asking for the person's name, not their age. The answer should focus on the name, not the age.
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
- Found (matched): `uma vermelha` -> `uma camisa vermelha` [spelling] The correct form is 'uma camisa vermelha' (a red shirt). 'Vermelha' is an adjective, and the article should be 'uma' (a) before it.
- Found (false positive): `uma calça preta` -> `uma calça preta` [spelling] The correct form is 'uma calça preta' (a black trousers). 'Preta' is an adjective, and the article should be 'uma' (a) before it.

### a1-content-jacket-colour (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Rafael usa uma jaqueta azul, calça jeans e tênis brancos.
- Reference: `azul` -> verde [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma jaqueta azul` -> `um` [spelling] The correct article for 'jaqueta' (jacket) in Portuguese is 'uma' (a), but 'uma' is not used with 'azul' (blue) in this context. 'Azul' is a color and does not require an article.
- Found (false positive): `calça jeans` -> `as` [spelling] The correct article for 'calça' (jeans) is 'as' (the), but 'calça' is a noun and does not require an article in this context.
- Found (false positive): `tênis brancos` -> `os` [spelling] The correct article for 'tênis' (sneakers) is 'os' (the), but 'tênis' is a noun and does not require an article in this context.

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo.` -> `` [content_mismatch] The answer discusses preferences and a favorite team, but it does not answer the question about what was eaten for breakfast.

### a1-english-lunch (not_portuguese, a1)

- Task: What did you eat for lunch?
- Answer: I ate rice and chicken for lunch.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-spelling-agua (erroneous, a2)

- Task: What did you buy at the market?
- Answer: No mercado eu comprei maçãs, dois quilos de arroz e uma garrafa de agua.
- Reference: `agua` -> água [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maçãs` -> `laranjas` [word_choice] The correct word for 'apples' in Portuguese is 'laranjas' (for oranges) or 'maçãs' (for apples). However, 'maçãs' is correct, but the sentence structure is slightly off.
- Found (false positive): `dois quilos de arroz` -> `dois quilos de arroz` [word_order] The preposition 'de' is used with 'quilos' (kilograms), but the sentence structure is correct. However, the word 'dois' is correctly used here.
- Found (matched): `uma garrafa de agua` -> `uma garrafa de água` [word_order] The preposition 'de' is used with 'agua' (water), but the sentence structure is correct. However, the word 'uma' is correctly used here.

### a2-spelling-three-errors (erroneous, a2)

- Task: What did you do today?
- Answer: Hoje eu acordei tarde, tomei cafe e fui trabalhar de onibus. Eu cheguei ao escritório as nove.
- Reference: `cafe` -> café [spelling]
- Reference: `onibus` -> ônibus [spelling]
- Reference: `as nove` -> às nove [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `tomaei cafe` -> `tomou café` [spelling] The correct spelling is 'tomou café' (third person singular) in the past tense. 'Tomaei' is not a correct form in Portuguese.
- Found (matched): `fui trabalhar de onibus` -> `em` [article_preposition] The correct preposition for 'bus' in Portuguese is 'em' (e em), not 'de'. 'De onibus' is incorrect.
- Missed: `cafe`
- Missed: `as nove`

### a2-verb-form-irregular-fazer (erroneous, a2)

- Task: What did you do yesterday evening?
- Answer: Ontem à noite eu fazi o jantar e depois fui dormir cedo.
- Reference: `fazi` -> fiz [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `fazi` -> `faz` [spelling] The correct spelling is 'faz' (with 'z') for the third person singular of the verb 'fazer' (to do).
- Found (quote not in the answer): `cemo` -> `dormir cedo` [spelling] The correct spelling is 'dormir cedo' (to sleep early). The answer has a typo in 'cemo' instead of 'cemo' (but the error_text is 'cemo', which is not present in the answer). However, the answer has a typo in 'cemo' (should be 'dormir cedo').

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
- Found (false positive): `quando eu era criança` -> `quando eu tinha criança` [word_choice] This is a correct form of the past tense of 'ser' (to be), but it is not commonly used in Brazilian Portuguese. It is more common to use 'quando eu tinha criança' or 'quando eu era pequeno' for a more natural expression.
- Found (matched): `moro numa cidade pequena` -> `moro em uma cidade pequena` [article_preposition] The preposition 'numa' is not commonly used in Brazilian Portuguese. The correct preposition is 'em' (in) or 'na' (in the). So, 'moro em uma cidade pequena' is more natural.

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `fiquei` -> `fiquei cansado` [verb_form] The verb 'fiquei' is incorrect in this context. The correct form for a man who is tired and slept all day is 'fiquei cansado' or 'fiquei em casa', but 'fiquei' is not the correct conjugation here. The correct form should be 'fiquei cansado' or 'fiquei em casa' depending on the context.
- Found (false positive): `dormi o dia todo` -> `dormi todo o dia` [article_preposition] The phrase 'dormi o dia todo' is correct, but the preposition 'o' is not needed here. The correct phrase is 'dormi todo o dia' or 'dormi o dia todo' (with 'o' as a contraction for 'todo'). However, 'dormi o dia todo' is grammatically correct and commonly used in Brazilian Portuguese.
- Missed: `era cansado`

### a2-agreement-two-errors (erroneous, a2)

- Task: Describe your best friend.
- Answer: Minha melhor amiga é muito inteligente e engraçado. Ela tem cabelos curtas e pretos.
- Reference: `engraçado` -> engraçada [agreement]
- Reference: `curtas` -> curtos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `Minha melhor amiga é muito inteligente e engraçado.` -> `Minha melhor amiga é muito inteligente e engraçada.` [spelling] The phrase 'muito inteligente e engraçado' is incorrect. 'Inteligente' is already an adjective, so adding 'muito' is redundant and incorrect. The correct form is 'muito inteligente' (very smart) or 'inteligente e engraçado' (smart and funny), but 'muito' is not needed before 'inteligente' in this context.
- Missed: `curtas`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Ontem à noite` -> `Ontem` [ser_estar] This is a correct form of 'last night' in Brazilian Portuguese.
- Found (false positive): `atendi` -> `atendi` [verb_form] The verb 'atender' is typically conjugated as 'atendi' in the third person singular, but 'atendi' is not the correct form here. The correct form is 'atendi' (third person singular) or 'atendei' (first person singular).
- Found (false positive): `um filme novo` -> `o filme novo` [agreement] The article 'um' is correct here, but 'novo' should be preceded by 'o' (the article) to match the gender and number of 'filme' (which is masculine).
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

### a2-control-weekend-plans (control, a2)

- Task: What are you going to do this weekend?
- Answer: No sábado vou visitar meus avós e no domingo vou ao mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no sábado vou visitar meus avós e no domingo vou ao mercado.` -> `no sábado vou visitar meus avós e no domingo vou ao mercado.` [not_brazilian] The phrase 'no sábado' is incorrect in Brazilian Portuguese. The correct form is 'no sábado' (but it's actually correct in Brazilian Portuguese, though it's more common in European Portuguese). However, the mistake here is that 'no' is not used before 'sábado' in Brazilian Portuguese. The correct form is 'no sábado' (but it's actually correct in Brazilian Portuguese, though it's more common in European Portuguese).

### a2-control-comparison (control, a2)

- Task: Compare your phone with your friend's phone.
- Answer: O meu celular é mais novo do que o seu, mas o seu é mais barato.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `O meu celular é mais novo do que o seu, mas o seu é mais barato.` -> `O meu celular é mais novo do que o seu, mas o seu é mais barato.` [spelling] The answer is correct in meaning, but it contains a spelling mistake. The word 'do' is not needed here because it's a contraction of 'de' and 'o' is not needed because it's a pronoun. The phrase 'o seu' is correct, but the word 'do' is not needed here.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vire à esquerda no banco e a estação fica à direita.` -> `vire à esquerda` [ser_estar] The phrase 'vire à esquerda no banco' is incorrect. In Brazilian Portuguese, you would say 'vire à esquerda' without the 'no' because it's a preposition used in the phrase 'vire à esquerda' (turn left).

### a2-control-free-time (control, a2)

- Task: What do you like to do in your free time?
- Answer: No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No meu tempo livre eu gosto de ler, cozinhar e passear com o meu cachorro.` -> `Em meu tempo livre` [not_brazilian] The answer is correct in terms of meaning, but the phrase 'No meu tempo livre' is not commonly used in Brazilian Portuguese. The correct phrase would be 'Em meu tempo livre' or 'No meu tempo livre' is considered more formal and less common in spoken Brazilian Portuguese.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `` [content_mismatch] This answer is not relevant to the scene facts. It talks about the speaker's residence and preference, which does not describe what the two people are eating.

### a2-english-day (not_portuguese, a2)

- Task: Describe your day.
- Answer: Today I woke up early and went to work.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-verb-form-subjunctive-present (erroneous, b1)

- Task: What do you hope for the future?
- Answer: Espero que meus filhos são felizes e que eles tenham uma boa educação.
- Reference: `são` -> sejam [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `que meus filhos são felizes` -> `que meus filhos sejam felizes` [ser_estar] The phrase 'que meus filhos são felizes' is incorrect. The correct phrase is 'que meus filhos sejam felizes' to match the future tense of the verb 'ser'.
- Found (false positive): `e que eles tenham uma boa educação` -> `e que eles tenham uma educação boa` [word_choice] The phrase 'e que eles tenham uma boa educação' is correct, but the sentence structure is slightly off. The correct structure for a future hope would be 'e que eles tenham uma boa educação' (though it's more common to say 'e que eles tenham uma educação boa' for a more natural tone).

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.` -> `Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.` [spelling] The answer is correct in meaning, but it contains a spelling mistake: 'viajaria' should be 'viajaria' (correct form) or 'viajaria' (correct form). However, the answer is already correct in terms of spelling and grammar. No mistake is reported here.
- Missed: `ganhava`

### b1-content-date (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1850 e restaurada em 2005.
- Reference: `1850` -> 1750 [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `construída` -> `construída` [spelling] The correct spelling is 'construída' (past tense of 'construir'), but 'construída' is not the correct spelling for the past tense of 'construir' in Brazilian Portuguese. The correct form is 'construída' (past participle), but the answer uses 'construída' which is correct. However, the task asks to describe the cathedral, and the answer is about the cathedral's construction date, which is correct. The error is not clear here.
- Missed: `1850`

### b1-control-subjunctive (control, b1)

- Task: Invite a friend to your party.
- Answer: Espero que você possa vir à festa e que traga seus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `que traga seus amigos` -> `que traga seus amigos` [ser_estar] The preposition 'se' is incorrectly used here. In Brazilian Portuguese, 'trazer' is used with 'seus' (possessive) when referring to bringing someone, but 'traga' is the correct form for the third person singular of the verb 'trazer' (to bring).

### b1-control-future-subjunctive (control, b1)

- Task: What will you do when you finish the course?
- Answer: Quando eu terminar o curso, vou procurar um emprego novo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `procurar` -> `procurar` [word_choice] The verb 'procurar' is correctly used here, but the answer is correct in meaning and context.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.` -> `viajaria ao redor do mundo` [word_choice] The answer is correct in meaning, but the phrase 'viajaria pelo mundo' is not commonly used in Brazilian Portuguese. A more natural expression would be 'viajaria ao mundo' or 'viajaria ao redor do mundo'.

### b1-control-personal-infinitive (control, b1)

- Task: Why did you arrive early?
- Answer: Eu cheguei cedo para nós podermos escolher uma boa mesa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para nós podermos` -> `para que` [agreement] The preposition 'para' is incorrectly used here. It should be 'para que' to express the purpose of arriving early.

### b1-control-formal-email (control, b1)

- Task: Write to your boss to ask for a day off.
- Answer: Prezado Sr. Souza, gostaria de pedir um dia de folga na próxima sexta-feira. Agradeço desde já.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Agradeço desde já.` -> `grato por` [word_choice] The phrase 'desde já' is not commonly used in formal Portuguese. The correct phrase is 'grato por' or 'grato desde já' in a more formal context.

### b2-verb-form-subjunctive-three-errors (erroneous, b2)

- Task: What do you recommend to a friend who wants to learn Portuguese?
- Answer: Recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência.
- Reference: `estuda` -> estude [verb_form]
- Reference: `assiste` -> assista [verb_form]
- Reference: `exige` -> exija [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `recomendo que você estuda todos os dias e que assiste a filmes brasileiros, embora isso exige paciência` -> `recomendo que você estude todos os dias e que assista a filmes brasileiros` [agreement] The phrase 'recomendo que você estuda todos os dias' is correct, but the error is in the part 'que assiste a filmes brasileiros' — the article 'a' should be 'as' because 'filmes' is a plural noun.
- Missed: `estuda`
- Missed: `exige`

### b2-control-reported-speech (control, b2)

- Task: Report what Ana said.
- Answer: A Ana disse que viajaria na semana seguinte e que já tinha comprado as passagens.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `que já tinha comprado as passagens` -> `que já comprou as passagens` [agreement] The correct form is 'que já comprou as passagens' (third person singular) since the subject is 'Ana'.

### b2-control-idiom (control, b2)

- Task: Describe a stressful day using an expression.
- Answer: Hoje foi um dia complicado: tive que apagar vários incêndios no trabalho e cheguei em casa morto de cansaço.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morto de cansaço` -> `muito cansado` [word_choice] The expression 'morto de cansaço' is not commonly used in Brazilian Portuguese. A more natural expression would be 'muito cansado' or 'com muita cansaço'.

### b2-control-formal-complaint (control, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar do atraso na entrega do meu pedido, que deveria ter chegado no dia dez. Solicito que o problema seja resolvido com urgência.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `atraso na entrega do meu pedido` -> `atraso na entrega do meu pedido` [word_choice] The phrase 'atraso na entrega' is not commonly used in formal complaints. A more natural and formal expression would be 'atraso na entrega do meu pedido' (though this is technically correct, it's less common in formal writing).
- Found (false positive): `no dia dez` -> `no dia 10` [word_order] The phrase 'no dia dez' is not correct in this context. The correct expression is 'no dia 10' (with a space between the day and the number).

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe disse` -> `disse` [word_order] The pronoun 'lhe' is correct, but the sentence structure is slightly off. 'Lhe' is correct, but the phrase 'Eu lhe disse' is slightly formal and not commonly used in everyday speech. A more natural phrase would be 'Eu disse para lhe dizer' or 'Eu disse que não me esperasse.'
- Found (false positive): `não me esperasse` -> `não me esperasse` [word_order] The phrase 'não me esperasse' is correct, but it is more formal and less common in everyday speech. A more natural phrase would be 'não me esperasse' or 'não me esperasse.'

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
- Found (false positive): `No` -> `No` [article_preposition] The preposition 'no' is incorrect here; the correct phrase is 'No almoço' meaning 'At lunch' but the structure should be 'No almoço' without a preposition before it.
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vermelha` -> `vermelha` [agreement] The adjective must agree with the noun 'camisa' which is feminine.
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
- Found (false positive): `fiquei` -> `fiquei` [verb_form] The verb 'ficar' in past perfect tense should be 'fiquei' but in this context it's better to use the simple past tense 'fiquei' for the action of staying home.
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
- Found (false positive): `moro` -> `moro` [verb_form] The verb should be in present continuous tense to describe what they are doing now.

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
- Found (false positive): `ganhasse` -> `ganhasse` [verb_form] The conditional tense should be 'ganhasse' for 'eu' in the past subjunctive, but the correct form is 'ganhasse' (with an accent).

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
- Found (false positive): `bebo` -> `bebo` [verb_form] The verb should be in the present tense for 'I drink' but 'bebo' is the first person present of 'beber' (to drink), so it's correct. Wait, no, let me check again. In Portuguese, the verb 'beber' in the present tense for 'I' is 'bebo', so this is correct. Maybe this is not a mistake.
- Missed: `cafe`

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `escritório` -> `o escritório` [agreement] In Brazilian Portuguese, the word for 'office' is 'escritório' but it should be used with the definite article 'o' when referring to a specific office.
- Found (false positive): `no` -> `no` [article_preposition] The contraction 'no' is not used in Brazilian Portuguese; it should be 'no' as a contraction of 'em + o' but in this context, it's better to use 'no' as a contraction for 'em o' which is correct in Brazilian Portuguese.

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
- Found (false positive): `Toronto` -> `Toronto` [content_mismatch] Toronto is a city in Canada, not in Brazil

### a1-verb-form-jantar (erroneous, a1)

- Task: What do you do at night?
- Answer: Eu jantar às oito e depois assisto televisão.
- Reference: `jantar` -> janto [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `jantar` -> `jantar` [word_choice] jantar is a verb meaning 'to eat dinner', but here it's used as a noun (dinner).
- Found (quote not in the answer): `assistos` -> `assisto` [verb_form] The verb 'assistir' should be in the present tense, not past tense.
- Missed: `jantar`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `chamo` -> `chamo` [verb_form] The verb 'chamar' is used in the present tense for 'I call', but here it's used in the past tense for 'I am called'.
- Found (false positive): `vinte` -> `vinte-` [spelling] The number 'twenty' in Portuguese is 'vinte' but it should be written with a hyphen as 'vinte-'.
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
- Found (false positive): `almoço` -> `almoço` [article_preposition] The word 'almoço' is the direct object of the verb 'comer' (to eat), but in this context, it should be the time of day (lunch) and thus should be in the prepositional phrase 'no almoço' (at lunch). However, the phrase 'no almoço' is used here as a prepositional phrase, so 'almoço' is correct but the structure is incorrect because 'no' is a contraction of 'não' and should be 'no' (with a space) for the preposition 'em' (at) or 'no' (at) in this context.
- Missed: `beans`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vermelha` -> `vermelha` [agreement] The adjective must agree with the noun 'camisa' which is feminine, but 'vermelha' is masculine form
- Found (false positive): `calça` -> `calça` [agreement] The noun 'calça' is feminine, but the article 'uma' is used for masculine nouns
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
- Found (false positive): `almoçaram` -> `almoçaram` [verb_form] The verb should be in the past tense for the third person plural, but 'almoçaram' is the past tense for the third person singular.
- Missed: `almoçaram`

### a2-serestar-tired-day (erroneous, a2)

- Task: Describe yesterday.
- Answer: Ontem estava muito frio e eu fiquei em casa. Eu era cansado e dormi o dia todo.
- Reference: `era cansado` -> estava cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `fiquei` -> `fiquei` [verb_form] The verb 'fiquei' is in the past perfect tense, but the scene says you slept all day, so it should be past simple.
- Found (matched): `era` -> `estava` [verb_form] The verb 'era' is in the past imperfect tense, but the scene says you were tired, so it should be past perfect tense.
- Found (false positive): `dormi` -> `dormi` [verb_form] The verb 'dormi' is in the past perfect tense, but the scene says you slept all day, so it should be past simple.

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
- Found (false positive): `filme` -> `filme` [agreement] In Brazilian Portuguese, 'filme' is masculine, so it should be 'filmes' for plural (since 'um filme novo' implies a specific movie, but the context suggests a general action of watching a movie, so it should be singular).

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
- Found (false positive): `vou ao mercado` -> `vou ao mercado` [verb_form] The verb 'vou' is in the present tense, but the context requires future tense for 'this weekend'.

### a2-control-directions (control, a2)

- Task: Give your friend directions to the station.
- Answer: Siga em frente, vire à esquerda no banco e a estação fica à direita.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `banco` -> `banco` [word_choice] The word 'banco' means 'bank' (the place where you put money) in Portuguese, but the scene says 'turn left at the bank' meaning the financial institution.

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
- Found (false positive): `cachorro` -> `cachorro` [spelling] The word 'cachorro' is correct for 'dog' in Portuguese, but the phrase 'passear com o meu cachorro' is grammatically correct. However, the error is in the capitalization of 'No' which is a spelling mistake.

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
- Found (false positive): `vou` -> `vou` [verb_form] The verb 'vou' is in the present tense, but it should be in the future tense for the main clause.

### b1-verb-form-present-perfect (erroneous, b1)

- Task: How long have you lived here?
- Answer: Eu moro aqui desde 2019 e nunca tenho visitado o Nordeste.
- Reference: `tenho visitado` -> visitei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `moro` -> `morerei` [verb_form] The present perfect continuous tense is not used in Portuguese for 'how long have you lived here'.
- Found (false positive): `nunca` -> `nunca` [word_choice] The phrase 'nunca tenho visitado' is incorrect because 'nunca' means 'never' and the verb 'visitar' is in the present perfect tense, which is not appropriate here.
- Missed: `tenho visitado`

### b1-agreement-passive (erroneous, b1)

- Task: Describe the cathedral.
- Answer: A catedral foi construída em 1750 e foi restaurado em 2005.
- Reference: `restaurado` -> restaurada [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `foi restaurado` -> `foi restaurado` [verb_form] The verb 'restaurar' is used in the past tense, but the correct past tense for 'restaurar' in this context is the past perfect (pretérito perfeito) or past continuous (pretérito imperfeito) depending on the context. However, in Portuguese, the correct past tense for 'restaurar' in this context is the pretérito perfeito (passado perfeito) for the simple past, but the verb 'restaurar' is not used in the past tense here. The correct verb form for 'restored' in Portuguese is 'reformado' or 'restaurado' in the pretérito perfeito (passado perfeito) tense.
- Missed: `restaurado`

### b1-control-future-subjunctive (control, b1)

- Task: What will you do when you finish the course?
- Answer: Quando eu terminar o curso, vou procurar um emprego novo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `novo` -> `novo` [agreement] The adjective 'novo' should agree with the noun 'emprego' which is masculine, so it should be 'novo' (not 'nova').

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
- Found (false positive): `trabalhar` -> `trabalhar` [spelling] The verb 'trabalhar' is used for the action of working, but in this context, the correct verb for 'to work' in the sense of 'to have a job' is 'trabalhar' (it's correct here).
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
- Found (false positive): `lhe` -> `lhe` [pronoun] The pronoun 'lhe' is used for indirect object pronouns in formal contexts, but in Brazilian Portuguese, the informal 'lhe' is not used here. The correct pronoun for indirect object in informal contexts is 'lhe' but in this context, the correct pronoun is 'lhe' for indirect object. Wait, this is confusing. Let me think again.
- Found (false positive): `não` -> `não` [spelling] The word 'não' is used correctly here as a negation.
- Found (false positive): `esperasse` -> `esperou` [verb_form] The verb 'esperasse' is in the imperfect subjunctive, but in this context, the correct tense is the past perfect (pretérito perfeito) or the simple past (pretérito imperfeito). Wait, the sentence is in the past tense, so the correct verb form is 'esperasse' for the imperfect subjunctive. Let me check.

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
- Found (false positive): `escritório` -> `escritório` [article_preposition] The word needs an article before it: 'um escritório' or 'uma escritório' (but 'escritório' is masculine, so 'um').
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
- Found (false positive): `chama` -> `chama` [not_brazilian] The correct form is 'chama' for 'chamar' (to call), but in Brazilian Portuguese, it is more common to use 'se chama' for formal or neutral contexts. However, at A1 level, 'chama' is acceptable. No mistake here.
- Missed: `chama`

### a1-word-order-adjective (erroneous, a1)

- Task: Describe the clothes Rafael is wearing.
- Answer: Ele está com uma vermelha camisa e uma calça preta.
- Reference: `vermelha camisa` -> camisa vermelha [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vermelha` -> `vermelha` [agreement] The adjective must agree in gender and number with 'camisa' (feminine singular).
- Found (false positive): `preta` -> `preta` [agreement] The adjective must agree in gender and number with 'calça' (feminine singular).
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `muito` -> `muito` [word_choice] 'Muito' is not used to describe the degree of liking something in this context. Use 'muito' with a verb or adjective that can take it, or use 'gosto muito de café' with a different structure.
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
- Found (false positive): `pequeno-almoço` -> `pequeno-almoço` [not_brazilian] This is the European Portuguese spelling; the Brazilian Portuguese spelling is 'pequeno-almoço' without the hyphen.
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
- Found (false positive): `reclamar` -> `reclamar` [word_choice] The verb 'reclamar' should be in the infinitive form 'reclamar' after 'por meio desta' in formal writing.
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

## qwen3:8b / plain

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `escritório` -> `escritório` [article_preposition] The definite article 'o' is missing.
- Missed: `uma escritório`

### a1-serestar-nationality (erroneous, a1)

- Task: Where are you from?
- Answer: Eu estou brasileiro e moro em Recife.
- Reference: `estou brasileiro` -> sou brasileiro [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `estou` -> `sou` [ser_estar] The verb 'estar' is not used to indicate origin.
- Found (false positive): `brasileiro` -> `brasileiro` [word_choice] The word 'brasileiro' is not used in Brazilian Portuguese to refer to someone from Brazil.

### a1-serestar-tired (erroneous, a1)

- Task: How do you feel after work?
- Answer: Depois do trabalho eu sou cansado e com fome.
- Reference: `sou cansado` -> estou cansado [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `sou` -> `estou` [ser_estar] The verb 'ser' is not used to describe temporary states like being tired.
- Found (false positive): `cansado` -> `cansado` [not_brazilian] The adjective 'cansado' is masculine, but the subject is 'eu' (masculine), so it is correct. However, in Brazilian Portuguese, 'cansado' is more commonly used with 'eu' than 'cansada'.

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
- Found (false positive): `vermelha` -> `vermelha` [agreement] The adjective 'vermelha' should agree in gender and number with the noun 'camisa', which is feminine singular. Therefore, it should be 'vermelha' (correct form).
- Found (false positive): `calça` -> `calça` [not_brazilian] The noun 'calça' is feminine, but the article 'uma' is used, which is correct. However, in Brazilian Portuguese, 'calça' is often used with the masculine article 'um' in informal contexts, though it's not a strict rule. This is not a mistake but a style choice.
- Missed: `vermelha camisa`

### a1-word-order-adverb (erroneous, a1)

- Task: Do you like coffee?
- Answer: Sim, eu gosto de café muito.
- Reference: `gosto de café muito` -> gosto muito de café [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `muito` -> `muito` [word_order] The word 'muito' is not used correctly in this context. It should be 'muito' as an adverb, but it is not properly placed.
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
- Found (false positive): `estudei` -> `estudei` [verb_form] The verb should be in the past historic tense for a narrative context.
- Found (matched): `portugues` -> `português` [spelling] The word is missing an accent mark.

### a2-verb-form-imperfect (erroneous, a2)

- Task: Describe your childhood.
- Answer: Quando eu era criança, eu moro numa cidade pequena e brincava na rua.
- Reference: `moro` -> morava [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `moro` -> `morava` [verb_form] The verb should be in the past tense because the sentence is about a childhood memory.
- Found (false positive): `brincava` -> `brincava` [verb_form] The verb should be in the past tense to match the rest of the sentence.

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
- Found (false positive): `nós` -> `nós` [word_choice] nós is not used in Brazilian Portuguese in this context with the verb 'ajudamos' when referring to a general habit.
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
