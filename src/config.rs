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
    pub user: Option<String>,
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

/// Discovers the pool configuration files:
/// - path passed as an argument (php-fpm.conf file or pool directory)
/// - otherwise searches the usual locations + includes
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
    collect_php_versions(
        Path::new("/usr/local/etc/php"),
        false,
        &mut files,
        &mut visited,
    );
    #[cfg(target_os = "macos")]
    collect_php_versions(
        Path::new("/opt/homebrew/etc/php"),
        false,
        &mut files,
        &mut visited,
    );
    // Debian and Ubuntu install each PHP version under /etc/php/<version>/fpm.
    collect_php_versions(Path::new("/etc/php"), true, &mut files, &mut visited);
    files
}

/// Collects versioned PHP installations under `base`.
///
/// Homebrew and source installs use `<base>/<version>/php-fpm.conf`, while
/// Debian and Ubuntu use `<base>/<version>/fpm/php-fpm.conf`.
fn collect_php_versions(
    base: &Path,
    debian_layout: bool,
    files: &mut Vec<PathBuf>,
    visited: &mut HashSet<PathBuf>,
) {
    let Ok(entries) = fs::read_dir(base) else {
        return;
    };
    for e in entries.flatten() {
        let version = e.path();
        if !version.is_dir() {
            continue;
        }
        let dir = if debian_layout {
            version.join("fpm")
        } else {
            version
        };
        if !dir.is_dir() {
            continue;
        }
        let conf = dir.join("php-fpm.conf");
        if conf.is_file() {
            parse_includes(&conf, files, visited);
        }
        for name in ["pool.d", "php-fpm.d"] {
            let pool_dir = dir.join(name);
            if pool_dir.is_dir() {
                collect_pool_files(&pool_dir, files, visited);
            }
        }
    }
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
    // The top-level file is part of the configuration too: it usually holds
    // the `[global]` directives (error_log, pid, ...).
    files.push(conf.to_path_buf());
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
            expand_glob(&expand_value(val), files, visited);
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
        let Some((key, raw_val)) = split_kv(t) else {
            continue;
        };
        let val = expand_value(raw_val);
        let n = val.parse::<u32>().ok();
        match section.as_deref() {
            Some(name) => {
                if let Some(p) = config.pools.get_mut(name) {
                    p.raw.push((key.to_string(), val.clone()));
                }
            }
            None => config.globals.push((key.to_string(), val.clone())),
        }
        match key {
            "user" => {
                if let Some(p) = current_pool(config, &section) {
                    p.user = Some(val.clone());
                }
            }
            "pm" => {
                if let Some(p) = current_pool(config, &section) {
                    p.pm = Some(val.clone());
                }
            }
            "listen" => {
                if let Some(p) = current_pool(config, &section) {
                    p.listen = Some(val.clone());
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
                    p.slowlog = Some(val.clone());
                }
            }
            "access.log" => {
                if let Some(p) = current_pool(config, &section) {
                    p.access_log = Some(val.clone());
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

/// Unquotes a value and expands PHP-FPM environment placeholders.
///
/// PHP-FPM accepts `${NAME}` and, since 8.3, `${NAME:-default}`. An unset or
/// empty variable uses the default, or an empty string when no default is set.
fn expand_value(s: &str) -> String {
    expand_env(unquote(s))
}

fn expand_env(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(start) = rest.find("${") {
        out.push_str(&rest[..start]);
        let tail = &rest[start + 2..];
        let Some(end) = tail.find('}') else {
            out.push_str(&rest[start..]);
            return out;
        };
        let expr = &tail[..end];
        let (name, default) = match expr.split_once(":-") {
            Some((name, default)) => (name, Some(default)),
            None => (expr, None),
        };
        if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            // Not a variable PHP-FPM would expand: keep the text as-is.
            out.push_str("${");
            out.push_str(expr);
            out.push('}');
        } else {
            match std::env::var(name) {
                Ok(value) if !value.is_empty() => out.push_str(&value),
                _ => out.push_str(default.unwrap_or("")),
            }
        }
        rest = &tail[end + 1..];
    }
    out.push_str(rest);
    out
}

/// Minimal glob matcher (supports `*` and `?`)
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
                '?' if j < n.len() => {
                    dp[i + 1][j + 1] = true;
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
        assert_eq!(p.user.as_deref(), Some("www-data"));
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

    #[test]
    fn strip_comments_and_values() {
        assert_eq!(strip_comment("pm = dynamic ; comment"), "pm = dynamic ");
        assert_eq!(strip_comment("pm = dynamic # comment"), "pm = dynamic ");
        assert_eq!(strip_comment("pm = dynamic"), "pm = dynamic");
        assert_eq!(strip_comment("; full line"), "");

        assert_eq!(split_kv(" pm = dynamic "), Some(("pm", "dynamic")));
        assert_eq!(split_kv("no separator"), None);
        assert_eq!(split_kv("a = b = c"), Some(("a", "b = c")));

        assert_eq!(unquote(" \"quoted\" "), "quoted");
        assert_eq!(unquote("'single'"), "single");
        assert_eq!(unquote("plain"), "plain");
        assert_eq!(unquote("\""), "\"");
    }

    #[test]
    fn expands_environment_placeholders() {
        std::env::set_var("FPM_MONITOR_TEST_POOL", "app");
        std::env::set_var("FPM_MONITOR_TEST_EMPTY", "");
        assert_eq!(
            expand_env("/run/php-${FPM_MONITOR_TEST_POOL}.sock"),
            "/run/php-app.sock"
        );
        assert_eq!(expand_env("${FPM_MONITOR_TEST_POOL:-www-data}"), "app");
        std::env::remove_var("FPM_MONITOR_TEST_POOL");
        assert_eq!(expand_env("${FPM_MONITOR_TEST_POOL:-www-data}"), "www-data");
        assert_eq!(
            expand_env("${FPM_MONITOR_TEST_EMPTY:-fallback}"),
            "fallback"
        );
        assert_eq!(expand_env("${FPM_MONITOR_TEST_UNSET}"), "");
        assert_eq!(expand_env("${BAD-NAME}"), "${BAD-NAME}");
        assert_eq!(expand_env("${UNCLOSED"), "${UNCLOSED");
        assert_eq!(expand_env("plain $value"), "plain $value");
        std::env::remove_var("FPM_MONITOR_TEST_EMPTY");
    }

    #[test]
    fn expands_environment_in_pool_config() {
        std::env::set_var("FPM_MONITOR_TEST_USER", "app");
        std::env::set_var("FPM_MONITOR_TEST_SUFFIX", "slow");
        let mut dir = std::env::temp_dir();
        dir.push(format!("fpm-monitor-test-env-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let file = dir.join("www.conf");
        fs::write(
            &file,
            "[www]\n\
             user = ${FPM_MONITOR_TEST_USER:-www-data}\n\
             listen = \"/run/php-${FPM_MONITOR_TEST_USER}.sock\"\n\
             pm.max_children = ${FPM_MONITOR_TEST_CHILDREN:-9}\n\
             slowlog = /var/log/php-${FPM_MONITOR_TEST_SUFFIX}.log\n",
        )
        .unwrap();
        std::env::remove_var("FPM_MONITOR_TEST_CHILDREN");

        let config = load(&[file]);
        let p = config.pools.get("www").unwrap();
        assert_eq!(p.user.as_deref(), Some("app"));
        assert_eq!(p.listen.as_deref(), Some("/run/php-app.sock"));
        assert_eq!(p.max_children, Some(9));
        assert_eq!(p.slowlog.as_deref(), Some("/var/log/php-slow.log"));
        assert_eq!(
            p.raw
                .iter()
                .find(|(key, _)| key == "user")
                .map(|(_, value)| value.as_str()),
            Some("app")
        );

        std::env::remove_var("FPM_MONITOR_TEST_USER");
        std::env::remove_var("FPM_MONITOR_TEST_SUFFIX");
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn quoted_values_and_duplicate_sections() {
        let mut dir = std::env::temp_dir();
        dir.push(format!("fpm-monitor-test-quoted-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let file = dir.join("www.conf");
        fs::write(
            &file,
            "[www]\nlisten = \"127.0.0.1:9000\"\n[www]\nslowlog = '/tmp/www-slow.log'\n",
        )
        .unwrap();

        let config = load(&[file]);
        let p = config.pools.get("www").unwrap();
        assert_eq!(p.listen.as_deref(), Some("127.0.0.1:9000"));
        assert_eq!(p.slowlog.as_deref(), Some("/tmp/www-slow.log"));

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn include_glob_and_missing_files() {
        let mut dir = std::env::temp_dir();
        dir.push(format!("fpm-monitor-test-include-{}", std::process::id()));
        let pools = dir.join("pool.d");
        fs::create_dir_all(&pools).unwrap();
        fs::write(
            dir.join("php-fpm.conf"),
            format!(
                "[global]\ninclude = {}/pool.d/*.conf\ninclude = /nonexistent/fpm/*.conf\n",
                dir.display()
            ),
        )
        .unwrap();
        fs::write(
            pools.join("www.conf"),
            "[www]\npm = dynamic\npm.max_children = 5\n",
        )
        .unwrap();
        fs::write(pools.join("ignored.txt"), "[ignored]\npm = static\n").unwrap();

        let files = discover(Some(&dir.join("php-fpm.conf")));
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|p| p.ends_with("php-fpm.conf")));
        assert!(files.iter().any(|p| p.ends_with("www.conf")));

        let config = load(&files);
        assert_eq!(config.pools["www"].max_children, Some(5));
        assert!(!config.pools.contains_key("ignored"));

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn expands_environment_in_include_path() {
        let mut dir = std::env::temp_dir();
        dir.push(format!(
            "fpm-monitor-test-include-env-{}",
            std::process::id()
        ));
        let pools = dir.join("pool.d");
        fs::create_dir_all(&pools).unwrap();
        std::env::set_var(
            "FPM_MONITOR_TEST_INCLUDE_DIR",
            dir.to_string_lossy().as_ref(),
        );
        fs::write(
            dir.join("php-fpm.conf"),
            "[global]\ninclude = ${FPM_MONITOR_TEST_INCLUDE_DIR}/pool.d/*.conf\n",
        )
        .unwrap();
        fs::write(pools.join("www.conf"), "[www]\npm = static\n").unwrap();

        let files = discover(Some(&dir.join("php-fpm.conf")));
        assert!(files.iter().any(|p| p.ends_with("www.conf")));

        std::env::remove_var("FPM_MONITOR_TEST_INCLUDE_DIR");
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn discovers_debian_and_homebrew_versioned_layouts() {
        let mut dir = std::env::temp_dir();
        dir.push(format!("fpm-monitor-test-versioned-{}", std::process::id()));
        let debian = dir.join("debian/8.4/fpm");
        let homebrew = dir.join("homebrew/8.4");
        fs::create_dir_all(debian.join("pool.d")).unwrap();
        fs::create_dir_all(homebrew.join("pool.d")).unwrap();
        fs::write(debian.join("php-fpm.conf"), "[global]\n").unwrap();
        fs::write(
            debian.join("pool.d/www.conf"),
            "[www]\npm = dynamic\npm.max_children = 5\n",
        )
        .unwrap();
        fs::write(homebrew.join("php-fpm.conf"), "[global]\n").unwrap();
        fs::write(homebrew.join("pool.d/app.conf"), "[app]\npm = static\n").unwrap();

        let mut files = Vec::new();
        let mut visited = HashSet::new();
        collect_php_versions(&dir.join("debian"), true, &mut files, &mut visited);
        collect_php_versions(&dir.join("homebrew"), false, &mut files, &mut visited);

        assert!(files
            .iter()
            .any(|p| p.ends_with("debian/8.4/fpm/php-fpm.conf")));
        assert!(files
            .iter()
            .any(|p| p.ends_with("debian/8.4/fpm/pool.d/www.conf")));
        assert!(files
            .iter()
            .any(|p| p.ends_with("homebrew/8.4/php-fpm.conf")));
        assert!(files
            .iter()
            .any(|p| p.ends_with("homebrew/8.4/pool.d/app.conf")));

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn discover_from_pool_directory() {
        let mut dir = std::env::temp_dir();
        dir.push(format!("fpm-monitor-test-dir-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("a.conf"), "[a]\npm = static\n").unwrap();
        fs::write(dir.join("b.conf"), "[b]\npm = ondemand\n").unwrap();

        let files = discover(Some(&dir));
        assert_eq!(files.len(), 2);
        let config = load(&files);
        assert_eq!(config.pools.len(), 2);

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn glob_edge_cases() {
        assert!(glob_match("*", ""));
        assert!(glob_match("*", "anything"));
        assert!(glob_match("a*b*c", "aXXbYYc"));
        assert!(!glob_match("a*b*c", "aXXbYY"));
        assert!(glob_match("???", "abc"));
        assert!(!glob_match("???", "ab"));
        assert!(!glob_match("abc", "abd"));
    }
}
