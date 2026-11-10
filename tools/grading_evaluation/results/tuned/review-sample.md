# Grading review sample

The same cases for every run, for rating the explanations by hand.

## aya-expanse:8b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sou em casa`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chamo`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo.` -> `Eu comi cereal, pão com manteiga ou um café da manhã típico.` [content_mismatch] This answer does not describe what the person ate for breakfast.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `atendi`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `está chovendo e faz frio` [word_choice] The correct phrase would be 'está chovendo' given the scene facts.
- Found (duplicate): `muito calor` -> `frio` [content_mismatch] The weather is described as cold, not hot.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true

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

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu me chamo Ana e moro em Curitiba.` -> `Meu nome é Ana e eu moro em Curitiba.` [word_order] The sentence is grammatically correct, but it's a bit formal for a casual conversation at the A1 level.

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `em casa com a minha família.` -> `em casa com minha família.` [article_preposition] The correct preposition to use with 'em casa' is 'em', not 'com'.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `Eu chamo Pedro e tenho vinte anos.` -> `Meu nome é Pedro, tenho 20 anos.` [word_order] The sentence is grammatically correct, but it's a formal way to introduce oneself. For a more casual A1 level, a simpler phrase would be more appropriate.

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo.` -> `Comi uma torrada com manteiga e café.` [content_mismatch] This sentence does not answer the question 'What did you eat for breakfast?',

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No fim de semana eu visitei meus avós e nós almoçamos juntos.` -> `No fim de semana eu visitei meus avós e eu almocei com eles.` [word_order] The phrase 'nós almoçamos juntos' is grammatically correct, but 'visitei meus avós e nós' is awkward. It should be 'visitei meus avós e eu almocei com eles.'
- Missed: `volti`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi um filme novo` -> `assisti um filme novo` [verb_form] The correct verb conjugation should be 'assisti' (past participle of 'assistir', meaning 'watched')

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Hoje está fazendo sol e muito calor.` -> `Hoje está chovendo e faz frio.` [content_mismatch] The answer contradicts the given scene facts, stating that it is sunny and hot instead of rainy and cold.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `gostei muito dele` -> `gostei muito dele.` [word_order] The correct expression would be 'gostei muito dele' (I liked it very much).

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `se eu ganhava` -> `se eu ganhasse` [verb_form] The correct verb conjugation is 'se eu ganhasse'.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `e viajaria pelo mundo.` -> `e viajaria pelo mundo se tivesse a oportunidade.` [verb_form] The correct verb form should be 'viajaria' (I would travel) to match the conditional tone of the sentence.

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `do atraso na entrega do meu pedido` -> `atraso na entrega do meu pedido` [word_order] The correct phrase is 'atraso na entrega do meu pedido'.

## gemma3:12b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `sou em` -> `estou em` [article_preposition] The correct preposition to use with 'casa' is 'em'.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chamo`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' needs to be in the pretérito perfeito tense here.

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `assisti` [word_choice] The verb 'assistir' (to watch) is needed here, not 'atender' (to attend to).

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

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `ganhava` -> `ganhasse` [verb_form] In a hypothetical situation, the imperfect subjunctive 'ganhasse' is needed, not the preterite indicative.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `no dez` [article_preposition] In formal writing, 'no dia' is often omitted when referring to a specific date.
- Missed: `sobre do`

