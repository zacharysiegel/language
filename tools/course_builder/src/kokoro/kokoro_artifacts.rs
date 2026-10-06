use std::f64::consts::{PI, SQRT_2};

use crate::kokoro::kokoro_synthesizer::SAMPLE_RATE;

pub const FRAME_SAMPLES: usize = 120;
const HIGH_PASS_CUTOFF_HERTZ: f64 = 3000.0;
const NOISE_HIGH_FREQUENCY_PROPORTION: f64 = 0.7;
const NOISE_LEVEL_FLOOR_DECIBELS: f64 = -30.0;
const ACTIVE_LEVEL_FLOOR_DECIBELS: f64 = -45.0;
const MINIMUM_NOISE_RUN_FRAMES: usize = 2;
const MINIMUM_START_RUN_FRAMES: usize = 6;
const MINIMUM_MIDDLE_RUN_FRAMES: usize = 4;
const START_EDGE_FRAMES: usize = 2;
const END_EDGE_FRAMES: usize = 20;
const END_QUIET_DECIBELS: f64 = -25.0;
const START_QUIET_DECIBELS: f64 = -22.0;
const GROW_FRAMES: usize = 2;
const GROW_HIGH_FREQUENCY_PROPORTION: f64 = 0.4;
const GROW_LEVEL_FLOOR_DECIBELS: f64 = -35.0;
const FADE_SAMPLES: usize = 72;

const VOWEL_SYMBOLS: &str = "aeiouɛɔɐɑɪʊəæAIOWUYQEɨ";
/* Phonemes a fricative, affricate or plosive burst can legitimately come from. */
const NOISY_SYMBOLS: &str = "pbtdkɡfvszʃʒʧʤʦʣxhSçɣχʁʔ";
const STRESS_MARKS: &str = "ˌˈ";
const CLAUSE_PUNCTUATION: &str = ",.;:!?…";
const IGNORED_SYMBOLS: &str = " ,.!?;:…—\"“”ˈˌː\u{303}";
const COMBINING_TILDE: char = '\u{303}';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RunPositionKind {
    Start,
    Middle,
    End,
}

#[derive(Debug, Clone, Copy)]
struct NoiseRun {
    start_frame: usize,
    end_frame: usize,
    position: RunPositionKind,
}

pub struct FrameFeatures {
    high_frequency_proportions: Vec<f64>,
    levels_decibels: Vec<f64>,
}

impl FrameFeatures {
    pub fn levels_decibels(&self) -> &[f64] {
        &self.levels_decibels
    }
}

/// Cuts the hissing bursts Kokoro sometimes adds at the start, end or clause boundaries of a clip where the phonemes have no fricative.
pub fn remove_spurious_noise(samples: &[f32], phonemes: &str) -> Vec<f32> {
    let features: FrameFeatures = frame_features(samples);
    let Some((first_active, end_active)) = active_frame_range(&features) else {
        return samples.to_vec();
    };

    let runs: Vec<NoiseRun> = find_noise_runs(&features, first_active, end_active);
    let removals: Vec<NoiseRun> = select_removals(&runs, phonemes);

    if removals.is_empty() {
        return samples.to_vec();
    }

    cut_runs(samples, &features, &removals, first_active, end_active)
}

pub fn frame_features(samples: &[f32]) -> FrameFeatures {
    let high_passed: Vec<f64> = high_pass(samples);
    let frame_count: usize = samples.len() / FRAME_SAMPLES;
    let mut total_energies: Vec<f64> = Vec::with_capacity(frame_count);
    let mut high_energies: Vec<f64> = Vec::with_capacity(frame_count);

    for frame_index in 0..frame_count {
        let range: std::ops::Range<usize> = frame_index * FRAME_SAMPLES..(frame_index + 1) * FRAME_SAMPLES;

        total_energies.push(samples[range.clone()].iter().map(|sample| f64::from(*sample).powi(2)).sum());
        high_energies.push(high_passed[range].iter().map(|sample| sample.powi(2)).sum());
    }

    let peak_energy: f64 = total_energies.iter().copied().fold(0.0, f64::max);

    FrameFeatures {
        high_frequency_proportions: total_energies
            .iter()
            .zip(&high_energies)
            .map(|(total, high)| high / (total + 1e-12))
            .collect(),
        levels_decibels: total_energies
            .iter()
            .map(|energy| 10.0 * (energy / (peak_energy + 1e-12) + 1e-12).log10())
            .collect(),
    }
}

