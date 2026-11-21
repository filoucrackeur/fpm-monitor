//! Web UI mode (`--watch`): a tiny dependency-free HTTP server, a background
//! sampler and a JSON snapshot consumed by the embedded single-page app.
//!
//! Everything is standard library only. The server binds to loopback, serves
//! three routes and never makes an outgoing request.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::config::Config;
use crate::data::{self, Row};
use crate::{i18n, logs, proc};

/// Number of history points kept for the charts.
pub const HISTORY_LEN: usize = 60;
/// Default web port.
pub const DEFAULT_PORT: u16 = 8471;

const INDEX_HTML: &str = include_str!("web/index.html");

#[derive(Clone)]
pub struct WorkerView {
    pub pid: i32,
    pub state: char,
    pub rss_kb: i64,
}

#[derive(Clone)]
pub struct PoolView {
    pub name: String,
    pub pm: Option<String>,
    pub workers: usize,
    pub running: usize,
    pub idle: usize,
    pub backlog: Option<u32>,
    pub backlog_max: Option<u32>,
    pub max_children: Option<u32>,
    pub max_requests: Option<u32>,
    pub min_spare: Option<u32>,
    pub max_spare: Option<u32>,
    pub memory_kb: i64,
    pub worker_list: Vec<WorkerView>,
}

#[derive(Clone)]
pub struct LogView {
    pub name: String,
    pub php_path: Option<String>,
    pub slow_path: Option<String>,
    pub access_path: Option<String>,
    pub php: Vec<String>,
    pub slow: Vec<String>,
    pub access: Vec<String>,
}

#[derive(Clone)]
pub struct PoolConfigView {
    pub name: String,
    pub directives: Vec<(String, String)>,
}

#[derive(Clone, Default)]
pub struct ConfigView {
    pub globals: Vec<(String, String)>,
    pub pools: Vec<PoolConfigView>,
}

#[derive(Clone)]
pub struct HistPool {
    pub name: String,
    pub workers: usize,
    pub running: usize,
    pub idle: usize,
    pub backlog: Option<u32>,
}

#[derive(Clone)]
pub struct Sample {
    pub time: u64,
    pub pools: Vec<HistPool>,
}

pub struct State {
    pub mock: bool,
    pub interval: u64,
    pub updated: u64,
    pub error: Option<String>,
    pub masters: Vec<i32>,
    pub pools: Vec<PoolView>,
    pub logs: Vec<LogView>,
    pub config: ConfigView,
    pub history: Vec<Sample>,
}

impl State {
    fn new(collected: Collected, interval: u64, mock: bool) -> State {
        let now = now_secs();
        let mut state = State {
            mock,
            interval,
            updated: now,
            error: collected.error,
            masters: collected.masters,
            pools: collected.pools,
            logs: collected.logs,
            config: collected.config,
            history: Vec::new(),
        };
        state.record(now);
        state
    }

    fn record(&mut self, now: u64) {
        if self.error.is_some() {
            return;
        }
        let pools = self
            .pools
            .iter()
            .map(|p| HistPool {
                name: p.name.clone(),
                workers: p.workers,
                running: p.running,
                idle: p.idle,
                backlog: p.backlog,
            })
            .collect();
        self.history.push(Sample { time: now, pools });
        if self.history.len() > HISTORY_LEN {
            self.history.remove(0);
        }
    }

    fn apply(&mut self, collected: Collected) {
        let now = now_secs();
        self.updated = now;
        self.error = collected.error;
        self.config = collected.config;
        if self.error.is_none() {
            self.masters = collected.masters;
            self.pools = collected.pools;
            self.logs = collected.logs;
        }
        self.record(now);
    }

