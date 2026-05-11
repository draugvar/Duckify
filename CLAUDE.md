# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run

```bash
cargo build              # debug
cargo build --release    # release
cargo run                # run debug
```

Linux requires: `libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libwayland-dev libegl1-mesa-dev pkg-config libssl-dev`

No test suite exists yet. `is_valid_email` and `convert_to_duck_email` in `src/main.rs` are the pure functions to test first with `cargo test`.

## Architecture

Single-file Rust app (`src/main.rs`, ~335 lines). Converts emails to DuckDuckGo Email Protection aliases (`user@example.com` → `user_at_example_com_yourname@duck.com`).

- **GUI**: egui 0.33 immediate-mode via eframe, with `persistence` feature for saving the user's Duck address
- **Icons**: `build.rs` generates `icon.png` (256px), `icon.icns` (macOS), `icon.ico` (Windows) from `assets/icon_source.png` using box-filter scaling and PNG embedding
- **Persistence**: eframe `Storage` key `"duck_address"`, saved to OS app-data dir (`~/Library/Application Support/Quackify/` on macOS)
- **State**: `App` struct with `email`, `duck_address`, `result`, `copied_since: Option<f64>` (timestamp from `ctx.input(|i| i.time)`)
- **Window**: 460×480, non-resizable, dark theme via `Visuals::dark()`

### Layout structure (3 sections)

1. **Header**: app icon (44×44, loaded as egui texture) + "Quackify" title + subtitle, all centered via measured text width + calculated offset
2. **Input card**: two full-width `TextEdit` fields + centered Convert button, wrapped in `card_frame()` (SURFACE fill, 12px corner radius, 20px inner margin)
3. **Result card**: shown conditionally when `result` is non-empty, centered content (label, monospace result text, Copy button)

### Conversion logic

Matches iOS Quackify: replaces `@` with `_at_` and `.` with `_`, then appends `_localPart@duck.com`.

```rust
fn convert_to_duck_email(email: &str, duck_address: &str) -> String {
    let local_part = duck_address.split('@').next().unwrap_or(duck_address);
    let sanitized = email.replace('@', "_at_").replace('.', "_");
    format!("{sanitized}_{local_part}@duck.com")
}
```

## egui quirks in this codebase

- **Frame width**: `Frame::show()` sizes to content, not full width. Input card gets full width via `desired_width(f32::INFINITY)` on TextEdits. Result card uses `ui.allocate_space(Vec2::new(ui.available_width(), 0.0))` at end to force full-width frame.
- **`centered_and_justified`**: Only accepts ONE widget. Use `allocate_ui_with_layout` with `left_to_right(Align::Center)` for multi-widget centering instead.
- **Text measurement**: Use `ctx.fonts_mut(|f| f.layout_no_wrap(text, font_id, color))` for precise width calculation. `ui.fonts()` gives immutable ref — layout methods need mutable, must use `ctx.fonts_mut()`.
- **Copy feedback**: Auto-resets after 2s by tracking `copied_since: Option<f64>` and comparing against `ctx.input(|i| i.time)` each frame.

## Release process

Triggered automatically when a new version is pushed to `main` (detected by comparing `Cargo.toml` version against existing git tags). Workflow in `.github/workflows/release.yml`:

1. Builds universal macOS binary (lipo x86_64 + aarch64), Linux x86_64, Windows x86_64
2. Packages macOS as `Quackify.app` in tar.gz with Info.plist and icon
3. Creates GitHub Release with all platform assets
4. Updates Homebrew tap at `draugvar/homebrew-quackify` (repo must exist — not auto-created)

To release: bump `version` in `Cargo.toml`, update CHANGELOG.md, push to `main`.

## Cross-platform parity

This macOS app mirrors the iOS Quackify app (`/Users/draugvar/iOSProjects/duckify-ios/`). When changing behavior, check iOS `ContentView.swift` for reference. Key differences: iOS has SwiftUI animations, DuckIcon image asset, and `textSelection(.enabled)` — desktop approximates these within egui limitations.
