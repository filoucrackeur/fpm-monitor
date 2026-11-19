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
  -h, --help             показать эту справку

Обнаружение выполняется автоматически (обычные расположения + директива include).
";
