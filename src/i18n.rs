use std::sync::OnceLock;

mod da;
mod el;
mod fi;
mod hi;
mod id;
mod ko;
mod no;
mod pl;
mod ru;
mod sv;
mod th;
mod tr;
mod uk;
mod vi;

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
    Ru,
    Hi,
    Id,
    Ko,
    No,
    Sv,
    Fi,
    Da,
    El,
    Tr,
    Pl,
    Uk,
    Th,
    Vi,
}

/// Sets the language from the `--lang` value.
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
        "ru" | "russe" | "russian" => Lang::Ru,
        "hi" | "hindi" => Lang::Hi,
        "id" | "indonesien" | "indonésien" | "indonesian" => Lang::Id,
        "ko" | "coreen" | "coréen" | "korean" => Lang::Ko,
        "no" | "nb" | "norvegien" | "norvégien" | "norwegian" => Lang::No,
        "sv" | "suedois" | "suédois" | "swedish" => Lang::Sv,
        "fi" | "finlandais" | "finnish" => Lang::Fi,
        "da" | "danois" | "danish" => Lang::Da,
        "el" | "grec" | "greek" => Lang::El,
        "tr" | "turc" | "turkish" => Lang::Tr,
        "pl" | "polonais" | "polish" => Lang::Pl,
        "uk" | "ukrainien" | "ukrainian" => Lang::Uk,
        "th" | "thai" | "thaï" => Lang::Th,
        "vi" | "vietnamien" | "vietnamese" => Lang::Vi,
        other => {
            return Err(format!(
                "unknown language: {other} (expected: en, fr, zh, ar, es, it, ja, de, \
ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi)"
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
        Lang::Ru => ru::LABELS,
        Lang::Hi => hi::LABELS,
        Lang::Id => id::LABELS,
        Lang::Ko => ko::LABELS,
        Lang::No => no::LABELS,
        Lang::Sv => sv::LABELS,
        Lang::Fi => fi::LABELS,
        Lang::Da => da::LABELS,
        Lang::El => el::LABELS,
        Lang::Tr => tr::LABELS,
        Lang::Pl => pl::LABELS,
        Lang::Uk => uk::LABELS,
        Lang::Th => th::LABELS,
        Lang::Vi => vi::LABELS,
    }
}

/// One selectable locale, with its native name for the web language menu.
pub struct Locale {
    pub code: &'static str,
    pub name: &'static str,
    pub labels: &'static Labels,
}

/// Every locale exposed by the web dashboard, English first.
pub fn catalogue() -> &'static [Locale] {
    &[
        Locale {
            code: "en",
            name: "English",
            labels: EN,
        },
        Locale {
            code: "fr",
            name: "Français",
            labels: FR,
        },
        Locale {
            code: "zh",
            name: "中文",
            labels: ZH,
        },
        Locale {
            code: "ar",
            name: "العربية",
            labels: AR,
        },
        Locale {
            code: "es",
            name: "Español",
            labels: ES,
        },
        Locale {
            code: "it",
            name: "Italiano",
            labels: IT,
        },
        Locale {
            code: "ja",
            name: "日本語",
            labels: JA,
        },
        Locale {
            code: "de",
            name: "Deutsch",
            labels: DE,
        },
        Locale {
            code: "ru",
            name: "Русский",
            labels: ru::LABELS,
        },
        Locale {
            code: "hi",
            name: "हिन्दी",
            labels: hi::LABELS,
        },
        Locale {
            code: "id",
            name: "Bahasa Indonesia",
            labels: id::LABELS,
        },
        Locale {
            code: "ko",
            name: "한국어",
            labels: ko::LABELS,
        },
        Locale {
            code: "no",
            name: "Norsk",
            labels: no::LABELS,
        },
        Locale {
            code: "sv",
            name: "Svenska",
            labels: sv::LABELS,
        },
        Locale {
            code: "fi",
            name: "Suomi",
            labels: fi::LABELS,
        },
        Locale {
            code: "da",
            name: "Dansk",
            labels: da::LABELS,
        },
        Locale {
            code: "el",
            name: "Ελληνικά",
            labels: el::LABELS,
        },
        Locale {
            code: "tr",
            name: "Türkçe",
            labels: tr::LABELS,
        },
        Locale {
            code: "pl",
            name: "Polski",
            labels: pl::LABELS,
        },
        Locale {
            code: "uk",
            name: "Українська",
            labels: uk::LABELS,
        },
        Locale {
            code: "th",
            name: "ไทย",
            labels: th::LABELS,
        },
        Locale {
            code: "vi",
            name: "Tiếng Việt",
            labels: vi::LABELS,
        },
    ]
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
    pub invalid_port: &'static str,
    pub web_serving: &'static str,
    pub web_error: &'static str,
    pub skip_content: &'static str,
    pub menu: &'static str,
    pub language: &'static str,
    pub web_dashboard: &'static str,
    pub theme_dark: &'static str,
    pub theme_light: &'static str,
    pub connecting: &'static str,
    pub disconnected: &'static str,
    pub mock_data: &'static str,
    pub live: &'static str,
    pub updated: &'static str,
    pub pool: &'static str,
    pub type_label: &'static str,
    pub listen_unix: &'static str,
    pub listen_tcp: &'static str,
    pub severity_other: &'static str,
    pub severity: &'static str,
    pub severity_notice: &'static str,
    pub severity_warning: &'static str,
    pub severity_error: &'static str,
    pub log_type: &'static str,
    pub pool_choice: &'static str,
    pub max_child: &'static str,
    pub max_req: &'static str,
    pub min_spare: &'static str,
    pub max_spare: &'static str,
    pub max_backlog: &'static str,
    pub configured_detected: &'static str,
    pub total_processes: &'static str,
    pub state_r: &'static str,
    pub state_s: &'static str,
    pub pending_connections: &'static str,
    pub sum_rss: &'static str,
    pub global_directives: &'static str,
    pub footer_tagline: &'static str,
    pub requests: &'static str,
    pub total_requests: &'static str,
    pub copy: &'static str,
    pub copied: &'static str,
    pub min: &'static str,
    pub max: &'static str,
    pub avg: &'static str,
    pub axis_x: &'static str,
    pub axis_y: &'static str,
    pub collection_error: &'static str,
    pub cannot_reach: &'static str,
    pub refresh: &'static str,
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
    invalid_port: "invalid port: {} (expected 1-65535)",
    web_serving: "web UI listening on http://127.0.0.1:{} (Ctrl-C to quit)",
    web_error: "unable to start the web UI: {}",
    skip_content: "Skip to content",
    menu: "Menu",
    language: "Language",
    web_dashboard: "web dashboard",
    theme_dark: "Dark mode",
    theme_light: "Light mode",
    connecting: "connecting…",
    disconnected: "disconnected",
    mock_data: "mock data",
    live: "live",
    updated: "updated",
    pool: "Pool",
    type_label: "Type",
    listen_unix: "Unix socket",
    listen_tcp: "TCP network",
    severity_other: "Other",
    severity: "Severity",
    severity_notice: "Notice",
    severity_warning: "Warning",
    severity_error: "Error",
    log_type: "Log type",
    pool_choice: "Pool selection",
    max_child: "Max child",
    max_req: "Max req",
    min_spare: "min spare",
    max_spare: "max spare",
    max_backlog: "max backlog",
    configured_detected: "configured + detected",
    total_processes: "total processes",
    state_r: "state R",
    state_s: "state S",
    pending_connections: "pending connections",
    sum_rss: "sum of worker RSS",
    global_directives: "Global directives",
    footer_tagline: "reads the local process table, no network access.",
    requests: "Requests/s",
    total_requests: "Total requests",
    copy: "Copy",
    copied: "Copied!",
    min: "min",
    max: "max",
    avg: "avg",
    axis_x: "time",
    axis_y: "value",
    collection_error: "Collection error:",
    cannot_reach: "Cannot reach fpm-monitor:",
    refresh: "refresh",
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
    invalid_port: "port invalide : {} (attendu 1-65535)",
    web_serving: "interface web à l'écoute sur http://127.0.0.1:{} (Ctrl-C pour quitter)",
    web_error: "impossible de démarrer l'interface web : {}",
    skip_content: "Aller au contenu",
    menu: "Menu",
    language: "Langue",
    web_dashboard: "tableau de bord web",
    theme_dark: "Mode sombre",
    theme_light: "Mode clair",
    connecting: "connexion…",
    disconnected: "déconnecté",
    mock_data: "données de démo",
    live: "en direct",
    updated: "mis à jour",
    pool: "Pool",
    type_label: "Type",
    listen_unix: "Socket Unix",
    listen_tcp: "Réseau TCP",
    severity_other: "Autre",
    severity: "Sévérité",
    severity_notice: "Notice",
    severity_warning: "Avertissement",
    severity_error: "Erreur",
    log_type: "Type de log",
    pool_choice: "Choix du pool",
    max_child: "Max enfants",
    max_req: "Max requêtes",
    min_spare: "Min. inactifs",
    max_spare: "Max. inactifs",
    max_backlog: "Max. d'attente",
    configured_detected: "configurés + détectés",
    total_processes: "processus au total",
    state_r: "état R",
    state_s: "état S",
    pending_connections: "connexions en attente",
    sum_rss: "somme RSS des workers",
    global_directives: "Directives globales",
    footer_tagline: "lit la table des processus locale, aucune requête réseau.",
    requests: "Requêtes/s",
    total_requests: "Requêtes totales",
    copy: "Copier",
    copied: "Copié !",
    min: "min",
    max: "max",
    avg: "moy.",
    axis_x: "temps",
    axis_y: "valeur",
    collection_error: "Erreur de collecte :",
    cannot_reach: "Impossible de joindre fpm-monitor :",
    refresh: "rafraîchissement",
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
    invalid_port: "无效端口：{}（应为 1-65535）",
    web_serving: "Web 界面正在监听 http://127.0.0.1:{}（Ctrl-C 退出）",
    web_error: "无法启动 Web 界面：{}",
    skip_content: "跳到内容",
    menu: "菜单",
    language: "语言",
    web_dashboard: "Web 仪表板",
    theme_dark: "深色模式",
    theme_light: "浅色模式",
    connecting: "连接中…",
    disconnected: "已断开",
    mock_data: "模拟数据",
    live: "实时",
    updated: "更新于",
    pool: "池",
    type_label: "类型",
    listen_unix: "Unix 套接字",
    listen_tcp: "TCP 网络",
    severity_other: "其他",
    severity: "级别",
    severity_notice: "通知",
    severity_warning: "警告",
    severity_error: "错误",
    log_type: "日志类型",
    pool_choice: "选择进程池",
    max_child: "最大子进程",
    max_req: "最大请求",
    min_spare: "最小空闲",
    max_spare: "最大空闲",
    max_backlog: "最大积压",
    configured_detected: "已配置 + 已检测",
    total_processes: "进程总数",
    state_r: "状态 R",
    state_s: "状态 S",
    pending_connections: "待处理连接",
    sum_rss: "工作进程 RSS 总和",
    global_directives: "全局指令",
    footer_tagline: "读取本地进程表，无网络访问。",
    requests: "请求/秒",
    total_requests: "请求总数",
    copy: "复制",
    copied: "已复制",
    min: "最小",
    max: "最大",
    avg: "平均",
    axis_x: "时间",
    axis_y: "数值",
    collection_error: "采集错误：",
    cannot_reach: "无法连接到 fpm-monitor：",
    refresh: "刷新",
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
    invalid_port: "منفذ غير صالح: {} (المتوقع 1-65535)",
    web_serving: "واجهة الويب تستمع على http://127.0.0.1:{} (Ctrl-C للخروج)",
    web_error: "تعذّر تشغيل واجهة الويب: {}",
    skip_content: "تخطَّ إلى المحتوى",
    menu: "القائمة",
    language: "اللغة",
    web_dashboard: "لوحة المعلومات على الويب",
    theme_dark: "الوضع الداكن",
    theme_light: "الوضع الفاتح",
    connecting: "جارٍ الاتصال…",
    disconnected: "غير متصل",
    mock_data: "بيانات تجريبية",
    live: "مباشر",
    updated: "آخر تحديث",
    pool: "المجموعة",
    type_label: "النوع",
    listen_unix: "مقبس Unix",
    listen_tcp: "شبكة TCP",
    severity_other: "أخرى",
    severity: "الخطورة",
    severity_notice: "ملاحظة",
    severity_warning: "تحذير",
    severity_error: "خطأ",
    log_type: "نوع السجل",
    pool_choice: "اختيار التجمع",
    max_child: "أقصى عدد أبناء",
    max_req: "أقصى طلبات",
    min_spare: "الحد الأدنى للانتظار",
    max_spare: "الحد الأقصى",
    max_backlog: "الحد الأقصى للتراكم",
    configured_detected: "مُهيأة + مكتشفة",
    total_processes: "إجمالي العمليات",
    state_r: "الحالة R",
    state_s: "الحالة S",
    pending_connections: "اتصالات معلّقة",
    sum_rss: "مجموع RSS للعمال",
    global_directives: "التوجيهات العامة",
    footer_tagline: "يقرأ جدول العمليات المحلي، دون أي وصول إلى الشبكة.",
    requests: "طلبات/ث",
    total_requests: "إجمالي الطلبات",
    copy: "نسخ",
    copied: "تم النسخ",
    min: "أدنى",
    max: "أقصى",
    avg: "متوسط",
    axis_x: "الوقت",
    axis_y: "القيمة",
    collection_error: "خطأ في الجمع:",
    cannot_reach: "تعذّر الوصول إلى fpm-monitor:",
    refresh: "التحديث",
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
    invalid_port: "puerto no válido: {} (se espera 1-65535)",
    web_serving: "interfaz web escuchando en http://127.0.0.1:{} (Ctrl-C para salir)",
    web_error: "no se puede iniciar la interfaz web: {}",
    skip_content: "Saltar al contenido",
    menu: "Menú",
    language: "Idioma",
    web_dashboard: "panel web",
    theme_dark: "Modo oscuro",
    theme_light: "Modo claro",
    connecting: "conectando…",
    disconnected: "desconectado",
    mock_data: "datos de demostración",
    live: "en vivo",
    updated: "actualizado",
    pool: "Pool",
    type_label: "Tipo",
    listen_unix: "Socket Unix",
    listen_tcp: "Red TCP",
    severity_other: "Otro",
    severity: "Gravedad",
    severity_notice: "Aviso",
    severity_warning: "Advertencia",
    severity_error: "Error",
    log_type: "Tipo de log",
    pool_choice: "Selección de pool",
    max_child: "Máx. hijos",
    max_req: "Máx. peticiones",
    min_spare: "Mín. inactivo",
    max_spare: "Máx. inactivo",
    max_backlog: "Máx. cola",
    configured_detected: "configurados + detectados",
    total_processes: "procesos totales",
    state_r: "estado R",
    state_s: "estado S",
    pending_connections: "conexiones pendientes",
    sum_rss: "suma de RSS de workers",
    global_directives: "Directivas globales",
    footer_tagline: "lee la tabla de procesos local, sin acceso a la red.",
    requests: "Solicitudes/s",
    total_requests: "Solicitudes totales",
    copy: "Copiar",
    copied: "¡Copiado!",
    min: "mín",
    max: "máx",
    avg: "med",
    axis_x: "tiempo",
    axis_y: "valor",
    collection_error: "Error de recopilación:",
    cannot_reach: "No se puede contactar con fpm-monitor:",
    refresh: "actualización",
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
    invalid_port: "porta non valida: {} (previsto 1-65535)",
    web_serving: "interfaccia web in ascolto su http://127.0.0.1:{} (Ctrl-C per uscire)",
    web_error: "impossibile avviare l'interfaccia web: {}",
    skip_content: "Vai al contenuto",
    menu: "Menu",
    language: "Lingua",
    web_dashboard: "dashboard web",
    theme_dark: "Modalità scura",
    theme_light: "Modalità chiara",
    connecting: "connessione…",
    disconnected: "disconnesso",
    mock_data: "dati dimostrativi",
    live: "in diretta",
    updated: "aggiornato",
    pool: "Pool",
    type_label: "Tipo",
    listen_unix: "Socket Unix",
    listen_tcp: "Rete TCP",
    severity_other: "Altro",
    severity: "Gravità",
    severity_notice: "Avviso",
    severity_warning: "Avvertenza",
    severity_error: "Errore",
    log_type: "Tipo di log",
    pool_choice: "Scelta del pool",
    max_child: "Max figli",
    max_req: "Max richieste",
    min_spare: "Min. inattivo",
    max_spare: "Max. inattivo",
    max_backlog: "Max. coda",
    configured_detected: "configurati + rilevati",
    total_processes: "processi totali",
    state_r: "stato R",
    state_s: "stato S",
    pending_connections: "connessioni in attesa",
    sum_rss: "somma RSS dei worker",
    global_directives: "Direttive globali",
    footer_tagline: "legge la tabella dei processi locale, nessun accesso alla rete.",
    requests: "Richieste/s",
    total_requests: "Richieste totali",
    copy: "Copia",
    copied: "Copiato!",
    min: "min",
    max: "max",
    avg: "media",
    axis_x: "tempo",
    axis_y: "valore",
    collection_error: "Errore di raccolta:",
    cannot_reach: "Impossibile raggiungere fpm-monitor:",
    refresh: "aggiornamento",
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
    invalid_port: "無効なポート: {}（1〜65535 の範囲）",
    web_serving: "Web UI は http://127.0.0.1:{} で待機中（Ctrl-C で終了）",
    web_error: "Web UI を起動できません：{}",
    skip_content: "コンテンツへスキップ",
    menu: "メニュー",
    language: "言語",
    web_dashboard: "Web ダッシュボード",
    theme_dark: "ダークモード",
    theme_light: "ライトモード",
    connecting: "接続中…",
    disconnected: "切断されました",
    mock_data: "モックデータ",
    live: "ライブ",
    updated: "更新",
    pool: "プール",
    type_label: "種類",
    listen_unix: "Unix ソケット",
    listen_tcp: "TCP ネットワーク",
    severity_other: "その他",
    severity: "重要度",
    severity_notice: "通知",
    severity_warning: "警告",
    severity_error: "エラー",
    log_type: "ログの種類",
    pool_choice: "プールの選択",
    max_child: "最大子プロセス",
    max_req: "最大リクエスト",
    min_spare: "最小アイドル",
    max_spare: "最大アイドル",
    max_backlog: "最大バックログ",
    configured_detected: "設定済み + 検出済み",
    total_processes: "プロセス合計",
    state_r: "状態 R",
    state_s: "状態 S",
    pending_connections: "保留中の接続",
    sum_rss: "ワーカー RSS 合計",
    global_directives: "グローバルディレクティブ",
    footer_tagline: "ローカルのプロセステーブルを読み取ります。ネットワークアクセスはありません。",
    requests: "リクエスト/秒",
    total_requests: "リクエスト総数",
    copy: "コピー",
    copied: "コピーしました",
    min: "最小",
    max: "最大",
    avg: "平均",
    axis_x: "時間",
    axis_y: "値",
    collection_error: "収集エラー:",
    cannot_reach: "fpm-monitor に接続できません:",
    refresh: "更新",
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
    invalid_port: "ungültiger Port: {} (erwartet 1-65535)",
    web_serving: "Weboberfläche lauscht auf http://127.0.0.1:{} (Ctrl-C zum Beenden)",
    web_error: "Weboberfläche konnte nicht gestartet werden: {}",
    skip_content: "Zum Inhalt springen",
    menu: "Menü",
    language: "Sprache",
    web_dashboard: "Web-Dashboard",
    theme_dark: "Dunkelmodus",
    theme_light: "Hellmodus",
    connecting: "Verbinden…",
    disconnected: "getrennt",
    mock_data: "Demodaten",
    live: "live",
    updated: "aktualisiert",
    pool: "Pool",
    type_label: "Typ",
    listen_unix: "Unix-Socket",
    listen_tcp: "TCP-Netzwerk",
    severity_other: "Sonstige",
    severity: "Schweregrad",
    severity_notice: "Hinweis",
    severity_warning: "Warnung",
    severity_error: "Fehler",
    log_type: "Logtyp",
    pool_choice: "Pool-Auswahl",
    max_child: "Max. Kinder",
    max_req: "Max. Anfragen",
    min_spare: "Min. Leerlauf",
    max_spare: "Max. Leerlauf",
    max_backlog: "Max. Rückstand",
    configured_detected: "konfiguriert + erkannt",
    total_processes: "Prozesse insgesamt",
    state_r: "Status R",
    state_s: "Status S",
    pending_connections: "ausstehende Verbindungen",
    sum_rss: "Summe der Worker-RSS",
    global_directives: "Globale Direktiven",
    footer_tagline: "liest die lokale Prozesstabelle, kein Netzwerkzugriff.",
    requests: "Anfragen/s",
    total_requests: "Anfragen insgesamt",
    copy: "Kopieren",
    copied: "Kopiert!",
    min: "min",
    max: "max",
    avg: "Ø",
    axis_x: "Zeit",
    axis_y: "Wert",
    collection_error: "Erfassungsfehler:",
    cannot_reach: "fpm-monitor ist nicht erreichbar:",
    refresh: "Aktualisierung",
};

