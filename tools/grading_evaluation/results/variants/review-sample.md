# Grading review sample

The same cases for every run, for rating the explanations by hand.

## gemma3:12b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [spelling] This word needs an accent mark.
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
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' needs to be in the pretérito perfeito tense here.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] This word is used in European Portuguese, in Brazilian Portuguese you should use ônibus.

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

## gemma3:12b / few_shot_in_system

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [spelling] The word needs an accent mark: escritório.
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
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' needs to be in the pretérito perfeito tense here.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [word_choice] In Brazil, 'autocarro' is not used; the correct word is 'ônibus'.

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
- Found (false positive): `lhe disse` -> `a ele disse` [pronoun] With 'Eu', the indirect object pronoun 'lhe' is not used; use 'a ele disse'.

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

## gemma3:12b / few_shot_mostly_correct

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `em uma escritório` -> `em um escritório` [article_preposition] 'escritório' is masculine, so it needs the article 'um', not 'uma'.

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
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' needs to be conjugated in the pretérito perfeito, so it should be 'voltei'.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] 'Autocarro' is European Portuguese for bus; in Brazilian Portuguese, you would say 'ônibus'.

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
- Found (false positive): `lhe disse` -> `disse a ele` [word_choice] With 'Eu', the indirect object pronoun 'lhe' is not commonly used in Brazil; 'Eu disse a ele' is more natural.

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

## gemma3:12b / few_shot_reversed

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [spelling] The word needs an accent on the r: escritório.
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
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' needs to be conjugated in the pretérito perfeito.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] 'Autocarro' is European Portuguese; in Brazilian Portuguese, you should use 'ônibus'.

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

## gemma3:12b / few_shot_six

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma escritório` -> `um escritório` [article_preposition] 'escritório' is masculine, so it needs the masculine article 'um'.

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
- Found (matched): `volti` -> `voltei` [verb_form] The verb must be conjugated in the pretérito perfeito: voltei.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] This word is used in European Portuguese, in Brazilian Portuguese you should use ônibus.

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

## gemma3:12b / one_shot_correct

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [agreement] The word should be feminine to agree with 'uma'.
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
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' needs to be in the pretérito perfeito tense here.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] This word is used in European Portuguese, not Brazilian Portuguese.

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

## gemma3:12b / one_shot_mistake

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [spelling] 'escritório' needs a cedilla under the 'c'.
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
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' needs to be conjugated in the preterite tense to match the context of the past.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] This word is used in European Portuguese, in Brazilian Portuguese you should use ônibus.

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

## gemma3:12b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `em uma` -> `em um` [article_preposition] The correct article to use before a feminine noun starting with a vowel is 'em uma'.

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
- Found (matched): `volti` -> `voltei` [verb_form] The verb needs to be in the pretérito perfeito tense to describe a completed action in the past.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] This word is used in European Portuguese, not Brazilian Portuguese.

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
- Found (false positive): `lhe` -> `a ele` [pronoun] The indirect object pronoun 'lhe' is not used in this context; 'a ele' is needed.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Ontem` -> `Amanhã` [word_choice] The correct word to use here is 'Amanhã'.
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

## gemma3:4b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
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
- Missed: `volti`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `autocarro` -> `autocarro` [spelling] The word needs an accent on the ‘o’: autocarro.
- Missed: `autocarro`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro` -> `Eles estão comendo` [content_mismatch] The answer should describe the meal, not the speaker's location.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The conditional form needs to be 'ganhasse'.

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

## gemma3:4b / few_shot_in_system

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `escritório` -> `escritório` [spelling] The word needs an accent on the i: escritório.
- Missed: `uma escritório`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No` -> `No` [spelling] The correct preposition is 'no' (with no accent mark).
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
- Found (changes nothing): `autocarro` -> `autocarro` [spelling] The word needs an accent on the ‘o’: autocarro.
- Missed: `autocarro`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu` -> `` [content_mismatch] This sentence is about the meal, not the speaker's location.

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

## gemma3:4b / few_shot_mostly_correct

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `em uma escritório` -> `em um escritório` [article_preposition] Needs the correct article: ‘um’ instead of ‘em uma’

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No` -> `no` [article_preposition] The correct preposition is 'no', but it needs a contraction with almoço: ‘no almoço’
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
- Found (changes nothing): `autocarro` -> `autocarro` [spelling] The word needs an accent on the o: autocarro.
- Missed: `autocarro`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro` -> `Eles estão comendo` [content_mismatch] This sentence is irrelevant to the task and contains a personal statement.

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

## gemma3:4b / few_shot_reversed

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A Marina` -> `` [article_preposition] The article 'A' is unnecessary here.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (matched): `em uma escritório` -> `em um escritório` [word_choice] The correct word is 'escritório' (without 'a').

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No almoço` -> `Para o almoço` [word_choice] This phrase is not natural in this context.
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `retornei` [pronoun] The correct reflexive pronoun is 'retornei'.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `autocarro` -> `autocarro` [spelling] The word needs an accent on the ‘o’: autocarro.
- Missed: `autocarro`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro` -> `Eles estão comendo` [word_choice] This sentence is irrelevant to the task and uses the wrong verb form.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ganhasse` -> `ganharia` [verb_form] The verb needs to be in the conditional: ganharia.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `a` [agreement] The correct pronoun to use here is 'a' because 'irmão' is masculine.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `na prova de matemática` -> `na prova matemática` [word_choice] The phrase needs a contraction: 'na prova matemática'.
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `não vai trabalhar hoje` -> `não vai trabalhar amanhã` [word_choice] The correct expression is 'não vai trabalhar amanhã'.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `convidei-a` -> `convidei-la` [pronoun] The correct reflexive pronoun is 'a' not 'i'.

## gemma3:4b / few_shot_six

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
- Flags: written_in_portuguese=true answers_the_question=false
- Found (changes nothing): `autocarro` -> `autocarro` [spelling] The word needs an accent on the ‘o’: autocarro.
- Missed: `autocarro`

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu` -> `Eles` [content_mismatch] The answer should describe the meal, not the speaker's location.

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