## gemma3:12b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `sou em` -> `estou em` [ser_estar] The correct preposition to use with 'casa' is 'em' when indicating location, but 'ser' is incorrect; 'estar' is needed to describe location.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `Eu chamo` -> `Meu nome é` [word_choice] The correct way to introduce yourself is 'Meu nome é'.

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol` -> `Eu comi...` [content_mismatch] The question asks what you ate, not what you like.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb needs to be in the pretérito perfeito tense to describe a completed action in the past.

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `assisti` [word_choice] The verb 'atender' means 'to assist' or 'to attend to', not 'to watch'.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `fazendo sol` -> `ensolarado` [word_choice] It's not natural to use 'fazer' with 'sol' in this context; 'estar' is used to describe the weather.
- Found (duplicate): `muito calor` -> `muito frio` [content_mismatch] The scene states it is cold, so 'muito calor' (very hot) is incorrect.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> `` [word_choice] The pronoun 'dele' is unnecessary and sounds unnatural after 'gostei'.

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `ganhava` -> `ganhasse` [verb_form] The conditional tense is needed here, not the imperfect.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `em dez` [article_preposition] The correct preposition to use with dates is 'em'.
- Missed: `sobre do`

## gemma3:1b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sou em casa`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chamo`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `atendi`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fazendo sol e muito calor`

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true

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

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `Eu` -> `Eu` [word_choice] The verb 'me chamar' (to call me) is used for a person, not a thing. 'Eu' is the subject of the sentence.

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `Agora` -> `Agora` [verb_form] This is the correct form of the verb 'to be' in Portuguese. 'Agora' means 'now'.
- Missed: `sou em casa`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (matched): `chamo` -> `eu` [verb_form] The verb 'chamar' (to call) is used for a person, not a thing. It should be 'eu chamo' (I call).

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `gosto` -> `gosto` [verb_form] The verb 'gostar' is used for liking, not 'to like'.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `visitei` -> `visitamos` [spelling] The verb 'visitar' means 'to visit'. 'Ver
- Missed: `volti`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (matched): `atendi` -> `estudei` [verb_form] This verb means 'to attend' or 'to watch'. It’s the wrong verb form for 'to watch a movie'.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (matched): `fazendo` -> `está` [verb_form] This verb is used to describe an action that is happening now, but it’s not the correct verb to use when describing the weather.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `gostei` -> `gostei de` [verb_form] This verb is used to express a feeling or opinion, not a simple statement of liking.

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `comprar` -> `ganhar` [verb_form] The verb 'comprar' means 'to buy'. 'Ganhar' means 'to win'. The correct verb to use is 'ganhar' to indicate winning.
- Missed: `ganhava`

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (false positive): `comprar` -> `gain` [verb_form] The verb 'comprar' means 'to buy'. 'Ganhar' means 'to win'. The correct verb to use is 'ganhar' to indicate winning.

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=false
- Found (matched): `do` -> `de` [spelling] The word 'do' is used incorrectly in this context. It should be 'de' to indicate the time.

## gemma3:4b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `sou em casa` -> `estou em casa` [ser_estar] The correct expression is 'estou em casa'.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Eu chamo` -> `Meu nome é` [word_choice] This phrase is not appropriate for answering 'What is your name?'

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de` -> `Eu comi` [word_choice] This phrase is not appropriate for answering the question.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `atendi` -> `assisti` [verb_form] The verb 'atender' is not appropriate in this context; 'assisti' is needed.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `está chovendo` [word_choice] The weather is raining, so 'fazendo sol' is incorrect.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> `` [word_choice] The pronoun 'dele' is not needed here.

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
- Found (false positive): `ganhasse` -> `ganhasse` [verb_form] The conditional form needs to be 'ganhasse'.

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `do meu pedido` -> `do meu pedido` [word_choice] The phrase 'do meu pedido' is redundant and less formal in this context.
- Missed: `sobre do`

## gemma3:4b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A Marina está` -> `Marina está` [article_preposition] The use of 'A' before Marina is incorrect.

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu` -> `Me` [word_choice] The question asks for a name, not a self-introduction.

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `sou` -> `estou` [ser_estar] ‘Sou’ is used to describe identity, not location.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `Eu chamo` -> `Qual é o seu nome?` [word_choice] The correct phrase to ask for someone's name is 'Qual é o seu nome?'
- Found (false positive): `e tenho vinte anos.` -> `` [content_mismatch] The question asks for the name, not the age.

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu gosto de futebol` -> `Eu comi` [content_mismatch] This response answers a question about breakfast, not a preference for football.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Finalmente` [article_preposition] ‘No’ is not the correct way to start this sentence in Brazilian Portuguese.
- Missed: `volti`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `assisti` [word_choice] The verb 'atender' means 'to attend' or 'to serve', not 'to watch'.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `fazendo sol` -> `fazendo chuva` [word_choice] The scene states it is raining, so 'sol' (sun) is incorrect.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> `do filme` [word_choice] The pronoun 'dele' is unnecessary and redundant in this context.

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `Se eu ganhava` -> `Se eu ganhasse` [verb_form] The conditional 'Se eu ganhasse' is needed after 'Se'.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ganhasse` -> `ganhasse` [verb_form] The conditional form of 'ganhar' (to win) needs the subjunctive mood after 'se'.

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `do atraso` -> `do atraso` [word_choice] ‘Do atraso’ is not a correct phrase in this context; ‘do atraso’ is a noun, not a correct way to express ‘of the delay’.
- Missed: `sobre do`

