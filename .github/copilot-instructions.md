# Copilot Instructions for Duckify

## Project Overview

Duckify is a minimal native desktop application written in Rust that converts any email address into a [DuckDuckGo Email Protection](https://duckduckgo.com/email/) alias. The alias format is:

```
original_user_at_original_domain.com_yourname@duck.com
```

## Tech Stack

- **Language**: Rust (edition 2024, requires 1.85+)
- **GUI framework**: [egui](https://github.com/emilk/egui) via [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) `0.33.x`
- **Image processing**: `png` crate (used in both `build.rs` and runtime icon loading)
- **Persistence**: eframe's built-in storage (key–value, saved to the OS app-data directory)

## Repository Structure

```
src/main.rs       – entire application logic and UI (single-file app)
build.rs          – build script: generates icon assets (icon.png, icon.icns, icon.ico)
assets/           – icon source and generated icon files; screenshot for README
Cargo.toml        – package manifest
.github/workflows/release.yml – CI/CD: builds universal macOS, Linux, and Windows binaries and publishes a GitHub Release
```

## How to Build and Run

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run directly
cargo run
```

On **Linux**, the following system libraries are required:
```
libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
libxkbcommon-dev libwayland-dev libegl1-mesa-dev pkg-config libssl-dev
```

## Testing

There is no dedicated test suite. Core logic functions (`is_valid_email`, `convert_to_duck_email`) live in `src/main.rs` and can be tested with `cargo test` once unit tests are added.

## Code Conventions

- All application code lives in **`src/main.rs`** — keep it that way unless the file grows substantially.
- UI is built with **immediate-mode egui**: every frame re-renders the entire UI.
- The colour palette is defined as `const Color32` values at the top of `main.rs`; use those constants rather than inline colours.
- Error handling in the build script uses `unwrap()`/`expect()` — acceptable because build failures should be fatal.
- Rust 2024 edition features (e.g., `let … else`, `is_some_and`) are in use; keep code idiomatic.

## Release Process

Releases are triggered automatically by the `release.yml` workflow when a commit to `main` contains a new version in `Cargo.toml`. The workflow:
1. Detects if the version tag already exists.
2. Builds universal macOS, Linux x86\_64, and Windows x86\_64 binaries.
3. Creates a GitHub Release with generated release notes.
4. Updates the Homebrew tap at `draugvar/homebrew-duckify`.

To cut a new release, bump the `version` field in `Cargo.toml` and push to `main`.
