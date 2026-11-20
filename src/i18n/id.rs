use super::Labels;

pub(super) const LABELS: &Labels = &Labels {
    tab_monitoring: "Pemantauan",
    tab_graphs: "Grafik",
    tab_logs: "Log",
    tab_configs: "Konfigurasi",
    pools: "Pool",
    workers: "Worker",
    running: "Berjalan",
    idle: "Menganggur",
    backlog: "Antrean",
    mem: "Mem",
    master: "Master",
    help: "q keluar · Tab tab berikutnya · 1-4 tab · ← → subtab · ↑ ↓ pemantauan",
    no_pool: "Tidak ada pool yang terdeteksi.",
    last_measurements: "{} pengukuran terakhir",
    g_running: "berjalan",
    g_workers: "worker",
    g_idle: "menganggur",
    g_backlog: "antrean",
    log_php: "Log PHP",
    slow_queries: "Kueri lambat",
    access: "Akses",
    no_entries: "(tidak ada entri)",
    older_lines: "+{} baris lebih lama",
    more_workers: "+{} lagi",
    pool_configs: "Konfigurasi pool",
    no_config: "Tidak ada konfigurasi yang ditemukan.",
    unknown_pool: "Pool tidak dikenal.",
    no_directives: "(tidak ada direktif yang terbaca)",
    master_pid: "PID master",
    legend: "Legenda:",
    leg_red: "merah = saturasi/antrean",
    leg_green: "hijau = sehat/menganggur",
    leg_white: "putih = netral/ondemand",
    leg_blue: "biru = aktif/static",
    read_proc: "tidak dapat membaca /proc (memerlukan Linux): {}",
    unknown_option: "opsi tidak dikenal: {}",
    run_help: "jalankan dengan --help",
    invalid_port: "port tidak valid: {} (diharapkan 1-65535)",
    web_serving: "antarmuka web mendengarkan di http://127.0.0.1:{} (Ctrl-C untuk keluar)",
    web_error: "tidak dapat memulai antarmuka web: {}",
};

pub(super) const USAGE: &str = "\
fpm-monitor — pemantau pool PHP-FPM (port Rust dari fpm-monitor.c)

Penggunaan: fpm-monitor [OPSI]

Opsi:
  -c, --config <PATH>    file php-fpm.conf atau direktori pool yang akan dianalisis
  -v, --verbose          Tampilkan detail setiap worker (pid, status, RSS)
      --color            Paksa warna
      --no-color         Nonaktifkan warna
  -t, --tui              Dashboard interaktif (seperti Ember) — tab
                         pemantauan, grafik, log, dan konfigurasi (sub-tab
                         dapat dinavigasi dengan ← →). Keluar: q / Ctrl-C
      --interval <SEC>   Interval penyegaran TUI (default 1)
      --lang <LANG>      Bahasa antarmuka: en (US), fr, zh, ar, es, it, ja, de,
                         ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
      --mock             Tampilkan data demo (uji lokal)
      --watch            Mulai antarmuka web alih-alih tampilan CLI
      --port <PORT>      Port antarmuka web (default 8471)
  -h, --help             Tampilkan bantuan ini

Deteksi bersifat otomatis (lokasi umum + direktif include).
";
