use crate::data::Row;
use crate::i18n;
use crate::term::{pad, Color, Style};

fn headers() -> Vec<String> {
    let l = i18n::l();
    vec![
        l.pools.to_uppercase(),
        "TYPE".to_string(),
        l.workers.to_uppercase(),
        l.running.to_uppercase(),
        l.idle.to_uppercase(),
        l.backlog.to_uppercase(),
        "MAX_CHILD".to_string(),
        "MAX_REQ".to_string(),
        l.mem.to_uppercase(),
    ]
}

struct Cell {
    plain: String,
    colored: String,
}

impl Cell {
    fn new(style: &Style, plain: &str, color: Option<Color>, bold: bool) -> Cell {
        let colored = match color {
            Some(c) if bold => style.paint_bold(plain, c),
            Some(c) => style.paint(plain, c),
            None => plain.to_string(),
        };
        Cell {
            plain: plain.to_string(),
            colored,
        }
    }
}

pub fn render(rows: &[Row], masters: &[i32], verbose: bool, style: &Style) {
    let table: Vec<Vec<Cell>> = rows.iter().map(|r| row_cells(r, style)).collect();
    let widths = column_widths(&table);

    let header: Vec<String> = headers()
        .iter()
        .enumerate()
        .map(|(i, h)| pad(&style.paint_bold(h, Color::Blue), h, widths[i]))
        .collect();
    println!("{}", header.join("  "));

    let total: usize = widths.iter().sum::<usize>() + (widths.len() - 1) * 2;
    println!("{}", style.paint(&"─".repeat(total), Color::Blue));

    for (i, row) in rows.iter().enumerate() {
        let line: Vec<String> = table[i]
            .iter()
            .enumerate()
            .map(|(j, c)| pad(&c.colored, &c.plain, widths[j]))
            .collect();
        println!("{}", line.join("  "));
        print_workers(row, verbose, style);
    }

    println!();
    print_summary(rows, masters, style);
    print_legend(style);
}

fn row_cells(row: &Row, style: &Style) -> Vec<Cell> {
    let cfg = row.cfg.as_ref();
    let max_children = cfg.and_then(|c| c.max_children);
    let max_requests = cfg.and_then(|c| c.max_requests);
    let pm = cfg.and_then(|c| c.pm.as_deref());

    let missing_static = row.workers == 0 && matches!(pm, Some("static") | Some("dynamic"));
    let workers_color = if missing_static {
        Color::Red
    } else {
        Color::White
    };

    let backlog_str = match (row.backlog, row.backlog_max) {
        (Some(b), Some(m)) if m > 0 => format!("{b}/{m}"),
        (Some(b), _) => b.to_string(),
        (None, _) => "-".to_string(),
    };
    let backlog_color = match row.backlog {
        Some(0) => Color::Green,
        Some(_) => Color::Red,
        None => Color::White,
    };
    let mem_str = match &row.scan {
        Some(sc) if !sc.workers.is_empty() => {
            let sum: i64 = sc.workers.iter().map(|w| w.rss_kb.max(0)).sum();
            fmt_mem(sum)
        }
        _ => "-".to_string(),
    };

    vec![
        Cell::new(style, &row.name, Some(Color::White), true),
        Cell::new(style, pm.unwrap_or("?"), Some(type_color(pm)), false),
        Cell::new(style, &row.workers.to_string(), Some(workers_color), false),
        Cell::new(
            style,
            &row.running.to_string(),
            Some(load_color(row.running, max_children)),
            false,
        ),
        Cell::new(style, &row.idle.to_string(), Some(idle_color(row)), false),
        Cell::new(style, &backlog_str, Some(backlog_color), false),
        Cell::new(
            style,
            &max_children.map_or("-".to_string(), |v| v.to_string()),
            Some(Color::White),
            false,
        ),
        Cell::new(
            style,
            &max_requests.map_or("-".to_string(), |v| v.to_string()),
            Some(max_requests_color(max_requests)),
            false,
        ),
        Cell::new(style, &mem_str, Some(Color::White), false),
    ]
}

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

