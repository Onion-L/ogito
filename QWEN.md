# Qwen Code Context for `ogito`

This document provides essential context about the `ogito` project for Qwen Code, enabling it to understand the codebase and provide effective assistance.

## Project Overview

`ogito` is a simple and efficient command-line tool for cloning Git repositories to create clean project copies. It allows users to quickly scaffold new projects based on existing Git repositories, with options to specify branches, clean the history, or force overwrite existing directories. The core application is written in Rust, leveraging `clap` for CLI argument parsing. There's also a companion TypeScript wrapper for publishing to npm, but the primary functionality resides in the Rust binary.

## Core Technologies & Architecture

- **Language**: Rust (primary), TypeScript (npm wrapper)
- **CLI Framework**: `clap` (Rust)
- **Build System**: `cargo` (Rust), `tsdown` (TypeScript)
- **Package Manager**: `cargo` (Rust), `pnpm` (TypeScript/JS)
- **Architecture**:
  - The main Rust binary is located at `src/main.rs`.
  - CLI logic is in `src/cli.rs`.
  - Cloning logic (Git and Tar modes) is in `src/clone.rs`.
  - Specific functionalities like Git interaction (`src/git.rs`), file handling (`src/file/`), fetching (`src/fetch/`), and mode management (`src/mode.rs`) are modularized.
  - Configuration structs are defined in `src/fetch/config.rs`.
  - A TypeScript script (`script/script.ts`) helps manage binary distribution for npm.

## Building and Running

### Rust Binary

- **Build Debug**: `cargo build`
- **Build Release**: `cargo build --release`
- **Run (Debug)**: `cargo run -- <args>`
- **Install Locally**: `cargo install --path .`

### TypeScript Wrapper (npm package)

- **Build**: `pnpm run build` (uses `tsdown`)
- **Run (after build)**: `node dist/run.js <args>`
- **Install Globally (from source)**: `pnpm run build && npm install -g .` (This builds the TS wrapper and installs the `package.json` defined package globally, which executes the built `dist/run.js`).

## Testing

- **Run Rust Tests**: `cargo test`
- **Linting (Clippy)**: `cargo clippy --all-targets --all-features -- -D warnings`
- **Formatting (rustfmt)**: `cargo fmt --all -- --check`

## Development Conventions

- Follow Rust idioms, using `snake_case` for functions/variables and `CamelCase` for structs/enums.
- Code should be formatted with `rustfmt` and pass `clippy` checks.
- Conventional commits are preferred for Git history (e.g., `feat:`, `fix:`, `refactor:`).
- Error handling is done using `color_eyre::Result`.
- TypeScript code follows ESNext style as enforced by `tsdown`.

## Key Project Files and Directories

- `src/`: Contains the main Rust source code.
- `src/main.rs`: Entry point of the Rust application.
- `src/cli.rs`: Defines the command-line interface using `clap`.
- `src/clone.rs`: Core logic for cloning repositories.
- `Cargo.toml`: Rust project manifest, defining dependencies and metadata.
- `package.json`: NPM package manifest for the TypeScript wrapper.
- `tsdown.config.ts`: Configuration for the TypeScript build process.
- `script/script.ts`: Helper script for preparing npm binary packages.
- `packages/_template.json`: Template for npm binary package.json files.
- `CONTRIBUTING.md`: Guidelines for contributing to the project.
- `PRD.md`: Product Requirements Document outlining features and workflows.
- `ROADMAP.md`: Future development plans.