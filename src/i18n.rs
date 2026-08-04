use std::sync::OnceLock;

pub static LANG: OnceLock<Lang> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    En,
    Fr,
    Zh,
    Ar,
    Es,
    It,
    Ja,
    De,
}

/// Définit la langue depuis la valeur de `--lang`.
pub fn set(name: &str) -> Result<Lang, String> {
    let lang = match name.trim().to_ascii_lowercase().as_str() {
        "en" | "us" | "english" | "anglais" => Lang::En,
        "fr" | "francais" | "français" | "french" => Lang::Fr,
        "zh" | "cn" | "chinois" | "chinese" => Lang::Zh,
        "ar" | "arabe" | "arabic" => Lang::Ar,
        "es" | "espagnol" | "spanish" => Lang::Es,
        "it" | "italien" | "italian" => Lang::It,
        "ja" | "japonais" | "japanese" => Lang::Ja,
        "de" | "allemand" | "german" => Lang::De,
        other => {
            return Err(format!(
                "unknown language: {other} (expected: en, fr, zh, ar, es, it, ja, de)"
            ))
        }
    };
    let _ = LANG.set(lang);
    Ok(lang)
}

pub fn lang() -> Lang {
    *LANG.get().unwrap_or(&Lang::En)
}

pub fn l() -> &'static Labels {
    match lang() {
        Lang::En => EN,
        Lang::Fr => FR,
        Lang::Zh => ZH,
        Lang::Ar => AR,
        Lang::Es => ES,
        Lang::It => IT,
        Lang::Ja => JA,
        Lang::De => DE,
    }
}

pub struct Labels {
    pub tab_monitoring: &'static str,
    pub tab_graphs: &'static str,
    pub tab_logs: &'static str,
    pub tab_configs: &'static str,
    pub pools: &'static str,
    pub workers: &'static str,
    pub running: &'static str,
    pub idle: &'static str,
    pub backlog: &'static str,
    pub mem: &'static str,
    pub master: &'static str,
    pub help: &'static str,
    pub no_pool: &'static str,
    pub last_measurements: &'static str,
    pub g_running: &'static str,
    pub g_workers: &'static str,
    pub g_idle: &'static str,
    pub g_backlog: &'static str,
    pub log_php: &'static str,
    pub slow_queries: &'static str,
    pub access: &'static str,
    pub no_entries: &'static str,
    pub older_lines: &'static str,
    pub more_workers: &'static str,
    pub pool_configs: &'static str,
    pub no_config: &'static str,
    pub unknown_pool: &'static str,
    pub no_directives: &'static str,
    pub master_pid: &'static str,
    pub legend: &'static str,
    pub leg_red: &'static str,
    pub leg_green: &'static str,
    pub leg_white: &'static str,
    pub leg_blue: &'static str,
    pub read_proc: &'static str,
    pub unknown_option: &'static str,
    pub run_help: &'static str,
}

const EN: &Labels = &Labels {
    tab_monitoring: "Monitoring",
    tab_graphs: "Graphs",
    tab_logs: "Logs",
    tab_configs: "Configurations",
    pools: "Pools",
    workers: "Workers",
    running: "Running",
    idle: "Idle",
    backlog: "Backlog",
    mem: "Mem",
    master: "Master",
    help: "q quit · Tab next tab · 1-4 tab · ← → subtab · ↑ ↓ monitoring",
    no_pool: "No pool detected.",
    last_measurements: "last {} measurements",
    g_running: "running",
    g_workers: "workers",
    g_idle: "idle",
    g_backlog: "backlog",
    log_php: "PHP log",
    slow_queries: "Slow queries",
    access: "Access",
    no_entries: "(no entries)",
    older_lines: "+{} older lines",
    more_workers: "+{} more",
    pool_configs: "Pool configurations",
    no_config: "No configuration found.",
    unknown_pool: "Unknown pool.",
    no_directives: "(no directives read)",
    master_pid: "Master PID",
    legend: "Legend:",
    leg_red: "red = saturation/backlog",
    leg_green: "green = healthy/idle",
    leg_white: "white = neutral/ondemand",
    leg_blue: "blue = active/static",
    read_proc: "unable to read /proc (Linux required): {}",
    unknown_option: "unknown option: {}",
    run_help: "run with --help",
};