## llama3.1:8b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `me` -> `` [verb_form] The pronoun 'me' is used as an object pronoun, but in this sentence, it should be 'chamo' without a pronoun, as it's a verb conjugation.

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sou em casa`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vinte anos` -> `vinte anos` [word_choice] The correct phrase is 'vinte anos de idade' or simply 'vinte anos' without 'de idade'.
- Missed: `chamo`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `gostei` [verb_form] The verb 'gosto' is in the present tense, but the question is about the past.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Response: invalid or missing

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `assisti` [word_choice] The verb 'atender' is not the correct choice for watching a movie. The correct verb is 'assistir'.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `sol e muito calor` -> `` [content_mismatch] The weather is described as cold and rainy, so this statement contradicts the facts.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> `dela` [agreement] The pronoun 'dele' is masculine and should be 'dela' to agree with the feminine noun 'filme'.

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
- Found (false positive): `ganhasse` -> `ganhasse` [verb_form] The verb 'ganhar' is in the wrong form for the subjunctive mood.

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `do` -> `` [article_preposition] The word 'do' is not needed here, and the correct preposition is 'da' or no preposition at all.

## llama3.1:8b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Response: invalid or missing

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `me` -> `` [verb_form] The verb 'chamar' is an intransitive verb, so it doesn't take a reflexive pronoun.
- Found (false positive): `e` -> `` [article_preposition] The conjunction 'e' is not needed here.

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `sou` -> `estou` [verb_form] The verb 'sou' is in the wrong form for the subject 'eu'.
- Found (duplicate): `em casa` -> `na casa` [article_preposition] The preposition 'em' is not needed before 'casa'.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `vinte` -> `vinte e um` [word_choice] The number should be written in words for this level.
- Missed: `chamo`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo` -> `` [content_mismatch] This sentence does not answer the question about breakfast.

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

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `vi` [verb_form] The verb 'atender' is not used in this context.
- Found (false positive): `um filme novo` -> `um filme novo` [agreement] The adjective 'novo' agrees with the masculine singular noun 'filme', but the article 'um' is used instead of 'um filme'.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `sol` -> `` [content_mismatch] It is raining, so the sun is not shining.
- Found (duplicate): `muito calor` -> `` [content_mismatch] It is cold, so it is not hot.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `gostei muito dele` -> `gostei muito` [verb_form] The verb 'gostei' is in the wrong form for the subject 'eu'.

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `ganhava` -> `ganharia` [verb_form] The correct verb form for the conditional tense is 'ganharia'.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ganhasse` -> `ganhas` [verb_form] The verb should be in the conditional tense, not the future tense.

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Venho` -> `Vieram` [verb_form] The verb 'venho' is in the present tense, but the situation is in the past.
- Found (matched): `do` -> `` [article_preposition] The word 'do' is not needed here.

## llama3.2:3b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de maiô` -> `de banho` [content_mismatch] The correct phrase is 'de banho' or 'de nadar', as 'maiô' is a specific type of swimsuit.

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sou em casa`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chamo`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `futebol` -> `futebol` [word_choice] The correct word is 'futebol' (the sport), not 'futebol' (the team).

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `visitei` -> `visitado` [verb_form] The verb should be in the past participle form to agree with the present perfect tense.
- Missed: `volti`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `atendi` -> `assisti` [verb_form] The verb should be in the past participle form to agree with 'on

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo` -> `está` [ser_estar] The weather is not sunny, so 'fazendo' is incorrect. It should be 'está' instead.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true

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
- Found (quote not in the answer): `ganasse` -> `ganharia` [verb_form] The verb conjugation is incorrect, it should be 'ganharia'.

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `do atraso` -> `atraso` [article_preposition] The preposition 'de' is not needed here.

## llama3.2:3b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de` -> `usando` [ser_estar] should be 'usando' to indicate she is wearing the clothes

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `moro` -> `morro` [word_choice] should be 'morro'

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `sou` -> `estou` [verb_form] should be 'estou'

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `chamo` -> `tenho` [verb_form] should be 'tenho'

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `futebol` -> `comida` [word_choice] football is not typically eaten for breakfast

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

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `assisti` [verb_form] should be 'assisti' because 'atender' is the infinitive form of 'atender' and not the present perfect tense
- Found (false positive): `novo` -> `novo` [word_choice] should be 'novo' because 'novi' is the present perfect tense of 'novar' and not the adjective 'novo'
- Found (false positive): `no` -> `no` [article_preposition] should be 'no' because 'no' is a determiner and not a preposition

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `sol` -> `chova` [content_mismatch] rain is not making the sun shine
- Found (duplicate): `calor` -> `frio` [content_mismatch] it is cold, not hot

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ontem` -> `hier` [content_mismatch] should be 'hier' or 'ontem' depending on the day of the week

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `ganhava` -> `ganharia` [verb_form] should be 'ganharia' because the subject is 'eu' (I) and the verb is in the future perfect tense

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `na` -> `a` [agreement] should be 'a' because 'loteria' is a feminine noun

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `do` -> `de` [word_choice] should be 'de' instead of 'do'
- Found (quote not in the answer): `d dia` -> `no dia` [word_order] should be 'no dia' instead of 'no dia'
- Found (false positive): `dez` -> `dezimo` [spelling] should be 'dezimo' instead of 'dez'

## phi4-mini / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sou em casa`

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chamo`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto de futebol` -> `gosto de futebol` [word_choice] The verb 'gostar' should be followed by 'de' when talking about liking something.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `atendi`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `está chuvoso e frio` [word_choice] The phrase should describe cold and rainy weather, not sunny and hot.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true

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

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `me chamo` -> `Eu me chamo Ana.` [word_choice] The verb 'chamar' is not used to mean 'to call' or 'to name oneself'. The correct verb is 'ter' or 'se nomear'.
- Found (quote not in the answer): `me moro` -> `Eu me chamo Ana.` [word_choice] The verb 'morar' means 'to live' but it is not used to mean 'to call oneself'. The correct verb is 'ter' or 'se nomear'.

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Agora eu sou` -> `Estou` [ser_estar] The verb 'ser' is used instead of the reflexive 'estar' which is needed in this context.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Eu chamo Pedro` -> `Meu nome é Pedro` [content_mismatch] The answer contains unnecessary information about the name.

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol` -> `Eu comecei o café com pão e iogurte.` [word_choice] The answer talks about liking football instead of answering what was eaten for breakfast.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `visitou` -> `visitei` [verb_form] The verb should be in the past tense to match the context of the sentence.
- Missed: `volti`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `ontem` -> `ontou` [verb_form] The word should be in the past tense to match the question about 'last night'.
- Found (matched): `atendi` -> `ateste` [verb_form] The verb should be in the past tense to match the question about 'last night'.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol e muito calor` -> `it is raining and cold.` [content_mismatch] The description is opposite to the scene facts, which state it is raining and cold.
- Found (false positive): `Hoje está` -> `Hoje está chovendo.` [word_choice] The phrase should indicate that it is raining, not sunny.
- Found (duplicate): `muito calor` -> `muito frio.` [word_choice] The description is opposite to the scene facts, which state it is cold.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu vi o filme` -> `Eu vi o filme` [spelling] The word 'filme' should be in lowercase.
- Found (false positive): `e gostei muito dele` -> `do dele` [agreement] The word 'dele' should be 'do dele' to match the masculine singular form.

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

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `ganhasse` -> `ganharia` [verb_form] The verb 'ganhar' should be conjugated to 'ganhar' in the first person singular in the conditional tense.

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

## qwen3.5:latest / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está` -> `` [word_choice] To describe what someone is wearing, the preposition 'de' is used without a verb.

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `sou` -> `estou` [ser_estar] The verb 'ser' is incorrect here; location requires the verb 'estar'.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chamo`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `comi` [verb_form] The verb tense is wrong; the question asks about the past (breakfast), but 'gosto' is present tense.
- Found (false positive): `futebol` -> `pão` [word_choice] This word does not answer the question about what was eaten for breakfast.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `avós` -> `avôs` [spelling] The noun 'avô' is masculine, so the plural must be 'avôs', not 'avós'.
- Missed: `volti`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> ` assisti` [word_choice] The verb 'assistir' is needed for watching a movie, not 'atender'.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo` -> `está` [word_choice] The verb 'fazer' is not used to describe weather in Brazilian Portuguese; use 'está'.
- Found (duplicate): `sol` -> `chovendo` [content_mismatch] The scene says it is raining, but the answer says it is sunny.
- Found (duplicate): `calor` -> `frio` [content_mismatch] The scene says it is cold, but the answer says it is hot.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `ganhava` -> `tivesse ganhado` [verb_form] The imperfect tense is incorrect here; the conditional perfect or pluperfect subjunctive is needed for a hypothetical situation.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `loteria` -> `loteria` [spelling] The word needs an accent on the i: loteria.

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `sobre do` -> `do` [article_preposition] The preposition 'de' and the article 'o' should be contracted into 'do', so 'sobre de' is incorrect here; it should be just 'do'.

