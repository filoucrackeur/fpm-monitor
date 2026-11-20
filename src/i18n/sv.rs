use super::Labels;

pub(super) const LABELS: &Labels = &Labels {
    tab_monitoring: "Övervakning",
    tab_graphs: "Diagram",
    tab_logs: "Loggar",
    tab_configs: "Konfigurationer",
    pools: "Pooler",
    workers: "Processer",
    running: "Aktiva",
    idle: "Inaktiva",
    backlog: "Kö",
    mem: "Minne",
    master: "Master",
    help: "q avsluta · Tab nästa flik · 1-4 flik · ← → underflik · ↑ ↓ övervakning",
    no_pool: "Ingen pool hittades.",
    last_measurements: "senaste {} mätningarna",
    g_running: "aktiva",
    g_workers: "processer",
    g_idle: "inaktiva",
    g_backlog: "kö",
    log_php: "PHP-logg",
    slow_queries: "Långsamma frågor",
    access: "Åtkomst",
    no_entries: "(inga poster)",
    older_lines: "+{} äldre rader",
    more_workers: "+{} fler",
    pool_configs: "Poolkonfigurationer",
    no_config: "Ingen konfiguration hittades.",
    unknown_pool: "Okänd pool.",
    no_directives: "(inga direktiv lästa)",
    master_pid: "Master-PID",
    legend: "Teckenförklaring:",
    leg_red: "röd = mättnad/kö",
    leg_green: "grön = frisk/inaktiv",
    leg_white: "vit = neutral/ondemand",
    leg_blue: "blå = aktiv/static",
    read_proc: "kan inte läsa /proc (Linux krävs): {}",
    unknown_option: "okänt alternativ: {}",
    run_help: "kör med --help",
    invalid_port: "ogiltig port: {} (förväntat 1-65535)",
    web_serving: "webbgränssnittet lyssnar på http://127.0.0.1:{} (Ctrl-C för att avsluta)",
    web_error: "kan inte starta webbgränssnittet: {}",
};

pub(super) const USAGE: &str = "\
fpm-monitor — PHP-FPM-poolmonitor (Rust-port av fpm-monitor.c)

Användning: fpm-monitor [ALTERNATIV]

Alternativ:
  -c, --config <SÖKVÄG>  php-fpm.conf-fil eller poolkatalog att analysera
  -v, --verbose          Visa detaljer för varje process (pid, status, RSS)
      --color            Tvinga färg
      --no-color         Inaktivera färg
  -t, --tui              Interaktiv instrumentpanel (som Ember) — flikar
                         övervakning, diagram, loggar och konfigurationer
                         (underflikar navigeras med ← →). Avsluta: q / Ctrl-C
      --interval <SEK>   Uppdateringsintervall för TUI (standard 1)
      --lang <SPRÅK>     Gränssnittsspråk: en (US), fr, zh, ar, es, it, ja, de,
                         ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
      --mock             Visa demodata (lokalt test)
      --watch            Starta webbgränssnittet i stället för CLI-vyn
      --port <PORT>      Port för webbgränssnittet (standard 8471)
  -h, --help             Visa denna hjälp

Identifieringen är automatisk (vanliga platser + include-direktiv).
";
