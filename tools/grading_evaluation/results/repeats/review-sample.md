# Grading review sample

The same cases for every run, for rating the explanations by hand.

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
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' needs to be in the pretérito perfeito tense.

### a2-word-choice-atender (erroneous, a2)

- Task: What did you do last night?
- Answer: Ontem à noite eu atendi um filme novo no cinema.
- Reference: `atendi` -> assisti [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `atendi` -> `assisti` [verb_form] The verb 'assistir' (to watch) is needed here, not 'atender' (to attend to).

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `está chovendo` [content_mismatch] It is not sunny, it is raining.
- Found (duplicate): `calor` -> `frio` [content_mismatch] It is not hot, it is cold.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dele` -> ` ` [article_preposition] 'Dele' is not needed after 'gostei'.

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `ganhava` -> `ganhasse` [verb_form] In a hypothetical situation, the imperfect subjunctive 'ganhasse' is needed, not the preterite.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no dia dez` -> `no dia 10` [article_preposition] In formal writing, it's better to use the full date.
- Missed: `sobre do`

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
- Found (matched): `sou` -> `estou` [ser_estar] The verb ser is used incorrectly here because location requires the verb estar.

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
- Found (false positive): `futebol` -> `café da manhã` [word_choice] 'Futebol' means football, which is not something one eats for breakfast.
- Found (false positive): `Flamengo` -> `` [content_mismatch] The Flamengo football team is irrelevant to the question about food eaten for breakfast.

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
- Found (matched): `atendi` -> `assisti` [word_choice] The verb 'atender' means to answer a phone or attend to someone; the correct verb for watching a movie is 'assistir'.

### a2-content-weather (erroneous, a2)

- Task: Describe the weather today.
- Answer: Hoje está fazendo sol e muito calor.
- Reference: `fazendo sol e muito calor` -> chovendo e fazendo frio [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `fazendo sol` -> `chovendo` [content_mismatch] This contradicts the scene fact that it is raining.
- Found (duplicate): `muito calor` -> `frio` [content_mismatch] This contradicts the scene fact that it is cold.

### a2-control-object-pronoun (control, a2)

- Task: What did you think of the film?
- Answer: Eu vi o filme ontem e gostei muito dele.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-verb-form-imperfect-subjunctive (erroneous, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhava na loteria, eu compraria uma casa e viajaria pelo mundo.
- Reference: `ganhava` -> ganhasse [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `ganhava` -> `ganhasse` [verb_form] The conditional perfect 'would win' requires the imperfect tense of the verb 'ganhar' in this hypothetical structure.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-preposition-sobre-do (erroneous, b2)

- Task: Write a short formal complaint about a late delivery.
- Answer: Venho por meio desta reclamar sobre do atraso na entrega do meu pedido, que deveria ter chegado no dia dez.
- Reference: `sobre do` -> do | sobre o [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `sobre do` -> `sobre o` [article_preposition] The preposition 'de' is incorrectly doubled before the noun 'atraso'.

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
- Found (matched): `chamo` -> `me chamo` [word_choice] The correct form is 'chamo' for 'chamar' with 'eu', but in Brazilian Portuguese, it's more common to say 'me chamo' for a more natural expression.
- Found (false positive): `tenho vinte anos` -> `tenho vinte anos` [word_order] This is incorrect; the correct phrase is 'tenho vinte anos' (which is correct), but the structure 'tenho vinte anos' is correct. However, in Brazilian Portuguese, it's more natural to say 'tenho vinte anos' (which is correct).

### a1-off-topic-football (off_topic, a1)

- Task: What did you eat for breakfast?
- Answer: Eu gosto de futebol e meu time favorito é o Flamengo.
- Flags: written_in_portuguese=true answers_the_question=false

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltagei` [verb_form] The verb 'voltar' should be in the preterite tense for 'eu' (voltagei) in Brazilian Portuguese.

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
- Found (matched): `fazendo sol` -> `fazendo chuva` [content_mismatch] fazendo sol means 'sunny', which contradicts the scene fact that it is raining.
- Found (duplicate): `muito calor` -> `muito frio` [content_mismatch] muito calor means 'very hot', which contradicts the scene fact that it is cold.

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
- Found (matched): `reclamar sobre do` -> `reclamar sobre` [word_order] The preposition 'sobre' is redundant here.
- Found (false positive): `deveria ter chegado no dia dez` -> `no dia dez` [article_preposition] In Brazilian Portuguese, it is more natural to say 'no dia dez' without 'no' before 'dia'.