## qwen3.5:latest / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `sou` -> `estou` [ser_estar] The verb 'ser' is incorrect here because location requires the verb 'estar'.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chamo`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `comi` [word_choice] The verb 'gostar' means 'to like', but the question asks what was eaten for breakfast.
- Found (false positive): `futebol` -> `pão` [word_choice] 'Futebol' means football, which is not something one eats for breakfast.
- Found (false positive): `Flamengo` -> `ovo` [content_mismatch] The Flamengo football team cannot be eaten for breakfast.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `vi` [word_choice] The verb 'atender' means to answer the phone or to serve a customer; it is not used for watching movies.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `chovendo` [content_mismatch] This contradicts the fact that it is raining.
- Found (duplicate): `muito calor` -> `frio` [content_mismatch] This contradicts the fact that it is cold.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `ganhava` -> `ganhasse` [verb_form] The imperfect tense is incorrect here; the conditional perfect or simple conditional structure is needed for a hypothetical situation.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `sobre do` -> `sobre o` [article_preposition] The preposition 'de' is incorrectly combined with the article 'o', creating a redundant contraction.

## qwen3:1.7b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `de maiô vermelho` -> `com maiô vermelho` [word_choice] The preposition 'de' should be replaced with 'com' to correctly express 'wears' in Portuguese.

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `sou em casa` -> `estou em casa` [ser_estar] The phrase 'sou em casa' is incorrect. The correct form is 'estou em casa' to match the present tense and the subject 'eu'.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chamo`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `atendi`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol e muito calor` -> `está chuvendo e muito frio` [content_mismatch] The weather is raining and cold, not sunny and hot.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true

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

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `sobre do`