## gemma3:4b / one_shot_correct

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `em uma escritório` -> `em um escritório` [word_choice] ‘escritório’ needs to be plural ‘escrítorios’ or ‘um escritório’

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
- Found (matched): `autocarro` -> `ônibus` [word_choice] ‘Autocarro’ is European Portuguese for ‘bus’; in Brazil, it’s ‘ônibus’.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu` -> `Eles` [pronoun] This refers to the speaker, not the meal.

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
- Found (false positive): `ontem` -> `na` [word_choice] The correct word to use here is 'na', indicating the day of the week.
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

## gemma3:4b / one_shot_mistake

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A Marina está` -> `Marina usa` [ser_estar] The verb ‘estar’ is not needed here, ‘Marina usa’ or ‘Marina tem’ would be more appropriate.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `em uma escritório` -> `em um escritório` [agreement] The correct word is ‘um’ (a) before ‘escritório’.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No` -> `No` [spelling] The correct preposition is ‘no’ (with no accent mark).
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `visitei` -> `visitei` [verb_form] The verb needs to be in the present tense: visitei.
- Missed: `volti`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [word_choice] 'Autocarro' is European Portuguese for bus; in Brazil, it's 'ônibus'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro` -> `Eles estão` [verb_form] This sentence is irrelevant to the task and uses the wrong verb form.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The correct form is 'ganhasse' (conditional), not 'ganhasse'.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `a` [pronoun] The correct pronoun to use here is 'a' or 'ao', not 'lhe'.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `fiz` -> `errei` [verb_form] The verb needs to be in the first person singular, so 'fiz' should be 'errei'.

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está` -> `é` [ser_estar] The correct verb to use here is 'é' because it's a statement about his state.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `convidei-a` -> `eu a convidei` [pronoun] The verb 'convidar' needs the subject pronoun 'eu': eu a convidei.

## gemma3:4b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A Marina está` -> `Marina está` [article_preposition] The use of 'A' before Marina is incorrect.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `em uma escritório` -> `uma escritório` [word_choice] The correct word is 'uma escritório'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Eu` [article_preposition] ‘No’ is not the correct way to start this sentence.
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Finalmente` [article_preposition] ‘No’ is not the correct way to start this sentence in Brazilian Portuguese.
- Missed: `volti`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [word_choice] ‘Autocarro’ is European Portuguese for bus; in Brazil, it’s ‘ônibus’.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Eu moro` -> `Eles` [word_choice] This phrase means 'I live' and is irrelevant to the task.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The conditional form of 'ganhar' (to win) needs the subjunctive mood after 'se'.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `a ele` [word_choice] ‘Lhe’ is overly formal; ‘a ele’ is more appropriate in this context.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Ontem` -> `Na` [word_choice] The question asks about 'your test', so 'yesterday' is not appropriate.
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `tenho` -> `tenho` [verb_form] The verb 'ter' needs to be conjugated in the present tense for 'I'.

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está` -> `é` [verb_form] The verb 'estar' is used incorrectly; 'está' is the third person singular form of 'estar', but the subject is 'irmão' (brother), which is masculine singular.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `convidei-a` -> `convidei-a para` [verb_form] The past perfect tense is not needed here.

## qwen3.5:latest / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está` -> `` [word_choice] To describe what someone is wearing, the preposition 'de' is used without a verb.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma` -> `um` [agreement] The article 'um' must agree in gender with the masculine noun 'escritório'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word is an English loanword; the correct Portuguese term is 'feijão'.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `avós` -> `avôs` [spelling] The noun 'avô' is masculine, so the plural must be 'avôs', not 'avós'.
- Missed: `volti`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] The word 'autocarro' is European Portuguese; in Brazil, the bus is called 'ônibus'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `moro` -> `Eles comem` [content_mismatch] The sentence does not describe the meal being eaten as requested.
- Found (false positive): `São Paulo` -> `espaguete` [content_mismatch] The sentence does not describe the meal being eaten as requested.
- Found (false positive): `cinco anos` -> `vinho tinto` [content_mismatch] The sentence does not describe the meal being eaten as requested.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `loteria` -> `loteria` [spelling] The word needs an accent on the i: loteria.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `para ele` [not_brazilian] In spoken Brazilian Portuguese, the indirect object pronoun 'lhe' is often replaced by 'para ele' or omitted in this context to avoid ambiguity with the direct object 'me'. While not strictly ungrammatical in formal writing, it is considered a European Portuguese influence or overly formal for A2/B2 learners describing casual speech. However, a more common error pattern here is actually the use of 'lhe' when 'para ele' is preferred in Brazil for clarity, but strictly speaking, 'lhe' is not a *mistake* in standard grammar unless we enforce spoken norms. Let's re-evaluate based on strict Brazilian norms: 'Eu disse para ele que...' is much more common than 'Eu lhe disse...'. But is it an error? Actually, the bigger issue might be the subjunctive 'esperasse' which is correct for past tense indirect speech. Wait, looking at the previous examples, the model seems to flag Europeanisms or agreement errors. Is 'lhe' considered a mistake here? In Brazil, 'lhe' is often avoided in favor of 'para ele' or rephrasing. Let's look closer. Actually, there is no hard grammatical error here for B2 level, but 'lhe' can be seen as formal/European. However, the most likely intended correction for a learner is often about word choice or clarity. Let's reconsider if there are NO mistakes. In Brazil, saying 'Eu lhe disse' is acceptable in writing and formal speech. Is it a mistake? Maybe not. What about 'esperasse'? Correct subjunctive. 'ouviu'? Correct past tense. Maybe the answer has no mistakes? But usually these tasks have one. Let's look at 'lhe' again. In Brazil, using 'lhe' as an indirect object pronoun is standard in formal contexts but often replaced by 'para ele' in informal. The prompt doesn't specify register. However, if I must find a mistake, 'lhe' is the only candidate for being 'not_brazilian' (European preference) or just less common. But wait, is there a simpler error? No. Let's assume 'lhe' is acceptable. Is there another error? 'Minhas irmãs são muito alta' in the previous example was agreement. Here everything seems correct grammatically. Perhaps the system expects 'para ele'. Let's check the rules: 'not_brazilian: a form or construction used only in European Portuguese'. 'Lhe' is not *only* European, but its usage frequency differs. Actually, I might be overthinking. Let's look at the sentence again. 'Eu lhe disse que não me esperasse'. This is perfectly valid Portuguese. Maybe there are no mistakes? But the previous examples always had one. Let's reconsider 'lhe'. In Brazil, it is very common to say 'Eu disse para ele' instead of 'Eu lhe disse'. Using 'lhe' might be flagged as 'not_brazilian' in the context of learner correction focusing on spoken norms or strong Brazilian preference. I will flag 'lhe' as not_brazilian because in Brazil, the construction with 'para + pronoun/name' is strongly preferred over 'lhe' for clarity and informality, whereas 'lhe' is more formal/European style in this specific indirect object context.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Ontem` -> `(none)` [content_mismatch] The task asks about the past event on 'your test', but the answer describes a different event ('yesterday') that contradicts the implied context of answering a specific question.
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