const FR: &Labels = &Labels {
    tab_monitoring: "Monitoring",
    tab_graphs: "Graphiques",
    tab_logs: "Logs",
    tab_configs: "Configurations",
    pools: "Pools",
    workers: "Workers",
    running: "Actifs",
    idle: "Inactifs",
    backlog: "Attente",
    mem: "Mémoire",
    master: "Maître",
    help: "q quitter · Tab onglet suivant · 1-4 onglet · ← → sous-onglet · ↑ ↓ monitoring",
    no_pool: "Aucun pool détecté.",
    last_measurements: "dernières {} mesures",
    g_running: "actifs",
    g_workers: "processus",
    g_idle: "inactifs",
    g_backlog: "attente",
    log_php: "Log PHP",
    slow_queries: "Requêtes lentes",
    access: "Accès",
    no_entries: "(aucune entrée)",
    older_lines: "+{} lignes plus anciennes",
    more_workers: "+{} autres",
    pool_configs: "Configurations des pools",
    no_config: "Aucune configuration trouvée.",
    unknown_pool: "Pool inconnu.",
    no_directives: "(aucune directive lue)",
    master_pid: "PID maître",
    legend: "Légende :",
    leg_red: "rouge = saturation/attente",
    leg_green: "vert = sain / inactifs",
    leg_white: "blanc = neutre / ondemand",
    leg_blue: "bleu = actif / static",
    read_proc: "impossible de lire /proc (Linux requis) : {}",
    unknown_option: "option inconnue : {}",
    run_help: "lancer avec --help",
};

const ZH: &Labels = &Labels {
    tab_monitoring: "监控",
    tab_graphs: "图表",
    tab_logs: "日志",
    tab_configs: "配置",
    pools: "池",
    workers: "进程",
    running: "运行",
    idle: "空闲",
    backlog: "积压",
    mem: "内存",
    master: "主进程",
    help: "q 退出 · Tab 下一标签 · 1-4 标签 · ← → 子标签 · ↑ ↓ 监控",
    no_pool: "未检测到池。",
    last_measurements: "最近 {} 次测量",
    g_running: "运行",
    g_workers: "进程",
    g_idle: "空闲",
    g_backlog: "积压",
    log_php: "PHP 日志",
    slow_queries: "慢查询",
    access: "访问",
    no_entries: "（无记录）",
    older_lines: "+{} 条更早的记录",
    more_workers: "+{} 更多",
    pool_configs: "池配置",
    no_config: "未找到配置。",
    unknown_pool: "未知池。",
    no_directives: "（未读取到指令）",
    master_pid: "主进程 PID",
    legend: "图例：",
    leg_red: "红色 = 饱和/积压",
    leg_green: "绿色 = 正常/空闲",
    leg_white: "白色 = 中性/按需",
    leg_blue: "蓝色 = 活动/静态",
    read_proc: "无法读取 /proc（需要 Linux）：{}",
    unknown_option: "未知选项：{}",
    run_help: "使用 --help 运行",
};

