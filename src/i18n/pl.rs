use super::Labels;

pub(super) const LABELS: &Labels = &Labels {
    tab_monitoring: "Monitorowanie",
    tab_graphs: "Wykresy",
    tab_logs: "Dzienniki",
    tab_configs: "Konfiguracje",
    pools: "Pule",
    workers: "Procesy",
    running: "Aktywne",
    idle: "Bezczynne",
    backlog: "Kolejka",
    mem: "Pamięć",
    master: "Master",
    help: "q wyjście · Tab następna zakładka · 1-4 zakładka · ← → podzakładka · ↑ ↓ monitorowanie",
    no_pool: "Nie wykryto żadnej puli.",
    last_measurements: "ostatnie {} pomiary",
    g_running: "aktywne",
    g_workers: "procesy",
    g_idle: "bezczynne",
    g_backlog: "kolejka",
    log_php: "Dziennik PHP",
    slow_queries: "Wolne zapytania",
    access: "Dostęp",
    no_entries: "(brak wpisów)",
    older_lines: "+{} starszych linii",
    more_workers: "+{} więcej",
    pool_configs: "Konfiguracje pul",
    no_config: "Nie znaleziono konfiguracji.",
    unknown_pool: "Nieznana pula.",
    no_directives: "(nie odczytano dyrektyw)",
    master_pid: "PID master",
    legend: "Legenda:",
    leg_red: "czerwony = nasycenie/kolejka",
    leg_green: "zielony = zdrowy/bezczynny",
    leg_white: "biały = neutralny/ondemand",
    leg_blue: "niebieski = aktywny/static",
    read_proc: "nie można odczytać /proc (wymagany Linux): {}",
    unknown_option: "nieznana opcja: {}",
    run_help: "uruchom z --help",
    invalid_port: "nieprawidłowy port: {} (oczekiwano 1-65535)",
    web_serving: "interfejs webowy nasłuchuje na http://127.0.0.1:{} (Ctrl-C, aby wyjść)",
    web_error: "nie można uruchomić interfejsu webowego: {}",
    skip_content: "Przejdź do treści",
    language: "Język",
    web_dashboard: "panel internetowy",
    theme_dark: "Tryb ciemny",
    theme_light: "Tryb jasny",
    connecting: "łączenie…",
    disconnected: "rozłączono",
    mock_data: "dane demonstracyjne",
    live: "na żywo",
    updated: "zaktualizowano",
    pool: "Pula",
    type_label: "Typ",
    max_child: "Maks. procesów potomnych",
    max_req: "Maks. żądań",
    min_spare: "min spare",
    max_spare: "max spare",
    max_backlog: "max backlog",
    configured_detected: "skonfigurowane + wykryte",
    total_processes: "procesy łącznie",
    state_r: "stan R",
    state_s: "stan S",
    pending_connections: "oczekujące połączenia",
    sum_rss: "suma RSS workerów",
    global_directives: "Globalne dyrektywy",
    footer_tagline: "odczytuje lokalną tabelę procesów, bez dostępu do sieci.",
    collection_error: "Błąd zbierania:",
    cannot_reach: "Nie można połączyć się z fpm-monitor:",
    refresh: "odświeżanie",
};

pub(super) const USAGE: &str = "\
fpm-monitor — monitor pul PHP-FPM (port fpm-monitor.c w Rust)

Użycie: fpm-monitor [OPCJE]

Opcje:
  -c, --config <ŚCIEŻKA> plik php-fpm.conf lub katalog pul do analizy
  -v, --verbose          pokaż szczegóły każdego procesu (pid, stan, RSS)
      --color            wymuś kolory
      --no-color         wyłącz kolory
  -t, --tui              interaktywny pulpit (jak Ember) — zakładki
                         monitorowanie, wykresy, dzienniki i konfiguracje
                         (podzakładki za pomocą ← →). Wyjście: q / Ctrl-C
      --interval <SEK>   interwał odświeżania TUI (domyślnie 1)
      --lang <JĘZYK>     język interfejsu: en (US), fr, zh, ar, es, it, ja, de,
                         ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
      --mock             pokaż dane demonstracyjne (test lokalny)
      --watch            uruchom interfejs webowy zamiast widoku CLI
      --port <PORT>      port interfejsu webowego (domyślnie 8471)
  -h, --help             pokaż tę pomoc

Wykrywanie jest automatyczne (typowe lokalizacje + dyrektywa include).
";
