use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub duration_seconds: f64,
    pub file_path: Option<String>,
    pub file_hash: Option<String>,
    pub bpm: Option<f64>,
    pub key: Option<String>,
    pub spectral_centroid: Option<f64>,
    pub spectral_rolloff: Option<f64>,
    pub spectral_flux: Option<f64>,
    pub rms_energy: Option<f64>,
    pub zero_crossing_rate: Option<f64>,
    pub mood_tags: Vec<String>,
    pub activity_tags: Vec<String>,
    pub genres: Vec<String>,
    pub added_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub tracks: Vec<String>,
    pub generation_params: GenerationParams,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationParams {
    pub mood: Option<String>,
    pub activity: Option<String>,
    pub min_bpm: Option<f64>,
    pub max_bpm: Option<f64>,
    pub target_key: Option<String>,
    pub genres: Vec<String>,
    pub max_duration_seconds: Option<f64>,
    pub similarity_weight: f64,
    pub diversity_weight: f64,
    pub popularity_weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistGenerationRequest {
    pub name: String,
    pub description: Option<String>,
    pub seed_tracks: Vec<String>,
    pub params: GenerationParams,
    pub requested_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFeatures {
    pub bpm: f64,
    pub key: String,
    pub spectral_centroid: f64,
    pub spectral_rolloff: f64,
    pub spectral_flux: f64,
    pub rms_energy: f64,
    pub zero_crossing_rate: f64,
    pub estimated_mood: Vec<(String, f64)>,
    pub estimated_activity: Vec<(String, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackSimilarity {
    pub track_id_1: String,
    pub track_id_2: String,
    pub similarity_score: f64,
}

impl Default for GenerationParams {
    fn default() -> Self {
        Self {
            mood: None,
            activity: None,
            min_bpm: None,
            max_bpm: None,
            target_key: None,
            genres: vec![],
            max_duration_seconds: None,
            similarity_weight: 0.5,
            diversity_weight: 0.3,
            popularity_weight: 0.2,
        }
    }
}

impl Track {
    pub fn new(title: String, artist: String, duration_seconds: f64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            artist,
            album: None,
            duration_seconds,
            file_path: None,
            file_hash: None,
            bpm: None,
            key: None,
            spectral_centroid: None,
            spectral_rolloff: None,
            spectral_flux: None,
            rms_energy: None,
            zero_crossing_rate: None,
            mood_tags: vec![],
            activity_tags: vec![],
            genres: vec![],
            added_at: Utc::now(),
        }
    }
}

impl Playlist {
    pub fn new(name: String, description: Option<String>, generation_params: GenerationParams) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            tracks: vec![],
            generation_params,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
