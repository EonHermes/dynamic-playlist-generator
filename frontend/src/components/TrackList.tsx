import React, { useState, useEffect } from 'react';
import type { Track } from '../types';

interface TrackListProps {
  onTrackSelect?: (track: Track) => void;
  selectable?: boolean;
}

const TrackList: React.FC<TrackListProps> = ({ onTrackSelect, selectable = false }) => {
  const [tracks, setTracks] = useState<Track[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchTracks();
  }, []);

  const fetchTracks = async () => {
    try {
      const response = await fetch('http://localhost:8080/api/tracks?limit=100');
      if (response.ok) {
        const data = await response.json();
        setTracks(data);
      }
    } catch (error) {
      console.error('Failed to fetch tracks:', error);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return <div className="loading">Loading tracks...</div>;
  }

  return (
    <div className="track-list">
      <h2>Track Library ({tracks.length})</h2>
      <div className="tracks-container">
        {tracks.map(track => (
          <div
            key={track.id}
            className={`track-card ${selectable ? 'selectable' : ''}`}
            onClick={() => selectable && onTrackSelect?.(track)}
          >
            <div className="track-info">
              <h3>{track.title}</h3>
              <p>{track.artist}</p>
              {track.album && <small>{track.album}</small>}
            </div>
            <div className="track-features">
              {track.bpm && <span>BPM: {track.bpm.toFixed(1)}</span>}
              {track.key && <span>Key: {track.key}</span>}
              {track.rms_energy && <span>Energy: {(track.rms_energy * 100).toFixed(0)}%</span>}
              {track.mood_tags.length > 0 && (
                <div className="tags">
                  {track.mood_tags.slice(0, 3).map(tag => (
                    <span key={tag} className="tag mood">{tag}</span>
                  ))}
                </div>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default TrackList;
