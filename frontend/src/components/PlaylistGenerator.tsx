import React, { useState } from 'react';
import type { PlaylistGenParams, Track } from '../types';
import { Sparkles, Music } from 'lucide-react';

interface PlaylistGeneratorProps {
  availableTracks: Track[];
  onPlaylistGenerated: (playlist: any) => void;
}

const PlaylistGenerator: React.FC<PlaylistGeneratorProps> = ({ availableTracks, onPlaylistGenerated }) => {
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [selectedTracks, setSelectedTracks] = useState<string[]>([]);
  const [length, setLength] = useState(20);
  const [params, setParams] = useState<PlaylistGenParams>({
    mood: '',
    activity: '',
    min_bpm: undefined,
    max_bpm: undefined,
    target_key: '',
    genres: [],
    max_duration_seconds: undefined,
    similarity_weight: 0.5,
    diversity_weight: 0.3,
    popularity_weight: 0.2
  });
  const [generating, setGenerating] = useState(false);

  const moods = ['energetic', 'calm', 'bright', 'dark', 'rhythmic', 'atmospheric'];
  const activities = ['workout', 'study', 'party', 'relax', 'focus', 'commute'];
  const genres = ['pop', 'rock', 'electronic', 'jazz', 'classical', 'hip-hop', 'ambient', 'folk'];

  const handleGenerate = async () => {
    if (!name.trim()) {
      alert('Please enter a playlist name');
      return;
    }

    if (availableTracks.length === 0) {
      alert('No tracks available. Please add tracks first.');
      return;
    }

    setGenerating(true);
    try {
      const request = {
        name,
        description: description || undefined,
        seed_tracks: selectedTracks,
        params,
        requested_length: length
      };

      const response = await fetch('http://localhost:8080/api/playlists', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(request)
      });

      if (response.ok) {
        const playlist = await response.json();
        onPlaylistGenerated(playlist);
      } else {
        alert('Failed to generate playlist');
      }
    } catch (error) {
      console.error('Error generating playlist:', error);
      alert('Error generating playlist');
    } finally {
      setGenerating(false);
    }
  };

  const toggleTrack = (trackId: string) => {
    setSelectedTracks(prev =>
      prev.includes(trackId) ? prev.filter(id => id !== trackId) : [...prev, trackId]
    );
  };

  const toggleGenre = (genre: string) => {
    setParams(prev => ({
      ...prev,
      genres: prev.genres.includes(genre)
        ? prev.genres.filter(g => g !== genre)
        : [...prev.genres, genre]
    }));
  };

  return (
    <div className="playlist-generator">
      <div className="generator-header">
        <Sparkles size={24} />
        <h2>Generate Playlist</h2>
      </div>

      <div className="form-section">
        <label>Playlist Name</label>
        <input
          type="text"
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder="My Awesome Playlist"
        />
      </div>

      <div className="form-section">
        <label>Description (optional)</label>
        <textarea
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          placeholder="What's this playlist for?"
          rows={2}
        />
      </div>

      <div className="form-section">
        <label>Target Length: {length} tracks</label>
        <input
          type="range"
          min="5"
          max="100"
          value={length}
          onChange={(e) => setLength(Number(e.target.value))}
        />
      </div>

      <div className="form-section">
        <label>Seed Tracks (optional - select to base playlist on)</label>
        <div className="track-select">
          {availableTracks.slice(0, 10).map(track => (
            <div
              key={track.id}
              className={`seed-track ${selectedTracks.includes(track.id) ? 'selected' : ''}`}
              onClick={() => toggleTrack(track.id)}
            >
              <Music size={14} />
              <span>{track.title} - {track.artist}</span>
            </div>
          ))}
          {availableTracks.length > 10 && (
            <small>...and {availableTracks.length - 10} more tracks</small>
          )}
        </div>
      </div>

      <div className="form-section">
        <label>Mood</label>
        <div className="tag-selector">
          {moods.map(mood => (
            <button
              key={mood}
              className={`tag ${params.mood === mood ? 'active' : ''}`}
              onClick={() => setParams(prev => ({ ...prev, mood: prev.mood === mood ? undefined : mood }))}
            >
              {mood}
            </button>
          ))}
        </div>
      </div>

      <div className="form-section">
        <label>Activity</label>
        <div className="tag-selector">
          {activities.map(activity => (
            <button
              key={activity}
              className={`tag ${params.activity === activity ? 'active' : ''}`}
              onClick={() => setParams(prev => ({ ...prev, activity: prev.activity === activity ? undefined : activity }))}
            >
              {activity}
            </button>
          ))}
        </div>
      </div>

      <div className="form-section">
        <label>Genres</label>
        <div className="tag-selector">
          {genres.map(genre => (
            <button
              key={genre}
              className={`tag ${params.genres.includes(genre) ? 'active' : ''}`}
              onClick={() => toggleGenre(genre)}
            >
              {genre}
            </button>
          ))}
        </div>
      </div>

      <div className="form-section row">
        <div className="slider-group">
          <label>BPM Range</label>
          <div className="bpm-inputs">
            <input
              type="number"
              placeholder="Min"
              value={params.min_bpm || ''}
              onChange={(e) => setParams(prev => ({ ...prev, min_bpm: e.target.value ? Number(e.target.value) : undefined }))}
            />
            <span>to</span>
            <input
              type="number"
              placeholder="Max"
              value={params.max_bpm || ''}
              onChange={(e) => setParams(prev => ({ ...prev, max_bpm: e.target.value ? Number(e.target.value) : undefined }))}
            />
          </div>
        </div>

        <div className="slider-group">
          <label>Similarity</label>
          <input
            type="range"
            min="0"
            max="1"
            step="0.1"
            value={params.similarity_weight}
            onChange={(e) => setParams(prev => ({ ...prev, similarity_weight: Number(e.target.value) }))}
          />
          <small>{(params.similarity_weight * 100).toFixed(0)}%</small>
        </div>

        <div className="slider-group">
          <label>Diversity</label>
          <input
            type="range"
            min="0"
            max="1"
            step="0.1"
            value={params.diversity_weight}
            onChange={(e) => setParams(prev => ({ ...prev, diversity_weight: Number(e.target.value) }))}
          />
          <small>{(params.diversity_weight * 100).toFixed(0)}%</small>
        </div>
      </div>

      <button
        className="generate-button"
        onClick={handleGenerate}
        disabled={generating}
      >
        {generating ? 'Generating...' : 'Generate Playlist'}
      </button>
    </div>
  );
};

export default PlaylistGenerator;
