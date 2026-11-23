use super::Labels;

pub(super) const LABELS: &Labels = &Labels {
    tab_monitoring: "Мониторинг",
    tab_graphs: "Графики",
    tab_logs: "Журналы",
    tab_configs: "Конфигурации",
    pools: "Пулы",
    workers: "Процессы",
    running: "Активные",
    idle: "Простаивающие",
    backlog: "Очередь",
    mem: "Память",
    master: "Master",
    help: "q выход · Tab следующая вкладка · 1-4 вкладка · ← → подвкладка · ↑ ↓ мониторинг",
    no_pool: "Пулы не обнаружены.",
    last_measurements: "последние {} измерений",
    g_running: "активные",
    g_workers: "процессы",
    g_idle: "простаивающие",
    g_backlog: "очередь",
    log_php: "Журнал PHP",
    slow_queries: "Медленные запросы",
    access: "Доступ",
    no_entries: "(нет записей)",
    older_lines: "+{} более старых строк",
    more_workers: "+{} ещё",
    pool_configs: "Конфигурации пулов",
    no_config: "Конфигурация не найдена.",
    unknown_pool: "Неизвестный пул.",
    no_directives: "(директивы не прочитаны)",
    master_pid: "PID master-процесса",
    legend: "Обозначения:",
    leg_red: "красный = насыщение/очередь",
    leg_green: "зелёный = норма/простой",
    leg_white: "белый = нейтрально/ondemand",
    leg_blue: "синий = активен/static",
    read_proc: "не удалось прочитать /proc (требуется Linux): {}",
    unknown_option: "неизвестный параметр: {}",
    run_help: "запустите с --help",
    invalid_port: "неверный порт: {} (ожидается 1-65535)",
    web_serving: "веб-интерфейс слушает http://127.0.0.1:{} (Ctrl-C для выхода)",
    web_error: "не удалось запустить веб-интерфейс: {}",
    skip_content: "Перейти к содержимому",
    language: "Язык",
    web_dashboard: "веб-панель",
    theme_dark: "Тёмный режим",
    theme_light: "Светлый режим",
    connecting: "подключение…",
    disconnected: "отключено",
    mock_data: "демо-данные",
    live: "в реальном времени",
    updated: "обновлено",
    pool: "Пул",
    type_label: "Тип",
    max_child: "Макс. потомков",
    max_req: "Макс. запросов",
    min_spare: "min spare",
    max_spare: "max spare",
    max_backlog: "max backlog",
    configured_detected: "настроено + обнаружено",
    total_processes: "всего процессов",
    state_r: "состояние R",
    state_s: "состояние S",
    pending_connections: "ожидающие соединения",
    sum_rss: "сумма RSS воркеров",
    global_directives: "Глобальные директивы",
    footer_tagline: "читает локальную таблицу процессов, без доступа к сети.",
    collection_error: "Ошибка сбора:",
    cannot_reach: "Не удаётся подключиться к fpm-monitor:",
    refresh: "обновление",
};

pub(super) const USAGE: &str = "\
fpm-monitor — монитор пулов PHP-FPM (порт fpm-monitor.c на Rust)

Использование: fpm-monitor [ПАРАМЕТРЫ]

Параметры:
  -c, --config <ПУТЬ>    файл php-fpm.conf или каталог пулов для анализа
  -v, --verbose          показать сведения о каждом процессе (pid, состояние, RSS)
      --color            принудительно включить цвета
      --no-color         отключить цвета
  -t, --tui              интерактивная панель (как Ember) — вкладки
                         мониторинг, графики, журналы и конфигурации
                         (подвкладки с помощью ← →). Выход: q / Ctrl-C
      --interval <СЕК>   интервал обновления TUI (по умолчанию 1)
      --lang <ЯЗЫК>      язык интерфейса: en (US), fr, zh, ar, es, it, ja, de,
                         ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
      --mock             показать демонстрационные данные (локальный тест)
      --watch            запустить веб-интерфейс вместо представления CLI
      --port <PORT>      порт веб-интерфейса (по умолчанию 8471)
  -h, --help             показать эту справку

Обнаружение выполняется автоматически (обычные расположения + директива include).
";
