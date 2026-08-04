use crate::config::Config;
use crate::data::Row;
use crate::i18n;
use crate::logs::PoolLogs;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Palette Ember (#FF6B35 = cadre/accents)
const EMBER: &str = "\x1b[38;2;255;107;53m";
const SUBTLE: &str = "\x1b[38;2;160;137;110m";
const AMBER: &str = "\x1b[38;2;255;170;0m";
const RED: &str = "\x1b[38;2;255;68;68m";
const GREEN: &str = "\x1b[38;2;68;204;68m";
const ROSE: &str = "\x1b[38;2;255;105;180m"; // min_spare
const VIOLET: &str = "\x1b[38;2;148;112;219m"; // max_spare
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

fn tabs() -> [&'static str; 4] {
    let l = i18n::l();
    [l.tab_monitoring, l.tab_graphs, l.tab_logs, l.tab_configs]
}

struct Sample {
    t: u64,                                    // timestamp (epoch secs)
    points: Vec<(String, u32, u32, u32, u32)>, // name, workers, running, idle, backlog
}

#[derive(Default)]
struct TuiState {
    tab: usize,
    history: Vec<Sample>,
    sub: usize,    // sous-onglet courant (pool ou section config)
    scroll: usize, // défilement de l'onglet monitoring
    esc: u8,       // état du parser de séquences d'échappement (flèches)
}

pub fn run<F>(mut refresh: F, cfg: &Config, interval: Duration)
where
    F: FnMut() -> (Vec<Row>, Vec<i32>, Vec<PoolLogs>) + Send + 'static,
{
    let raw_ok = raw_mode(true);
    let _guard = RawGuard(raw_ok);

    // Thread de rafraîchissement : FastCGI + /proc peuvent prendre du temps
    // (timeouts), il ne faut donc jamais bloquer le rendu dessus.
    let (dtx, drx) = mpsc::channel::<(Vec<Row>, Vec<i32>, Vec<PoolLogs>)>();
    thread::spawn(move || loop {
        // Cadence fixe : on ne cumule pas la durée du refresh avec l'intervalle,
        // sinon une lecture FastCGI lente espacerait trop les mesures.
        let start = std::time::Instant::now();
        let data = refresh();
        if dtx.send(data).is_err() {
            break;
        }
        if let Some(rem) = interval.checked_sub(start.elapsed()) {
            thread::sleep(rem);
        }
    });

    let (ktx, krx) = mpsc::channel::<u8>();
    thread::spawn(move || {
        let mut buf = [0u8; 1];
        loop {
            match io::stdin().read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(_) => {
                    if ktx.send(buf[0]).is_err() {
                        break;
                    }
                }
            }
        }
    });

    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut state = TuiState::default();

    // Première fournée de données (bloque jusqu'à la première mesure).
    let (mut rows, mut masters, mut logs) = drx.recv().unwrap_or_default();
    push_history(&mut state, &rows);

    let mut quit = false;
    while !quit {
        let size = term_size();
        render(&mut out, cfg, &mut state, &rows, &masters, &logs, size);
        let _ = out.flush();

        while let Ok((r, m, l)) = drx.try_recv() {
            rows = r;
            masters = m;
            logs = l;
            push_history(&mut state, &rows);
        }
        while let Ok(b) = krx.try_recv() {
            if !handle_key(b, &mut state) {
                quit = true;
            }
        }
        thread::sleep(Duration::from_millis(40));
    }

    cleanup(&mut out);
}

fn handle_key(b: u8, state: &mut TuiState) -> bool {
    match state.esc {
        0 => match b {
            b'q' | b'Q' | 3 => false,
            b'\x1b' => {
                state.esc = 1;
                true
            }
            b'\t' => {
                state.tab = (state.tab + 1) % tabs().len();
                true
            }
            b'1' => {
                state.tab = 0;
                true
            }
            b'2' => {
                state.tab = 1;
                true
            }
            b'3' => {
                state.tab = 2;
                true
            }
            b'4' => {
                state.tab = 3;
                true
            }
            _ => true,
        },
        1 => {
            if b == b'[' {
                state.esc = 2;
            } else {
                state.esc = 0;
            }
            true
        }
        2 => {
            state.esc = 0;
            match b {
                b'A' => {
                    if state.tab == 0 {
                        state.scroll = state.scroll.saturating_sub(1);
                    }
                    true
                }
                b'B' => {
                    if state.tab == 0 {
                        state.scroll += 1;
                    }
                    true
                }
                b'C' => {
                    if state.tab != 0 {
                        state.sub += 1;
                    }
                    true
                }
                b'D' => {
                    if state.tab != 0 {
                        state.sub = state.sub.saturating_sub(1);
                    }
                    true
                }
                _ => true,
            }
        }
        _ => {
            state.esc = 0;
            true
        }
    }
}

fn raw_mode(on: bool) -> bool {
    let mut r = Command::new("stty");
    let r = if on {
        r.args(["raw", "-echo"])
    } else {
        r.arg("sane")
    };
    r.status().map(|s| s.success()).unwrap_or(false)
}

struct RawGuard(bool);
impl Drop for RawGuard {
    fn drop(&mut self) {
        if self.0 {
            let _ = raw_mode(false);
        }
    }
}