/* Second-order Butterworth high-pass, by the bilinear transform. */
fn high_pass(samples: &[f32]) -> Vec<f64> {
    let warped: f64 = (PI * HIGH_PASS_CUTOFF_HERTZ / f64::from(SAMPLE_RATE)).tan();
    let normalization: f64 = 1.0 / (1.0 + SQRT_2 * warped + warped * warped);
    let (b0, b1, b2): (f64, f64, f64) = (normalization, -2.0 * normalization, normalization);
    let a1: f64 = 2.0 * (warped * warped - 1.0) * normalization;
    let a2: f64 = (1.0 - SQRT_2 * warped + warped * warped) * normalization;
    let (mut x1, mut x2, mut y1, mut y2): (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.0);
    let mut filtered: Vec<f64> = Vec::with_capacity(samples.len());

    for sample in samples {
        let x0: f64 = f64::from(*sample);
        let y0: f64 = b0 * x0 + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2;

        filtered.push(y0);
        (x2, x1, y2, y1) = (x1, x0, y1, y0);
    }

    filtered
}

pub fn active_frame_range(features: &FrameFeatures) -> Option<(usize, usize)> {
    let is_active = |level: &f64| *level > ACTIVE_LEVEL_FLOOR_DECIBELS;
    let first: usize = features.levels_decibels.iter().position(is_active)?;
    let last: usize = features.levels_decibels.iter().rposition(is_active)?;

    Some((first, last + 1))
}

fn find_noise_runs(features: &FrameFeatures, first_active: usize, end_active: usize) -> Vec<NoiseRun> {
    let is_noisy = |frame: usize| {
        features.high_frequency_proportions[frame] > NOISE_HIGH_FREQUENCY_PROPORTION
            && features.levels_decibels[frame] > NOISE_LEVEL_FLOOR_DECIBELS
    };
    let mut runs: Vec<NoiseRun> = Vec::new();
    let mut run_start: Option<usize> = None;

    for frame in first_active..=end_active {
        let noisy: bool = frame < end_active && is_noisy(frame);

        match (noisy, run_start) {
            (true, None) => run_start = Some(frame),
            (false, Some(start)) => {
                if frame - start >= MINIMUM_NOISE_RUN_FRAMES {
                    runs.push(NoiseRun {
                        start_frame: start,
                        end_frame: frame,
                        position: classify_run(features, start, frame, first_active, end_active),
                    });
                }

                run_start = None;
            },
            _ => {},
        }
    }

    runs
}

fn classify_run(features: &FrameFeatures, start: usize, end: usize, first_active: usize, end_active: usize) -> RunPositionKind {
    let quiet_before: bool = start - first_active <= START_EDGE_FRAMES
        && features.levels_decibels[first_active..start].iter().all(|level| *level < START_QUIET_DECIBELS);
    let quiet_after: bool = end_active - end <= END_EDGE_FRAMES
        && features.levels_decibels[end..end_active].iter().all(|level| *level < END_QUIET_DECIBELS);

    if quiet_before {
        RunPositionKind::Start
    } else if quiet_after {
        RunPositionKind::End
    } else {
        RunPositionKind::Middle
    }
}

fn select_removals(runs: &[NoiseRun], phonemes: &str) -> Vec<NoiseRun> {
    let symbols: Vec<char> = phonemes.chars().filter(|symbol| !IGNORED_SYMBOLS.contains(*symbol)).collect();
    let Some(last_symbol) = symbols.last() else {
        return Vec::new();
    };
    let starts_with_stressed_vowel: bool = starts_with_stressed_vowel(phonemes);
    let mut removals: Vec<NoiseRun> = Vec::new();
    let mut middle_runs: Vec<NoiseRun> = Vec::new();

    for run in runs {
        let length: usize = run.end_frame - run.start_frame;

        match run.position {
            RunPositionKind::End if VOWEL_SYMBOLS.contains(*last_symbol) => removals.push(*run),
            RunPositionKind::Start if starts_with_stressed_vowel && length >= MINIMUM_START_RUN_FRAMES => removals.push(*run),
            RunPositionKind::Middle if length >= MINIMUM_MIDDLE_RUN_FRAMES => middle_runs.push(*run),
            _ => {},
        }
    }

    let expected_noise: usize = symbols.iter().filter(|symbol| NOISY_SYMBOLS.contains(**symbol)).count();
    let junction_count: usize = count_vowel_to_stressed_vowel_junctions(phonemes);
    let surplus: usize = middle_runs.len().saturating_sub(expected_noise);

    if junction_count > 0 && surplus > 0 {
        removals.extend(middle_runs.into_iter().take(junction_count.min(surplus)));
    }

    removals
}

