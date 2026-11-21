use super::Labels;

pub(super) const LABELS: &Labels = &Labels {
    tab_monitoring: "การตรวจสอบ",
    tab_graphs: "กราฟ",
    tab_logs: "บันทึก",
    tab_configs: "การกำหนดค่า",
    pools: "พูล",
    workers: "เวิร์กเกอร์",
    running: "กำลังทำงาน",
    idle: "ว่าง",
    backlog: "คิวงาน",
    mem: "หน่วยความจำ",
    master: "Master",
    help: "q ออก · Tab แท็บถัดไป · 1-4 แท็บ · ← → แท็บย่อย · ↑ ↓ การตรวจสอบ",
    no_pool: "ไม่พบพูล",
    last_measurements: "การวัด {} ครั้งล่าสุด",
    g_running: "กำลังทำงาน",
    g_workers: "เวิร์กเกอร์",
    g_idle: "ว่าง",
    g_backlog: "คิวงาน",
    log_php: "บันทึก PHP",
    slow_queries: "คำสั่งที่ช้า",
    access: "การเข้าถึง",
    no_entries: "(ไม่มีรายการ)",
    older_lines: "+{} บรรทัดเก่ากว่า",
    more_workers: "+{} รายการ",
    pool_configs: "การกำหนดค่าพูล",
    no_config: "ไม่พบการกำหนดค่า",
    unknown_pool: "ไม่รู้จักพูล",
    no_directives: "(ไม่ได้อ่านคำสั่ง)",
    master_pid: "PID ของ Master",
    legend: "คำอธิบาย:",
    leg_red: "แดง = อิ่มตัว/คิวงาน",
    leg_green: "เขียว = ปกติดี/ว่าง",
    leg_white: "ขาว = เป็นกลาง/ondemand",
    leg_blue: "น้ำเงิน = กำลังทำงาน/static",
    read_proc: "ไม่สามารถอ่าน /proc ได้ (ต้องใช้ Linux): {}",
    unknown_option: "ตัวเลือกที่ไม่รู้จัก: {}",
    run_help: "เรียกใช้ด้วย --help",
    invalid_port: "พอร์ตไม่ถูกต้อง: {} (ต้องเป็น 1-65535)",
    web_serving: "เว็บ UI กำลังฟังที่ http://127.0.0.1:{} (Ctrl-C เพื่อออก)",
    web_error: "ไม่สามารถเริ่มเว็บ UI: {}",
};

pub(super) const USAGE: &str = "\
fpm-monitor — โปรแกรมตรวจสอบพูล PHP-FPM (พอร์ตของ fpm-monitor.c ในภาษา Rust)

การใช้งาน: fpm-monitor [ตัวเลือก]

ตัวเลือก:
  -c, --config <เส้นทาง>  ไฟล์ php-fpm.conf หรือไดเรกทอรีพูลที่ต้องการวิเคราะห์
  -v, --verbose           แสดงรายละเอียดของแต่ละเวิร์กเกอร์ (pid, สถานะ, RSS)
      --color             บังคับใช้สี
      --no-color          ปิดการใช้สี
  -t, --tui               แดชบอร์ดแบบโต้ตอบ (เหมือน Ember) — แท็บ
                          การตรวจสอบ กราฟ บันทึก และการกำหนดค่า
                          (แท็บย่อยด้วย ← →) ออก: q / Ctrl-C
      --interval <วินาที>  ช่วงเวลารีเฟรช TUI (ค่าเริ่มต้น 1)
      --lang <ภาษา>       ภาษาของอินเทอร์เฟซ: en (US), fr, zh, ar, es, it, ja, de,
                          ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
      --mock              แสดงข้อมูลตัวอย่าง (ทดสอบในเครื่อง)
      --watch            เริ่มเว็บ UI แทนมุมมอง CLI
      --port <PORT>      พอร์ตของเว็บ UI (ค่าเริ่มต้น 8471)
  -h, --help              แสดงความช่วยเหลือนี้

การตรวจจับเป็นไปโดยอัตโนมัติ (ตำแหน่งปกติ + คำสั่ง include)
";
