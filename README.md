# Dynamic Playlist Generator

An intelligent music playlist generator that creates playlists based on mood, activity, musical features, and listening history. Built with Rust (Actix-web) for the backend and React (TypeScript + Vite) for the frontend.

## Features

### Backend (Rust)

- **Audio Analysis**: Extract audio features from WAV files including BPM, key, spectral centroid, rolloff, flux, RMS energy, and zero-crossing rate
- **Mood & Activity Classification**: Automatically tag tracks with mood (energetic, calm, bright, dark, rhythmic, atmospheric) and activity (workout, study, party, relax, focus, commute) labels
- **Track Similarity**: Compute multi-dimensional similarity scores between tracks based on audio features
- **Intelligent Playlist Generation**: Generate playlists using a weighted combination of:
  - **Similarity**: How well tracks match seed tracks or mood parameters
  - **Diversity**: Avoid repetitive artists or similar-sounding tracks
  - **Popularity**: (Future: track popularity metrics)
- **RESTful API**: Clean JSON API for track management and playlist generation
- **Persistent Storage**: SQLite database via `rusqlite` with proper schema and indexing
- **Parallel Processing**: Rayon for efficient similarity computation across track pairs

### Frontend (React + TypeScript)

- **Modern UI**: Clean, dark-themed interface with Tailwind-inspired CSS
- **Interactive Playlist Generator**: Slider controls and tag selectors for fine-tuning playlist generation
- **Track Library Visualization**: Browse your music library with audio feature displays
- **Playlist Viewer**: View generated playlists with track details, total duration, and generation parameters
- **Responsive Design**: Works on desktop and mobile

## Tech Stack

### Backend
- **Language**: Rust 2024 Edition
- **Web Framework**: Actix-web 4.x
- **Database**: SQLite via rusqlite with bundled library
- **Audio Processing**: hound (WAV reading), custom DSP for feature extraction
- **Parallelism**: rayon for data parallelism
- **Utilities**: serde (serialization), chrono (timestamps), uuid (IDs), rand (RNG)

### Frontend
- **Language**: TypeScript
- **Framework**: React 18
- **Build Tool**: Vite 5
- **Icons**: Lucide React
- **Charts**: Recharts (ready for future enhancements)
- **Styling**: CSS with CSS variables for theming

## Project Structure

```
dynamic-playlist-generator/
├── backend/
│   ├── src/
│   │   ├── main.rs          # Actix-web server entry point
│   │   ├── lib.rs           # Library exports
│   │   ├── db.rs            # Database access layer
│   │   ├── models.rs        # Data models (Track, Playlist, etc.)
│   │   ├── audio.rs         # Audio analysis & feature extraction
│   │   ├── services.rs      # Playlist generation logic
│   │   └── handlers.rs      # HTTP request handlers
│   ├── Cargo.toml
│   └── target/              # Build artifacts
├── frontend/
│   ├── src/
│   │   ├── App.tsx          # Main application component
│   │   ├── types.ts         # TypeScript type definitions
│   │   └── components/
│   │       ├── TrackList.tsx       # Track library display
│   │       ├── PlaylistGenerator.tsx  # Generation controls
│   │       └── PlaylistViewer.tsx # Playlist display
│   ├── public/
│   ├── index.html
│   ├── package.json
│   └── tsconfig.json
├── README.md
└── .gitignore
```

## Getting Started

### Prerequisites

- **Rust**: Install via rustup (https://rustup.rs/)
- **Node.js**: Version 18+ recommended
- **npm** or **yarn**

### Backend Setup

```bash
cd backend
cargo build --release
cargo run --release
```

The server will start at `http://127.0.0.1:8080`

API Endpoints:
- `GET  /health` - Health check
- `POST /api/tracks` - Upload/add a track
- `GET  /api/tracks` - List tracks
- `GET  /api/tracks/:id` - Get track details
- `POST /api/playlists` - Generate playlist
- `GET  /api/playlists` - List playlists
- `GET  /api/playlists/:id` - Get playlist with track details
- `POST /api/similarities/recompute` - Recompute all pairwise similarities

### Frontend Setup

```bash
cd frontend
npm install
npm run dev
```

Open `http://localhost:5173` in your browser.

### Quick Start with Demo Data

1. Start the backend server
2. Start the frontend dev server
3. Open the frontend and click "Add Demo Tracks" in the Library tab
4. Switch to the "Generate" tab to create a playlist

## API Examples

### Add a Track

```bash
curl -X POST http://localhost:8080/api/tracks \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Blinding Lights",
    "artist": "The Weeknd",
    "duration_seconds": 200,
    "bpm": 171,
    "key": "F#m",
    "mood_tags": ["energetic", "rhythmic"],
    "activity_tags": ["party", "workout"],
    "genres": ["pop", "electronic"]
  }'
```

### Generate a Playlist

```bash
curl -X POST http://localhost:8080/api/playlists \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Workout Mix",
    "params": {
      "mood": "energetic",
      "activity": "workout",
      "min_bpm": 120,
      "genres": ["pop", "rock"],
      "similarity_weight": 0.5,
      "diversity_weight": 0.3,
      "popularity_weight": 0.2
    },
    "requested_length": 20
  }'
```

## Audio Feature Extraction

The system analyzes WAV files (16-bit PCM or 32-bit float) to extract:

- **BPM**: Beats per minute via onset detection and peak finding
- **Key**: Estimated pitch class via chroma features
- **Spectral Centroid**: Brightness of the sound (frequency centroid)
- **Spectral Rolloff**: Frequency below which 85% of spectral energy accumulates
- **Spectral Flux**: Frame-to-frame spectral change (indicates texture)
- **RMS Energy**: Loudness/energy of the audio
- **Zero-Crossing Rate**: Approximates noisiness vs. tonal content

These features drive similarity computation and mood/activity inference.

## Playlist Generation Algorithm

1. **Seed Selection**: If seed tracks are provided, they anchor the playlist. Otherwise, tracks matching the requested mood/activity are selected.
2. **Similarity Scoring**: Each candidate track is compared to seed tracks (or previously selected tracks) using a weighted combination of:
   - BPM proximity (25%)
   - Spectral centroid similarity (20%)
   - RMS energy similarity (20%)
   - Mood tag overlap (20%)
   - Activity tag overlap (20%)
3. **Diversity Penalty**: Tracks too similar to already-selected tracks are penalized.
4. **Greedy Selection**: The highest-scoring track is added iteratively until the target length is reached.

## Future Enhancements

- **Audio File Upload**: Accept actual WAV/MP3 uploads for automatic feature extraction
- **Spotify/Last.fm Integration**: Import listening history and library
- **Advanced Recommendations**: Collaborative filtering and deep learning embeddings
- **Export Formats**: M3U/PLS playlist export, Spotify playlist creation
- **WebSocket Streaming**: Real-time playlist generation progress
- **User Accounts**: Multi-user support with personalized data
- **Advanced UI**: Drag-and-drop reordering, real-time previews, charts

## Performance

- Rust backend provides fast audio analysis and similarity computation
- SQLite with proper indexing for responsive queries even with large libraries
- Parallel similarity computation via Rayon
- React frontend optimized with Vite and TypeScript

## License

MIT License - see LICENSE file for details.

## Author

Built as EON-007: Dynamic Playlist Generator
Created by Eon (AI Assistant) for Daniel Lindestad
Technology: Rust + React with excellent tests and documentation

---

**Note**: This project is under active development. The audio analysis heuristics are simplified for performance and may not match professional DAW-level analysis. For production use, consider integrating established audio analysis libraries or services.
