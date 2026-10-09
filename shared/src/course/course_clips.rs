use crate::course::{Dialogue, Exercise, GrammarNote, PhrasePair, Reading, VocabularyItem};
use crate::progress::VoiceChoiceKind;

/// A clip a learner can play, with the texts that decide which recording set it plays from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpokenClip {
    pub clip_key: String,
    pub spoken_text: String,
    pub voice_choice: VoiceChoiceKind,
}

impl SpokenClip {
    fn of_own_text(clip_key: Option<&str>, spoken_text: &str) -> Option<SpokenClip> {
        let clip_key: &str = clip_key?;

        Some(SpokenClip {
            clip_key: clip_key.to_string(),
            spoken_text: spoken_text.to_string(),
            voice_choice: VoiceChoiceKind::OwnText,
        })
    }

    fn of_phrase(phrase: &PhrasePair) -> Option<SpokenClip> {
        SpokenClip::of_own_text(phrase.clip_key.as_deref(), &phrase.target)
    }
}

impl Dialogue {
    /// Every line plays from one recording set, chosen by all of its lines' texts.
    pub fn voice_choice(&self) -> VoiceChoiceKind {
        VoiceChoiceKind::WholeDialogue {
            line_texts: self.lines.iter().map(|line| line.target.clone()).collect(),
        }
    }

    pub fn spoken_clips(&self) -> Vec<SpokenClip> {
        let voice_choice: VoiceChoiceKind = self.voice_choice();

        self.lines
            .iter()
            .filter_map(|line| {
                let clip_key: &str = line.clip_key.as_deref()?;

                Some(SpokenClip {
                    clip_key: clip_key.to_string(),
                    spoken_text: line.target.clone(),
                    voice_choice: voice_choice.clone(),
                })
            })
            .collect()
    }
}

impl VocabularyItem {
    pub fn spoken_clips(&self) -> Vec<SpokenClip> {
        [SpokenClip::of_own_text(self.clip_key.as_deref(), &self.target), SpokenClip::of_phrase(&self.example)]
            .into_iter()
            .flatten()
            .collect()
    }
}

impl GrammarNote {
    pub fn spoken_clips(&self) -> Vec<SpokenClip> {
        self.examples.iter().filter_map(SpokenClip::of_phrase).collect()
    }
}

impl Reading {
    /// Its paragraphs' clips, then its questions'.
    pub fn spoken_clips(&self) -> Vec<SpokenClip> {
        let paragraph_clips = self.paragraphs.iter().filter_map(SpokenClip::of_phrase);
        let question_clips = self.questions.iter().flat_map(Exercise::spoken_clips);

        paragraph_clips.chain(question_clips).collect()
    }
}

impl Exercise {
    /// The clips the exercise plays, before or after it is answered.
    pub fn spoken_clips(&self) -> Vec<SpokenClip> {
        let Exercise::Match(exercise) = self
        else {
            let spoken_text: Option<String> = self.spoken_text();

            return spoken_text
                .and_then(|spoken_text| SpokenClip::of_own_text(self.clip_key(), &spoken_text))
                .into_iter()
                .collect();
        };

        exercise.pairs.iter().filter_map(SpokenClip::of_phrase).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::course::{DialogueLine, ListeningChoiceExercise, MatchExercise, MultipleChoiceExercise, SpeakerGenderKind};

    fn phrase(target: &str, clip_key: Option<&str>) -> PhrasePair {
        PhrasePair {
            target: target.to_string(),
            source: format!("source of {target}"),
            source_literal: None,
            clip_key: clip_key.map(str::to_string),
        }
    }

    fn line(target: &str, clip_key: Option<&str>) -> DialogueLine {
        DialogueLine {
            speaker: "Ana".to_string(),
            gender: SpeakerGenderKind::Female,
            target: target.to_string(),
            source: format!("source of {target}"),
            source_literal: None,
            clip_key: clip_key.map(str::to_string),
        }
    }

    #[test]
    fn dialogue_spoken_clips_share_the_whole_dialogues_voice_choice_and_skip_lines_without_a_clip() {
        let dialogue: Dialogue = Dialogue {
            id: "greeting".to_string(),
            title: "Greeting".to_string(),
            context: "Two friends meet.".to_string(),
            lines: vec![line("Oi!", Some("line-1")), line("Tudo bem com você?", None), line("Tudo ótimo.", Some("line-3"))],
        };
        let whole_dialogue: VoiceChoiceKind = VoiceChoiceKind::WholeDialogue {
            line_texts: vec!["Oi!".to_string(), "Tudo bem com você?".to_string(), "Tudo ótimo.".to_string()],
        };

        let clips: Vec<SpokenClip> = dialogue.spoken_clips();

        assert_eq!(
            clips,
            vec![
                SpokenClip {
                    clip_key: "line-1".to_string(),
                    spoken_text: "Oi!".to_string(),
                    voice_choice: whole_dialogue.clone(),
                },
                SpokenClip {
                    clip_key: "line-3".to_string(),
                    spoken_text: "Tudo ótimo.".to_string(),
                    voice_choice: whole_dialogue,
                },
            ],
        );
    }

    #[test]
    fn exercise_spoken_clips_are_a_match_exercises_pairs_or_the_sentence_another_kind_speaks() {
        let match_exercise: Exercise = Exercise::Match(MatchExercise {
            prompt: None,
            pairs: vec![phrase("casa", Some("casa-clip")), phrase("rua", None)],
            explanation: None,
        });
        let listening_choice: Exercise = Exercise::ListeningChoice(ListeningChoiceExercise {
            audio_target: "Bom dia.".to_string(),
            choices: vec!["Good morning.".to_string(), "Good night.".to_string()],
            answer_index: 0,
            explanation: None,
            clip_key: Some("bom-dia-clip".to_string()),
        });
        let multiple_choice: Exercise = Exercise::MultipleChoice(MultipleChoiceExercise {
            prompt: "Which is a greeting?".to_string(),
            choices: vec!["Oi".to_string(), "Casa".to_string()],
            answer_index: 0,
            explanation: None,
        });

        let match_clip_keys: Vec<String> = match_exercise.spoken_clips().into_iter().map(|clip| clip.clip_key).collect();
        let listening_clips: Vec<SpokenClip> = listening_choice.spoken_clips();

        assert_eq!(match_clip_keys, vec!["casa-clip".to_string()]);
        assert_eq!(
            listening_clips,
            vec![SpokenClip {
                clip_key: "bom-dia-clip".to_string(),
                spoken_text: "Bom dia.".to_string(),
                voice_choice: VoiceChoiceKind::OwnText,
            }],
        );
        assert!(multiple_choice.spoken_clips().is_empty());
    }
}