## qwen3:1.7b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A Marina está de maiô vermelho` -> `A Marina está com um maiô vermelho` [spelling] The phrase 'de maiô' is not correct in Brazilian Portuguese. The correct form is 'com maiô' or 'em um maiô' when referring to wearing a swimsuit.
- Found (false positive): `chapéu de palha e sandálias brancas` -> `chapéu de palha e sandálias brancas` [content_mismatch] The phrase 'chapéu de palha' is correct, but 'sandálias brancas' is not correct. The correct form is 'sandálias brancas' (with a 'c' instead of a 'ç' in 'sandálias'), but in this context, the mistake is not a spelling error but a false friend. However, since the answer is written in Portuguese and the task is to describe the clothes, this is a content mismatch.

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu me chamo Ana e moro em Curitiba.` -> `Resido em Curitiba.` [word_choice] The answer is correct in meaning, but the phrase 'moro' is not commonly used in Brazilian Portuguese. The more natural phrase would be 'resido' or 'livo em'.

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `Agora eu sou em casa com a minha família` -> `Estou em casa` [ser_estar] The phrase 'sou em casa' is incorrect. The correct form is 'estou em casa' to indicate being at home at the moment.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tenho vinte anos` -> `replace with the person's name` [content_mismatch] The phrase 'tenho vinte anos' is correct, but the task is asking for the person's name, not their age. The answer should focus on the name, not the age.
- Missed: `chamo`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo.` -> `` [content_mismatch] The answer discusses preferences and a favorite team, but it does not answer the question about what was eaten for breakfast.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Ontem à noite` -> `Ontem` [ser_estar] This is a correct form of 'last night' in Brazilian Portuguese.
- Found (false positive): `atendi` -> `atendi` [verb_form] The verb 'atender' is typically conjugated as 'atendi' in the third person singular, but 'atendi' is not the correct form here. The correct form is 'atendi' (third person singular) or 'atendei' (first person singular).
- Found (false positive): `um filme novo` -> `o filme novo` [agreement] The article 'um' is correct here, but 'novo' should be preceded by 'o' (the article) to match the gender and number of 'filme' (which is masculine).
- Missed: `atendi`

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `Hoje está fazendo sol e muito calor.` -> `Hoje está chuvendo e muito frio.` [content_mismatch] The answer incorrectly states that it is sunny and hot, but the scene facts say it is raining and cold.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.` -> `Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.` [spelling] The answer is correct in meaning, but it contains a spelling mistake: 'viajaria' should be 'viajaria' (correct form) or 'viajaria' (correct form). However, the answer is already correct in terms of spelling and grammar. No mistake is reported here.
- Missed: `ganhava`

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.` -> `viajaria ao redor do mundo` [word_choice] The answer is correct in meaning, but the phrase 'viajaria pelo mundo' is not commonly used in Brazilian Portuguese. A more natural expression would be 'viajaria ao mundo' or 'viajaria ao redor do mundo'.

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `do atraso na entrega do meu pedido` -> `de atraso na entrega do meu pedido` [ser_estar] The preposition 'do' is incorrect. The correct preposition for 'late delivery' in this context is 'de' (de + delivery).

