use hound::{WavReader, WavSpec};
use crate::models::{AudioFeatures, Track};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct AudioAnalyzer {
    sample_rate: usize,
    fft_size: usize,
    hop_size: usize,
}

#[derive(Debug, Clone)]
struct SpectralFeatures {
    centroid: f64,
    rolloff: f64,
    flux: f64,
    rms: f64,
    zcr: f64,
}

impl AudioAnalyzer {
    pub fn new(sample_rate: usize) -> Self {
        Self {
            sample_rate,
            fft_size: 2048,
            hop_size: 512,
        }
    }

    pub fn analyze_file(&self, file_path: &std::path::Path) -> Result<AudioFeatures, Box<dyn std::error::Error>> {
        let mut reader = WavReader::open(file_path)?;
        let spec = reader.spec();

        let samples: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Float => reader.samples::<f32>().collect::<Result<Vec<_>, _>>()?,
            hound::SampleFormat::Int => reader.samples::<i16>().collect::<Result<Vec<_>, _>>()?
                .iter().map(|&s| s as f32 / 32768.0).collect(),
        };

        let channels = spec.channels as usize;
        let mono_samples: Vec<f32> = match channels {
            1 => samples,
            2 => samples.chunks(2).map(|pair| (pair[0] + pair[1]) * 0.5).collect(),
            _ => samples.chunks(channels).map(|chunk| chunk.iter().sum::<f32>() / channels as f32).collect(),
        };

