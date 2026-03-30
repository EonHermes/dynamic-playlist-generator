export interface Track {
  id: string;
  title: string;
  artist: string;
  album?: string;
  duration_seconds: number;
  file_path?: string;
  file_hash?: string;
  bpm?: number;
  key?: string;
  spectral_centroid?: number;
  spectral_rolloff?: number;
  spectral_flux?: number;
  rms_energy?: number;
  zero_crossing_rate?: number;
  mood_tags: string[];
  activity_tags: string[];
  genres: string[];
  added_at: string;
}

export interface PlaylistGenParams {
  mood?: string;
  activity?: string;
  min_bpm?: number;
  max_bpm?: number;
  target_key?: string;
  genres: string[];
  max_duration_seconds?: number;
  similarity_weight: number;
  diversity_weight: number;
  popularity_weight: number;
}

export interface Playlist {
  id: string;
  name: string;
  description?: string;
  tracks: string[];
  generation_params: PlaylistGenParams;
  created_at: string;
  updated_at: string;
}

export interface PlaylistGenerationRequest {
  name: string;
  description?: string;
  seed_tracks: string[];
  params: PlaylistGenParams;
  requested_length: number;
}