fn term_size() -> (usize, usize) {
    // `output()` ferme stdin par défaut : il faut l'hériter pour que
    // `stty size` lise la taille du terminal depuis le tty.
    let out = Command::new("stty")
        .arg("size")
        .stdin(Stdio::inherit())
        .output();
    if let Ok(out) = out {
        let s = String::from_utf8_lossy(&out.stdout);
        let mut it = s.split_whitespace();
        if let (Some(r), Some(c)) = (it.next(), it.next()) {
            if let (Ok(r), Ok(c)) = (r.parse::<usize>(), c.parse::<usize>()) {
                return (r.max(10), c.max(20));
            }
        }
    }
    (24, 80)
}

fn cleanup(out: &mut dyn Write) {
    let _ = out.write_all(b"\x1b[0m\x1b[?25h\x1b[2J\x1b[H");
    let _ = out.flush();
}

fn render(
    out: &mut dyn Write,
    cfg: &Config,
    state: &mut TuiState,
    rows: &[Row],
    masters: &[i32],
    logs: &[PoolLogs],
    size: (usize, usize),
) {
    let (h, w) = size;
    let mut s = String::new();
    s.push_str("\x1b[?25l\x1b[2J\x1b[H");

    top_border(&mut s, w);
    line(
        &mut s,
        w,
        &format!("{}   {}", title(), tabs_line(state.tab)),
    );
    sep(&mut s, w);

    let content_h = h.saturating_sub(6);
    let content = match state.tab {
        0 => monitoring(rows, masters, w),
        1 => {
            state.sub = state.sub.min(rows.len().saturating_sub(1));
            graphs(rows, &state.history, w, content_h, state.sub)
        }
        2 => {
            state.sub = state.sub.min(rows.len().saturating_sub(1));
            logs_tab(rows, logs, w, content_h, state.sub)
        }
        _ => {
            let subs = config_subs(cfg);
            if !subs.is_empty() {
                state.sub = state.sub.min(subs.len() - 1);
            }
            configs(cfg, &subs, state.sub)
        }
    };
    let start = if state.tab == 0 {
        state.scroll.min(content.len().saturating_sub(content_h))
    } else {
        0
    };
    for l in content.iter().skip(start).take(content_h) {
        line(&mut s, w, l);
    }

    sep(&mut s, w);
    line(&mut s, w, &help_line());
    bottom_border(&mut s, w);

    let _ = out.write_all(s.as_bytes());
}

fn title() -> String {
    format!("{BOLD}{EMBER}fpm-monitor{RESET}")
}

fn tabs_line(active: usize) -> String {
    let tabs = tabs();
    let mut s = String::new();
    for (i, name) in tabs.iter().enumerate() {
        let label = format!("[{} {}]", i + 1, name);
        if i == active {
            s.push_str(&format!("{BOLD}{EMBER}{label}{RESET}"));
        } else {
            s.push_str(&format!("{SUBTLE}{label}{RESET}"));
        }
        if i + 1 < tabs.len() {
            s.push(' ');
        }
    }
    s
}

fn help_line() -> String {
    format!("{SUBTLE}{}{RESET}", i18n::l().help)
}

fn top_border(s: &mut String, w: usize) {
    s.push_str(EMBER);
    s.push('╭');
    for _ in 0..w.saturating_sub(2) {
        s.push('─');
    }
    s.push('╮');
    s.push_str(RESET);
    s.push_str("\r\n");
}

fn bottom_border(s: &mut String, w: usize) {
    s.push_str(EMBER);
    s.push('╰');
    for _ in 0..w.saturating_sub(2) {
        s.push('─');
    }
    s.push('╯');
    s.push_str(RESET);
    s.push_str("\r\n");
}

fn sep(s: &mut String, w: usize) {
    s.push_str(SUBTLE);
    s.push('├');
    for _ in 0..w.saturating_sub(2) {
        s.push('─');
    }
    s.push('┤');
    s.push_str(RESET);
    s.push_str("\r\n");
}

fn line(s: &mut String, w: usize, content: &str) {
    let inner = w.saturating_sub(2);
    let fitted = fit(content, inner.saturating_sub(1));
    let vis = plain_len(&fitted);
    s.push_str(EMBER);
    s.push('│');
    s.push_str(RESET);
    s.push(' ');
    s.push_str(&fitted);
    for _ in 0..(inner.saturating_sub(1).saturating_sub(vis)) {
        s.push(' ');
    }
    s.push_str(EMBER);
    s.push('│');
    s.push_str(RESET);
    s.push_str("\r\n");
}

/// Longueur visible d'une chaîne (codes ANSI ignorés).
fn plain_len(s: &str) -> usize {
    let mut n = 0;
    let mut it = s.chars().peekable();
    while let Some(c) = it.next() {
        if c == '\x1b' {
            if it.peek() == Some(&'[') {
                it.next();
                while let Some(&c2) = it.peek() {
                    if c2.is_ascii_alphabetic() {
                        it.next();
                        break;
                    }
                    it.next();
                }
            }
            continue;
        }
        n += 1;
    }
    n
}

