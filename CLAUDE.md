# CLAUDE.md

## Project Overview

pencil_analyzer is a Rust CLI tool that parses Pencil (pencil.dev) `.pen` files. Its goal is to extract design file structure, components, and style information, and output them in a format suitable as AI context.

## Tech Stack

- Rust (edition 2024)
- Build management via Cargo

## Build & Run

```bash
cargo build          # Debug build
cargo run            # Run
cargo build --release # Release build
cargo test           # Run tests
cargo clippy         # Lint
```

## Project Structure

```
src/
  main.rs    # Entry point
Cargo.toml   # Package definition & dependencies
```

## Coding Conventions

- Follow Rust standard formatter (`cargo fmt`)
- Resolve all `cargo clippy` warnings
- Use `Result` types for error handling; avoid excessive `unwrap()`

## About .pen Files

- `.pen` files are a proprietary format of the Pencil design tool
- File contents are encrypted and cannot be read as plain text
- Accessible only via Pencil MCP tools