## qwen3.5:latest / few_shot_in_system

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma` -> `um` [agreement] The word 'escritório' is masculine, so the article must be 'um', not 'uma'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word 'beans' is English; the Portuguese word is 'feijão'.

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
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] The word 'autocarro' is European Portuguese; in Brazil, the bus is called 'ônibus'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `` [content_mismatch] The answer describes the speaker's life in a city instead of describing the meal being eaten by two people at a restaurant.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

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

## qwen3.5:latest / few_shot_mostly_correct

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma` -> `um` [agreement] The article 'um' must agree in gender with the masculine noun 'escritório'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word is an English false friend; the Portuguese word is feijão.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei-me` [pronoun] The verb 'voltar' is reflexive in this context ('to return home'), so the reflexive pronoun 'me' is needed.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] Autocarro is European Portuguese; in Brazil, the word for bus is ônibus.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `moro` -> `They are eating spaghetti` [content_mismatch] The answer describes where the speaker lives, but the task asks to describe a meal being eaten.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The verb 'ganhar' in the imperfect subjunctive is formed with 'ganhasse', not 'ganhasse'.
- Found (quote not in the answer): `viaria` -> `viasse` [verb_form] The verb 'viajar' in the imperfect subjunctive is formed with 'viasse', not 'viaria'.

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

## qwen3.5:latest / few_shot_reversed

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A` -> `` [article_preposition] The article 'A' is not used before a proper name like 'Marina'.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma` -> `um` [agreement] The article 'um' must agree in gender with the masculine noun 'escritório'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word 'beans' is English; the Portuguese equivalent is 'feijão'.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `avós` -> `avôs` [agreement] The noun 'avô' is masculine, so the plural must be 'avôs'.
- Missed: `volti`

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] The word 'autocarro' is European Portuguese; in Brazil, the bus is called 'ônibus'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `moro` -> `They are eating` [content_mismatch] The verb 'morar' does not match the task of describing a meal.
- Found (false positive): `São Paulo` -> `at a restaurant` [content_mismatch] This location contradicts the scene facts about people at a restaurant eating spaghetti and drinking wine.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `loteria` -> `loteria` [spelling] The word needs an accent on the first 'o': loteria.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `a ele` [word_choice] The indirect object pronoun 'lhe' is not used with the verb 'dizer' in this context; 'disse a ele' or just 'disse que' is preferred.

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

## qwen3.5:latest / few_shot_six

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `está de` -> `tem` [not_brazilian] The construction 'estar de' is used for clothing in European Portuguese; Brazilian Portuguese uses 'ter' or just the noun.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma` -> `um` [agreement] The word 'escritório' is masculine, so the article must be 'um'.
- Found (duplicate): `em uma escritório` -> `num` [article_preposition] 'em' and 'um' contract to 'num'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word is an English false friend; the Portuguese word is 'feijão'.

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
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] 'Autocarro' is European Portuguese; use 'ônibus' in Brazilian Portuguese.

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

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `o` [not_brazilian] 'Lhe' is European Portuguese; use 'o' in Brazilian Portuguese.

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

## qwen3.5:latest / one_shot_correct

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma` -> `um` [agreement] The noun 'escritório' is masculine, so the article must be 'um', not 'uma'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word 'beans' is English; the Portuguese word should be 'feijão'.

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
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] The word 'autocarro' is European Portuguese; in Brazilian Portuguese, the correct term is 'ônibus'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `` [content_mismatch] The answer describes where the speaker lives and their opinion about the city, but fails to describe the meal as requested by the task.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `` [pronoun] The indirect object pronoun 'lhe' is not used with the verb 'dizer' when followed by a 'que' clause; it should be omitted.
- Found (false positive): `não me esperasse` -> `não esperar` [not_brazilian] The imperfect subjunctive form 'esperasse' is European Portuguese; in Brazilian Portuguese, the future subjunctive 'esperar' is used after 'que'.

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

## qwen3.5:latest / one_shot_mistake

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma` -> `um` [agreement] The word 'escritório' is masculine, so it requires the article 'um', not 'uma'.
- Found (changes nothing): `escritório` -> `escritório` [spelling] The word needs an accent on the i: escritório.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word 'beans' is English; the Portuguese word is 'feijão'.

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
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] The word for bus in Brazilian Portuguese is ônibus; autocarro is European Portuguese.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Eles comem espaguete e bebem vinho tinto.` [content_mismatch] The answer describes living in a city instead of describing the meal being eaten.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `a ele` [word_choice] The indirect object pronoun 'lhe' is not used with the verb 'dizer' in this context; 'disse a ele' or just 'disse que' is preferred.
- Found (false positive): `não me esperasse` -> `não me esperou` [verb_form] 'Não me esperasse' uses the imperfect subjunctive, but after 'que não' with a past tense verb like 'disse', the indicative past ('esperou') or present perfect ('esperou') is required to state a fact.

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

## qwen3.5:latest / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `uma` -> `um` [agreement] The noun 'escritório' is masculine, so the feminine article 'uma' is incorrect.
- Found (duplicate): `escritório` -> `escritórios` [spelling] In Brazilian Portuguese, the plural form 'escritórios' is required here because there are multiple offices in a city center.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word 'beans' is English; the Portuguese word for beans is 'feijão'.

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
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] The word 'autocarro' is European Portuguese; in Brazilian Portuguese, the correct term is 'ônibus'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro` -> `` [content_mismatch] The answer describes the speaker's residence instead of describing the meal as requested.
- Found (false positive): `São Paulo` -> `` [content_mismatch] The answer mentions a city instead of describing the food and drink from the scene.
- Found (false positive): `cinco anos` -> `` [content_mismatch] The answer gives a duration of residence instead of describing the meal.
- Found (false positive): `gosto muito da cidade` -> `` [content_mismatch] The answer expresses an opinion about the city instead of describing the meal.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `lhe` -> `` [word_choice] The indirect object pronoun 'lhe' is not used with the verb 'dizer' when referring to a person in this context; 'disse para o meu irmão' or just 'disse que' is preferred.
- Found (false positive): `não me esperasse` -> `não me esperava` [verb_form] The subjunctive form 'esperasse' is incorrect here because the main clause uses the past tense 'disse', requiring the imperfect indicative 'esperava' to describe a habitual or expected action in the past.

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

