# Governance

fpm-monitor is a small, maintainer-led project. This document explains who makes
decisions, how contributions become part of the project, and how the process can
evolve.

## Maintainer

The project currently has one maintainer:

- **Philippe Court** ([@filoucrackeur](https://github.com/filoucrackeur))

The maintainer is responsible for the roadmap, releases, security reports, code
of conduct reports, and the final decision on pull requests. Administrative
access is limited to people who need it to publish releases and manage the
repository.

## Decisions

- Everyday changes are discussed in the relevant issue or pull request.
- The project uses **lazy consensus**: if nobody raises a substantive objection
  within a reasonable review period, a change can be merged.
- For significant changes — a new dependency, a breaking interface change, a new
  platform, a change to the security model, or a change to this document — open
  an issue first and wait for the maintainer's agreement.
- If consensus is not reached, the maintainer makes the final decision and
  explains the reasoning in the issue or pull request.
- Security reports and code of conduct reports are handled privately, following
  [SECURITY.md](SECURITY.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Becoming a contributor

Anyone can contribute by opening an issue, improving documentation, translating
the interface, testing a release, or sending a pull request. There is no formal
membership step and no minimum number of contributions.

Regular contributors who demonstrate good judgment, constructive reviews, and a
reliable understanding of the project's constraints may be invited to help with
triage, reviews, or releases. This is always opt-in and can be declined.

## Releases

- Versions follow [Semantic Versioning](https://semver.org/).
- The maintainer prepares releases from `main` by bumping the version in
  `Cargo.toml` and `Cargo.lock`, updating the release notes, and pushing a
  `vX.Y.Z` tag.
- The release workflow builds the supported targets, archives, packages, the
  Homebrew formula, and the multi-architecture Docker image.
- A release is published only after the CI checks on the tagged commit pass.

## Changes to this document

Propose a change with a pull request that explains the problem it solves. The
maintainer reviews governance changes with the same care as security-sensitive
changes. If the project gains co-maintainers, this document will be updated to
describe their roles and the decision process.