## qwen3:4b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `sou` -> `estou` [verb_form] The verb 'ser' is used for identity, but here we need 'estou' for location.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `chamo`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `gostei` [verb_form] The verb 'gosto' is present tense, but the question asks for past tense.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' is conjugated incorrectly; it should be 'voltei' for past tense.

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `assisti` [verb_form] The verb should be in the past perfect tense (pretérito perfeito) for 'did', but 'atendi' is the past tense of 'atender' (to answer).

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo` -> `está fazendo` [verb_form] The verb 'fazer' in the present continuous tense should be 'está fazendo' for 'it is raining'.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `ganhava` -> `ganhasse` [verb_form] The verb should be in the conditional tense, not the imperfect tense.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ganhasse` -> `ganhasse` [verb_form] The conditional tense should be 'ganhasse' for 'eu' in the past subjunctive, but the correct form is 'ganhasse' (with an accent).

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `reclamar sobre do` -> `reclamar sobre o` [article_preposition] The preposition 'sobre' does not require 'do' here; it should be 'reclamar sobre o atraso'.

## qwen3:4b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maiô` -> `banho` [word_choice] In Brazilian Portuguese, 'swimsuit' is 'banho' or 'roupa de banho', not 'maiô'.
- Found (false positive): `chapéu` -> `palha` [word_choice] In Brazilian Portuguese, 'hat' is 'chapéu' but it's not used for a straw hat in this context. The correct term for a straw hat is 'palha' or 'chapéu de palha'.

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `sou` -> `estou` [verb_form] The verb 'ser' is used for permanent states, but 'estar' is needed for temporary locations.
- Found (duplicate): `em` -> `na` [article_preposition] The preposition 'em' is used for locations in Portuguese, but it should be 'na' for 'in the house' (since 'casa' is feminine).

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `chamo` -> `chamo` [verb_form] The verb 'chamar' is used in the present tense for 'I call', but here it's used in the past tense for 'I am called'.
- Found (false positive): `vinte` -> `vinte-` [spelling] The number 'twenty' in Portuguese is 'vinte' but it should be written with a hyphen as 'vinte-'.
- Missed: `chamo`

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' is conjugated with the past tense, but 'volti' is incorrect. The correct form is 'voltei'.

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `assisti` [word_choice] The verb 'atender' is used for 'to attend' (e.g., a call), but here it should be 'assistir' for 'to watch a movie'.
- Found (false positive): `filme` -> `filme` [agreement] In Brazilian Portuguese, 'filme' is masculine, so it should be 'filmes' for plural (since 'um filme novo' implies a specific movie, but the context suggests a general action of watching a movie, so it should be singular).

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `fazendo` -> `chovendo` [content_mismatch] The scene says it is raining, but the answer says it is sunny
- Found (duplicate): `muito` -> `frio` [content_mismatch] The scene says it is cold, but the answer says it is hot

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `ganhava` -> `ganhasse` [verb_form] The verb 'ganhar' should be in the present tense for a hypothetical situation, not the imperfect tense.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ganhasse` -> `ganhar` [verb_form] The conditional tense should be 'ganhar' for the past conditional, but 'ganhasse' is the imperfect subjunctive
- Found (false positive): `compraria` -> `comprar` [verb_form] The conditional tense should be 'comprar' for the past conditional, but 'compraria' is the imperfect subjunctive
- Found (false positive): `viajaria` -> `viajar` [verb_form] The conditional tense should be 'viajar' for the past conditional, but 'viajaria' is the imperfect subj. tense

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `do` -> `da` [article_preposition] The preposition 'de' is missing the contraction with 'a' to form 'da' (from the)
- Found (false positive): `que` -> `o` [article_preposition] The relative pronoun 'que' should be 'que' (correct) but in this context, it's better to use 'o' for specificity, but the main issue is the contraction of 'de' with 'a'

## qwen3:8b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `sou` -> `estou` [ser_estar] 'Sou' is the conjugation of 'ser', but 'estar' is needed to describe location.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `chamo` -> `me chamo` [word_choice] The verb 'chamar' should be in the first person singular present tense: 'chamo' is correct, but the sentence structure is incorrect for answering 'What is your name?'

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `gostei` [verb_form] The verb should be in the past tense to match the question about what was eaten.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' should be in the preterite tense 'voltei' for first person singular.

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `assistir` [word_choice] The verb 'assistir' is used for watching a movie, not 'atender' which means 'to attend' or 'to take care of'.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `fazendo chuva` [content_mismatch] This contradicts the scene fact that it is raining.
- Found (duplicate): `muito calor` -> `muito frio` [content_mismatch] This contradicts the scene fact that it is cold.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `ganhava` -> `ganhasse` [verb_form] The verb form should be conditional, not past perfect.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `reclamar` -> `reclamar` [word_choice] The verb 'reclamar' should be in the infinitive form 'reclamar' after 'por meio desta' in formal writing.
- Found (matched): `do` -> `` [article_preposition] 'Do' is not used in this context; it should be omitted.

## qwen3:8b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-control-reflexive-name (control, a1)

- Task: What is your name?
- Answer: Eu me chamo Ana e moro em Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-serestar-location-now (erroneous, a1)

- Task: Where are you now?
- Answer: Agora eu sou em casa com a minha família.
- Reference: `sou em casa` -> estou em casa [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `sou` -> `estou` [ser_estar] The verb 'ser' is used where 'estar' should be for location.

### a1-pronoun-reflexive-name (erroneous, a1)

- Task: What is your name?
- Answer: Eu chamo Pedro e tenho vinte anos.
- Reference: `chamo` -> me chamo [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `chamo` -> `me chamo` [word_choice] The correct form is 'chamo' for 'chamar' when using 'eu', but in Brazilian Portuguese, it's more common to say 'me chamo' for a more natural and polite expression.

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de futebol e meu time favorito é o Flamengo.` -> `Eu comi pão e café da manhã.` [content_mismatch] The answer does not address the question about what was eaten for breakfast.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' should be in the preterite tense for 'eu' (voltei) instead of the imperfect tense (volti).

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `vi` [word_choice] The verb 'atender' is not used to mean 'watch a movie'.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `fazendo sol` -> `fazendo chuva` [content_mismatch] This phrase describes sunny weather, which contradicts the scene fact that it is raining.
- Found (duplicate): `muito calor` -> `muito frio` [content_mismatch] This phrase describes hot weather, which contradicts the scene fact that it is cold.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `ganhava` -> `ganhasse` [verb_form] The verb form should be in the conditional tense, not the imperfect tense.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `reclamar sobre do` -> `reclamar por` [article_preposition] The preposition 'sobre' is incorrectly used with 'reclamar'; the correct preposition is 'por' or 'contra'.
- Found (false positive): `deveria ter chegado no dia dez` -> `no dia dez` [article_preposition] In Brazilian Portuguese, it is more natural to say 'no dia dez' without the preposition 'no' when referring to a specific date.
