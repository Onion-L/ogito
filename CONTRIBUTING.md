# Contributing to **ogito**

_Thanks for your interest in improving **ogito**! We welcome all kinds of contributions including bug fixes, new features, documentation and translations._

## Table of Contents

1. Getting Help
2. Project Overview
3. Prerequisites
4. Local Setup
5. Building & Running
6. Testing
7. Coding Standards & Style
8. Commit & PR Guidelines
9. Internationalisation / Localisation
10. Release Process
11. Code of Conduct
12. License

---

## 1. Getting Help

Feel free to open a [GitHub Issue](https://github.com/Onion-L/ogito/issues) if you have questions, run into problems or would like to propose enhancements. Please search existing issues first.

## 2. Project Overview

`ogito` is a cross-platform command-line application written primarily in **Rust** with a small companion **TypeScript** wrapper published to npm. It provides ultra-fast Git repository cloning via either `git` or direct **tarball** download.

- Rust sources live under `src/`.
- Integration & unit tests live under `tests/`.
- The npm wrapper is located in `npm/` and configured via `tsdown.config.ts`.

## 3. Prerequisites

| Tool           | Minimum Version     | Notes                                                                |
| -------------- | ------------------- | -------------------------------------------------------------------- |
| Rust toolchain | 1.78 (Edition 2024) | Install via [`rustup`](https://rustup.rs). Use the _stable_ channel. |
| Node.js        | ≥ 20                | Needed only when working on the npm package.                         |
| pnpm           | ≥ 9                 | Preferred JS package manager (alternatively npm/yarn).               |
| Git            | ≥ 2.34              |                                                                      |

> Tip: run `rustup component add clippy rustfmt` to enable linting & formatting.

## 4. Local Setup

```bash
# 1. Fork & Clone
git clone https://github.com/<your-user>/ogito.git && cd ogito

# 2. Install Rust deps (one-off)
rustup target add x86_64-unknown-linux-gnu   # or your host target

# 3. (Optional) Install JS deps
pnpm install --ignore-scripts
```

## 5. Building & Running

### Rust binary

```bash
# debug build
cargo run -- <repository-url>

# release build
cargo build --release
```

### npm package (TypeScript wrapper)

```bash
pnpm build           # outputs to ./output
node output/index.js <repository-url>
```

## 6. Testing

```bash
# Rust tests
cargo test            # run all tests

# Linting
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

> Please ensure tests and lints pass **before** opening a pull request.

## 7. Coding Standards & Style

Rust code must follow `rustfmt` default style and be free of Clippy warnings. Prefer expressive error handling with `eyre::Result`. Use `snake_case` for functions, `CamelCase` for structs and enums.

For TypeScript, follow the latest **ESNext** style enforced by the `tsdown` compiler. Run `pnpm eslint .` if you add an ESLint configuration.

## 8. Commit & PR Guidelines

- Use **conventional commits** (e.g. `feat: add --depth option`, `fix: handle HTTP 404`).
- Make your PRs small and focused.
- Reference related issues in the description, e.g. `Closes #42`.
- Update documentation and tests alongside code changes.
- Target the `main` branch. CI will run automatically.

## 9. Internationalisation / Localisation

We aim to support multiple UI languages. When adding user-facing strings, place them in the relevant localisation file (TBD) and submit translations if possible.

## 10. Release Process

Releases are made from `main` and tagged using **semantic versioning** (`vX.Y.Z`). Upon tagging, GitHub Actions will:

1. Build and upload cross-compiled binaries.
2. Publish the crate to [crates.io](https://crates.io/crates/ogito).
3. Publish the npm package (after manual approval).

## 11. Code of Conduct

By participating you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md). Please report unacceptable behaviour to `onionl5236@gmail.com`.

## 12. License

`ogito` is dual-licensed under **MIT**. Any contribution intentionally submitted for inclusion in the work is licensed under the same terms.

---

_Happy hacking 🍸!_
