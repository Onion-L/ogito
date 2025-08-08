# Security Policy for **ogito**

## Supported Versions

| Version | Supported | Security Updates |
| ------- | --------- | ---------------- |
| `main`  | ✔️        | ✔️               |
| `1.0.x` | ✔️        | ✔️               |
| `0.x.x` | ❌        | ❌               |

The project is currently **1.0.x** and follows [Semantic Versioning](https://semver.org/). We provide security fixes only for the latest stable minor release (`1.0.x`). Please upgrade to the most recent `1.0` patch release (or the `main` branch) to receive fixes.

## Reporting a Vulnerability

If you discover a security vulnerability **PLEASE DO NOT** create a public issue. Instead, responsible disclosure helps protect users while a fix is prepared.

1. Email the maintainers at **onionl5236@gmail.com** _(or contact via GitHub private message if you do not receive a response within 48 hours)._
2. Provide the following:
   - A description of the vulnerability and its impact.
   - Steps to reproduce or a proof-of-concept exploit.
   - Any known work-arounds.
3. You will receive a reply within **2 business days** confirming receipt and next steps.
4. We aim to release a patch and publish a CVE (if applicable) within **30 days**. You will be credited in the release notes unless you request otherwise.

## Scope

`ogito` is a command-line tool written in Rust that downloads source code using either:

- `git clone` (via the system-installed Git client), or
- GitHub/GitLab tarball downloads.

The following are **in scope**:

- Remote-code-execution, privilege escalation, or directory traversal when cloning/extracting repositories.
- Unsafe deserialization or parsing within the application.
- Manipulation of environment variables leading to command or path injection.

The following are **out of scope**:

- Vulnerabilities in third-party services such as the Git hosting provider.
- Issues requiring physical access or social engineering of maintainers.

## Security Best Practices for Users

While we strive to keep `ogito` secure, users should also:

1. Install releases from trusted sources only (e.g. `cargo install ogito` from crates.io).
2. Avoid running `ogito` with elevated privileges (`sudo`, Administrator) unless absolutely necessary.
3. Review repositories before cloning/extracting them, especially if using `--force` or custom destination directories.
4. Keep Rust, Git, and system libraries up to date to receive upstream security patches.

## Development Process

Security is integrated into our development workflow:

- **Dependencies** – Dependabot alerts are enabled; vulnerable crates are updated promptly.
- **Continuous Integration** – CI runs `cargo audit` and `cargo clippy` to detect known CVEs and unsafe code patterns.
- **Code Review** – All changes require pull-request review. Use of `unsafe` Rust code is heavily scrutinized.
- **Testing** – Unit and integration tests cover input validation, error handling, and edge cases.

## Cryptographic Practices

`ogito` uses HTTPS for tarball downloads and relies on Git’s own authentication mechanisms when cloning via SSH/HTTPS. No additional cryptography is implemented in-house.

## Contact

Email onionl5236@gmail.com or open a confidential GitHub security advisory.

Thank you for helping keep `ogito` and its users safe!
