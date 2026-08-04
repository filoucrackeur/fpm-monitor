mod config;
mod data;
mod i18n;
mod logs;
mod proc;
mod render;
mod term;
mod tui;

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
    let mut interval = Duration::from_secs(1);

    let args: Vec<String> = std::env::args().collect();

    // Pré-scan de `--lang` : la langue doit être connue avant `--help` et avant
    // l'affichage des erreurs d'option.
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
            "--mock" => mock = true,
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
            "-h" | "--help" => {
                print!("{}", i18n::usage());
                exit(0);
            }
            s if s.starts_with("--config=") => {
                cli_config = Some(PathBuf::from(&s[9..]));
            }
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
    let cfg = config::load(&files);

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
    let masters = scan.masters.clone();
    let rows = data::build_rows(scan, &cfg);
    render::render(&rows, &masters, verbose, &style);
}
