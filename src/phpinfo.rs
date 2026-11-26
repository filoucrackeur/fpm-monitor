//! Raw HTML output of `phpinfo()`, for the dashboard's "info" tab.
//!
//! `php-cgi -i` produces exactly the `phpinfo()` HTML document (CGI SAPI),
//! unlike `php -i` / `php-fpm -i`, which produce raw text. We
//! only strip the HTTP headers emitted by the CGI binary.

use std::io::{BufReader, Read};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

/// Usual names and locations of the CGI binary; `php-cgi` is tried first.
const CANDIDATES: &[&str] = &[
    "php-cgi",
    "/opt/homebrew/bin/php-cgi",
    "/usr/local/bin/php-cgi",
    "/usr/bin/php-cgi",
    "/usr/bin/php-cgi8.4",
    "/usr/bin/php-cgi8.3",
    "/usr/bin/php-cgi8.2",
    "/usr/bin/php-cgi8.1",
    "/usr/bin/php-cgi8.0",
    "/usr/bin/php-cgi7.4",
];

const TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Clone, Default)]
pub struct Info {
    /// HTML document as returned by PHP (without the CGI headers).
    pub html: String,
    pub error: Option<String>,
}

/// Collects the `phpinfo()` HTML. In demo mode, a dummy document is enough.
pub fn collect(mock: bool) -> Info {
    if mock {
        return Info {
            html: "<!DOCTYPE html><html><head><title>phpinfo()</title></head>\
                   <body><h1>phpinfo()</h1><p>mock data</p></body></html>"
                .to_string(),
            error: None,
        };
    }
    let mut errors = Vec::new();
    for candidate in CANDIDATES {
        match run(candidate) {
            Ok(raw) => match html_body(&raw) {
                Some(html) => return Info { html, error: None },
                None => errors.push(format!("{candidate}: no HTML in output")),
            },
            Err(e) => errors.push(format!("{candidate}: {e}")),
        }
    }
    Info {
        html: String::new(),
        error: Some(errors.join("; ")),
    }
}

/// Removes the CGI headers (`Content-type`, `X-Powered-By`, …).
fn html_body(raw: &str) -> Option<String> {
    let body = if let Some(pos) = raw.find("\r\n\r\n") {
        &raw[pos + 4..]
    } else if let Some(pos) = raw.find("\n\n") {
        &raw[pos + 2..]
    } else {
        raw
    };
    let body = body.trim_start();
    if body.to_ascii_lowercase().contains("<html")
        || body.to_ascii_lowercase().contains("<!doctype")
    {
        Some(body.to_string())
    } else {
        None
    }
}

fn run(binary: &str) -> Result<String, String> {
    let mut child = Command::new(binary)
        .arg("-i")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| e.to_string())?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "stdout unavailable".to_string())?;
    // The document is ~100 KB: drain the pipe while the process
    // is running, otherwise PHP blocks on writing before it can exit.
    let reader = thread::spawn(move || {
        let mut text = String::new();
        BufReader::new(stdout)
            .read_to_string(&mut text)
            .map(|_| text)
    });

    let deadline = Instant::now() + TIMEOUT;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => {}
            Err(e) => {
                let _ = child.kill();
                return Err(e.to_string());
            }
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err("timed out".to_string());
        }
        thread::sleep(Duration::from_millis(20));
    };
    let text = reader
        .join()
        .map_err(|_| "output reader failed".to_string())?
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err(format!("exit status {status}"));
    }
    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_has_html() {
        let info = collect(true);
        assert!(info.html.contains("phpinfo()"));
        assert!(info.error.is_none());
    }

    #[test]
    fn html_body_strips_cgi_headers() {
        let raw = "X-Powered-By: PHP/8.6.0\r\nContent-type: text/html; charset=UTF-8\r\n\r\n<!DOCTYPE html><html><body>ok</body></html>";
        assert_eq!(
            html_body(raw).as_deref(),
            Some("<!DOCTYPE html><html><body>ok</body></html>")
        );
        let raw = "Content-type: text/html\n\n<html><body>ok</body></html>";
        assert_eq!(
            html_body(raw).as_deref(),
            Some("<html><body>ok</body></html>")
        );
    }

    #[test]
    fn html_body_rejects_plain_text() {
        assert_eq!(html_body("phpinfo()\nPHP Version => 8.6.0\n"), None);
    }
}
