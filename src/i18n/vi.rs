use super::Labels;

pub(super) const LABELS: &Labels = &Labels {
    tab_monitoring: "Giám sát",
    tab_graphs: "Biểu đồ",
    tab_logs: "Nhật ký",
    tab_configs: "Cấu hình",
    pools: "Pool",
    workers: "Tiến trình",
    running: "Đang chạy",
    idle: "Rảnh rỗi",
    backlog: "Tồn đọng",
    mem: "Mem",
    master: "Master",
    help: "q thoát · Tab tab tiếp theo · 1-4 tab · ← → tab con · ↑ ↓ giám sát",
    no_pool: "Không phát hiện pool nào.",
    last_measurements: "{} lần đo gần nhất",
    g_running: "đang chạy",
    g_workers: "tiến trình",
    g_idle: "rảnh rỗi",
    g_backlog: "tồn đọng",
    log_php: "Nhật ký PHP",
    slow_queries: "Truy vấn chậm",
    access: "Truy cập",
    no_entries: "(không có mục nào)",
    older_lines: "+{} dòng cũ hơn",
    more_workers: "+{} nữa",
    pool_configs: "Cấu hình pool",
    no_config: "Không tìm thấy cấu hình.",
    unknown_pool: "Pool không xác định.",
    no_directives: "(không có chỉ thị nào được đọc)",
    master_pid: "PID master",
    legend: "Chú giải:",
    leg_red: "đỏ = bão hòa/tồn đọng",
    leg_green: "xanh lá = khỏe mạnh/rảnh rỗi",
    leg_white: "trắng = trung tính/ondemand",
    leg_blue: "xanh dương = hoạt động/static",
    read_proc: "không thể đọc /proc (cần Linux): {}",
    unknown_option: "tùy chọn không xác định: {}",
    run_help: "chạy với --help",
    invalid_port: "cổng không hợp lệ: {} (mong đợi 1-65535)",
    web_serving: "giao diện web đang lắng nghe tại http://127.0.0.1:{} (Ctrl-C để thoát)",
    web_error: "không thể khởi động giao diện web: {}",
};

pub(super) const USAGE: &str = "\
fpm-monitor — trình giám sát pool PHP-FPM (bản chuyển sang Rust của fpm-monitor.c)

Cách dùng: fpm-monitor [TÙY CHỌN]

Tùy chọn:
  -c, --config <PATH>    tệp php-fpm.conf hoặc thư mục pool cần phân tích
  -v, --verbose          Hiển thị chi tiết từng worker (pid, trạng thái, RSS)
      --color            Buộc dùng màu
      --no-color         Tắt màu
  -t, --tui              Bảng điều khiển tương tác (như Ember) — các tab
                         giám sát, biểu đồ, nhật ký và cấu hình (tab con
                         điều hướng bằng ← →). Thoát: q / Ctrl-C
      --interval <SEC>   Chu kỳ làm mới TUI (mặc định 1)
      --lang <LANG>      Ngôn ngữ giao diện: en (US), fr, zh, ar, es, it, ja, de,
                         ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
      --mock             Hiển thị dữ liệu demo (kiểm thử cục bộ)
      --watch            Khởi động giao diện web thay vì chế độ CLI
      --port <PORT>      Cổng của giao diện web (mặc định 8471)
  -h, --help             Hiển thị trợ giúp này

Việc phát hiện diễn ra tự động (vị trí thông thường + chỉ thị include).
";
