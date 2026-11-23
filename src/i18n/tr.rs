use super::Labels;

pub(super) const LABELS: &Labels = &Labels {
    tab_monitoring: "İzleme",
    tab_graphs: "Grafikler",
    tab_logs: "Günlükler",
    tab_configs: "Yapılandırmalar",
    pools: "Havuzlar",
    workers: "Süreçler",
    running: "Etkin",
    idle: "Boşta",
    backlog: "Birikim",
    mem: "Bellek",
    master: "Master",
    help: "q çıkış · Tab sonraki sekme · 1-4 sekme · ← → alt sekme · ↑ ↓ izleme",
    no_pool: "Havuz bulunamadı.",
    last_measurements: "son {} ölçüm",
    g_running: "etkin",
    g_workers: "süreç",
    g_idle: "boşta",
    g_backlog: "birikim",
    log_php: "PHP günlüğü",
    slow_queries: "Yavaş sorgular",
    access: "Erişim",
    no_entries: "(kayıt yok)",
    older_lines: "+{} eski satır",
    more_workers: "+{} daha",
    pool_configs: "Havuz yapılandırmaları",
    no_config: "Yapılandırma bulunamadı.",
    unknown_pool: "Bilinmeyen havuz.",
    no_directives: "(yönerge okunmadı)",
    master_pid: "Master PID",
    legend: "Açıklama:",
    leg_red: "kırmızı = doygunluk/birikim",
    leg_green: "yeşil = sağlıklı/boşta",
    leg_white: "beyaz = nötr/ondemand",
    leg_blue: "mavi = etkin/static",
    read_proc: "/proc okunamadı (Linux gerekli): {}",
    unknown_option: "bilinmeyen seçenek: {}",
    run_help: "--help ile çalıştırın",
    invalid_port: "geçersiz port: {} (1-65535 bekleniyor)",
    web_serving: "web arayüzü http://127.0.0.1:{} adresinde dinliyor (çıkmak için Ctrl-C)",
    web_error: "web arayüzü başlatılamadı: {}",
    skip_content: "İçeriğe geç",
    menu: "Menü",
    language: "Dil",
    web_dashboard: "web panosu",
    theme_dark: "Koyu mod",
    theme_light: "Açık mod",
    connecting: "bağlanıyor…",
    disconnected: "bağlantı kesildi",
    mock_data: "örnek veriler",
    live: "canlı",
    updated: "güncellendi",
    pool: "Havuz",
    type_label: "Tür",
    max_child: "Maks. alt",
    max_req: "Maks. istek",
    min_spare: "min spare",
    max_spare: "max spare",
    max_backlog: "max backlog",
    configured_detected: "yapılandırılmış + algılanan",
    total_processes: "toplam süreç",
    state_r: "durum R",
    state_s: "durum S",
    pending_connections: "bekleyen bağlantılar",
    sum_rss: "worker RSS toplamı",
    global_directives: "Genel yönergeler",
    footer_tagline: "yerel süreç tablosunu okur, ağ erişimi yoktur.",
    requests: "İstek/s",
    total_requests: "Toplam istek",
    copy: "Kopyala",
    copied: "Kopyalandı",
    min: "min",
    max: "maks",
    avg: "ort",
    axis_x: "zaman",
    axis_y: "değer",
    collection_error: "Toplama hatası:",
    cannot_reach: "fpm-monitor'a ulaşılamıyor:",
    refresh: "yenileme",
};

pub(super) const USAGE: &str = "\
fpm-monitor — PHP-FPM havuz izleyicisi (fpm-monitor.c'nin Rust portu)

Kullanım: fpm-monitor [SEÇENEKLER]

Seçenekler:
  -c, --config <YOL>     analiz edilecek php-fpm.conf dosyası veya havuz dizini
  -v, --verbose          her sürecin ayrıntılarını göster (pid, durum, RSS)
      --color            renkleri zorla
      --no-color         renkleri devre dışı bırak
  -t, --tui              etkileşimli pano (Ember gibi) — sekmeler
                         izleme, grafikler, günlükler ve yapılandırmalar
                         (← → ile alt sekmeler). Çıkış: q / Ctrl-C
      --interval <SN>    TUI yenileme aralığı (varsayılan 1)
      --lang <DİL>       arayüz dili: en (US), fr, zh, ar, es, it, ja, de,
                         ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
      --mock             demo verilerini göster (yerel test)
      --watch            CLI görünümü yerine web arayüzünü başlat
      --port <PORT>      Web arayüzü portu (varsayılan 8471)
  -h, --help             bu yardımı göster

Algılama otomatiktir (olağan konumlar + include yönergesi).
";
