use std::io;

#[cfg(target_os = "linux")]
use std::fs;

#[derive(Debug, Clone)]
pub struct Worker {
    pub pid: i32,
    pub state: char,
    pub rss_kb: i64,
}

#[derive(Debug, Clone)]
pub struct PoolScan {
    pub name: String,
    pub workers: Vec<Worker>,
}

pub struct ScanResult {
    pub pools: Vec<PoolScan>,
    pub masters: Vec<i32>,
}

/// Scans /proc for PHP-FPM processes.
/// The master is excluded from the worker count but recorded.
#[cfg(target_os = "linux")]
pub fn scan() -> io::Result<ScanResult> {
    let mut pools: Vec<PoolScan> = Vec::new();
    let mut masters: Vec<i32> = Vec::new();

    for e in fs::read_dir("/proc")? {
        let e = e?;
        let fname = e.file_name();
        let Ok(pid) = fname.to_string_lossy().parse::<i32>() else {
            continue;
        };
        let Some(cmd) = read_first_arg(pid) else {
            continue;
        };
        if !cmd.contains("php-fpm:") {
            continue;
        }
        if cmd.contains("master process") {
            masters.push(pid);
            continue;
        }
        let Some(name) = pool_name(&cmd) else {
            continue;
        };
        let worker = Worker {
            pid,
            state: get_state(pid),
            rss_kb: get_rss_kb(pid),
        };
        match pools.iter_mut().find(|p| p.name == name) {
            Some(p) => p.workers.push(worker),
            None => pools.push(PoolScan {
                name,
                workers: vec![worker],
            }),
        }
    }

    pools.sort_by(|a, b| a.name.cmp(&b.name));
    masters.sort();
    Ok(ScanResult { pools, masters })
}

/// Depth of a pool's TCP accept queue (backlog),
/// read directly from /proc/net/tcp and /proc/net/tcp6: for a socket in
/// LISTEN state, rx_queue contains the number of pending connections.
/// Returns None for a Unix socket (unreadable) or a port that cannot be found.
#[cfg(target_os = "linux")]
pub fn accept_backlog(listen: &str) -> Option<u32> {
    if listen.starts_with('/') {
        return None;
    }
    let (host, port) = split_listen(listen);
    let port_hex = format!("{port:04X}");
    let want_ip = if host.is_empty() || host == "0.0.0.0" || host.contains('[') {
        None
    } else {
        Some(ip_to_hex_le(&host))
    };
    for path in ["/proc/net/tcp", "/proc/net/tcp6"] {
        let Ok(s) = fs::read_to_string(path) else {
            continue;
        };
        for line in s.lines().skip(1) {
            let f: Vec<&str> = line.split_whitespace().collect();
            if f.len() < 5 || f[3] != "0A" {
                continue;
            }
            let Some((ip, prt)) = f[1].split_once(':') else {
                continue;
            };
            if prt != port_hex {
                continue;
            }
            if let Some(want) = &want_ip {
                if ip != want.as_str() {
                    continue;
                }
            }
            let rx = f[4]
                .rsplit_once(':')
                .and_then(|(_, r)| u32::from_str_radix(r, 16).ok())
                .unwrap_or(0);
            return Some(rx);
        }
    }
    None
}

/// Splits "ip:port" or "port" into (host, port).
/// Without a host, php-fpm listens on 0.0.0.0 (INADDR_ANY): return "".
#[cfg(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly"
))]
fn split_listen(listen: &str) -> (String, u16) {
    let l = listen.trim();
    match l.rsplit_once(':') {
        Some((host, port)) => {
            let host = if host.is_empty() { "" } else { host };
            let port = port.parse().unwrap_or(9000);
            (host.to_string(), port)
        }
        None => ("".to_string(), l.parse().unwrap_or(9000)),
    }
}

/// IPv4 address to the little-endian hexadecimal notation of /proc/net/tcp.
/// "127.0.0.1" → "0100007F".
#[cfg(target_os = "linux")]
fn ip_to_hex_le(host: &str) -> String {
    let mut out = String::new();
    for part in host.split('.') {
        let n: u8 = part.parse().unwrap_or(0);
        out.insert_str(0, &format!("{n:02X}"));
    }
    out
}

