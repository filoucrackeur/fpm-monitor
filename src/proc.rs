use std::io;

#[cfg(not(target_os = "macos"))]
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

/// Scanne /proc à la recherche des processus PHP-FPM.
/// Le master est écarté du comptage des workers mais enregistré.
#[cfg(not(target_os = "macos"))]
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

/// Profondeur de la file d'attente d'acceptation (backlog) d'un pool en TCP,
/// lue directement dans /proc/net/tcp et /proc/net/tcp6 : pour un socket en
/// état LISTEN, rx_queue contient le nombre de connexions en attente.
/// Retourne None pour un socket unix (non lisible) ou un port introuvable.
#[cfg(not(target_os = "macos"))]
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

/// Sépare "ip:port" ou "port" → (hôte, port).
/// Sans hôte, php-fpm écoute sur 0.0.0.0 (INADDR_ANY) : on renvoie "".
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

/// Adresse IPv4 vers la notation hexadécimale petit-boutiste de /proc/net/tcp.
/// "127.0.0.1" → "0100007F".
#[cfg(not(target_os = "macos"))]
fn ip_to_hex_le(host: &str) -> String {
    let mut out = String::new();
    for octet in host.split('.') {
        let n: u8 = octet.parse().unwrap_or(0);
        out.insert_str(0, &format!("{n:02X}"));
    }
    out
}

/// Lit le premier argument de /proc/[pid]/cmdline.
/// PHP-FPM met tout dans argv[0] : "php-fpm: pool www".
#[cfg(not(target_os = "macos"))]
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

pub fn pool_name(cmd: &str) -> Option<String> {
    let idx = cmd.find("pool ")?;
    let name = cmd[idx + 5..].trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

/// État du process depuis /proc/[pid]/stat (3e champ après le nom).
#[cfg(not(target_os = "macos"))]
fn get_state(pid: i32) -> char {
    let Ok(s) = fs::read_to_string(format!("/proc/{}/stat", pid)) else {
        return '?';
    };
    let Some(idx) = s.rfind(')') else {
        return '?';
    };
    let after: Vec<char> = s[idx..].chars().collect();
    // ')', ' ', state
    if after.len() >= 3 {
        after[2]
    } else {
        '?'
    }
}

/// VmRSS depuis /proc/[pid]/status, en Ko. -1 si indisponible.
#[cfg(not(target_os = "macos"))]
fn get_rss_kb(pid: i32) -> i64 {
    let Ok(s) = fs::read_to_string(format!("/proc/{}/status", pid)) else {
        return -1;
    };
    for line in s.lines() {
        if line.starts_with("VmRSS:") {
            let rest = line.trim_start_matches("VmRSS:").trim();
            let num: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            return num.parse().unwrap_or(-1);
        }
    }
    -1
}

/// Backend macOS : pas de /proc, les données viennent de `ps` et `netstat`.
/// PHP-FPM y renseigne aussi son titre de process, exploité tel quel.
#[cfg(target_os = "macos")]
mod macos {
    use super::*;
    use std::process::Command;

    /// Scanne les processus via `ps` : pid, état, RSS et ligne de commande.
    /// Le master ("php-fpm: master process (...)") est écarté des workers.
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

    /// Parse une ligne `ps -o pid=,state=,rss=,command=`, ex. "1234 S  8192
    /// php-fpm: pool www". L'état peut être multi-caractères ("Ss") : on garde
    /// le premier (R = running, S = sleeping, comme /proc/[pid]/stat).
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

    /// File d'attente d'acceptation (backlog) d'un pool en TCP, lue dans
    /// `netstat -an`: pour un socket LISTEN, la colonne Recv-Q vaut le nombre
    /// de connexions en attente d'acceptation (comme rx_queue dans
    /// /proc/net/tcp). Retourne None pour un socket unix ou un port inconnu.
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

    /// Parse une ligne LISTEN de `netstat -an -p tcp`, ex.
    /// "tcp4 0 0 127.0.0.1.9000 *.* LISTEN" → (port, recv-q, hôte).
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

#[cfg(target_os = "macos")]
pub use macos::{accept_backlog, scan};

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn split_listen_addr() {
        assert_eq!(split_listen("9001"), ("".to_string(), 9001));
        assert_eq!(split_listen("0.0.0.0:9001"), ("0.0.0.0".to_string(), 9001));
        assert_eq!(
            split_listen("127.0.0.1:9001"),
            ("127.0.0.1".to_string(), 9001)
        );
    }

    #[test]
    #[cfg(not(target_os = "macos"))]
    fn ipv4_to_le_hex() {
        assert_eq!(ip_to_hex_le("127.0.0.1"), "0100007F");
        assert_eq!(ip_to_hex_le("0.0.0.0"), "00000000");
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
