# Changelog

All notable changes to Duckify will be documented in this file.

## [1.1.8] - 2026-03-29

### Fixed
- Release workflow CI failure: removed the non-existent `actions/delete-artifact` action (caused "repository not found" and exit code 123)
- Added `actions: write` to the workflow-level `permissions` block — without it the `GITHUB_TOKEN` was silently capped, causing HTTP 403 on every artifact deletion attempt
- Cleanup job now uses [`geekyeggo/delete-artifact@v6`](https://github.com/GeekyEggo/delete-artifact) (the standard community action for this) with `failOnError: false` so the workflow never breaks if an artifact is already gone

## [1.1.7] - 2026-03-29

### Fixed
- Release workflow artifact cleanup

### Changed
- README header now shows the app icon for a nicer look on the GitHub page

## [1.1.6] - 2026-03-23

### Changed
- License changed from MIT to GPL-3.0-or-later — all redistributions and derivative works must remain open source

## [1.1.5] - 2026-03-22

### Changed
- README now includes a screenshot of the app

## [1.1.4] - 2026-03-22

### Changed
- README modernized: cleaner structure, install table per platform, Homebrew install instructions as primary method

## [1.1.3] - 2026-03-22

### Added
- Homebrew Cask support via `draugvar/homebrew-duckify` tap — install with `brew tap draugvar/duckify && brew install --cask duckify`
- Release workflow now automatically updates the Homebrew tap formula with the correct version and SHA256 after each release

## [1.1.2] - 2026-03-22

### Fixed
- macOS app icon not showing in Finder — `icon.icns` was missing from `Contents/Resources/` in the `.app` bundle produced by the release workflow
- `Info.plist` path in release workflow corrected from root to `assets/Info.plist`

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
