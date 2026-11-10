# Grading review sample

The same cases for every run, for rating the explanations by hand.

## gemma3n:e2b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [spelling] The word needs an accent on the i: escritório.
- Missed: `uma escritório`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] Beans is not a food.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `eu` [verb_form] The pronoun 'nós' is incorrect here. The verb 'almoçamos' needs to agree with the subject 'eu'.
- Missed: `volti`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [word_choice] The word 'autocarro' is not commonly used in Brazil. 'Ônibus' is more common.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `há cinco anos` -> `sou morador(a) de São Paulo há cinco anos` [word_order] The correct expression is 'sou morador(a) de São Paulo há cinco anos'.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ganhasse` -> `ganhassei` [verb_form] The verb form is incorrect; it should be 'ganhassei' (past subjunctive) to express a hypothetical situation.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma3n:e2b / few_shot_six

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [spelling] The word needs an accent on the i: escritório.
- Missed: `uma escritório`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `eu` [pronoun] The pronoun should be 'eu' because the subject is 'eu'.
- Missed: `volti`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `autocarro`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `há cinco anos` -> `sou morador há cinco anos` [word_order] The correct expression is 'sou morador há cinco anos'.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The verb should be in the subjunctive mood, but the form is incorrect.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma3n:e2b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de` -> `` [article_preposition] The preposition 'de' is unnecessary in this sentence.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `escritório` -> `empresa` [word_choice] The word 'escritório' is not the best choice for 'office'. 'Escritório' means 'office' but it is more formal. 'Empresa' or 'condomínio' are better options.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] Beans is an English word, not a food item commonly eaten in Brazil.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós` -> `nós comemos` [agreement] The pronoun 'nós' (we) is used with the verb 'almoçamos' (we ate) in the third person plural, which is incorrect. The correct form is 'nós comemos' (we ate).
- Missed: `volti`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] The word 'autocarro' is used in European Portuguese, not Brazilian Portuguese.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=false answers_the_question=false
- Found (changes nothing): `moro` -> `moro` [verb_form] The verb 'morar' (to live) needs to agree with the subject 'eu' (I) in the present tense. The correct form is 'moro'.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The verb 'ganhar' needs to be conjugated to the subjunctive mood in the conditional sentence.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `a ele` [article_preposition] The pronoun 'lhe' is not needed in this sentence; 'a ele' is more natural.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `fiz` -> `cometi` [verb_form] The verb 'fazer' (to do/make) is used for actions, but in this context, it should be used with the verb 'cometer' (to commit/make a mistake).

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `irmã` -> `irmã` [agreement] The word 'irmã' is correct.

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `doente` -> `doente` [word_choice] The adjective 'doente' is correct, but the sentence could be improved for clarity.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `convidei-a` -> `convidei-a` [agreement] The pronoun 'a' is unnecessary after 'convidei' and the verb 'invitar' is in the imperative mood.

## gemma3n:e4b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `escritório` -> `um escritório` [article_preposition] The word needs an article before it.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `nós almoçamos` -> `nós almoçamos` [verb_form] The verb 'almoçar' does not agree with the pronoun 'nós'.
- Missed: `volti`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `autocarro`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The correct form is 'ganhasse'.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `ele` [pronoun] 'lhe' is too formal; 'ele' is more appropriate.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma3n:e4b / few_shot_six

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `em uma` -> `na` [article_preposition] 'em' and 'uma' contract to 'na'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `autocarro`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The correct form is 'ganhasse'.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `a ele` [pronoun] 'a ele' is more common in Brazilian Portuguese.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma3n:e4b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de maiô` -> `em` [article_preposition] The preposition 'de' is not needed here; 'em' is more appropriate.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `escritório` -> `no` [article_preposition] The word 'escritório' is correct, but 'no' is missing before it.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word 'beans' is not commonly used in Brazilian Portuguese; 'feijão' is the correct term.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `nós almoçamos` -> `nós almoçamos` [verb_form] The verb 'almoçar' is not used with the pronoun 'nós'; 'nós almoçamos' is grammatically incorrect.
- Missed: `volti`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para o trabalho` -> `ao trabalho` [article_preposition] The preposition 'para' is not needed here; 'a' is more appropriate.
- Missed: `autocarro`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eles estão comendo espaguete.` [content_mismatch] The answer describes where the speaker lives, not the meal.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The correct form of the verb 'ganhar' in the conditional tense is 'ganhasse'.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `ele` [pronoun] The pronoun 'lhe' is too formal for speaking to a brother; 'ele' is more appropriate.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `um erro` -> `cometi um erro` [word_choice] The phrase 'fazer um erro' is not incorrect, but 'cometi um erro' is more natural in this context.

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `uma irmã` -> `uma irmã` [agreement] The word 'irmã' is feminine, so it needs a feminine article 'uma'.

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `doente` -> `como está` [word_choice] While not strictly incorrect, 'doente' implies a more serious illness than simply 'how is your brother'.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `convidei-a` -> `convidei ela` [pronoun] The pronoun 'a' is unnecessary and makes the sentence sound overly formal in Brazilian Portuguese.

