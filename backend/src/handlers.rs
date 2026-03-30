use crate::{db::Database, models::{Track, Playlist, PlaylistGenerationRequest, GenerationParams, AudioFeatures}};
use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use std::path::PathBuf;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok", "service": "dynamic-playlist-generator" }))
}

pub async fn upload_track(
    db: web::Data<Database>,
    payload: web::Json<TrackUploadRequest>,
) -> impl Responder {
    let file_path = if let Some(ref path) = payload.file_path {
        PathBuf::from(path)
    } else {
        return HttpResponse::BadRequest().json(serde_json::json!({ "error": "file_path required" }));
    };

    let mut track = Track::new(payload.title.clone(), payload.artist.clone(), payload.duration_seconds);
    track.album = payload.album.clone();
    if let Some(ref path) = payload.file_path {
        track.file_path = Some(path.clone());
    }

    if file_path.exists() {
        if let Ok(features) = crate::audio::extract_features_from_track(&mut track, &file_path) {
            // Features extracted successfully
        } else {
            return HttpResponse::BadRequest().json(serde_json::json!({ "error": "Failed to analyze audio file" }));
        }
    } else {
        track.mood_tags = payload.mood_tags.clone().unwrap_or_default();
        track.activity_tags = payload.activity_tags.clone().unwrap_or_default();
        track.genres = payload.genres.clone().unwrap_or_default();
        if let Some(bpm) = payload.bpm { track.bpm = Some(bpm); }
        if let Some(key) = payload.key.clone() { track.key = Some(key); }
        if let Some(cent) = payload.spectral_centroid { track.spectral_centroid = Some(cent); }
        if let Some(roll) = payload.spectral_rolloff { track.spectral_rolloff = Some(roll); }
        if let Some(flux) = payload.spectral_flux { track.spectral_flux = Some(flux); }
        if let Some(rms) = payload.rms_energy { track.rms_energy = Some(rms); }
        if let Some(zcr) = payload.zero_crossing_rate { track.zero_crossing_rate = Some(zcr); }
    }

    track.file_hash = payload.file_hash.clone();

    if let Err(e) = db.save_track(&track) {
        return HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() }));
    }

    HttpResponse::Created().json(track)
}

pub async fn get_track(
    db: web::Data<Database>,
    track_id: web::Path<String>,
) -> impl Responder {
    match db.get_track(&track_id) {
        Ok(Some(track)) => HttpResponse::Ok().json(track),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({ "error": "Track not found" })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn list_tracks(
    db: web::Data<Database>,
    query: web::Query<TrackListQuery>,
) -> impl Responder {
    let limit = query.limit.unwrap_or(100).min(1000);
    match db.list_tracks(Some(limit)) {
        Ok(tracks) => HttpResponse::Ok().json(tracks),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn create_playlist(
    db: web::Data<Database>,
    payload: web::Json<PlaylistGenerationRequest>,
) -> impl Responder {
    let generator = crate::services::PlaylistGenerator::new(db.get_ref().clone());
    match generator.generate_playlist(payload.into_inner()) {
        Ok(playlist) => HttpResponse::Created().json(playlist),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn get_playlist(
    db: web::Data<Database>,
    playlist_id: web::Path<String>,
) -> impl Responder {
    match db.get_playlist(&playlist_id) {
        Ok(Some(playlist)) => HttpResponse::Ok().json(playlist),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({ "error": "Playlist not found" })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn list_playlists(
    db: web::Data<Database>,
    query: web::Query<PlaylistListQuery>,
) -> impl Responder {
    let limit = query.limit.unwrap_or(50).min(500);
    match db.list_playlists(Some(limit)) {
        Ok(playlists) => HttpResponse::Ok().json(playlists),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn recompute_similarities(db: web::Data<Database>) -> impl Responder {
    use rayon::prelude::*;
    let tracks = match db.list_tracks(None) {
        Ok(t) => t,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() })),
    };

    let pairs: Vec<(usize, usize)> = (0..tracks.len())
        .flat_map(|i| ((i+1)..tracks.len()).map(move |j| (i, j)))
        .collect();

    let results: Vec<_> = pairs.par_iter()
        .map(|(i, j)| {
            let sim = crate::audio::compute_similarity(&tracks[*i], &tracks[*j]);
            ((&tracks[*i]).id.clone(), (&tracks[*j]).id.clone(), sim)
        })
        .collect();

    for (id1, id2, score) in results {
        let _ = db.save_similarity(&id1, &id2, score);
    }

    HttpResponse::Ok().json(serde_json::json!({ "status": "success", "pairs_computed": pairs.len() }))
}

#[derive(serde::Deserialize)]
pub struct TrackUploadRequest {
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
    pub mood_tags: Option<Vec<String>>,
    pub activity_tags: Option<Vec<String>>,
    pub genres: Option<Vec<String>>,
}

#[derive(serde::Deserialize)]
pub struct TrackListQuery {
    pub limit: Option<usize>,
}

#[derive(serde::Deserialize)]
pub struct PlaylistListQuery {
    pub limit: Option<usize>,
}