## qwen3:4b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `escritório` -> `escritórios` [agreement] The word 'escritório' is singular, but it needs to be plural because 'uma escritório' is incorrect.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No` -> `No` [article_preposition] The preposition 'no' is incorrect here; the correct phrase is 'No almoço' meaning 'At lunch' but the structure should be 'No almoço' without a preposition before it.
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' is conjugated incorrectly; it should be 'voltei' for past tense.

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
- Found (changes nothing): `moro` -> `moro` [verb_form] The verb should be in present continuous tense to describe what they are doing now.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The conditional tense should be 'ganhasse' for 'eu' in the past subjunctive, but the correct form is 'ganhasse' (with an accent).

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
- Found (false positive): `convidei-a` -> `convidei` [verb_form] The verb 'convite' is in the wrong tense; it should be 'convidar' in the past perfect tense.

## qwen3:4b / few_shot_in_system

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maiô` -> `banho` [word_choice] The word 'maiô' is a false friend; in Brazilian Portuguese, the correct term for a swimsuit is 'banho' or 'lingerie', but in this context, 'maiô' is used in Portuguese for a type of swimsuit, but it's not the standard term for a swimsuit in Brazil. However, in Brazilian Portuguese, 'maiô' is actually a common term for a swimsuit, so this might be correct. Wait, let me check: In Brazil, 'maiô' is a type of swimsuit, but the more common term for a swimsuit is 'banho' or 'lingerie'? No, actually, in Brazil, 'maiô' is a term used for a swimsuit, but it's not the standard term. Wait, I think in Brazil, the standard term for a swimsuit is 'banho' or 'lingerie'? No, I think 'maiô' is used in Brazil for a swimsuit. Let me think: In Brazil, 'maiô' is a type of swimsuit, but it's not the standard term. Wait, I'm confused. Let me check the example: In the example, they used 'sanduíche' for 'sandwich', which is correct in Brazil. So, 'maiô' is a term used in Brazil for a swimsuit? Actually, in Brazil, 'maiô' is a term for a swimsuit, but it's not the most common. However, the task says 

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `escritório` -> `escritórios` [agreement] The word 'escritório' is singular but the context requires a plural form for 'office' in this context.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `almoço` -> `como` [word_choice] The word 'almoço' means 'lunch' but in this context it should be the verb 'to eat' for the action of eating lunch.
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' is conjugated incorrectly; it should be 'voltei' for the past tense.

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
- Found (false positive): `Eu` -> `Eles` [pronoun] The sentence should be in third person plural to describe what the people are eating.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The conditional perfect tense requires 'ganhasse' to be conjugated with the past subjunctive, but here it's used in the indicative tense.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `lhe` -> `lhe` [pronoun] The pronoun 'lhe' is used for indirect object pronouns in the past tense, but in this context, the correct pronoun for indirect object in the past tense is 'lhe' for 'to him' when the verb is in the past tense.

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
- Found (false positive): `convidei-a` -> `convidei` [verb_form] The verb should be in the past perfect tense (pretérito perfeito composto) for 'convite' (to invite) in this context.

## qwen3:4b / few_shot_mostly_correct

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maiô` -> `sanduíche` [spelling] The word 'maiô' is a false friend; in Portuguese, 'maiô' refers to a type of swimsuit, but the correct term for a swimsuit is 'banho' or 'roupa de banho'. However, in Brazilian Portuguese, 'maiô' is commonly used for a one-piece swimsuit, so this might be acceptable. But the scene facts say Marina wears a red swimsuit, and in Brazilian Portuguese, 'maiô' is indeed used for a swimsuit. Wait, let me double-check. In Brazilian Portuguese, 'maiô' is a term for a one-piece swimsuit, so it's correct. However, the scene facts say 'a red swimsuit', and in Brazilian Portuguese, 'maiô' is used for that. But the answer says 'maiô vermelho', which is correct. Wait, but the task is to describe what Marina is wearing, and the scene facts say she wears a red swimsuit, a straw hat, and white sandals. In Brazilian Portuguese, 'maiô' is a common term for a swimsuit, so this might be correct. However, the answer uses 'maiô', which is correct in Brazilian Portuguese. Let me check the spelling. The word 'maiô' has an accent on the ô, but the answer has 'maiô' with an accent. Wait, the answer says 'maiô', which is correct. But the task is for A1 level, and the answer might be using a term that's not common. Wait, no, in Brazilian Portuguese, 'maiô' is used for a swimsuit. So maybe there's no mistake here. Let me think again. The scene facts say Marina wears a red swimsuit, and in Brazilian Portuguese, 'maiô' is a term for a swimsuit. So the answer is correct. But the answer uses 'maiô', which is correct. Wait, but the answer says 'A Marina está de maiô vermelho', which is correct. However, the task is for A1 level, and the word 'maiô' might be too advanced. But the task says Level: A1, and the answer uses 'maiô', which is a common term in Brazil. Wait, but the answer might have a mistake in the verb form. The verb 'estar de' is used for clothing, so 'A Marina está de maiô vermelho' is correct. So maybe there's no mistake here. Let me check the other parts. The answer says 'chapéu de palha' which is correct for straw hat, and 'sandálias brancas' which is correct for white sandals. So the only possible mistake is the spelling of 'maiô'. But in Portuguese, 'maiô' has an accent on the ô, and the answer has it with an accent. Wait, the answer says 'maiô', which is correct. So maybe there's no mistake. But the previous example had a mistake with 'sanduiche' needing an accent. In this case, 'maiô' is spelled correctly with an accent. So the answer is correct. Therefore, no mistakes. But wait, the task says Level: A1, and the word 'maiô' might not be taught at A1 level. However, the rules say: 

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [agreement] The word 'escritório' is singular, but the phrase 'uma escritório' uses the plural form of the article.
- Missed: `uma escritório`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No almoço` -> `Eu como` [word_order] The phrase 'No almoço' means 'At lunch' but the question asks for what you eat for lunch, so it should be 'Eu como' instead of 'No almoço'.
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' is conjugated incorrectly; it should be 'voltei' for the past tense.

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
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The conditional tense should be 'ganhasse' but the verb form is incorrect for the conditional mood.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `lhe` -> `lhe` [pronoun] The pronoun 'lhe' is used for indirect object pronouns in the past tense, but in this context, the correct pronoun for indirect object in the past tense is 'lhe' for 'to him' when the verb is in the past tense. However, in Brazilian Portuguese, the indirect object pronoun 'lhe' is used correctly here. Wait, let me double-check.

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
- Found (false positive): `convidei-a` -> `convidei` [verb_form] The verb 'convidar' requires the preterite tense for past actions, but 'convidei' is in the infinitive form.

## qwen3:4b / few_shot_reversed

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maiô` -> `banho` [word_choice] The word 'maiô' is used in Brazil for a swimsuit, but it's more common to say 'banho' or 'roupa de banho' for swimsuit in this context. However, 'maiô' is acceptable in Brazil for a one-piece swimsuit.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [agreement] The word 'escritório' is singular, but the context requires a plural form because 'uma escritório' is incorrect; it should be 'um escritório' for singular.
- Missed: `uma escritório`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `almoço` -> `meu almoço` [word_choice] The word 'almoço' means 'lunch' but in this context it's used as a noun for the meal, which is incorrect here. The correct phrase should be 'No meu almoço' or 'Para o almoço' to indicate what you eat for lunch.
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' is conjugated incorrectly; it should be 'voltei' for past tense.

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
- Found (false positive): `moro` -> `vivo` [verb_form] The verb 'morrer' is used for death, but here it should be 'vivo' for 'live'.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The conditional tense should be 'ganhasse' for 'eu' (first person singular), but the verb form is correct for the past subjunctive.

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
- Found (false positive): `convidei-a` -> `convidei` [verb_form] The verb 'convite' is in the wrong tense; it should be 'convidar' in the past perfect tense.