/// Localised `--help` text.
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
      --lang <LANG>      UI language: en (US), fr, zh, ar, es, it, ja, de,
                         ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
      --mock             Show demo data (local test)
      --watch            Start the web UI instead of the CLI view
      --port <PORT>      Web UI port (default 8471)
  -h, --help             Show this help
  -V, --version          Show the version

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
      --lang <LANG>      Langue de l'interface : en (US), fr, zh, ar, es, it, ja, de,
                         ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
      --mock             Affiche des données de démonstration (test local)
      --watch            Démarre l'interface web au lieu de la vue CLI
      --port <PORT>      Port de l'interface web (défaut 8471)
  -h, --help             Affiche cette aide
  -V, --version          Show the version

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
      --lang <语言>      界面语言：en (US)、fr、zh、ar、es、it、ja、de、
                         ru、hi、id、ko、no、sv、fi、da、el、tr、pl、uk、th、vi
      --mock             显示演示数据（本地测试）
      --watch            启动 Web 界面，而不是 CLI 视图
      --port <PORT>      Web 界面端口（默认 8471）
  -h, --help             显示此帮助
  -V, --version          Show the version

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
      --lang <اللغة>     لغة الواجهة: en (US)، fr، zh، ar، es، it، ja، de،
                         ru، hi، id، ko، no، sv، fi، da، el، tr، pl، uk، th، vi
      --mock             عرض بيانات تجريبية (اختبار محلي)
      --watch            تشغيل واجهة الويب بدلاً من عرض CLI
      --port <PORT>      منفذ واجهة الويب (الافتراضي 8471)
  -h, --help             عرض هذه المساعدة
  -V, --version          Show the version

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
      --lang <LANG>      Idioma: en (US), fr, zh, ar, es, it, ja, de,
                         ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
      --mock             Mostrar datos de demostración (prueba local)
      --watch            Inicia la interfaz web en lugar de la vista CLI
      --port <PORT>      Puerto de la interfaz web (por defecto 8471)
  -h, --help             Muestra esta ayuda
  -V, --version          Show the version

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
      --lang <LINGUA>      Lingua interfaccia: en (US), fr, zh, ar, es, it, ja, de,
                         ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
      --mock               Mostra dati dimostrativi (test locale)
      --watch            Avvia l'interfaccia web invece della vista CLI
      --port <PORT>      Porta dell'interfaccia web (predefinita 8471)
  -h, --help               Mostra questo aiuto
  -V, --version          Show the version

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
      --lang <言語>      UI 言語: en (US)、fr、zh、ar、es、it、ja、de、
                         ru、hi、id、ko、no、sv、fi、da、el、tr、pl、uk、th、vi
      --mock            デモデータを表示（ローカルテスト）
      --watch            CLI ビューの代わりに Web UI を起動します
      --port <PORT>      Web UI のポート（デフォルト 8471）
  -h, --help            このヘルプを表示
  -V, --version          Show the version

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
      --lang <SPRACHE>   UI-Sprache: en (US), fr, zh, ar, es, it, ja, de,
                         ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
      --mock             Demodaten anzeigen (lokaler Test)
      --watch            Startet die Weboberfläche statt der CLI-Ansicht
      --port <PORT>      Port der Weboberfläche (Standard 8471)
  -h, --help             Diese Hilfe anzeigen
  -V, --version          Show the version

Die Erkennung ist automatisch (übliche Orte + include-Direktive).
"
        }
        Lang::Ru => ru::USAGE,
        Lang::Hi => hi::USAGE,
        Lang::Id => id::USAGE,
        Lang::Ko => ko::USAGE,
        Lang::No => no::USAGE,
        Lang::Sv => sv::USAGE,
        Lang::Fi => fi::USAGE,
        Lang::Da => da::USAGE,
        Lang::El => el::USAGE,
        Lang::Tr => tr::USAGE,
        Lang::Pl => pl::USAGE,
        Lang::Uk => uk::USAGE,
        Lang::Th => th::USAGE,
        Lang::Vi => vi::USAGE,
    }
}
