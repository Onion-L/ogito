# About This Project: ogito

`ogito` is a command-line interface (CLI) tool designed to manage and generate projects from a user's personal code templates.

## Core Philosophy

The vision for `ogito` is not to replace comprehensive scaffolders like `vite` or `create-react-app`. Instead, it aims to provide a more flexible and lightweight solution for developers to quickly start new projects based on their own curated set of templates. The core principle is user freedom and control over their own project boilerplates.

## Key Technologies

*   **Core Application:** Rust
*   **CLI Framework:** `clap` (inferred)
*   **Scripting/Tooling:** TypeScript with `pnpm` for package management.

## Project Structure

*   `src/`: Contains all the core Rust application logic.
    *   `main.rs`: The entry point of the Rust application.
    *   `cli.rs`: Defines the structure of the command-line arguments and subcommands.
    *   `cmd/`: Contains the implementation for each subcommand (e.g., `add.rs`, `new.rs`).
*   `npm/`, `script/`: Helper scripts and tooling written in TypeScript.
*   `Cargo.toml`: Defines Rust dependencies, project metadata, and workspace configuration.
*   `package.json`: Defines Node.js dependencies and scripts for the TypeScript tooling.
*   `templates/` (Suggested): A directory (to be created) where user-defined templates will be stored.

## Common Commands

Here are the essential commands for developing, testing, and maintaining this project.

### Rust Workflow

*   **Run in development:** `cargo run -- <SUBCOMMAND> <OPTIONS>`
    *   *Example:* `cargo run -- new my-awesome-project`
*   **Build for release:** `cargo build --release`
*   **Run tests:** `cargo test`
*   **Check formatting:** `cargo fmt -- --check`
*   **Apply formatting:** `cargo fmt`
*   **Lint with Clippy:** `cargo clippy -- -D warnings`

### NPM/Scripting Workflow

*   **Install dependencies:** `pnpm install`
*   **Run a script:** `pnpm run <script_name>`

## Development Goals (Your Plan)

The primary goal is to implement the full set of intended subcommands.

### Usage

`ogito <COMMAND>`

### Commands

*   `add`: Add a new template from a local path or a git repository.
*   `new`: Create a new project from a specified template.
*   `update`: Update an existing template from its source.
*   `remove`/`delete`: Remove a template.
*   `list`: List all available templates.
*   `clear`: Clear the template cache.