## gemma4:12b-it-qat / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma escritório` -> `um escritório` [agreement] The word 'escritório' is masculine, so it requires the feminine article 'uma' to be changed to 'um'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=false answers_the_question=true
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The past tense of 'voltar' is 'voltei'.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] 'Autocarro' is the term used in Portugal; in Brazil, 'ônibus' is used.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma4:12b-it-qat / few_shot_six

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma` -> `um` [agreement] The word 'escritório' is masculine, so the article must be 'um'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=false answers_the_question=true
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The past tense of 'voltar' for the first person is 'voltei'.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] 'Autocarro' is the term used in Portugal; 'ônibus' is the standard term in Brazil.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma4:12b-it-qat / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A Marina` -> `Marina` [article_preposition] In Portuguese, you generally do not use a definite article before a person's name.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma` -> `um` [agreement] The word 'escritório' is masculine, so the article must be masculine.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No almoço` -> `No almoço` [article_preposition] The preposition 'no' (em + o) is used for location; 'no' (na) should be used for time periods like lunch.
- Found (matched): `beans` -> `feijão` [word_choice] The word 'beans' is English and should be translated to Portuguese.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The past tense of the verb 'voltar' is 'voltei'.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] The word 'autocarro' is used in European Portuguese; in Brazil, 'ônibus' is the standard term.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eles estão comendo espaguete e bebendo vinho tinto.` [content_mismatch] The answer describes where the speaker lives instead of describing the meal in the scene.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `disse para ele` [pronoun] In Brazilian Portuguese, 'te' is the standard pronoun for 'you' (informal) in this context, or 'disse para ele' should be used.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma4:12b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma escritório` -> `um escritório` [agreement] The word 'escritório' is masculine, so it requires the feminine article 'uma' to be replaced by 'um'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word for beans in Portuguese is 'feijão'.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The past tense of the verb 'voltar' is 'voltei'.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] 'Autocarro' is the term used in European Portuguese; 'ônibus' is the standard Brazilian Portuguese term.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma4:12b / few_shot_six

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma` -> `um` [agreement] The word 'escritório' is masculine, so it requires the feminine article 'um'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word for beans in Portuguese is 'feijão'.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The past tense of 'voltar' is 'voltei'.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] 'Ônibus' is the standard Brazilian Portuguese word for bus.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma4:12b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma escritório` -> `um escritório` [agreement] The word 'escritório' is masculine, so it requires the feminine article 'uma' to be changed to 'um'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word for beans in Portuguese is 'feijão'.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [spelling] The past tense of the verb 'voltar' is 'voltei'.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] The word 'autocarro' is used in European Portuguese; in Brazil, 'ônibus' is the standard term.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eles estão comendo espaguete e bebendo vinho tinto.` [content_mismatch] The answer describes where the person lives instead of describing the meal in the scene.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

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

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `convidei-a` -> `a convidei` [word_order] In Brazilian Portuguese, it is much more natural to place the pronoun before the verb (clitic placement) in this context.

## gemma4:e2b-it-qat / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma escritório` -> `um escritório` [agreement] 'Escritório' is masculine, so the article should be 'um'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No almoço` -> `No almoço eu como` [word_order] The correct structure is 'No almoço eu como' or 'Para almoço eu como'.
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The correct past tense of 'voltar' is 'voltei'.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [word_choice] The correct Portuguese word for bus is 'autocarro' (European Portuguese), but in Brazilian Portuguese, it is 'ônibus'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eles comem espaguete e bebem vinho tinto.` [content_mismatch] The answer does not describe the meal.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `convidei-a` -> `convidei-a` [verb_form] The past tense of 'convidar' is 'convidei', and the pronoun should be 'a' (object pronoun).

