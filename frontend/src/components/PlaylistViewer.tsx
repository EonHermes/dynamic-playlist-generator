import React from 'react';
import type { Playlist, Track } from '../types';
import { Clock, Music } from 'lucide-react';

interface PlaylistViewerProps {
  playlist: Playlist;
  tracks: Track[];
}

const PlaylistViewer: React.FC<PlaylistViewerProps> = ({ playlist, tracks }) => {
  const playlistTracks = playlist.tracks
    .map(trackId => tracks.find(t => t.id === trackId))
    .filter((t): t is NonNullable<typeof t> => t !== undefined);

  const totalDuration = playlistTracks.reduce((acc, t) => acc + t.duration_seconds, 0);
  const formatDuration = (seconds: number) => {
    const hours = Math.floor(seconds / 3600);
    const mins = Math.floor((seconds % 3600) / 60);
    return hours > 0 ? `${hours}h ${mins}m` : `${mins}m`;
  };

  return (
    <div className="playlist-viewer">
      <div className="playlist-header">
        <h2>{playlist.name}</h2>
        {playlist.description && <p className="description">{playlist.description}</p>}
        <div className="playlist-meta">
          <span><Music size={16} /> {playlistTracks.length} tracks</span>
          <span><Clock size={16} /> {formatDuration(totalDuration)}</span>
          <span>Generated with AI</span>
        </div>
        {playlist.generation_params && (
          <div className="generation-params">
            <h4>Generation Parameters:</h4>
            <ul>
              {playlist.generation_params.mood && <li>Mood: {playlist.generation_params.mood}</li>}
              {playlist.generation_params.activity && <li>Activity: {playlist.generation_params.activity}</li>}
              {playlist.generation_params.min_bpm && <li>Min BPM: {playlist.generation_params.min_bpm}</li>}
              {playlist.generation_params.max_bpm && <li>Max BPM: {playlist.generation_params.max_bpm}</li>}
              {playlist.generation_params.target_key && <li>Key: {playlist.generation_params.target_key}</li>}
              {playlist.generation_params.genres.length > 0 && (
                <li>Genres: {playlist.generation_params.genres.join(', ')}</li>
              )}
            </ul>
          </div>
        )}
      </div>

      <div className="playlist-tracks">
        {playlistTracks.map((track, index) => (
          <div key={track.id} className="playlist-track-item">
            <div className="track-number">{index + 1}</div>
            <div className="track-details">
              <h4>{track.title}</h4>
              <p>{track.artist}</p>
              {track.album && <small>{track.album}</small>}
            </div>
            <div className="track-specs">
              {track.bpm && <span>{track.bpm.toFixed(0)} BPM</span>}
              {track.key && <span>{track.key}</span>}
              <span className="duration">{Math.floor(track.duration_seconds / 60)}:{(track.duration_seconds % 60).toFixed(0).padStart(2, '0')}</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default PlaylistViewer;