## qwen3:4b / few_shot_six

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `em uma` -> `na` [article_preposition] The preposition 'em' contracts with 'a' to 'na' before a definite article.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No` -> `No` [article_preposition] The preposition 'no' is incorrect; it should be 'No' for 'na' (in the) but the correct phrase is 'No almoço' meaning 'At lunch'.
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb must be conjugated correctly: voltei.

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
- Found (false positive): `moro` -> `They are eating spaghetti and drinking red wine` [content_mismatch] The verb must agree with 'eu': moro is correct for first person singular, but the sentence is about what they are eating, not where they live.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The verb must be in the conditional tense for the past subjunctive: ganhasse.

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
- Found (false positive): `convidei-a` -> `convidei` [verb_form] The verb 'convite' is in the wrong form; it should be 'convidei' for the past tense.

## qwen3:4b / one_shot_correct

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `A` -> `Marina` [article_preposition] The definite article 'a' is used for feminine nouns in Portuguese, but 'Marina' is a proper noun and doesn't require an article.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `escritório` -> `o escritório` [article_preposition] In Brazilian Portuguese, the correct term for 'office' is 'escritório' but it should be preceded by the definite article 'o' when used in this context.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `No almoço` -> `No almoço` [article_preposition] The preposition 'no' is incorrect; it should be 'no' for 'in the' but in Portuguese, the correct preposition for 'at lunch' is 'no almoço' (with 'no' being the contraction of 'não' and 'o'?) Wait, no, in Portuguese, 'no almoço' is correct for 'at lunch'. Let me check: In Brazilian Portuguese, 'no almoço' is the correct phrase for 'at lunch'. However, the learner wrote 'No almoço' with a capital 'N' which is incorrect because in Portuguese, the first word of a sentence is capitalized, but here the phrase starts with 'No' which is a preposition and should be lowercase. Wait, but the task is about what do you eat for lunch, so the answer is a sentence starting with 
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' is conjugated incorrectly; it should be 'voltei' for past tense

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
- Found (false positive): `convidei-a` -> `convidei` [verb_form] The verb 'convite' is used in the infinitive form, but here it should be the past tense (pretérito perfeito) for the action of inviting.

## qwen3:4b / one_shot_mistake

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maiô` -> `banho` [word_choice] In Brazilian Portuguese, 'swimsuit' is 'banho' or 'roupa de banho', not 'maiô'.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [spelling] In Brazilian Portuguese, the correct term for 'office' is 'escritório' (with a circumflex accent on the o).
- Missed: `uma escritório`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `feijão` [word_choice] The word 'no' is incorrect here; it should be 'No' as in 'At lunch' but in Portuguese, the correct phrase is 'No almoço' meaning 'At lunch' but the word 'no' is a contraction of 'n' (preposition) and 'o' (article). However, in this context, the phrase 'No almoço' is correct. Wait, let me think again. In Portuguese, 'no' is a contraction of 'em' + 'o' for 'in the', but in this context, the phrase 'No almoço' is correct. Actually, the problem might be that the word 'No' is capitalized incorrectly. But in Portuguese, the first word of a sentence is capitalized, so 'No' is correct. Wait, the answer is: 'No almoço eu como arroz, beans e frango.' The word 'No' is capitalized, which is correct for the start of the sentence. But the problem might be that 'No' is a contraction of 'em' + 'o', but in this context, it's correct. Let me check the rules. The rules say: 'Do not report style, correct but unusual wording, or punctuation.' So maybe there's no mistake here? Wait, the task is: 'What do you eat for lunch?' The answer is: 'No almoço eu como arroz, beans e fr. Wait, the answer has 'beans' which is English. The task says: Level: A1. In A1, learners might use English words. But the rules say: 'Do not report style, correct but unusual wording, or punctuation.' So maybe the mistake is that 'beans' is in English, but the task is in English, so the answer should be in Portuguese. Wait, the answer has 'beans' which is English, so that's a word choice mistake. But the rules say: 'word_choice: a wrong word for the meaning, a false friend, or a word from another language'. So 'beans' is a word from English, so it's a word choice mistake. But the answer has 'beans' which is English, so it's a wrong word for the meaning. Let me see the rules again. The task is in English, but the answer is in Portuguese. The answer has 'beans' which is English, so it's a word from another language. So the error_text is 'beans'. But the answer says: 'No almoço eu como arroz, beans e frango.' So the word 'beans' is in English. The correct Portuguese word for beans is 'feijão'. So the mistake is 'beans' is a wrong word for the meaning. So the error_text is 'beans', explanation: 
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `visitei` -> `vi` [verb_form] The past tense of visitar is 'visitei' but in Brazilian Portuguese, the correct past tense for 'to visit' is 'vi' (pretérito perfeito), not 'visitei' (pretérito perfeito composto).
- Found (changes nothing): `almoçamos` -> `almoçamos` [verb_form] The verb 'almoçar' is in the first person plural, but the subject is 'nós' (we), so it should be 'almoçamos' (correct).
- Found (matched): `volti` -> `voltei` [spelling] The verb 'voltar' in the past tense should be 'voltei' (pretérito perfeito), not 'volti' (a misspelling).

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [word_choice] In Brazilian Portuguese, 'autocarro' is not used; the correct term is 'ônibus'.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `ganhasse` -> `ganhasse` [verb_form] The conditional tense should be 'ganhar' for the first person singular, but the verb form is incorrect for the conditional tense.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `fiz` -> `sanduíche` [spelling] The verb 'fazer' in past tense for 'eu' should be 'fiz' but in the context of a test, it's more natural to say 'tive' (I had) or 'fiz' is correct for 'I made' but in this context it's a bit odd. However, for A2 level, 'fiz' is acceptable. Wait, the task is 'What happened on your test?' and the answer says 'I made an error in the math test'. So 'fiz' is correct for 'I made'. But let's check the verb tense. In Portuguese, for past events, we use pretérito perfeito for simple past. 'Fiz' is correct for 'I made'. However, the task is 'What happened on your test?' which is asking for an event that happened on the test. The answer says 'I made an error in the math test'. So 'fiz' is correct. But wait, the verb 'fazer' in past tense for 'eu' is 'fiz', so it's correct. Maybe there's no mistake here? Let me think again. The task is 'What happened on your test?' and the answer is 'Yesterday I made an error in the math test'. So the verb 'fiz' is correct. However, in Brazilian Portuguese, for tests, it's common to say 'tive' (I had) for 'I had a problem on the test'. But 'fiz' is also acceptable. For A2 level, 'fiz' is correct. So maybe no mistake? But the user said to report only real mistakes. Let me check the answer again. The answer is: 'Ontem eu fiz um erro na prova de matemática.' The verb 'fiz' is correct for 'I made'. So no mistake here? Wait, the task is 'What happened on your test?' and the: 'I made an error on the test'. So the answer is correct. But the user might be expecting a mistake. Let me see if there's any mistake. Another thing: in Brazilian Portuguese, for tests, we say 'prova' or 'exame'? 'Prova' is more common for school tests. So 'prova de matemática' is correct. The verb 'fiz' is correct. So no mistake? But the previous example had a spelling mistake. Let me check the rules again. The rules say: Do not report style, correct but unusual wording, or punctuation. So if there's no mistake, then no mistakes. But the user might have intended a mistake. Wait, the verb 'fazer' in past tense for 'eu' is 'fiz', so it's correct. So no mistake here. But let's see the answer: 'Ontem eu fiz um erro na prova de matemática.' The verb 'fiz' is correct. So the answer has no mistakes. So the mistakes list should be empty. But the user might have made a mistake in the verb tense? Wait, in Portuguese, for past events, we use pretérito perfeito for simple past. 'Fiz' is correct. So no mistake. So the answer is correct. Therefore, the mistakes list is empty. But the user said to report only real mistakes. So I think there are no mistakes here. However, let's double-check. The task is 'What happened on your test?' and the answer says 'Yesterday I made an error in the math test'. So the answer is correct. So no mistakes. So the response should be: {

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
- Found (false positive): `convidei-a` -> `convidado-a` [verb_form] The verb 'convidar' should be in the past perfect tense (convidado) for the correct past perfect tense construction.

