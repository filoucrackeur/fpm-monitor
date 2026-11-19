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
  -h, --help             Vis denne hjælp

Registreringen sker automatisk (sædvanlige placeringer + include-direktiv).
";
