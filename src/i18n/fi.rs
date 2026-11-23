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
    invalid_port: "virheellinen portti: {} (odotettu 1-65535)",
    web_serving: "verkkokäyttöliittymä kuuntelee osoitteessa http://127.0.0.1:{} (Ctrl-C lopettaa)",
    web_error: "verkkokäyttöliittymää ei voi käynnistää: {}",
    skip_content: "Siirry sisältöön",
    menu: "Valikko",
    language: "Kieli",
    web_dashboard: "verkkopaneeli",
    theme_dark: "Tumma tila",
    theme_light: "Vaalea tila",
    connecting: "yhdistetään…",
    disconnected: "yhteys katkaistu",
    mock_data: "demodata",
    live: "live",
    updated: "päivitetty",
    pool: "Pooli",
    type_label: "Tyyppi",
    max_child: "Enint. lapset",
    max_req: "Enint. pyynnöt",
    min_spare: "min spare",
    max_spare: "max spare",
    max_backlog: "max backlog",
    configured_detected: "määritetty + havaittu",
    total_processes: "prosessit yhteensä",
    state_r: "tila R",
    state_s: "tila S",
    pending_connections: "odottavat yhteydet",
    sum_rss: "workerien RSS yhteensä",
    global_directives: "Globaalit direktiivit",
    footer_tagline: "lukee paikallisen prosessitaulun, ei verkkoyhteyttä.",
    requests: "Pyynnöt/s",
    total_requests: "Pyyntöjä yhteensä",
    copy: "Kopioi",
    copied: "Kopioitu",
    min: "min",
    max: "max",
    avg: "keskiarvo",
    axis_x: "aika",
    axis_y: "arvo",
    collection_error: "Keräysvirhe:",
    cannot_reach: "fpm-monitoriin ei saada yhteyttä:",
    refresh: "päivitys",
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
      --watch            Käynnistä verkkokäyttöliittymä CLI-näkymän sijaan
      --port <PORT>      Verkkokäyttöliittymän portti (oletus 8471)
  -h, --help             Näytä tämä ohje

Tunnistus on automaattinen (tavalliset sijainnit + include-direktiivi).
";
