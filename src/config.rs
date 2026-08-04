use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

pub const DEFAULT_CONF_FILES: &[&str] = &[
    "/etc/php-fpm.conf",
    "/usr/local/etc/php-fpm.conf",
    "/usr/local/var/etc/php-fpm.conf",
];

pub const DEFAULT_POOL_DIRS: &[&str] =
    &["/etc/php-fpm.d", "/usr/local/etc/php-fpm.d", "/etc/php-fpm"];

#[derive(Debug, Clone, Default)]
pub struct PoolConfig {
    pub pm: Option<String>,
    pub listen: Option<String>,
    pub listen_backlog: Option<u32>,
    pub max_children: Option<u32>,
    pub start_servers: Option<u32>,
    pub min_spare_servers: Option<u32>,
    pub max_spare_servers: Option<u32>,
    pub max_requests: Option<u32>,
    pub slowlog: Option<String>,
    pub access_log: Option<String>,
    pub raw: Vec<(String, String)>,
}

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub pools: HashMap<String, PoolConfig>,
    pub global_max_requests: Option<u32>,
    pub globals: Vec<(String, String)>,
}

/// Découvre les fichiers de config des pools :
/// - chemin passé en argument (fichier php-fpm.conf ou dossier de pools)
/// - sinon recherche dans les emplacements usuels + includes
pub fn discover(cli_path: Option<&Path>) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    let mut visited: HashSet<PathBuf> = HashSet::new();

    if let Some(p) = cli_path {
        if p.is_file() {
            parse_includes(p, &mut files, &mut visited);
        } else if p.is_dir() {
            collect_pool_files(p, &mut files, &mut visited);
        }
        return files;
    }

    for c in DEFAULT_CONF_FILES {
        let p = Path::new(c);
        if p.is_file() {
            parse_includes(p, &mut files, &mut visited);
        }
    }
    for d in DEFAULT_POOL_DIRS {
        let p = Path::new(d);
        if p.is_dir() {
            collect_pool_files(p, &mut files, &mut visited);
        }
    }
    if let Ok(entries) = fs::read_dir("/usr/local/etc/php") {
        for e in entries.flatten() {
            let p = e.path();
            if !p.is_dir() {
                continue;
            }
            let conf = p.join("php-fpm.conf");
            if conf.is_file() {
                parse_includes(&conf, &mut files, &mut visited);
            }
            let pool_d = p.join("pool.d");
            if pool_d.is_dir() {
                collect_pool_files(&pool_d, &mut files, &mut visited);
            }
        }
    }
    files
}

pub fn load(files: &[PathBuf]) -> Config {
    let mut config = Config::default();
    for f in files {
        parse_file(f, &mut config);
    }
    config
}

fn parse_includes(conf: &Path, files: &mut Vec<PathBuf>, visited: &mut HashSet<PathBuf>) {
    if !visited.insert(conf.to_path_buf()) {
        return;
    }
    let Ok(content) = fs::read_to_string(conf) else {
        return;
    };
    for line in content.lines() {
        let t = strip_comment(line).trim();
        if t.is_empty() || t.starts_with('[') {
            continue;
        }
        let Some((key, val)) = split_kv(t) else {
            continue;
        };
        if key == "include" {
            expand_glob(unquote(val), files, visited);
        }
    }
}

fn collect_pool_files(dir: &Path, files: &mut Vec<PathBuf>, visited: &mut HashSet<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else {
        return;
    };
    let mut v: Vec<PathBuf> = rd
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().is_some_and(|x| x == "conf"))
        .collect();
    v.sort();
    for p in v {
        if visited.insert(p.clone()) {
            files.push(p);
        }
    }
}