fn starts_with_stressed_vowel(phonemes: &str) -> bool {
    let mut characters = phonemes.trim_start().chars();

    matches!(
        (characters.next(), characters.next()),
        (Some(mark), Some(vowel)) if STRESS_MARKS.contains(mark) && VOWEL_SYMBOLS.contains(vowel)
    )
}

/* A vowel ending a clause, then a word that begins with a stressed vowel. */
fn count_vowel_to_stressed_vowel_junctions(phonemes: &str) -> usize {
    let characters: Vec<char> = phonemes.chars().collect();
    let mut count: usize = 0;

    for (index, character) in characters.iter().enumerate() {
        let follows_punctuation: bool = CLAUSE_PUNCTUATION.contains(*character) && (index == 0 || !CLAUSE_PUNCTUATION.contains(characters[index - 1]));

        if !follows_punctuation {
            continue;
        }

        let before: Option<char> = characters[..index].iter().rev().find(|symbol| **symbol != COMBINING_TILDE).copied();
        let mut after = characters[index..].iter().skip_while(|symbol| CLAUSE_PUNCTUATION.contains(**symbol) || symbol.is_whitespace());
        let (mark, vowel) = (after.next(), after.next());

        let vowel_before: bool = before.is_some_and(|symbol| VOWEL_SYMBOLS.contains(symbol));
        let stressed_vowel_after: bool = matches!((mark, vowel), (Some(mark), Some(vowel)) if STRESS_MARKS.contains(*mark) && VOWEL_SYMBOLS.contains(*vowel));

        if vowel_before && stressed_vowel_after {
            count += 1;
        }
    }

    count
}

fn cut_runs(samples: &[f32], features: &FrameFeatures, removals: &[NoiseRun], first_active: usize, end_active: usize) -> Vec<f32> {
    let mut intervals: Vec<(usize, usize)> = removals
        .iter()
        .map(|run| grown_interval(features, run, first_active, end_active, samples.len()))
        .collect();

    intervals.sort_unstable();

    let mut kept_segments: Vec<(usize, usize)> = Vec::new();
    let mut cursor: usize = 0;

    for (start, end) in intervals {
        if start > cursor {
            kept_segments.push((cursor, start));
        }

        cursor = cursor.max(end);
    }

    if cursor < samples.len() {
        kept_segments.push((cursor, samples.len()));
    }

    let mut output: Vec<f32> = Vec::with_capacity(samples.len());

    for (index, (start, end)) in kept_segments.iter().enumerate() {
        let mut segment: Vec<f32> = samples[*start..*end].to_vec();
        let length: usize = segment.len();

        if index > 0 && length > FADE_SAMPLES {
            for (position, sample) in segment.iter_mut().take(FADE_SAMPLES).enumerate() {
                *sample *= position as f32 / FADE_SAMPLES as f32;
            }
        }

        if index + 1 < kept_segments.len() && length > FADE_SAMPLES {
            for (position, sample) in segment.iter_mut().rev().take(FADE_SAMPLES).enumerate() {
                *sample *= position as f32 / FADE_SAMPLES as f32;
            }
        }

        output.extend(segment);
    }

    output
}

