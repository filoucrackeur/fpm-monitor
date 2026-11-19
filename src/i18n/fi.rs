use super::Labels;

pub(super) const LABELS: &Labels = &Labels {
    tab_monitoring: "Seuranta",
    tab_graphs: "Kaaviot",
    tab_logs: "Lokit",
    tab_configs: "Määritykset",
    pools: "Poolit",
    workers: "Prosessit",
    running: "Käynnissä",
    idle: "Vapaa",
    backlog: "Jono",
    mem: "Muisti",
    master: "Master",
    help: "q lopeta · Tab seuraava välilehti · 1-4 välilehti · ← → alavälilehti · ↑ ↓ seuranta",
    no_pool: "Poolia ei havaittu.",
    last_measurements: "viimeiset {} mittausta",
    g_running: "käynnissä",
    g_workers: "prosessia",
    g_idle: "vapaa",
    g_backlog: "jono",
    log_php: "PHP-loki",
    slow_queries: "Hitaat kyselyt",
    access: "Käyttö",
    no_entries: "(ei merkintöjä)",
    older_lines: "+{} vanhempaa riviä",
    more_workers: "+{} lisää",
    pool_configs: "Poolien määritykset",
    no_config: "Määrityksiä ei löytynyt.",
    unknown_pool: "Tuntematon pool.",
    no_directives: "(direktiivejä ei luettu)",
    master_pid: "Master-PID",
    legend: "Merkkien selitykset:",
    leg_red: "punainen = kyllästyminen/jono",
    leg_green: "vihreä = kunnossa/vapaa",
    leg_white: "valkoinen = neutraali/ondemand",
    leg_blue: "sininen = aktiivinen/static",
    read_proc: "/proc-tiedostoa ei voi lukea (vaatii Linuxin): {}",
    unknown_option: "tuntematon valitsin: {}",
    run_help: "suorita --help-valitsimella",
};

pub(super) const USAGE: &str = "\
fpm-monitor — PHP-FPM-poolien seuranta (Rust-portti fpm-monitor.c:stä)

Käyttö: fpm-monitor [VALITSIMET]

Valitsimet:
  -c, --config <POLKU>   analysoitava php-fpm.conf-tiedosto tai poolikansio
  -v, --verbose          Näytä jokaisen prosessin tiedot (pid, tila, RSS)
      --color            Pakota väri
      --no-color         Poista väri käytöstä
  -t, --tui              Interaktiivinen kojelauta (kuten Ember) — välilehdet
                         seuranta, kaaviot, lokit ja määritykset (alavälilehdet
                         ← → -näppäimillä). Lopeta: q / Ctrl-C
      --interval <SEK>   TUI:n päivitysväli (oletus 1)
      --lang <KIELI>     Käyttöliittymän kieli: en (US), fr, zh, ar, es, it, ja, de,
                         ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
      --mock             Näytä demodata (paikallinen testi)
  -h, --help             Näytä tämä ohje

Tunnistus on automaattinen (tavalliset sijainnit + include-direktiivi).
";
