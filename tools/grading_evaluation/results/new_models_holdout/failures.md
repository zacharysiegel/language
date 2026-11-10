# Grading failures

Every call with a missed error, a false alarm, a quote not in the answer, a wrong flag, or an invalid response.

## gemma3n:e2b / few_shot

### h-a2-control-yesterday-park (control, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu fui ao parque com minha irmã e nós jogamos bola.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `minha irmã` [pronoun] The pronoun 'nós' is incorrect here. The verb 'jogamos' needs to agree with the subject 'minha irmã'.

### h-a1-spelling-sao-paulo (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Brasil e moro em Sao Paulo.
- Reference: `Sao` -> São [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Sao`

### h-a1-agreement-black-cat (erroneous, a1)

- Task: Describe your cats.
- Answer: Meu gato é pequeno e minha gata é preto.
- Reference: `preto` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `preto`

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu tem`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `assisto a rádio`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [word_choice] The word 'branca' is invariable.
- Missed: `branca casa`

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `o guarda-chuva` -> `o guarda-chuva` [article_preposition] The article 'o' is unnecessary before 'guarda-chuva'.
- Missed: `de o`

### h-a2-pronoun-reflexive-hurt (erroneous, a2)

- Task: What happened to you?
- Answer: Eu machuquei ontem jogando futebol.
- Reference: `Eu machuquei` -> Eu me machuquei [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu machuquei`

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-not-brazilian-train (erroneous, a2)

- Task: How did you travel to the city?
- Answer: Eu viajei de comboio até a cidade.
- Reference: `comboio` -> trem [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `até a` -> `para` [article_preposition] The preposition 'a' is unnecessary here.
- Missed: `comboio`

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `uma pizza` -> `uma pizza` [word_choice] The word 'uma' is unnecessary and incorrect.
- Missed: `perguntei`

### h-b1-verb-form-future-subjunctive-querer (erroneous, b1)

- Task: Will you help me?
- Answer: Se você querer, eu ajudo você a mudar.
- Reference: `Se você querer` -> Se você quiser [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `mudar` -> `mudar` [verb_form] The verb 'ajudar' needs to be conjugated to the infinitive form.
- Missed: `Se você querer`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a escola`

### h-a1-off-topic-music (off_topic, a1)

- Task: What is your name?
- Answer: Eu gosto de música brasileira e toco violão.
- Flags: written_in_portuguese=true answers_the_question=true

### h-b1-off-topic-rain (off_topic, b1)

- Task: Do you think cities need more bicycle lanes?
- Answer: Ontem choveu muito e eu fiquei em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Ontem` -> `Hoje` [verb_form] The verb 'fiquei' is not the correct form to use with 'ontem'.

## gemma3n:e2b / few_shot_six

### h-a2-control-yesterday-park (control, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu fui ao parque com minha irmã e nós jogamos bola.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `nós` -> `nós` [pronoun] The pronoun should be 'nós' when referring to the group 'nós'.

### h-a1-spelling-sao-paulo (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Brasil e moro em Sao Paulo.
- Reference: `Sao` -> São [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Sao`

### h-a1-agreement-black-cat (erroneous, a1)

- Task: Describe your cats.
- Answer: Meu gato é pequeno e minha gata é preto.
- Reference: `preto` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `preto`

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu tem`

### h-a1-serestar-father-location (erroneous, a1)

- Task: Where is your father?
- Answer: Meu pai é no trabalho agora.
- Reference: `é no` -> está no [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `no` -> `no` [article_preposition] The preposition should be 'no' to indicate location.
- Missed: `é no`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `assisto a rádio`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `branca casa`

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

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-not-brazilian-train (erroneous, a2)

- Task: How did you travel to the city?
- Answer: Eu viajei de comboio até a cidade.
- Reference: `comboio` -> trem [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `até a` -> `para` [article_preposition] The preposition 'até' is not needed here.
- Missed: `comboio`

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `uma pizza` -> `uma pizza` [word_choice] The word 'uma' is unnecessary.
- Missed: `perguntei`

### h-b1-verb-form-future-subjunctive-querer (erroneous, b1)

- Task: Will you help me?
- Answer: Se você querer, eu ajudo você a mudar.
- Reference: `Se você querer` -> Se você quiser [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `mudar` -> `mudar` [verb_form] The verb should be conjugated to agree with the subject 'eu'.
- Missed: `Se você querer`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a escola`

### h-b2-spelling-vários (erroneous, b2)

- Task: Describe the situation.
- Answer: A situação é muito delicada, porque o resultado depende de varios fatores.
- Reference: `varios` -> vários [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `varios`

### h-a1-off-topic-music (off_topic, a1)

- Task: What is your name?
- Answer: Eu gosto de música brasileira e toco violão.
- Flags: written_in_portuguese=true answers_the_question=true

### h-b1-off-topic-rain (off_topic, b1)

- Task: Do you think cities need more bicycle lanes?
- Answer: Ontem choveu muito e eu fiquei em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `em casa` -> `em casa` [article_preposition] The preposition 'em' is incorrect here.

## gemma3n:e2b / plain

### h-a1-control-weather-cold (control, a1)

- Task: Describe the weather today.
- Answer: Hoje está frio e o céu está nublado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está` -> `é` [ser_estar] The verb 'estar' is used to describe temporary conditions like weather, while 'ser' is used for permanent characteristics.

### h-a1-control-mother-job (control, a1)

- Task: What does your mother do?
- Answer: Minha mãe é professora e trabalha numa escola.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `numa` -> `na` [article_preposition] The preposition 'em' is needed here to indicate location within a school.

### h-a1-control-colloquial-ta (control, a1)

- Task: Are you ready?
- Answer: Tá, só um minuto, já vou.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `já vou` -> `Vou` [verb_form] The verb 'ir' needs to be conjugated to the present tense to indicate a future action.

### h-a1-control-market-purchases (control, a1)

- Task: What did you buy at the market?
- Answer: Comprei bananas, pão e queijo no mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `no` -> `no` [article_preposition] The preposition 'no' is unnecessary and incorrect in this context.

### h-a1-control-bedroom (control, a1)

- Task: Describe your bedroom.
- Answer: O meu quarto é pequeno. Tem uma cama, uma mesa e uma janela grande.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `grande` -> `alta` [word_choice] The adjective 'grande' doesn't fit well with 'janela' (window) in this context. 'Grande' implies size, but 'grande' is more commonly used for things like 'a casa' (house) or 'a cidade' (city).

### h-a2-control-yesterday-park (control, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu fui ao parque com minha irmã e nós jogamos bola.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `nós` -> `nós` [agreement] The pronoun 'nós' is used for the second person plural (you all). The correct pronoun to use with 'e' (and) when referring to a group of people is 'nós'.

### h-a2-control-childhood-house (control, a2)

- Task: Where did you live as a child?
- Answer: Quando eu era criança, morávamos numa casa perto do rio.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `morávamos` -> `morávamos` [verb_form] The verb 'morar' (to live) needs to be conjugated in the first person plural (nós) when talking about a habitual action in the past. The correct form is 'morávamos'.

### h-a2-control-progressive-study (control, a2)

- Task: What is your sister doing now?
- Answer: Ela está estudando para a prova de matemática.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `estudando` -> `está estudando` [verb_form] The verb 'estudar' needs to agree with the subject 'ela' in number. The correct form is 'está estudando'.

### h-a2-control-comparison-cities (control, a2)

- Task: Compare the two cities.
- Answer: Recife é mais quente do que Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `mais quente` -> `quente` [word_choice] The word 'mais' is redundant when used with 'quente'.

### h-a2-control-future-tomorrow (control, a2)

- Task: What will you do tomorrow?
- Answer: Amanhã vou acordar cedo e depois vou encontrar meus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `cedo` -> `antes` [word_choice] The word 'cedo' means 'early', but it's not the most natural way to say 'wake up early' in Portuguese.

### h-a2-control-object-pronoun-call (control, a2)

- Task: Did you call your mother?
- Answer: Sim, liguei para ela hoje de manhã.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `liguei` -> `liguei` [verb_form] The verb 'ligar' (to call) needs to agree with the subject 'eu' (I) in number and person. The correct form is 'liguei'.

### h-a2-control-health-symptoms (control, a2)

- Task: How do you feel?
- Answer: Estou com dor de cabeça e um pouco de febre.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `dor de cabeça` -> `dor de cabeça` [word_choice] While grammatically correct, 'dor de cabeça' is a very common and literal translation of 'headache'. A more natural way to express feeling unwell would be to say 'Estou com dor de cabeça'.

### h-a2-control-directions-pharmacy (control, a2)

- Task: Where is the pharmacy?
- Answer: A farmácia fica duas quadras à frente, à esquerda, ao lado da padaria.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `duas quadras` -> `duas quadras` [word_choice] The phrase 'duas quadras' is not the standard way to say 'two blocks' in Portuguese.
- Found (false positive): `à frente` -> `perto` [word_choice] While grammatically correct, 'à frente' is a bit formal and less common than 'perto' or 'em frente' in this context.
- Found (false positive): `ao lado` -> `próximo a` [article_preposition] The preposition 'ao lado' is correct, but 'próximo a' is more common and natural when describing a location.

### h-a2-control-a-gente-sunday (control, a2)

- Task: How do you and your friends spend Sundays?
- Answer: Aos domingos a gente almoça junto e depois assiste a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a gente` -> `nós` [pronoun] While 'a gente' is common, it's often better to use 'nós' (we) for clarity in this context.

### h-b1-control-opinion-bike-lanes (control, b1)

- Task: Do you think cities should have more bicycle lanes?
- Answer: Acho que sim, porque as ciclovias deixam o trânsito mais seguro e as pessoas gastam menos com carro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `mais seguro` -> `mais seguro` [agreement] The adjective 'seguro' needs to agree in number with the noun 'trânsito'.

### h-b1-control-subjunctive-wish (control, b1)

- Task: What do you wish for your friend?
- Answer: Desejo que ela seja feliz e que encontre um bom emprego.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `seja` -> `esteja` [ser_estar] The verb 'ser' (to be) is used here, but 'ser' is not the correct verb to express a wish for happiness. 'Estar' is more appropriate.

### h-b1-control-future-subjunctive-salvador (control, b1)

- Task: What will you do when you arrive in Salvador?
- Answer: Quando eu chegar a Salvador, vou ligar para a minha tia.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a` -> `` [article_preposition] The preposition 'a' is unnecessary after 'chegar' in this context.

### h-b1-control-imperfect-subjunctive-month (control, b1)

- Task: What would you do with a free month?
- Answer: Se eu tivesse um mês livre, viajaria pelo interior do Brasil.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tivesse` -> `tivesse` [verb_form] The verb 'ter' (to have) needs to be conjugated with the subjunctive mood in this sentence.

### h-b2-control-reported-doctor (control, b2)

- Task: Report what the doctor said.
- Answer: O médico disse que eu precisava descansar e beber mais água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `eu` -> `precisava descansar e beber mais água` [pronoun] The pronoun 'eu' is unnecessary and redundant in this sentence.

### h-b2-control-formal-thanks (control, b2)

- Task: Write a short formal thank-you to a colleague.
- Answer: Prezada Maria, agradeço sinceramente pela sua ajuda no projeto. Cordialmente, João.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Prezada` -> `Prezada` [word_choice] The form 'Prezada' is not the most common or natural way to start a formal thank you in Portuguese.
- Found (changes nothing): `sinceramente` -> `sinceramente` [word_choice] While correct, 'sinceramente' is a bit redundant in a thank you. 'Agradeço sinceramente' is a bit long.

### h-a1-spelling-sao-paulo (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Brasil e moro em Sao Paulo.
- Reference: `Sao` -> São [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `moro` -> `moro` [verb_form] The verb 'moro' is the third person singular present indicative of 'morar' (to live). The correct form for 'I' is 'moro'.
- Missed: `Sao`

### h-a1-article-contraction-hospital (erroneous, a1)

- Task: Where does your father work?
- Answer: Meu pai trabalha em o hospital da cidade.
- Reference: `em o` -> no [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `o` -> `o` [article_preposition] The word 'o' is unnecessary and incorrect in this context.
- Missed: `em o`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `bem` -> `feliz` [word_choice] The word 'bem' is too general; a more specific adjective is needed to describe the feeling.
- Missed: `Eu sinto`

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `a rádio` -> `a rádio` [article_preposition] The word 'a' is unnecessary when referring to a radio as a general object.
- Missed: `assisto a rádio`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective 'branca' (white) doesn't agree with the noun 'casa' (house) in number. It should be 'branca' (white) or 'casas' (houses) depending on the number.
- Missed: `branca casa`

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para` -> `a` [article_preposition] The preposition 'para' is redundant in this context; 'a trabalho' is more natural.
- Missed: `em bicicleta`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `o` -> `o` [article_preposition] The article 'o' is unnecessary before 'guarda-chuva' in this context.
- Missed: `de o`

### h-a2-not-brazilian-train (erroneous, a2)

- Task: How did you travel to the city?
- Answer: Eu viajei de comboio até a cidade.
- Reference: `comboio` -> trem [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a` -> `de` [article_preposition] The preposition 'a' is unnecessary after 'de comboio'.
- Missed: `comboio`

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `perguntei` -> `perguntei` [verb_form] The verb 'perguntar' (to ask) needs to be conjugated to the past tense, which is 'perguntei'.
- Missed: `perguntei`

### h-b1-verb-form-future-subjunctive-querer (erroneous, b1)

- Task: Will you help me?
- Answer: Se você querer, eu ajudo você a mudar.
- Reference: `Se você querer` -> Se você quiser [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `querer` -> `gostar` [word_choice] The verb 'querer' (to want) is not the most natural way to express willingness to help.
- Found (changes nothing): `a mudar` -> `a mudar` [word_choice] The phrase 'a mudar' is not the correct way to express 'to change'. The verb 'ajudar' needs a gerund to describe the action.

### h-b2-verb-form-pluperfect-agreement (erroneous, b2)

- Task: What had happened before the meeting started?
- Answer: Antes da reunião começar, os diretores já tinha decidido o orçamento.
- Reference: `já tinha` -> já tinham [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tinha` -> `tinha` [verb_form] The correct form of the verb 'ter' (to have) in the past is 'tinha', but it should be 'tinha' to agree with the subject 'os diretores' (the directors).
- Missed: `já tinha`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `cheguei` -> `cheguei` [verb_form] The verb 'chegar' (to arrive) is in the past tense. The correct form for 'at eight' is the present tense.
- Missed: `a escola`

### h-a1-off-topic-music (off_topic, a1)

- Task: What is your name?
- Answer: Eu gosto de música brasileira e toco violão.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `to` -> `tô` [verb_form] The verb 'to' should be 'tô' in spoken Brazilian Portuguese.

### h-b1-off-topic-rain (off_topic, b1)

- Task: Do you think cities need more bicycle lanes?
- Answer: Ontem choveu muito e eu fiquei em casa.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Ontem choveu muito` -> `Eu fiquei em casa porque choveu muito ontem.` [content_mismatch] The sentence is grammatically correct, but it doesn't directly answer the question about bicycle lanes.

## gemma3n:e4b / few_shot

### h-a1-spelling-sao-paulo (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Brasil e moro em Sao Paulo.
- Reference: `Sao` -> São [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Sao`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `assisto a rádio`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `numa` -> `em` [article_preposition] The preposition 'em' is more appropriate here.
- Missed: `branca casa`

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

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-not-brazilian-train (erroneous, a2)

- Task: How did you travel to the city?
- Answer: Eu viajei de comboio até a cidade.
- Reference: `comboio` -> trem [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `até a` -> `à` [article_preposition] The preposition 'a' is not needed here.
- Missed: `comboio`

### h-a2-content-car (erroneous, a2)

- Task: Describe the car.
- Answer: O carro é grande e vermelho.
- Reference: `grande e vermelho` -> pequeno e azul [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `grande` -> `pequeno` [word_choice] The car is small, so the adjective should be 'pequeno'.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `uma pizza` -> `o pizza` [article_preposition] The word 'pizza' is a masculine noun, so it needs an article 'o'.
- Missed: `perguntei`

### h-b1-verb-form-future-subjunctive-querer (erroneous, b1)

- Task: Will you help me?
- Answer: Se você querer, eu ajudo você a mudar.
- Reference: `Se você querer` -> Se você quiser [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `a mudar` -> `a mudar` [word_choice] The verb 'ajudar' requires the preposition 'a' followed by an infinitive, but the infinitive should be 'mudar', not 'mudar a'.
- Missed: `Se você querer`

### h-b2-spelling-ótima (erroneous, b2)

- Task: Describe your idea.
- Answer: Tive uma ideia otima para o projeto.
- Reference: `otima` -> ótima [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `otima`

## gemma3n:e4b / few_shot_six

### h-b2-control-formal-thanks (control, b2)

- Task: Write a short formal thank-you to a colleague.
- Answer: Prezada Maria, agradeço sinceramente pela sua ajuda no projeto. Cordialmente, João.
- Flags: written_in_portuguese=false answers_the_question=true

### h-a1-spelling-sao-paulo (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Brasil e moro em Sao Paulo.
- Reference: `Sao` -> São [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Sao`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `assisto a rádio`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `numa` -> `em uma` [article_preposition] The preposition 'em' should be used with 'casa'.
- Missed: `branca casa`

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

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-not-brazilian-train (erroneous, a2)

- Task: How did you travel to the city?
- Answer: Eu viajei de comboio até a cidade.
- Reference: `comboio` -> trem [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `até a` -> `até` [article_preposition] The preposition 'a' is unnecessary.
- Missed: `comboio`

### h-b1-verb-form-future-subjunctive-querer (erroneous, b1)

- Task: Will you help me?
- Answer: Se você querer, eu ajudo você a mudar.
- Reference: `Se você querer` -> Se você quiser [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `você a` -> `você` [article_preposition] The preposition 'a' is unnecessary here.
- Missed: `Se você querer`

### h-b2-verb-form-pluperfect-agreement (erroneous, b2)

- Task: What had happened before the meeting started?
- Answer: Antes da reunião começar, os diretores já tinha decidido o orçamento.
- Reference: `já tinha` -> já tinham [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tinha decidido` -> `tinha decidido` [verb_form] The verb 'ter' is used with the infinitive form of the verb.
- Missed: `já tinha`

## gemma3n:e4b / plain

### h-a1-control-weather-cold (control, a1)

- Task: Describe the weather today.
- Answer: Hoje está frio e o céu está nublado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está nublado` -> `o céu está nublado` [word_order] The verb 'estar' is correct here, but 'nublado' is an adjective and should be placed after the verb.

### h-a1-control-mother-job (control, a1)

- Task: What does your mother do?
- Answer: Minha mãe é professora e trabalha numa escola.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `numa` -> `na` [article_preposition] The preposition 'em' is required before 'escola' when referring to a specific school.

### h-a1-control-brother-age (control, a1)

- Task: How old is your brother?
- Answer: Meu irmão tem doze anos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tem doze anos` -> `é doze anos` [verb_form] The verb 'ter' is used to express age in Portuguese, but it's more common to use 'ser' and state the age directly.

### h-a1-control-hobbies (control, a1)

- Task: What do you like to do?
- Answer: Eu gosto de ler livros e ouvir música.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de ler livros` -> `ler livros` [article_preposition] The preposition 'de' is not needed here; 'de' is used to indicate origin or material.

### h-a1-control-colloquial-ta (control, a1)

- Task: Are you ready?
- Answer: Tá, só um minuto, já vou.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `já vou` -> `I'm ready. I'll be right there.` [word_choice] The phrase 'já vou' is grammatically correct, but it's a bit informal and could be improved for a more formal context.

### h-a1-control-market-purchases (control, a1)

- Task: What did you buy at the market?
- Answer: Comprei bananas, pão e queijo no mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `no mercado` -> `` [article_preposition] The preposition 'no' is redundant when used with 'mercado'.

### h-a1-control-bedroom (control, a1)

- Task: Describe your bedroom.
- Answer: O meu quarto é pequeno. Tem uma cama, uma mesa e uma janela grande.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `uma janela grande` -> `uma janela grande` [word_order] The adjective 'grande' should come after the noun it modifies in Brazilian Portuguese.

### h-a2-control-yesterday-park (control, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu fui ao parque com minha irmã e nós jogamos bola.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `nós jogamos` -> `joguei` [verb_form] The verb 'jogar' should be conjugated in the past tense, and 'nós' is not necessary when the verb conjugation already indicates the subject.

### h-a2-control-childhood-house (control, a2)

- Task: Where did you live as a child?
- Answer: Quando eu era criança, morávamos numa casa perto do rio.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morávamos` -> `morei` [verb_form] The verb 'morar' in the past continuous tense should be conjugated in the imperfect form, but the correct form is 'morei' or 'morava' depending on the desired nuance. 'Morávamos' implies a habitual action in the past, which is not the intended meaning here.

### h-a2-control-progressive-study (control, a2)

- Task: What is your sister doing now?
- Answer: Ela está estudando para a prova de matemática.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `para a prova` -> `para a prova` [article_preposition] The preposition 'para' is not correct here; 'para' indicates destination, not purpose.

### h-a2-control-comparison-cities (control, a2)

- Task: Compare the two cities.
- Answer: Recife é mais quente do que Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `é mais quente` -> `está mais quente` [ser_estar] The verb 'ser' is not needed here; 'estar' is required to express temperature.

### h-a2-control-future-tomorrow (control, a2)

- Task: What will you do tomorrow?
- Answer: Amanhã vou acordar cedo e depois vou encontrar meus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `meus amigos` -> `meus amigos` [agreement] The possessive pronoun should agree in number with the noun it modifies.

### h-a2-control-object-pronoun-call (control, a2)

- Task: Did you call your mother?
- Answer: Sim, liguei para ela hoje de manhã.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para ela` -> `ela` [article_preposition] The preposition 'para' is not needed after 'liguei' when indicating the person you called.

### h-a2-control-health-symptoms (control, a2)

- Task: How do you feel?
- Answer: Estou com dor de cabeça e um pouco de febre.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `de cabeça` -> `de cabeça` [article_preposition] The preposition 'de' is not needed here; 'Estou com dor de cabeça' is the correct expression.

### h-a2-control-directions-pharmacy (control, a2)

- Task: Where is the pharmacy?
- Answer: A farmácia fica duas quadras à frente, à esquerda, ao lado da padaria.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `duas quadras à frente` -> `duas quadras à frente` [word_order] The phrase 'duas quadras à frente' is not a natural way to express 'two blocks ahead' in Brazilian Portuguese.

### h-a2-control-a-gente-sunday (control, a2)

- Task: How do you and your friends spend Sundays?
- Answer: Aos domingos a gente almoça junto e depois assiste a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `junto` -> `` [word_choice] While not strictly incorrect, 'junto' is redundant here; 'juntos' is more natural.

### h-b1-control-opinion-bike-lanes (control, b1)

- Task: Do you think cities should have more bicycle lanes?
- Answer: Acho que sim, porque as ciclovias deixam o trânsito mais seguro e as pessoas gastam menos com carro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `deixam o trânsito mais seguro` -> `tornam o trânsito mais seguro` [verb_form] The verb 'deixar' is not appropriate here; 'tornam' or 'fazem' are better choices to express making the traffic safer.

### h-b1-control-subjunctive-wish (control, b1)

- Task: What do you wish for your friend?
- Answer: Desejo que ela seja feliz e que encontre um bom emprego.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `seja feliz` -> `esteja feliz` [ser_estar] The verb 'ser' is not the most natural choice here; 'estar' is more appropriate to express a state of being.

### h-b1-control-future-subjunctive-salvador (control, b1)

- Task: What will you do when you arrive in Salvador?
- Answer: Quando eu chegar a Salvador, vou ligar para a minha tia.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ligar para a minha tia` -> `encontrar a minha tia` [word_choice] The verb 'ligar' means to call on the phone, but the question asks about actions upon arrival, not phone calls.

### h-b1-control-imperfect-subjunctive-month (control, b1)

- Task: What would you do with a free month?
- Answer: Se eu tivesse um mês livre, viajaria pelo interior do Brasil.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `pelo interior do Brasil` -> `pelo interior do Brasil` [article_preposition] The preposition 'por' is not appropriate when indicating travel *through* a region.

### h-b2-control-reported-doctor (control, b2)

- Task: Report what the doctor said.
- Answer: O médico disse que eu precisava descansar e beber mais água.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `precisava` -> `preciso` [verb_form] The use of the conditional 'precisava' implies a past action that was not completed, which is not the intended meaning. The doctor is giving a present recommendation.

### h-b2-control-formal-thanks (control, b2)

- Task: Write a short formal thank-you to a colleague.
- Answer: Prezada Maria, agradeço sinceramente pela sua ajuda no projeto. Cordialmente, João.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `agradeço sinceramente` -> `agradeci sinceramente` [verb_form] The verb 'agradecer' should be conjugated in the past tense to match the context of a thank you.

### h-a1-spelling-sao-paulo (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Brasil e moro em Sao Paulo.
- Reference: `Sao` -> São [spelling]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `Sao Paulo` -> `São Paulo` [spelling] The name of the city should not be capitalized.

### h-a2-verb-form-ir-summer (erroneous, a2)

- Task: Where did you go last summer?
- Answer: No verão passado eu foi para a praia com a família.
- Reference: `eu foi` -> eu fui [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `foi` -> `fui` [verb_form] The verb 'ir' requires the pronoun 'eu' to be conjugated as 'fui' in the past tense.
- Found (false positive): `para a praia` -> `à` [article_preposition] The preposition 'para' is not needed here; 'à' is more appropriate.

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para o trabalho` -> `ao trabalho` [article_preposition] The preposition 'para' is not needed here; 'a' is more appropriate.
- Missed: `em bicicleta`

### h-a2-not-brazilian-train (erroneous, a2)

- Task: How did you travel to the city?
- Answer: Eu viajei de comboio até a cidade.
- Reference: `comboio` -> trem [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `até a cidade` -> `à cidade` [article_preposition] The preposition 'a' is not needed before a city name.
- Missed: `comboio`

### h-b1-verb-form-future-subjunctive-querer (erroneous, b1)

- Task: Will you help me?
- Answer: Se você querer, eu ajudo você a mudar.
- Reference: `Se você querer` -> Se você quiser [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ajudo você a` -> `ajudar` [article_preposition] The verb 'ajudar' is followed by the indirect object pronoun directly, without the preposition 'a'.
- Missed: `Se você querer`

### h-b2-verb-form-pluperfect-agreement (erroneous, b2)

- Task: What had happened before the meeting started?
- Answer: Antes da reunião começar, os diretores já tinha decidido o orçamento.
- Reference: `já tinha` -> já tinham [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `já tinha decidido` -> `já tido decidido` [verb_form] The verb 'ter' is used as an auxiliary verb, so the past participle 'decidido' should come after 'tinha'.

### h-a1-off-topic-music (off_topic, a1)

- Task: What is your name?
- Answer: Eu gosto de música brasileira e toco violão.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `toco` -> `toco` [verb_form] The verb 'tocar' (to play) needs to agree with the subject 'eu' (I), so the correct form is 'toco'.

### h-b1-off-topic-rain (off_topic, b1)

- Task: Do you think cities need more bicycle lanes?
- Answer: Ontem choveu muito e eu fiquei em casa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `fiquei em casa` -> `Sim, eu acho que as cidades precisam de mais ciclovias.` [content_mismatch] The answer does not address the question about bicycle lanes.

## gemma4:12b-it-qat / few_shot

### h-a2-control-childhood-house (control, a2)

- Task: Where did you live as a child?
- Answer: Quando eu era criança, morávamos numa casa perto do rio.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morávamos` -> `morava` [verb_form] The verb should be in the first person singular to agree with 'eu'.

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `a rádio` -> `a rádio` [article_preposition] In Brazilian Portuguese, the verb 'assistir' (to watch/listen to) is usually used without a preposition when referring to media.
- Missed: `assisto a rádio`

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`

## gemma4:12b-it-qat / few_shot_six

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

## gemma4:12b-it-qat / plain

### h-a2-control-childhood-house (control, a2)

- Task: Where did you live as a child?
- Answer: Quando eu era criança, morávamos numa casa perto do rio.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morávamos` -> `morava` [verb_form] The verb should be in the first person singular to match the subject 'eu'.

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `a rádio` -> `a rádio` [article_preposition] In Brazilian Portuguese, the verb 'assistir' (to watch/listen to) is usually used with a direct object without a preposition in this context, or 'ouvir' is preferred for radio.
- Missed: `assisto a rádio`

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
- Found (matched): `Eu machuquei` -> `Eu me machuquei` [verb_form] The verb 'machucar' is usually reflexive when referring to an injury one sustained to themselves.
- Found (changes nothing): `ontem` -> `ontem` [word_order] In Portuguese, time expressions like 'ontem' usually come at the beginning or the end of the sentence, but the verb requires a direct object or reflexive pronoun before the adverb.

### h-b1-off-topic-rain (off_topic, b1)

- Task: Do you think cities need more bicycle lanes?
- Answer: Ontem choveu muito e eu fiquei em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Ontem choveu muito e eu fiquei em casa` -> `` [content_mismatch] The answer does not address the question about bicycle lanes.

## gemma4:12b / few_shot

### h-a2-control-childhood-house (control, a2)

- Task: Where did you live as a child?
- Answer: Quando eu era criança, morávamos numa casa perto do rio.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `morávamos` -> `morávamos` [verb_form] The verb should be in the imperfect tense to describe a continuous state in the past.

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`

## gemma4:12b / few_shot_six

### h-a2-control-childhood-house (control, a2)

- Task: Where did you live as a child?
- Answer: Quando eu era criança, morávamos numa casa perto do rio.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morávamos` -> `morava` [verb_form] The verb should be in the first person singular to agree with 'eu': morava.

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

## gemma4:12b / plain

### h-a2-control-childhood-house (control, a2)

- Task: Where did you live as a child?
- Answer: Quando eu era criança, morávamos numa casa perto do rio.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morávamos` -> `morava` [verb_form] The verb should be in the first-person singular to agree with the subject 'eu'.

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`

### h-b2-spelling-ótima (erroneous, b2)

- Task: Describe your idea.
- Answer: Tive uma ideia otima para o projeto.
- Reference: `otima` -> ótima [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tive` -> `Tenho` [verb_form] The verb 'ter' should be conjugated as 'Tenho' for the present tense.
- Found (matched): `otima` -> `ótima` [spelling] The word 'ótima' requires an acute accent.

### h-a1-off-topic-music (off_topic, a1)

- Task: What is your name?
- Answer: Eu gosto de música brasileira e toco violão.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de música brasileira e toco violão.` -> `Meu nome é [Name].` [content_mismatch] The answer does not address the question about your name.

## gemma4:e2b-it-qat / few_shot

### h-a2-control-a-gente-sunday (control, a2)

- Task: How do you and your friends spend Sundays?
- Answer: Aos domingos a gente almoça junto e depois assiste a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `a gente` -> `a gente` [word_order] The spoken Brazilian form is 'a gente', but in written form, it is usually written as 'a gente' or 'nós'. However, the prompt asks for real mistakes, and 'a gente' is acceptable in spoken context. No clear grammatical error is present here based on the rules.

### h-a1-spelling-sao-paulo (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Brasil e moro em Sao Paulo.
- Reference: `Sao` -> São [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Sao`

### h-a1-serestar-father-location (erroneous, a1)

- Task: Where is your father?
- Answer: Meu pai é no trabalho agora.
- Reference: `é no` -> está no [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `é no`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

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

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `o` -> `de` [article_preposition] The preposition 'de' is required here, not the article 'o'.

### h-a2-pronoun-reflexive-hurt (erroneous, a2)

- Task: What happened to you?
- Answer: Eu machuquei ontem jogando futebol.
- Reference: `Eu machuquei` -> Eu me machuquei [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu machuquei`

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-not-brazilian-train (erroneous, a2)

- Task: How did you travel to the city?
- Answer: Eu viajei de comboio até a cidade.
- Reference: `comboio` -> trem [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `comboio`

### h-a2-content-car (erroneous, a2)

- Task: Describe the car.
- Answer: O carro é grande e vermelho.
- Reference: `grande e vermelho` -> pequeno e azul [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `grande e vermelho` -> `pequeno e azul` [content_mismatch] The description should be small and blue, not big and red.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No restaurante` -> `No` [article_preposition] The correct preposition to indicate location is 'No'.
- Missed: `perguntei`

### h-b1-verb-form-subjunctive-advice (erroneous, b1)

- Task: What do you advise your friend?
- Answer: Aconselho que você fala com o seu chefe antes de decidir.
- Reference: `fala` -> fale [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fala`

### h-b1-verb-form-future-subjunctive-querer (erroneous, b1)

- Task: Will you help me?
- Answer: Se você querer, eu ajudo você a mudar.
- Reference: `Se você querer` -> Se você quiser [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a mudar` -> `mudar` [word_order] The verb 'ajudar' requires the infinitive without the preposition 'a' in this context.
- Missed: `Se você querer`

### h-b2-verb-form-pluperfect-agreement (erroneous, b2)

- Task: What had happened before the meeting started?
- Answer: Antes da reunião começar, os diretores já tinha decidido o orçamento.
- Reference: `já tinha` -> já tinham [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tinha` -> `tinha` [verb_form] The correct past perfect tense form is 'tinha' (imperfect), but in this context, the simple past 'tinha decidido' or 'tiveram decidido' might be more appropriate depending on the nuance, however, 'tinha' is grammatically acceptable here. A more natural phrasing might be 'tiveram decidido' if referring to a completed action before another past action, but 'tinha' is not strictly an error. Let's check for a clearer error. The structure is correct for the past perfect. No clear error found based on the prompt's constraints.
- Missed: `já tinha`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a escola`

### h-b2-spelling-vários (erroneous, b2)

- Task: Describe the situation.
- Answer: A situação é muito delicada, porque o resultado depende de varios fatores.
- Reference: `varios` -> vários [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `varios`

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=true answers_the_question=false

### h-a2-english-market (not_portuguese, a2)

- Task: Describe your weekend.
- Answer: On Saturday I went to the market.
- Flags: written_in_portuguese=true answers_the_question=false

## gemma4:e2b-it-qat / few_shot_six

### h-a2-control-childhood-house (control, a2)

- Task: Where did you live as a child?
- Answer: Quando eu era criança, morávamos numa casa perto do rio.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morávamos` -> `morava` [verb_form] The subject is 'eu', so the verb should be 'morava'.

### h-a2-control-directions-pharmacy (control, a2)

- Task: Where is the pharmacy?
- Answer: A farmácia fica duas quadras à frente, à esquerda, ao lado da padaria.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `quadras` -> `quadras` [word_choice] The correct term for blocks is 'quadras'.

### h-a2-control-a-gente-sunday (control, a2)

- Task: How do you and your friends spend Sundays?
- Answer: Aos domingos a gente almoça junto e depois assiste a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `a gente` -> `a gente` [word_order] The spoken Brazilian form is 'a gente', but in written form, it is usually written as 'a gente' or 'nós'. However, the prompt asks for real mistakes, and 'a gente' is acceptable in spoken context, but let's check for more concrete errors. The sentence structure is fine for A2.

### h-a1-spelling-sao-paulo (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Brasil e moro em Sao Paulo.
- Reference: `Sao` -> São [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Sao`

### h-a1-serestar-father-location (erroneous, a1)

- Task: Where is your father?
- Answer: Meu pai é no trabalho agora.
- Reference: `é no` -> está no [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `é no`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `assisto a rádio`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `uma branca` -> `uma branca` [agreement] The adjective must agree with the feminine noun 'casa': uma branca.
- Missed: `branca casa`

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

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-not-brazilian-train (erroneous, a2)

- Task: How did you travel to the city?
- Answer: Eu viajei de comboio até a cidade.
- Reference: `comboio` -> trem [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `comboio`

### h-a2-content-car (erroneous, a2)

- Task: Describe the car.
- Answer: O carro é grande e vermelho.
- Reference: `grande e vermelho` -> pequeno e azul [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `grande e vermelho` -> `pequeno e azul` [content_mismatch] The description should be small and blue, not big and red.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No restaurante` -> `No` [article_preposition] The correct preposition is 'No'.
- Missed: `perguntei`

### h-b1-verb-form-subjunctive-advice (erroneous, b1)

- Task: What do you advise your friend?
- Answer: Aconselho que você fala com o seu chefe antes de decidir.
- Reference: `fala` -> fale [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fala`

### h-b1-verb-form-future-subjunctive-querer (erroneous, b1)

- Task: Will you help me?
- Answer: Se você querer, eu ajudo você a mudar.
- Reference: `Se você querer` -> Se você quiser [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `a mudar` -> `a mudar` [word_order] The verb 'ajudar' is followed by the infinitive without a preposition.
- Missed: `Se você querer`

### h-b2-verb-form-pluperfect-agreement (erroneous, b2)

- Task: What had happened before the meeting started?
- Answer: Antes da reunião começar, os diretores já tinha decidido o orçamento.
- Reference: `já tinha` -> já tinham [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tinha` -> `tinha` [verb_form] The past perfect tense requires the auxiliary verb 'tinha' (had), which is correct here, but the structure is slightly awkward. A more natural phrasing would be 'tinha decidido'. However, sticking strictly to grammatical errors, this is acceptable in context. No clear grammatical error found based on the prompt's strict rules.
- Missed: `já tinha`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a escola`

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=true answers_the_question=false

### h-a2-english-market (not_portuguese, a2)

- Task: Describe your weekend.
- Answer: On Saturday I went to the market.
- Flags: written_in_portuguese=true answers_the_question=false

## gemma4:e2b-it-qat / plain

### h-a1-control-weather-cold (control, a1)

- Task: Describe the weather today.
- Answer: Hoje está frio e o céu está nublado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `está` -> `está` [ser_estar] The verb 'estar' is used correctly here, but 'está' is the correct form for the subject 'o céu' (the sky).

### h-a1-control-mother-job (control, a1)

- Task: What does your mother do?
- Answer: Minha mãe é professora e trabalha numa escola.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Minha mãe é professora e trabalha numa escola.` -> `Minha mãe é professora e trabalha numa escola.` [spelling] The sentence is grammatically correct and makes sense in Portuguese.

### h-a1-control-brother-age (control, a1)

- Task: How old is your brother?
- Answer: Meu irmão tem doze anos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tem` -> `é` [word_choice] The verb 'ter' is used to indicate possession, not age.

### h-a1-control-colloquial-ta (control, a1)

- Task: Are you ready?
- Answer: Tá, só um minuto, já vou.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tá` -> `Estou` [word_choice] The correct informal contraction for 'está' is 'Tá' but in this context, a more standard and slightly less colloquial form is better for A1.

### h-a1-control-market-purchases (control, a1)

- Task: What did you buy at the market?
- Answer: Comprei bananas, pão e queijo no mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `pão` -> `pão` [spelling] The correct spelling for bread in Portuguese is 'pão'.

### h-a1-control-bedroom (control, a1)

- Task: Describe your bedroom.
- Answer: O meu quarto é pequeno. Tem uma cama, uma mesa e uma janela grande.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Tem` -> `Tem` [verb_form] The verb 'ter' should be conjugated as 'Tem' (singular subject 'quarto').

### h-a2-control-yesterday-park (control, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu fui ao parque com minha irmã e nós jogamos bola.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `jogamos` [pronoun] The subject 'nós' is redundant when the verb 'jogamos' is used.

### h-a2-control-childhood-house (control, a2)

- Task: Where did you live as a child?
- Answer: Quando eu era criança, morávamos numa casa perto do rio.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morávamos` -> `morava` [verb_form] The subject 'eu' requires the first-person singular verb 'morava'.

### h-a2-control-progressive-study (control, a2)

- Task: What is your sister doing now?
- Answer: Ela está estudando para a prova de matemática.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Ela está estudando` -> `Ela está estudando` [not_brazilian] The verb 'estar' is used correctly here, but the context implies a continuous action, which is fine, however, the sentence structure is correct.

### h-a2-control-comparison-cities (control, a2)

- Task: Compare the two cities.
- Answer: Recife é mais quente do que Curitiba.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `mais` -> `é` [word_choice] The comparative structure 'mais' is used for adding more, not for comparison.

### h-a2-control-future-tomorrow (control, a2)

- Task: What will you do tomorrow?
- Answer: Amanhã vou acordar cedo e depois vou encontrar meus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `vou` -> `Vou` [word_choice] The verb 'vou' is used correctly, but in this context, a simple future tense is more natural for a plan.

### h-a2-control-object-pronoun-call (control, a2)

- Task: Did you call your mother?
- Answer: Sim, liguei para ela hoje de manhã.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `liguei` -> `liguei` [verb_form] The correct past tense of 'ligar' (to call) for 'eu' is 'liguei' (simple past), but in this context, the simple past is correct, however, the structure is fine.

### h-a2-control-health-symptoms (control, a2)

- Task: How do you feel?
- Answer: Estou com dor de cabeça e um pouco de febre.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Estou com dor de cabeça e um pouco de febre.` -> `Estou com dor de cabeça e um pouco de febre.` [spelling] The sentence is grammatically correct and natural.

### h-a2-control-directions-pharmacy (control, a2)

- Task: Where is the pharmacy?
- Answer: A farmácia fica duas quadras à frente, à esquerda, ao lado da padaria.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `quadras` -> `blocos` [word_choice] The correct term for blocks in this context is 'quadras' or 'blocos', but 'quadras' is less common than 'blocos' or 'ruas' depending on the region, though 'quadras' is acceptable.

### h-a2-control-a-gente-sunday (control, a2)

- Task: How do you and your friends spend Sundays?
- Answer: Aos domingos a gente almoça junto e depois assiste a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a gente` -> `nós` [word_choice] In this context, 'a gente' is informal and often used in spoken Brazilian Portuguese, but for a general answer, a more formal or standard construction is better.

### h-b1-control-opinion-bike-lanes (control, b1)

- Task: Do you think cities should have more bicycle lanes?
- Answer: Acho que sim, porque as ciclovias deixam o trânsito mais seguro e as pessoas gastam menos com carro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `gastam` -> `gastam menos` [word_choice] The verb 'gastar' is used incorrectly here; 'economizar' or 'gastar menos' is more appropriate in this context.

### h-b1-control-subjunctive-wish (control, b1)

- Task: What do you wish for your friend?
- Answer: Desejo que ela seja feliz e que encontre um bom emprego.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Desejo` -> `Eu desejo` [word_order] The verb 'desejar' is in the first person singular, so the correct form is 'Eu desejo' or simply 'Desejo' (omitting the subject is common but the structure is fine here, however, the structure is slightly awkward for a direct wish).

### h-b1-control-future-subjunctive-salvador (control, b1)

- Task: What will you do when you arrive in Salvador?
- Answer: Quando eu chegar a Salvador, vou ligar para a minha tia.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Quando eu chegar a Salvador` -> `Quando eu chegar em Salvador` [word_order] The structure is slightly awkward; a more natural way to express this future action is to use the simple future tense or a structure indicating a future plan.

### h-b1-control-imperfect-subjunctive-month (control, b1)

- Task: What would you do with a free month?
- Answer: Se eu tivesse um mês livre, viajaria pelo interior do Brasil.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `viajaria` -> `viajo` [verb_form] The future tense is not the most natural choice here; the simple future or present tense is better for hypothetical situations.

### h-b2-control-reported-doctor (control, b2)

- Task: Report what the doctor said.
- Answer: O médico disse que eu precisava descansar e beber mais água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `eu` -> `você` [pronoun] The pronoun should be 'você' or implied, as the doctor is speaking to the patient.

### h-b2-control-formal-thanks (control, b2)

- Task: Write a short formal thank-you to a colleague.
- Answer: Prezada Maria, agradeço sinceramente pela sua ajuda no projeto. Cordialmente, João.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Cordialmente` -> `Atenciosamente` [word_choice] This is an adjective, but it should be an adverb in this context.

### h-a1-spelling-sao-paulo (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Brasil e moro em Sao Paulo.
- Reference: `Sao` -> São [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Eu sou do Brasil` -> `Eu sou do Brasil` [word_choice] The correct way to express origin is 'Eu sou do Brasil' or 'Eu sou brasileiro'.
- Missed: `Sao`

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `À noite` -> `À noite` [word_order] The correct way to say 'at night' is 'À noite' or 'De noite', but 'À noite' is acceptable in this context.
- Missed: `assisto a rádio`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `numa` -> `uma` [article_preposition] The correct article before a feminine noun starting with a consonant is 'uma'.
- Missed: `branca casa`

### h-a2-not-brazilian-train (erroneous, a2)

- Task: How did you travel to the city?
- Answer: Eu viajei de comboio até a cidade.
- Reference: `comboio` -> trem [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `comboio` -> `comboio` [spelling] The correct spelling for train in Portuguese is 'comboio'.
- Missed: `comboio`

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No restaurante` -> `No restaurante` [article_preposition] The correct preposition to use here is 'No' (in the context of location).
- Missed: `perguntei`

### h-b1-verb-form-future-subjunctive-querer (erroneous, b1)

- Task: Will you help me?
- Answer: Se você querer, eu ajudo você a mudar.
- Reference: `Se você querer` -> Se você quiser [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ajudo você` -> `te ajudo` [pronoun] The pronoun 'te' should be used instead of 'você' when the verb is conjugated in the first person singular.
- Missed: `Se você querer`

### h-a1-off-topic-music (off_topic, a1)

- Task: What is your name?
- Answer: Eu gosto de música brasileira e toco violão.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de música brasileira e toco violão.` -> `Meu nome é...` [content_mismatch] The answer does not answer the question 'What is your name?'

## gemma4:e2b / few_shot

### h-a1-control-brother-age (control, a1)

- Task: How old is your brother?
- Answer: Meu irmão tem doze anos.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `doze` -> `doze` [word_choice] The correct number for twelve is 'doze' or 'doze', but the phrasing is slightly unnatural for this level. However, the main issue is the lack of correct number usage in this context. The most direct error is the lack of correct number form for 'twelve'.

### h-a2-control-childhood-house (control, a2)

- Task: Where did you live as a child?
- Answer: Quando eu era criança, morávamos numa casa perto do rio.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morávamos` -> `morava` [agreement] The subject is 'eu' (I), so the verb should be singular.

### h-a2-control-health-symptoms (control, a2)

- Task: How do you feel?
- Answer: Estou com dor de cabeça e um pouco de febre.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `com` -> `estar` [word_choice] The correct structure for expressing having an illness is 'estar com'.

### h-a2-control-directions-pharmacy (control, a2)

- Task: Where is the pharmacy?
- Answer: A farmácia fica duas quadras à frente, à esquerda, ao lado da padaria.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `quadras` -> `quadras` [word_choice] The correct word for blocks is 'quadras'.

### h-a2-control-a-gente-sunday (control, a2)

- Task: How do you and your friends spend Sundays?
- Answer: Aos domingos a gente almoça junto e depois assiste a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a gente` -> `aos domingos` [article_preposition] The correct pronoun for 'we' or 'people' in this context is 'nós' or 'a gente' is acceptable but 'a gente' is often used without the article 'a' when referring to people in general. However, the structure here is slightly awkward. The main issue is the missing article or the structure. A more natural phrasing would be 'nós'. Given the context, 'a gente' is acceptable, but the structure needs correction. The most direct error is the missing article or the phrasing. Let's focus on the common usage. The structure 'a gente almoça' is common but the lack of 'nós' or a clearer subject is a point of concern. However, the most concrete error is the missing article before 'domingos' if we consider the full sentence structure. Let's stick to the most obvious grammatical issue for A2 level. The structure is acceptable, but the lack of a definite article or a clearer subject is a weak point. Let's re-evaluate. The most common error in this context is the missing article or the structure. Let's check the verb agreement. 'almoça' is correct for 'a gente'. The main issue is the missing article before 'domingos'.

### h-a1-spelling-sao-paulo (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Brasil e moro em Sao Paulo.
- Reference: `Sao` -> São [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Sao`

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `Eu tem` -> `Eu tenho` [verb_form] The verb 'ter' must be conjugated correctly for the first person singular.

### h-a1-serestar-father-location (erroneous, a1)

- Task: Where is your father?
- Answer: Meu pai é no trabalho agora.
- Reference: `é no` -> está no [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `é no`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `assisto a rádio`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `numa` -> `na` [article_preposition] The correct preposition for 'live in' with a place is 'em' or 'na'.
- Missed: `branca casa`

### h-a2-agreement-red-shoes (erroneous, a2)

- Task: Describe the shoes.
- Answer: Ela comprou dois sapatos novo e vermelhos.
- Reference: `novo` -> novos [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `novo` -> `novos` [agreement] The adjective must agree in number with 'sapatos' (plural).

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `de o` -> `de` [article_preposition] The structure for forgetting something is 'esqueci de' without the pronoun 'o'.

### h-a2-pronoun-reflexive-hurt (erroneous, a2)

- Task: What happened to you?
- Answer: Eu machuquei ontem jogando futebol.
- Reference: `Eu machuquei` -> Eu me machuquei [pronoun]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `machuquei` -> `me machuquei` [verb_form] The verb 'machucar' is incorrect for 'to get hurt'. 'Machuquei' is the past tense of 'to injure'. The correct verb is 'me machuquei' or 'eu me machuquei'. For this context, a simple past tense is better.

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-not-brazilian-train (erroneous, a2)

- Task: How did you travel to the city?
- Answer: Eu viajei de comboio até a cidade.
- Reference: `comboio` -> trem [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `comboio` -> `comboio` [word_choice] The correct word for train is comboio.
- Missed: `comboio`

### h-a2-content-car (erroneous, a2)

- Task: Describe the car.
- Answer: O carro é grande e vermelho.
- Reference: `grande e vermelho` -> pequeno e azul [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `grande` -> `pequeno` [content_mismatch] The car is described as small, not large.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No restaurante` -> `no restaurante` [article_preposition] The correct way to say 'at the restaurant' is 'no restaurante' or 'no restaurante' depending on context, but 'no restaurante' is more natural here. The main issue is the missing article/preposition.
- Missed: `perguntei`

### h-b1-verb-form-subjunctive-advice (erroneous, b1)

- Task: What do you advise your friend?
- Answer: Aconselho que você fala com o seu chefe antes de decidir.
- Reference: `fala` -> fale [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fala`

### h-b1-verb-form-future-subjunctive-querer (erroneous, b1)

- Task: Will you help me?
- Answer: Se você querer, eu ajudo você a mudar.
- Reference: `Se você querer` -> Se você quiser [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `a mudar` -> `a mudar` [word_choice] The verb form is incorrect for this context. It should be an infinitive or a different construction.
- Missed: `Se você querer`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `cheguei` -> `cheguei` [verb_form] The correct past tense conjugation for 'eu' is 'cheguei' (with an accent).
- Missed: `a escola`

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=false

### h-a2-english-market (not_portuguese, a2)

- Task: Describe your weekend.
- Answer: On Saturday I went to the market.
- Flags: written_in_portuguese=false answers_the_question=false

## gemma4:e2b / few_shot_six

### h-a2-control-yesterday-park (control, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu fui ao parque com minha irmã e nós jogamos bola.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `eu fui` -> `eu fui` [verb_form] The verb 'fui' is correct, but the structure is slightly informal; 'eu fui' is acceptable but 'eu fui' is more common in this context. However, the main issue is the lack of agreement in the past tense for the verb 'jogamos' if it was meant to be 'nós jogamos'. Assuming the intended meaning is correct, no grammatical error is strictly present in the provided answer for this task.

### h-a2-control-childhood-house (control, a2)

- Task: Where did you live as a child?
- Answer: Quando eu era criança, morávamos numa casa perto do rio.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morávamos` -> `morava` [verb_form] The subject is 'eu' (I), so the verb should be singular: morava.

### h-a2-control-health-symptoms (control, a2)

- Task: How do you feel?
- Answer: Estou com dor de cabeça e um pouco de febre.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `com dor` -> `com` [word_choice] The correct structure is 'estar com' or 'estar com' for feeling/symptoms. 'Com' is incorrect here.

### h-a2-control-directions-pharmacy (control, a2)

- Task: Where is the pharmacy?
- Answer: A farmácia fica duas quadras à frente, à esquerda, ao lado da padaria.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `duas quadras` -> `quadras` [agreement] The correct plural form for 'quadra' is 'quadras'.

### h-a2-control-a-gente-sunday (control, a2)

- Task: How do you and your friends spend Sundays?
- Answer: Aos domingos a gente almoça junto e depois assiste a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a gente` -> `nos domingos` [article_preposition] The correct pronoun for 'we' or 'people' in this context is 'nós' or 'a gente' is acceptable but 'a gente' is often used in spoken Brazilian Portuguese, but the structure is slightly awkward. However, the main issue is the missing article or structure. Let's focus on the most obvious grammatical issue for A2. The structure is acceptable but 'a gente' is often used without the article 'a' when referring to a group. The main issue here is the missing article before 'domingos'.

### h-b2-control-formal-thanks (control, b2)

- Task: Write a short formal thank-you to a colleague.
- Answer: Prezada Maria, agradeço sinceramente pela sua ajuda no projeto. Cordialmente, João.
- Flags: written_in_portuguese=true answers_the_question=false

### h-a1-spelling-sao-paulo (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Brasil e moro em Sao Paulo.
- Reference: `Sao` -> São [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `moro` -> `moro` [verb_form] The verb 'morar' is conjugated incorrectly for the first person singular present tense. It should be 'moro'.
- Missed: `Sao`

### h-a1-serestar-father-location (erroneous, a1)

- Task: Where is your father?
- Answer: Meu pai é no trabalho agora.
- Reference: `é no` -> está no [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `é no`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `a rádio` -> `a rádio` [article_preposition] The definite article 'a' is incorrect here; it should be 'a'.
- Missed: `assisto a rádio`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `numa` -> `uma` [article_preposition] The correct contraction for 'numa' is 'numa' (or 'numa' is acceptable, but the structure is slightly awkward). The main issue is the article usage here. It should be 'uma'.
- Missed: `branca casa`

### h-a2-agreement-red-shoes (erroneous, a2)

- Task: Describe the shoes.
- Answer: Ela comprou dois sapatos novo e vermelhos.
- Reference: `novo` -> novos [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `novo` -> `novos` [agreement] The adjective must agree with 'sapatos' (plural).

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `de o` -> `de` [article_preposition] The structure for forgetting something is 'esqueci de' followed by the infinitive or noun. 'de' alone is correct here.

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-not-brazilian-train (erroneous, a2)

- Task: How did you travel to the city?
- Answer: Eu viajei de comboio até a cidade.
- Reference: `comboio` -> trem [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `comboio` -> `comboio` [word_choice] The correct word for train is comboio.
- Missed: `comboio`

### h-a2-content-car (erroneous, a2)

- Task: Describe the car.
- Answer: O carro é grande e vermelho.
- Reference: `grande e vermelho` -> pequeno e azul [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `grande e vermelho` -> `pequeno e azul` [content_mismatch] The description contradicts the facts given about the car.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `perguntei` -> `perguntei` [verb_form] The verb needs to be in the past tense for 'eu': perguntei.
- Missed: `perguntei`

### h-b1-verb-form-subjunctive-advice (erroneous, b1)

- Task: What do you advise your friend?
- Answer: Aconselho que você fala com o seu chefe antes de decidir.
- Reference: `fala` -> fale [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `você` -> `você` [word_choice] The correct form for 'you' in this context is 'tu' or 'você' (depending on region), but 'você' is often considered non-standard in formal advice, and the structure is slightly awkward. However, the main issue is the use of 'você' in this context. A more natural phrasing would use 'você' or 'tu'. Given the context, the main issue is the lack of standard phrasing for advice. The most direct grammatical error is the use of 'você' which is often replaced by 'você' or 'tu'. Let's focus on the common usage. The structure is acceptable but 'você' is less common than 'você' or 'tu'. Let's check for a clearer error. The structure is fine, but the word choice is slightly off for a B1 level. Let's re-evaluate. The structure is grammatically acceptable for A2/B1. No clear grammatical error is present based on the rules. Re-checking for a potential issue: The use of 'você' is common in some regions, but let's assume it's acceptable for now unless a clear rule violation is found. No mistakes found based on the provided rules.
- Missed: `fala`

### h-b1-verb-form-future-subjunctive-querer (erroneous, b1)

- Task: Will you help me?
- Answer: Se você querer, eu ajudo você a mudar.
- Reference: `Se você querer` -> Se você quiser [verb_form]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `a mudar` -> `ajudar você a mudar` [word_choice] The infinitive form is incorrect; it should be 'a mudar' or 'a mudar' depending on context, but here it sounds unnatural. The correct structure is usually 'ajudar você a fazer algo'. However, the main issue is the missing preposition or structure for 'mudar'. Given the context, 'ajudar você a mudar' is grammatically awkward. A more natural phrasing would be needed. Assuming the intended meaning is 'help you move'.
- Missed: `Se você querer`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `cheguei` -> `cheguei` [verb_form] The correct past tense conjugation for 'eu' is 'cheguei'.
- Missed: `a escola`

### h-b2-spelling-ótima (erroneous, b2)

- Task: Describe your idea.
- Answer: Tive uma ideia otima para o projeto.
- Reference: `otima` -> ótima [spelling]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `otima` -> `ótima` [spelling] The adjective needs an accent: ótima.

### h-a1-off-topic-music (off_topic, a1)

- Task: What is your name?
- Answer: Eu gosto de música brasileira e toco violão.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `Meu nome é...` [word_choice] The verb 'gostar' is incorrect for expressing liking in this context. 'Eu gosto' is fine, but the sentence structure is not a direct answer to 'What is your name?'. However, focusing on the grammar error: 'gosto' is correct but the sentence is irrelevant. The main issue is the content mismatch. Since the task asks for a name, this answer is wrong. I will focus on the grammatical structure if possible, but here the content is the main issue. I will flag the verb form as it is a common error type in these checks.

### h-b1-off-topic-rain (off_topic, b1)

- Task: Do you think cities need more bicycle lanes?
- Answer: Ontem choveu muito e eu fiquei em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `Ontem` -> `Ontem` [word_order] The correct past tense for 'yesterday' is 'ontem'.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=false answers_the_question=false

### h-a2-english-market (not_portuguese, a2)

- Task: Describe your weekend.
- Answer: On Saturday I went to the market.
- Flags: written_in_portuguese=false answers_the_question=false

## gemma4:e2b / plain

### h-a1-control-weather-cold (control, a1)

- Task: Describe the weather today.
- Answer: Hoje está frio e o céu está nublado.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `o céu está nublado` -> `o céu está nublado` [word_choice] The correct way to say the sky is cloudy is 'o céu está nublado' or 'o céu está nublado'. The structure is correct but the phrasing is slightly unnatural.

### h-a1-control-mother-job (control, a1)

- Task: What does your mother do?
- Answer: Minha mãe é professora e trabalha numa escola.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Minha mãe é professora e trabalha numa escola.` -> `Minha mãe é professora.` [agreement] The sentence structure is correct, but the context implies a personal answer, and the grammar is fine for an A1 level.

### h-a1-control-brother-age (control, a1)

- Task: How old is your brother?
- Answer: Meu irmão tem doze anos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `tem` -> `é` [verb_form] The verb 'ter' is used for possession, but 'ser' is used for age.

### h-a1-control-hobbies (control, a1)

- Task: What do you like to do?
- Answer: Eu gosto de ler livros e ouvir música.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Eu gosto de ler livros e ouvir música.` -> `Eu gosto de ler livros e ouvir música.` [not_brazilian] The sentence is grammatically correct in Brazilian Portuguese.

### h-a1-control-colloquial-ta (control, a1)

- Task: Are you ready?
- Answer: Tá, só um minuto, já vou.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Tá` -> `Estou` [word_choice] This is a colloquial contraction and should be written out in a more standard form.

### h-a1-control-market-purchases (control, a1)

- Task: What did you buy at the market?
- Answer: Comprei bananas, pão e queijo no mercado.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `pão` -> `pão` [word_choice] The correct word for bread in this context is 'pão'.

### h-a1-control-bedroom (control, a1)

- Task: Describe your bedroom.
- Answer: O meu quarto é pequeno. Tem uma cama, uma mesa e uma janela grande.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `O meu quarto` -> `Meu quarto` [article_preposition] The definite article 'o' is usually omitted before possessive pronouns like 'meu' in this context.

### h-a2-control-yesterday-park (control, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu fui ao parque com minha irmã e nós jogamos bola.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Ontem` -> `Ontem` [word_choice] The correct way to say 'yesterday' in this context is 'Ontem' or 'No dia anterior'. 'Ontem' is acceptable but 'Yesterday' is more common in this structure.

### h-a2-control-childhood-house (control, a2)

- Task: Where did you live as a child?
- Answer: Quando eu era criança, morávamos numa casa perto do rio.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `morávamos` -> `morava` [verb_form] The verb 'morávamos' is incorrect for a singular subject ('I').

### h-a2-control-progressive-study (control, a2)

- Task: What is your sister doing now?
- Answer: Ela está estudando para a prova de matemática.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Ela está estudando` -> `Ela estuda` [verb_form] The present continuous tense is used incorrectly for a simple statement about what someone is doing now.

### h-a2-control-comparison-cities (control, a2)

- Task: Compare the two cities.
- Answer: Recife é mais quente do que Curitiba.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `é` -> `está` [verb_form] The verb 'ser' is used for inherent characteristics like temperature.

### h-a2-control-future-tomorrow (control, a2)

- Task: What will you do tomorrow?
- Answer: Amanhã vou acordar cedo e depois vou encontrar meus amigos.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `vou` -> `iria` [verb_form] The future tense is usually expressed with 'ir' in this context.

### h-a2-control-object-pronoun-call (control, a2)

- Task: Did you call your mother?
- Answer: Sim, liguei para ela hoje de manhã.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `liguei` -> `liguei` [verb_form] The correct past tense for 'I called' in this context is 'liguei' or 'liguei' (depending on the dialect, but 'liguei' is common in some contexts, however, the structure is slightly off for a simple past action in this context, but the main issue is the tense/verb form for a simple past action).

### h-a2-control-health-symptoms (control, a2)

- Task: How do you feel?
- Answer: Estou com dor de cabeça e um pouco de febre.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Estou com dor de cabeça e um pouco de febre.` -> `Estou com dor de cabeça e um pouco de febre.` [ser_estar] The correct way to express this in Portuguese is 'Estou com dor de cabeça e um pouco de febre.'

### h-a2-control-directions-pharmacy (control, a2)

- Task: Where is the pharmacy?
- Answer: A farmácia fica duas quadras à frente, à esquerda, ao lado da padaria.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `farmácia` -> `farmácia` [spelling] The correct spelling for pharmacy is 'farmácia'.

### h-a2-control-a-gente-sunday (control, a2)

- Task: How do you and your friends spend Sundays?
- Answer: Aos domingos a gente almoça junto e depois assiste a um filme.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Aos` -> `No` [article_preposition] The correct preposition for this context is 'No'
- Found (false positive): `a gente` -> `nós` [pronoun] The correct way to say 'we' is 'nós' or 'a gente' is acceptable but 'nós' is more standard in this context.
- Found (false positive): `assiste` -> `assistimos` [verb_form] The verb should be in the infinitive form or conjugated correctly for the subject.

### h-b1-control-opinion-bike-lanes (control, b1)

- Task: Do you think cities should have more bicycle lanes?
- Answer: Acho que sim, porque as ciclovias deixam o trânsito mais seguro e as pessoas gastam menos com carro.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `as` -> `as` [article_preposition] The definite article 'as' is incorrect here.

### h-b1-control-subjunctive-wish (control, b1)

- Task: What do you wish for your friend?
- Answer: Desejo que ela seja feliz e que encontre um bom emprego.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Desejo` -> `Eu quero` [word_choice] The verb 'desejo' is too formal for this context.

### h-b1-control-future-subjunctive-salvador (control, b1)

- Task: What will you do when you arrive in Salvador?
- Answer: Quando eu chegar a Salvador, vou ligar para a minha tia.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `ligar` -> `telefonar` [word_choice] The correct verb for 'to call' in this context is 'telefonar' or 'ligar' is acceptable but the context implies a more natural phrasing.

### h-b1-control-imperfect-subjunctive-month (control, b1)

- Task: What would you do with a free month?
- Answer: Se eu tivesse um mês livre, viajaria pelo interior do Brasil.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Se eu tivesse` -> `Se eu tivesse` [verb_form] The conditional tense in Portuguese requires the subjunctive mood after 'se' in this context.

### h-b2-control-reported-doctor (control, b2)

- Task: Report what the doctor said.
- Answer: O médico disse que eu precisava descansar e beber mais água.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `eu precisava` -> `você precisa` [verb_form] The doctor said 'You must rest', which is a command/suggestion, not a past necessity.

### h-b2-control-formal-thanks (control, b2)

- Task: Write a short formal thank-you to a colleague.
- Answer: Prezada Maria, agradeço sinceramente pela sua ajuda no projeto. Cordialmente, João.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Prezada` -> `Dear Maria,` [word_choice] This is too informal for a formal thank-you.

### h-a1-spelling-sao-paulo (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Brasil e moro em Sao Paulo.
- Reference: `Sao` -> São [spelling]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Eu sou do Brasil` -> `Eu sou do Brasil` [word_choice] The correct way to say where you are from is 'Eu sou do Brasil' or 'Eu sou brasileiro(a)', but the structure is slightly unnatural for this simple question.
- Missed: `Sao`

### h-a1-serestar-father-location (erroneous, a1)

- Task: Where is your father?
- Answer: Meu pai é no trabalho agora.
- Reference: `é no` -> está no [ser_estar]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `Meu pai é no trabalho agora.` -> `Meu pai está no trabalho agora.` [word_order] The correct structure for this statement is 'My father is at work now.'

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `Eu sinto` -> `Eu me sinto` [verb_form] The verb 'sentir' is not used in this context; 'sentir-se' is reflexive and the correct structure is needed.
- Found (changes nothing): `quando acordo cedo` -> `quando acordo cedo.` [word_order] The structure for expressing feelings is different; a simple statement about how one feels is expected.

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `À noite` -> `De noite` [article_preposition] The correct way to say 'at night' in this context is 'De noite' or simply 'À noite' without the accent, or 'À noite' is acceptable but the structure is slightly unnatural.
- Missed: `assisto a rádio`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `numa` -> `uma` [article_preposition] The correct article for a feminine noun is 'uma'.
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective should agree in gender with 'casa', which is feminine.
- Missed: `branca casa`

### h-a2-verb-form-ir-summer (erroneous, a2)

- Task: Where did you go last summer?
- Answer: No verão passado eu foi para a praia com a família.
- Reference: `eu foi` -> eu fui [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `eu foi` -> `eu fui` [verb_form] The verb 'ir' in the past tense needs to be conjugated correctly for the first person singular.

### h-a2-verb-form-pôr-table (erroneous, a2)

- Task: What did you do before dinner?
- Answer: Antes do jantar eu pôs a mesa e chamei todo mundo.
- Reference: `eu pôs` -> eu pus [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `pôs` -> `pus` [verb_form] The past tense of 'pôr' in this context should be 'pus', which is the simple past tense.
- Found (false positive): `chamei todo mundo` -> `chamei as pessoas` [word_choice] The phrasing is unnatural; a more natural way to say this would be different.

### h-a2-agreement-red-shoes (erroneous, a2)

- Task: Describe the shoes.
- Answer: Ela comprou dois sapatos novo e vermelhos.
- Reference: `novo` -> novos [agreement]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `novo` -> `novos` [agreement] The adjective must agree in gender and number with the noun 'sapatos' (masculine plural).
- Found (changes nothing): `vermelhos` -> `vermelhos` [agreement] The adjective must agree in gender and number with the noun 'sapatos' (masculine plural).

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `Eu vou para o trabalho em bicicleta` -> `Eu vou de bicicleta` [word_choice] The correct way to say this is 'Eu vou para o trabalho de bicicleta' or 'Eu vou de bicicleta'.

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `Eu esqueci de o guarda-chuva em casa.` -> `Eu esqueci o guarda-chuva em casa.` [word_order] The structure for forgetting something is usually 'esquecer de' followed by the infinitive, not 'de o'.

### h-a2-pronoun-reflexive-hurt (erroneous, a2)

- Task: What happened to you?
- Answer: Eu machuquei ontem jogando futebol.
- Reference: `Eu machuquei` -> Eu me machuquei [pronoun]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `Eu machuquei` -> `Eu me machuquei` [verb_form] The correct past tense for this situation is the simple past or a more appropriate past tense.

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `O leite está no frigorífico.` -> `O leite está na geladeira.` [word_choice] The correct word for refrigerator is 'geladeira' or 'frigorífico' is less common.

### h-a2-not-brazilian-train (erroneous, a2)

- Task: How did you travel to the city?
- Answer: Eu viajei de comboio até a cidade.
- Reference: `comboio` -> trem [not_brazilian]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Eu viajei` -> `Eu viajei` [verb_form] The correct past tense for 'I traveled' in this context is 'Eu viajei' or 'Eu viajei' (depending on the intended meaning, but the structure is slightly awkward for a simple past action).
- Missed: `comboio`

### h-a2-content-car (erroneous, a2)

- Task: Describe the car.
- Answer: O carro é grande e vermelho.
- Reference: `grande e vermelho` -> pequeno e azul [content_mismatch]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `é grande e vermelho` -> `é pequeno e azul` [content_mismatch] The car is described as small and blue in the facts.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `No restaurante` -> `No restaurante` [article_preposition] The correct way to say 'at the restaurant' is 'no restaurante' or 'no restaurante' depending on context, but the structure is wrong here.
- Missed: `perguntei`

### h-b1-verb-form-subjunctive-advice (erroneous, b1)

- Task: What do you advise your friend?
- Answer: Aconselho que você fala com o seu chefe antes de decidir.
- Reference: `fala` -> fale [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Aconselho` -> `Eu aconselho` [verb_form] The correct verb form for 'I advise' in this context is 'Eu aconselho' or simply using the imperative/subjunctive depending on context, but 'Aconselho' is not the standard way to start this advice.
- Found (changes nothing): `você` -> `você` [word_choice] The correct form for 'you' in Brazilian Portuguese is 'você' or 'você' is acceptable, but the structure needs adjustment for natural flow.
- Found (false positive): `o seu chefe` -> `seu chefe` [article_preposition] The definite article 'o' is unnecessary here.
- Missed: `fala`

### h-b1-verb-form-future-subjunctive-querer (erroneous, b1)

- Task: Will you help me?
- Answer: Se você querer, eu ajudo você a mudar.
- Reference: `Se você querer` -> Se você quiser [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `você` -> `você` [pronoun] The correct form for 'you' in Brazilian Portuguese is 'você' or 'você' is acceptable, but 'você' is often replaced by 'você' or 'você' is used in a way that sounds unnatural.
- Missed: `Se você querer`

### h-b1-verb-form-passive-agreement (erroneous, b1)

- Task: Describe what was done to the room.
- Answer: A sala foi pintada e as janelas foi lavadas.
- Reference: `foi lavadas` -> foram lavadas [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `as janelas foi lavadas` -> `as janelas foram lavadas` [agreement] The subject 'janelas' is plural, so the verb should be plural.

### h-b2-verb-form-pluperfect-agreement (erroneous, b2)

- Task: What had happened before the meeting started?
- Answer: Antes da reunião começar, os diretores já tinha decidido o orçamento.
- Reference: `já tinha` -> já tinham [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `já tinha decidido` -> `já tinham decidido` [verb_form] The past perfect tense is not used in this context.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Eu cheguei` -> `Eu cheguei` [verb_form] The verb 'chegar' in the past tense requires the auxiliary verb 'ter' for the first person singular.
- Missed: `a escola`

### h-b2-spelling-ótima (erroneous, b2)

- Task: Describe your idea.
- Answer: Tive uma ideia otima para o projeto.
- Reference: `otima` -> ótima [spelling]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `otima` -> `ótima` [spelling] The correct spelling for 'great' is 'ótima' in Portuguese.

### h-b2-spelling-vários (erroneous, b2)

- Task: Describe the situation.
- Answer: A situação é muito delicada, porque o resultado depende de varios fatores.
- Reference: `varios` -> vários [spelling]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `delicada` -> `delicada` [word_choice] The correct adjective for a situation is 'delicada' or 'delicada', but the context suggests a slight issue with the word choice or phrasing.
- Missed: `varios`

### h-a1-off-topic-music (off_topic, a1)

- Task: What is your name?
- Answer: Eu gosto de música brasileira e toco violão.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Eu gosto de música brasileira e toco violão.` -> `Meu nome é [Your Name].` [content_mismatch] The answer does not address the question 'What is your name?'

### h-b1-off-topic-rain (off_topic, b1)

- Task: Do you think cities need more bicycle lanes?
- Answer: Ontem choveu muito e eu fiquei em casa.
- Response: invalid or missing

## gemma4:e4b-it-qat / few_shot

### h-a1-spelling-sao-paulo (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Brasil e moro em Sao Paulo.
- Reference: `Sao` -> São [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Sao`

### h-a1-spelling-pao (erroneous, a1)

- Task: What do you eat for breakfast?
- Answer: Eu como pao com manteiga.
- Reference: `pao` -> pão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pao`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `assisto a rádio`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `branca casa`

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

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-content-car (erroneous, a2)

- Task: Describe the car.
- Answer: O carro é grande e vermelho.
- Reference: `grande e vermelho` -> pequeno e azul [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Missed: `grande e vermelho`

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `perguntei`

### h-b1-verb-form-subjunctive-advice (erroneous, b1)

- Task: What do you advise your friend?
- Answer: Aconselho que você fala com o seu chefe antes de decidir.
- Reference: `fala` -> fale [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fala`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a escola`

### h-b2-spelling-ótima (erroneous, b2)

- Task: Describe your idea.
- Answer: Tive uma ideia otima para o projeto.
- Reference: `otima` -> ótima [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `otima`

### h-b2-spelling-vários (erroneous, b2)

- Task: Describe the situation.
- Answer: A situação é muito delicada, porque o resultado depende de varios fatores.
- Reference: `varios` -> vários [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `varios`

## gemma4:e4b-it-qat / few_shot_six

### h-a1-spelling-sao-paulo (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Brasil e moro em Sao Paulo.
- Reference: `Sao` -> São [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Sao`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `numa` -> `em uma` [article_preposition] The contraction of 'em' and 'uma' is 'numa'.
- Missed: `branca casa`

### h-a2-agreement-red-shoes (erroneous, a2)

- Task: Describe the shoes.
- Answer: Ela comprou dois sapatos novo e vermelhos.
- Reference: `novo` -> novos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `novo`

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-content-car (erroneous, a2)

- Task: Describe the car.
- Answer: O carro é grande e vermelho.
- Reference: `grande e vermelho` -> pequeno e azul [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Missed: `grande e vermelho`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a escola`

### h-a2-english-market (not_portuguese, a2)

- Task: Describe your weekend.
- Answer: On Saturday I went to the market.
- Flags: written_in_portuguese=true answers_the_question=false

## gemma4:e4b-it-qat / plain

### h-a2-control-directions-pharmacy (control, a2)

- Task: Where is the pharmacy?
- Answer: A farmácia fica duas quadras à frente, à esquerda, ao lado da padaria.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `farmácia` -> `farmácia` [spelling] The word 'farmácia' is misspelled; it needs an accent on the 'a'.

### h-b2-control-reported-doctor (control, b2)

- Task: Report what the doctor said.
- Answer: O médico disse que eu precisava descansar e beber mais água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `eu precisava` -> `eu deveria` [verb_form] The doctor gave a direct command, so the reported speech should use the imperative or a modal verb, not the imperfect indicative.

### h-a1-spelling-sao-paulo (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Brasil e moro em Sao Paulo.
- Reference: `Sao` -> São [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Sao`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `assisto a rádio`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `numa` -> `uma` [article_preposition] The correct contraction for 'em uma' is 'numa', but since 'casa' is feminine, the correct contraction is 'em uma' or 'uma'.
- Missed: `branca casa`

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `esqueci de o` -> `esqueci o` [article_preposition] The correct structure after the verb 'esquecer' when referring to an object requires the preposition 'de' followed by the object, not 'de o'.

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-content-car (erroneous, a2)

- Task: Describe the car.
- Answer: O carro é grande e vermelho.
- Reference: `grande e vermelho` -> pequeno e azul [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `grande` -> `pequeno` [content_mismatch] The car is small, not big.
- Found (duplicate): `vermelho` -> `azul` [content_mismatch] The car is blue, not red.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No restaurante` -> `No restaurante` [article_preposition] The preposition 'em' is needed before 'restaurante' when indicating location.
- Missed: `perguntei`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a escola`

### h-b2-spelling-ótima (erroneous, b2)

- Task: Describe your idea.
- Answer: Tive uma ideia otima para o projeto.
- Reference: `otima` -> ótima [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `otima`

### h-a1-off-topic-music (off_topic, a1)

- Task: What is your name?
- Answer: Eu gosto de música brasileira e toco violão.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de música brasileira e toco violão.` -> `Meu nome é [Your Name].` [content_mismatch] The answer does not state the name as requested by the task.

### h-b1-off-topic-rain (off_topic, b1)

- Task: Do you think cities need more bicycle lanes?
- Answer: Ontem choveu muito e eu fiquei em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Ontem choveu muito e eu fiquei em casa.` -> `` [content_mismatch] The answer does not address the question about bicycle lanes in cities.

## gemma4:e4b / few_shot

### h-a2-control-yesterday-park (control, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu fui ao parque com minha irmã e nós jogamos bola.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `minha` -> `` [content_mismatch] The possessive adjective must be masculine to agree with 'irmão' (implied by the context of the action, though 'irmã' is feminine, the structure suggests a general activity, and 'irmã' is feminine, so the error is in the agreement of the possessive pronoun if we assume the speaker is male, but since the speaker is not specified, we check the noun 'irmã'. 'Minha' is correct for 'irmã'. However, if we strictly check the grammar, 'minha irmã' is correct. Let's re-evaluate. The sentence is grammatically sound in Portuguese. No mistakes found based on the provided facts or common errors.

### h-a1-agreement-black-cat (erroneous, a1)

- Task: Describe your cats.
- Answer: Meu gato é pequeno e minha gata é preto.
- Reference: `preto` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `preto` -> `preta` [agreement] The adjective must be feminine to agree with 'gata'.

### h-a1-article-contraction-hospital (erroneous, a1)

- Task: Where does your father work?
- Answer: Meu pai trabalha em o hospital da cidade.
- Reference: `em o` -> no [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `em o` -> `no` [article_preposition] The preposition 'em' must contract with the masculine definite article 'o' to form 'no'.

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `assisto a rádio`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective must be feminine to agree with 'casa'.
- Missed: `branca casa`

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

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-content-car (erroneous, a2)

- Task: Describe the car.
- Answer: O carro é grande e vermelho.
- Reference: `grande e vermelho` -> pequeno e azul [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `grande` -> `pequeno` [content_mismatch] The car is described as small, not large.
- Found (duplicate): `vermelho` -> `azul` [content_mismatch] The car is described as blue, not red.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `a escola` -> `na escola` [article_preposition] The preposition 'em' is needed before 'escola' to indicate location.

### h-b2-spelling-ótima (erroneous, b2)

- Task: Describe your idea.
- Answer: Tive uma ideia otima para o projeto.
- Reference: `otima` -> ótima [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `otima`

### h-a1-off-topic-music (off_topic, a1)

- Task: What is your name?
- Answer: Eu gosto de música brasileira e toco violão.
- Response: invalid or missing

### h-b1-off-topic-rain (off_topic, b1)

- Task: Do you think cities need more bicycle lanes?
- Answer: Ontem choveu muito e eu fiquei em casa.
- Response: invalid or missing

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=true answers_the_question=false

### h-a2-english-market (not_portuguese, a2)

- Task: Describe your weekend.
- Answer: On Saturday I went to the market.
- Flags: written_in_portuguese=true answers_the_question=false

## gemma4:e4b / few_shot_six

### h-a2-control-yesterday-park (control, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu fui ao parque com minha irmã e nós jogamos bola.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `com minha irmã` -> `com a minha irmã` [article_preposition] The possessive adjective 'minha' requires a definite article before 'irmã' in this context, or the structure should be changed.

### h-a2-control-childhood-house (control, a2)

- Task: Where did you live as a child?
- Answer: Quando eu era criança, morávamos numa casa perto do rio.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morávamos` -> `moro` [verb_form] The verb must agree with the subject 'eu'.

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective must be feminine to agree with 'casa'.
- Missed: `branca casa`

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
- Found (changes nothing): `machuquei` -> `machuquei` [verb_form] The verb 'machucar' in the past tense for 'eu' is 'machuquei'.
- Missed: `Eu machuquei`

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-not-brazilian-train (erroneous, a2)

- Task: How did you travel to the city?
- Answer: Eu viajei de comboio até a cidade.
- Reference: `comboio` -> trem [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `comboio`

### h-a2-content-car (erroneous, a2)

- Task: Describe the car.
- Answer: O carro é grande e vermelho.
- Reference: `grande e vermelho` -> pequeno e azul [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `grande` -> `pequeno` [content_mismatch] The car is described as small, not large.
- Found (duplicate): `vermelho` -> `azul` [content_mismatch] The car is described as blue, not red.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `a escola` -> `na escola` [article_preposition] The preposition 'a' is not needed before 'escola' in this context.

### h-a1-off-topic-music (off_topic, a1)

- Task: What is your name?
- Answer: Eu gosto de música brasileira e toco violão.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto` -> `Meu nome é` [content_mismatch] The answer does not state the name.

### h-b1-off-topic-rain (off_topic, b1)

- Task: Do you think cities need more bicycle lanes?
- Answer: Ontem choveu muito e eu fiquei em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Ontem` -> `(no answer)` [content_mismatch] The answer does not address the question about bicycle lanes.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=true answers_the_question=false

### h-a2-english-market (not_portuguese, a2)

- Task: Describe your weekend.
- Answer: On Saturday I went to the market.
- Flags: written_in_portuguese=true answers_the_question=false

## gemma4:e4b / plain

### h-a2-control-childhood-house (control, a2)

- Task: Where did you live as a child?
- Answer: Quando eu era criança, morávamos numa casa perto do rio.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morávamos` -> `morava` [verb_form] The subject 'eu' (I) requires the first-person singular form of the verb.

### h-a2-control-directions-pharmacy (control, a2)

- Task: Where is the pharmacy?
- Answer: A farmácia fica duas quadras à frente, à esquerda, ao lado da padaria.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `farmácia` -> `farmácia` [spelling] The word for pharmacy in Portuguese is 'farmácia', which requires an accent.

### h-b1-control-opinion-bike-lanes (control, b1)

- Task: Do you think cities should have more bicycle lanes?
- Answer: Acho que sim, porque as ciclovias deixam o trânsito mais seguro e as pessoas gastam menos com carro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `gastam` -> `gastam` [word_choice] The verb 'gastar' in this context should be in the infinitive form after 'menos com'.

### h-b2-control-reported-doctor (control, b2)

- Task: Report what the doctor said.
- Answer: O médico disse que eu precisava descansar e beber mais água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `eu` -> `você` [pronoun] The doctor was speaking to the patient, so the pronoun should be 'você' or 'vocês' depending on the context, but 'você' is the most direct translation for 'you'.

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `rádio` -> `rádio` [word_choice] The word 'rádio' refers to the medium, but the activity is usually 'listen to the radio'.
- Missed: `assisto a rádio`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective 'branca' should be preceded by the article 'uma' which is feminine, but the noun 'casa' is feminine, so the adjective should agree in gender and number.
- Missed: `branca casa`

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-content-car (erroneous, a2)

- Task: Describe the car.
- Answer: O carro é grande e vermelho.
- Reference: `grande e vermelho` -> pequeno e azul [content_mismatch]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `grande` -> `pequeno` [content_mismatch] The car is small, not big.
- Found (duplicate): `vermelho` -> `azul` [content_mismatch] The car is blue, not red.

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `oito` -> `oito` [article_preposition] The time 'oito' needs the article 'as' when referring to the hour.
- Missed: `a escola`

### h-b2-spelling-ótima (erroneous, b2)

- Task: Describe your idea.
- Answer: Tive uma ideia otima para o projeto.
- Reference: `otima` -> ótima [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `otima`

### h-a1-off-topic-music (off_topic, a1)

- Task: What is your name?
- Answer: Eu gosto de música brasileira e toco violão.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de música brasileira e toco violão.` -> `Meu nome é [Your Name]` [content_mismatch] The answer does not state the name as requested by the question.

### h-b1-off-topic-rain (off_topic, b1)

- Task: Do you think cities need more bicycle lanes?
- Answer: Ontem choveu muito e eu fiquei em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Ontem choveu muito e eu fiquei em casa.` -> `` [content_mismatch] The answer does not address the question about bicycle lanes.

## granite4.2:3b / few_shot

### h-a1-spelling-sao-paulo (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Brasil e moro em Sao Paulo.
- Reference: `Sao` -> São [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Sao`

### h-a1-spelling-pao (erroneous, a1)

- Task: What do you eat for breakfast?
- Answer: Eu como pao com manteiga.
- Reference: `pao` -> pão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pao`

### h-a1-agreement-black-cat (erroneous, a1)

- Task: Describe your cats.
- Answer: Meu gato é pequeno e minha gata é preto.
- Reference: `preto` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `preto`

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu tem`

### h-a1-serestar-father-location (erroneous, a1)

- Task: Where is your father?
- Answer: Meu pai é no trabalho agora.
- Reference: `é no` -> está no [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `é no`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `assisto a rádio`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `branca casa`

### h-a2-verb-form-ir-summer (erroneous, a2)

- Task: Where did you go last summer?
- Answer: No verão passado eu foi para a praia com a família.
- Reference: `eu foi` -> eu fui [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu foi`

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

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `de o`

### h-a2-pronoun-reflexive-hurt (erroneous, a2)

- Task: What happened to you?
- Answer: Eu machuquei ontem jogando futebol.
- Reference: `Eu machuquei` -> Eu me machuquei [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu machuquei`

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-not-brazilian-train (erroneous, a2)

- Task: How did you travel to the city?
- Answer: Eu viajei de comboio até a cidade.
- Reference: `comboio` -> trem [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `comboio`

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `perguntei`

### h-b1-verb-form-subjunctive-advice (erroneous, b1)

- Task: What do you advise your friend?
- Answer: Aconselho que você fala com o seu chefe antes de decidir.
- Reference: `fala` -> fale [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fala`

### h-b1-verb-form-future-subjunctive-querer (erroneous, b1)

- Task: Will you help me?
- Answer: Se você querer, eu ajudo você a mudar.
- Reference: `Se você querer` -> Se você quiser [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Se você querer`

### h-b2-verb-form-pluperfect-agreement (erroneous, b2)

- Task: What had happened before the meeting started?
- Answer: Antes da reunião começar, os diretores já tinha decidido o orçamento.
- Reference: `já tinha` -> já tinham [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `já tinha`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a escola`

### h-b2-spelling-ótima (erroneous, b2)

- Task: Describe your idea.
- Answer: Tive uma ideia otima para o projeto.
- Reference: `otima` -> ótima [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `otima`

### h-b2-spelling-vários (erroneous, b2)

- Task: Describe the situation.
- Answer: A situação é muito delicada, porque o resultado depende de varios fatores.
- Reference: `varios` -> vários [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `varios`

### h-a1-off-topic-music (off_topic, a1)

- Task: What is your name?
- Answer: Eu gosto de música brasileira e toco violão.
- Flags: written_in_portuguese=true answers_the_question=true

### h-b1-off-topic-rain (off_topic, b1)

- Task: Do you think cities need more bicycle lanes?
- Answer: Ontem choveu muito e eu fiquei em casa.
- Flags: written_in_portuguese=true answers_the_question=true

## granite4.2:3b / few_shot_six

### h-a1-spelling-sao-paulo (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Brasil e moro em Sao Paulo.
- Reference: `Sao` -> São [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Sao`

### h-a1-spelling-pao (erroneous, a1)

- Task: What do you eat for breakfast?
- Answer: Eu como pao com manteiga.
- Reference: `pao` -> pão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pao`

### h-a1-agreement-black-cat (erroneous, a1)

- Task: Describe your cats.
- Answer: Meu gato é pequeno e minha gata é preto.
- Reference: `preto` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `preto`

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu tem`

### h-a1-serestar-father-location (erroneous, a1)

- Task: Where is your father?
- Answer: Meu pai é no trabalho agora.
- Reference: `é no` -> está no [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `é no`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `assisto a rádio`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `branca casa`

### h-a2-verb-form-ir-summer (erroneous, a2)

- Task: Where did you go last summer?
- Answer: No verão passado eu foi para a praia com a família.
- Reference: `eu foi` -> eu fui [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu foi`

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

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `de o`

### h-a2-pronoun-reflexive-hurt (erroneous, a2)

- Task: What happened to you?
- Answer: Eu machuquei ontem jogando futebol.
- Reference: `Eu machuquei` -> Eu me machuquei [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu machuquei`

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-not-brazilian-train (erroneous, a2)

- Task: How did you travel to the city?
- Answer: Eu viajei de comboio até a cidade.
- Reference: `comboio` -> trem [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `comboio`

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `perguntei`

### h-b1-verb-form-subjunctive-advice (erroneous, b1)

- Task: What do you advise your friend?
- Answer: Aconselho que você fala com o seu chefe antes de decidir.
- Reference: `fala` -> fale [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fala`

### h-b1-verb-form-future-subjunctive-querer (erroneous, b1)

- Task: Will you help me?
- Answer: Se você querer, eu ajudo você a mudar.
- Reference: `Se você querer` -> Se você quiser [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Se você querer`

### h-b2-verb-form-pluperfect-agreement (erroneous, b2)

- Task: What had happened before the meeting started?
- Answer: Antes da reunião começar, os diretores já tinha decidido o orçamento.
- Reference: `já tinha` -> já tinham [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (quote not in the answer): `dirtores` -> `diretores` [spelling] The word 'diretores' is misspelled; it should be 'diretores' with an accent on the 'e'.
- Missed: `já tinha`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a escola`

### h-b2-spelling-ótima (erroneous, b2)

- Task: Describe your idea.
- Answer: Tive uma ideia otima para o projeto.
- Reference: `otima` -> ótima [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `otima`

### h-b2-spelling-vários (erroneous, b2)

- Task: Describe the situation.
- Answer: A situação é muito delicada, porque o resultado depende de varios fatores.
- Reference: `varios` -> vários [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `varios`

### h-a1-off-topic-music (off_topic, a1)

- Task: What is your name?
- Answer: Eu gosto de música brasileira e toco violão.
- Flags: written_in_portuguese=true answers_the_question=true

### h-b1-off-topic-rain (off_topic, b1)

- Task: Do you think cities need more bicycle lanes?
- Answer: Ontem choveu muito e eu fiquei em casa.
- Flags: written_in_portuguese=true answers_the_question=true

## granite4.2:3b / plain

### h-a1-spelling-sao-paulo (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Brasil e moro em Sao Paulo.
- Reference: `Sao` -> São [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Sao`

### h-a1-spelling-pao (erroneous, a1)

- Task: What do you eat for breakfast?
- Answer: Eu como pao com manteiga.
- Reference: `pao` -> pão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pao`

### h-a1-agreement-black-cat (erroneous, a1)

- Task: Describe your cats.
- Answer: Meu gato é pequeno e minha gata é preto.
- Reference: `preto` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `preto`

### h-a1-verb-form-ter-age (erroneous, a1)

- Task: How old are you?
- Answer: Eu tem vinte e cinco anos.
- Reference: `Eu tem` -> Eu tenho [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu tem`

### h-a1-serestar-father-location (erroneous, a1)

- Task: Where is your father?
- Answer: Meu pai é no trabalho agora.
- Reference: `é no` -> está no [ser_estar]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `é no`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `assisto a rádio`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `branca casa`

### h-a2-verb-form-ir-summer (erroneous, a2)

- Task: Where did you go last summer?
- Answer: No verão passado eu foi para a praia com a família.
- Reference: `eu foi` -> eu fui [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `eu foi`

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

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `de o`

### h-a2-pronoun-reflexive-hurt (erroneous, a2)

- Task: What happened to you?
- Answer: Eu machuquei ontem jogando futebol.
- Reference: `Eu machuquei` -> Eu me machuquei [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu machuquei`

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-not-brazilian-train (erroneous, a2)

- Task: How did you travel to the city?
- Answer: Eu viajei de comboio até a cidade.
- Reference: `comboio` -> trem [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `comboio`

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `perguntei`

### h-b1-verb-form-subjunctive-advice (erroneous, b1)

- Task: What do you advise your friend?
- Answer: Aconselho que você fala com o seu chefe antes de decidir.
- Reference: `fala` -> fale [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fala`

### h-b1-verb-form-future-subjunctive-querer (erroneous, b1)

- Task: Will you help me?
- Answer: Se você querer, eu ajudo você a mudar.
- Reference: `Se você querer` -> Se você quiser [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Se você querer`

### h-b2-crase-school (erroneous, b2)

- Task: What did you do at eight?
- Answer: Eu cheguei a escola às oito.
- Reference: `a escola` -> à escola [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `a escola`

### h-b2-spelling-ótima (erroneous, b2)

- Task: Describe your idea.
- Answer: Tive uma ideia otima para o projeto.
- Reference: `otima` -> ótima [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `otima`

### h-b2-spelling-vários (erroneous, b2)

- Task: Describe the situation.
- Answer: A situação é muito delicada, porque o resultado depende de varios fatores.
- Reference: `varios` -> vários [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `varios`

### h-a1-off-topic-music (off_topic, a1)

- Task: What is your name?
- Answer: Eu gosto de música brasileira e toco violão.
- Flags: written_in_portuguese=true answers_the_question=true

### h-b1-off-topic-rain (off_topic, b1)

- Task: Do you think cities need more bicycle lanes?
- Answer: Ontem choveu muito e eu fiquei em casa.
- Flags: written_in_portuguese=true answers_the_question=true

### h-a2-english-market (not_portuguese, a2)

- Task: Describe your weekend.
- Answer: On Saturday I went to the market.
- Flags: written_in_portuguese=true answers_the_question=true

## ministral-3:14b / few_shot

### h-a2-control-childhood-house (control, a2)

- Task: Where did you live as a child?
- Answer: Quando eu era criança, morávamos numa casa perto do rio.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morávamos` -> `morava` [verb_form] The verb should be in the first-person singular (morava) to match the subject 'eu'.

### h-b1-control-future-subjunctive-salvador (control, b1)

- Task: What will you do when you arrive in Salvador?
- Answer: Quando eu chegar a Salvador, vou ligar para a minha tia.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a` -> `em` [article_preposition] The correct preposition for cities is 'em' (not 'a') when referring to arrival.

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `bem` -> `me sinto bem` [word_choice] 'Bem' is an adverb; the correct adjective form is 'bem' (good) but the correct phrase here is 'me sinto bem' (I feel good).
- Missed: `Eu sinto`

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `rádio` -> `rádio` [article_preposition] The correct word for 'radio' in this context (listening) is 'rádio' but the preposition 'a' is incorrect for this verb.
- Missed: `assisto a rádio`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective must match the gender of 'casa' (feminine).
- Missed: `branca casa`

### h-a2-verb-form-pôr-table (erroneous, a2)

- Task: What did you do before dinner?
- Answer: Antes do jantar eu pôs a mesa e chamei todo mundo.
- Reference: `eu pôs` -> eu pus [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `pôs` -> `pus` [verb_form] The verb 'por' in the past tense for 'eu' should be 'pus'.
- Found (false positive): `chamei` -> `` [verb_form] The verb 'chamar' in the past tense for 'eu' should be 'chamei' is correct, but the correct form is 'chamei' (no mistake here). However, the correct verb for 'call' in this context is 'chamei' is correct, but the verb 'chamar' is correct. The mistake is in the verb form for 'call everyone', which should be 'chamei' is correct, but the correct verb form is 'chamei' is correct. **Correction: No mistake here.**

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

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [verb_form] The verb should be 'pedi' (past tense of 'pedir' meaning 'to order').
- Found (false positive): `uma pizza e um suco` -> `uma pizza e um suco de laranja` [word_choice] The correct word for 'juice' in this context is 'suco de' + the drink (e.g., 'suco de laranja'). 'Suco' alone is too vague.

## ministral-3:14b / few_shot_six

### h-a2-control-childhood-house (control, a2)

- Task: Where did you live as a child?
- Answer: Quando eu era criança, morávamos numa casa perto do rio.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morávamos` -> `morava` [verb_form] The verb must agree with 'eu' (singular), not 'nós'.

### h-b1-control-future-subjunctive-salvador (control, b1)

- Task: What will you do when you arrive in Salvador?
- Answer: Quando eu chegar a Salvador, vou ligar para a minha tia.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a` -> `em` [article_preposition] The correct preposition for cities is 'em' (not 'a').

### h-b2-control-reported-doctor (control, b2)

- Task: Report what the doctor said.
- Answer: O médico disse que eu precisava descansar e beber mais água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `precisava` -> `preciso` [verb_form] The doctor is giving a present directive, so the present tense 'preciso' is needed.

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [word_choice] 'Branca' does not describe a house; it describes color.
- Missed: `branca casa`

### h-a2-verb-form-pôr-table (erroneous, a2)

- Task: What did you do before dinner?
- Answer: Antes do jantar eu pôs a mesa e chamei todo mundo.
- Reference: `eu pôs` -> eu pus [verb_form]
- Response: invalid or missing

### h-a2-agreement-red-shoes (erroneous, a2)

- Task: Describe the shoes.
- Answer: Ela comprou dois sapatos novo e vermelhos.
- Reference: `novo` -> novos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `novo` -> `novos` [agreement] The adjective must agree in number with 'sapatos'.
- Found (false positive): `e vermelhos` -> `vermelhos e novos` [word_order] The adjective should be placed before the noun for clarity and natural word order.

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] The verb should be 'pedi' (to order).
- Found (false positive): `uma pizza e um suco` -> `uma pizza **por** e um suco` [article_preposition] The preposition 'por' is missing before 'uma pizza' (you ordered *for* yourself).

## ministral-3:14b / plain

### h-a1-control-weather-cold (control, a1)

- Task: Describe the weather today.
- Answer: Hoje está frio e o céu está nublado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nublado` -> `encoberto` [not_brazilian] The adjective for 'cloudy' in Brazilian Portuguese is *encoberto* (or *coberto*), not *nublado* (which is used in European Portuguese).

### h-a1-control-mother-job (control, a1)

- Task: What does your mother do?
- Answer: Minha mãe é professora e trabalha numa escola.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `num` -> `na` [article_preposition] The contraction 'num' is not used before a feminine noun ('escola').

### h-a1-control-brother-age (control, a1)

- Task: How old is your brother?
- Answer: Meu irmão tem doze anos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `doze` -> `dois` [agreement] The number twelve is written as *dois* (masculine) when used with *anos* (masculine plural).

### h-a1-control-colloquial-ta (control, a1)

- Task: Are you ready?
- Answer: Tá, só um minuto, já vou.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Tá` -> `Estou` [not_brazilian] The task asks for a written answer, and 'tá' is a spoken contraction of 'está'.

### h-a1-control-market-purchases (control, a1)

- Task: What did you buy at the market?
- Answer: Comprei bananas, pão e queijo no mercado.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Comprei` -> `Compro` [verb_form] The verb should be in the present tense for a general statement about purchases.

### h-a1-control-bedroom (control, a1)

- Task: Describe your bedroom.
- Answer: O meu quarto é pequeno. Tem uma cama, uma mesa e uma janela grande.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `uma janela grande` -> `uma grande janela` [word_order] The adjective should come after the noun in Brazilian Portuguese at this level.

### h-a2-control-yesterday-park (control, a2)

- Task: What did you do yesterday?
- Answer: Ontem eu fui ao parque com minha irmã e nós jogamos bola.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `jogamos bola` -> `jogamos bola de futebol` [article_preposition] The verb *jogar* requires a preposition (*de*) when referring to playing a ball game.

### h-a2-control-childhood-house (control, a2)

- Task: Where did you live as a child?
- Answer: Quando eu era criança, morávamos numa casa perto do rio.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `morávamos` -> `morava` [agreement] The verb should agree with the subject 'eu' (I), not 'nós' (we).

### h-a2-control-future-tomorrow (control, a2)

- Task: What will you do tomorrow?
- Answer: Amanhã vou acordar cedo e depois vou encontrar meus amigos.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `encontrar` -> `encontrar**me**` [verb_form] The verb should be in the future tense to match the question about tomorrow.

### h-a2-control-health-symptoms (control, a2)

- Task: How do you feel?
- Answer: Estou com dor de cabeça e um pouco de febre.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `um pouco` -> `pouco` [word_choice] The phrase 'um pouco de' is incorrect here; 'um pouco' alone is sufficient to mean 'a little'.

### h-a2-control-directions-pharmacy (control, a2)

- Task: Where is the pharmacy?
- Answer: A farmácia fica duas quadras à frente, à esquerda, ao lado da padaria.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `à frente` -> `adiante` [article_preposition] The correct preposition for 'ahead' in this context is *adiante*, not *à frente* (which is used with *ir* or *vir*).

### h-a2-control-a-gente-sunday (control, a2)

- Task: How do you and your friends spend Sundays?
- Answer: Aos domingos a gente almoça junto e depois assiste a um filme.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `assiste` -> `assistem` [verb_form] The verb should agree with the plural subject 'a gente'.

### h-b1-control-opinion-bike-lanes (control, b1)

- Task: Do you think cities should have more bicycle lanes?
- Answer: Acho que sim, porque as ciclovias deixam o trânsito mais seguro e as pessoas gastam menos com carro.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ciclovias` -> `faixas de bicicleta` [word_choice] The word *ciclovia* in Brazilian Portuguese refers to a shared path for pedestrians and cyclists, not a dedicated bicycle lane.
- Found (false positive): `menos com carro` -> `menos em carro` [word_order] The phrase *menos com carro* is incorrect; the correct expression is *menos em carro* (or *menos gastando com carro* for full clarity).

### h-b1-control-subjunctive-wish (control, b1)

- Task: What do you wish for your friend?
- Answer: Desejo que ela seja feliz e que encontre um bom emprego.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ela` -> `a` [pronoun] The verb *encontrar* is in the subjunctive mood, requiring the object pronoun *a* (not *ela*) to avoid ambiguity or awkwardness in this context.

### h-b1-control-future-subjunctive-salvador (control, b1)

- Task: What will you do when you arrive in Salvador?
- Answer: Quando eu chegar a Salvador, vou ligar para a minha tia.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `a` -> `em` [article_preposition] The correct preposition for arriving in a city is *em*, not *a*.

### h-b1-control-imperfect-subjunctive-month (control, b1)

- Task: What would you do with a free month?
- Answer: Se eu tivesse um mês livre, viajaria pelo interior do Brasil.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `interior` -> `campo` [word_choice] The intended meaning is likely 'the countryside' or 'rural areas,' not the literal 'interior' (which refers to the inner regions of a country).

### h-b2-control-reported-doctor (control, b2)

- Task: Report what the doctor said.
- Answer: O médico disse que eu precisava descansar e beber mais água.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `precisava` -> `preciso` [verb_form] The doctor gave a command, so the present tense is needed.

### h-b2-control-formal-thanks (control, b2)

- Task: Write a short formal thank-you to a colleague.
- Answer: Prezada Maria, agradeço sinceramente pela sua ajuda no projeto. Cordialmente, João.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Prezada` -> `Prezada` [word_choice] The greeting 'Prezada' is overly formal for most professional contexts in Brazilian Portuguese; 'Prezada' is more common.

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `bem` -> `me bem` [word_choice] The adjective 'bem' means 'good' but does not collocate with 'sinto'. The correct collocation is 'sinto-me bem'.
- Found (false positive): `cedo` -> `cedinho` [word_choice] The correct spelling is 'cedo' but the correct form for this context is 'cedinho' or 'cedinho' (informal) to express 'early' in a more natural way when referring to waking up.
- Missed: `Eu sinto`

### h-a1-word-order-white-house (erroneous, a1)

- Task: Describe your house.
- Answer: Eu moro numa branca casa perto da praia.
- Reference: `branca casa` -> casa branca [word_order]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `branca` -> `branca` [agreement] The adjective 'branca' does not match the noun 'casa' in gender.
- Missed: `branca casa`

### h-a2-verb-form-pôr-table (erroneous, a2)

- Task: What did you do before dinner?
- Answer: Antes do jantar eu pôs a mesa e chamei todo mundo.
- Reference: `eu pôs` -> eu pus [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `pôs` -> `pus` [verb_form] The verb *por* in the past tense for 'I' should be *pus*, not *pôs* (which is the third-person form).
- Found (false positive): `chamei` -> `` [verb_form] The verb *chamar* in the past tense for 'I' should be *chamei*, but the correct form is actually *chamei* (which is correct). This is a false flag—no mistake here.

### h-a2-agreement-red-shoes (erroneous, a2)

- Task: Describe the shoes.
- Answer: Ela comprou dois sapatos novo e vermelhos.
- Reference: `novo` -> novos [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `dois` -> `duas` [agreement] The subject is feminine ('sapatos' is plural of 'sapato', but the adjective 'novo' must agree with the subject's gender, which is masculine here).
- Found (matched): `novo` -> `novos` [agreement] The adjective must agree in gender and number with the noun it modifies ('sapatos' is masculine plural).
- Found (false positive): `e` -> `,` [word_order] The correct conjunction for adding two adjectives is 'e' only if they are not cumulative. Here, 'e' is unnecessary; the adjectives should be separated by a comma or 'e' in a different context.

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`

### h-a2-article-contraction-umbrella (erroneous, a2)

- Task: Why are you wet?
- Answer: Eu esqueci de o guarda-chuva em casa.
- Reference: `de o` -> do [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de o` -> `do` [article_preposition] The preposition *de* cannot be used with the definite article *o* in this context.
- Found (false positive): `esqueci` -> `Eu esqueci` [pronoun] The verb *esquecer* requires the preposition *de* before the object, but the subject pronoun *eu* is missing.

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] The verb *pedir* (to order) is needed here, not *perguntar* (to ask).
- Found (changes nothing): `uma pizza e um suco` -> `uma pizza e um suco` [word_order] The correct word for *juice* in Brazilian Portuguese is *suco* (correct), but the article *um* should be *um* before *suco* (correct), but the mistake is in the word order—*uma pizza* is correct, but *um suco* is correct too, but the issue is the missing contraction or article before *suco* is not wrong here. However, the main mistake is the verb choice.

### h-a1-off-topic-music (off_topic, a1)

- Task: What is your name?
- Answer: Eu gosto de música brasileira e toco violão.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu gosto de música brasileira e toco violão.` -> `N/A (answer should state the name)` [content_mismatch] The answer does not address the question about the learner's name.

## qwen3.5:4b / few_shot

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] The verb 'perguntar' is not used with direct objects like food items; one should use 'pedir'.
- Found (false positive): `um suco` -> `o suco` [article_preposition] The structure 'pedir um [item]' is incorrect for ordering; the correct form is 'pedir o [item]'.

### h-b1-verb-form-future-subjunctive-querer (erroneous, b1)

- Task: Will you help me?
- Answer: Se você querer, eu ajudo você a mudar.
- Reference: `Se você querer` -> Se você quiser [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `mudar` -> `você` [word_choice] The verb 'mudar' does not match the context of helping with a task or favor implied by the question.
- Missed: `Se você querer`

### h-a1-off-topic-music (off_topic, a1)

- Task: What is your name?
- Answer: Eu gosto de música brasileira e toco violão.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu` -> `(remove)` [word_order] The subject 'I' is unnecessary and makes the sentence sound unnatural for this context.
- Found (false positive): `gosto de música brasileira e toco violão` -> `Meu nome é...` [content_mismatch] This statement does not answer the question about the name.

### h-b1-off-topic-rain (off_topic, b1)

- Task: Do you think cities need more bicycle lanes?
- Answer: Ontem choveu muito e eu fiquei em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Ontem choveu muito e eu fiquei em casa.` -> `Acho que as cidades precisam de mais faixas para bicicletas.` [content_mismatch] The answer does not address the question about bicycle lanes or cities.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=true answers_the_question=false

## qwen3.5:4b / few_shot_six

### h-a1-spelling-sao-paulo (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Brasil e moro em Sao Paulo.
- Reference: `Sao` -> São [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Sao`

### h-a1-agreement-black-cat (erroneous, a1)

- Task: Describe your cats.
- Answer: Meu gato é pequeno e minha gata é preto.
- Reference: `preto` -> preta [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `preto` -> `preto` [agreement] The adjective must be masculine to agree with 'gato'.
- Missed: `preto`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a1-word-choice-radio (erroneous, a1)

- Task: What do you do at night?
- Answer: À noite eu assisto a rádio.
- Reference: `assisto a rádio` -> ouço rádio | ouço a rádio [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `assisto a rádio`

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-a2-word-choice-pedir-pizza (erroneous, a2)

- Task: What did you do at the restaurant?
- Answer: No restaurante eu perguntei uma pizza e um suco.
- Reference: `perguntei` -> pedi [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `perguntei` -> `pedi` [word_choice] The verb 'perguntar' is not used with a direct object like 'pizza'; one orders food, not asks it.
- Found (false positive): `um suco` -> `de um suco` [article_preposition] The verb 'pedir' requires the preposition 'de' before the object being ordered.

### h-b1-verb-form-future-subjunctive-querer (erroneous, b1)

- Task: Will you help me?
- Answer: Se você querer, eu ajudo você a mudar.
- Reference: `Se você querer` -> Se você quiser [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ajudo` -> `ajuda` [pronoun] The verb 'ajudar' is transitive and does not require an object pronoun before the infinitive in this context.
- Missed: `Se você querer`

### h-a1-off-topic-music (off_topic, a1)

- Task: What is your name?
- Answer: Eu gosto de música brasileira e toco violão.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `me chamo` [word_choice] The verb 'gostar' does not answer the question about the name; it means 'to like'.

### h-b1-off-topic-rain (off_topic, b1)

- Task: Do you think cities need more bicycle lanes?
- Answer: Ontem choveu muito e eu fiquei em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Ontem choveu muito e eu fiquei em casa.` -> `Acho que sim, porque as cidades precisam de mais espaço para bicicletas.` [content_mismatch] The answer does not address the question about bicycle lanes or cities.

### h-a1-english-apartment (not_portuguese, a1)

- Task: Where do you live?
- Answer: I live in a small apartment near the beach.
- Flags: written_in_portuguese=true answers_the_question=false

### h-a2-english-market (not_portuguese, a2)

- Task: Describe your weekend.
- Answer: On Saturday I went to the market.
- Flags: written_in_portuguese=true answers_the_question=false

## qwen3.5:4b / plain

### h-a1-spelling-sao-paulo (erroneous, a1)

- Task: Where are you from?
- Answer: Eu sou do Brasil e moro em Sao Paulo.
- Reference: `Sao` -> São [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Sao`

### h-a1-spelling-pao (erroneous, a1)

- Task: What do you eat for breakfast?
- Answer: Eu como pao com manteiga.
- Reference: `pao` -> pão [spelling]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `pao`

### h-a1-pronoun-reflexive-feel (erroneous, a1)

- Task: How do you feel when you wake up early?
- Answer: Eu sinto bem quando acordo cedo.
- Reference: `Eu sinto` -> Eu me sinto [pronoun]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `Eu sinto`

### h-a2-preposition-bicycle (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho em bicicleta.
- Reference: `em bicicleta` -> de bicicleta [article_preposition]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `em bicicleta`

### h-a2-not-brazilian-fridge (erroneous, a2)

- Task: Where is the milk?
- Answer: O leite está no frigorífico.
- Reference: `frigorífico` -> geladeira [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `frigorífico`

### h-b1-verb-form-future-subjunctive-querer (erroneous, b1)

- Task: Will you help me?
- Answer: Se você querer, eu ajudo você a mudar.
- Reference: `Se você querer` -> Se você quiser [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ajudo` -> `ajuda` [verb_form] The verb 'ajudar' requires the pronoun 'você' to be attached after it when used as a transitive verb with an object.
- Missed: `Se você querer`

### h-b1-verb-form-passive-agreement (erroneous, b1)

- Task: Describe what was done to the room.
- Answer: A sala foi pintada e as janelas foi lavadas.
- Reference: `foi lavadas` -> foram lavadas [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `janelas` -> `foram` [verb_form] The verb 'foram' must agree with the plural subject 'janelas'.
- Missed: `foi lavadas`

### h-a1-off-topic-music (off_topic, a1)

- Task: What is your name?
- Answer: Eu gosto de música brasileira e toco violão.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `gosto` -> `chamo-me` [content_mismatch] The verb 'gostar' is not appropriate for answering a question about one's name.

### h-b1-off-topic-rain (off_topic, b1)

- Task: Do you think cities need more bicycle lanes?
- Answer: Ontem choveu muito e eu fiquei em casa.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Ontem choveu muito e eu fiquei em casa.` -> `Sim, acho que as cidades precisam de mais faixas para bicicletas.` [content_mismatch] The answer does not address the question about whether cities need more bicycle lanes.
