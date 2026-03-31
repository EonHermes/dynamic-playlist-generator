# Contributing to Dynamic Playlist Generator

Thank you for contributing! This document outlines the development workflow and standards.

## Development Setup

### Prerequisites

- **Rust** 1.70+ (install via https://rustup.rs/)
- **Node.js** 18+ with npm
- **Git**

### Quick Start

```bash
# Clone and enter
git clone https://github.com/EonHermes/dynamic-playlist-generator.git
cd dynamic-playlist-generator

# Build everything
make build

# Run backend
cd backend && cargo run --release

# In another terminal, run frontend dev server
cd frontend && npm run dev
```

### Directory Structure

```
dynamic-playlist-generator/
├── backend/           # Rust (Actix-web) backend
│   ├── src/
│   │   ├── main.rs           # Server entry point
│   │   ├── lib.rs            # Library exports
│   │   ├── db.rs             # SQLite database layer
│   │   ├── models.rs         # Data structures
│   │   ├── audio.rs          # Audio analysis (hound + DSP)
│   │   ├── services.rs       # Playlist generation logic
│   │   └── handlers.rs       # HTTP request handlers
│   ├── Cargo.toml
│   └── tests/                 # Integration tests
├── frontend/          # React (TypeScript + Vite) frontend
│   ├── src/
│   │   ├── App.tsx
│   │   ├── components/
│   │   │   ├── TrackList.tsx
│   │   │   ├── PlaylistGenerator.tsx
│   │   │   └── PlaylistViewer.tsx
│   │   └── types.ts
│   ├── package.json
│   └── index.html
├── Makefile           # Build automation
├── README.md          # User documentation
└── CONTRIBUTING.md    # This file
```

## Code Standards

### Rust Backend

- **Edition**: 2024
- **Formatting**: `cargo fmt`
- **Linting**: `cargo clippy` (all warnings treated as errors in CI)
- **Testing**: `cargo test --release` (integration + unit tests)
- **Error Handling**: Use `thiserror` for custom error types; propagate errors with `?`
- **Async**: Actix-web with async handlers; use `tokio` test runtime
- **Serialization**: `serde` with `derive` feature; use snake_case for JSON fields
- **Documentation**: rustdoc comments for public APIs (`///`)

### TypeScript Frontend

- **Strict Mode**: `strict: true` in tsconfig
- **Components**: Functional components with hooks; no class components
- **Styling**: CSS modules or plain CSS; avoid inline styles
- **State Management**: React useState/useEffect; avoid external state libs for now
- **API Calls**: `fetch` with proper error handling
- **Linting**: ESLint with react-hooks rules

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add BPM detection for polyrhythms
fix: handle empty track library gracefully
docs: update API examples in README
test: add unit tests for similarity algorithm
refactor: simplify playlist selection logic
chore: update dependencies
```

## Testing Strategy

### Backend Tests

- **Unit tests**: In `src/` files with `#[cfg(test)] mod tests`
- **Integration tests**: In `tests/` directory
- **Coverage focus**: Audio analysis, similarity computation, database operations

Run: `cd backend && cargo test --release`

### Frontend Tests

Future: Vitest + React Testing Library for component tests.

## API Design

When modifying the API:

1. Update models in `models.rs`
2. Add/modify handlers in `handlers.rs`
3. Update TypeScript types in `frontend/src/types.ts`
4. Update API examples in README
5. Ensure backward compatibility or version the API (`/api/v1/...`)

## Database Migrations

The project uses SQLite with rusqlite. Schema changes should:

1. Update the `CREATE TABLE` statements in `db.rs::initialize_schema`
2. Add migration logic if needed (future enhancement)
3. Test with fresh database and existing data if possible

## Performance Considerations

- **Audio analysis**: Optimize inner loops; avoid allocations in hot paths
- **Similarity computation**: Cache results; use rayon for parallelization if library grows large
- **Database**: Add indexes for frequent queries (e.g., tracks by mood)
- **Frontend**: Lazy load components; optimize bundle size with Vite

## Submitting a Pull Request

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make changes with clear, focused commits
4. Ensure all tests pass (`make test`)
5. Update documentation (README, code comments)
6. Submit PR with clear description and linked issues

## Questions?

Open an issue or contact the maintainer.

---

**Thank you for making the Dynamic Playlist Generator better!** 🎵
