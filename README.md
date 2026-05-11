<p align="center">
  <img src="assets/icon.png" width="96" alt="Quackify icon" />
</p>

<h1 align="center">Quackify</h1>

<p align="center">
  A minimal native desktop app that converts any email address into a <a href="https://duckduckgo.com/email/">DuckDuckGo Email Protection</a> alias — instantly copied to your clipboard.
</p>

<p align="center">
  <img src="assets/screenshot.png" width="445" alt="Quackify screenshot" />
</p>

```
user.name@example.com  →  user_name_at_example_com_yourname@duck.com
```

---

## Install

### macOS — Homebrew (recommended)

```bash
brew tap draugvar/quackify
brew install --cask quackify
```

### All platforms — pre-built binaries

Download the latest release from the [Releases](../../releases) page:

| Platform | File |
|----------|------|
| macOS (Apple Silicon + Intel) | `quackify-macos-universal.tar.gz` |
| Linux x86_64 | `quackify-linux-x86_64.tar.gz` |
| Windows x86_64 | `quackify-windows-x86_64.zip` |

### Build from source

Requires [Rust](https://rustup.rs/) 1.85+.

```bash
git clone https://github.com/draugvar/Duckify.git
cd Duckify
cargo build --release
```

Binary: `target/release/quackify` (or `quackify.exe` on Windows).

---

## Features

- Converts any valid email to its `duck.com` alias in one click
- Copies the result to clipboard automatically
- Remembers your Personal Duck Address across sessions
- Press Enter to convert without reaching for the mouse
- Native UI — no browser, no Electron, no runtime dependencies

---

## How it works

DuckDuckGo Email Protection generates aliases in the form:

```
original_user_at_original_domain.com_yourname@duck.com
```

Quackify automates that transformation. Paste any email, hit **Convert**, and the alias is ready to paste anywhere.

---

## License

GPL-3.0 — see [LICENSE](LICENSE) for details.

This project is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
