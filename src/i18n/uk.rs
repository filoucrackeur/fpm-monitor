use super::Labels;

pub(super) const LABELS: &Labels = &Labels {
    tab_monitoring: "Моніторинг",
    tab_graphs: "Графіки",
    tab_logs: "Журнали",
    tab_configs: "Конфігурації",
    pools: "Пули",
    workers: "Процеси",
    running: "Активні",
    idle: "Простоюють",
    backlog: "Черга",
    mem: "Пам'ять",
    master: "Master",
    help: "q вихід · Tab наступна вкладка · 1-4 вкладка · ← → підвкладка · ↑ ↓ моніторинг",
    no_pool: "Пулів не виявлено.",
    last_measurements: "останні {} вимірювань",
    g_running: "активні",
    g_workers: "процеси",
    g_idle: "простоюють",
    g_backlog: "черга",
    log_php: "Журнал PHP",
    slow_queries: "Повільні запити",
    access: "Доступ",
    no_entries: "(немає записів)",
    older_lines: "+{} старіших рядків",
    more_workers: "+{} ще",
    pool_configs: "Конфігурації пулів",
    no_config: "Конфігурацію не знайдено.",
    unknown_pool: "Невідомий пул.",
    no_directives: "(директиви не прочитано)",
    master_pid: "PID master-процесу",
    legend: "Позначення:",
    leg_red: "червоний = насичення/черга",
    leg_green: "зелений = норма/простій",
    leg_white: "білий = нейтрально/ondemand",
    leg_blue: "синій = активний/static",
    read_proc: "не вдалося прочитати /proc (потрібен Linux): {}",
    unknown_option: "невідомий параметр: {}",
    run_help: "запустіть із --help",
    invalid_port: "недійсний порт: {} (очікується 1-65535)",
    web_serving: "веб-інтерфейс слухає http://127.0.0.1:{} (Ctrl-C для виходу)",
    web_error: "не вдалося запустити веб-інтерфейс: {}",
};

pub(super) const USAGE: &str = "\
fpm-monitor — монітор пулів PHP-FPM (порт fpm-monitor.c на Rust)

Використання: fpm-monitor [ПАРАМЕТРИ]

Параметри:
  -c, --config <ШЛЯХ>    файл php-fpm.conf або каталог пулів для аналізу
  -v, --verbose          показати відомості про кожен процес (pid, стан, RSS)
      --color            примусово увімкнути кольори
      --no-color         вимкнути кольори
  -t, --tui              інтерактивна панель (як Ember) — вкладки
                         моніторинг, графіки, журнали та конфігурації
                         (підвкладки за допомогою ← →). Вихід: q / Ctrl-C
      --interval <СЕК>   інтервал оновлення TUI (типово 1)
      --lang <МОВА>      мова інтерфейсу: en (US), fr, zh, ar, es, it, ja, de,
                         ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
      --mock             показати демонстраційні дані (локальний тест)
      --watch            запустити веб-інтерфейс замість подання CLI
      --port <PORT>      порт веб-інтерфейсу (типово 8471)
  -h, --help             показати цю довідку

Виявлення виконується автоматично (звичайні розташування + директива include).
";
