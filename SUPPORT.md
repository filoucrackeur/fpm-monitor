# Support

Thanks for using fpm-monitor. Here is where to find help and how to report a
problem.

## Read the documentation first

- The [README](README.md) covers installation, options, the TUI, and the web
  dashboard. It is translated into 22 languages.
- `fpm-monitor --help` lists every option and the accepted language codes.
- The [project structure](README.md#project-structure) explains where each kind
  of data is read.
- The [contributing guide](CONTRIBUTING.md) explains how to build and test the
  project.

## Ask a question

Use [GitHub Discussions](https://github.com/filoucrackeur/fpm-monitor/discussions)
for usage questions, configuration help, ideas, and general feedback. Please do
not open an issue for a question that has not been diagnosed as a bug yet.

## Report a bug

Open an issue with the **Bug report** template:

<https://github.com/filoucrackeur/fpm-monitor/issues/new/choose>

Include the version or commit, operating system, architecture, installation
method, the exact command, and the smallest reproduction you can find. A
reproduction with `--mock` is ideal when the problem is in the interface, the
TUI, or the web dashboard.

Before posting logs, remove passwords, tokens, private keys, and personal data.
If you can reproduce the problem with demo data, mention it:

```sh
fpm-monitor --mock --no-color
fpm-monitor --mock -t
fpm-monitor --mock --watch
```

## Request a feature

Open an issue with the **Feature request** template. Explain the problem you are
trying to solve, not only the solution you have in mind. Small, focused proposals
are easier to discuss.

## Report a security issue

Do **not** open a public issue. Follow [SECURITY.md](SECURITY.md) and use GitHub's
private vulnerability reporting.

## Report a code of conduct issue

Follow [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). Reports are handled privately by
the maintainer.

## Support expectations

This is a volunteer project. There is no paid support, service-level agreement,
or guaranteed response time. Issues and discussions are usually answered on a
best-effort basis. A clear report with a reproduction is the fastest way to get
help.
