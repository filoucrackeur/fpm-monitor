use std::fs;
use std::io;

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
