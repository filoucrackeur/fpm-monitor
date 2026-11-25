use super::Labels;

pub(super) const LABELS: &Labels = &Labels {
    tab_monitoring: "Παρακολούθηση",
    tab_graphs: "Γραφήματα",
    tab_logs: "Καταγραφές",
    tab_configs: "Διαμορφώσεις",
    pools: "Δεξαμενές",
    workers: "Διεργασίες",
    running: "Ενεργές",
    idle: "Αδράνεια",
    backlog: "Εκκρεμότητες",
    mem: "Μνήμη",
    master: "Κύρια",
    help: "q έξοδος · Tab επόμενη καρτέλα · 1-4 καρτέλα · ← → υποκαρτέλα · ↑ ↓ παρακολούθηση",
    no_pool: "Δεν εντοπίστηκε δεξαμενή.",
    last_measurements: "τελευταίες {} μετρήσεις",
    g_running: "ενεργές",
    g_workers: "διεργασίες",
    g_idle: "αδράνεια",
    g_backlog: "εκκρεμότητες",
    log_php: "Καταγραφή PHP",
    slow_queries: "Αργά ερωτήματα",
    access: "Πρόσβαση",
    no_entries: "(χωρίς καταχωρήσεις)",
    older_lines: "+{} παλαιότερες γραμμές",
    more_workers: "+{} ακόμη",
    pool_configs: "Διαμορφώσεις δεξαμενών",
    no_config: "Δεν βρέθηκε διαμόρφωση.",
    unknown_pool: "Άγνωστη δεξαμενή.",
    no_directives: "(δεν διαβάστηκαν οδηγίες)",
    master_pid: "PID κύριας",
    legend: "Υπόμνημα:",
    leg_red: "κόκκινο = κορεσμός/εκκρεμότητες",
    leg_green: "πράσινο = υγιές/αδράνεια",
    leg_white: "λευκό = ουδέτερο/ondemand",
    leg_blue: "μπλε = ενεργό/static",
    read_proc: "αδυναμία ανάγνωσης /proc (απαιτείται Linux): {}",
    unknown_option: "άγνωστη επιλογή: {}",
    run_help: "εκτελέστε με --help",
    invalid_port: "μη έγκυρη θύρα: {} (αναμένεται 1-65535)",
    web_serving: "η διεπαφή web ακούει στο http://127.0.0.1:{} (Ctrl-C για έξοδο)",
    web_error: "δεν ήταν δυνατή η εκκίνηση της διεπαφής web: {}",
    skip_content: "Μετάβαση στο περιεχόμενο",
    menu: "Μενού",
    language: "Γλώσσα",
    web_dashboard: "διαδικτυακός πίνακας",
    theme_dark: "Σκοτεινή λειτουργία",
    theme_light: "Φωτεινή λειτουργία",
    connecting: "σύνδεση…",
    disconnected: "αποσυνδεδεμένο",
    mock_data: "δεδομένα επίδειξης",
    live: "ζωντανά",
    updated: "ενημερώθηκε",
    pool: "Pool",
    type_label: "Τύπος",
    max_child: "Μέγ. θυγατρικά",
    max_req: "Μέγ. αιτήματα",
    min_spare: "min spare",
    max_spare: "max spare",
    max_backlog: "max backlog",
    configured_detected: "ρυθμισμένα + εντοπισμένα",
    total_processes: "σύνολο διεργασιών",
    state_r: "κατάσταση R",
    state_s: "κατάσταση S",
    pending_connections: "εκκρεμείς συνδέσεις",
    sum_rss: "άθροισμα RSS workers",
    global_directives: "Καθολικές οδηγίες",
    footer_tagline: "διαβάζει τον τοπικό πίνακα διεργασιών, χωρίς πρόσβαση δικτύου.",
    requests: "Αιτήματα/δευτ",
    total_requests: "Συνολικά αιτήματα",
    copy: "Αντιγραφή",
    copied: "Αντιγράφηκε",
    min: "min",
    max: "max",
    avg: "μέσος",
    axis_x: "χρόνος",
    axis_y: "τιμή",
    collection_error: "Σφάλμα συλλογής:",
    cannot_reach: "Δεν είναι δυνατή η σύνδεση με το fpm-monitor:",
    refresh: "ανανέωση",
};

pub(super) const USAGE: &str = "\
fpm-monitor — παρακολούθηση δεξαμενών PHP-FPM (μεταφορά σε Rust του fpm-monitor.c)

Χρήση: fpm-monitor [ΕΠΙΛΟΓΕΣ]

Επιλογές:
  -c, --config <ΔΙΑΔΡΟΜΗ>  αρχείο php-fpm.conf ή φάκελος δεξαμενών προς ανάλυση
  -v, --verbose             Εμφάνιση λεπτομερειών κάθε worker (pid, κατάσταση, RSS)
      --color               Επιβολή χρώματος
      --no-color            Απενεργοποίηση χρώματος
  -t, --tui                 Διαδραστικό ταμπλό (σαν το Ember) — καρτέλες
                            παρακολούθηση, γραφήματα, καταγραφές και διαμορφώσεις
                            (υποκαρτέλες με ← →). Έξοδος: q / Ctrl-C
      --interval <ΔΕΥΤ>     Διάστημα ανανέωσης του TUI (προεπιλογή 1)
      --lang <ΓΛΩΣΣΑ>       Γλώσσα διεπαφής: en (US), fr, zh, ar, es, it, ja, de,
                            ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
      --mock                Εμφάνιση δεδομένων επίδειξης (τοπική δοκιμή)
      --watch            Εκκίνηση της διεπαφής web αντί της προβολής CLI
      --port <PORT>      Θύρα της διεπαφής web (προεπιλογή 8471)
  -h, --help                Εμφάνιση αυτής της βοήθειας
  -V, --version          Show the version

Η ανίχνευση είναι αυτόματη (συνήθεις τοποθεσίες + οδηγία include).
";
