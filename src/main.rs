mod access;
mod config;
mod data;
mod i18n;
mod logs;
mod phpinfo;
mod proc;
mod render;
mod term;
mod tui;
mod web;

use std::path::PathBuf;
use std::process::exit;
use std::time::Duration;

use term::Style;

fn main() {
    let mut cli_config: Option<PathBuf> = None;
    let mut verbose = false;
    let mut force_color = false;
    let mut no_color = false;
    let mut mock = false;
    let mut tui_mode = false;
    let mut watch = false;
    let mut port = web::DEFAULT_PORT;
    let mut interval = Duration::from_secs(1);

    let args: Vec<String> = std::env::args().collect();

    // Pre-scan of `--lang`: the language must be known before `--help` and before
    // option errors are displayed.
    let mut lang: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--lang" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    lang = Some(v.clone());
                }
            }
            s if s.starts_with("--lang=") => lang = Some(s[7..].to_string()),
            _ => {}
        }
        i += 1;
    }
    if let Some(v) = lang {
        if let Err(e) = i18n::set(&v) {
            eprintln!("{e}");
            exit(2);
        }
    }

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-c" | "--config" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    cli_config = Some(PathBuf::from(v));
                }
            }
            "-v" | "--verbose" => verbose = true,
            "--color" => force_color = true,
            "--no-color" => no_color = true,
            "-t" | "--tui" => tui_mode = true,
            "--watch" => watch = true,
            "--mock" => mock = true,
            "--port" => {
                i += 1;
                let value = args.get(i).map(String::as_str).unwrap_or("");
                port = port_arg(value);
            }
            "--interval" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    if let Ok(secs) = v.parse::<u64>() {
                        interval = Duration::from_secs(secs.max(1));
                    }
                }
            }
            "--lang" => {
                i += 1;
            }
            s if s.starts_with("--lang=") => {}
            "-V" | "--version" => {
                println!("fpm-monitor {}", env!("CARGO_PKG_VERSION"));
                exit(0);
            }
            "-h" | "--help" => {
                print!("{}", i18n::usage());
                exit(0);
            }
            s if s.starts_with("--config=") => {
                cli_config = Some(PathBuf::from(&s[9..]));
            }
            s if s.starts_with("--port=") => port = port_arg(&s[7..]),
            s => {
                eprintln!("{}", i18n::l().unknown_option.replace("{}", s));
                eprintln!("{}", i18n::l().run_help);
                exit(2);
            }
        }
        i += 1;
    }

    let style = Style::detect(force_color, no_color);

    if mock {
        let cfg = data::mock_config();
        if watch {
            if let Err(e) = web::run(cfg, None, interval, port, true) {
                eprintln!("{}", i18n::l().web_error.replace("{}", &e.to_string()));
                exit(1);
            }
            return;
        }
        if tui_mode {
            let cfg_thread = cfg.clone();
            let masters = data::mock_scan().masters;
            tui::run(
                move || {
                    let rows = data::mock_rows(&cfg_thread);
                    let names: Vec<String> = rows.iter().map(|r| r.name.clone()).collect();
                    let lg = logs::mock(&names);
                    (rows, masters.clone(), lg)
                },
                &cfg,
                interval,
            );
            return;
        }
        let rows = data::mock_rows(&cfg);
        render::render(&rows, &[4242], verbose, &style);
        return;
    }

    let files = config::discover(cli_config.as_deref());
    let full_cfg = config::load(&files);
    let identity = access::Identity::current();
    let allowed = access::allowed_names(&identity, &full_cfg);
    let cfg = access::filter_config(full_cfg, &allowed);

    if watch {
        if let Err(e) = web::run(cfg, allowed, interval, port, false) {
            eprintln!("{}", i18n::l().web_error.replace("{}", &e.to_string()));
            exit(1);
        }
        return;
    }

    if tui_mode {
        let cfg2 = cfg.clone();
        tui::run(
            move || {
                let scan = match proc::scan() {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("{}", i18n::l().read_proc.replace("{}", &e.to_string()));
                        exit(1);
                    }
                };
                let scan = access::filter_scan(scan, &allowed);
                let masters = scan.masters.clone();
                let rows = data::build_rows(scan, &cfg2);
                let names: Vec<String> = rows.iter().map(|r| r.name.clone()).collect();
                let lg = logs::collect(&cfg2, &names, 100);
                (rows, masters, lg)
            },
            &cfg,
            interval,
        );
        return;
    }

    let scan = match proc::scan() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", i18n::l().read_proc.replace("{}", &e.to_string()));
            exit(1);
        }
    };
    let scan = access::filter_scan(scan, &allowed);
    let masters = scan.masters.clone();
    let rows = data::build_rows(scan, &cfg);
    render::render(&rows, &masters, verbose, &style);
}

/// Parses `--port`, exiting with code 2 and a localized message on error.
fn port_arg(value: &str) -> u16 {
    match web::parse_port(value) {
        Some(port) => port,
        None => {
            eprintln!("{}", i18n::l().invalid_port.replace("{}", value));
            exit(2);
        }
    }
}
