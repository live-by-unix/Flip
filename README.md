# flip — the thing which flips anything, but not your storage as a negative integer

flip is a tiny, fast Rust tool that flips text, emojis, and full Unicode grapheme clusters.
It works as both a CLI utility and an HTTP server.

## Features
- Flip any text
- Perfect emoji + grapheme‑cluster flipping (`--flipe`)
- CLI mode (`flip text`)
- HTTP server mode (`flip serve`)
- Save‑to‑file prompt
- IPv4‑safe binding
- Zero dependencies beyond Rust crates

## Installation

### Build from source
\`\`\`bash
git clone <your repo>
cd flip
cargo install --path .
\`\`\`

Ensure Cargo's bin directory is in your PATH:
\`\`\`bash
export PATH="$HOME/.cargo/bin:$PATH"
\`\`\`

## Usage

### Flip text from CLI
\`\`\`bash
flip text "hello world"
\`\`\`

### Flip emojis (grapheme‑cluster aware)
\`\`\`bash
flip --flipe text "hello 👨‍👩‍👧‍👦 world"
\`\`\`

### Flip text from stdin
\`\`\`bash
echo "abcdef" | flip text
\`\`\`

### Save output to file
After flipping, flip will ask:
\`\`\`
Save to flipped-thing.txt? (y/N):
\`\`\`

## Server Mode

Start the server:
\`\`\`bash
flip serve
\`\`\`

Custom address:
\`\`\`bash
flip serve --addr 0.0.0.0:8080
\`\`\`

## HTTP API

### POST /flip

Basic flip:
\`\`\`bash
curl -X POST http://127.0.0.1:8080/flip -d "hello"
\`\`\`

Emoji flip:
\`\`\`bash
curl -X POST "http://127.0.0.1:8080/flip?flipe=1" -d "hello 🌈"
\`\`\`

## Flags

### --flipe
Flip emojis and grapheme clusters as atomic units.

Works in:
- CLI mode
- Server mode (global)
- HTTP query parameter (`?flipe=1`)

## Project Structure
\`\`\`
flip/
  Cargo.toml
  src/
    main.rs
\`\`\`

## License
MIT
