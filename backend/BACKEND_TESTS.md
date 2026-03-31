# Backend Test Suite

This document provides an overview of the test coverage for the Dynamic Playlist Generator backend.

## Test Summary

| Category | Tests | Status |
|----------|------:|--------:|
| **Unit Tests** | 7 | ✅ All passing |
| **Integration Tests** | 4 | ✅ All passing |
| **Total** | **11** | **✅ 100% pass** |

## Unit Tests (`src/`)

Located in `src/audio.rs` under `#[cfg(test)] mod tests`.

### Audio Module Tests

1. **`test_similarity_identical_tracks`** - Verifies that identical tracks have similarity 1.0
2. **`test_similarity_same_features`** - Tracks with matching audio features score perfectly
3. **`test_similarity_different_tracks`** - Dissimilar tracks have lower scores
4. **`test_similarity_no_features`** - Tracks with no features return 0.0 similarity
5. **`test_similarity_partial_features`** - Tracks with subset of features still produce valid scores
6. **`test_analyze_empty_samples`** - Empty audio buffer returns an error
7. **`test_estimate_bpm_fallback`** - Fallback BPM of 120.0 for silence/insufficient data

### Running Unit Tests

```bash
cd backend && cargo test --lib --release
```

## Integration Tests (`tests/`)

Located in `tests/integration_test.rs`.

### Database Tests

1. **`test_database_creation_and_track_storage`**
   - Creates a temporary SQLite database
   - Saves a track and retrieves it
   - Validates data integrity

### Playlist Generation Tests

2. **`test_playlist_generation_with_seed_tracks`**
   - Creates tracks with audio features (BPM, RMS, mood tags)
   - Generates a playlist using seed track
   - Verifies playlist length, content, and database persistence

3. **`test_playlist_generation_by_mood`**
   - Creates tracks with different mood tags
   - Requests playlist filtered by "energetic" mood
   - Asserts only matching tracks are included

### Audio Module Integration

4. **`test_similarity_computation`**
   - Creates tracks with similar and dissimilar features
   - Uses `compute_similarity` function directly
   - Validates scoring differentiation

### Running Integration Tests

```bash
cd backend && cargo test --test integration_test --release
```

Or run all tests:

```bash
cd backend && cargo test --release
```

## Test Dependencies

- **tokio** with `macros` and `rt-multi-thread` features (async test runtime)
- **tempfile** for temporary database files
- **rusqlite** for in-memory SQLite testing

All test dependencies are declared in `Cargo.toml` under `[dev-dependencies]`.

## Code Coverage

To generate coverage reports (requires `cargo tarpaulin` or `cargo llvm-cov`):

```bash
cargo llvm-cov --release --lcov --output-path lcov.info
```

Current focus: Core audio analysis and similarity functions have high coverage.

## Quality Gates

- All tests must pass before merge (`cargo test --release`)
- Warnings treated as errors in CI (future enhancement)
- New features require corresponding tests
- Bug fixes should include regression tests

## Test Data

Tests use **synthetic data** generated within test code:
- Tracks with controlled BPM, spectral features, mood tags
- Temporary SQLite databases in temp directories
- No external audio files needed for current test suite

---

**Last Updated:** 2026-03-31  
**Maintainer:** Eon (AI Assistant)