    /// Serialises the whole snapshot as JSON (the shape the page expects).
    pub fn to_json(&self) -> String {
        let pools: Vec<String> = self.pools.iter().map(pool_json).collect();
        let logs: Vec<String> = self.logs.iter().map(log_json).collect();
        let pool_configs: Vec<String> = self
            .config
            .pools
            .iter()
            .map(|p| {
                format!(
                    "{{\"name\":\"{}\",\"directives\":{}}}",
                    esc(&p.name),
                    pairs_json(&p.directives)
                )
            })
            .collect();
        let history: Vec<String> = self
            .history
            .iter()
            .map(|s| {
                let points: Vec<String> = s
                    .pools
                    .iter()
                    .map(|p| {
                        format!(
                            "{{\"name\":\"{}\",\"workers\":{},\"running\":{},\"idle\":{},\"backlog\":{}}}",
                            esc(&p.name),
                            p.workers,
                            p.running,
                            p.idle,
                            opt_u32(p.backlog)
                        )
                    })
                    .collect();
                format!("{{\"time\":{},\"pools\":[{}]}}", s.time, points.join(","))
            })
            .collect();
        let masters: Vec<String> = self.masters.iter().map(|m| m.to_string()).collect();
        format!(
            "{{\"mock\":{},\"interval\":{},\"updated\":{},\"error\":{},\"masters\":[{}],\
             \"pools\":[{}],\"logs\":[{}],\
             \"config\":{{\"globals\":{},\"pools\":[{}]}},\"history\":[{}]}}",
            self.mock,
            self.interval,
            self.updated,
            opt_str(self.error.as_deref()),
            masters.join(","),
            pools.join(","),
            logs.join(","),
            pairs_json(&self.config.globals),
            pool_configs.join(","),
            history.join(",")
        )
    }
}

pub struct Response {
    pub status: u16,
    pub content_type: &'static str,
    pub body: Vec<u8>,
}

impl Response {
    fn new(status: u16, content_type: &'static str, body: impl Into<Vec<u8>>) -> Response {
        Response {
            status,
            content_type,
            body: body.into(),
        }
    }

    fn reason(&self) -> &'static str {
        match self.status {
            200 => "OK",
            204 => "No Content",
            400 => "Bad Request",
            404 => "Not Found",
            405 => "Method Not Allowed",
            _ => "Internal Server Error",
        }
    }
}

/// Accepts a port in the `1-65535` range. `None` for anything else,
/// including `0` (the OS would pick an ephemeral port, which the CLI cannot
/// advertise) and non-numeric values.
pub fn parse_port(value: &str) -> Option<u16> {
    match value.parse::<u32>() {
        Ok(n) if (1..=65535).contains(&n) => Some(n as u16),
        _ => None,
    }
}

/// Starts the web UI and blocks forever (Ctrl-C terminates the process).
pub fn run(cfg: Config, interval: Duration, port: u16, mock: bool) -> std::io::Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    let secs = interval.as_secs().max(1);
    println!("{}", i18n::l().web_serving.replace("{}", &port.to_string()));
    let state = Arc::new(Mutex::new(State::new(collect(&cfg, mock), secs, mock)));
    spawn_sampler(cfg, interval, mock, Arc::clone(&state));
    for stream in listener.incoming().flatten() {
        let state = Arc::clone(&state);
        thread::spawn(move || handle_conn(stream, state));
    }
    Ok(())
}

fn spawn_sampler(cfg: Config, interval: Duration, mock: bool, state: Arc<Mutex<State>>) {
    thread::spawn(move || loop {
        thread::sleep(interval);
        let collected = collect(&cfg, mock);
        let mut guard = state.lock().unwrap_or_else(|e| e.into_inner());
        guard.apply(collected);
    });
}

struct Collected {
    error: Option<String>,
    masters: Vec<i32>,
    pools: Vec<PoolView>,
    logs: Vec<LogView>,
    config: ConfigView,
}

fn collect(cfg: &Config, mock: bool) -> Collected {
    if mock {
        let rows = data::mock_rows(cfg);
        let mut names: Vec<String> = rows.iter().map(|r| r.name.clone()).collect();
        names.sort();
        return Collected {
            error: None,
            masters: vec![4242],
            pools: pool_views(&rows),
            logs: logs::mock(&names).iter().map(LogView::from).collect(),
            config: config_view(cfg),
        };
    }

    let scan = match proc::scan() {
        Ok(scan) => scan,
        Err(e) => {
            return Collected {
                error: Some(e.to_string()),
                masters: Vec::new(),
                pools: Vec::new(),
                logs: Vec::new(),
                config: config_view(cfg),
            }
        }
    };
    let masters = scan.masters.clone();
    let rows = data::build_rows(scan, cfg);
    let names: Vec<String> = rows.iter().map(|r| r.name.clone()).collect();
    Collected {
        error: None,
        masters,
        pools: pool_views(&rows),
        logs: logs::collect(cfg, &names, 100)
            .iter()
            .map(LogView::from)
            .collect(),
        config: config_view(cfg),
    }
}