/// Reads the first argument of /proc/[pid]/cmdline.
/// PHP-FPM puts everything in argv[0]: "php-fpm: pool www".
#[cfg(target_os = "linux")]
fn read_first_arg(pid: i32) -> Option<String> {
    let bytes = fs::read(format!("/proc/{}/cmdline", pid)).ok()?;
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    if end == 0 {
        return None;
    }
    let arg = String::from_utf8_lossy(&bytes[..end]).into_owned();
    if arg.is_empty() {
        None
    } else {
        Some(arg)
    }
}

#[cfg(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly"
))]
pub fn pool_name(cmd: &str) -> Option<String> {
    let idx = cmd.find("pool ")?;
    let name = cmd[idx + 5..].trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

/// Process state from /proc/[pid]/stat (3rd field after the name).
#[cfg(target_os = "linux")]
fn get_state(pid: i32) -> char {
    let Ok(s) = fs::read_to_string(format!("/proc/{}/stat", pid)) else {
        return '?';
    };
    parse_state(&s)
}

/// Extracts the state from a `/proc/[pid]/stat` line.
///
/// The process name may contain spaces and parentheses: find
/// the last `)` (end of the `comm` field) before reading the `state` field.
#[cfg(any(target_os = "linux", test))]
fn parse_state(stat: &str) -> char {
    let Some(idx) = stat.rfind(')') else {
        return '?';
    };
    stat.get(idx + 1..)
        .and_then(|rest| rest.trim_start().chars().next())
        .unwrap_or('?')
}

/// VmRSS from /proc/[pid]/status, in KB. -1 if unavailable.
#[cfg(target_os = "linux")]
fn get_rss_kb(pid: i32) -> i64 {
    let Ok(s) = fs::read_to_string(format!("/proc/{}/status", pid)) else {
        return -1;
    };
    parse_rss_kb(&s)
}

/// Extracts `VmRSS` from a `/proc/[pid]/status` file, in KB.
#[cfg(any(target_os = "linux", test))]
fn parse_rss_kb(status: &str) -> i64 {
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("VmRSS:") {
            let num: String = rest
                .trim()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            return num.parse().unwrap_or(-1);
        }
    }
    -1
}

/// BSD/macOS backend: no /proc, data comes from `ps` and `netstat`.
/// PHP-FPM also sets its process title there, used as-is.
#[cfg(any(
    target_os = "macos",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly"
))]
mod bsd {
    use super::*;
    use std::process::Command;

    /// Scans processes via `ps`: pid, state, RSS and command line.
    /// The master ("php-fpm: master process (...)") is excluded from the workers.
    pub fn scan() -> io::Result<ScanResult> {
        let out = Command::new("ps")
            .args(["-axo", "pid=,state=,rss=,command="])
            .output()?;
        if !out.status.success() {
            return Err(io::Error::other("ps exited with an error status"));
        }
        let text = String::from_utf8_lossy(&out.stdout);
        let mut pools: Vec<PoolScan> = Vec::new();
        let mut masters: Vec<i32> = Vec::new();

        for line in text.lines() {
            let Some((pid, state, rss_kb, cmd)) = parse_ps_line(line) else {
                continue;
            };
            if !cmd.contains("php-fpm:") {
                continue;
            }
            if cmd.contains("master process") {
                masters.push(pid);
                continue;
            }
            let Some(name) = pool_name(&cmd) else {
                continue;
            };
            let worker = Worker { pid, state, rss_kb };
            match pools.iter_mut().find(|p| p.name == name) {
                Some(p) => p.workers.push(worker),
                None => pools.push(PoolScan {
                    name,
                    workers: vec![worker],
                }),
            }
        }

        pools.sort_by(|a, b| a.name.cmp(&b.name));
        masters.sort();
        Ok(ScanResult { pools, masters })
    }

