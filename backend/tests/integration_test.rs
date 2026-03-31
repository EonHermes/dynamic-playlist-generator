use dynamic_playlist_generator::db::Database;
use dynamic_playlist_generator::services::PlaylistGenerator;
use dynamic_playlist_generator::models::{Track, PlaylistGenerationRequest, GenerationParams};
use tempfile::tempdir;
use std::fs;

#[tokio::test]
async fn test_database_creation_and_track_storage() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    
    let db = Database::new(db_path.to_str().unwrap()).unwrap();
    
    let track = Track::new(
        "Test Song".to_string(),
        "Test Artist".to_string(),
        180.0
    );
    
    db.save_track(&track).unwrap();
    
    let retrieved = db.get_track(&track.id).unwrap().unwrap();
    assert_eq!(retrieved.title, "Test Song");
    assert_eq!(retrieved.artist, "Test Artist");
    assert_eq!(retrieved.duration_seconds, 180.0);
}

#[tokio::test]
async fn test_playlist_generation_with_seed_tracks() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = Database::new(db_path.to_str().unwrap()).unwrap();
    
    // Create test tracks with varying features
    let mut track1 = Track::new("Energetic Track".to_string(), "Artist A".to_string(), 200.0);
    track1.bpm = Some(140.0);
    track1.rms_energy = Some(0.4);
    track1.mood_tags = vec!["energetic".to_string()];
    track1.activity_tags = vec!["workout".to_string()];
    
    let mut track2 = Track::new("Calm Track".to_string(), "Artist B".to_string(), 240.0);
    track2.bpm = Some(70.0);
    track2.rms_energy = Some(0.1);
    track2.mood_tags = vec!["calm".to_string()];
    track2.activity_tags = vec!["relax".to_string()];
    
    let mut track3 = Track::new("Medium Track".to_string(), "Artist C".to_string(), 180.0);
    track3.bpm = Some(100.0);
    track3.rms_energy = Some(0.25);
    track3.mood_tags = vec!["bright".to_string()];
    track3.activity_tags = vec!["focus".to_string()];
    
    db.save_track(&track1).unwrap();
    db.save_track(&track2).unwrap();
    db.save_track(&track3).unwrap();
    
    let generator = PlaylistGenerator::new(db.clone());
    
    // Generate playlist using track1 as seed
    let request = PlaylistGenerationRequest {
        name: "Workout Mix".to_string(),
        description: Some("Generated from seed track".to_string()),
        seed_tracks: vec![track1.id.clone()],
        params: GenerationParams::default(),
        requested_length: 3,
    };
    
    let playlist = generator.generate_playlist(request).unwrap();
    
    assert_eq!(playlist.name, "Workout Mix");
    assert_eq!(playlist.tracks.len(), 3);
    assert!(playlist.tracks.contains(&track1.id));
    
    // Verify playlist saved
    let retrieved = db.get_playlist(&playlist.id).unwrap().unwrap();
    assert_eq!(retrieved.name, "Workout Mix");
}

#[tokio::test]
async fn test_playlist_generation_by_mood() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = Database::new(db_path.to_str().unwrap()).unwrap();
    
    // Create tracks with mood tags
    let mut track1 = Track::new("Party Song".to_string(), "DJ".to_string(), 210.0);
    track1.bpm = Some(128.0);
    track1.rms_energy = Some(0.35);
    track1.mood_tags = vec!["energetic".to_string()];
    track1.activity_tags = vec!["party".to_string()];
    
    let mut track2 = Track::new("Study Song".to_string(), "Ambient".to_string(), 300.0);
    track2.bpm = Some(60.0);
    track2.rms_energy = Some(0.05);
    track2.mood_tags = vec!["calm".to_string()];
    track2.activity_tags = vec!["study".to_string()];
    
    db.save_track(&track1).unwrap();
    db.save_track(&track2).unwrap();
    
    let generator = PlaylistGenerator::new(db.clone());
    
    let request = PlaylistGenerationRequest {
        name: "Party Playlist".to_string(),
        description: None,
        seed_tracks: vec![],
        params: GenerationParams {
            mood: Some("energetic".to_string()),
            activity: None,
            min_bpm: None,
            max_bpm: None,
            target_key: None,
            genres: vec![],
            max_duration_seconds: None,
            similarity_weight: 0.5,
            diversity_weight: 0.3,
            popularity_weight: 0.2,
        },
        requested_length: 2,
    };
    
    let playlist = generator.generate_playlist(request).unwrap();
    assert!(playlist.tracks.contains(&track1.id));
    assert!(!playlist.tracks.contains(&track2.id));
}

#[tokio::test]
async fn test_similarity_computation() {
    use dynamic_playlist_generator::audio::compute_similarity;
    
    let mut track_a = Track::new("Song A".to_string(), "Artist".to_string(), 180.0);
    track_a.bpm = Some(120.0);
    track_a.spectral_centroid = Some(500.0);
    track_a.rms_energy = Some(0.3);
    track_a.mood_tags = vec!["energetic".to_string()];
    track_a.activity_tags = vec!["workout".to_string()];
    
    let mut track_b = track_a.clone();
    track_b.id = "different-id".to_string();
    track_b.title = "Song B".to_string();
    
    let mut track_c = Track::new("Song C".to_string(), "Other".to_string(), 200.0);
    track_c.bpm = Some(80.0);
    track_c.spectral_centroid = Some(200.0);
    track_c.rms_energy = Some(0.1);
    track_c.mood_tags = vec!["calm".to_string()];
    track_c.activity_tags = vec!["relax".to_string()];
    
    let similarity_ab = compute_similarity(&track_a, &track_b);
    let similarity_ac = compute_similarity(&track_a, &track_c);
    
    // Same tracks should have perfect similarity (1.0)
    assert!((similarity_ab - 1.0).abs() < 0.01);
    
    // Different tracks should have lower similarity
    assert!(similarity_ac < similarity_ab);
}