## gemma4:e2b-it-qat / few_shot_six

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `maiô` -> `maiô` [spelling] The correct spelling is 'maiô'.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `em uma escritório` -> `em um escritório` [agreement] The preposition 'em' requires the feminine article 'uma' to agree with 'escritório'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Não` [word_order] The correct negation is 'Não'.
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The correct past tense of 'voltar' is 'voltei'.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `de autocarro` -> `de autocarro` [word_order] The correct preposition is 'de autocarro' or 'de autocarro' is acceptable, but 'de autocarro' is more common for transport. However, the main issue is the lack of an article or a slightly awkward phrasing. A more natural phrasing would be 'de autocarro' or 'de autocarro'. Given the context, the structure is acceptable, but I will check for a more precise error. The structure is fine, but 'de autocarro' is the standard way. I will check for a more definite error. The sentence is grammatically acceptable for A2. No mandatory correction found based on the rules.
- Missed: `autocarro`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

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

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `em uma escritório` -> `em um escritório` [article_preposition] The preposition 'em' is incorrect here; it should be 'em um' because 'escritório' is masculine.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Eu` [word_order] The correct way to start this sentence is with a subject pronoun or a simple statement.
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The correct past tense of 'voltar' (to go back) is 'voltei'.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `de autocarro` -> `de autocarro` [word_choice] The correct preposition for public transport is 'de autocarro' or 'de elétrico', but 'de autocarro' is more common in European Portuguese. In Brazilian Portuguese, 'de ônibus' is more common, but 'de autocarro' is not strictly wrong.
- Missed: `autocarro`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eles estão comendo espaguete e bebendo vinho tinto.` [content_mismatch] The answer does not describe the meal mentioned in the task.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The conditional tense should be used here.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu lhe disse` -> `Eu disse a ele` [pronoun] The pronoun 'lhe' is used incorrectly here; 'te' or 'a ele' would be more natural in this context.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Ontem` -> `Hoje` [word_choice] The correct word for 'yesterday' in this context is 'Hoje' or a more specific time reference, but 'Ontem' is grammatically acceptable.
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `doente` -> `doente` [word_choice] The correct adjective for 'sick' is 'doente' or 'doente' is correct, but the context implies a state, and 'doente' is acceptable here.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `convidei-a` -> `convidei a` [word_order] The past tense of 'convidar' is 'convidei' (I invited), and the pronoun should be 'a' (to her) without the hyphen.

## gemma4:e2b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `maiô` -> `maiô` [word_choice] The word for swimsuit is 'maiô'.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma escritório` -> `um escritório` [article_preposition] The article 'uma' is incorrect before a masculine noun like 'escritório'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No almoço eu` -> `Eu almoço` [word_order] The word order is incorrect for a simple statement.
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The correct past tense conjugation for 'eu' is 'voltei'.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `autocarro`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro` -> `Eles estão com` [word_choice] The sentence does not describe the meal. The verb tense is also incorrect for the context.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `lhe` -> `a ele` [word_choice] The correct pronoun for 'to him/brother' in this context is 'a ele' or simply omitting it if the context is clear, but 'lhe' is often used incorrectly here. For a direct speech, 'ele' or 'a ele' is more natural. However, the main issue is the structure. A more natural phrasing would be 'Eu disse a ele que não me esperasse'.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `uma irmã` -> `algumas irmãs` [agreement] The sentence implies the speaker has only one sister, but the context doesn't specify the number of siblings. However, the main issue is the lack of plural agreement if the intent was to say 'I have siblings'. Since the task is just 'Do you have siblings?', the answer is grammatically acceptable but contextually incomplete. Given the constraints, no clear grammatical error is flagged here.

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=false

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma4:e2b / few_shot_six

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `maiô` -> `maiô` [spelling] The word for swimsuit is 'maiô'.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `em uma escritório` -> `num escritório` [agreement] The preposition 'em' is correct, but 'escritório' is masculine, so it should be 'num escritório'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Eu` [word_choice] The correct word for 'I' is eu.
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The correct past tense for 'eu' is 'voltei'.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `para o trabalho` -> `de autocarro` [word_choice] The correct preposition is 'para o trabalho' or 'de autocarro'. 'para' is acceptable but 'de' is more natural here. However, the main issue is the missing article before 'trabalho' if it's general, but the main error is the missing article before 'trabalho' if it's a specific place. Given the context, 'para o trabalho' is acceptable, but let's check for a more obvious error. The structure is fine, but 'de autocarro' is more idiomatic. Let's re-evaluate. The structure is acceptable for A2. No mistake found based on strict rules.
- Missed: `autocarro`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro` -> `Eles estão com` [verb_form] The verb tense is incorrect for describing what people are eating. It should be present tense or a description of the meal.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `Ontem` -> `Ontem` [word_order] The correct way to say 'yesterday' is 'ontem'.
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `uma irmã` -> `uma irmã` [agreement] The sentence structure implies a general statement, but the context is about having siblings in general. If the intent is to state the number, it's fine, but the context is missing. However, based on the rules, this is not a clear grammatical error in this context. Re-evaluating: the sentence is grammatically correct for A1. No mistake reported.

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma4:e2b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `A Marina está de` -> `Marina está usando` [word_choice] The correct way to describe what someone is wearing is by saying they are wearing something.
- Found (changes nothing): `maiô` -> `maiô` [word_choice] The correct word for swimsuit is 'maiô' or 'biquíni', but the structure is wrong.
- Found (changes nothing): `sandálias` -> `sandálias` [spelling] The correct spelling for sandals is 'sandálias'.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (matched): `em uma escritório` -> `em um escritório` [article_preposition] The correct preposition for a place is 'em um escritório' and 'escritório' is masculine.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `No almoço eu` -> `Eu almoço` [word_order] The word order is incorrect for a simple statement.
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `eu visitei` -> `eu visitei` [verb_form] The verb 'visitei' is in the simple past tense, but the context implies a completed action in the past, and the structure is slightly awkward for a simple past narrative.
- Found (matched): `eu volti` -> `eu voltei` [verb_form] The verb 'volti' is incorrect; the correct past tense for 'voltar' (to return) is 'voltei'.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Eu vou para o trabalho` -> `Eu vou para o trabalho` [article_preposition] The correct way to express this is 'Eu vou para o trabalho' or 'Eu vou para o trabalho' without the article 'o' in this context.
- Missed: `autocarro`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `They are eating spaghetti and drinking red wine.` [content_mismatch] This sentence does not describe the meal.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Se eu ganhasse na loteria` -> `Se eu ganhasse na loteria` [verb_form] The conditional tense is incorrect for this type of hypothetical situation.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Eu lhe disse` -> `Eu disse` [word_choice] The correct way to say this in this context is 'Eu disse' or 'Eu lhe disse' is too formal/old-fashioned.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `Ontem` -> `ontem` [spelling] The correct word for 'yesterday' in this context is 'ontem'.
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `Sim` -> `Sim, eu tenho` [word_choice] The correct affirmative answer in this context is usually 'Sim, eu tenho...' or simply 'Sim, tenho...', but the structure is slightly unnatural for a simple yes/no question.

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (changes nothing): `doente` -> `doente` [word_choice] The correct adjective for illness is 'doente' in this context, but the sentence structure is slightly unnatural for a simple status update.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=false answers_the_question=true
- Found (false positive): `convidei-a` -> `convidei-la` [verb_form] The correct past tense conjugation for 'convidar' in this context is 'convidei-la' or simply 'convidei', and the pronoun placement is incorrect.

## gemma4:e4b-it-qat / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma escritório` -> `um escritório` [agreement] The noun 'escritório' is masculine, so the article must be 'um'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [word_choice] In Brazilian Portuguese, the common word for bus is 'ônibus'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma4:e4b-it-qat / few_shot_six

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma` -> `um` [agreement] 'Escritório' is a masculine noun, so the article should be 'um'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word 'beans' is in English and should be translated to Portuguese.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [word_choice] In Brazilian Portuguese, the common word for bus is 'ônibus'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma4:e4b-it-qat / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma` -> `um` [agreement] The noun 'escritório' is masculine, so the article must be masculine.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word 'beans' should be translated to Portuguese.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [word_choice] The word for bus in Brazilian Portuguese is 'ônibus'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eles estão comendo espaguete e bebendo vinho tinto.` [content_mismatch] The answer does not describe the meal as requested by the task.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma4:e4b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `uma escritório` -> `um escritório` [agreement] 'Escritório' is a masculine noun, so it requires 'um'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word is English and should be translated to Portuguese.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós almoçamos` -> `almoçamos` [pronoun] The subject pronoun 'nós' is redundant when the verb conjugation already implies it.
- Missed: `volti`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] This word is more common in European Portuguese; 'ônibus' is the standard term in Brazilian Portuguese.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro` -> `Eles comem` [content_mismatch] The answer does not describe the meal.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma4:e4b / few_shot_six

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma escritório` -> `um escritório` [agreement] 'Escritório' is a masculine noun, so it requires 'um'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] This is an English word and should be translated to Portuguese.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' in the past tense for 'eu' is 'voltei'.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] This word is more common in European Portuguese; 'ônibus' is the standard Brazilian term.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro` -> `(no answer)` [content_mismatch] The answer does not describe the meal.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## gemma4:e4b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A` -> `` [article_preposition] The definite article 'A' is not needed before the name 'Marina' in this context.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma escritório` -> `um escritório` [agreement] Escritório is a masculine noun, so it requires the masculine article.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word 'beans' is English and should be translated to Portuguese.
- Found (false positive): `frango` -> `o frango` [article_preposition] The word 'frango' is correct, but it is better to use the definite article 'o' before it in this context.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `nós almoçamos` -> `almoçamos` [pronoun] The subject pronoun 'nós' is redundant when the verb conjugation already implies the first-person plural.
- Missed: `volti`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [word_choice] The common word for bus in Brazilian Portuguese is 'ônibus'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eles estão comendo espaguete e bebendo vinho tinto.` [content_mismatch] The answer does not describe the meal as requested by the task.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe disse` -> `disse` [word_choice] The indirect object pronoun 'lhe' is not needed here because the direct object is 'que não me esperasse'.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## granite4.2:3b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `uma escritório`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `autocarro`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## granite4.2:3b / few_shot_six

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `uma escritório`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `autocarro`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## granite4.2:3b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `uma escritório`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `volti`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `autocarro`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## ministral-3:14b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de maiô` -> `usando um maiô` [article_preposition] The correct preposition for clothing is *usando* or *com* (not *de*).
- Found (false positive): `chapéu de palha` -> `boné de palha` [word_choice] For a beach hat, *boné* or *chapéu de sol* is more natural than *chapéu de palha* (straw hat).

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [article_preposition] The word needs a contraction with 'em' in spoken Brazilian Portuguese.
- Found (matched): `em uma escritório` -> `num escritório` [article_preposition] The correct contraction for 'em + uma' is 'num' or 'numa' (feminine).

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The correct past tense of 'voltar' is 'voltei'.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [word_choice] In Brazilian Portuguese, the word for bus is 'ônibus'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `-a` -> `a` [pronoun] The clitic pronoun for 'her' in this context is **-lhe** (indirect object) or **-a** (direct object), but the correct form here is **-a** (direct object) without a hyphen.

