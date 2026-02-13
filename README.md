# rslist

Minimal Rust CLI Todo List

# Overview

- Designed to be dependency-free (no external crates).
- Intends to work with plain text (.txt) files for list persistence.
- Starts with a friendly introduction panel and a simple command scaffold.

# Features (Planned vs. Implemented)

- Implemented
    - Colorful introduction panel on startup (inspired by polished CLI start panels)
    - Basic command scaffold and friendly usage messages
    - No external crates; runs entirely with Rust `std` library
- Planned
    - Lists stored as .txt files
    - Create lists, add tasks, read lists, update tasks
    - Read and list tasks within a list
    - Update task descriptions or statuses (e.g., done)
    - Read/write via a simple, predictable file format
    - Optional local directory for lists (e.g., a lists/ folder)

# Usage

```bash
# Build
cargo build
```

```bash
# Run
cargo run -- rslist test # Output includes the introduction panel followed by: Test Command
cargo run -- rslist config # Output includes the introduction panel followed by: Config Command
```
