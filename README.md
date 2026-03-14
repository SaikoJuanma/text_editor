# text_editor

A terminal-based text editor written in Rust. Built as a learning project, with the long-term goal of becoming a feature-rich editor inspired by Obsidian — focused on writing, notes, and plain-text workflows.

## Status

Early development. Native GUI window opens with a full-screen text editor.

## Features

- Native GUI window (egui/eframe)
- Full-screen text editing area

## Roadmap

- [x] Native GUI window with text editor
- [ ] Open a file from the command line
- [ ] Save with keyboard shortcut
- [ ] Status bar (file name, modified indicator, cursor position)
- [ ] Multiple files / tabs
- [ ] Markdown rendering
- [ ] File tree / vault navigation (Obsidian-style)
- [ ] Search within file
- [ ] Global search across files
- [ ] AutoSave
- [ ] Links between files

## Usage

```bash
cargo run -- path/to/file.txt
```

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (stable)

### Run

```bash
cargo run -- myfile.txt
```

### Test

```bash
cargo test
```

### Lint & Format

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
```

## CI

Every push and pull request on any branch runs:

- `cargo fmt` — formatting check
- `cargo clippy` — lints (pedantic + nursery, warnings as errors)
- `cargo test` — unit and doc tests
- `cargo build` — verifies the project compiles

## License

MIT