fn pool_mem_kb(row: &Row) -> i64 {
    match &row.scan {
        Some(sc) => sc.workers.iter().map(|w| w.rss_kb.max(0)).sum(),
        None => 0,
    }
}

fn column_widths(table: &[Vec<Cell>]) -> Vec<usize> {
    let mut widths: Vec<usize> = headers().iter().map(|h| h.chars().count()).collect();
    for row in table {
        for (i, c) in row.iter().enumerate() {
            let len = c.plain.chars().count();
            if len > widths[i] {
                widths[i] = len;
            }
        }
    }
    widths
}

fn type_color(pm: Option<&str>) -> Color {
    match pm {
        Some("static") => Color::Blue,
        Some("dynamic") => Color::Green,
        Some("ondemand") => Color::White,
        _ => Color::White,
    }
}

fn load_color(running: usize, max_children: Option<u32>) -> Color {
    if running == 0 {
        return Color::Green;
    }
    match max_children {
        Some(mc) if mc > 0 => {
            let ratio = running as f32 / mc as f32;
            if ratio >= 0.8 {
                Color::Red
            } else if ratio >= 0.5 {
                Color::White
            } else {
                Color::Green
            }
        }
        _ => Color::White,
    }
}

fn idle_color(row: &Row) -> Color {
    if row.idle > 0 {
        Color::Green
    } else if row.running > 0 {
        Color::Red
    } else {
        Color::White
    }
}

fn max_requests_color(mr: Option<u32>) -> Color {
    match mr {
        Some(0) | None => Color::White,
        Some(n) if n < 100 => Color::Red,
        Some(_) => Color::Green,
    }
}

fn print_workers(row: &Row, _verbose: bool, style: &Style) {
    if let Some(scan) = &row.scan {
        for (i, w) in scan.workers.iter().enumerate() {
            let state_color = match w.state {
                'R' => Color::Blue,
                'S' => Color::Green,
                'D' | 'Z' | 'X' | 'T' => Color::Red,
                _ => Color::White,
            };
            let branch = if i + 1 == scan.workers.len() {
                "└─"
            } else {
                "├─"
            };
            let rss = if w.rss_kb >= 0 {
                format!("{} Ko", w.rss_kb)
            } else {
                "-".to_string()
            };
            println!(
                "   {branch} pid={:<7} state={} rss={}",
                w.pid,
                style.paint(&w.state.to_string(), state_color),
                style.paint(&rss, Color::White),
            );
        }
    }
}

fn print_summary(rows: &[Row], masters: &[i32], style: &Style) {
    let l = i18n::l();
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
    let idle_c = if idle > 0 { Color::Green } else { Color::White };
    let backlog_c = if backlog > 0 {
        Color::Red
    } else {
        Color::Green
    };

    println!(
        "{}: {} | {}: {} | {}: {} | {}: {} | {}: {} | {}: {}",
        style.paint_bold(l.pools, Color::Blue),
        style.paint(&rows.len().to_string(), Color::Blue),
        style.paint_bold(l.workers, Color::Blue),
        style.paint(&total.to_string(), Color::White),
        style.paint_bold(l.running, Color::Blue),
        style.paint(&running.to_string(), load),
        style.paint_bold(l.idle, Color::Blue),
        style.paint(&idle.to_string(), idle_c),
        style.paint_bold(l.backlog, Color::Blue),
        style.paint(&backlog.to_string(), backlog_c),
        style.paint_bold(l.mem, Color::Blue),
        style.paint(&fmt_mem(rows.iter().map(pool_mem_kb).sum()), Color::White,),
    );

    if !masters.is_empty() {
        let m: Vec<String> = masters.iter().map(|p| p.to_string()).collect();
        println!(
            "{} {}",
            style.paint_bold(l.master_pid, Color::Blue),
            style.paint(&m.join(", "), Color::White)
        );
    }
}

fn print_legend(style: &Style) {
    let l = i18n::l();
    println!(
        "{} {}",
        style.paint_bold(l.legend, Color::Blue),
        [
            style.paint(l.leg_red, Color::Red),
            style.paint(l.leg_green, Color::Green),
            style.paint(l.leg_white, Color::White),
            style.paint(l.leg_blue, Color::Blue),
        ]
        .join(" · ")
    );
}