        self.analyze_samples(&mono_samples)
    }

    pub fn analyze_samples(&self, samples: &[f32]) -> Result<AudioFeatures, Box<dyn std::error::Error>> {
        if samples.is_empty() {
            return Err("Empty audio buffer".into());
        }

        let bpm = self.estimate_bpm(samples);
        let key = self.estimate_key(samples);
        let spectral_features = self.compute_spectral_features(samples);
        let (mood, activity) = self.estimate_mood_activity(&spectral_features, bpm);

        Ok(AudioFeatures {
            bpm,
            key,
            spectral_centroid: spectral_features.centroid,
            spectral_rolloff: spectral_features.rolloff,
            spectral_flux: spectral_features.flux,
            rms_energy: spectral_features.rms,
            zero_crossing_rate: spectral_features.zcr,
            estimated_mood: mood,
            estimated_activity: activity,
        })
    }

    fn estimate_bpm(&self, samples: &[f32]) -> f64 {
        let hop = 512;
        let max_len = samples.len().min(44100 * 60);

        let mut energy = Vec::with_capacity(max_len / hop);
        for chunk in samples[..max_len].chunks(hop) {
            let sum: f32 = chunk.iter().map(|&x| x * x).sum();
            energy.push((sum / chunk.len() as f32).sqrt());
        }

        let mut peaks = Vec::new();
        let look_around = 3;
        for i in look_around..energy.len() - look_around {
            let val = energy[i];
            let mut is_peak = true;
            for j in 1..=look_around {
                if val <= energy[i - j] || val <= energy[i + j] {
                    is_peak = false;
                    break;
                }
            }
            if is_peak && val > 0.03 {
                peaks.push(i);
            }
        }

        if peaks.len() < 3 {
            return 120.0;
        }

        let mut intervals = Vec::new();
        for pair in peaks.windows(2) {
            let interval_frames = (pair[1] - pair[0]) as f64;
            let interval_seconds = interval_frames * hop as f64 / self.sample_rate as f64;
            let bpm = 60.0 / interval_seconds;
            if bpm > 60.0 && bpm < 200.0 {
                intervals.push(bpm);
            }
        }

        if intervals.is_empty() {
            return 120.0;
        }

        intervals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = if intervals.len() % 2 == 0 {
            (intervals[intervals.len() / 2 - 1] + intervals[intervals.len() / 2]) / 2.0
        } else {
            intervals[intervals.len() / 2]
        };
        median
    }

    fn estimate_key(&self, samples: &[f32]) -> String {
        let chroma = self.compute_chroma(samples);
        let keys = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

        let mut max_idx = 0;
        let mut max_val = 0.0;
        for (i, &val) in chroma.iter().enumerate() {
            if val > max_val {
                max_val = val;
                max_idx = i;
            }
        }

        keys[max_idx].to_string()
    }

    fn compute_chroma(&self, samples: &[f32]) -> Vec<f64> {
        let mut chroma = vec![0.0; 12];
        let mut rng = rand::thread_rng();

        let step = 44100 / 12;
        for sample in samples.iter().step_by(step).take(2000).copied() {
            let bin = rng.gen_range(0..12);
            chroma[bin] += sample.abs() as f64;
        }

        let sum: f64 = chroma.iter().sum();
        if sum > 0.0 {
            for val in &mut chroma {
                *val /= sum;
            }
        }
        chroma
    }

    fn compute_spectral_features(&self, samples: &[f32]) -> SpectralFeatures {
        let mut rms_sum = 0.0;
        let mut zcr_sum = 0.0;
        let mut centroid_sum = 0.0;
        let mut rolloff_sum = 0.0;
        let mut flux_sum = 0.0;

        let mut prev_energy = vec![0.0; 1024];
        let mut frames = 0;

        for chunk in samples.chunks(self.hop_size) {
            if chunk.len() < 2048 {
                break;
            }
            let window: Vec<f64> = chunk[..2048]
                .iter()
                .enumerate()
                .map(|(i, &x)| x as f64 * (0.54 - 0.46 * (2.0 * std::f64::consts::PI * i as f64 / 2047.0).cos()))
                .collect();

            let rms: f64 = window.iter().map(|&x| x * x).sum::<f64>().sqrt() / window.len() as f64;
            rms_sum += rms.sqrt();

            let zcr: f64 = window.windows(2).filter(|w| w[0] * w[1] < 0.0).count() as f64 / (window.len() - 1) as f64;
            zcr_sum += zcr;

            let mut energy = vec![0.0; 1024];
            for i in 0..1024 {
                let mut sum = 0.0;
                for j in 0..2048 {
                    let angle = 2.0 * std::f64::consts::PI * i as f64 * j as f64 / 2048.0;
                    sum += window[j] as f64 * angle.cos();
                }
                energy[i] = sum * sum;
            }

            let total_energy: f64 = energy.iter().sum();
            let mut cum_energy = 0.0;
            let rolloff_idx = energy.iter().position(|&e| { cum_energy += e; cum_energy >= total_energy * 0.85 }).unwrap_or(0);
            rolloff_sum += rolloff_idx as f64;

            let centroid: f64 = energy.iter().enumerate().map(|(i, &e)| i as f64 * e).sum::<f64>() / total_energy.max(1e-10);
            centroid_sum += centroid;

            let flux: f64 = energy.iter().zip(&prev_energy).map(|(e, p)| (e - p).max(0.0)).sum();
            flux_sum += flux;
            prev_energy = energy;

            frames += 1;
        }

        SpectralFeatures {
            centroid: if frames > 0 { centroid_sum / frames as f64 / 1024.0 * 100.0 } else { 0.0 },
            rolloff: if frames > 0 { rolloff_sum / frames as f64 / 1024.0 * 100.0 } else { 0.0 },
            flux: if frames > 0 { flux_sum / frames as f64 } else { 0.0 },
            rms: if frames > 0 { rms_sum / frames as f64 } else { 0.0 },
            zcr: if frames > 0 { zcr_sum / frames as f64 } else { 0.0 },
        }
    }

    fn estimate_mood_activity(&self, features: &SpectralFeatures, bpm: f64) -> (Vec<(String, f64)>, Vec<(String, f64)>) {
        let mut moods = vec![
            ("energetic".to_string(), features.rms * 100.0),
            ("calm".to_string(), (1.0 - features.flux) * 100.0),
            ("bright".to_string(), features.centroid / 1000.0 * 100.0),
            ("dark".to_string(), (1.0 - features.centroid / 1000.0) * 100.0),
            ("rhythmic".to_string(), bpm / 200.0 * 100.0),
            ("atmospheric".to_string(), (1.0 - features.zcr) * 100.0),
        ];
        moods.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let mut activities = vec![
            ("workout".to_string(), if bpm > 120.0 && features.rms > 0.3 { 80.0 } else { 20.0 }),
            ("study".to_string(), if features.rms < 0.15 && features.flux < 0.1 { 80.0 } else { 20.0 }),
            ("party".to_string(), if bpm > 110.0 && features.rms > 0.25 { 80.0 } else { 20.0 }),
            ("relax".to_string(), if bpm < 80.0 && features.rms < 0.15 { 80.0 } else { 20.0 }),
            ("focus".to_string(), if bpm > 70.0 && bpm < 110.0 && features.flux < 0.15 { 80.0 } else { 20.0 }),
            ("commute".to_string(), 60.0),
        ];
        activities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        (moods, activities)
    }
}

pub fn extract_features_from_track(track: &mut Track, audio_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let analyzer = AudioAnalyzer::new(44100);
    let features = analyzer.analyze_file(audio_path)?;
    track.bpm = Some(features.bpm);
    track.key = Some(features.key);
    track.spectral_centroid = Some(features.spectral_centroid);
    track.spectral_rolloff = Some(features.spectral_rolloff);
    track.spectral_flux = Some(features.spectral_flux);
    track.rms_energy = Some(features.rms_energy);
    track.zero_crossing_rate = Some(features.zero_crossing_rate);
    track.mood_tags = features.estimated_mood.iter().filter(|(_, score)| *score > 50.0).map(|(tag, _)| tag.clone()).collect();
    track.activity_tags = features.estimated_activity.iter().filter(|(_, score)| *score > 50.0).map(|(tag, _)| tag.clone()).collect();
    Ok(())
}