## qwen3:4b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maiô` -> `banho` [word_choice] In Brazilian Portuguese, 'swimsuit' is 'banho' or 'roupa de banho', not 'maiô'.
- Found (false positive): `chapéu` -> `palha` [word_choice] In Brazilian Portuguese, 'hat' is 'chapéu' but it's not used for a straw hat in this context. The correct term for a straw hat is 'palha' or 'chapéu de palha'.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `escritório` -> `o escritório` [agreement] In Brazilian Portuguese, the word for 'office' is 'escritório' but it should be used with the definite article 'o' when referring to a specific office.
- Found (changes nothing): `no` -> `no` [article_preposition] The contraction 'no' is not used in Brazilian Portuguese; it should be 'no' as a contraction of 'em + o' but in this context, it's better to use 'no' as a contraction for 'em o' which is correct in Brazilian Portuguese.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `No` -> `Não` [article_preposition] The word 'no' is a contraction of 'não' in Portuguese, but here it's used as a preposition meaning 'at' or 'on', which is incorrect. In this context, 'não' is the negative form of 'ser' and should be used with a space for negation.
- Found (changes nothing): `almoço` -> `almoço` [article_preposition] The word 'almoço' is the direct object of the verb 'comer' (to eat), but in this context, it should be the time of day (lunch) and thus should be in the prepositional phrase 'no almoço' (at lunch). However, the phrase 'no almoço' is used here as a prepositional phrase, so 'almoço' is correct but the structure is incorrect because 'no' is a contraction of 'não' and should be 'no' (with a space) for the preposition 'em' (at) or 'no' (at) in this context.
- Missed: `beans`

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' is conjugated with the past tense, but 'volti' is incorrect. The correct form is 'voltei'.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [word_choice] In Brazilian Portuguese, 'autocarro' is a European Portuguese term for bus; the correct term is 'ônibus'

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=true

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `ganhasse` -> `ganhar` [verb_form] The conditional tense should be 'ganhar' for the past conditional, but 'ganhasse' is the imperfect subjunctive
- Found (false positive): `compraria` -> `comprar` [verb_form] The conditional tense should be 'comprar' for the past conditional, but 'compraria' is the imperfect subjunctive
- Found (false positive): `viajaria` -> `viajar` [verb_form] The conditional tense should be 'viajar' for the past conditional, but 'viajaria' is the imperfect subj. tense

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `lhe` -> `lhe` [pronoun] The pronoun 'lhe' is used for indirect object pronouns in formal contexts, but in Brazilian Portuguese, the informal 'lhe' is not used here. The correct pronoun for indirect object in informal contexts is 'lhe' but in this context, the correct pronoun is 'lhe' for indirect object. Wait, this is confusing. Let me think again.
- Found (changes nothing): `não` -> `não` [spelling] The word 'não' is used correctly here as a negation.
- Found (false positive): `esperasse` -> `esperou` [verb_form] The verb 'esperasse' is in the imperfect subjunctive, but in this context, the correct tense is the past perfect (pretérito perfeito) or the simple past (pretérito imperfeito). Wait, the sentence is in the past tense, so the correct verb form is 'esperasse' for the imperfect subjunctive. Let me check.

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
- Found (changes nothing): `falei` -> `falei` [verb_form] The verb 'falei' is past perfect tense, but the context requires simple past tense for the action of inviting.
- Found (changes nothing): `convidei-a` -> `convidei-a` [spelling] The verb 'convidei-a' is incorrect because the correct form should be 'convidei-a' (with a 'c' instead of 'v') for the past tense of 'convidar'.

