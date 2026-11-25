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
    skip_content: "Перейти до вмісту",
    menu: "Меню",
    language: "Мова",
    web_dashboard: "вебпанель",
    theme_dark: "Темний режим",
    theme_light: "Світлий режим",
    connecting: "підключення…",
    disconnected: "від'єднано",
    mock_data: "демо-дані",
    live: "у реальному часі",
    updated: "оновлено",
    pool: "Пул",
    type_label: "Тип",
    max_child: "Макс. нащадків",
    max_req: "Макс. запитів",
    min_spare: "min spare",
    max_spare: "max spare",
    max_backlog: "max backlog",
    configured_detected: "налаштовано + виявлено",
    total_processes: "усього процесів",
    state_r: "стан R",
    state_s: "стан S",
    pending_connections: "очікувані з'єднання",
    sum_rss: "сума RSS воркерів",
    global_directives: "Глобальні директиви",
    footer_tagline: "читає локальну таблицю процесів, без доступу до мережі.",
    requests: "Запити/с",
    total_requests: "Усього запитів",
    copy: "Копіювати",
    copied: "Скопійовано",
    min: "мін",
    max: "макс",
    avg: "середнє",
    axis_x: "час",
    axis_y: "значення",
    collection_error: "Помилка збору:",
    cannot_reach: "Не вдається підключитися до fpm-monitor:",
    refresh: "оновлення",
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
  -V, --version          Show the version

Виявлення виконується автоматично (звичайні розташування + директива include).
";
