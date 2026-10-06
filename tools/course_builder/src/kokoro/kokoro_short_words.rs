use crate::kokoro::kokoro_artifacts::{self, FRAME_SAMPLES};

const MINIMUM_VOICED_FRAMES: usize = 4;
const MINIMUM_PAUSE_FRAMES: usize = 6;
const PAUSE_LEVEL_DECIBELS: f64 = -30.0;
const MINIMUM_WORD_FRAMES: usize = 12;
const MAXIMUM_WORD_FRAMES: usize = 200;
const FADE_OUT_SAMPLES: usize = 240;

/// Phonemes of a neutral word spoken after the target, which gives a one- or two-sound input the context Kokoro needs.
pub const CARRIER_PHONEMES: &str = "kˈazæ";

/// Returns the sound before the first pause, which is the target word when it was followed by a carrier.
pub fn leading_word_before_pause(samples: &[f32]) -> Option<Vec<f32>> {
    let features: kokoro_artifacts::FrameFeatures = kokoro_artifacts::frame_features(samples);
    let (first_active, end_active) = kokoro_artifacts::active_frame_range(&features)?;
    let mut voiced_frames: usize = 0;
    let mut frame: usize = first_active;

    while frame < end_active {
        if features.levels_decibels()[frame] > PAUSE_LEVEL_DECIBELS {
            voiced_frames += 1;
            frame += 1;
            continue;
        }

        let pause_end: usize = (frame..end_active)
            .find(|candidate| features.levels_decibels()[*candidate] > PAUSE_LEVEL_DECIBELS)
            .unwrap_or(end_active);

        if voiced_frames >= MINIMUM_VOICED_FRAMES && pause_end - frame >= MINIMUM_PAUSE_FRAMES {
            return word_ending_at(samples, first_active, frame);
        }

        frame = pause_end;
    }

    None
}

fn word_ending_at(samples: &[f32], first_active: usize, pause_start: usize) -> Option<Vec<f32>> {
    let length: usize = pause_start - first_active;

    if !(MINIMUM_WORD_FRAMES..=MAXIMUM_WORD_FRAMES).contains(&length) {
        return None;
    }

    let mut word: Vec<f32> = samples[first_active * FRAME_SAMPLES..pause_start * FRAME_SAMPLES].to_vec();
    let fade_length: usize = FADE_OUT_SAMPLES.min(word.len());
    let word_length: usize = word.len();

    for (position, sample) in word[word_length - fade_length..].iter_mut().enumerate() {
        *sample *= 1.0 - position as f32 / fade_length as f32;
    }

    Some(word)
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;
    use crate::kokoro::kokoro_synthesizer::SAMPLE_RATE;

    fn tone(frames: usize) -> Vec<f32> {
        (0..frames * FRAME_SAMPLES)
            .map(|index| 0.3 * (2.0 * PI * 200.0 * index as f32 / SAMPLE_RATE as f32).sin())
            .collect()
    }

    fn silence(frames: usize) -> Vec<f32> {
        vec![0.0; frames * FRAME_SAMPLES]
    }

    #[test]
    fn leading_word_stops_at_the_first_pause() {
        let samples: Vec<f32> = [tone(20), silence(10), tone(40)].concat();

        let word: Vec<f32> = leading_word_before_pause(&samples).expect("a pause follows the word");

        assert_eq!(word.len(), 20 * FRAME_SAMPLES);
    }

    #[test]
    fn leading_word_is_absent_without_a_pause() {
        let samples: Vec<f32> = tone(60);

        assert!(leading_word_before_pause(&samples).is_none());
    }

    #[test]
    fn leading_word_rejects_a_word_that_is_too_short() {
        let samples: Vec<f32> = [tone(5), silence(10), tone(40)].concat();

        assert!(leading_word_before_pause(&samples).is_none());
    }
}