const AR: &Labels = &Labels {
    tab_monitoring: "مراقبة",
    tab_graphs: "رسوم بيانية",
    tab_logs: "السجلات",
    tab_configs: "الإعدادات",
    pools: "المجموعات",
    workers: "العمال",
    running: "قيد التشغيل",
    idle: "خامل",
    backlog: "تراكم",
    mem: "الذاكرة",
    master: "الرئيسي",
    help: "q خروج · Tab التبويب التالي · 1-4 تبويب · ← → تبويب فرعي · ↑ ↓ مراقبة",
    no_pool: "لم يتم اكتشاف أي مجموعة.",
    last_measurements: "آخر {} قياسات",
    g_running: "تشغيل",
    g_workers: "عمال",
    g_idle: "خامل",
    g_backlog: "تراكم",
    log_php: "سجل PHP",
    slow_queries: "استعلامات بطيئة",
    access: "الوصول",
    no_entries: "(لا توجد إدخالات)",
    older_lines: "+{} أسطر أقدم",
    more_workers: "+{} آخرون",
    pool_configs: "إعدادات المجموعات",
    no_config: "لم يتم العثور على إعداد.",
    unknown_pool: "مجموعة غير معروفة.",
    no_directives: "(لا توجيهات مقروءة)",
    master_pid: "PID الرئيسي",
    legend: "مفتاح الرموز:",
    leg_red: "أحمر = تشبع/تراكم",
    leg_green: "أخضر = سليم/خامل",
    leg_white: "أبيض = محايد/عند الطلب",
    leg_blue: "أزرق = نشط/ثابت",
    read_proc: "تعذّر قراءة /proc (يتطلب لينكس): {}",
    unknown_option: "خيار غير معروف: {}",
    run_help: "شغّل مع --help",
};

const ES: &Labels = &Labels {
    tab_monitoring: "Monitoreo",
    tab_graphs: "Gráficas",
    tab_logs: "Registros",
    tab_configs: "Configuración",
    pools: "Pools",
    workers: "Trabajadores",
    running: "Activos",
    idle: "Inactivos",
    backlog: "Cola",
    mem: "Mem",
    master: "Maestro",
    help: "q salir · Tab siguiente pestaña · 1-4 pestaña · ← → subpestaña · ↑ ↓ monitoreo",
    no_pool: "No se detectó ningún pool.",
    last_measurements: "últimas {} mediciones",
    g_running: "activos",
    g_workers: "trabajadores",
    g_idle: "inactivos",
    g_backlog: "cola",
    log_php: "Log PHP",
    slow_queries: "Consultas lentas",
    access: "Acceso",
    no_entries: "(sin entradas)",
    older_lines: "+{} líneas más antiguas",
    more_workers: "+{} más",
    pool_configs: "Configuración de pools",
    no_config: "No se encontró configuración.",
    unknown_pool: "Pool desconocido.",
    no_directives: "(sin directivas leídas)",
    master_pid: "PID maestro",
    legend: "Leyenda:",
    leg_red: "rojo = saturación/cola",
    leg_green: "verde = sano/inactivo",
    leg_white: "blanco = neutro/ondemand",
    leg_blue: "azul = activo/static",
    read_proc: "no se puede leer /proc (requiere Linux): {}",
    unknown_option: "opción desconocida: {}",
    run_help: "ejecuta con --help",
};

const IT: &Labels = &Labels {
    tab_monitoring: "Monitoraggio",
    tab_graphs: "Grafici",
    tab_logs: "Log",
    tab_configs: "Configurazioni",
    pools: "Pool",
    workers: "Worker",
    running: "Attivi",
    idle: "Inattivi",
    backlog: "Coda",
    mem: "Mem",
    master: "Master",
    help: "q esci · Tab scheda successiva · 1-4 scheda · ← → sottoscheda · ↑ ↓ monitoraggio",
    no_pool: "Nessun pool rilevato.",
    last_measurements: "ultime {} misurazioni",
    g_running: "attivi",
    g_workers: "worker",
    g_idle: "inattivi",
    g_backlog: "coda",
    log_php: "Log PHP",
    slow_queries: "Query lente",
    access: "Accessi",
    no_entries: "(nessuna voce)",
    older_lines: "+{} righe più vecchie",
    more_workers: "+{} in più",
    pool_configs: "Configurazioni pool",
    no_config: "Nessuna configurazione trovata.",
    unknown_pool: "Pool sconosciuto.",
    no_directives: "(nessuna direttiva letta)",
    master_pid: "PID master",
    legend: "Legenda:",
    leg_red: "rosso = saturazione/coda",
    leg_green: "verde = sano/inattivo",
    leg_white: "bianco = neutro/ondemand",
    leg_blue: "blu = attivo/static",
    read_proc: "impossibile leggere /proc (richiede Linux): {}",
    unknown_option: "opzione sconosciuta: {}",
    run_help: "esegui con --help",
};

