use rusqlite::{Connection, Result as SqlResult};
use chrono::{DateTime, Utc};
use std::sync::Mutex;
use crate::models::{Track, Playlist};

#[derive(Clone)]
pub struct Database {
    conn: std::sync::Arc<std::sync::Mutex<Connection>>,
}

impl Database {
    pub fn new(db_path: &str) -> SqlResult<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS tracks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                artist TEXT NOT NULL,
                album TEXT,
                duration_seconds REAL NOT NULL,
                file_path TEXT,
                file_hash TEXT,
                bpm REAL,
                key TEXT,
                spectral_centroid REAL,
                spectral_rolloff REAL,
                spectral_flux REAL,
                rms_energy REAL,
                zero_crossing_rate REAL,
                mood_tags TEXT,
                activity_tags TEXT,
                genres TEXT,
                added_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS playlists (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                generation_params TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS playlist_tracks (
                playlist_id TEXT NOT NULL,
                track_id TEXT NOT NULL,
                position INTEGER NOT NULL,
                PRIMARY KEY (playlist_id, track_id),
                FOREIGN KEY (playlist_id) REFERENCES playlists(id),
                FOREIGN KEY (track_id) REFERENCES tracks(id)
            );

            CREATE TABLE IF NOT EXISTS track_similarities (
                track_id_1 TEXT NOT NULL,
                track_id_2 TEXT NOT NULL,
                similarity_score REAL NOT NULL,
                PRIMARY KEY (track_id_1, track_id_2),
                FOREIGN KEY (track_id_1) REFERENCES tracks(id),
                FOREIGN KEY (track_id_2) REFERENCES tracks(id)
            );

            CREATE INDEX IF NOT EXISTS idx_tracks_artist ON tracks(artist);
            CREATE INDEX IF NOT EXISTS idx_tracks_bpm ON tracks(bpm);
            CREATE INDEX IF NOT EXISTS idx_tracks_genres ON tracks(genres);
            CREATE INDEX IF NOT EXISTS idx_playlist_created ON playlists(created_at);
            "#,
        )?;
        Ok(Self { conn: std::sync::Arc::new(std::sync::Mutex::new(conn)) })
    }

    pub fn save_track(&self, track: &Track) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO tracks (id, title, artist, album, duration_seconds, file_path, file_hash, bpm, key, spectral_centroid, spectral_rolloff, spectral_flux, rms_energy, zero_crossing_rate, mood_tags, activity_tags, genres, added_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
            rusqlite::params![
                track.id,
                track.title,
                track.artist,
                track.album,
                track.duration_seconds,
                track.file_path,
                track.file_hash,
                track.bpm,
                track.key,
                track.spectral_centroid,
                track.spectral_rolloff,
                track.spectral_flux,
                track.rms_energy,
                track.zero_crossing_rate,
                serde_json::to_string(&track.mood_tags).unwrap_or_default(),
                serde_json::to_string(&track.activity_tags).unwrap_or_default(),
                serde_json::to_string(&track.genres).unwrap_or_default(),
                track.added_at.to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn get_track(&self, track_id: &str) -> SqlResult<Option<Track>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, artist, album, duration_seconds, file_path, file_hash, bpm, key, spectral_centroid, spectral_rolloff, spectral_flux, rms_energy, zero_crossing_rate, mood_tags, activity_tags, genres, added_at FROM tracks WHERE id = ?1"
        )?;

        let mut rows = stmt.query(rusqlite::params![track_id])?;
        if let Some(row) = rows.next()? {
            let track = Track {
                id: row.get::<_, String>(0)?,
                title: row.get::<_, String>(1)?,
                artist: row.get::<_, String>(2)?,
                album: row.get::<_, Option<String>>(3)?,
                duration_seconds: row.get::<_, f64>(4)?,
                file_path: row.get::<_, Option<String>>(5)?,
                file_hash: row.get::<_, Option<String>>(6)?,
                bpm: row.get::<_, Option<f64>>(7)?,
                key: row.get::<_, Option<String>>(8)?,
                spectral_centroid: row.get::<_, Option<f64>>(9)?,
                spectral_rolloff: row.get::<_, Option<f64>>(10)?,
                spectral_flux: row.get::<_, Option<f64>>(11)?,
                rms_energy: row.get::<_, Option<f64>>(12)?,
                zero_crossing_rate: row.get::<_, Option<f64>>(13)?,
                mood_tags: serde_json::from_str(&row.get::<_, String>(14)?).unwrap_or_default(),
                activity_tags: serde_json::from_str(&row.get::<_, String>(15)?).unwrap_or_default(),
                genres: serde_json::from_str(&row.get::<_, String>(16)?).unwrap_or_default(),
                added_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(17)?)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(17, rusqlite::types::Type::Text, Box::new(e)))?
                    .with_timezone(&Utc),
            };
            Ok(Some(track))
        } else {
            Ok(None)
        }
    }

    pub fn list_tracks(&self, limit: Option<usize>) -> SqlResult<Vec<Track>> {
        let conn = self.conn.lock().unwrap();
        let query = if let Some(limit) = limit {
            format!("SELECT * FROM tracks ORDER BY added_at DESC LIMIT {}", limit)
        } else {
            "SELECT * FROM tracks ORDER BY added_at DESC".to_string()
        };

        let mut stmt = conn.prepare(&query)?;
        let mut rows = stmt.query([])?;

        let mut tracks = Vec::new();
        while let Some(row) = rows.next()? {
            let track = Track {
                id: row.get::<_, String>(0)?,
                title: row.get::<_, String>(1)?,
                artist: row.get::<_, String>(2)?,
                album: row.get::<_, Option<String>>(3)?,
                duration_seconds: row.get::<_, f64>(4)?,
                file_path: row.get::<_, Option<String>>(5)?,
                file_hash: row.get::<_, Option<String>>(6)?,
                bpm: row.get::<_, Option<f64>>(7)?,
                key: row.get::<_, Option<String>>(8)?,
                spectral_centroid: row.get::<_, Option<f64>>(9)?,
                spectral_rolloff: row.get::<_, Option<f64>>(10)?,
                spectral_flux: row.get::<_, Option<f64>>(11)?,
                rms_energy: row.get::<_, Option<f64>>(12)?,
                zero_crossing_rate: row.get::<_, Option<f64>>(13)?,
                mood_tags: serde_json::from_str(&row.get::<_, String>(14)?).unwrap_or_default(),
                activity_tags: serde_json::from_str(&row.get::<_, String>(15)?).unwrap_or_default(),
                genres: serde_json::from_str(&row.get::<_, String>(16)?).unwrap_or_default(),
                added_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(17)?)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(17, rusqlite::types::Type::Text, Box::new(e)))?
                    .with_timezone(&Utc),
            };
            tracks.push(track);
        }
        Ok(tracks)
    }

    pub fn save_playlist(&self, playlist: &Playlist) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO playlists (id, name, description, generation_params, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                playlist.id,
                playlist.name,
                playlist.description,
                serde_json::to_string(&playlist.generation_params).unwrap_or_default(),
                playlist.created_at.to_rfc3339(),
                playlist.updated_at.to_rfc3339()
            ],
        )?;

        conn.execute(
            "DELETE FROM playlist_tracks WHERE playlist_id = ?1",
            rusqlite::params![playlist.id]
        )?;

        for (pos, track_id) in playlist.tracks.iter().enumerate() {
            conn.execute(
                "INSERT INTO playlist_tracks (playlist_id, track_id, position) VALUES (?1, ?2, ?3)",
                rusqlite::params![playlist.id, track_id, pos as i32]
            )?;
        }

        Ok(())
    }

    pub fn get_playlist(&self, playlist_id: &str) -> SqlResult<Option<Playlist>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, generation_params, created_at, updated_at FROM playlists WHERE id = ?1"
        )?;

        let mut rows = stmt.query(rusqlite::params![playlist_id])?;
        if let Some(row) = rows.next()? {
            let mut playlist = Playlist {
                id: row.get::<_, String>(0)?,
                name: row.get::<_, String>(1)?,
                description: row.get::<_, Option<String>>(2)?,
                tracks: vec![],
                generation_params: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(4, rusqlite::types::Type::Text, Box::new(e)))?
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(5, rusqlite::types::Type::Text, Box::new(e)))?
                    .with_timezone(&Utc),
            };

            let mut track_stmt = conn.prepare(
                "SELECT track_id FROM playlist_tracks WHERE playlist_id = ?1 ORDER BY position ASC"
            )?;
            let mut track_rows = track_stmt.query(rusqlite::params![playlist_id])?;
            while let Some(row) = track_rows.next()? {
                if let Ok(track_id) = row.get::<_, String>(0) {
                    playlist.tracks.push(track_id);
                }
            }

            return Ok(Some(playlist));
        }
        Ok(None)
    }

    pub fn list_playlists(&self, limit: Option<usize>) -> SqlResult<Vec<Playlist>> {
        let conn = self.conn.lock().unwrap();
        let query = if let Some(limit) = limit {
            format!("SELECT * FROM playlists ORDER BY created_at DESC LIMIT {}", limit)
        } else {
            "SELECT * FROM playlists ORDER BY created_at DESC".to_string()
        };

        let mut stmt = conn.prepare(&query)?;
        let mut rows = stmt.query([])?;

        let mut playlists = Vec::new();
        while let Some(row) = rows.next()? {
            let mut playlist = Playlist {
                id: row.get::<_, String>(0)?,
                name: row.get::<_, String>(1)?,
                description: row.get::<_, Option<String>>(2)?,
                tracks: vec![],
                generation_params: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(4, rusqlite::types::Type::Text, Box::new(e)))?
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(5, rusqlite::types::Type::Text, Box::new(e)))?
                    .with_timezone(&Utc),
            };

            let mut track_stmt = conn.prepare(
                "SELECT track_id FROM playlist_tracks WHERE playlist_id = ?1 ORDER BY position ASC"
            )?;
            let mut track_rows = track_stmt.query(rusqlite::params![&playlist.id])?;
            while let Some(trow) = track_rows.next()? {
                if let Ok(track_id) = trow.get::<_, String>(0) {
                    playlist.tracks.push(track_id);
                }
            }

            playlists.push(playlist);
        }
        Ok(playlists)
    }

    pub fn save_similarity(&self, track_id_1: &str, track_id_2: &str, score: f64) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO track_similarities (track_id_1, track_id_2, similarity_score) VALUES (?1, ?2, ?3)",
            rusqlite::params![track_id_1, track_id_2, score]
        )?;
        Ok(())
    }

    pub fn get_similarities(&self, track_id: &str) -> SqlResult<Vec<(String, f64)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT track_id_2, similarity_score FROM track_similarities WHERE track_id_1 = ?1
             UNION
             SELECT track_id_1, similarity_score FROM track_similarities WHERE track_id_2 = ?1
             ORDER BY similarity_score DESC"
        )?;

        let mut rows = stmt.query(rusqlite::params![track_id])?;
        let mut results = Vec::new();
        while let Some(row) = rows.next()? {
            let (other_id, score) = (row.get::<_, String>(0)?, row.get::<_, f64>(1)?);
            results.push((other_id, score));
        }
        Ok(results)
    }
}
