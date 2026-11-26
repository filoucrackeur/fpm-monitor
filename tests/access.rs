#![cfg(unix)]
//! End-to-end checks of per-pool isolation for non-root launchers.

use std::fs;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_fpm-monitor"))
        .args(args)
        .output()
        .expect("failed to run fpm-monitor")
}

fn command_text(args: &[&str]) -> Option<String> {
    let output = Command::new("id").args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    (!text.is_empty()).then_some(text)
}

#[test]
fn non_root_launcher_only_sees_own_pool() {
    // CI often runs as root; root is intentionally unrestricted.
    if command_text(&["-u"]).as_deref() == Some("0") {
        return;
    }
    let Some(user) = command_text(&["-un"]) else {
        return;
    };

    let mut dir = std::env::temp_dir();
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    dir.push(format!("fpm-access-test-{}-{stamp}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();

    fs::write(
        dir.join("own.conf"),
        format!("[own]\nuser = {user}\nlisten = 9000\n"),
    )
    .unwrap();
    fs::write(
        dir.join("other.conf"),
        format!("[other]\nuser = not-{user}\nlisten = 9001\n"),
    )
    .unwrap();

    let path = dir.to_string_lossy().into_owned();
    let out = run(&["-c", &path, "--no-color"]);
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    let errors = String::from_utf8_lossy(&out.stderr).into_owned();

    assert!(out.status.success(), "fpm-monitor failed: {errors}");
    assert!(text.contains("own"), "own pool missing from:\n{text}");
    assert!(!text.contains("other"), "other pool leaked into:\n{text}");

    fs::remove_dir_all(&dir).ok();
}
