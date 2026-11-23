# Contributing to fpm-monitor

Thanks for taking the time to contribute! This document explains how to report
problems, propose changes, set up the project, and get a pull request merged.

By participating, you agree to follow our [Code of Conduct](CODE_OF_CONDUCT.md).
For usage questions, see [SUPPORT.md](SUPPORT.md). To report a vulnerability,
follow [SECURITY.md](SECURITY.md) instead of opening a public issue.

## Ways to contribute

- Report a reproducible bug with the bug report issue template.
- Suggest a feature or an improvement with the feature request template.
- Improve the documentation or one of the 22 translations.
- Fix a bug, improve portability, performance, accessibility, or test coverage.
- Test a release on a platform or architecture we do not have easy access to.
- Triage issues, review pull requests, or answer questions in Discussions.

You do not need to write Rust to help. Documentation, translations, bug reports,
and careful testing are all valuable contributions.

## Before you start

1. Search the existing [issues](https://github.com/filoucrackeur/fpm-monitor/issues)
   and [discussions](https://github.com/filoucrackeur/fpm-monitor/discussions).
2. For a large change, open an issue first so we can agree on the approach.
3. Keep each pull request focused on one problem. Split unrelated changes.
4. Never include secrets, private logs, or personal data in an issue or a commit.

## Development setup

You need:

- Rust 1.78 or newer (the MSRV tested in CI).
- Cargo, which ships with Rust.
- Python 3 for the translated README check.
- PHP-FPM only if you want to test live data. The `--mock` mode works without it.

```sh
git clone https://github.com/filoucrackeur/fpm-monitor.git
cd fpm-monitor

cargo build
cargo run -- --mock                 # CLI table with demo data
cargo run -- --mock -t              # interactive terminal dashboard
cargo run -- --mock --watch         # web dashboard on http://127.0.0.1:8471
```

The web dashboard binds to loopback only. Do not expose it through a public
reverse proxy: it has no authentication and is designed for local use.

## Project constraints

Please keep these constraints in mind; they are part of the project's design:

- **No third-party crates.** The binary uses the Rust standard library only.
  Discuss any new dependency before adding it.
- **No outgoing network calls.** Data comes from `/proc`, `ps`, `netstat`, and
  local configuration and log files. The web server is loopback-only.
- **Portable by design.** Linux, macOS, and Windows are supported, along with 18
  release targets. Use `cfg` fallbacks rather than breaking a platform.
- **Rust 1.78 MSRV.** Do not use newer language or standard library features
  without prior discussion.
- **22 languages.** A user-visible string belongs in the `Labels` struct and in
  every locale under `src/i18n/`, not hard-coded in the interface.
- **Coverage stays at or above 75% of lines.** Add tests with behavior changes.
- **The documentation is translated.** Update the English README and the
  translated READMEs when a user-visible change affects them.

## Running the checks

Run the same checks as CI before opening a pull request:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
python3 scripts/check-readmes.py
```

If you have `cargo-llvm-cov`, you can also check the coverage gate locally:

```sh
cargo llvm-cov --locked --all-targets --lcov --output-path lcov.info \
  --fail-under-lines 75
```

CI runs the test suite on Linux, macOS, and Windows, on the MSRV (Rust 1.78),
checks formatting and Clippy, validates the 22 translated READMEs, and builds
the release targets. A pull request is ready when these checks are green.

## Tests

- Add a unit or integration test for every behavior change and bug fix.
- Tests must not require a running PHP-FPM instance, a network connection, or a
  specific host configuration. Use `--mock` and temporary fixtures.
- Prefer deterministic tests. Avoid timing-based tests and sleeps.
- For web changes, test request parsing, status codes, JSON/HTML escaping, and
  serialization without starting an external service.
- For i18n changes, keep placeholders such as `{}` intact and check that every
  locale has the new label.

The end-to-end CLI tests live in `tests/cli.rs`. Unit tests live next to the code
they cover.

## Code style

- Format with `cargo fmt`; CI rejects unformatted code.
- Keep Clippy clean with `-D warnings`.
- Follow the existing style and module layout. Keep functions focused.
- Write comments that explain *why* a decision was made, not what the code does.
- Treat configuration files, log files, process names, and HTTP requests as
  untrusted input.
- Do not add generated binaries, build output, editor files, or local screenshots
  to the repository.

## Commit messages

Use [Conventional Commits](https://www.conventionalcommits.org/) with an
imperative, present-tense summary:

```text
feat(web): add a language menu
fix(proc): handle a missing /proc entry
docs(i18n): translate the new label
test(tui): cover the empty graph
ci: run the MSRV job on pull requests
```

Common types are `feat`, `fix`, `docs`, `test`, `perf`, `refactor`, `ci`, and
`chore`. A scope is optional; `web`, `tui`, `i18n`, `proc`, `config`, and `logs`
are useful ones. Keep one logical change per commit.

## Pull requests

1. Branch from `main` and give the branch a descriptive name.
2. Fill in the pull request template.
3. Link the issue it fixes, for example `Closes #123`.
4. Describe the user-visible change, the tests you ran, and the platforms you
   tested.
5. Add screenshots or a short recording for visual TUI or web changes.
6. Update the READMEs and translations when needed.
7. Make sure CI is green. Rebase on `main` if the branch is out of date.

Maintainers may ask for changes, squash commits, or split a pull request. Please
do not force-push over review comments without explaining why.

## Documentation and translations

The interface and the README are available in 22 languages: `en`, `fr`, `zh`,
`ar`, `es`, `it`, `ja`, `de`, `ru`, `hi`, `id`, `ko`, `no`, `sv`, `fi`, `da`,
`el`, `tr`, `pl`, `uk`, `th`, and `vi`.

For a new interface string:

1. Add the field to `Labels` in `src/i18n.rs`.
2. Add the translation to the inline locales and to every file in `src/i18n/`.
3. Use the label from the CLI, TUI, or web interface instead of a literal.
4. Check Arabic right-to-left layout if the change is visual.
5. Run `cargo test --all-targets` and `python3 scripts/check-readmes.py`.

Keep command names, option names, brand names, and placeholders unchanged.
Translations should read naturally in the target language; a literal word-for-word
translation is not required.

## Releases

Releases are prepared by the maintainer. A release bumps the version in
`Cargo.toml` and `Cargo.lock`, updates the changelog or release notes, and pushes
a `vX.Y.Z` tag. The release workflow builds the 18 targets, the archives, the
`.deb`/`.rpm` packages, the Homebrew formula, and the multi-architecture Docker
image. See [GOVERNANCE.md](GOVERNANCE.md) for the decision process.

## License

By contributing, you agree that your contributions are licensed under the
[MIT License](LICENSE), without additional terms or conditions.
