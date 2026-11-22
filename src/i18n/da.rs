use super::Labels;

pub(super) const LABELS: &Labels = &Labels {
    tab_monitoring: "Overvågning",
    tab_graphs: "Grafer",
    tab_logs: "Logfiler",
    tab_configs: "Konfigurationer",
    pools: "Pools",
    workers: "Workers",
    running: "Aktive",
    idle: "Inaktive",
    backlog: "Kø",
    mem: "Hukommelse",
    master: "Master",
    help: "q afslut · Tab næste fane · 1-4 fane · ← → underfane · ↑ ↓ overvågning",
    no_pool: "Ingen pool fundet.",
    last_measurements: "seneste {} målinger",
    g_running: "aktive",
    g_workers: "workers",
    g_idle: "inaktive",
    g_backlog: "kø",
    log_php: "PHP-log",
    slow_queries: "Langsomme forespørgsler",
    access: "Adgang",
    no_entries: "(ingen poster)",
    older_lines: "+{} ældre linjer",
    more_workers: "+{} flere",
    pool_configs: "Pool-konfigurationer",
    no_config: "Ingen konfiguration fundet.",
    unknown_pool: "Ukendt pool.",
    no_directives: "(ingen direktiver læst)",
    master_pid: "Master-PID",
    legend: "Forklaring:",
    leg_red: "rød = mætning/kø",
    leg_green: "grøn = sund/inaktiv",
    leg_white: "hvid = neutral/ondemand",
    leg_blue: "blå = aktiv/static",
    read_proc: "kan ikke læse /proc (kræver Linux): {}",
    unknown_option: "ukendt valgmulighed: {}",
    run_help: "kør med --help",
    invalid_port: "ugyldig port: {} (forventet 1-65535)",
    web_serving: "webgrænsefladen lytter på http://127.0.0.1:{} (Ctrl-C for at afslutte)",
    web_error: "kan ikke starte webgrænsefladen: {}",
    skip_content: "Spring til indhold",
    language: "Sprog",
    web_dashboard: "webdashboard",
    theme_dark: "Mørk tilstand",
    theme_light: "Lys tilstand",
    connecting: "forbinder…",
    disconnected: "afbrudt",
    mock_data: "demodata",
    live: "live",
    updated: "opdateret",
    pool: "Pool",
    type_label: "Type",
    max_child: "Maks. børn",
    max_req: "Maks. anmodninger",
    min_spare: "min spare",
    max_spare: "max spare",
    max_backlog: "max backlog",
    configured_detected: "konfigureret + registreret",
    total_processes: "processer i alt",
    state_r: "tilstand R",
    state_s: "tilstand S",
    pending_connections: "ventende forbindelser",
    sum_rss: "sum af worker-RSS",
    global_directives: "Globale direktiver",
    footer_tagline: "læser den lokale procestabel, ingen netværksadgang.",
    collection_error: "Indsamlingsfejl:",
    cannot_reach: "Kan ikke nå fpm-monitor:",
    refresh: "opdatering",
};

pub(super) const USAGE: &str = "\
fpm-monitor — PHP-FPM-puljemonitor (Rust-port af fpm-monitor.c)

Anvendelse: fpm-monitor [MULIGHEDER]

Muligheder:
  -c, --config <STI>     php-fpm.conf-fil eller mappe med puljer, der skal analyseres
  -v, --verbose          Vis detaljer for hver worker (pid, tilstand, RSS)
      --color            Tving farver
      --no-color         Deaktiver farver
  -t, --tui              Interaktivt dashboard (som Ember) — faner
                         overvågning, grafer, logfiler og konfigurationer
                         (underfaner med ← →). Afslut: q / Ctrl-C
      --interval <SEK>   Opdateringsinterval for TUI (standard 1)
      --lang <SPROG>     UI-sprog: en (US), fr, zh, ar, es, it, ja, de,
                         ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
      --mock             Vis demodata (lokal test)
      --watch            Start webgrænsefladen i stedet for CLI-visningen
      --port <PORT>      Port til webgrænsefladen (standard 8471)
  -h, --help             Vis denne hjælp

Registreringen sker automatisk (sædvanlige placeringer + include-direktiv).
";