/// Tronque une chaîne colorée à `max` caractères visibles sans casser les codes ANSI.
fn fit(s: &str, max: usize) -> String {
    if plain_len(s) <= max {
        return s.to_string();
    }
    let mut out = String::with_capacity(max + 16);
    let mut vis = 0;
    let mut it = s.chars().peekable();
    while let Some(c) = it.next() {
        if c == '\x1b' {
            out.push(c);
            if it.peek() == Some(&'[') {
                out.push(it.next().unwrap());
                while let Some(&c2) = it.peek() {
                    if c2.is_ascii_alphabetic() {
                        out.push(c2);
                        it.next();
                        break;
                    }
                    out.push(c2);
                    it.next();
                }
            }
            continue;
        }
        if vis < max {
            out.push(c);
            vis += 1;
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Onglet 1 : Monitoring
// ---------------------------------------------------------------------------

fn monitoring(rows: &[Row], masters: &[i32], w: usize) -> Vec<String> {
    let l = i18n::l();
    let mut lines = Vec::new();
    let total: usize = rows.iter().map(|r| r.workers).sum();
    let running: usize = rows.iter().map(|r| r.running).sum();
    let idle: usize = rows.iter().map(|r| r.idle).sum();
    let backlog: u32 = rows.iter().filter_map(|r| r.backlog).sum();
    let max_children: Option<u32> = rows
        .iter()
        .filter_map(|r| r.cfg.as_ref())
        .filter_map(|c| c.max_children)
        .max();
    let load = load_color(running, max_children);
    let idle_c = if idle > 0 { GREEN } else { SUBTLE };
    let backlog_c = if backlog > 0 { RED } else { GREEN };

    let mut sum = format!(
        "{BOLD}{EMBER}{}{RESET} {} {BOLD}{EMBER}{}{RESET} {} {BOLD}{EMBER}{}{RESET} {} {BOLD}{EMBER}{}{RESET} {} {BOLD}{EMBER}{}{RESET} {}",
        l.pools,
        sub(&rows.len().to_string()),
        l.workers,
        sub(&total.to_string()),
        l.running,
        paint(&running.to_string(), load),
        l.idle,
        paint(&idle.to_string(), idle_c),
        l.backlog,
        paint(&backlog.to_string(), backlog_c),
    );
    let mem_total: i64 = rows.iter().filter_map(pool_mem_kb).sum();
    sum.push_str(&format!(
        "  {BOLD}{EMBER}{}{RESET} {}",
        l.mem,
        paint(&fmt_mem(mem_total), SUBTLE)
    ));
    if !masters.is_empty() {
        let m: Vec<String> = masters.iter().map(|p| p.to_string()).collect();
        sum.push_str(&format!(
            "  {BOLD}{EMBER}{}{RESET} {}",
            l.master,
            sub(&m.join(", "))
        ));
    }
    lines.push(sum);
    lines.push(String::new());

    let head = [
        l.pools.to_uppercase(),
        "TYPE".to_string(),
        l.workers.to_uppercase(),
        l.running.to_uppercase(),
        l.idle.to_uppercase(),
        l.backlog.to_uppercase(),
        l.mem.to_uppercase(),
    ];
    // Largeur des colonnes : en-tête ou contenu, selon le plus large (la
    // colonne pool reste à 16 pour l'alignement des sous-lignes worker).
    let mut widths = [16usize, 9, 7, 8, 5, 8, 9];
    for (i, h) in head.iter().enumerate() {
        widths[i] = widths[i].max(h.chars().count());
    }
    let mut cells: Vec<[String; 6]> = Vec::with_capacity(rows.len());
    for row in rows {
        let pm = row
            .cfg
            .as_ref()
            .and_then(|c| c.pm.as_deref())
            .unwrap_or("?");
        let backlog_str = match (row.backlog, row.backlog_max) {
            (Some(b), Some(m)) if m > 0 => format!("{b}/{m}"),
            (Some(b), _) => b.to_string(),
            (None, _) => "-".to_string(),
        };
        let mem_s = pool_mem_kb(row)
            .map(fmt_mem)
            .unwrap_or_else(|| "-".to_string());
        cells.push([
            pm.to_string(),
            row.workers.to_string(),
            row.running.to_string(),
            row.idle.to_string(),
            backlog_str,
            mem_s,
        ]);
    }
    for i in 1..7 {
        for c in &cells {
            let len = c[i - 1].chars().count();
            if len > widths[i] {
                widths[i] = len;
            }
        }
    }

    let (w0, w1, w2, w3, w4, w5, w6) = (
        widths[0], widths[1], widths[2], widths[3], widths[4], widths[5], widths[6],
    );
    lines.push(format!(
        "{BOLD}{SUBTLE}{:<w0$} {:<w1$} {:>w2$} {:>w3$} {:>w4$} {:>w5$} {:>w6$}{RESET}",
        head[0], head[1], head[2], head[3], head[4], head[5], head[6],
    ));

    for (row, c) in rows.iter().zip(&cells) {
        let pm = &c[0];
        let mc = row.cfg.as_ref().and_then(|c| c.max_children);
        let run_c = load_color(row.running, mc);
        let idle_c = if row.idle > 0 {
            GREEN
        } else if row.running > 0 {
            RED
        } else {
            SUBTLE
        };
        let back_c = match row.backlog {
            Some(0) => GREEN,
            Some(_) => RED,
            None => SUBTLE,
        };

        lines.push(format!(
            "{}{:<w0$}{RESET} {} {} {} {} {} {}",
            BOLD,
            row.name,
            paint(&format!("{:<w1$}", c[0]), type_color(pm)),
            paint(&format!("{:>w2$}", c[1]), SUBTLE),
            paint(&format!("{:>w3$}", c[2]), run_c),
            paint(&format!("{:>w4$}", c[3]), idle_c),
            paint(&format!("{:>w5$}", c[4]), back_c),
            paint(&format!("{:>w6$}", c[5]), SUBTLE),
        ));

        worker_lines(row).iter().for_each(|l| lines.push(l.clone()));
    }
    let _ = w;
    lines
}

fn worker_lines(row: &Row) -> Vec<String> {
    let mut lines = Vec::new();
    if let Some(scan) = &row.scan {
        for (i, w) in scan.workers.iter().enumerate() {
            if i >= 4 {
                lines.push(format!(
                    "   {SUBTLE}└─ … {}{RESET}",
                    i18n::l()
                        .more_workers
                        .replace("{}", &(scan.workers.len() - 4).to_string())
                ));
                break;
            }
            let branch = if i + 1 == scan.workers.len() || i == 3 {
                "└─"
            } else {
                "├─"
            };
            let state_c = match w.state {
                'R' => AMBER,
                'S' => GREEN,
                'D' | 'Z' | 'X' | 'T' => RED,
                _ => SUBTLE,
            };
            let rss_s = if w.rss_kb >= 0 {
                format!("  {SUBTLE}rss={}{RESET}", fmt_mem(w.rss_kb))
            } else {
                String::new()
            };
            lines.push(format!(
                "   {SUBTLE}{branch}{RESET} pid={:<7} {}{}",
                w.pid,
                paint(&w.state.to_string(), state_c),
                rss_s,
            ));
        }
    }
    lines
}

fn type_color(pm: &str) -> &'static str {
    match pm {
        "static" => EMBER,
        "dynamic" => AMBER,
        "ondemand" => SUBTLE,
        _ => SUBTLE,
    }
}

fn load_color(running: usize, max_children: Option<u32>) -> &'static str {
    if running == 0 {
        return GREEN;
    }
    match max_children {
        Some(mc) if mc > 0 => {
            let ratio = running as f32 / mc as f32;
            if ratio >= 0.8 {
                RED
            } else if ratio >= 0.5 {
                AMBER
            } else {
                GREEN
            }
        }
        _ => SUBTLE,
    }
}

fn paint(text: &str, color: &str) -> String {
    format!("{color}{text}{RESET}")
}

fn sub(text: &str) -> String {
    paint(text, SUBTLE)
}

/// RSS total du pool (somme des workers), en Ko.
fn pool_mem_kb(row: &Row) -> Option<i64> {
    let sc = row.scan.as_ref()?;
    if sc.workers.is_empty() {
        return None;
    }
    Some(sc.workers.iter().map(|w| w.rss_kb.max(0)).sum())
}

/// Formate une quantité en Ko en valeur lisible (K / M / G).
fn fmt_mem(kb: i64) -> String {
    if kb < 0 {
        return "-".to_string();
    }
    if kb >= 1_048_576 {
        format!("{:.1}G", kb as f64 / 1_048_576.0)
    } else if kb >= 1024 {
        format!("{:.1}M", kb as f64 / 1024.0)
    } else {
        format!("{kb}K")
    }
}

// ---------------------------------------------------------------------------
// Onglet 2 : Graphiques (un grand graphique par pool, style Ember)
// ---------------------------------------------------------------------------

fn graphs(rows: &[Row], history: &[Sample], w: usize, content_h: usize, sel: usize) -> Vec<String> {
    let mut lines = Vec::new();
    if rows.is_empty() {
        lines.push(format!("{BOLD}{EMBER}{}{RESET}", i18n::l().tab_graphs));
        lines.push(String::new());
        lines.push(sub(i18n::l().no_pool));
        return lines;
    }
    let idx = sel.min(rows.len() - 1);
    let row = &rows[idx];
    let (workers, running, idle, backlog) = series(history, &row.name);
    let times = times_for(history, &row.name);
    let mc = row.cfg.as_ref().and_then(|c| c.max_children);

    lines.push(format!(
        "{BOLD}{EMBER}{}{RESET} {SUBTLE}· {}{RESET} {SUBTLE}{}{RESET}",
        i18n::l().tab_graphs,
        row.name,
        i18n::l()
            .last_measurements
            .replace("{}", &history.len().max(1).to_string())
    ));
    lines.push(pool_subtabs(rows, idx));

    let inner = w.saturating_sub(3).max(20);
    let pm = row
        .cfg
        .as_ref()
        .and_then(|c| c.pm.as_deref())
        .unwrap_or("?");
    let left = format!("{BOLD}{}{RESET} {SUBTLE}· {}{RESET}", row.name, pm);
    lines.push(join_left_right(&left, &right_info(row), inner));
    lines.push(String::new());

    let bl = row.backlog.unwrap_or(0);
    // backlog toujours affiché (même vide) : 3 panneaux empilés, plus
    // l'en-tête (titre + sous-onglets + infos + ligne vide) et une ligne vide.
    let panel_h = (content_h.saturating_sub(11) / 3).max(3);

    let run_title = format!(
        "{} · {}{}  {}",
        i18n::l().g_running,
        row.running,
        mc.map_or(String::new(), |v| format!("/{v}")),
        load_bar(row.running, mc)
    );
    let mut run_refs: Vec<(u32, &'static str, bool)> = Vec::new();
    if let Some(mc) = mc {
        run_refs.push((mc, SUBTLE, true));
    }
    if let Some(mr) = row
        .cfg
        .as_ref()
        .and_then(|c| c.max_requests)
        .filter(|&m| m > 0)
    {
        run_refs.push((mr, RED, false));
    }
    lines.extend(ascii_panel(
        &run_title,
        &[(running.as_slice(), AMBER)],
        &times,
        &run_refs,
        inner,
        panel_h,
    ));
    lines.push(String::new());

    let mut idle_refs: Vec<(u32, &'static str, bool)> = Vec::new();
    if let Some(v) = row.cfg.as_ref().and_then(|c| c.min_spare_servers) {
        idle_refs.push((v, ROSE, true));
    }
    if let Some(v) = row.cfg.as_ref().and_then(|c| c.max_spare_servers) {
        idle_refs.push((v, VIOLET, true));
    }
    let half = inner / 2;
    let a = ascii_panel(
        &format!("{} · {}", i18n::l().g_workers, row.workers),
        &[(workers.as_slice(), SUBTLE)],
        &times,
        &[],
        half,
        panel_h,
    );
    let b = ascii_panel(
        &format!("{} · {}", i18n::l().g_idle, row.idle),
        &[(idle.as_slice(), GREEN)],
        &times,
        &idle_refs,
        inner - half,
        panel_h,
    );
    lines.extend(join_panels(a, b));

    let bl_c = if bl > 0 { RED } else { GREEN };
    lines.extend(ascii_panel(
        &format!("{} · {bl}", i18n::l().g_backlog),
        &[(backlog.as_slice(), bl_c)],
        &times,
        &[],
        inner,
        panel_h,
    ));
    lines
}

/// Bande de sous-onglets (un par pool), l'actif étant surligné.
fn pool_subtabs(rows: &[Row], active: usize) -> String {
    let mut s = format!("{SUBTLE}← →{RESET}");
    for (i, row) in rows.iter().enumerate() {
        let label = format!("[{}]", row.name);
        if i == active {
            s.push_str(&format!("  {BOLD}{EMBER}{label}{RESET}"));
        } else {
            s.push_str(&format!("  {SUBTLE}{label}{RESET}"));
        }
    }
    s
}

/// Panneau type Ember : grand graphique encadré avec axes Y/X (asciigraph-like).
/// `series` est une liste de (valeurs, couleur) superposées dans le même panneau.
/// `refs` est une liste de (seuil, couleur, étend_l_échelle) dessinés en pointillés
/// (`┄`) ; si `étend_l_échelle` est vrai, l'axe Y est élargi pour que la ligne soit
/// toujours visible (comportement « seuil de référence »).
fn ascii_panel(
    title: &str,
    series: &[(&[u32], &'static str)],
    times: &[u64],
    refs: &[(u32, &'static str, bool)],
    w: usize,
    h: usize,
) -> Vec<String> {
    let mut lines = Vec::with_capacity(h + 2);
    lines.push(panel_top(title, w));

    let plot_w = w.saturating_sub(6).max(1);
    let (mut lo, mut hi) = series_range(series, plot_w);
    for (m, _, extend) in refs {
        if *extend {
            lo = lo.min(*m);
            hi = hi.max(*m);
        }
    }
    if hi <= lo {
        hi = lo + 1;
    }
    let (grid, colors) = fill_grid_multi(series, plot_w, h, lo, hi);

    // Lignes de référence (seuils de config) en pointillés, une couleur par seuil.
    let mut ref_lines: Vec<Vec<Option<&'static str>>> = vec![vec![None; plot_w]; h];
    if lo < hi {
        let span = (hi - lo) as f64;
        let sub_total = (h * 2) as f64;
        for (m, color, _) in refs {
            if *m >= lo && *m <= hi {
                let s = (*m as f64 - lo as f64) / span * (sub_total - 1.0);
                // Inversion : la sub-ligne 0 est en haut (label hi).
                let sr = ((sub_total - 1.0) - s).round() as i32;
                let sr = (sr).clamp(0, (h * 2) as i32 - 1) as usize / 2;
                for c in (0..plot_w).step_by(2) {
                    ref_lines[sr][c] = Some(*color);
                }
            }
        }
    }

    let maxlab = format!("{hi}");
    let minlab = format!("{lo}");
    for r in 0..h {
        let label = if r == 0 {
            maxlab.as_str()
        } else if r == h - 1 {
            minlab.as_str()
        } else {
            ""
        };
        let mut row_s = String::from("│");
        row_s.push_str(&format!("{label:>3}"));
        row_s.push('┤');
        for c in 0..plot_w {
            if let Some(col) = ref_lines[r][c] {
                if grid[r][c] == 0 {
                    row_s.push_str(&format!("{col}┄{RESET}"));
                    continue;
                }
            }
            let b = grid[r][c];
            let col = colors[r][c];
            match b {
                0 => row_s.push(' '),
                1 => row_s.push_str(&format!("{col}▀{RESET}")),
                2 => row_s.push_str(&format!("{col}▄{RESET}")),
                _ => row_s.push_str(&format!("{col}█{RESET}")),
            }
        }
        row_s.push('│');
        lines.push(row_s);
    }
    lines.push(panel_bottom(times, w));
    lines
}

/// Bornes (min, max) de la fenêtre de données des séries.
fn series_range(series: &[(&[u32], &'static str)], plot_w: usize) -> (u32, u32) {
    let take = plot_w.min(series.iter().map(|(v, _)| v.len()).max().unwrap_or(0));
    let mut lo = u32::MAX;
    let mut hi = 0u32;
    for (v, _) in series {
        if v.is_empty() {
            continue;
        }
        let start = v.len().saturating_sub(take);
        for &x in &v[start..] {
            lo = lo.min(x);
            hi = hi.max(x);
        }
    }
    if lo == u32::MAX {
        lo = 0;
    }
    if hi == lo {
        hi += 1;
        lo = lo.saturating_sub(1);
    }
    (lo, hi)
}

/// Grille de pixels (2 sous-lignes par rangée) pour les séries données.
/// La série est alignée à droite (valeurs les plus récentes à droite).
/// L'échelle verticale (`lo`..`hi`) est imposée par l'appelant.
fn fill_grid_multi(
    series: &[(&[u32], &'static str)],
    plot_w: usize,
    plot_h: usize,
    lo: u32,
    hi: u32,
) -> (Vec<Vec<u8>>, Vec<Vec<&'static str>>) {
    let take = plot_w.min(series.iter().map(|(v, _)| v.len()).max().unwrap_or(0));
    let start_col = plot_w - take;
    let sub_total = plot_h * 2;
    let span = (hi - lo) as f64;
    // Sub-ligne 0 = haut du panneau, sub_total-1 = bas. Les valeurs hautes
    // doivent donc être en haut (labels hi en ligne 0).
    let to_sub = |x: u32| -> i32 {
        let s = (x as f64 - lo as f64) / span * (sub_total as f64 - 1.0);
        ((sub_total as f64 - 1.0) - s).round() as i32
    };
    let mut grid: Vec<Vec<u8>> = vec![vec![0u8; plot_w]; plot_h];
    let mut colors: Vec<Vec<&'static str>> = vec![vec![""; plot_w]; plot_h];
    for (v, color) in series {
        if v.is_empty() {
            continue;
        }
        let start = v.len() - take;
        let slice = &v[start..];
        for i in 0..take {
            let s0 = to_sub(slice[i]).clamp(0, sub_total as i32 - 1);
            let s1 = if i + 1 < take {
                to_sub(slice[i + 1]).clamp(0, sub_total as i32 - 1)
            } else {
                s0
            };
            let (a, b) = (s0.min(s1), s0.max(s1));
            for sub in a..=b {
                let r = (sub / 2) as usize;
                let c = start_col + i;
                if grid[r][c] == 0 {
                    colors[r][c] = color;
                }
                grid[r][c] |= if sub % 2 == 0 { 1 } else { 2 };
            }
        }
    }
    (grid, colors)
}

fn panel_top(title: &str, w: usize) -> String {
    let t = format!(" {title} ");
    let tlen = plain_len(&t);
    let fill = w.saturating_sub(2).saturating_sub(tlen);
    format!("╭─{t}{}{RESET}╮", "─".repeat(fill))
}

fn panel_bottom(times: &[u64], w: usize) -> String {
    let (t0, t1) = if times.is_empty() {
        ("--:--:--".to_string(), "--:--:--".to_string())
    } else {
        (fmt_time(times[0]), fmt_time(*times.last().unwrap()))
    };
    let pre = format!("╰─ {t0} ");
    let suf = format!(" {t1} ─╯");
    let inner = w.saturating_sub(plain_len(&pre) + plain_len(&suf));
    format!("{pre}{}{suf}", "─".repeat(inner))
}

/// Assemble deux panneaux côte à côte (style Ember : 2 par ligne).
fn join_panels(left: Vec<String>, right: Vec<String>) -> Vec<String> {
    left.into_iter()
        .zip(right)
        .map(|(l, r)| format!("{l}  {r}"))
        .collect()
}

fn series(history: &[Sample], name: &str) -> (Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>) {
    let mut workers = Vec::new();
    let mut running = Vec::new();
    let mut idle = Vec::new();
    let mut backlog = Vec::new();
    for s in history {
        if let Some((_, wk, rn, id, bl)) = s.points.iter().find(|(n, _, _, _, _)| n == name) {
            workers.push(*wk);
            running.push(*rn);
            idle.push(*id);
            backlog.push(*bl);
        }
    }
    (workers, running, idle, backlog)
}

fn load_bar(used: usize, max: Option<u32>) -> String {
    let pct = match max {
        Some(m) if m > 0 => used.saturating_mul(100) / m as usize,
        _ => 0,
    };
    let color = if pct >= 80 {
        RED
    } else if pct >= 50 {
        AMBER
    } else {
        GREEN
    };
    let filled = (pct / 10).min(10);
    let mut s = String::from(color);
    for i in 0..10 {
        s.push(if i < filled { '█' } else { '░' });
    }
    s.push_str(RESET);
    s.push_str(&sub(&format!(" {pct}%")));
    s
}

/// Infos affichées à droite de l'en-tête : max_requests (rouge), min/max spare.
/// Les couleurs correspondent aux lignes en pointillés des panneaux.
fn right_info(row: &Row) -> String {
    let mut parts = Vec::new();
    if let Some(mr) = row
        .cfg
        .as_ref()
        .and_then(|c| c.max_requests)
        .filter(|&m| m > 0)
    {
        parts.push(paint(&format!("max_requests={mr}"), RED));
    }
    if let Some(v) = row.cfg.as_ref().and_then(|c| c.min_spare_servers) {
        parts.push(paint(&format!("min_spare={v}"), ROSE));
    }
    if let Some(v) = row.cfg.as_ref().and_then(|c| c.max_spare_servers) {
        parts.push(paint(&format!("max_spare={v}"), VIOLET));
    }
    parts.join("  ")
}

/// Assemble une ligne avec une partie gauche et une partie droite alignée à droite.
/// Si ça ne tient pas, c'est la partie gauche qui est tronquée (la droite est prioritaire).
fn join_left_right(left: &str, right: &str, width: usize) -> String {
    let l = plain_len(left);
    let r = plain_len(right);
    if l + r <= width {
        let mut s = String::from(left);
        for _ in 0..(width - l - r) {
            s.push(' ');
        }
        s.push_str(right);
        return s;
    }
    let left_max = width.saturating_sub(r).max(1);
    let mut s = fit(left, left_max);
    s.push_str(right);
    s
}

/// Timestamps (epoch s) des mesures du pool, alignés avec `series`.
fn times_for(history: &[Sample], name: &str) -> Vec<u64> {
    history
        .iter()
        .filter(|s| s.points.iter().any(|(n, _, _, _, _)| n == name))
        .map(|s| s.t)
        .collect()
}

fn fmt_time(secs: u64) -> String {
    let s = secs % 60;
    let m = (secs / 60) % 60;
    let h = (secs / 3600) % 24;
    format!("{h:02}:{m:02}:{s:02}")
}

// ---------------------------------------------------------------------------
// Onglet 3 : Logs (logs PHP + requêtes lentes, par pool)
// ---------------------------------------------------------------------------

fn logs_tab(
    rows: &[Row],
    logs: &[PoolLogs],
    w: usize,
    content_h: usize,
    sel: usize,
) -> Vec<String> {
    let mut lines = Vec::new();
    if rows.is_empty() {
        lines.push(format!("{BOLD}{EMBER}{}{RESET}", i18n::l().tab_logs));
        lines.push(String::new());
        lines.push(sub(i18n::l().no_pool));
        return lines;
    }
    let idx = sel.min(rows.len() - 1);
    let pool = &rows[idx].name;
    let lg = logs.iter().find(|l| l.name == *pool);

    lines.push(format!(
        "{BOLD}{EMBER}{}{RESET} {SUBTLE}· {}{RESET}",
        i18n::l().tab_logs,
        pool
    ));
    lines.push(pool_subtabs(rows, idx));
    lines.push(String::new());

    let inner = w.saturating_sub(3).max(20);
    let empty: Vec<String> = Vec::new();
    let php: &Vec<String> = match lg {
        Some(l) => &l.php,
        None => &empty,
    };
    let slow: &Vec<String> = match lg {
        Some(l) => &l.slow,
        None => &empty,
    };
    let access: &Vec<String> = match lg {
        Some(l) => &l.access,
        None => &empty,
    };

    // Trois sections verticales : logs PHP, requêtes lentes, accès.
    let remaining = content_h.saturating_sub(3);
    let mut per = (remaining.saturating_sub(9) / 3).max(1);

    let php_title = match lg.and_then(|l| l.php_log.as_deref()) {
        Some(p) => format!("{} {SUBTLE}· {}{RESET}", i18n::l().log_php, p),
        None => i18n::l().log_php.to_string(),
    };
    push_section(&mut lines, &php_title, php, inner, &mut per, log_line);

    let slow_title = match lg.and_then(|l| l.slow_log.as_deref()) {
        Some(p) => format!("{} {SUBTLE}· {}{RESET}", i18n::l().slow_queries, p),
        None => i18n::l().slow_queries.to_string(),
    };
    push_section(&mut lines, &slow_title, slow, inner, &mut per, log_line);

    let access_title = match lg.and_then(|l| l.access_log.as_deref()) {
        Some(p) => format!("{} {SUBTLE}· {}{RESET}", i18n::l().access, p),
        None => i18n::l().access.to_string(),
    };
    push_section(
        &mut lines,
        &access_title,
        access,
        inner,
        &mut per,
        access_line,
    );
    lines
}

/// Titre de section style config (`── titre ──`) + les N dernières lignes.
fn push_section(
    out: &mut Vec<String>,
    title: &str,
    lines: &[String],
    width: usize,
    budget: &mut usize,
    colorize: fn(&str) -> String,
) {
    let t = format!("── {title} ──");
    out.push(format!("{BOLD}{SUBTLE}{}{RESET}", fit(&t, width)));
    if lines.is_empty() {
        out.push(format!("   {SUBTLE}{}{RESET}", i18n::l().no_entries));
        return;
    }
    let per = (*budget).max(1);
    if lines.len() > per {
        out.push(sub(&format!(
            "   … {}",
            i18n::l()
                .older_lines
                .replace("{}", &(lines.len() - per).to_string())
        )));
    }
    for l in lines.iter().rev().take(per).rev() {
        out.push(format!("  {}", fit(&colorize(l), width)));
    }
    *budget = per;
    out.push(String::new());
}

/// Colore le niveau de sévérité d'une ligne de log PHP.
fn log_line(line: &str) -> String {
    const SEV: [(&str, &str); 5] = [
        ("EMERGENCY", RED),
        ("ALERT", RED),
        ("ERROR", RED),
        ("WARNING", AMBER),
        ("NOTICE", SUBTLE),
    ];
    for (sev, color) in SEV {
        if let Some(i) = line.find(sev) {
            let (a, _) = line.split_at(i);
            let b = &line[i + sev.len()..];
            return format!("{a}{}{b}", paint(sev, color));
        }
    }
    line.to_string()
}

/// Colore le code de statut HTTP d'une ligne d'access log FPM.
fn access_line(line: &str) -> String {
    let code = line
        .split_whitespace()
        .rev()
        .find(|t| t.len() == 3 && t.chars().all(|c| c.is_ascii_digit()))
        .map(|t| t.to_string());
    match code {
        Some(c) => {
            let color = match c.as_str() {
                c if c.starts_with('5') => RED,
                c if c.starts_with('4') => AMBER,
                c if c.starts_with('3') => SUBTLE,
                _ => GREEN,
            };
            let idx = line.rfind(&c).unwrap_or(0);
            let (a, _) = line.split_at(idx);
            let b = &line[idx + c.len()..];
            format!("{a}{}{b}", paint(&c, color))
        }
        None => line.to_string(),
    }
}

// ---------------------------------------------------------------------------
// Onglet 4 : Configurations
// ---------------------------------------------------------------------------

fn config_subs(cfg: &Config) -> Vec<String> {
    let mut v = Vec::new();
    if !cfg.globals.is_empty() {
        v.push("global".to_string());
    }
    let mut names: Vec<&String> = cfg.pools.keys().collect();
    names.sort();
    for n in names {
        v.push(n.clone());
    }
    v
}

fn configs(cfg: &Config, subs: &[String], idx: usize) -> Vec<String> {
    let l = i18n::l();
    let mut lines = Vec::new();
    lines.push(format!("{BOLD}{EMBER}{}{RESET}", l.pool_configs));

    if subs.is_empty() {
        lines.push(sub(l.no_config));
        return lines;
    }

    lines.push(String::new());
    let mut strip = format!("{SUBTLE}← →{RESET}");
    for (i, s) in subs.iter().enumerate() {
        let label = format!("[{}]", s);
        if i == idx {
            strip.push_str(&format!("  {BOLD}{EMBER}{label}{RESET}"));
        } else {
            strip.push_str(&format!("  {SUBTLE}{label}{RESET}"));
        }
    }
    lines.push(strip);
    lines.push(String::new());

    let sel = &subs[idx];
    if sel == "global" {
        lines.push(format!("{BOLD}{SUBTLE}── global ──{RESET}"));
        for (k, v) in &cfg.globals {
            lines.push(format!("  {} = {}", sub(k), v));
        }
        return lines;
    }

    let Some(p) = cfg.pools.get(sel) else {
        lines.push(sub(i18n::l().unknown_pool));
        return lines;
    };
    let pm = p.pm.as_deref().unwrap_or("?");
    let listen = p.listen.as_deref().unwrap_or("-");
    lines.push(format!(
        "{BOLD}{EMBER}{}{RESET} {SUBTLE}· type {}{RESET}  {SUBTLE}· listen {}{RESET}",
        sel,
        paint(pm, type_color(pm)),
        listen
    ));
    for (k, v) in &p.raw {
        lines.push(format!("  {} = {}", sub(k), v));
    }
    if p.raw.is_empty() {
        lines.push(format!("  {SUBTLE}{}{RESET}", i18n::l().no_directives));
    }
    lines
}

fn push_history(state: &mut TuiState, rows: &[Row]) {
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let points: Vec<(String, u32, u32, u32, u32)> = rows
        .iter()
        .map(|r| {
            (
                r.name.clone(),
                r.workers as u32,
                r.running as u32,
                r.idle as u32,
                r.backlog.unwrap_or(0),
            )
        })
        .collect();
    state.history.push(Sample { t, points });
    if state.history.len() > 120 {
        state.history.remove(0);
    }
}
