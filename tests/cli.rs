//! End-to-end tests of the CLI surface.
//!
//! They only rely on `--help` and `--mock`, so they run on every platform
//! (Linux, macOS, Windows) without needing PHP-FPM or `/proc`.

use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_fpm-monitor"))
        .args(args)
        .output()
        .expect("failed to run fpm-monitor")
}

fn stdout(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

#[test]
fn help_english() {
    let out = run(&["--help"]);
    assert!(out.status.success());
    let text = stdout(&out);
    assert!(text.contains("Usage: fpm-monitor [OPTIONS]"));
    assert!(text.contains("--tui"));
    assert!(text.contains("--lang"));
    assert!(text.contains("--mock"));
}

#[test]
fn help_is_localized() {
    for (lang, needle) in [
        ("fr", "moniteur de pools PHP-FPM"),
        ("de", "Verwendung: fpm-monitor"),
        ("es", "Uso: fpm-monitor"),
        ("it", "Uso: fpm-monitor"),
        ("ja", "使用法: fpm-monitor"),
        ("zh", "用法: fpm-monitor"),
        ("ar", "الاستخدام: fpm-monitor"),
    ] {
        let out = run(&["--lang", lang, "--help"]);
        assert!(out.status.success(), "--lang {lang} failed");
        assert!(
            stdout(&out).contains(needle),
            "--lang {lang}: missing {needle:?}"
        );
    }
}

#[test]
fn lang_equals_form_is_accepted() {
    let out = run(&["--lang=de", "--help"]);
    assert!(out.status.success());
    assert!(stdout(&out).contains("Verwendung: fpm-monitor"));
}

#[test]
fn unknown_language_exits_with_code_2() {
    let out = run(&["--lang", "xx", "--help"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("unknown language"));
}

#[test]
fn unknown_option_exits_with_code_2() {
    let out = run(&["--definitely-not-an-option"]);
    assert_eq!(out.status.code(), Some(2));
    let text = stderr(&out);
    assert!(text.contains("unknown option"));
    assert!(text.contains("--help"));
}

#[test]
fn mock_prints_the_pool_table() {
    let out = run(&["--mock", "--no-color"]);
    assert!(out.status.success());
    let text = stdout(&out);
    for needle in [
        "Pools", "Workers", "Running", "Idle", "Backlog", "www", "api",
    ] {
        assert!(text.contains(needle), "missing {needle:?} in:\n{text}");
    }
    assert!(text.contains("4242"), "master PID missing");
}

#[test]
fn mock_verbose_adds_worker_details() {
    let short = stdout(&run(&["--mock", "--no-color"]));
    let verbose = stdout(&run(&["--mock", "--no-color", "-v"]));
    assert!(verbose.lines().count() > short.lines().count());
    assert!(verbose.contains("pid="));
    assert!(!short.contains("pid="));
}

#[test]
fn mock_is_localized() {
    let out = run(&["--mock", "--no-color", "--lang", "fr"]);
    assert!(out.status.success());
    let text = stdout(&out);
    assert!(text.contains("Mémoire"));
    assert!(text.contains("PID maître"));
}