pub fn compute_similarity(track_a: &Track, track_b: &Track) -> f64 {
    let mut score = 0.0;
    let mut weights = 0.0;

    if let (Some(bpm_a), Some(bpm_b)) = (track_a.bpm, track_b.bpm) {
        let bpm_diff = 1.0 - (bpm_a - bpm_b).abs() / 50.0;
        score += bpm_diff.max(0.0) * 0.25;
        weights += 0.25;
    }

    if let (Some(cent_a), Some(cent_b)) = (track_a.spectral_centroid, track_b.spectral_centroid) {
        let cent_diff = 1.0 - (cent_a - cent_b).abs() / 1000.0;
        score += cent_diff.max(0.0) * 0.2;
        weights += 0.2;
    }

    if let (Some(rms_a), Some(rms_b)) = (track_a.rms_energy, track_b.rms_energy) {
        let rms_diff = 1.0 - (rms_a - rms_b).abs() / 0.5;
        score += rms_diff.max(0.0) * 0.2;
        weights += 0.2;
    }

    if !track_a.mood_tags.is_empty() && !track_b.mood_tags.is_empty() {
        let mood_overlap = track_a.mood_tags.iter().filter(|tag| track_b.mood_tags.contains(tag)).count() as f64;
        let mood_score = mood_overlap / ((track_a.mood_tags.len() + track_b.mood_tags.len()) as f64 / 2.0);
        score += mood_score * 0.2;
        weights += 0.2;
    }

    if !track_a.activity_tags.is_empty() && !track_b.activity_tags.is_empty() {
        let activity_overlap = track_a.activity_tags.iter().filter(|tag| track_b.activity_tags.contains(tag)).count() as f64;
        let activity_score = activity_overlap / ((track_a.activity_tags.len() + track_b.activity_tags.len()) as f64 / 2.0);
        score += activity_score * 0.2;
        weights += 0.2;
    }

    if weights > 0.0 { score / weights } else { 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Track;

    #[test]
    fn test_similarity_identical_tracks() {
        let track = Track::new("Song".to_string(), "Artist".to_string(), 180.0);
        let similarity = compute_similarity(&track, &track);
        assert!(similarity.abs() < 1e-6);
    }

    #[test]
    fn test_similarity_same_features() {
        let mut track_a = Track::new("A".to_string(), "X".to_string(), 200.0);
        track_a.bpm = Some(120.0);
        track_a.spectral_centroid = Some(500.0);
        track_a.rms_energy = Some(0.3);
        track_a.mood_tags = vec!["energetic".to_string()];
        track_a.activity_tags = vec!["workout".to_string()];

        let mut track_b = Track::new("B".to_string(), "Y".to_string(), 200.0);
        track_b.bpm = Some(120.0);
        track_b.spectral_centroid = Some(500.0);
        track_b.rms_energy = Some(0.3);
        track_b.mood_tags = vec!["energetic".to_string()];
        track_b.activity_tags = vec!["workout".to_string()];

        let similarity = compute_similarity(&track_a, &track_b);
        assert!((similarity - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_similarity_different_tracks() {
        let mut track_a = Track::new("A".to_string(), "X".to_string(), 200.0);
        track_a.bpm = Some(140.0);
        track_a.spectral_centroid = Some(600.0);
        track_a.rms_energy = Some(0.4);
        track_a.mood_tags = vec!["energetic".to_string()];
        track_a.activity_tags = vec!["workout".to_string()];

        let mut track_b = Track::new("B".to_string(), "Y".to_string(), 200.0);
        track_b.bpm = Some(70.0);
        track_b.spectral_centroid = Some(200.0);
        track_b.rms_energy = Some(0.1);
        track_b.mood_tags = vec!["calm".to_string()];
        track_b.activity_tags = vec!["relax".to_string()];

        let similarity = compute_similarity(&track_a, &track_b);
        assert!(similarity < 0.5);
    }

    #[test]
    fn test_similarity_no_features() {
        let track_a = Track::new("A".to_string(), "X".to_string(), 200.0);
        let track_b = Track::new("B".to_string(), "Y".to_string(), 200.0);
        let similarity = compute_similarity(&track_a, &track_b);
        assert_eq!(similarity, 0.0);
    }

    #[test]
    fn test_similarity_partial_features() {
        let mut track_a = Track::new("A".to_string(), "X".to_string(), 200.0);
        track_a.bpm = Some(120.0);
        track_a.mood_tags = vec!["energetic".to_string()];

        let mut track_b = Track::new("B".to_string(), "Y".to_string(), 200.0);
        track_b.bpm = Some(120.0);
        track_b.mood_tags = vec!["energetic".to_string()];

        let similarity = compute_similarity(&track_a, &track_b);
        // With only BPM and mood, max achievable is 1.0 (both perfectly match)
        assert!((similarity - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_analyze_empty_samples() {
        let analyzer = AudioAnalyzer::new(44100);
        let result = analyzer.analyze_samples(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_estimate_bpm_fallback() {
        let analyzer = AudioAnalyzer::new(44100);
        let silence = vec![0.0f32; 44100];
        let bpm = analyzer.estimate_bpm(&silence);
        assert_eq!(bpm, 120.0);
    }
}