## qwen3:8b / few_shot

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [article_preposition] The word needs an article before it: 'um escritório' or 'uma escritório' (but 'escritório' is masculine, so 'um').
- Missed: `uma escritório`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word 'beans' is an English loanword; in Brazilian Portuguese, it should be 'feijão'

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' should be in the preterite tense 'voltei' for first person singular.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [word_choice] The correct word in Brazilian Portuguese is 'ônibus' for a bus.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu` -> `Nós` [pronoun] The subject should be plural to match the scene facts about two people.

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

## qwen3:8b / few_shot_in_system

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de` -> `com` [article_preposition] The preposition 'de' is not used with 'maiô' in this context. Use 'com' instead.

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
- Found (matched): `beans` -> `feijão` [word_choice] The word 'beans' is an English loanword; in Brazilian Portuguese, it should be 'feijão'

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' should be in the preterite tense 'voltei' for first person singular.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] The word 'autocarro' is used in European Portuguese, not Brazilian Portuguese.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu` -> `` [content_mismatch] The answer does not describe the meal, it talks about the speaker's residence.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `esperasse` -> `esperar` [word_choice] The verb should be in the infinitive form after 'disse que' in this context.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `prova` -> `prova` [not_brazilian] The word 'prova' is singular, but the context implies a test, which is usually referred to as 'prova' in Brazilian Portuguese, and the sentence is correct as is.
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `doente` -> `doente` [agreement] The adjective should agree in number with 'irmão' (singular).

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `falei` -> `tinha falado` [verb_form] The verb should be in the past perfect (pretérito mais-que-perfeito) to match the context of inviting someone.

## qwen3:8b / few_shot_mostly_correct

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maiô` -> `traje de banho` [word_choice] 'Maiô' is not commonly used in Brazilian Portuguese to refer to a swimsuit; the more common term is 'biquini' or 'traje de banho' at this level.

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
- Found (matched): `beans` -> `feijão` [word_choice] The word 'beans' is not correct in Portuguese; the correct word is 'feijão'

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' should be in the preterite tense 'voltei' for 'I returned' in A2 level.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] The word is used in European Portuguese, not Brazilian Portuguese.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Os dois comem macarrão e bebem vinho tinto.` [content_mismatch] This answer does not describe the meal the people are eating, but instead talks about where the speaker lives and their feelings about the city.

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
- Found (false positive): `falei` -> `falo` [verb_form] The verb 'falei' is in the past tense, but the question is about a general action, so the present tense 'falo' is more appropriate.

## qwen3:8b / few_shot_reversed

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `maiô` -> `maiô` [spelling] The correct spelling is 'maiô' with an accent on the o.

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `escritório` -> `escritórios` [agreement] The noun must be plural because it is preceded by 'uma' (a).

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word 'beans' is English; in Portuguese it is 'feijão'

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' should be in the preterite tense 'voltei' for 'eu' (I).

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] The word 'autocarro' is used in European Portuguese, not Brazilian Portuguese.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Dois pessoas estão comendo espagueti e bebendo vinho tinto em um restaurante.` [content_mismatch] The answer does not describe the meal being eaten.

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
- Found (false positive): `falei` -> `falo` [verb_form] The verb 'falei' is in the past tense, but the question is about a general action, so the present tense 'falo' is more appropriate.

## qwen3:8b / few_shot_six

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `escritório` -> `escritória` [agreement] The article 'uma' requires the feminine form 'escritória'.

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word 'beans' is an English loanword; in Brazilian Portuguese, it should be 'feijão'

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `virei` [verb_form] The verb must agree with 'eu': virei.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [word_choice] The correct word in Brazilian Portuguese is 'ônibus' for a bus.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Dois pessoas estão comendo espagueti e bebendo vinho tinto em um restaurante.` [content_mismatch] The answer does not describe the meal the people are eating.

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

