use super::Labels;

pub(super) const LABELS: &Labels = &Labels {
    tab_monitoring: "Overvåking",
    tab_graphs: "Grafer",
    tab_logs: "Logger",
    tab_configs: "Konfigurasjoner",
    pools: "Pooler",
    workers: "Arbeidere",
    running: "Aktive",
    idle: "Ledige",
    backlog: "Kø",
    mem: "Minne",
    master: "Master",
    help: "q avslutt · Tab neste fane · 1-4 fane · ← → underfane · ↑ ↓ overvåking",
    no_pool: "Ingen pool oppdaget.",
    last_measurements: "siste {} målinger",
    g_running: "aktive",
    g_workers: "arbeidere",
    g_idle: "ledige",
    g_backlog: "kø",
    log_php: "PHP-logg",
    slow_queries: "Trege spørringer",
    access: "Tilgang",
    no_entries: "(ingen oppføringer)",
    older_lines: "+{} eldre linjer",
    more_workers: "+{} flere",
    pool_configs: "Poolkonfigurasjoner",
    no_config: "Ingen konfigurasjon funnet.",
    unknown_pool: "Ukjent pool.",
    no_directives: "(ingen direktiver lest)",
    master_pid: "Master-PID",
    legend: "Forklaring:",
    leg_red: "rød = metning/kø",
    leg_green: "grønn = sunn/ledig",
    leg_white: "hvit = nøytral/ondemand",
    leg_blue: "blå = aktiv/static",
    read_proc: "kan ikke lese /proc (krever Linux): {}",
    unknown_option: "ukjent alternativ: {}",
    run_help: "kjør med --help",
};

pub(super) const USAGE: &str = "\
fpm-monitor — overvåking av PHP-FPM-pooler (Rust-port av fpm-monitor.c)

Bruk: fpm-monitor [ALTERNATIVER]

Alternativer:
  -c, --config <STI>     php-fpm.conf-fil eller poolmappe som skal analyseres
  -v, --verbose          Vis detaljer for hver arbeider (pid, tilstand, RSS)
      --color            Tving farger
      --no-color         Slå av farger
  -t, --tui              Interaktivt dashbord (som Ember) — faner
                         overvåking, grafer, logger og konfigurasjoner
                         (underfaner med ← →). Avslutt: q / Ctrl-C
      --interval <SEK>   Oppdateringsintervall for TUI (standard 1)
      --lang <SPRÅK>     UI-språk: en (US), fr, zh, ar, es, it, ja, de,
                         ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
      --mock             Vis demodata (lokal test)
  -h, --help             Vis denne hjelpen

Gjenkjenning skjer automatisk (vanlige plasseringer + include-direktiv).
";