fn expand_glob(pat: &str, files: &mut Vec<PathBuf>, visited: &mut HashSet<PathBuf>) {
    if pat.contains('*') || pat.contains('?') {
        let (dir, pat_name) = match pat.rfind('/') {
            Some(i) => {
                let d = &pat[..i];
                let d = if d.is_empty() { "/" } else { d };
                (d, &pat[i + 1..])
            }
            None => (".", pat),
        };
        if let Ok(rd) = fs::read_dir(dir) {
            let mut names: Vec<String> = rd
                .flatten()
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .filter(|n| glob_match(pat_name, n))
                .collect();
            names.sort();
            for n in names {
                let p = PathBuf::from(dir).join(&n);
                if p.extension().is_some_and(|x| x == "conf") && visited.insert(p.clone()) {
                    files.push(p);
                }
            }
        }
    } else {
        let p = PathBuf::from(pat);
        if p.is_dir() {
            collect_pool_files(&p, files, visited);
        } else if p.extension().is_some_and(|x| x == "conf") && visited.insert(p.clone()) {
            files.push(p);
        }
    }
}

fn parse_file(path: &Path, config: &mut Config) {
    let Ok(content) = fs::read_to_string(path) else {
        return;
    };
    let mut section: Option<String> = None;
    for line in content.lines() {
        let t = strip_comment(line).trim();
        if t.is_empty() {
            continue;
        }
        if t.starts_with('[') && t.ends_with(']') {
            let name = t[1..t.len() - 1].trim().to_string();
            if name == "global" || name.is_empty() {
                section = None;
            } else {
                section = Some(name.clone());
                config.pools.entry(name.clone()).or_default();
            }
            continue;
        }
        let Some((key, val)) = split_kv(t) else {
            continue;
        };
        let n = val.parse::<u32>().ok();
        match section.as_deref() {
            Some(name) => {
                if let Some(p) = config.pools.get_mut(name) {
                    p.raw.push((key.to_string(), unquote(val).to_string()));
                }
            }
            None => config
                .globals
                .push((key.to_string(), unquote(val).to_string())),
        }
        match key {
            "pm" => {
                if let Some(p) = current_pool(config, &section) {
                    p.pm = Some(unquote(val).to_string());
                }
            }
            "listen" => {
                if let Some(p) = current_pool(config, &section) {
                    p.listen = Some(unquote(val).to_string());
                }
            }
            "listen.backlog" => {
                if let Some(p) = current_pool(config, &section) {
                    p.listen_backlog = n;
                }
            }
            "pm.max_children" => {
                if let Some(p) = current_pool(config, &section) {
                    p.max_children = n;
                }
            }
            "pm.start_servers" => {
                if let Some(p) = current_pool(config, &section) {
                    p.start_servers = n;
                }
            }
            "pm.min_spare_servers" => {
                if let Some(p) = current_pool(config, &section) {
                    p.min_spare_servers = n;
                }
            }
            "pm.max_spare_servers" => {
                if let Some(p) = current_pool(config, &section) {
                    p.max_spare_servers = n;
                }
            }
            "pm.max_requests" => {
                if let Some(p) = current_pool(config, &section) {
                    p.max_requests = n;
                } else {
                    config.global_max_requests = n;
                }
            }
            "slowlog" => {
                if let Some(p) = current_pool(config, &section) {
                    p.slowlog = Some(unquote(val).to_string());
                }
            }
            "access.log" => {
                if let Some(p) = current_pool(config, &section) {
                    p.access_log = Some(unquote(val).to_string());
                }
            }
            _ => {}
        }
    }
}

fn current_pool<'a>(
    config: &'a mut Config,
    section: &Option<String>,
) -> Option<&'a mut PoolConfig> {
    let name = section.as_ref()?;
    config.pools.get_mut(name)
}

fn strip_comment(line: &str) -> &str {
    for (i, c) in line.char_indices() {
        if c == ';' || c == '#' {
            return &line[..i];
        }
    }
    line
}

fn split_kv(line: &str) -> Option<(&str, &str)> {
    let idx = line.find('=')?;
    Some((line[..idx].trim(), line[idx + 1..].trim()))
}

