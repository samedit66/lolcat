# lolcat 🌈

A small, idiomatic Rust reimplementation of the classic [lolcat](https://github.com/busyloop/lolcat) utility — it colorizes text output with a pleasant rainbow gradient.
This clone is now rewritten in Rust — enjoy the blazingly fast performance and strong safety guarantees Rust provides🔥.

## Screenshot

<img width="764" height="488" alt="lolcat" src="https://github.com/user-attachments/assets/dd0d783f-067b-4b77-bf5d-ef328320ecb4" />

## Features
- Colorizes piped or file text with a smooth rainbow gradient
- Streaming-friendly (works well with pipes)
- UTF-8 aware
- Cross-platform (where the terminal supports ANSI colors)
- Small, dependency-minimal, and designed to be simple to use

## Quickstart

### Prerequsites
- Rust toolchain (stable rustc + cargo). Install from https://rustup.rs if you do not have it.

### Build from source
```bash
git clone https://github.com/samedit66/lolcat.git
cd lolcat
cargo build --release
```
The compiled binary will be at `./target/release/lolcat`.

### Install (optional)

Install locally with `cargo`:
```bash
cargo install --path .
```

Or install directly from GitHub:
```bash
cargo install --git https://github.com/samedit66/lolcat.git
```

## Usage
Basic usage reads from standard input or one or more files:
```bash
# pipe text into lolcat
echo "Hello, world!" | lolcat

# colorize a file
lolcat README.md

# read multiple files
lolcat file1.txt file2.txt
```
