use crate::{db::Database, models::{Track, Playlist, GenerationParams, PlaylistGenerationRequest}};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct PlaylistGenerator {
    db: Database,
}

impl PlaylistGenerator {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn generate_playlist(&self, request: PlaylistGenerationRequest) -> Result<Playlist, Box<dyn std::error::Error>> {
        let mut playlist = Playlist::new(request.name, request.description, request.params.clone());

        let seed_tracks: Vec<Track> = request.seed_tracks.iter()
            .filter_map(|id| self.db.get_track(id).ok().flatten())
            .collect();

        if seed_tracks.is_empty() && !request.params.mood.is_none() {
            let matching_mood = self.db.list_tracks(None)?
                .into_iter()
                .filter(|t| t.mood_tags.iter().any(|tag| tag == request.params.mood.as_ref().unwrap()))
                .collect::<Vec<_>>();
            if !matching_mood.is_empty() {
                playlist.tracks = self.select_diverse_tracks(&matching_mood, request.requested_length, &request.params);
            } else {
                playlist.tracks = self.select_random_tracks(request.requested_length);
            }
        } else if !seed_tracks.is_empty() {
            playlist.tracks = self.expand_from_seeds(&seed_tracks, request.requested_length, &request.params);
        } else {
            playlist.tracks = self.select_random_tracks(request.requested_length);
        }

        playlist.updated_at = chrono::Utc::now();
        self.db.save_playlist(&playlist)?;
        Ok(playlist)
    }

    fn expand_from_seeds(&self, seeds: &[Track], target_length: usize, params: &GenerationParams) -> Vec<String> {
        if seeds.is_empty() {
            return vec![];
        }

        let mut selected = seeds.iter().map(|t| t.id.clone()).collect::<Vec<_>>();
        let mut candidates = self.db.list_tracks(None)
            .ok()
            .unwrap_or_default()
            .into_iter()
            .filter(|t| !selected.contains(&t.id))
            .collect::<Vec<_>>();

        candidates.retain(|t| {
            if let Some(min) = params.min_bpm {
                if t.bpm.unwrap_or(120.0) < min { return false; }
            }
            if let Some(max) = params.max_bpm {
                if t.bpm.unwrap_or(120.0) > max { return false; }
            }
            if let Some(key) = &params.target_key {
                if t.key.as_ref().map(|k| k != key).unwrap_or(false) { return false; }
            }
            if !params.genres.is_empty() {
                if !t.genres.iter().any(|g| params.genres.contains(g)) { return false; }
            }
            if let Some(max_dur) = params.max_duration_seconds {
                if t.duration_seconds > max_dur { return false; }
            }
            true
        });

        let mut rng = thread_rng();

        while selected.len() < target_length && !candidates.is_empty() {
            let seed = seeds.choose(&mut rng).unwrap();
            let mut scored: Vec<(f64, Track)> = candidates.iter()
                .map(|t| {
                    let sim = crate::audio::compute_similarity(seed, t);
                    let diversity_penalty = selected.iter()
                        .filter(|sid| {
                            let selected_track = self.db.get_track(sid).ok().flatten();
                            if let Some(st) = selected_track {
                                crate::audio::compute_similarity(t, &st) > 0.7
                            } else { false }
                        })
                        .count() as f64 * 0.3;
                    let score = sim * params.similarity_weight - diversity_penalty * params.diversity_weight;
                    (score, t.clone())
                })
                .filter(|(score, _)| *score > 0.2)
                .collect();

            scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

            let chosen = if scored.is_empty() {
                candidates.pop()
            } else {
                scored.into_iter().next().map(|(_, t)| t)
            };

            if let Some(chosen_track) = chosen {
                selected.push(chosen_track.id.clone());
                candidates.retain(|t| t.id != chosen_track.id);
            } else {
                break;
            }
        }

        selected
    }

    fn select_diverse_tracks(&self, pool: &[Track], count: usize, _params: &GenerationParams) -> Vec<String> {
        let mut selected = Vec::new();
        let mut rng = thread_rng();
        let mut pool = pool.to_vec();

        pool.shuffle(&mut rng);
        for track in pool.iter().take(count) {
            selected.push(track.id.clone());
        }

        selected
    }

    fn select_random_tracks(&self, count: usize) -> Vec<String> {
        let tracks = self.db.list_tracks(None).unwrap_or_default();
        let mut rng = thread_rng();
        tracks.choose_multiple(&mut rng, count.min(tracks.len()))
            .map(|t| t.id.clone())
            .collect()
    }
}