fn unquote(s: &str) -> &str {
    let t = s.trim();
    let quoted = t.len() >= 2
        && ((t.starts_with('"') && t.ends_with('"')) || (t.starts_with('\'') && t.ends_with('\'')));
    if quoted {
        &t[1..t.len() - 1]
    } else {
        t
    }
}

/// Matcher de glob minimal (supporte `*` et `?`)
fn glob_match(pattern: &str, name: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let n: Vec<char> = name.chars().collect();
    let mut dp = vec![vec![false; n.len() + 1]; p.len() + 1];
    dp[0][0] = true;
    for i in 0..p.len() {
        if p[i] == '*' && dp[i][0] {
            dp[i + 1][0] = true;
        }
        for j in 0..=n.len() {
            if !dp[i][j] {
                continue;
            }
            match p[i] {
                '*' => {
                    dp[i + 1][j] = true;
                    if j < n.len() {
                        dp[i][j + 1] = true;
                    }
                }
                '?' => {
                    if j < n.len() {
                        dp[i + 1][j + 1] = true;
                    }
                }
                c if j < n.len() && c == n[j] => dp[i + 1][j + 1] = true,
                _ => {}
            }
        }
    }
    dp[p.len()][n.len()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn glob() {
        assert!(glob_match("*.conf", "www.conf"));
        assert!(glob_match("*.conf", ".conf"));
        assert!(!glob_match("*.conf", "www.txt"));
        assert!(glob_match("php*", "php-fpm.conf"));
        assert!(!glob_match("php*", "nginx.conf"));
        assert!(glob_match("pool-?.conf", "pool-1.conf"));
        assert!(!glob_match("pool-?.conf", "pool-12.conf"));
    }

    #[test]
    fn parse_pool_conf() {
        let mut dir = std::env::temp_dir();
        dir.push(format!("fpm-monitor-test-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let file = dir.join("www.conf");
        let mut f = fs::File::create(&file).unwrap();
        writeln!(f, "; commentaire").unwrap();
        writeln!(f, "[www]").unwrap();
        writeln!(f, "user = www-data # inline").unwrap();
        writeln!(f, "pm = dynamic").unwrap();
        writeln!(f, "pm.max_children = 12").unwrap();
        writeln!(f, "pm.start_servers = 4").unwrap();
        writeln!(f, "pm.min_spare_servers = 2").unwrap();
        writeln!(f, "pm.max_spare_servers = 8").unwrap();
        writeln!(f, "pm.max_requests = 500").unwrap();
        writeln!(f, "slowlog = /var/log/php-fpm/www-slow.log").unwrap();
        writeln!(f, "access.log = /var/log/php-fpm/www.access.log").unwrap();
        f.sync_all().unwrap();

        let config = load(&[file]);
        let p = config.pools.get("www").unwrap();
        assert_eq!(p.pm.as_deref(), Some("dynamic"));
        assert_eq!(p.max_children, Some(12));
        assert_eq!(p.start_servers, Some(4));
        assert_eq!(p.min_spare_servers, Some(2));
        assert_eq!(p.max_spare_servers, Some(8));
        assert_eq!(p.max_requests, Some(500));
        assert_eq!(p.slowlog.as_deref(), Some("/var/log/php-fpm/www-slow.log"));
        assert_eq!(
            p.access_log.as_deref(),
            Some("/var/log/php-fpm/www.access.log")
        );

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn global_max_requests() {
        let mut dir = std::env::temp_dir();
        dir.push(format!("fpm-monitor-test-g-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let file = dir.join("php-fpm.conf");
        let mut f = fs::File::create(&file).unwrap();
        writeln!(f, "pm.max_requests = 1000").unwrap();
        writeln!(f, "[www]").unwrap();
        writeln!(f, "pm = static").unwrap();
        f.sync_all().unwrap();

        let config = load(&[file]);
        assert_eq!(config.global_max_requests, Some(1000));
        assert_eq!(config.pools["www"].pm.as_deref(), Some("static"));

        fs::remove_dir_all(&dir).ok();
    }
}
