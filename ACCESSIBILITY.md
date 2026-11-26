# Accessibility

fpm-monitor is a local monitoring tool. We want the CLI, the terminal dashboard,
and the web dashboard to be usable without a mouse, without relying on color,
and with assistive technology.

This document describes what is currently supported and what is still limited.
If something here is wrong or incomplete, please open an issue with the
**Accessibility issue** template.

## Web dashboard (`--watch`)

The embedded page at <http://127.0.0.1:8471> includes:

- **Keyboard navigation.** A "Skip to content" link is the first focusable
  element. `Tab` and `Shift+Tab` move between controls. The header tabs form a
  tablist: `1`–`5` select a tab directly, and `←`/`→`, `Home`, and `End` move
  within the tablist. Outside form fields, `←`/`→` cycle the selected pool,
  graph, log, or configuration. `Escape` closes the mobile menu.
- **Visible focus.** Keyboard focus is drawn with a high-contrast outline in
  both themes and inside the tablist, worker summaries, selects, and log
  controls.
- **Focus preservation.** The page polls for fresh data, but it restores the
  focused control and scroll position after each refresh. Selecting a different
  pool or log type does not steal focus from the controls you are using.
- **Accessible names and state.** Buttons, selects, the theme switch, the
  mobile menu, log panels, and copy buttons expose translated labels through
  `aria-label` or visible text. State is exposed with `aria-selected`,
  `aria-expanded`, `aria-pressed`, and `aria-checked`. Decorative icons are
  hidden from assistive technology; the listen-type icons have text labels.
- **Errors and status.** Collection failures appear in a banner with
  `role="alert"`. Updated times and master PIDs are available as text.
- **Color and contrast.** Light and dark themes are provided. Severity levels
  are written as text (`NOTICE`, `WARNING`, `ERROR`) in addition to their
  colors, and HTTP status colors are paired with the numeric status code. The
  CLI also supports `--no-color` and respects `NO_COLOR`.
- **Reduced motion and forced colors.** `prefers-reduced-motion` disables
  transitions and animations. In Windows high-contrast mode, controls keep
  borders and system colors.
- **Language and direction.** The interface is translated into 22 languages.
  `<html lang>` follows the selected locale, and Arabic switches the page to
  right-to-left layout. Labels are translated; code and comments are English.
- **Log controls.** The severity menu uses real checkboxes, with all severities
  selected by default. Copy buttons expose a translated label and a copied
  state. Log terminals are focusable and scrollable by keyboard.

## Terminal interfaces

The CLI table is plain text and works without a terminal emulator that supports
color. Use `--no-color` or set `NO_COLOR` to disable ANSI styling everywhere.

The interactive dashboard (`-t` / `--tui`) is keyboard-driven: `q` quits,
`Tab` moves to the next tab, `1`–`4` select a tab, `←`/`→` change subtab, and
`↑`/`↓` move through monitoring rows.

## Known limitations

- Graphs are drawn as SVG with accessible labels, but they are visual summaries
  rather than full data tables. The same values are available in the metric
  cards and summary text.
- The `phpinfo()` panel is an iframe served by PHP. Its internal markup is not
  controlled by fpm-monitor and may not follow the same accessibility patterns.
- Log panels intentionally do not use `aria-live`: the dashboard refreshes every
  second, and announcing every new line would be disruptive. Focus and scroll
  position are preserved instead.
- Contrast targets WCAG AA where practical, but an independent audit has not
  been published. Report any combination that is hard to read.

## Reporting a problem

Use the **Accessibility issue** template and include:

- The version or commit, operating system, and installation method.
- The browser, screen reader, or other assistive technology you use, plus zoom
  and theme settings if relevant.
- The smallest sequence of steps that shows the barrier.
- What you expected and what happened instead.

Remove personal data, tokens, and private logs before attaching them. Security
problems must be reported privately as described in [SECURITY.md](SECURITY.md).
