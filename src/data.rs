use crate::config::{Config, PoolConfig};
use crate::proc::{PoolScan, ScanResult, Worker};
use std::collections::HashSet;

pub struct Row {
    pub name: String,
    pub cfg: Option<PoolConfig>,
    pub workers: usize,
    pub running: usize,
    pub idle: usize,
    pub backlog: Option<u32>,
    pub backlog_max: Option<u32>,
    pub scan: Option<PoolScan>,
}

/// Construit les lignes (pools) à partir du scan /proc + de la config.
/// Aucun appel réseau : workers/états lus dans /proc, backlog lu dans les
/// tables socket (/proc/net/tcp{,6}) pour les listens TCP, et pour les
/// sockets unix on ne garde que le maximum fourni par listen.backlog.
pub fn build_rows(scan: ScanResult, cfg: &Config) -> Vec<Row> {
    let mut names: Vec<String> = scan
        .pools
        .iter()
        .map(|p| p.name.clone())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    for name in cfg.pools.keys() {
        names.push(name.clone());
    }
    names.sort();
    names.dedup();

    names
        .into_iter()
        .map(|name| {
            let sc = scan.pools.iter().find(|p| p.name == name).cloned();
            let workers = sc.as_ref().map(|p| p.workers.len()).unwrap_or(0);
            let running = sc
                .as_ref()
                .map(|p| p.workers.iter().filter(|w| w.state == 'R').count())
                .unwrap_or(0);
            let idle = sc
                .as_ref()
                .map(|p| p.workers.iter().filter(|w| w.state == 'S').count())
                .unwrap_or(0);
            let cfg = cfg.pools.get(&name).cloned();

            let (backlog, backlog_max) = match &cfg {
                Some(c) => match &c.listen {
                    Some(listen) if listen.starts_with('/') => (None, c.listen_backlog),
                    Some(listen) => (crate::proc::accept_backlog(listen), c.listen_backlog),
                    None => (None, None),
                },
                None => (None, None),
            };

            Row {
                cfg,
                name,
                workers,
                running,
                idle,
                backlog,
                backlog_max,
                scan: sc,
            }
        })
        .collect()
}

pub fn mock_scan() -> ScanResult {
    let mut pools = Vec::new();
    let mk = |name: &str, n: usize, running: usize| {
        let workers: Vec<Worker> = (1..=n)
            .map(|i| Worker {
                pid: 10000 + i as i32,
                state: if i <= running { 'R' } else { 'S' },
                rss_kb: 20_000 + (i as i64) * 137,
            })
            .collect();
        PoolScan {
            name: name.to_string(),
            workers,
        }
    };
    pools.push(mk("www", 8, 4));
    pools.push(mk("api", 10, 9));
    pools.push(mk("legacy", 2, 2));
    pools.push(PoolScan {
        name: "app".to_string(),
        workers: Vec::new(),
    });
    ScanResult {
        pools,
        masters: vec![4242],
    }
}

pub fn mock_config() -> Config {
    use std::collections::HashMap;

    let pool = |_name: &str, pm: &str, max_children: u32, max_requests: u32, listen: &str| {
        let raw = vec![
            ("pm".to_string(), pm.to_string()),
            ("listen".to_string(), listen.to_string()),
            ("pm.max_children".to_string(), max_children.to_string()),
            ("pm.max_requests".to_string(), max_requests.to_string()),
        ];
        PoolConfig {
            pm: Some(pm.to_string()),
            listen: Some(listen.to_string()),
            listen_backlog: Some(128),
            max_children: Some(max_children),
            max_requests: Some(max_requests),
            raw,
            ..Default::default()
        }
    };

    let mut pools = HashMap::new();
    pools.insert("www".to_string(), pool("www", "static", 8, 500, "9000"));
    let mut api = pool("api", "dynamic", 10, 10, "9001");
    api.min_spare_servers = Some(2);
    api.max_spare_servers = Some(8);
    pools.insert("api".to_string(), api);
    pools.insert("app".to_string(), pool("app", "ondemand", 3, 1000, "9002"));

    Config {
        pools,
        global_max_requests: None,
        globals: vec![("pid.file".to_string(), "/run/php-fpm.pid".to_string())],
    }
}

pub fn mock_rows(cfg: &Config) -> Vec<Row> {
    let mut cfg_map = cfg.pools.clone();
    let mut take = |name: &str| cfg_map.remove(name);

    let scan = |name: &str, states: &[char], rss: i64| {
        Some(PoolScan {
            name: name.to_string(),
            workers: states
                .iter()
                .enumerate()
                .map(|(i, s)| Worker {
                    pid: 10000 + i as i32 + name.as_bytes()[0] as i32 * 100,
                    state: *s,
                    rss_kb: rss + i as i64 * 137,
                })
                .collect(),
        })
    };

    vec![
        Row {
            cfg: take("www"),
            name: "www".to_string(),
            workers: 3,
            running: 1,
            idle: 2,
            backlog: Some(0),
            backlog_max: Some(128),
            scan: scan("www", &['R', 'S', 'S'], 20_000),
        },
        Row {
            cfg: take("api"),
            name: "api".to_string(),
            workers: 10,
            running: 9,
            idle: 1,
            backlog: Some(2),
            backlog_max: Some(511),
            scan: scan(
                "api",
                &['R', 'R', 'S', 'R', 'R', 'R', 'R', 'R', 'R', 'R'],
                22_000,
            ),
        },
        Row {
            cfg: take("app"),
            name: "app".to_string(),
            workers: 0,
            running: 0,
            idle: 0,
            backlog: Some(0),
            backlog_max: Some(128),
            scan: scan("app", &[], 0),
        },
        Row {
            cfg: None,
            name: "legacy".to_string(),
            workers: 2,
            running: 2,
            idle: 0,
            backlog: None,
            backlog_max: None,
            scan: scan("legacy", &['R', 'R'], 18_000),
        },
    ]
}