fn grown_interval(features: &FrameFeatures, run: &NoiseRun, first_active: usize, end_active: usize, sample_count: usize) -> (usize, usize) {
    let is_noise_like = |frame: usize| {
        features.high_frequency_proportions[frame] > GROW_HIGH_FREQUENCY_PROPORTION && features.levels_decibels[frame] > GROW_LEVEL_FLOOR_DECIBELS
    };
    let mut start_frame: usize = run.start_frame;
    let mut end_frame: usize = run.end_frame;

    for _ in 0..GROW_FRAMES {
        if start_frame > first_active + 1 && is_noise_like(start_frame - 1) {
            start_frame -= 1;
        }
    }

    for _ in 0..GROW_FRAMES {
        if end_frame < end_active && is_noise_like(end_frame) {
            end_frame += 1;
        }
    }

    let start_sample: usize = if run.position == RunPositionKind::Start { 0 } else { start_frame * FRAME_SAMPLES };
    let end_sample: usize = if run.position == RunPositionKind::End { sample_count } else { end_frame * FRAME_SAMPLES };

    (start_sample, end_sample)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tone(frames: usize) -> Vec<f32> {
        (0..frames * FRAME_SAMPLES)
            .map(|index| 0.3 * (2.0 * PI as f32 * 200.0 * index as f32 / SAMPLE_RATE as f32).sin())
            .collect()
    }

    /* Differenced pseudo-random noise, which concentrates its energy above 3 kHz. */
    fn hiss(frames: usize) -> Vec<f32> {
        let mut state: u32 = 12345;
        let mut previous: f32 = 0.0;

        (0..frames * FRAME_SAMPLES)
            .map(|_| {
                state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);

                let current: f32 = (state >> 8) as f32 / (1u32 << 24) as f32 - 0.5;
                let differenced: f32 = (current - previous) * 0.25;

                previous = current;
                differenced
            })
            .collect()
    }

    fn concatenated(parts: &[Vec<f32>]) -> Vec<f32> {
        parts.iter().flatten().copied().collect()
    }

    #[test]
    fn trailing_hiss_after_a_final_vowel_is_cut() {
        let samples: Vec<f32> = concatenated(&[tone(40), hiss(12)]);

        let repaired: Vec<f32> = remove_spurious_noise(&samples, "olˈa");

        assert!(repaired.len() < samples.len() - 6 * FRAME_SAMPLES);
    }

    #[test]
    fn trailing_hiss_after_a_final_fricative_is_kept() {
        let samples: Vec<f32> = concatenated(&[tone(40), hiss(12)]);

        let repaired: Vec<f32> = remove_spurious_noise(&samples, "kˈazas");

        assert_eq!(repaired.len(), samples.len());
    }

    #[test]
    fn leading_hiss_before_a_stressed_vowel_is_cut() {
        let samples: Vec<f32> = concatenated(&[hiss(10), tone(40)]);

        let repaired: Vec<f32> = remove_spurious_noise(&samples, "ˌobriɡˈadæ");

        assert!(repaired.len() < samples.len() - 5 * FRAME_SAMPLES);
    }

    #[test]
    fn leading_hiss_before_a_fricative_is_kept() {
        let samples: Vec<f32> = concatenated(&[hiss(10), tone(40)]);

        let repaired: Vec<f32> = remove_spurious_noise(&samples, "sˈɔ");

        assert_eq!(repaired.len(), samples.len());
    }

    #[test]
    fn hiss_at_a_clause_junction_between_vowels_is_cut() {
        let samples: Vec<f32> = concatenated(&[tone(30), hiss(9), tone(30)]);

        let repaired: Vec<f32> = remove_spurious_noise(&samples, "ˈoɪ, ˈɐ̃næ!");

        assert!(repaired.len() < samples.len() - 5 * FRAME_SAMPLES);
    }

    #[test]
    fn hiss_inside_a_word_with_a_fricative_is_kept() {
        let samples: Vec<f32> = concatenated(&[tone(30), hiss(9), tone(30)]);

        let repaired: Vec<f32> = remove_spurious_noise(&samples, "kˈasa");

        assert_eq!(repaired.len(), samples.len());
    }

    #[test]
    fn silence_is_returned_unchanged() {
        let samples: Vec<f32> = vec![0.0; 10 * FRAME_SAMPLES];

        assert_eq!(remove_spurious_noise(&samples, "a"), samples);
    }

    #[test]
    fn junctions_need_a_vowel_then_punctuation_then_a_stressed_vowel() {
        assert_eq!(count_vowel_to_stressed_vowel_junctions("ˈoɪ, ˈɐ̃næ!"), 1);
        assert_eq!(count_vowel_to_stressed_vowel_junctions("bˈoŋ ʤˈiæ, ˈɐ̃næ!"), 1);
        assert_eq!(count_vowel_to_stressed_vowel_junctions("ˈoɪ, kˈaɾəlʊs!"), 0);
        assert_eq!(count_vowel_to_stressed_vowel_junctions("ˈoɪ ˈɐ̃næ"), 0);
    }
}