    /// Parses a `ps -o pid=,state=,rss=,command=` line, e.g. "1234 S  8192
    /// php-fpm: pool www". The state may be multi-character ("Ss"): keep
    /// the first one (R = running, S = sleeping, as in /proc/[pid]/stat).
    fn parse_ps_line(line: &str) -> Option<(i32, char, i64, String)> {
        let mut f = line.split_whitespace();
        let pid = f.next()?.parse().ok()?;
        let state = f.next()?.chars().next()?;
        let rss = f.next()?.parse().ok()?;
        let cmd: Vec<&str> = f.collect();
        if cmd.is_empty() {
            return None;
        }
        Some((pid, state, rss, cmd.join(" ")))
    }

    /// TCP accept queue (backlog) of a pool, read from
    /// `netstat -an`: for a LISTEN socket, the Recv-Q column is the number
    /// of connections awaiting acceptance (like rx_queue in
    /// /proc/net/tcp). Returns None for a Unix socket or an unknown port.
    pub fn accept_backlog(listen: &str) -> Option<u32> {
        if listen.starts_with('/') {
            return None;
        }
        let (host, port) = split_listen(listen);
        let want_host = if host.is_empty() || host == "0.0.0.0" || host == "*" || host.contains('[')
        {
            None
        } else {
            Some(host)
        };
        let out = Command::new("netstat")
            .args(["-an", "-p", "tcp"])
            .output()
            .ok()?;
        if !out.status.success() {
            return None;
        }
        let text = String::from_utf8_lossy(&out.stdout);
        for line in text.lines() {
            let Some((lport, recvq, h)) = parse_netstat_line(line) else {
                continue;
            };
            if lport != port {
                continue;
            }
            if let Some(want) = &want_host {
                if &h != want {
                    continue;
                }
            }
            return Some(recvq);
        }
        None
    }

    /// Parses a LISTEN line from `netstat -an -p tcp`, e.g.
    /// "tcp4 0 0 127.0.0.1.9000 *.* LISTEN" → (port, recv-q, host).
    fn parse_netstat_line(line: &str) -> Option<(u16, u32, String)> {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() < 6 || !f[0].starts_with("tcp") {
            return None;
        }
        if f[5] != "LISTEN" {
            return None;
        }
        let (h, p) = f[3].rsplit_once('.')?;
        let lport = p.parse::<u16>().ok()?;
        let recvq = f[1].parse::<u32>().ok()?;
        Some((lport, recvq, h.to_string()))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parse_ps_worker_line() {
            let (pid, state, rss, cmd) = parse_ps_line("1234 S  8192 php-fpm: pool www").unwrap();
            assert_eq!(pid, 1234);
            assert_eq!(state, 'S');
            assert_eq!(rss, 8192);
            assert_eq!(cmd, "php-fpm: pool www");
        }

        #[test]
        fn parse_ps_state_multichar() {
            let (_, state, _, _) = parse_ps_line("1234 Ss  8192 /sbin/launchd").unwrap();
            assert_eq!(state, 'S');
        }

        #[test]
        fn parse_ps_garbage() {
            assert!(parse_ps_line("").is_none());
            assert!(parse_ps_line("abc def ghi").is_none());
            assert!(parse_ps_line("1234 S  8192").is_none());
        }

        #[test]
        fn parse_netstat_listen_line() {
            let (port, recvq, host) = parse_netstat_line(
                "tcp4       0      0  127.0.0.1.9000         *.*                    LISTEN",
            )
            .unwrap();
            assert_eq!(port, 9000);
            assert_eq!(recvq, 0);
            assert_eq!(host, "127.0.0.1");
        }

        #[test]
        fn parse_netstat_wildcard_and_v6() {
            let (port, recvq, host) = parse_netstat_line(
                "tcp46      0      0  *.8082                 *.*                    LISTEN",
            )
            .unwrap();
            assert_eq!(port, 8082);
            assert_eq!(recvq, 0);
            assert_eq!(host, "*");
            let (port, recvq, host) = parse_netstat_line(
                "tcp6       3      0  ::1.9000               *.*                    LISTEN",
            )
            .unwrap();
            assert_eq!(port, 9000);
            assert_eq!(recvq, 3);
            assert_eq!(host, "::1");
        }

        #[test]
        fn parse_netstat_non_listen() {
            assert!(parse_netstat_line(
                "tcp4       0      0  192.168.1.82.60168     13.226.183.20.443      ESTABLISHED"
            )
            .is_none());
        }
    }
}

