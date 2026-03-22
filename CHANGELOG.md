# Changelog

All notable changes to Duckify will be documented in this file.

## [1.1.1] - 2026-03-22

### Fixed
- GitHub release cleanup job now uses explicit `actions: write` permissions so artifact deletion API calls no longer fail with HTTP 403
- Artifact cleanup now handles empty lists and continues on single delete failures, emitting warnings instead of failing the whole workflow

## [1.1.0] - 2026-03-22

### Added
- App icon for Windows (`.ico`), macOS (`.icns`) and Linux (`.png`), generated at build time from `assets/icon_source.png`
- Build script (`build.rs`) that automatically recolors the source icon: yellow duck, black envelope outlines, white envelope interior, transparent background
- Flood-fill algorithm to correctly detect and fill the inner area of the envelope with white, regardless of the source pixel alpha
- ASCII art duck `>('.)>` next to the app title in the header

### Fixed
- Result card was wider than the input card and not centered — fixed layout using a constrained horizontal strip
- "Copied to clipboard!" message was rendered outside the bottom of the result card
- Checkmark character `✓` was not rendered by the default egui font — replaced with a clean text label

## [1.0.0] - Initial Release

### Added
- Convert any email address to a duck.com alias
- Personal Duck Address input with persistent storage
- One-click copy of the generated alias to clipboard
- Modern dark theme UI built with `eframe` / `egui`
- Cross-platform support: macOS, Windows, Linux