const JA: &Labels = &Labels {
    tab_monitoring: "モニタリング",
    tab_graphs: "グラフ",
    tab_logs: "ログ",
    tab_configs: "設定",
    pools: "プール",
    workers: "プロセス",
    running: "実行中",
    idle: "アイドル",
    backlog: "バックログ",
    mem: "メモリ",
    master: "マスター",
    help: "q 終了 · Tab 次のタブ · 1-4 タブ · ← → サブタブ · ↑ ↓ モニタリング",
    no_pool: "プールが検出されませんでした。",
    last_measurements: "直近 {} 回の測定",
    g_running: "実行中",
    g_workers: "プロセス",
    g_idle: "アイドル",
    g_backlog: "バックログ",
    log_php: "PHP ログ",
    slow_queries: "スロークエリ",
    access: "アクセス",
    no_entries: "（エントリなし）",
    older_lines: "+{} 件の古い行",
    more_workers: "+{} 件",
    pool_configs: "プール設定",
    no_config: "設定が見つかりません。",
    unknown_pool: "不明なプール。",
    no_directives: "（読み込まれたディレクティブなし）",
    master_pid: "マスター PID",
    legend: "凡例：",
    leg_red: "赤 = 飽和/バックログ",
    leg_green: "緑 = 正常/アイドル",
    leg_white: "白 = 中立/オンデマンド",
    leg_blue: "青 = アクティブ/静的",
    read_proc: "/proc を読み取れません（Linux が必要）：{}",
    unknown_option: "不明なオプション：{}",
    run_help: "--help で実行",
};

const DE: &Labels = &Labels {
    tab_monitoring: "Überwachung",
    tab_graphs: "Diagramme",
    tab_logs: "Logs",
    tab_configs: "Konfigurationen",
    pools: "Pools",
    workers: "Arbeiter",
    running: "Aktiv",
    idle: "Leerlauf",
    backlog: "Rückstand",
    mem: "RAM",
    master: "Master",
    help: "q beenden · Tab nächster Tab · 1-4 Tab · ← → Untertab · ↑ ↓ Überwachung",
    no_pool: "Kein Pool erkannt.",
    last_measurements: "letzte {} Messungen",
    g_running: "aktiv",
    g_workers: "arbeiter",
    g_idle: "leerlauf",
    g_backlog: "rückstand",
    log_php: "PHP-Protokoll",
    slow_queries: "Langsame Abfragen",
    access: "Zugriffe",
    no_entries: "(keine Einträge)",
    older_lines: "+{} ältere Zeilen",
    more_workers: "+{} weitere",
    pool_configs: "Pool-Konfigurationen",
    no_config: "Keine Konfiguration gefunden.",
    unknown_pool: "Unbekannter Pool.",
    no_directives: "(keine Direktiven gelesen)",
    master_pid: "Master-PID",
    legend: "Legende:",
    leg_red: "rot = Sättigung/Rückstand",
    leg_green: "grün = gesund/Leerlauf",
    leg_white: "weiß = neutral/ondemand",
    leg_blue: "blau = aktiv/static",
    read_proc: "/proc kann nicht gelesen werden (Linux erforderlich): {}",
    unknown_option: "unbekannte Option: {}",
    run_help: "mit --help ausführen",
};