#[cfg(any(
    target_os = "macos",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly"
))]
pub use bsd::{accept_backlog, scan};

/// Fallback backend for platforms without `/proc` or a compatible `ps`
/// (notably Windows, where PHP-FPM is not supported anyway).
#[cfg(not(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly"
)))]
pub fn scan() -> io::Result<ScanResult> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "PHP-FPM process scanning is only supported on Linux, macOS and BSD",
    ))
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly"
)))]
pub fn accept_backlog(_listen: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    ))]
    #[test]
    fn extract_pool_name() {
        assert_eq!(pool_name("php-fpm: pool www").as_deref(), Some("www"));
        assert_eq!(pool_name("php-fpm: pool www ").as_deref(), Some("www"));
        assert_eq!(
            pool_name("php-fpm: master process (/etc/php-fpm.conf)"),
            None
        );
        assert_eq!(pool_name("nginx"), None);
    }

    #[cfg(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    ))]
    #[test]
    fn split_listen_addr() {
        assert_eq!(split_listen("9001"), ("".to_string(), 9001));
        assert_eq!(split_listen("0.0.0.0:9001"), ("0.0.0.0".to_string(), 9001));
        assert_eq!(
            split_listen("127.0.0.1:9001"),
            ("127.0.0.1".to_string(), 9001)
        );
    }

    #[cfg(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    ))]
    #[test]
    fn split_listen_edge_cases() {
        assert_eq!(
            split_listen(" 127.0.0.1:9001 "),
            ("127.0.0.1".to_string(), 9001)
        );
        assert_eq!(split_listen("9001"), ("".to_string(), 9001));
        assert_eq!(split_listen(":9001"), ("".to_string(), 9001));
        assert_eq!(split_listen("[::1]:9001"), ("[::1]".to_string(), 9001));
        assert_eq!(split_listen("not-a-port"), ("".to_string(), 9000));
    }

    #[cfg(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    ))]
    #[test]
    fn pool_name_accepts_spaces() {
        assert_eq!(
            pool_name("php-fpm: pool my pool").as_deref(),
            Some("my pool")
        );
        assert_eq!(pool_name("php-fpm: pool   "), None);
        assert_eq!(pool_name("php-fpm: pool"), None);
    }

    #[test]
    fn state_from_proc_stat() {
        assert_eq!(parse_state("1234 (php-fpm) S 1 2 3"), 'S');
        assert_eq!(parse_state("1234 (php-fpm) R 1 2 3"), 'R');
        assert_eq!(parse_state("1234 (a b) c) Z 1 2 3"), 'Z');
        assert_eq!(parse_state("garbage"), '?');
        assert_eq!(parse_state(""), '?');
    }

    #[test]
    fn rss_from_proc_status() {
        let status = "Name:\tphp-fpm\nVmPeak:\t 1234 kB\nVmRSS:\t 8192 kB\n";
        assert_eq!(parse_rss_kb(status), 8192);
        assert_eq!(parse_rss_kb("VmRSS:\t 12 kB\n"), 12);
        assert_eq!(parse_rss_kb("Name:\tphp-fpm\n"), -1);
        assert_eq!(parse_rss_kb("VmRSS:\t abc kB\n"), -1);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn ipv4_to_le_hex() {
        assert_eq!(ip_to_hex_le("127.0.0.1"), "0100007F");
        assert_eq!(ip_to_hex_le("0.0.0.0"), "00000000");
        assert_eq!(ip_to_hex_le("192.168.1.10"), "0A01A8C0");
    }

    #[test]
    fn accept_backlog_unix_returns_none() {
        assert_eq!(accept_backlog("/run/php-fpm/www.sock"), None);
    }

    #[test]
    fn accept_backlog_unknown_port() {
        assert_eq!(accept_backlog("59999"), None);
    }
}