impl From<&logs::PoolLogs> for LogView {
    fn from(l: &logs::PoolLogs) -> LogView {
        LogView {
            name: l.name.clone(),
            php_path: l.php_log.clone(),
            slow_path: l.slow_log.clone(),
            access_path: l.access_log.clone(),
            php: l.php.clone(),
            slow: l.slow.clone(),
            access: l.access.clone(),
        }
    }
}

fn pool_views(rows: &[Row]) -> Vec<PoolView> {
    rows.iter()
        .map(|r| {
            let memory_kb = r
                .scan
                .as_ref()
                .map(|s| s.workers.iter().map(|w| w.rss_kb.max(0)).sum())
                .unwrap_or(0);
            let worker_list = r
                .scan
                .as_ref()
                .map(|s| {
                    s.workers
                        .iter()
                        .map(|w| WorkerView {
                            pid: w.pid,
                            state: w.state,
                            rss_kb: w.rss_kb,
                        })
                        .collect()
                })
                .unwrap_or_default();
            PoolView {
                name: r.name.clone(),
                pm: r.cfg.as_ref().and_then(|c| c.pm.clone()),
                workers: r.workers,
                running: r.running,
                idle: r.idle,
                backlog: r.backlog,
                backlog_max: r.backlog_max,
                max_children: r.cfg.as_ref().and_then(|c| c.max_children),
                max_requests: r.cfg.as_ref().and_then(|c| c.max_requests),
                min_spare: r.cfg.as_ref().and_then(|c| c.min_spare_servers),
                max_spare: r.cfg.as_ref().and_then(|c| c.max_spare_servers),
                memory_kb,
                worker_list,
            }
        })
        .collect()
}

fn config_view(cfg: &Config) -> ConfigView {
    let mut pools: Vec<PoolConfigView> = cfg
        .pools
        .iter()
        .map(|(name, p)| PoolConfigView {
            name: name.clone(),
            directives: p.raw.clone(),
        })
        .collect();
    pools.sort_by(|a, b| a.name.cmp(&b.name));
    ConfigView {
        globals: cfg.globals.clone(),
        pools,
    }
}

/// Routes a request path. Query strings are ignored.
pub fn route(state: &State, raw_path: &str) -> Response {
    let path = raw_path.split('?').next().unwrap_or("");
    match path {
        "/" | "/index.html" => Response::new(200, "text/html; charset=utf-8", INDEX_HTML),
        "/api/state" => Response::new(200, "application/json; charset=utf-8", state.to_json()),
        "/favicon.ico" => Response::new(204, "", Vec::new()),
        _ => Response::new(404, "text/plain; charset=utf-8", "not found\n"),
    }
}

fn handle_conn(mut stream: TcpStream, state: Arc<Mutex<State>>) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(5)));
    let mut reader = match stream.try_clone() {
        Ok(clone) => BufReader::new(clone),
        Err(_) => return,
    };

    let mut line = String::new();
    if reader.read_line(&mut line).unwrap_or(0) == 0 {
        return;
    }
    // Drain headers (bounded) until the empty line.
    for _ in 0..64 {
        let mut header = String::new();
        match reader.read_line(&mut header) {
            Ok(0) => break,
            Ok(_) if header == "\r\n" || header == "\n" => break,
            Ok(_) => {}
            Err(_) => break,
        }
    }

    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");
    if method.is_empty() || path.is_empty() {
        let _ = write_response(
            &mut stream,
            &Response::new(400, "text/plain; charset=utf-8", "bad request\n"),
            false,
        );
        return;
    }
    if method != "GET" && method != "HEAD" {
        let _ = write_response(
            &mut stream,
            &Response::new(405, "text/plain; charset=utf-8", "method not allowed\n"),
            false,
        );
        return;
    }

    let response = {
        let guard = state.lock().unwrap_or_else(|e| e.into_inner());
        route(&guard, path)
    };
    let _ = write_response(&mut stream, &response, method == "HEAD");
}

