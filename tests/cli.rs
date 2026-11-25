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
    assert!(text.contains("--watch"));
    assert!(text.contains("--port"));
    assert!(text.contains("--lang"));
    assert!(text.contains("--mock"));
    assert!(text.contains("--version"));
}

#[test]
fn version_flag_prints_the_current_version() {
    let expected = format!("fpm-monitor {}\n", env!("CARGO_PKG_VERSION"));
    for flag in ["--version", "-V"] {
        let out = run(&[flag]);
        assert!(out.status.success(), "{flag} failed");
        assert_eq!(stdout(&out), expected, "{flag} output");
        assert!(stderr(&out).is_empty(), "{flag} wrote to stderr");
    }
}

#[test]
fn invalid_port_exits_with_code_2() {
    for value in ["0", "65536", "abc", "-1", "12.5"] {
        let out = run(&["--mock", "--port", value]);
        assert_eq!(out.status.code(), Some(2), "--port {value}");
        let text = stderr(&out);
        assert!(text.contains("invalid port"), "--port {value}: {text:?}");
        assert!(text.contains(value), "--port {value}: value missing");
    }
}

#[test]
fn invalid_port_is_localized() {
    let out = run(&["--mock", "--port", "0", "--lang", "fr"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("port invalide"));
}

#[test]
fn port_equals_form_is_validated() {
    let out = run(&["--mock", "--port=0"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("invalid port"));
}

#[test]
fn watch_serves_the_dashboard_and_the_api() {
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::time::{Duration, Instant};

    // Reserve a free loopback port, then hand it to the child process.
    let probe = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = probe.local_addr().unwrap().port();
    drop(probe);

    let mut child = Command::new(env!("CARGO_BIN_EXE_fpm-monitor"))
        .args([
            "--mock",
            "--watch",
            "--port",
            &port.to_string(),
            "--lang",
            "en",
            "--interval",
            "1",
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("failed to start the web UI");

    let deadline = Instant::now() + Duration::from_secs(10);
    let mut stream = loop {
        match TcpStream::connect(("127.0.0.1", port)) {
            Ok(stream) => break stream,
            Err(e) if Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(50));
                let _ = e;
            }
            Err(e) => {
                let _ = child.kill();
                let _ = child.wait();
                panic!("web UI did not start on port {port}: {e}");
            }
        }
    };
    stream
        .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .unwrap();
    let mut page = String::new();
    stream.read_to_string(&mut page).unwrap();
    assert!(page.starts_with("HTTP/1.1 200 OK\r\n"), "{page}");
    assert!(page.contains("fpm-monitor"));
    assert!(page.contains("api/state"));

    let mut api = TcpStream::connect(("127.0.0.1", port)).unwrap();
    api.write_all(b"GET /api/state HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .unwrap();
    let mut body = String::new();
    api.read_to_string(&mut body).unwrap();
    let _ = child.kill();
    let _ = child.wait();

    assert!(body.starts_with("HTTP/1.1 200 OK\r\n"), "{body}");
    assert!(body.contains("\"pools\""));
    assert!(body.contains("\"www\""));
    assert!(body.contains("\"history\""));
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
        ("ru", "Использование: fpm-monitor"),
        ("hi", "उपयोग: fpm-monitor"),
        ("id", "Penggunaan: fpm-monitor"),
        ("ko", "사용법: fpm-monitor"),
        ("no", "Bruk: fpm-monitor"),
        ("sv", "Användning: fpm-monitor"),
        ("fi", "Käyttö: fpm-monitor"),
        ("da", "Anvendelse: fpm-monitor"),
        ("el", "Χρήση: fpm-monitor"),
        ("tr", "Kullanım: fpm-monitor"),
        ("pl", "Użycie: fpm-monitor"),
        ("uk", "Використання: fpm-monitor"),
        ("th", "การใช้งาน: fpm-monitor"),
        ("vi", "Cách dùng: fpm-monitor"),
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
fn every_locale_lists_all_language_codes() {
    const LANGS: [&str; 22] = [
        "en", "fr", "zh", "ar", "es", "it", "ja", "de", "ru", "hi", "id", "ko", "no", "sv", "fi",
        "da", "el", "tr", "pl", "uk", "th", "vi",
    ];
    for lang in LANGS {
        let out = run(&["--lang", lang, "--help"]);
        assert!(out.status.success(), "--lang {lang} failed");
        let text = stdout(&out);
        assert!(
            text.contains("en (US)"),
            "--lang {lang}: missing the en (US) label"
        );
        for code in LANGS {
            assert!(text.contains(code), "--lang {lang}: missing code {code:?}");
        }
    }
}

#[test]
fn mock_output_is_translated_for_every_language() {
    let english = stdout(&run(&["--mock", "--no-color"]));
    for lang in [
        "fr", "zh", "ar", "es", "it", "ja", "de", "ru", "hi", "id", "ko", "no", "sv", "fi", "da",
        "el", "tr", "pl", "uk", "th", "vi",
    ] {
        let text = stdout(&run(&["--mock", "--no-color", "--lang", lang]));
        assert_ne!(text, english, "--lang {lang} still prints English labels");
        assert!(text.contains("4242"), "--lang {lang}: master PID missing");
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