/// Aide `--help` localisée.
pub fn usage() -> &'static str {
    match lang() {
        Lang::En => {
            "\
fpm-monitor — PHP-FPM pools monitor (Rust port of fpm-monitor.c)

Usage: fpm-monitor [OPTIONS]

Options:
  -c, --config <PATH>    php-fpm.conf file or pools directory to analyze
  -v, --verbose          Show each worker detail (pid, state, RSS)
      --color            Force color
      --no-color         Disable color
  -t, --tui              Interactive dashboard (like Ember) — tabs
                         monitoring, graphs, logs and configs (sub-tabs
                         navigable with ← →). Quit: q / Ctrl-C
      --interval <SEC>   TUI refresh interval (default 1)
      --lang <LANG>      UI language: en (US), fr, zh, ar, es, it, ja, de
      --mock             Show demo data (local test)
  -h, --help             Show this help

Detection is automatic (usual locations + include directive).
"
        }
        Lang::Fr => {
            "\
fpm-monitor — moniteur de pools PHP-FPM (port Rust de fpm-monitor.c)

Usage: fpm-monitor [OPTIONS]

Options:
  -c, --config <PATH>    Fichier php-fpm.conf ou dossier de pools à analyser
  -v, --verbose          Affiche le détail de chaque worker (pid, état, RSS)
      --color            Force la couleur
      --no-color         Désactive la couleur
  -t, --tui              Dashboard interactif (comme Ember) — onglets
                         monitoring, graphiques, logs et configs (sous-
                         onglets navigables ← →). Quitter: q / Ctrl-C
      --interval <SEC>   Intervalle de rafraîchissement du TUI (défaut 1)
      --lang <LANG>      Langue de l'interface : en (US), fr, zh, ar, es, it, ja, de
      --mock             Affiche des données de démonstration (test local)
  -h, --help             Affiche cette aide

La détection est automatique (emplacements usuels + directive include).
"
        }
        Lang::Zh => {
            "\
fpm-monitor — PHP-FPM 进程池监视器（fpm-monitor.c 的 Rust 移植版）

用法: fpm-monitor [选项]

选项:
  -c, --config <路径>    要分析的 php-fpm.conf 文件或进程池目录
  -v, --verbose          显示每个 worker 的详细信息（pid、状态、RSS）
      --color            强制启用颜色
      --no-color         禁用颜色
  -t, --tui              交互式仪表盘（类似 Ember）—— 标签页
                         监控、图表、日志和配置（子标签可用 ← → 导航）。退出: q / Ctrl-C
      --interval <秒>    TUI 刷新间隔（默认 1）
      --lang <语言>      界面语言：en (US)、fr、zh、ar、es、it、ja、de
      --mock             显示演示数据（本地测试）
  -h, --help             显示此帮助

自动检测（常用位置 + include 指令）。
"
        }
        Lang::Ar => {
            "\
fpm-monitor — مراقب تجمعات PHP-FPM (منفذ بلغة Rust من fpm-monitor.c)

الاستخدام: fpm-monitor [خيارات]

الخيارات:
  -c, --config <المسار>  ملف php-fpm.conf أو مجلد التجمعات للتحليل
  -v, --verbose          عرض تفاصيل كل عامل (pid، الحالة، RSS)
      --color            فرض الألوان
      --no-color         تعطيل الألوان
  -t, --tui              لوحة تفاعلية (مثل Ember) — تبويبات
                         المراقبة والرسوم البيانية والسجلات والإعدادات (تبويبات
                         فرعية تُتنقّل بـ ← →). الخروج: q / Ctrl-C
      --interval <ث>     فترة تحديث الواجهة (الافتراضي 1)
      --lang <اللغة>     لغة الواجهة: en (US)، fr، zh، ar، es، it، ja، de
      --mock             عرض بيانات تجريبية (اختبار محلي)
  -h, --help             عرض هذه المساعدة

الاكتشاف تلقائي (الأماكن المعتادة + توجيه include).
"
        }
        Lang::Es => {
            "\
fpm-monitor — monitor de pools PHP-FPM (port en Rust de fpm-monitor.c)

Uso: fpm-monitor [OPCIONES]

Opciones:
  -c, --config <RUTA>    Archivo php-fpm.conf o carpeta de pools a analizar
  -v, --verbose          Muestra el detalle de cada worker (pid, estado, RSS)
      --color            Forzar color
      --no-color         Desactivar color
  -t, --tui              Dashboard interactivo (como Ember) — pestañas
                         monitoreo, gráficas, registros y configuración (sub-
                         pestañas navegables con ← →). Salir: q / Ctrl-C
      --interval <SEG>   Intervalo de refresco del TUI (por defecto 1)
      --lang <LANG>      Idioma: en (US), fr, zh, ar, es, it, ja, de
      --mock             Mostrar datos de demostración (prueba local)
  -h, --help             Muestra esta ayuda

La detección es automática (ubicaciones usuales + directiva include).
"
        }
        Lang::It => {
            "\
fpm-monitor — monitor dei pool PHP-FPM (port in Rust di fpm-monitor.c)

Uso: fpm-monitor [OPZIONI]

Opzioni:
  -c, --config <PERCORSO>  File php-fpm.conf o cartella dei pool da analizzare
  -v, --verbose            Mostra il dettaglio di ogni worker (pid, stato, RSS)
      --color              Forza il colore
      --no-color           Disabilita il colore
  -t, --tui                Dashboard interattivo (come Ember) — schede
                           monitoraggio, grafici, log e configurazioni (sotto-
                           schede navigabili con ← →). Esci: q / Ctrl-C
      --interval <SEC>     Intervallo di aggiornamento del TUI (default 1)
      --lang <LINGUA>      Lingua interfaccia: en (US), fr, zh, ar, es, it, ja, de
      --mock               Mostra dati dimostrativi (test locale)
  -h, --help               Mostra questo aiuto

La rilevazione è automatica (posizioni usuali + direttiva include).
"
        }
        Lang::Ja => {
            "\
fpm-monitor — PHP-FPM プールモニター（fpm-monitor.c の Rust 移植版）

使用法: fpm-monitor [オプション]

オプション:
  -c, --config <パス>   解析する php-fpm.conf ファイルまたはプールディレクトリ
  -v, --verbose         各ワーカーの詳細を表示（pid、状態、RSS）
      --color           色を強制
      --no-color        色を無効化
  -t, --tui             インタラクティブダッシュボード（Ember 風）— タブ
                         モニタリング、グラフ、ログ、設定（サブタブは
                         ← → で移動）。終了: q / Ctrl-C
      --interval <秒>   TUI 更新間隔（デフォルト 1）
      --lang <言語>      UI 言語: en (US)、fr、zh、ar、es、it、ja、de
      --mock            デモデータを表示（ローカルテスト）
  -h, --help            このヘルプを表示

検出は自動です（一般的な場所 + include ディレクティブ）。
"
        }
        Lang::De => {
            "\
fpm-monitor — PHP-FPM-Pool-Monitor (Rust-Port von fpm-monitor.c)

Verwendung: fpm-monitor [OPTIONEN]

Optionen:
  -c, --config <PFAD>    php-fpm.conf-Datei oder Pool-Verzeichnis zur Analyse
  -v, --verbose          Worker-Details anzeigen (PID, Status, RSS)
      --color            Farbe erzwingen
      --no-color         Farbe deaktivieren
  -t, --tui              Interaktives Dashboard (wie Ember) — Tabs
                         Überwachung, Diagramme, Logs und Konfigurationen
                         (Untertabs mit ← →). Beenden: q / Ctrl-C
      --interval <SEK>   TUI-Aktualisierungsintervall (Standard 1)
      --lang <SPRACHE>   UI-Sprache: en (US), fr, zh, ar, es, it, ja, de
      --mock             Demodaten anzeigen (lokaler Test)
  -h, --help             Diese Hilfe anzeigen

Die Erkennung ist automatisch (übliche Orte + include-Direktive).
"
        }
    }
}