fn write_response(stream: &mut TcpStream, response: &Response, head: bool) -> std::io::Result<()> {
    let mut out: Vec<u8> = Vec::with_capacity(response.body.len() + 128);
    out.extend_from_slice(
        format!("HTTP/1.1 {} {}\r\n", response.status, response.reason()).as_bytes(),
    );
    if !response.content_type.is_empty() {
        out.extend_from_slice(format!("Content-Type: {}\r\n", response.content_type).as_bytes());
    }
    out.extend_from_slice(
        format!(
            "Content-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n",
            response.body.len()
        )
        .as_bytes(),
    );
    if response.status == 405 {
        out.extend_from_slice(b"Allow: GET, HEAD\r\n");
    }
    out.extend_from_slice(b"\r\n");
    if !head {
        out.extend_from_slice(&response.body);
    }
    stream.write_all(&out)
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn opt_str(value: Option<&str>) -> String {
    match value {
        Some(v) => format!("\"{}\"", esc(v)),
        None => "null".to_string(),
    }
}

fn opt_u32(value: Option<u32>) -> String {
    match value {
        Some(v) => v.to_string(),
        None => "null".to_string(),
    }
}

fn strings_json(values: &[String]) -> String {
    let items: Vec<String> = values.iter().map(|v| format!("\"{}\"", esc(v))).collect();
    format!("[{}]", items.join(","))
}

fn pairs_json(pairs: &[(String, String)]) -> String {
    let items: Vec<String> = pairs
        .iter()
        .map(|(k, v)| format!("[\"{}\",\"{}\"]", esc(k), esc(v)))
        .collect();
    format!("[{}]", items.join(","))
}

fn pool_json(p: &PoolView) -> String {
    let workers: Vec<String> = p
        .worker_list
        .iter()
        .map(|w| {
            format!(
                "{{\"pid\":{},\"state\":\"{}\",\"rss_kb\":{}}}",
                w.pid,
                esc(&w.state.to_string()),
                w.rss_kb
            )
        })
        .collect();
    format!(
        "{{\"name\":\"{}\",\"pm\":{},\"workers\":{},\"running\":{},\"idle\":{},\
         \"backlog\":{},\"backlog_max\":{},\"max_children\":{},\"max_requests\":{},\
         \"min_spare\":{},\"max_spare\":{},\"memory_kb\":{},\"worker_list\":[{}]}}",
        esc(&p.name),
        opt_str(p.pm.as_deref()),
        p.workers,
        p.running,
        p.idle,
        opt_u32(p.backlog),
        opt_u32(p.backlog_max),
        opt_u32(p.max_children),
        opt_u32(p.max_requests),
        opt_u32(p.min_spare),
        opt_u32(p.max_spare),
        p.memory_kb,
        workers.join(",")
    )
}

fn log_json(l: &LogView) -> String {
    format!(
        "{{\"name\":\"{}\",\"php_path\":{},\"slow_path\":{},\"access_path\":{},\
         \"php\":{},\"slow\":{},\"access\":{}}}",
        esc(&l.name),
        opt_str(l.php_path.as_deref()),
        opt_str(l.slow_path.as_deref()),
        opt_str(l.access_path.as_deref()),
        strings_json(&l.php),
        strings_json(&l.slow),
        strings_json(&l.access)
    )
}

/// Tiny helper used to keep the mock log collection readable.
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use std::net::TcpListener;

    fn mock_state() -> State {
        State::new(collect(&data::mock_config(), true), 1, true)
    }

    fn http_get(request: &str) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let state = Arc::new(Mutex::new(mock_state()));
        let server = thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            handle_conn(stream, state);
        });
        let mut stream = TcpStream::connect(addr).unwrap();
        stream.write_all(request.as_bytes()).unwrap();
        let mut out = String::new();
        stream.read_to_string(&mut out).unwrap();
        server.join().unwrap();
        out
    }

    #[test]
    fn parse_port_accepts_valid_values() {
        assert_eq!(parse_port("1"), Some(1));
        assert_eq!(parse_port("8471"), Some(8471));
        assert_eq!(parse_port("65535"), Some(65535));
    }

    #[test]
    fn parse_port_rejects_invalid_values() {
        for value in ["", "0", "65536", "-1", "abc", "84 71", "12.5"] {
            assert_eq!(parse_port(value), None, "{value}");
        }
    }

    #[test]
    fn json_escapes_control_characters() {
        assert_eq!(esc("a\"b\\c\n\t"), "a\\\"b\\\\c\\n\\t");
        assert_eq!(esc("é"), "é");
        assert!(esc("\u{1}").starts_with("\\u0001"));
    }

    #[test]
    fn route_serves_the_page_and_the_api() {
        let state = mock_state();
        let page = route(&state, "/");
        assert_eq!(page.status, 200);
        assert_eq!(page.content_type, "text/html; charset=utf-8");
        assert!(String::from_utf8_lossy(&page.body).contains("fpm-monitor"));

        let api = route(&state, "/api/state");
        assert_eq!(api.status, 200);
        assert_eq!(api.content_type, "application/json; charset=utf-8");
        let json = String::from_utf8_lossy(&api.body);
        assert!(json.starts_with('{'));
        assert!(json.contains("\"pools\""));
    }

    #[test]
    fn route_uses_the_first_path_segment_only() {
        let state = mock_state();
        assert_eq!(route(&state, "/?tab=graphs").status, 200);
        assert_eq!(route(&state, "/api/state?x=1").status, 200);
        assert_eq!(route(&state, "/index.html").status, 200);
    }

    #[test]
    fn route_serves_favicon_and_404() {
        let state = mock_state();
        let icon = route(&state, "/favicon.ico");
        assert_eq!(icon.status, 204);
        assert!(icon.body.is_empty());
        assert_eq!(route(&state, "/nope").status, 404);
    }

    #[test]
    fn snapshot_json_exposes_pools_logs_and_config() {
        let state = mock_state();
        let json = state.to_json();
        assert!(json.contains("\"mock\":true"));
        assert!(json.contains("\"name\":\"www\""));
        assert!(json.contains("\"worker_list\""));
        assert!(json.contains("\"max_children\":8"));
        assert!(json.contains("\"php_log\"") || json.contains("\"php_path\""));
        assert!(json.contains("\"globals\""));
        assert!(json.contains("\"history\""));
        assert!(json.contains("\"error\":null"));
    }

    #[test]
    fn history_is_capped() {
        let cfg = data::mock_config();
        let mut state = State::new(collect(&cfg, true), 1, true);
        for _ in 0..HISTORY_LEN * 2 {
            state.apply(collect(&cfg, true));
        }
        assert_eq!(state.history.len(), HISTORY_LEN);
        assert!(!state.history.is_empty());
    }

    #[test]
    fn error_does_not_append_history() {
        let cfg = data::mock_config();
        let mut state = State::new(collect(&cfg, true), 1, true);
        let before = state.history.len();
        state.apply(Collected {
            error: Some("boom".to_string()),
            masters: Vec::new(),
            pools: Vec::new(),
            logs: Vec::new(),
            config: config_view(&cfg),
        });
        assert_eq!(state.error.as_deref(), Some("boom"));
        assert_eq!(state.history.len(), before);
        assert!(state.to_json().contains("\"error\":\"boom\""));
    }

    #[test]
    fn config_view_lists_globals_and_sorted_pools() {
        let cfg = data::mock_config();
        let view = config_view(&cfg);
        assert!(!view.globals.is_empty());
        let names: Vec<&str> = view.pools.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, vec!["api", "app", "www"]);
        assert!(!view.pools[0].directives.is_empty());
    }

    #[test]
    fn http_get_returns_the_index() {
        let response = http_get("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n");
        assert!(response.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(response.contains("Content-Type: text/html; charset=utf-8"));
        assert!(response.contains("fpm-monitor"));
        assert!(response.contains("id=\"theme\""));
        assert!(response.contains("data-theme"));
        assert!(response.contains("details.workers"));
        assert!(response.contains("openPools"));
    }

    #[test]
    fn http_head_has_headers_but_no_body() {
        let response = http_get("HEAD / HTTP/1.1\r\nHost: localhost\r\n\r\n");
        assert!(response.starts_with("HTTP/1.1 200 OK\r\n"));
        let (headers, body) = response.split_once("\r\n\r\n").unwrap();
        assert!(headers.contains("Content-Length:"));
        assert!(body.is_empty());
    }

    #[test]
    fn http_rejects_unsupported_methods() {
        let response = http_get("POST /api/state HTTP/1.1\r\nHost: localhost\r\n\r\n");
        assert!(response.starts_with("HTTP/1.1 405 Method Not Allowed\r\n"));
        assert!(response.contains("Allow: GET, HEAD"));
    }

    #[test]
    fn http_returns_404_for_unknown_paths() {
        let response = http_get("GET /missing HTTP/1.1\r\nHost: localhost\r\n\r\n");
        assert!(response.starts_with("HTTP/1.1 404 Not Found\r\n"));
    }
}
