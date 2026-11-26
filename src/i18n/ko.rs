use super::Labels;

pub(super) const LABELS: &Labels = &Labels {
    tab_monitoring: "모니터링",
    tab_graphs: "그래프",
    tab_logs: "로그",
    tab_configs: "설정",
    pools: "풀",
    workers: "워커",
    running: "실행 중",
    idle: "유휴",
    backlog: "백로그",
    mem: "메모리",
    master: "마스터",
    help: "q 종료 · Tab 다음 탭 · 1-4 탭 · ← → 하위 탭 · ↑ ↓ 모니터링",
    no_pool: "풀이 감지되지 않았습니다.",
    last_measurements: "최근 {}회 측정",
    g_running: "실행 중",
    g_workers: "워커",
    g_idle: "유휴",
    g_backlog: "백로그",
    log_php: "PHP 로그",
    slow_queries: "느린 쿼리",
    access: "액세스",
    no_entries: "(항목 없음)",
    older_lines: "이전 줄 +{}개",
    more_workers: "+{}개 더",
    pool_configs: "풀 설정",
    no_config: "설정을 찾을 수 없습니다.",
    unknown_pool: "알 수 없는 풀입니다.",
    no_directives: "(읽은 지시문 없음)",
    master_pid: "마스터 PID",
    legend: "범례:",
    leg_red: "빨강 = 포화/백로그",
    leg_green: "초록 = 정상/유휴",
    leg_white: "흰색 = 중립/ondemand",
    leg_blue: "파랑 = 활성/static",
    read_proc: "/proc을(를) 읽을 수 없습니다 (Linux 필요): {}",
    unknown_option: "알 수 없는 옵션: {}",
    run_help: "--help로 실행하세요",
    invalid_port: "잘못된 포트: {} (1-65535 기대)",
    web_serving: "웹 UI가 http://127.0.0.1:{} 에서 대기 중입니다 (Ctrl-C로 종료)",
    web_error: "웹 UI를 시작할 수 없습니다: {}",
    skip_content: "콘텐츠로 건너뛰기",
    menu: "메뉴",
    language: "언어",
    web_dashboard: "웹 대시보드",
    theme_dark: "어두운 모드",
    theme_light: "밝은 모드",
    connecting: "연결 중…",
    disconnected: "연결 끊김",
    mock_data: "모의 데이터",
    live: "실시간",
    updated: "업데이트",
    pool: "풀",
    type_label: "유형",
    listen_unix: "Unix 소켓",
    listen_tcp: "TCP 네트워크",
    severity_other: "기타",
    severity: "심각도",
    severity_notice: "알림",
    severity_warning: "경고",
    severity_error: "오류",
    log_type: "로그 유형",
    pool_choice: "풀 선택",
    max_child: "최대 자식",
    max_req: "최대 요청",
    min_spare: "min spare",
    max_spare: "max spare",
    max_backlog: "max backlog",
    configured_detected: "구성됨 + 감지됨",
    total_processes: "전체 프로세스",
    state_r: "상태 R",
    state_s: "상태 S",
    pending_connections: "대기 중인 연결",
    sum_rss: "워커 RSS 합계",
    global_directives: "전역 지시문",
    footer_tagline: "로컬 프로세스 테이블을 읽으며 네트워크에 접근하지 않습니다.",
    requests: "요청/초",
    total_requests: "총 요청 수",
    copy: "복사",
    copied: "복사됨",
    min: "최소",
    max: "최대",
    avg: "평균",
    axis_x: "시간",
    axis_y: "값",
    collection_error: "수집 오류:",
    cannot_reach: "fpm-monitor에 연결할 수 없습니다:",
    refresh: "새로 고침",
};

pub(super) const USAGE: &str = "\
fpm-monitor — PHP-FPM 풀 모니터 (fpm-monitor.c의 Rust 포팅)

사용법: fpm-monitor [옵션]

옵션:
  -c, --config <경로>    분석할 php-fpm.conf 파일 또는 풀 디렉터리
  -v, --verbose          각 워커의 상세 정보 표시 (pid, 상태, RSS)
      --color            색상 강제 적용
      --no-color         색상 비활성화
  -t, --tui              대화형 대시보드 (Ember 스타일) — 탭
                         모니터링, 그래프, 로그, 설정 (하위 탭은
                         ← →로 이동). 종료: q / Ctrl-C
      --interval <초>    TUI 새로 고침 간격 (기본값 1)
      --lang <언어>      UI 언어: en (US), fr, zh, ar, es, it, ja, de,
                         ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
      --mock             데모 데이터 표시 (로컬 테스트)
      --watch            CLI 보기 대신 웹 UI를 시작합니다
      --port <PORT>      웹 UI 포트 (기본값 8471)
  -h, --help             이 도움말 표시
  -V, --version          Show the version

감지는 자동입니다 (일반적인 위치 + include 지시문).
";