## ministral-3:14b / few_shot_six

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [spelling] The word needs an accent on the o: escritório.
- Missed: `uma escritório`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb must agree with 'eu' in the past tense: voltei.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] The word is European Portuguese; in Brazilian Portuguese, use 'ônibus'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ganhasse` -> `ganharia` [verb_form] The conditional form should be 'ganhasse' (subjunctive) for hypotheticals, but the correct conditional form for 'ganhar' is 'ganharia'.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

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

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [spelling] The correct spelling is 'escritório' with an accent on the 'o'.
- Found (matched): `em uma escritório` -> `num escritório` [article_preposition] The preposition 'em' requires the contraction 'num' before a masculine noun starting with a vowel.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word 'beans' is in English; the correct Brazilian Portuguese word is 'feijão'.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The correct past tense of *voltar* for 'I returned' is *voltei* (not *volti*).

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] The word for 'bus' in Brazilian Portuguese is *ônibus*, not *autocarro* (which is used in European Portuguese).

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The conditional tense should be used for hypothetical situations in Portuguese, but the verb form here is subjunctive (imperfect subjunctive).

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `me esperasse` -> `esperasse` [verb_form] The subjunctive form is incorrect for a negative command in the past.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `um erro` -> `um erro / um equívoco / um deslize` [word_choice] The correct expression is *um erro* (a mistake) in the context of a test, but the verb *fazer* does not collocate with *erro* here. The correct collocation is *cometer um erro*.
- Found (false positive): `na prova` -> `na minha prova` [word_order] While not strictly incorrect, *na prova* is slightly less natural than *na minha prova* or simply *na prova de matemática* (without repetition). However, this is not a grammatical error but a stylistic choice.

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `uma irmã` -> `irmã` [article_preposition] At A1 level, the indefinite article *uma* is not typically used with *irmã* when answering 'Do you have siblings?' in a simple yes/no context.

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `doente` -> `doente**m**` [agreement] The adjective 'doente' should agree in gender with the subject 'irmão' (masculine).

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

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `escritório` -> `um escritório` [agreement] The noun 'escritório' is masculine, so it requires the article 'um' instead of 'uma'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijões` [word_choice] The word 'beans' is in English and should be translated to Portuguese.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' must be conjugated in the past tense to match the other verbs.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] The word 'autocarro' is European Portuguese; the Brazilian equivalent is 'ônibus'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `They are eating spaghetti and drinking red wine at a restaurant.` [content_mismatch] The answer does not describe the meal or the people eating it; it talks about where the speaker lives and their feelings about the city.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## qwen3.5:4b / few_shot_six

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Maiô` -> `maio` [spelling] The word 'maiô' is misspelled; it should be 'maio'.
- Found (false positive): `vermelho` -> `vermelha` [agreement] The adjective must agree in gender with 'maio' (feminine).

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `um` -> `um` [agreement] The article 'um' is masculine, but 'escritório' is neuter (or feminine in this context).
- Found (matched): `em uma escritório` -> `no` [article_preposition] The preposition 'em' requires a definite article before the noun.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' must be conjugated in the past tense: voltei.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] The word 'autocarro' is European Portuguese; the Brazilian equivalent is 'ônibus'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eles estão comendo espagueti e bebendo vinho tinto no restaurante.` [content_mismatch] The answer does not describe the meal or the people eating it as specified in the task and scene facts.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true

## qwen3.5:4b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `escritório` -> `uma` [agreement] The word 'escritório' is masculine, but it must agree with the feminine article 'um'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Em` [word_choice] The word 'no' is a false friend for English 'on'; the correct preposition is 'em'.
- Missed: `volti`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] The word 'autocarro' is European Portuguese; Brazilian Portuguese uses 'ônibus'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos` -> `They are eating spaghetti and drinking red wine.` [content_mismatch] This sentence does not describe the meal being eaten.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
