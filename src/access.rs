//! Per-pool access scope based on the user who started fpm-monitor.
//!
//! Root (or a platform where the user cannot be determined) sees everything.
//! Any other user only sees the pools whose `user` directive matches that
//! account. This is application-level filtering: it does not replace Unix file
//! permissions, `hidepid`, or any other OS isolation mechanism.

use crate::config::{Config, PoolConfig};
use crate::proc::ScanResult;
use std::collections::HashSet;
use std::fs;
use std::process::Command;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identity {
    pub uid: Option<u32>,
    pub username: Option<String>,
}

impl Identity {
    /// Resolves the launcher identity once, at startup.
    pub fn current() -> Self {
        let uid = uid_from_proc().or_else(|| command_output(&["-u"]).and_then(|s| s.parse().ok()));
        let username = command_output(&["-un"]).or_else(|| uid.and_then(username_from_passwd));
        Self { uid, username }
    }

    /// True when filtering is meaningful: a non-root Unix user was detected.
    pub fn restricted(&self) -> bool {
        matches!(self.uid, Some(uid) if uid != 0)
    }

    /// Whether a pool belongs to this identity.
    pub fn allows(&self, pool: &PoolConfig) -> bool {
        if !self.restricted() {
            return true;
        }
        match pool.user.as_deref() {
            // PHP-FPM allows `user` to be omitted when it is not started as
            // root; those processes then run as the launcher.
            None => true,
            Some(declared) => {
                if self.username.as_deref() == Some(declared) {
                    return true;
                }
                match declared.parse::<u32>() {
                    Ok(uid) => Some(uid) == self.uid,
                    Err(_) => false,
                }
            }
        }
    }
}

/// Names of the pools visible to this identity. `None` means no filtering.
pub fn allowed_names(identity: &Identity, cfg: &Config) -> Option<HashSet<String>> {
    if !identity.restricted() {
        return None;
    }
    Some(
        cfg.pools
            .iter()
            .filter(|(_, pool)| identity.allows(pool))
            .map(|(name, _)| name.clone())
            .collect(),
    )
}

/// Keeps only the configuration of the allowed pools.
pub fn filter_config(cfg: Config, allowed: &Option<HashSet<String>>) -> Config {
    let Some(allowed) = allowed else {
        return cfg;
    };
    Config {
        pools: cfg
            .pools
            .into_iter()
            .filter(|(name, _)| allowed.contains(name))
            .collect(),
        ..cfg
    }
}

/// Keeps only the scanned pools of the allowed identity.
///
/// Master PIDs are hidden for a non-root user: they cannot be safely mapped to
/// a single pool, and exposing them would leak information about other pools.
pub fn filter_scan(scan: ScanResult, allowed: &Option<HashSet<String>>) -> ScanResult {
    let Some(allowed) = allowed else {
        return scan;
    };
    ScanResult {
        pools: scan
            .pools
            .into_iter()
            .filter(|pool| allowed.contains(&pool.name))
            .collect(),
        masters: Vec::new(),
    }
}

/// Reads the effective UID from Linux `/proc`, without spawning a process.
fn uid_from_proc() -> Option<u32> {
    let status = fs::read_to_string("/proc/self/status").ok()?;
    parse_uid_status(&status)
}