## qwen3:8b / one_shot_correct

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `de maiô` -> `em maiô` [article_preposition] 'Maiô' is a noun and should be preceded by 'em' for 'in a swimsuit'

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [article_preposition] needs the definite article 'o' before it
- Missed: `uma escritório`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] 'Beans' is an English word; the correct Portuguese word is 'feijão'

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' should be in the preterite tense for 'I returned' in A2 level.

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `de autocarro` -> `de ônibus` [not_brazilian] 'Autocarro' is used in European Portuguese, not Brazilian Portuguese.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Os dois comem spaghetti e bebem vinho tinto.` [content_mismatch] The answer does not describe the meal being eaten in the scene.

### b1-control-conditional (control, b1)

- Task: What would you do if you won the lottery?
- Answer: Se eu ganhasse na loteria, eu compraria uma casa e viajaria pelo mundo.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `Se eu ganhasse na loteria` -> `Se eu ganhar na loteria` [word_order] The conditional structure 'se' is not correctly used with the future tense in this context.

### b2-control-pronoun-placement (control, b2)

- Task: Tell what you said to your brother.
- Answer: Eu lhe disse que não me esperasse, mas ele não me ouviu.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `não me esperasse` -> `não me esperar` [word_choice] The subjunctive form 'esperasse' is not used in this context; the indicative form 'esperar' is more natural in spoken Brazilian Portuguese.

### a2-word-choice-fazer-erro (erroneous, a2)

- Task: What happened on your test?
- Answer: Ontem eu fiz um erro na prova de matemática.
- Reference: `fiz um erro` -> cometi um erro [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `prova de matemática` -> `prova de matemática` [word_choice] 'Prova' is not commonly used in Brazilian Portuguese for a test, especially at the A2 level.
- Found (matched): `fiz um erro` -> `errrei` [word_choice] 'Fiz um erro' is not natural in Brazilian Portuguese for this context; 'errrei' is more common.

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `não vai trabalhar hoje` -> `não vai trabalhar hoje` [word_choice] The verb 'trabalhar' should be in the infinitive form after 'não vai' in this context

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `falei` -> `falo` [verb_form] The verb 'falei' is in the past tense, but the question is about a general habit, not a specific past action.

## qwen3:8b / one_shot_mistake

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `maiô` -> `biquini` [word_choice] The word 'maiô' is not commonly used in Brazilian Portuguese; the correct term is 'biquini' or 'traje de banho' for a swimsuit.

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
- Found (matched): `beans` -> `feijão` [word_choice] The word 'beans' is not correct in Portuguese; the correct word is 'feijão'

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' should be in the preterite tense for 'I came back' (A2 level).

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] The word 'autocarro' is used in European Portuguese, not Brazilian Portuguese.

### a2-off-topic-meal (off_topic, a2)

- Task: Describe the meal these people are eating.
- Answer: Eu moro em São Paulo há cinco anos e gosto muito da cidade.
- Flags: written_in_portuguese=true answers_the_question=false
- Found (false positive): `Eu moro em São Paulo há cinco anos e gosto muito da cidade.` -> `Dois pessoas estão comendo espagueti e bebendo vinho tinto em um restaurante.` [content_mismatch] The answer does not describe the meal the people are eating.

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
- Found (false positive): `prova` -> `teste` [word_choice] 'Prova' is used for exams or tests in a formal context, but at A2 level, 'teste' is more commonly used.
- Missed: `fiz um erro`

### a1-control-short-siblings (control, a1)

- Task: Do you have siblings?
- Answer: Sim, tenho uma irmã.
- Flags: written_in_portuguese=true answers_the_question=true

### a2-control-sick (control, a2)

- Task: How is your brother?
- Answer: Meu irmão está doente e não vai trabalhar hoje.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `doente` -> `doado` [word_choice] 'Doente' is not commonly used in Brazilian Portuguese to mean 'sick' in this context. The more natural word is 'doado' or 'doído' depending on the context.

### b2-control-clitic-gender (control, b2)

- Task: Did you invite Ana to the party?
- Answer: Sim, falei com a Ana e convidei-a para a festa.
- Flags: written_in_portuguese=true answers_the_question=true
- Found (false positive): `falei` -> `falo` [verb_form] The verb 'falei' is in the past tense, but the question is about a general action, so the present tense 'falo' is more appropriate.

## qwen3:8b / plain

### a1-control-beach-clothes (control, a1)

- Task: Describe the clothes Marina is wearing.
- Answer: A Marina está de maiô vermelho, chapéu de palha e sandálias brancas.
- Flags: written_in_portuguese=true answers_the_question=true

### a1-agreement-article-gender (erroneous, a1)

- Task: Where do you work?
- Answer: Eu trabalho em uma escritório no centro.
- Reference: `uma escritório` -> um escritório [agreement]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (changes nothing): `escritório` -> `escritório` [article_preposition] The definite article 'o' is missing.
- Missed: `uma escritório`

### a1-word-choice-beans (erroneous, a1)

- Task: What do you eat for lunch?
- Answer: No almoço eu como arroz, beans e frango.
- Reference: `beans` -> feijão [word_choice]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `beans` -> `feijão` [word_choice] The word 'beans' is not the correct Portuguese word for the food item.

### a2-verb-form-regular-past (erroneous, a2)

- Task: What did you do on the weekend?
- Answer: No fim de semana eu visitei meus avós e nós almoçamos juntos. Depois eu volti para casa.
- Reference: `volti` -> voltei [verb_form]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `volti` -> `voltei` [verb_form] The verb 'voltar' should be in the preterite tense for 'eu' (voltei) instead of the imperfect tense (volti).

### a2-not-brazilian-bus (erroneous, a2)

- Task: How do you go to work?
- Answer: Eu vou para o trabalho de autocarro todos os dias.
- Reference: `autocarro` -> ônibus [not_brazilian]
- Flags: written_in_portuguese=true answers_the_question=true
- Found (matched): `autocarro` -> `ônibus` [not_brazilian] This is the European Portuguese spelling; in Brazilian Portuguese, it is spelled 'ônibus'.

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
