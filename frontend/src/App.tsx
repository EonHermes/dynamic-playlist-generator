import { useState, useEffect } from 'react';
import TrackList from './components/TrackList';
import PlaylistGenerator from './components/PlaylistGenerator';
import PlaylistViewer from './components/PlaylistViewer';
import type { Track, Playlist } from './types';
import { Disc3, ListMusic, Wand2 } from 'lucide-react';
import './App.css';

function App() {
  const [tracks, setTracks] = useState<Track[]>([]);
  const [playlists, setPlaylists] = useState<Playlist[]>([]);
  const [selectedPlaylist, setSelectedPlaylist] = useState<Playlist | null>(null);
  const [activeTab, setActiveTab] = useState<'library' | 'generate' | 'playlists'>('library');
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchTracks();
    fetchPlaylists();
  }, []);

  const fetchTracks = async () => {
    try {
      const response = await fetch('http://localhost:8080/api/tracks?limit=200');
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

  const fetchPlaylists = async () => {
    try {
      const response = await fetch('http://localhost:8080/api/playlists?limit=50');
      if (response.ok) {
        const data = await response.json();
        setPlaylists(data);
        if (data.length > 0 && !selectedPlaylist) {
          setSelectedPlaylist(data[0]);
        }
      }
    } catch (error) {
      console.error('Failed to fetch playlists:', error);
    }
  };

  const handleTrackSelect = (track: Track) => {
    alert(`Selected: ${track.title} by ${track.artist}\n\n(You would add this as a seed track in the generator)`);
  };

  const handlePlaylistGenerated = (newPlaylist: Playlist) => {
    setPlaylists(prev => [newPlaylist, ...prev]);
    setSelectedPlaylist(newPlaylist);
    setActiveTab('playlists');
  };

  const addTrackForDemo = async () => {
    const demoTracks = [
      { title: "Bohemian Rhapsody", artist: "Queen", duration_seconds: 354, bpm: 72, key: "Bb", mood_tags: ["dramatic"], activity_tags: ["focus"], genres: ["rock"] },
      { title: "Levitating", artist: "Dua Lipa", duration_seconds: 203, bpm: 103, key: "Bm", mood_tags: ["energetic"], activity_tags: ["workout"], genres: ["pop"] },
      { title: "Blinding Lights", artist: "The Weeknd", duration_seconds: 200, bpm: 171, key: "F#m", mood_tags: ["rhythmic"], activity_tags: ["party"], genres: ["pop", "electronic"] },
      { title: "Shape of You", artist: "Ed Sheeran", duration_seconds: 233, bpm: 96, key: "C#m", mood_tags: ["bright"], activity_tags: ["commute"], genres: ["pop"]},
      { title: "Hotel California", artist: "Eagles", duration_seconds: 391, bpm: 75, key: "Bm", mood_tags: ["atmospheric"], activity_tags: ["relax"], genres: ["rock"] },
    ];

    for (const trackData of demoTracks) {
      const response = await fetch('http://localhost:8080/api/tracks', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(trackData)
      });
      if (response.ok) {
        console.log(`Added: ${trackData.title}`);
      }
    }
    fetchTracks();
    alert('Demo tracks added! You can now generate playlists.');
  };

  return (
    <div className="app">
      <header>
        <div className="logo">
          <Disc3 size={32} />
          <h1>Dynamic Playlist Generator</h1>
        </div>
        <p className="tagline">AI-powered music playlist creation with Rust & React</p>
      </header>

      <nav className="tabs">
        <button
          className={`tab ${activeTab === 'library' ? 'active' : ''}`}
          onClick={() => setActiveTab('library')}
        >
          <ListMusic size={18} />
          Library
          <span className="badge">{tracks.length}</span>
        </button>
        <button
          className={`tab ${activeTab === 'generate' ? 'active' : ''}`}
          onClick={() => setActiveTab('generate')}
        >
          <Wand2 size={18} />
          Generate
        </button>
        <button
          className={`tab ${activeTab === 'playlists' ? 'active' : ''}`}
          onClick={() => setActiveTab('playlists')}
        >
          <Disc3 size={18} />
          Playlists
          <span className="badge">{playlists.length}</span>
        </button>
      </nav>

      <main>
        {loading ? (
          <div className="loading">Loading...</div>
        ) : (
          <>
            {activeTab === 'library' && (
              <div className="library-section">
                <div className="section-header">
                  <h2>Your Music Library</h2>
                  <button className="demo-button" onClick={addTrackForDemo}>
                    Add Demo Tracks
                  </button>
                </div>
                <p className="help-text">
                  Add tracks to your library. Each track should have audio features like BPM, key, and mood tags.
                  Tracks can be added via the API or the Demo button.
                </p>
                <TrackList onTrackSelect={handleTrackSelect} selectable />
              </div>
            )}

            {activeTab === 'generate' && (
              <div className="generate-section">
                <PlaylistGenerator
                  availableTracks={tracks}
                  onPlaylistGenerated={handlePlaylistGenerated}
                />
              </div>
            )}

            {activeTab === 'playlists' && (
              <div className="playlists-section">
                <div className="section-header">
                  <h2>Generated Playlists</h2>
                </div>
                {playlists.length === 0 ? (
                  <div className="empty-state">
                    <p>No playlists yet. Generate one to get started!</p>
                  </div>
                ) : (
                  <div className="playlists-grid">
                    <div className="playlist-selector">
                      <h3>Select a Playlist</h3>
                      {playlists.map(playlist => (
                        <div
                          key={playlist.id}
                          className={`playlist-option ${selectedPlaylist?.id === playlist.id ? 'selected' : ''}`}
                          onClick={() => setSelectedPlaylist(playlist)}
                        >
                          {playlist.name}
                        </div>
                      ))}
                    </div>
                    {selectedPlaylist && (
                      <PlaylistViewer
                        playlist={selectedPlaylist}
                        tracks={tracks}
                      />
                    )}
                  </div>
                )}
              </div>
            )}
          </>
        )}
      </main>

      <footer>
        <p>Built with Rust (Actix-web) + React (Vite + TypeScript)</p>
        <p>Audio feature extraction with hound + custom DSP</p>
      </footer>
    </div>
  );
}

export default App;
