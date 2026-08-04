use crate::config::{Config, PoolConfig};
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct PoolLogs {
    pub name: String,
    pub php_log: Option<String>,
    pub slow_log: Option<String>,
    pub access_log: Option<String>,
    pub php: Vec<String>,
    pub slow: Vec<String>,
    pub access: Vec<String>,
}

/// Collecte les logs PHP et les requêtes lentes de chaque pool.
/// `max_lines` est le nombre de lignes de queue lues par fichier.
pub fn collect(cfg: &Config, names: &[String], max_lines: usize) -> Vec<PoolLogs> {
    let error_log = cfg
        .globals
        .iter()
        .rev()
        .find(|(k, _)| k == "error_log")
        .map(|(_, v)| v.clone());
    names
        .iter()
        .map(|name| {
            let pool = cfg.pools.get(name);
            // Un pool peut surcharger le error_log global.
            let pool_error = pool.and_then(|p| raw_get(p, "php_admin_value[error_log]"));
            let php = pool_error.or_else(|| error_log.clone());
            let slow = pool.and_then(|p| p.slowlog.clone());
            let access = pool.and_then(|p| p.access_log.clone());
            PoolLogs {
                name: name.clone(),
                php_log: php.clone(),
                slow_log: slow.clone(),
                access_log: access.clone(),
                php: php
                    .as_deref()
                    .map(|p| read_tail(p, max_lines))
                    .unwrap_or_default(),
                slow: slow
                    .as_deref()
                    .map(|p| read_tail(p, max_lines))
                    .unwrap_or_default(),
                access: access
                    .as_deref()
                    .map(|p| read_tail(p, max_lines))
                    .unwrap_or_default(),
            }
        })
        .collect()
}

/// Logs de démonstration pour le mode `--mock`.
pub fn mock(names: &[String]) -> Vec<PoolLogs> {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| {
            let secs = d.as_secs();
            format!(
                "{:02}-{:02}-{:04} {:02}:{:02}:{:02}",
                (secs / 86400) % 31 + 1,
                (secs / 86400) % 12 + 1,
                2026,
                (secs / 3600) % 24,
                (secs / 60) % 60,
                secs % 60
            )
        })
        .unwrap_or_else(|_| "01-01-2026 00:00:00".to_string());
    names
        .iter()
        .enumerate()
        .map(|(i, name)| PoolLogs {
            name: name.clone(),
            php_log: Some("/var/log/php-fpm.log".to_string()),
            slow_log: Some(format!("/var/log/php-fpm/{name}-slow.log")),
            access_log: Some(format!("/var/log/nginx/{name}.access.log")),
            php: vec![
                format!("[{ts}] NOTICE: fpm is running, pid 1"),
                format!("[{ts}] NOTICE: ready to handle connections"),
                format!("[{ts}] WARNING: [pool {name}] child 100 exited on signal 15 (SIGTERM)"),
                format!("[{ts}] NOTICE: [pool {name}] child 101 started"),
                if i % 2 == 0 {
                    format!("[{ts}] ERROR: [pool {name}] accept() failed: Too many open files (24)")
                } else {
                    format!("[{ts}] NOTICE: [pool {name}] child 102 running")
                },
            ],
            slow: vec![
                format!("[pool {name}] pid 10000 script = /index.php (req \"GET /index.php\")"),
                format!("[pool {name}] execution time 3.21 sec, CPU 0.45 sec"),
                format!("[pool {name}] pid 10001 script = /api/users (req \"GET /api/users\")"),
                format!("[pool {name}] execution time 5.44 sec, CPU 1.02 sec"),
            ],
            access: vec![
                format!("127.0.0.1 -  {ts} \"GET /index.php\" 200"),
                format!("127.0.0.1 -  {ts} \"GET /api/users\" 200"),
                format!("203.0.113.9 -  {ts} \"GET /error.php\" 500"),
                format!("198.51.100.7 -  {ts} \"GET /missing.php\" 404"),
            ],
        })
        .collect()
}

/// Lit les `max_lines` dernières lignes d'un fichier (tail efficace :
/// on lit par blocs depuis la fin jusqu'à avoir assez de lignes).
pub fn read_tail(path: &str, max_lines: usize) -> Vec<String> {
    let mut f = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let len = match f.metadata() {
        Ok(m) => m.len(),
        Err(_) => return Vec::new(),
    };
    if len == 0 {
        return Vec::new();
    }
    let mut size = 4096usize;
    loop {
        let start = len.saturating_sub(size as u64);
        let chunk_len = (len - start) as usize;
        let mut buf = vec![0u8; chunk_len];
        if f.seek(SeekFrom::Start(start)).is_err() || f.read_exact(&mut buf).is_err() {
            return Vec::new();
        }
        let newlines = buf.iter().filter(|&&b| b == b'\n').count();
        let lines = tail_of(&buf, max_lines);
        if newlines >= max_lines || start == 0 {
            return lines;
        }
        if size >= (1 << 20) {
            return lines;
        }
        size *= 2;
    }
}

fn tail_of(buf: &[u8], max_lines: usize) -> Vec<String> {
    let text = String::from_utf8_lossy(buf);
    let mut v: Vec<String> = text.lines().map(|s| s.to_string()).collect();
    let n = v.len();
    if n > max_lines {
        v = v.split_off(n - max_lines);
    }
    v
}

fn raw_get(p: &PoolConfig, key: &str) -> Option<String> {
    p.raw.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn tail_small_file() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("fpm-logs-test-{}", std::process::id()));
        let mut f = fs::File::create(&path).unwrap();
        for i in 1..=5 {
            writeln!(f, "ligne {i}").unwrap();
        }
        f.sync_all().unwrap();
        let t = read_tail(path.to_str().unwrap(), 3);
        assert_eq!(t, vec!["ligne 3", "ligne 4", "ligne 5"]);
        fs::remove_file(&path).ok();
    }

    #[test]
    fn tail_missing_file() {
        assert!(read_tail("/nonexistent/fpm-monitor.log", 10).is_empty());
    }

    #[test]
    fn tail_larger_than_chunk() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("fpm-logs-test-big-{}", std::process::id()));
        let mut f = fs::File::create(&path).unwrap();
        for i in 0..20_000 {
            writeln!(f, "ligne de log numéro {i} avec un peu de contenu").unwrap();
        }
        f.sync_all().unwrap();
        let t = read_tail(path.to_str().unwrap(), 50);
        assert_eq!(t.len(), 50);
        assert!(t[0].starts_with("ligne de log numéro 19950"));
        assert!(t[49].starts_with("ligne de log numéro 19999"));
        fs::remove_file(&path).ok();
    }

    #[test]
    fn error_log_derniere_directive_gagne() {
        let mut dir = std::env::temp_dir();
        dir.push(format!("fpm-monitor-logs-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let f1 = dir.join("docker.conf");
        fs::write(&f1, "[global]\nerror_log = /proc/self/fd/2\n").unwrap();
        let f2 = dir.join("zz-errorlog.conf");
        fs::write(&f2, "[global]\nerror_log = /var/log/php-fpm/error.log\n").unwrap();

        let cfg = crate::config::load(&[f1, f2]);
        let lg = collect(&cfg, &["www".to_string()], 10);
        assert_eq!(lg[0].php_log.as_deref(), Some("/var/log/php-fpm/error.log"));

        fs::remove_dir_all(&dir).ok();
    }
}
