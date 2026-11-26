# Security Policy

## Supported versions

| Version | Supported |
| ------- | --------- |
| 1.2.x   | Yes       |
| < 1.2   | No        |

Security fixes are shipped in the latest minor release. If you are unsure
whether a version is affected, reproduce the issue on the latest release first.

## Reporting a vulnerability

**Please do not open a public issue for a security problem.**

Use GitHub's private vulnerability reporting:

1. Open <https://github.com/filoucrackeur/fpm-monitor/security/advisories/new>.
2. Describe the issue and include a minimal reproduction.
3. Submit the report. Only the maintainer can see it.

If you cannot use GitHub's private reporting, open a minimal public issue asking
for a private channel. Do not include vulnerability details in that issue.

Please include:

- The affected version or commit (`git rev-parse HEAD` for a source build).
- The operating system, architecture, and how fpm-monitor was installed.
- The command line and configuration needed to reproduce the issue.
- The expected and observed behavior, with logs or a proof of concept.
- The impact you believe the issue has, and any suggested fix.

Remove passwords, tokens, private keys, and personal data from logs before
sending them. If a file is essential to the report, redact the sensitive parts.

## What to expect

- We aim to acknowledge a report within 7 days.
- We will investigate, keep you informed, and agree on a disclosure timeline.
- We will credit you in the advisory unless you ask to remain anonymous.
- Please give us a reasonable opportunity to release a fix before public
  disclosure. We aim to publish an advisory once a fix is available.

## Scope

fpm-monitor reads local process, socket, configuration, and log data, and can
serve a local web dashboard with `--watch`. The following areas are in scope:

- The HTTP server: request parsing, path traversal, header injection, response
  splitting, cross-site scripting, JSON/HTML escaping, and denial of service
  through oversized or malformed requests.
- Configuration and log parsing: malformed input, symlinks, path handling,
  unbounded memory, and panics caused by untrusted files.
- The CLI and TUI: terminal escape injection, panics on malformed data, and
  unsafe handling of process or socket information.
- Process and socket collection on Linux, macOS, BSD, and Windows.
- Release artifacts, packaging scripts, GitHub Actions workflows, and the
  Homebrew formula.

Out of scope:

- Vulnerabilities in PHP-FPM, PHP, the operating system, Docker, or other tools
  that are not part of this repository.
- Problems that require an attacker to already have root or administrator access,
  or to have compromised the host.
- Social engineering, spam, and reports without a reproducible technical issue.
- Automated scanner output without a concrete impact or reproduction.

## Security model

- fpm-monitor makes **no outgoing network calls**. It reads local data only.
- The web dashboard binds to `127.0.0.1` and has no authentication. It is meant
  for local use; exposing it through a reverse proxy is not supported without
  adding your own access control.
- The binary has no third-party runtime dependencies. The embedded HTML, fonts,
  and favicon are served from the binary itself.
- Configuration files, log files, process names, and HTTP requests are treated as
  untrusted input.
- fpm-monitor runs with the privileges of the user who starts it. It does not
  need root, and running it as root is not recommended.
- When started by a non-root user, fpm-monitor restricts itself to the pools
  whose `user` directive matches that account (a numeric UID is also accepted).
  A pool without a `user` directive is treated as belonging to the launcher.
  Environment placeholders in `user` (`${NAME}` or `${NAME:-default}`) are
  expanded with the monitor's environment; start it with the same variables as
  the FPM master when that directive uses them. Root sees every pool. Master
  PIDs are hidden for non-root users because they cannot be safely mapped to a
  single pool.
- External commands (`id`, `php-cgi`, `ps`, `netstat`) inherit the launcher's
  identity. To run them as a pool user, start fpm-monitor as that user. Root is
  intentionally unrestricted.
- This filtering only controls what fpm-monitor displays. It does not replace
  Unix file permissions, `hidepid` on `/proc`, or service-level sandboxing.

## Safe harbor

We will not pursue legal action against researchers who act in good faith, avoid
privacy violations, avoid service disruption, and give us a reasonable chance to
fix the issue before disclosure.
