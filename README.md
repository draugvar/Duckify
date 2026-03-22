# Duckify

A minimal native desktop app that converts any email address into a [DuckDuckGo Email Protection](https://duckduckgo.com/email/) alias.

## What it does

DuckDuckGo Email Protection lets you create aliases in the form:

```
original_user_at_original_domain.com_yourname@duck.com
```

Duckify automates that transformation — paste any email, click **Convert**, and the alias is ready in your clipboard.

## Features

- Converts any valid email address to its duck.com alias
- Copies the result to the clipboard automatically
- Remembers your Personal Duck Address across sessions
- Supports Enter key to convert quickly
- Native UI, no browser required

## Download

Pre-built binaries are available on the [Releases](../../releases) page for:

- **macOS** — universal binary (Apple Silicon + Intel), packaged as `.app`
- **Linux** — x86_64 binary
- **Windows** — x86_64 executable

## Build from source

Requires [Rust](https://rustup.rs/) 1.85+.

```bash
git clone https://github.com/YOUR_USERNAME/duckify.git
cd duckify
cargo build --release
```

The binary will be at `target/release/duckify` (or `duckify.exe` on Windows).

## How the alias is built

```
user@example.com  →  user_at_example.com_yourname@duck.com
```

Where `yourname` is the local part of your Personal Duck Address (e.g. `yourname@duck.com`).

## License

MIT
