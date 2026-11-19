use super::Labels;

pub(super) const LABELS: &Labels = &Labels {
    tab_monitoring: "निगरानी",
    tab_graphs: "ग्राफ़",
    tab_logs: "लॉग",
    tab_configs: "कॉन्फ़िगरेशन",
    pools: "पूल",
    workers: "वर्कर",
    running: "चालू",
    idle: "निष्क्रिय",
    backlog: "बैकलॉग",
    mem: "मेम",
    master: "मास्टर",
    help: "q बाहर · Tab अगला टैब · 1-4 टैब · ← → उप-टैब · ↑ ↓ निगरानी",
    no_pool: "कोई पूल नहीं मिला।",
    last_measurements: "पिछले {} माप",
    g_running: "चालू",
    g_workers: "वर्कर",
    g_idle: "निष्क्रिय",
    g_backlog: "बैकलॉग",
    log_php: "PHP लॉग",
    slow_queries: "धीमी क्वेरी",
    access: "एक्सेस",
    no_entries: "(कोई प्रविष्टि नहीं)",
    older_lines: "+{} पुरानी पंक्तियाँ",
    more_workers: "+{} और",
    pool_configs: "पूल कॉन्फ़िगरेशन",
    no_config: "कोई कॉन्फ़िगरेशन नहीं मिला।",
    unknown_pool: "अज्ञात पूल।",
    no_directives: "(कोई निर्देश नहीं पढ़ा गया)",
    master_pid: "मास्टर PID",
    legend: "संकेत:",
    leg_red: "लाल = संतृप्ति/बैकलॉग",
    leg_green: "हरा = स्वस्थ/निष्क्रिय",
    leg_white: "सफ़ेद = तटस्थ/ondemand",
    leg_blue: "नीला = सक्रिय/static",
    read_proc: "/proc पढ़ने में असमर्थ (Linux आवश्यक): {}",
    unknown_option: "अज्ञात विकल्प: {}",
    run_help: "--help के साथ चलाएँ",
};

pub(super) const USAGE: &str = "\
fpm-monitor — PHP-FPM पूल मॉनिटर (fpm-monitor.c का Rust पोर्ट)

उपयोग: fpm-monitor [विकल्प]

विकल्प:
  -c, --config <पथ>    विश्लेषण के लिए php-fpm.conf फ़ाइल या पूल निर्देशिका
  -v, --verbose          प्रत्येक वर्कर का विवरण दिखाएँ (pid, स्थिति, RSS)
      --color            रंग बाध्य करें
      --no-color         रंग अक्षम करें
  -t, --tui              इंटरैक्टिव डैशबोर्ड (Ember जैसा) — टैब
                         निगरानी, ग्राफ़, लॉग और कॉन्फ़िगरेशन (उप-टैब
                         ← → से नेविगेट करें)। बाहर: q / Ctrl-C
      --interval <सेकंड>   TUI रीफ़्रेश अंतराल (डिफ़ॉल्ट 1)
      --lang <भाषा>      UI भाषा: en (US), fr, zh, ar, es, it, ja, de,
                         ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
      --mock             डेमो डेटा दिखाएँ (स्थानीय परीक्षण)
  -h, --help             यह सहायता दिखाएँ

पहचान स्वचालित है (सामान्य स्थान + include निर्देश)।
";
