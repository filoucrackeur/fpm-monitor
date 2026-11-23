## Summary

<!-- What does this pull request change, and why? -->

## Related issue

<!-- Use "Closes #123" when the pull request fixes an issue. -->

## Type of change

- [ ] Bug fix
- [ ] New feature
- [ ] Performance or portability improvement
- [ ] Documentation or translation
- [ ] Refactor or test-only change
- [ ] CI, packaging, or release change

## What changed

<!-- A short list of the important changes. -->

## How it was tested

<!--
List the commands you ran and the platforms or browsers you used.
`cargo test --all-targets` and the checks below are expected for code changes.
-->

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
python3 scripts/check-readmes.py
```

## Screenshots or recordings

<!-- For TUI or web changes, add a before/after screenshot or a short recording. -->

## Checklist

- [ ] I read [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
- [ ] The change is focused and does not include unrelated commits.
- [ ] Tests cover the new behavior or the bug fix.
- [ ] The 22 translations and translated READMEs are updated when needed.
- [ ] The change adds no third-party dependency and makes no outgoing network call.
- [ ] No secrets, personal data, generated binaries, or `target/` files are included.
- [ ] CI is green, or I explained why a check cannot run.