fn parse_uid_status(status: &str) -> Option<u32> {
    status
        .lines()
        .find(|line| line.starts_with("Uid:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|value| value.parse().ok())
}

/// Debian/macOS fallback when `id -un` is unavailable.
fn username_from_passwd(uid: u32) -> Option<String> {
    let passwd = fs::read_to_string("/etc/passwd").ok()?;
    parse_passwd(&passwd, uid)
}

fn parse_passwd(passwd: &str, uid: u32) -> Option<String> {
    for line in passwd.lines() {
        let mut fields = line.split(':');
        let name = fields.next()?;
        let _password = fields.next()?;
        let value = fields.next()?;
        if value.parse::<u32>().ok() == Some(uid) {
            return Some(name.to_string());
        }
    }
    None
}

fn command_output(args: &[&str]) -> Option<String> {
    for binary in ["/usr/bin/id", "id"] {
        let Ok(output) = Command::new(binary).args(args).output() else {
            continue;
        };
        if !output.status.success() {
            continue;
        }
        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !text.is_empty() {
            return Some(text);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proc::Worker;

    fn identity(uid: Option<u32>, username: Option<&str>) -> Identity {
        Identity {
            uid,
            username: username.map(str::to_string),
        }
    }

    fn pool(user: Option<&str>) -> PoolConfig {
        PoolConfig {
            user: user.map(str::to_string),
            ..Default::default()
        }
    }

    #[test]
    fn root_sees_every_pool() {
        let root = identity(Some(0), Some("root"));
        let cfg = Config {
            pools: [("www".into(), pool(Some("www-data")))]
                .into_iter()
                .collect(),
            ..Default::default()
        };
        assert!(root.allows(&pool(Some("nobody"))));
        assert!(allowed_names(&root, &cfg).is_none());
    }

    #[test]
    fn non_root_only_matches_declared_user() {
        let user = identity(Some(1000), Some("www-data"));
        assert!(user.allows(&pool(Some("www-data"))));
        assert!(user.allows(&pool(Some("1000"))));
        assert!(user.allows(&pool(None)));
        assert!(!user.allows(&pool(Some("api"))));
        assert!(!user.allows(&pool(Some("2000"))));
    }

    #[test]
    fn unknown_username_fails_closed() {
        let user = identity(Some(1000), None);
        assert!(user.restricted());
        assert!(!user.allows(&pool(Some("www-data"))));
        assert!(user.allows(&pool(None)));
    }

    #[test]
    fn restricted_scope_filters_config_and_scan() {
        let user = identity(Some(1000), Some("www-data"));
        let mut cfg = Config::default();
        cfg.pools.insert("www".into(), pool(Some("www-data")));
        cfg.pools.insert("api".into(), pool(Some("api")));
        let allowed = allowed_names(&user, &cfg).unwrap();
        assert_eq!(allowed, ["www".to_string()].into_iter().collect());

        let filtered = filter_config(cfg, &Some(allowed.clone()));
        assert!(filtered.pools.contains_key("www"));
        assert!(!filtered.pools.contains_key("api"));

        let scan = ScanResult {
            pools: vec![
                crate::proc::PoolScan {
                    name: "www".into(),
                    workers: vec![Worker {
                        pid: 1,
                        state: 'S',
                        rss_kb: 1,
                    }],
                },
                crate::proc::PoolScan {
                    name: "api".into(),
                    workers: Vec::new(),
                },
            ],
            masters: vec![42],
        };
        let scan = filter_scan(scan, &Some(allowed));
        assert_eq!(scan.pools.len(), 1);
        assert_eq!(scan.pools[0].name, "www");
        assert!(scan.masters.is_empty());
    }

    #[test]
    fn unrestricted_scope_keeps_everything() {
        let cfg = Config {
            pools: [("api".into(), pool(Some("api")))].into_iter().collect(),
            ..Default::default()
        };
        let scan = ScanResult {
            pools: vec![crate::proc::PoolScan {
                name: "api".into(),
                workers: Vec::new(),
            }],
            masters: vec![42],
        };
        assert!(filter_config(cfg.clone(), &None).pools.contains_key("api"));
        let kept = filter_scan(scan, &None);
        assert_eq!(kept.pools.len(), 1);
        assert_eq!(kept.masters, vec![42]);
    }

    #[test]
    fn parses_linux_uid_line() {
        let status = "Name:\tphp-fpm\nUid:\t1000\t1000\t1000\t1000\nGid:\t1000\n";
        assert_eq!(parse_uid_status(status), Some(1000));
        assert_eq!(parse_uid_status("Name:\tphp-fpm\n"), None);
    }

    #[test]
    fn parses_passwd_uid() {
        let passwd = "root:x:0:0:root:/root:/bin/sh\nwww-data:x:33:33:www:/var/www:/bin/sh\n";
        assert_eq!(parse_passwd(passwd, 33).as_deref(), Some("www-data"));
        assert_eq!(parse_passwd(passwd, 99), None);
    }
}
