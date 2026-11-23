<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="760">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/github/downloads/filoucrackeur/fpm-monitor/total)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/github/issues/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md) | [Русский](README.ru.md) | [हिन्दी](README.hi.md) | [Bahasa Indonesia](README.id.md) | [한국어](README.ko.md) | [Norsk](README.no.md) | [Svenska](README.sv.md) | **Suomi** | [Dansk](README.da.md) | [Ελληνικά](README.el.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md) | [Українська](README.uk.md) | [ไทย](README.th.md) | [Tiếng Việt](README.vi.md)

Rustilla kirjoitettu PHP-FPM-poolien valvontaohjelma (portti `fpm-monitor.c`:stä),
jossa on [Emberin](https://github.com/alexandre-daubois/ember) innoittama
interaktiivinen päätekojelauta. Se toimitetaan yhtenä riippuvuudettomana
binäärinä ja toimii **Linuxilla, macOS:llä ja Windowsilla** **18
kohdearkkitehtuurilla**. Käyttöliittymä on saatavilla 22 kielellä (oletus:
englanti) `--lang`-valitsimella, ja tämä dokumentaatio on käännetty 22 kielelle.

<p align="center">
  <img src="docs/screenshot.en.animated.svg" alt="fpm-monitor TUI — interaktiivinen kojelauta (englanti)" width="700">
</p>

## Ominaisuudet

- **CLI-tuloste**: poolitaulukko (pool, tyyppi, workers, running, idle, backlog,
  max_children, max_requests, muisti) ja työprosessikohtaiset tiedot (`-v`: pid,
  tila, RSS).
- **TUI-kojelauta** (`-t`): interaktiivinen käyttöliittymä, jossa on 4 välilehteä.
  - **Seuranta**: poolien ja niiden työprosessien reaaliaikainen tila (muisti
    pooleittain).
  - **Kaaviot**: yksi suuri kaavio poolia kohden (Ember-tyyliin)
    `←`/`→`-alivälilehdillä: `running`, `workers`, `idle`, `backlog` (näytetään
    aina, myös tyhjänä), sekä katkoviivaiset kynnysviivat: `max_children`
    (harmaa), `max_requests` (punainen), `min_spare` (pinkki) ja `max_spare`
    (violetti) `idle`-paneelissa.
  - **Lokit**: pooleittain PHP-lokin (`error_log`) viimeisimmät rivit, hitaat
    kyselyt (`slowlog`) ja **käyttöloki** (`access.log`, värilliset HTTP-koodit)
    alivälilehdillä.
  - **Asetukset**: tiedostoista luetut direktiivit (`global` + poolit)
    alivälilehdillä.
- **Verkkopaneeli** (`--watch`): samat neljä näkymää selaimessa,
  `http://127.0.0.1:8471`-osoitteessa ilman riippuvuuksia toimivan HTTP-palvelimen
  tarjoamana. Portti asetetaan `--port`-valitsimella; tiedot kerätään taustalla
  `--interval`-välillä ja sivu säilyttää 60 historia­pistettä kaavioilleen.
- Automaattinen asetusten tunnistus (tavalliset sijainnit + `include`-direktiivi).
- Ei verkkokutsuja: kaikki luetaan suoraan `/proc`-hakemistosta (prosessit, RSS,
  tila, TCP-soketin vastaanottojono). macOS:llä ja BSD:llä samat tiedot luetaan
  `ps`- ja `netstat`-komennoilla; FPM-tilan asetusta (`pm.status_path`) ei
  tarvita.
- **Siirrettävyys suunnittelun lähtökohtana**: staattiset musl-binäärit 10
  Linux-arkkitehtuurille, glibc-koonnokset lopuille sekä macOS ja Windows. Ei
  ajonaikaisia riippuvuuksia eikä libc-versiota, joka pitäisi sovittaa (lukuun
  ottamatta dokumentoituja glibc-koonnoksia).

## Vaatimukset

- **Rust 1.78+** lähdekoodista kääntämiseen (MSRV testataan CI:ssä).
- **Linux**: `/proc`-hakemistoa käytetään todelliseen dataan. TCP-soketeille
  vastaanottojono luetaan tiedostosta `/proc/net/tcp`; unix-soketeille
  enimmäisarvona näytetään vain `listen.backlog`.
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**: ei `/proc`-hakemistoa;
  prosessit ja soketit luetaan `ps`- ja `netstat`-komennoilla.
- **Windows**: CLI kääntyy ja toimii (`--help`, `--mock`), mutta PHP-FPM ei ole
  saatavilla Windowsilla, joten prosessien live-tunnistus palauttaa selkeän
  virheen.

## Asennus

### Valmiit binäärit

Jokainen [julkaisu](https://github.com/filoucrackeur/fpm-monitor/releases)
sisältää raa'at binäärit kaikille tuetuille kohteille, arkiston kohdetta kohden
(`.tar.gz` Unixissa, `.zip` Windowsissa), `.deb`/`.rpm`-paketit ja
Homebrew-kaavan.

Arkistot ovat itsenäisiä: jokainen sisältää `fpm-monitor`-ohjelman sekä
`LICENSE`- ja `README.md`-tiedostot:

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

Tai käytä raakaa binääriä suoraan:

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### Tuetut alustat ja arkkitehtuurit

| Alusta | Arkkitehtuurit | Rust-kohteet | Jakelu |
| ------ | -------------- | ------------ | ------ |
| Linux (staattinen musl) | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | binääri, deb/rpm |
| Linux (glibc ≥ 2.17) | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | binääri, deb/rpm |
| macOS 11+ | Apple Silicon, Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | binääri, Homebrew |
| Windows 10+ | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | OCI-levykuva |

### Lähdekoodista

```sh
cargo build --release
# binääri on polussa target/release/fpm-monitor
```

### Ristikäännös

CI käyttää [cargo-zigbuildiä](https://github.com/rust-cross/cargo-zigbuild)
kaikkien Linux-binäärien tuottamiseen yhdeltä x86_64-koontikoneelta:

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# binääri on polussa target/aarch64-unknown-linux-musl/release/fpm-monitor
```

Korvaa kohde millä tahansa yllä olevan taulukon kohdekolmikolla. Jos haluat ottaa
sen käyttöön `php:*-fpm-alpine`-kontissa:

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Linux-paketit (Red Hat / Debian)

`.deb`- ja `.rpm`-paketit liitetään jokaiseen julkaisuun arkkitehtuureille amd64,
arm64, armhf, armel, i386, riscv64, ppc64el, ppc64, s390x, loong64 ja sparc64.
Musl-paketeilla ei ole ulkoisia riippuvuuksia, ja ne toimivat missä tahansa
uudehkossa RHEL-, Fedora-, CentOS-, Debian- tai Ubuntu-versiossa.

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- Muilla arkkitehtuureilla korvaa `amd64`/`x86_64` vastaavalla paketin nimellä
  (`arm64`, `armhf`, `armel`, `i386`, `riscv64`, `ppc64el`, `ppc64`, `s390x`,
  `loong64`, `sparc64`).

Arkistojen ja pakettien uudelleenrakentaminen paikallisesti olemassa olevasta
`target/`-hakemistosta:

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # edellyttää nfpm-työkalua PATH-polussa
```

### macOS (Homebrew)

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

Moniarkkitehtuurinen levykuva julkaistaan GitHub Container Registryssä alustoille
`linux/amd64`, `linux/arm64`, `linux/ppc64le` ja `linux/riscv64`:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host` antaa valvontaohjelman lukea PHP-FPM-prosessit isäntäkoneen
`/proc`-hakemistosta; liitä FPM-poolien asetukset `-v`-valitsimella, jotta
poolit löydetään (myös tavalliset `/etc`-polut skannataan).

## Käyttö

```
fpm-monitor [OPTIONS]

Valinnat:
  -c, --config <PATH>     analysoitava php-fpm.conf-tiedosto tai poolien hakemisto
  -v, --verbose           Näytä jokaisen työprosessin tiedot (pid, tila, RSS)
      --color             Pakota väri
      --no-color          Poista värit käytöstä
  -t, --tui               Interaktiivinen kojelauta (välilehdet, `←`/`→`-alivälilehdet)
      --interval <SEC>    TUI:n päivitysväli (oletus 1)
      --watch             Verkkopaneeli CLI-näkymän sijaan
      --port <PORT>       Verkkopaneelin portti (oletus 8471)
      --lang <LANG>       Käyttöliittymän kieli: en, fr, zh, ar, es, it, ja, de,
                          ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
                          (oletus: en)
      --mock              Demotiedot (paikallinen testi)
  -h, --help              Näytä tämä ohje
```

### Käyttöliittymän kieli

`--lang` hyväksyy arvot `en` (US, oletus), `fr`, `zh`, `ar`, `es`, `it`, `ja`,
`de`, `ru`, `hi`, `id`, `ko`, `no`, `sv`, `fi`, `da`, `el`, `tr`, `pl`, `uk`, `th`, `vi` (myös muoto `--lang=fr` käy). Se koskee TUI-kojelautaa sekä CLI-tulostetta
(otsikot, yhteenveto, selite) ja `--help`-ohjetta.

### TUI-kojelauta

TUI täyttää koko päätteen (dynaaminen koon muutos) ja pysyy responsiivisena:
tiedot päivitetään taustalla määritetyllä välillä (`--interval`, oletus 1 s);
vaikka `/proc`-luku olisi hidas, piirto jatkuu.

| Näppäin       | Toiminto                           |
| ------------- | ---------------------------------- |
| `1` – `4`, `Tab` | Vaihda välilehteä               |
| `←` / `→`     | Siirry alivälilehdillä (poolit/asetukset) |
| `↑` / `↓`     | Vieritä seurantanäkymää            |
| `q`, `Ctrl-C` | Lopeta                             |

### Verkkopaneeli

`--watch` tarjoilee samat tiedot yksisivuisena verkkosovelluksena CLI-taulukon
tulostamisen sijaan:

```sh
fpm-monitor --watch                 # http://127.0.0.1:8471
fpm-monitor --watch --port 9000     # mukautettu portti
fpm-monitor --watch --mock          # demotiedot ilman PHP-FPM:ää
```

Palvelin kuuntelee vain paikallista rajapintaa eikä käytä ulkoisia
riippuvuuksia; sivu kysyy `/api/state`-rajapinnasta poolit, workerien tiedot,
lokit, kokoonpanodirektiivit ja viimeiset 60 näytettä. Tiedot kerätään taustalla
`--interval`-välillä (oletus 1 s), joten hidas skannaus ei koskaan estä pyyntöä.
Neljä välilehteä ovat **Valvonta**, **Kaaviot**, **Lokit** ja **Kokoonpanot**;
`1`–`4` ja `←`/`→` valitsevat välilehdet ja alivälilehdet.

`--port` hyväksyy arvot `1`–`65535` (`--port=9000` toimii myös); muu arvo
lopettaa koodilla `2`. Palvelin ei koskaan tee lähtevää pyyntöä.

## Projektin rakenne

| Tiedosto          | Tehtävä                                             |
| ----------------- | --------------------------------------------------- |
| `src/main.rs`     | CLI, valinnat, koordinointi                         |
| `src/config.rs`   | Asetusten tunnistus ja jäsennys                     |
| `src/proc.rs`     | `/proc`-luku (workers, RSS, tila, TCP-backlog)      |
| `src/data.rs`     | Tietojen yhdistäminen taulukkoriveiksi              |
| `src/logs.rs`     | PHP-lokin ja hitaiden kyselyjen luku                |
| `src/render.rs`   | Tekstituloste (CLI-taulukko)                        |
| `src/tui.rs`      | Interaktiivinen kojelauta (4 välilehteä, kaaviot)   |
| `src/web.rs`       | Verkkopalvelin, taustakeruu, JSON-tilannekuva        |
| `src/web/index.html` | Upotettu yksisivuinen verkkopaneeli                |
| `src/i18n.rs`     | Lokalisointi (22 kieltä, `--lang`)                   |
| `src/term.rs`     | Värintunnistus / tyylit                             |
| `tests/cli.rs`    | CLI:n end-to-end-testit (`--help`, `--mock`, i18n)  |
| `.github/workflows` | CI (testit, kattavuus, 18 koontia, QEMU-savutestit) |

## Kehitys

```sh
cargo build          # koonti (debug)
cargo test           # yksikkö- + integraatiotestit
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # muotoilu
```

CI ajaa testisarjan Linuxilla, macOS:llä ja Windowsilla MSRV-versiolla (Rust
1.78), julkaisee kattavuusmerkin, ristikääntää 18 julkaisukohdetta ja suorittaa
vieraan arkkitehtuurin binäärit QEMU:n alla.

## Osallistuminen

Osallistuminen on tervetullutta! Lue [CONTRIBUTING.md](CONTRIBUTING.md) ennen
issuen tai pull requestin avaamista, noudata [toimintaohjeistustamme](CODE_OF_CONDUCT.md)
ja katso käyttökysymyksiä varten [SUPPORT.md](SUPPORT.md). Ilmoita
tietoturvaongelmista yksityisesti [SECURITY.md](SECURITY.md):ssä kuvatulla tavalla.

## Lisenssi

[MIT](LICENSE)
