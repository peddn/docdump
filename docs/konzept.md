# docdump – Konzept

> Stand: 26.11.2026 · Status: Entwurf
> Entscheidungen mit Begründung: [entscheidungen.md](entscheidungen.md) · Begriffe Deutsch/Englisch: [begriffe.md](begriffe.md)

## 1. Idee in einem Satz

Ein selbst gehosteter Dokumentations-Server, dessen Oberfläche einen Ordner mit Markdown-Dateien spiegelt: Jede Aktion in der Oberfläche ist eine Dateioperation, und nach jeder Operation ist der Ordner noch gültig.

## 2. Alleinstellungsmerkmal

| Eigenschaft | Bedeutung |
|---|---|
| **Dateien sind die einzige Wahrheit** | Keine Datenbank, kein Cache, der vom Ordner abweichen kann. Löscht man die App, bleibt ein vollständiger Markdown-Ordner. |
| **Umbauen ohne Reue** | Verschieben und Umbenennen in der Oberfläche passt alle Verweise im Workspace automatisch an. |
| **Live gespiegelt** | Änderungen von außen (Editor, `git pull`, Dateimanager) erscheinen sofort in der Oberfläche. |

Abgrenzung zu bestehenden Tools (Recherche vom 26.11.2026):

- **Typemill:** Drag & drop im Baum, aber PHP.
- **LeafWiki:** Menüstruktur liegt in JSON/SQLite statt im Ordner.
- **SilverBullet:** Anhänge lassen sich in der App nicht umbenennen.
- **Markoun:** Bilder liegen neben der Seite statt zentral.

Keines der Tools garantiert, dass der Ordner nach einer Umbau-Aktion intakt bleibt.

## 3. Grundprinzipien

Diese Regeln sind nicht verhandelbar. Jede Implementierungsentscheidung muss sie einhalten.

- **P1 – Dateien sind die einzige Wahrheit.** Alles, was die App dauerhaft weiß, steht in Dateien: im Workspace oder in der Serverkonfiguration. Abgeleiteter Zustand (Baum, Verweisliste) liegt nur im Speicher und lässt sich jederzeit aus den Dateien neu aufbauen.
- **P2 – Ein Weg für Änderungen.** Die Oberfläche ändert ihren Baum nur aufgrund von Ereignissen aus der Dateiüberwachung, auch bei eigenen Aktionen. Es gibt keine optimistischen Updates des Baums.
- **P3 – Der Ordner bleibt gültig.** Keine Operation der App hinterlässt kaputte Verweise, die sie hätte anpassen können. Was sie nicht anpassen kann, meldet sie.
- **P4 – Standard-Markdown.** CommonMark plus GitHub-Erweiterungen (Tabellen, Aufgabenlisten, Durchstreichen, Fußnoten). Keine eigene Syntax, keine Wikilinks.
- **P5 – Nichts außerhalb des Workspace.** Jeder Pfad wird geprüft, bevor er das Dateisystem erreicht.

## 4. Begriffe

Alle Begriffe mit deutscher und englischer Bezeichnung und Erklärung stehen in [begriffe.md](begriffe.md). Im Code gilt die englische, in Dokumentation und Kommentaren die deutsche Bezeichnung.

Die wichtigsten für dieses Konzept:

- **Workspace** – ein Wurzelordner mit Dokumentation, z. B. „Beruf“ oder „Hobby“.
- **Seite** – eine `.md`-Datei.
- **Bildverzeichnis** – das in der Konfiguration festgelegte Verzeichnis für Bilder.
- **Verweis** – Oberbegriff für Seitenlink und Bildeinbindung.
- **Wurzelpfad** – Pfad mit führendem `/` in einem Seitenlink, bezogen auf die Workspace-Wurzel.
- **Bildpfad** – Pfad mit führendem `/` in einer Bildeinbindung, bezogen auf das Bildverzeichnis.
- **Ungültiger Verweis** – interner Verweis ohne führenden `/`.
- **Workspace-Pfad** – interne Form ohne führenden `/`, z. B. `server/backup.md`; so auch in der API.

## 5. Dateikonventionen

### 5.1 Aufbau eines Workspace

```
beruf/                         ← Workspace-Wurzel
├── .docdump.toml              ← optionale Workspace-Konfiguration
├── img/                       ← Bildverzeichnis
│   ├── netzwerk.png
│   └── server/
│       └── rack.jpg
├── index.md
├── server/
│   ├── 01-einrichtung.md
│   └── 02-backup.md
└── netzwerk.md
```

### 5.2 Verweise

```markdown
![Netzwerkplan](/netzwerk.png)               ← Bildeinbindung: ab dem Bildverzeichnis
![Rack](/server/rack.jpg)                    ← Unterordner im Bildverzeichnis
[Backup einrichten](/server/02-backup.md)    ← Seitenlink: ab der Workspace-Wurzel
[Abschnitt Ports](/netzwerk.md#ports)
[Externe Seite](https://example.org)
```

Regeln:

1. Ein führender `/` hat zwei Bezugspunkte. Welcher gilt, entscheidet allein die Syntax, nicht die Dateiendung:
   - **Bildeinbindung** `![…](/…)` → Bildverzeichnis (Bildpfad). Der Name des Bildverzeichnisses steht nur in der Konfiguration, nie im Markdown.
   - **Seitenlink** `[…](/…)` → Workspace-Wurzel (Wurzelpfad). Das gilt auch für einen Link auf eine Bilddatei zum Herunterladen: `[Plan](/img/netzwerk.png)`.
2. Die App **schreibt** Verweise immer in dieser Form.
3. **Interne Verweise müssen immer mit `/` beginnen.** Relative Pfade (`bild.png`, `../server/a.md`) sind ungültige Verweise:
   - Die Vorschau markiert sie (Klasse `link-invalid`), statt sie aufzulösen.
   - Die Verweisanpassung lässt sie unverändert.
   - Eine spätere Verweisprüfung listet sie auf; für bestehende Dokumentation (z. B. aus MkDocs) gibt es später eine Umwandlung (6.2).
4. Externe URLs (`http:`, `https:`, `mailto:` usw.) und reine Anker (`#abschnitt`) bleiben unberührt.
5. Folgen:
   - Wird eine Seite verschoben, bleiben ihre eigenen Verweise gültig. Anzupassen sind nur Seitenlinks anderer Seiten, die auf sie zeigen.
   - Bildeinbindungen brechen beim Umbau von Seiten und Ordnern nie.
   - Wird das Bildverzeichnis umbenannt, ändert sich keine Bildeinbindung (5.3).
6. Kompatibilität mit anderen Programmen ist zweitrangig. GitHub oder VS Code zeigen die Bilder nicht an. Für die Weitergabe gibt es den Export (Kapitel 9), der Bild- und Wurzelpfade in relative Pfade umschreibt.

### 5.3 Bildverzeichnis

- Der Name steht in der Workspace-Konfiguration (`image_dir`, Standard `img`) und wird in Bildeinbindungen nie angegeben. Unterordner sind erlaubt.
- Hochgeladene oder aus der Zwischenablage eingefügte Bilder landen im Bildverzeichnis. Existiert der Name schon, hängt die App `-1`, `-2` usw. an.
- Das Bildverzeichnis erscheint nicht im Seitenbaum, sondern im Bilder-Browser (10.4).
- Umbenennen des Bildverzeichnisses (später): Verzeichnis verschieben, `image_dir` anpassen, Seitenlinks ins Bildverzeichnis nach 7.6 anpassen. Bildeinbindungen bleiben unverändert.
- Zulässige Formate im MVP: PNG, JPEG, GIF, WebP, SVG (zu SVG siehe 7.8).

### 5.4 Dateibrowser und ls-Befehl

Es gibt keine gespeicherte Reihenfolge. Der Dateibrowser wird über ein Eingabefeld eingestellt, in das man einen `ls`-Befehl tippt. Die Optionen haben die Bedeutung von GNU `ls` und gelten für jede Ebene des Baums.

```
$ ls -v --group-directories-first     ← Standard
$ ls -t                               ← zuletzt geänderte zuerst
$ ls -vlh -I '*.alt.md'               ← mit Größe und Datum, ohne alte Entwürfe
$ ls -v /server                       ← nur den Ordner server zeigen
```

**Sortierung.** Bei mehreren Sortieroptionen gilt die letzte, wie bei `ls`. Bei Gleichstand entscheidet der Name.

| Option | Wirkung |
|---|---|
| *(keine)* | Name, alphabetisch nach deutschen Sortierregeln |
| `-v`, `--sort=version` | Name, natürlich: `2-x.md` vor `10-x.md` |
| `-t`, `--sort=time` | Änderungsdatum, neueste zuerst |
| `-S`, `--sort=size` | Größe, größte zuerst |
| `-X`, `--sort=extension` | Endung, dann Name |
| `-r` | Reihenfolge umkehren |
| `--group-directories-first` | Ordner vor Dateien, unabhängig von der Sortierung |

**Anzeige und Filter:**

| Option | Wirkung |
|---|---|
| `-l` | Detailansicht: Größe und Änderungsdatum als Spalten |
| `-h` | mit `-l`: Größen lesbar (`1,2K`, `3,4M`) |
| `-F` | `/` hinter Ordnernamen |
| `-A`, `-a` | versteckte Einträge (Name beginnt mit `.`) anzeigen; `.git` und temporäre Dateien bleiben immer verborgen |
| `-I MUSTER`, `--ignore=MUSTER` | Einträge ausblenden, deren Name auf das Muster passt (`*`, `?`, `[…]`); mehrfach erlaubt |

**Pfad-Argument.** Ein Ordner, beginnend mit `/` wie alle Pfade, beschränkt den Dateibrowser auf diesen Ordner: `ls -v /server`.

**Regeln für die Eingabe:**

- `ls` am Anfang ist optional. `-vlh` ist dasselbe wie `-v -l -h`.
- Muster mit Sonderzeichen setzt man wie in der Shell in Anführungszeichen (`'*.alt.md'`).
- Unbekannte Optionen oder andere Fehler: Meldung unter dem Feld im Stil von `ls` (z. B. `ls: ungültige Option -- 'z'`). Der Baum behält die letzte gültige Einstellung.
- `--help` zeigt die unterstützten Optionen.
- Leeres Feld: Es gilt der Standard aus `.docdump.toml` (`ls = …`), ohne Eintrag dort `-v --group-directories-first`.
- Die Eingabe wird pro Workspace im Browser gespeichert, mit Verlauf (`↑`/`↓`).

**Hinweise:**

- Eine feste Reihenfolge entsteht über Nummern vor den Dateinamen (`01-…`) zusammen mit `-v`.
- Ausgewertet und sortiert wird im Frontend, ohne Serveranfrage (D10).
- Die Sortierung entspricht GNU `ls` in der Wirkung, nicht zeichengenau: Sie nutzt die Sortierregeln des Browsers (`Intl.Collator`).
- Das Bildverzeichnis erscheint nie im Dateibrowser (5.3).

### 5.5 Was angezeigt wird

- Ordner und `.md`-Dateien, immer mit ihrem echten, vollständigen Dateinamen, inklusive Endung und Nummern (`01-einrichtung.md`).
- **Ordnerseite:** Ein Klick auf einen Ordner klappt ihn auf und zeigt seine `index.md` im Anzeigebereich, falls vorhanden. `index.md` erscheint zusätzlich als normaler Eintrag im Baum.
- Versteckt: Einträge, die mit `.` beginnen (sichtbar mit `ls -A`, außer `.git` und temporären Dateien), sowie Muster aus `ignore` in `.docdump.toml` (immer verborgen).
- Andere Dateitypen (PDF usw.): siehe offene Entscheidung E3.

### 5.6 Konfiguration

**Serverkonfiguration** (`docdump.toml`, Pfad per `--config`), gehört nicht in einen Workspace:

```toml
listen = "127.0.0.1:8080"

[[workspace]]
id = "beruf"
name = "Beruf"
path = "/srv/doku/beruf"

[[workspace]]
id = "hobby"
name = "Hobby"
path = "/srv/doku/hobby"
poll = false        # true für Netzlaufwerke, siehe 7.4
```

**Workspace-Konfiguration** (`.docdump.toml` in der Workspace-Wurzel, optional):

```toml
image_dir = "img"
ignore = ["entwurf-*", "archiv/"]
ls = "-v --group-directories-first"   # Standard für den Dateibrowser (5.4)
ls_images = "-t"                      # optional: Standard für den Bilder-Browser (10.4); sonst gilt ls
```

### 5.7 Dateinamen

Beim Anlegen einer Seite oder eines Ordners in der App wird der eingegebene Name umgewandelt. Dasselbe gilt beim Hochladen von Bildern und beim Umbenennen in der App (D17):

- Kleinbuchstaben
- Leerzeichen → `-`, mehrere `-` hintereinander → ein `-`
- `ä ö ü ß` → `ae oe ue ss`
- alle übrigen Zeichen außer `a–z`, `0–9`, `-`, `_`, `.` werden entfernt
- bei Seiten wird `.md` ergänzt, falls es fehlt
- Beispiel: `Übersicht Netzwerk (alt)` → `uebersicht-netzwerk-alt.md`

Weitere Regeln:

- Die App zeigt den umgewandelten Namen vor dem Anlegen an.
- Beim Verschieben per Drag & drop bleibt der Name unverändert; umgewandelt wird nur ein neu eingegebener Name.
- Von außen angelegte Dateien behalten ihren Namen und werden voll unterstützt. Verweise auf sie schreibt die App mit URL-Kodierung (`/mein%20dokument.md`).
- Maßgeblich ist die Umwandlung im Backend (`core`). Das Frontend zeigt die Vorschau nach derselben Regel. Beide testen gegen dieselbe Tabelle von Testfällen (`fixtures/name-normalization.json`).

## 6. Funktionsumfang

### 6.1 MVP

- Mehrere Workspaces, Auswahl in der Oberfläche
- Dateibrowser, eingestellt über einen `ls`-Befehl
- Anzeigebereich: Vorschau, darüber per Taste oder Knopf der Ausklapp-Editor nach Art von Yakuake/Guake; Breite, Höhe, Ausrichtung und Deckkraft einstellbar
- Live-Aktualisierung bei Änderungen von außen
- Editor mit Vorschau, Speichern mit Konflikterkennung
- Seiten und Ordner anlegen, umbenennen, verschieben (Drag & drop), löschen
- Anpassung aller eingehenden Verweise bei Umbenennen und Verschieben
- Löschen mit Warnung, wenn noch Verweise auf das Ziel zeigen
- Bilder hochladen, aus der Zwischenablage einfügen, im Bilder-Browser ansehen und verwalten (umbenennen und verschieben mit Verweisanpassung, löschen mit Warnung)

### 6.2 Später

- Export (Kapitel 9)
- Volltextsuche
- Prüfung auf kaputte und ungültige Verweise sowie unbenutzte Bilder
- Umwandlung relativer Pfade in Bild- und Wurzelpfade, für das Übernehmen bestehender Dokumentation (z. B. aus MkDocs)
- Eingehende Verweise pro Seite anzeigen
- Git-Integration (E6)
- Anmeldung (E5)
- Idee: Befehle im Stil des ls-Felds für Dateioperationen (`mv`, `mkdir`, `rm`), ausgeführt mit denselben Regeln wie Drag & drop

### 6.3 Nicht-Ziele

- Datenbank jeder Art
- Gleichzeitiges Bearbeiten derselben Seite durch mehrere Personen in Echtzeit
- WYSIWYG-Editor (es gibt Editor + Vorschau)
- Eigene Markdown-Syntax
- Rechteverwaltung pro Seite

## 7. Architektur

### 7.1 Überblick

```
┌──────────────────────────┐        REST (JSON)        ┌──────────────────────────┐
│  Browser                 │ ────────────────────────▶ │  Server (Rust, axum)     │
│  Vue 3 · Pinia · Router  │                           │                          │
│  Tailwind · CodeMirror   │ ◀──────────────────────── │  ┌────────────────────┐  │
└──────────────────────────┘   Ereignisse (SSE)        │  │  core              │  │
                                                       │  │  Pfade · Baum      │  │
                                                       │  │  Operationen       │  │
                                                       │  │  Verweise · Render │  │
                                                       │  │  Überwachung       │  │
                                                       │  └─────────┬──────────┘  │
                                                       └────────────┼─────────────┘
                                                                    │ lesen/schreiben
                                                                    ▼  inotify
                                                           ┌──────────────────┐
                                                           │  Workspace(s)    │
                                                           │  Markdown + img  │
                                                           └──────────────────┘
```

### 7.2 Ablauf einer Aktion (Beispiel: Seite verschieben)

1. Benutzer zieht `netzwerk.md` in den Ordner `server/`.
2. Frontend sendet `POST /api/w/beruf/move` mit `from` und `to`.
3. Backend prüft beide Pfade (P5), prüft, dass das Ziel nicht existiert, und verschiebt die Datei.
4. Backend sucht alle Seiten mit Verweisen auf `/netzwerk.md` und schreibt diese Verweise auf `/server/netzwerk.md` um (7.6).
5. Backend antwortet mit der Liste der angepassten Dateien und allen Stellen, die es nicht anpassen konnte.
6. Die Dateiüberwachung meldet Umbenennung und Änderungen. Das Backend sendet daraus Ereignisse per SSE.
7. Der Pinia-Store wendet die Ereignisse an. Erst jetzt ändert sich der Baum (P2).
8. Frontend zeigt eine kurze Meldung („3 Verweise angepasst“) und ggf. die nicht anpassbaren Stellen.

### 7.3 Backend

Cargo-Workspace mit zwei Crates:

| Crate | Aufgabe |
|---|---|
| `core` | Gesamte Fachlogik, ohne HTTP. Gut testbar mit temporären Verzeichnissen. |
| `server` | axum-Routen, SSE, Auslieferung des gebauten Frontends (alle Pfade außer `/api` und `/raw` liefern `index.html`, damit der Router im History-Modus arbeitet), Laden der Konfiguration, Kommandozeile. |

Module in `core`:

| Modul | Aufgabe |
|---|---|
| `path` | Typ `WsPath`: geprüfter Workspace-Pfad. Einziger Weg, aus Benutzereingaben Dateipfade zu machen. |
| `config` | Server- und Workspace-Konfiguration lesen. |
| `tree` | Workspace einlesen, Einträge mit Typ, Größe, Änderungsdatum. Sortiert wird nicht hier, sondern im Frontend. |
| `ops` | Anlegen, Schreiben (atomar), Umbenennen, Verschieben, Löschen. |
| `links` | Verweise finden (mit Quellposition), auflösen, umschreiben; eingehende Verweise ermitteln. |
| `render` | Markdown → HTML für die Vorschau, inkl. Pfadumschreibung und Bereinigung. |
| `watch` | Dateiüberwachung, Bündelung, Umsetzung in Ereignisse. |
| `export` | Später: Export (Kapitel 9). |

Vorgeschlagene Crates (Versionen beim Projektstart prüfen und aktuelle stabile wählen):

| Zweck | Crate |
|---|---|
| HTTP, SSE | `axum`, `tokio`, `tower-http` |
| Dateiüberwachung | `notify`, `notify-debouncer-full` |
| Markdown | `pulldown-cmark` oder `comrak`: ein Parser für Verweise mit Quellposition **und** Vorschau mit Quellzeilen; Wahl vor M1 (D9) |
| HTML bereinigen | `ammonia` |
| Konfiguration | `serde`, `toml` |
| Inhalts-Hash | `blake3` |
| Frontend einbetten | `rust-embed` |
| Fehler, Logging | `thiserror`, `anyhow` (nur `server`), `tracing` |
| Tests | `tempfile` |

Auslieferung als **eine Programmdatei**, in die das gebaute Frontend eingebettet ist.

### 7.4 Dateiüberwachung

- `notify` nutzt unter Linux inotify. Jeder Workspace wird rekursiv überwacht; das Crate legt die nötigen Watches pro Unterverzeichnis an.
- `notify-debouncer-full` bündelt Ereignisse (Startwert 200 ms). Wichtig, weil Editoren über temporäre Dateien speichern und `git checkout` viele Ereignisse auf einmal erzeugt.
- Ereignisse zu ignorierten Pfaden (5.5) und zu eigenen temporären Dateien (7.5) werden verworfen.
- Gehen Ereignisse verloren (Überlauf der Warteschlange) oder meldet der Watcher einen Fehler, liest das Backend den Workspace neu ein und sendet `tree.resync`.
- **Netzlaufwerke:** inotify meldet keine Änderungen, die andere Rechner auf NFS/SMB vornehmen. Dafür gibt es pro Workspace `poll = true` (Polling-Watcher von `notify`).
- **Grenze:** `fs.inotify.max_user_watches` gilt pro Benutzer, für alle Programme zusammen. Je nach Kernel liegt der Standard teils nur bei 8192. Beim Start die Anzahl der Verzeichnisse loggen und bei Fehlern einen klaren Hinweis zur Erhöhung ausgeben.

### 7.5 Schreiben und Konflikte

- **Speichern nur ausdrücklich:** mit `Strg+S` oder Knopf (D17).
  - Ausblenden des Editors speichert nicht.
  - Beim Seitenwechsel mit ungespeicherten Änderungen fragt die App nach.
  - Beim Schließen des Browser-Tabs mit ungespeicherten Änderungen warnt der Browser (`beforeunload`).
  - Automatisches Speichern kommt später als Einstellung.
- **Atomar schreiben:** Inhalt in eine temporäre Datei im selben Verzeichnis schreiben (`.docdump-tmp-<zufall>`), dann umbenennen. Nie halb geschriebene Seiten.
- **Version:** Jede gelesene Datei bekommt eine Version (BLAKE3-Hash des Inhalts). Beim Speichern schickt das Frontend die Version mit, auf der die Bearbeitung beruht.
- **Konflikt:** Weicht die Version auf der Platte ab, antwortet das Backend mit `409` und dem aktuellen Inhalt. Das Frontend bietet an: Fremde Version laden, eigene Version speichern (überschreiben) oder beide nebeneinander anzeigen.
- **Ereignis zur offenen Seite:** Meldet die Überwachung eine Änderung an der gerade geöffneten Seite:
  - Version gleich der eigenen gespeicherten → ignorieren (eigenes Speichern).
  - Keine ungespeicherten Änderungen → still neu laden.
  - Ungespeicherte Änderungen → Hinweisleiste „Datei wurde außerhalb geändert“.

### 7.6 Verweise anpassen

**Auslöser:** Umbenennen oder Verschieben einer Seite, eines Ordners oder eines Bildes. Bilder werden nur innerhalb des Bildverzeichnisses verschoben.

**Ablauf:**

1. Abbildung alt → neu bilden. Bei Ordnern gilt sie für alle Pfade mit dem alten Präfix.
2. Datei bzw. Ordner im Dateisystem verschieben.
3. Alle Seiten des Workspace durchsuchen. Für jeden Verweis:
   1. Ziel nach 5.2 auflösen: Bildeinbindung mit `/` → Bildverzeichnis, Seitenlink mit `/` → Workspace-Wurzel. Ungültige Verweise (ohne `/`) werden übersprungen. URL-Kodierung wie `%20` dekodieren.
   2. Liegt das Ziel in der Abbildung, neuen Pfad in der Form nach 5.2 bilden und Anker (`#…`) übernehmen.
   3. Ersetzung an der Quellposition vormerken.
4. Pro Datei prüfen, ob sie seit dem Lesen unverändert ist (Version). Wenn ja, alle Ersetzungen von hinten nach vorne anwenden und atomar schreiben. Wenn nein, Datei auslassen und melden.
5. Ergebnis zurückgeben: geänderte Dateien, Anzahl Ersetzungen, nicht anpassbare Stellen.

**Regeln:**

- Ersetzt wird nur das Ziel eines Verweises, gefunden über die Quellposition des Parsers. Nie per Suchen/Ersetzen im ganzen Text, damit Codeblöcke und Fließtext unberührt bleiben.
- Referenzdefinitionen (`[ref]: /pfad.md`) werden ebenfalls angepasst.
- Lässt sich das Ziel innerhalb der Quellposition nicht eindeutig finden, wird die Stelle nicht geändert, sondern gemeldet.
- Verweise in rohem HTML (`<img src="…">`) werden im MVP nicht angepasst, aber gemeldet.
- Umgeschriebene Verweise werden immer in der Form nach 5.2 geschrieben.
- Abgelehnt mit `409`: Ziel existiert schon; Ordner soll in sich selbst verschoben werden.

**Bekanntes Risiko:** Verschieben plus Anpassen mehrerer Dateien ist nicht atomar. Bricht der Vorgang ab, bleibt ein Teil der Verweise alt. Gegenmittel:
- Erst verschieben, dann anpassen.
- Jeden Schritt loggen.
- Später eine Verweisprüfung, die solche Reste findet.

**Leistung:** Jede Seite bei jedem Verschieben zu lesen genügt für einige tausend Seiten. Ein Verweisindex im Speicher ist eine spätere Optimierung und muss P1 einhalten.

### 7.7 Vorschau

- Gerendert wird im Backend mit demselben Parser, der auch die Verweise findet: `POST /api/w/{ws}/render`. Das Frontend sendet beim Tippen gebündelt (Startwert 250 ms).
- Blockelemente (Absätze, Überschriften, Listen, Tabellen, Codeblöcke, Bilder) tragen ihre Quellzeilen als `data-sourcepos`.
  - Damit springt der Ausklapp-Editor beim Öffnen an die sichtbare Stelle, und die Vorschau folgt dem Cursor (10.3).
  - `ammonia` muss so eingestellt sein, dass `data-sourcepos` erhalten bleibt.
- Pfade werden beim Rendern umgeschrieben:
  - Bildeinbindungen `/x.png` → `/raw/{ws}/<image_dir>/x.png`
  - Seitenlinks auf Seiten `/server/backup.md` → `/w/{ws}/doc/server/backup.md`, mit `data-internal`, damit der Router den Klick übernimmt
  - Seitenlinks auf andere Dateien `/img/plan.png` → `/raw/{ws}/img/plan.png`
- Verweise auf nicht existierende Ziele bekommen die Klasse `link-broken`, ungültige Verweise (ohne `/`) die Klasse `link-invalid`.
- HTML wird mit `ammonia` bereinigt (Schutz vor Skripten in Markdown-Dateien).

### 7.8 Sicherheit

- **Pfade:** Jeder Pfad aus einer Anfrage wird zu `WsPath`: kein `..`, keine absoluten Pfade, keine Steuerzeichen. Danach wird geprüft, dass der aufgelöste Pfad im Workspace liegt. Symlinks, die aus dem Workspace hinausführen, werden nicht verfolgt.
- **Netz:** Standardmäßig nur `127.0.0.1`. Für Zugriff von außen: Reverse Proxy mit Anmeldung, bis E5 entschieden ist.
- **HTML:** Vorschau-HTML wird bereinigt (7.7).
- **Uploads:** Größenlimit (Startwert 20 MB), nur zugelassene Formate (5.3).
- **SVG:** kann Skripte enthalten. Ausliefern unter `/raw/` mit `Content-Security-Policy: sandbox` und `X-Content-Type-Options: nosniff`.

## 8. Schnittstelle

Alle Pfade in der API sind Workspace-Pfade ohne führenden `/`. Einzige Ausnahme: Der Bild-Upload antwortet mit dem Bildpfad, der direkt ins Markdown eingefügt wird. `{*path}` steht für einen Pfad mit beliebig vielen Segmenten.

### 8.1 REST

| Methode | Pfad | Zweck | Antwort |
|---|---|---|---|
| GET | `/api/workspaces` | Liste der Workspaces | `[{id, name}]` |
| GET | `/api/w/{ws}/config` | Workspace-Konfiguration | Inhalt `.docdump.toml` als JSON, mit Standardwerten |
| GET | `/api/w/{ws}/tree` | Ganzer Baum | `{entries: [{path, kind, size, mtime}]}` |
| GET | `/api/w/{ws}/files/{*path}` | Seite lesen | `{content, version, mtime}` |
| PUT | `/api/w/{ws}/files/{*path}` | Seite speichern | Body `{content, baseVersion}` → `{version}` oder `409 {content, version}` |
| POST | `/api/w/{ws}/files/{*path}` | Seite anlegen | Body `{content?}` → `201 {path}` mit umgewandeltem Namen (5.7) oder `409` |
| POST | `/api/w/{ws}/dirs/{*path}` | Ordner anlegen | `201 {path}` mit umgewandeltem Namen (5.7) oder `409` |
| POST | `/api/w/{ws}/move` | Verschieben/Umbenennen | Body `{from, to, rename: bool}` → `{to, rewritten: [{path, count}], unresolved: [{path, line, target}]}`; bei `rename` wird der neue Name umgewandelt (5.7) |
| DELETE | `/api/w/{ws}/entries/{*path}` | Löschen | `409 {backlinks}` wenn verwiesen, außer mit `?force=true` |
| GET | `/api/w/{ws}/backlinks/{*path}` | Eingehende Verweise | `[{path, line}]` |
| POST | `/api/w/{ws}/images` | Bild hochladen (multipart) | `{path}` als Bildpfad, z. B. `/foto-1.png` |
| POST | `/api/w/{ws}/render` | Vorschau rendern | Body `{markdown, path}` → `{html}` |
| GET | `/raw/{ws}/{*path}` | Datei roh ausliefern (Bilder) | Datei |
| GET | `/api/w/{ws}/events` | Ereignisstrom (SSE) | siehe 8.2 |

Fehler einheitlich als `{error: {code, message}}` mit passendem HTTP-Status.

### 8.2 Ereignisse (SSE)

| Typ | Daten | Bedeutung |
|---|---|---|
| `entry.created` | `{path, kind, size, mtime}` | Datei oder Ordner neu |
| `entry.removed` | `{path}` | gelöscht |
| `entry.renamed` | `{from, to}` | umbenannt/verschoben (bei Ordnern: ein Ereignis für den Ordner) |
| `file.modified` | `{path, version, mtime}` | Inhalt geändert |
| `tree.resync` | `{}` | Frontend soll den ganzen Baum neu laden |

Nach jedem (Neu-)Verbinden lädt das Frontend den ganzen Baum neu, weil Ereignisse während der Trennung verloren sein können.

## 9. Export (später)

Der Export verändert nie den Workspace. Er arbeitet auf einer Kopie und liefert eine Datei zum Herunterladen.

| Ziel | Inhalt |
|---|---|
| **ZIP mit relativen Pfaden** | Workspace-Kopie, alle Bild- und Wurzelpfade in relative Pfade umgeschrieben (aus `server/a.md`: Bildpfad `/x.png` → `../img/x.png`, Wurzelpfad `/netzwerk.md` → `../netzwerk.md`). Funktioniert danach in GitHub, VS Code und jedem Markdown-Programm. |
| **Statisches HTML** | Gerenderte Seiten mit Navigation aus dem Baum, ohne Server lesbar. |
| **Zensical/MkDocs-Ordner** | Wie ZIP, plus Konfigurationsdatei mit `nav` aus dem sortierten Baum. |

## 10. Frontend

### 10.1 Stack

- Vue 3 mit Composition API und `<script setup>`
- TypeScript (Vorschlag)
- Vite
- Pinia
- Vue Router
- Tailwind CSS mit Typography-Plugin für die Vorschau (`prose`)
- CodeMirror 6 mit Markdown-Modus als Editor

Im Entwicklungsbetrieb leitet Vite `/api` und `/raw` an den Rust-Server weiter.

### 10.2 Layout

```
┌────────────────────────────────────┬─────────────────────────────────────────────────┐
│ Beruf ▾                            │ 01-einrichtung.md ●                Strg+^ [</>] │
│ $ ls -v --group-directories-first  ├─────────────────────────────────────────────────┤
│                                    │ Einri│ # Einrichtung                     │      │
│ ▣ Bilder                           │ ━━━━━│                                   │      │
│ ▾ server/                          │ Zuers│ Zuerst das Netz, dann der Server. │      │
│     01-einrichtung.md              │ Danac│ ![Plan](/netzwerk.png)▌           │      │
│     02-backup.md                   │      ╰─────────────────────────────── ⚙ ─╯      │
│   index.md                         │                                                 │
│   netzwerk.md                      │ ┌──────────────────┐                            │
│                                    │ │   netzwerk.png   │                            │
│                                    │ └──────────────────┘                            │
└────────────────────────────────────┴─────────────────────────────────────────────────┘
 Dateibrowser                         Editor liegt über der Vorschau
```

- **Links: Dateibrowser.**
  - Oben die Workspace-Wahl, darunter das ls-Feld, darunter der feste Eintrag **Bilder** (10.4) und der Baum.
  - Eingestellt wird er nur über das ls-Feld (5.4).
  - Breite verstellbar.
- **Rechts: Anzeigebereich.**
  - Kopfzeile: Pfad der Seite, `●` bei ungespeicherten Änderungen, Hinweis auf das Tastenkürzel und ein kleiner Knopf `[</>]` für den Editor.
  - Grundfläche ist die Vorschau. Darüber legt sich bei Bedarf der Ausklapp-Editor (10.3).

### 10.3 Ausklapp-Editor und Ansichtsmodi

Vorbild sind Drop-down-Terminals wie Yakuake oder Guake. Der Editor mit dem Code der Seite gleitet von oben herein und legt sich über die Vorschau. Er verdrängt sie nicht: Die Vorschau bleibt darunter unverändert und ist neben, unter oder (bei Halbtransparenz) durch den Editor sichtbar.

**Ansichtsmodi:**

| Modus | Im Code | Zustand des Ausklapp-Editors | Bearbeitbar |
|---|---|---|---|
| Vorschau | `preview` | ausgeblendet, nur gerendertes Markdown | nein |
| Überlagert | `overlay` | eingeblendet in der eingestellten Größe über der Vorschau | ja |
| Code | `code` | maximiert: volle Größe, voll deckend | ja |

**Einstellungen** (im Browser gespeichert):

| Einstellung | Bereich | Standard | Einstellen über |
|---|---|---|---|
| Breite | 20–100 % des Anzeigebereichs | 100 % | seitliche Ränder ziehen oder Menü |
| Höhe | 20–100 % des Anzeigebereichs | 50 % | Unterkante ziehen oder Menü |
| Ausrichtung | links · Mitte · rechts (immer am oberen Rand) | Mitte | Menü |
| Deckkraft des Hintergrunds | 50–100 % | 100 % | Menü |
| Weichzeichner für die durchscheinende Vorschau | an · aus | aus | Menü |

- Das Menü öffnet sich über das Zahnrad `⚙` an der Unterkante des Editors.
- Mit Breite 50 %, Höhe 100 % und Ausrichtung rechts entsteht eine Ansicht mit Code und Vorschau nebeneinander.
- Bei Halbtransparenz wird nur der Hintergrund durchscheinend. Der Code-Text bleibt voll deckend und gut lesbar.
- Bei schmalem Fenster gilt immer die volle Breite.

**Bedienung:**

- **Ein- und ausblenden:** `Strg+^` oder Klick auf den Knopf `[</>]` in der Kopfzeile.
  - Gemeint ist die Taste links neben der `1` (`event.code === "Backquote"`), unabhängig vom Tastaturlayout. Sie liegt an der Stelle der Quake-Konsole und entspricht dem Terminal in VS Code.
  - `F12` wie bei Yakuake scheidet aus, weil Browser damit die Entwicklerwerkzeuge öffnen.
- **Maximieren:** `Strg+Umschalt+^` oder Doppelklick auf die Unterkante. Nochmals: zurück zur eingestellten Größe.
- Das Ein- und Ausblenden ist kurz animiert (ca. 150 ms). Bei `prefers-reduced-motion` erscheint der Editor ohne Animation.

**Verhalten:**

- Der Editor überdeckt nur den Anzeigebereich. Der Dateibrowser bleibt bedienbar, auch für Drag & drop.
- Klicks und Scrollen in sichtbaren Teilen der Vorschau neben oder unter dem Editor wirken auf die Vorschau. Der Editor bleibt dabei offen.
- **Öffnen:** Der Editor bekommt den Fokus und springt an die Quellzeile des obersten sichtbaren Blocks der Vorschau. Man landet also dort, wo man gerade liest.
- **Tippen:** Die Vorschau aktualisiert sich laufend und scrollt den Block mit dem Cursor in den freien Bereich:
  - unter den Editor, wenn dort mindestens ein Viertel der Höhe frei ist,
  - sonst auf Höhe des Cursors, sichtbar neben dem Editor oder durch ihn hindurch,
  - im Modus Code entfällt das Mitscrollen.
- **Ausblenden:** Der Inhalt bleibt erhalten. Ungespeicherte Änderungen zeigt `●` in der Kopfzeile; die Vorschau zeigt den aktuellen Stand im Editor.
- **Seitenwechsel bei offenem Editor:** Der Editor bleibt offen und zeigt den Code der neuen Seite. Bei ungespeicherten Änderungen fragt die App vorher nach.
- Der Modus steht in der URL (`?view=overlay`), damit ein Link einen Modus mitgeben kann. Ohne Angabe gilt der zuletzt genutzte Modus, beim ersten Besuch Vorschau.

### 10.4 Bilder-Browser

```
┌────────────────────────────────────┬─────────────────────────────────────────────────┐
│ Beruf ▾                            │ Bilder: /server                     [Hochladen] │
│ $ ls -t                            ├─────────────────────────────────────────────────┤
│                                    │                                                 │
│ ◂ Seiten                           │  ┌───────┐  ┌───────┐                           │
│ ▾ server/                          │  │  ▒▒▒  │  │  ▒▒▒  │                           │
│     rack.jpg                       │  └───────┘  └───────┘                           │
│     ports.png                      │  rack.jpg   ports.png                           │
│   netz.png                         │                                                 │
│   plan.svg                         │ ─────────────────────────────────────────────── │
│                                    │ /server/rack.jpg · 2,1M · 1600×1200             │
│                                    │ Verwendet in: /server/01-einrichtung.md         │
│                                    │ [Pfad kopieren] [Umbenennen] [Löschen]          │
└────────────────────────────────────┴─────────────────────────────────────────────────┘
 Bildverzeichnis                      Raster und Details zum gewählten Bild
```

- **Wechsel:**
  - Klick auf den festen Eintrag **Bilder** über dem Baum. Der Dateibrowser zeigt dann das Bildverzeichnis, der Anzeigebereich den Bilder-Browser.
  - Zurück über **Seiten** an derselben Stelle.
- **Links, Baum des Bildverzeichnisses:**
  - Unterordner und Bilddateien mit echten Dateinamen.
  - Es gilt ein eigener ls-Befehl: Standard `ls_images` aus `.docdump.toml`, sonst wie bei den Seiten. Beispiel: `ls -t` zeigt die neuesten Bilder zuerst.
  - Drag & drop verschiebt Bilder und Ordner innerhalb des Bildverzeichnisses, mit Verweisanpassung (7.6).
- **Rechts, Raster:**
  - Vorschaubilder des gewählten Ordners mit Dateinamen, sortiert nach demselben ls-Befehl.
  - Bilder werden verzögert geladen (`loading="lazy"`) und vom Browser verkleinert.
  - Hochladen per Drag & drop ins Raster oder über den Knopf `[Hochladen]`. Ziel ist der gewählte Ordner.
- **Rechts, Details** (Klick auf ein Bild im Raster oder im Baum):
  - großes Bild
  - Bildpfad (z. B. `/server/rack.jpg`) mit Knopf zum Kopieren
  - Größe, Abmessungen, Änderungsdatum
  - eingehende Verweise als Links zu den Seiten; ohne Verweise der Hinweis „nicht verwendet“ (E8)
  - Umbenennen (mit Namensumwandlung 5.7 und Verweisanpassung) und Löschen (mit Warnung bei Verweisen)
  - `Esc` oder der Ordnername in der Kopfzeile führt zurück zum Raster.
- **Im Editor:** Beim Tippen von `![…](/` schlägt die Autovervollständigung Bildpfade vor, bei `[…](/` Seiten. Für ein vorhandenes Bild braucht man den Bilder-Browser also nicht.
- Vorschaubilder auf dem Server zu erzeugen, ist eine spätere Optimierung. Sie darf P1 nicht verletzen (z. B. nur im Speicher halten).

### 10.5 Routen

| Route | Ansicht |
|---|---|
| `/` | Workspace-Auswahl; bei nur einem Workspace direkt weiter |
| `/w/:ws` | Startseite des Workspace (`index.md`, falls vorhanden) |
| `/w/:ws/doc/:path(.*)` | Seite im Anzeigebereich; Query `view` = `preview` · `overlay` · `code` |
| `/w/:ws/images/:path(.*)?` | Bilder-Browser: Ordner im Bildverzeichnis (Raster) oder einzelnes Bild (Details) |

Es gibt keine getrennten Routen für Lesen und Bearbeiten; der Modus ist nur Darstellung.

### 10.6 Stores und Module

| Store | Inhalt |
|---|---|
| `workspaces` | Liste, aktueller Workspace, dessen Konfiguration |
| `tree` | Einträge als flache Map `path → entry`; sortierter und gefilterter Baum als berechneter Wert aus den ls-Optionen, getrennt für Seiten und Bildverzeichnis; wendet Ereignisse an |
| `document` | Geöffnete Seite: Inhalt, Basisversion, ungespeicherte Änderungen, Konfliktzustand |
| `settings` | ls-Eingabe pro Workspace und Bereich (Seiten, Bilder) mit Verlauf, zuletzt genutzter Ansichtsmodus, Einstellungen des Ausklapp-Editors (10.3), Theme; im `localStorage` gespeichert |
| `notifications` | Kurze Meldungen (z. B. „3 Verweise angepasst“) |

- Der ls-Befehl wird in einem eigenen Modul ohne Vue-Abhängigkeit verarbeitet (`src/ls/`).
  - Funktionen: `parseLs(input)` liefert Optionen oder einen Fehler; dazu Vergleichs- und Filterfunktionen für den Baum.
  - Vollständig mit Vitest testbar.
- Die SSE-Verbindung ist ein Composable (`useEvents`). Es öffnet pro Workspace eine `EventSource` und verteilt die Ereignisse an `tree` und `document`.

### 10.7 Komponenten

- `AppShell` – Grundlayout mit Dateibrowser und Anzeigebereich, verschiebbare Trennlinie
- `WorkspaceSwitcher`
- `LsInput` – Eingabefeld mit Prompt `$ ls`, Fehlermeldung, Verlauf, `--help`
- `TreeView`, `TreeNode` – Baum mit Drag & drop, Kontextmenü (neu, umbenennen, löschen); Spalten bei `-l`, Kennzeichen bei `-F`; dient für Seiten und Bildverzeichnis
- `SectionEntry` – fester Eintrag **Bilder** bzw. **Seiten** über dem Baum
- `DocHeader` – Pfad, Kennzeichen für ungespeicherte Änderungen, Hinweis auf das Tastenkürzel, kleiner Knopf `[</>]` zum Ein- und Ausblenden des Editors
- `DocPane` – Vorschau als Grundfläche, darüber `DropDownEditor`
- `DropDownEditor` – Ausklapp-Editor: Ein- und Ausblenden, Maximieren, ziehbare Ränder, Ausrichtung, Deckkraft und Weichzeichner, Menü an der Unterkante, Sprung zur sichtbaren Stelle; enthält `EditorPane`
- `EditorPane` – CodeMirror; Einfügen von Bildern aus der Zwischenablage und per Drag & drop; Autovervollständigung für Bildpfade und Seitenlinks
- `PreviewPane` – gerenderte Seite; scrollt zu einer Quellzeile
- `ConflictBanner`, `ConfirmDialog`, `Toast`
- `ImageGrid` – Raster der Vorschaubilder eines Ordners, Hochladen per Drag & drop oder Knopf
- `ImageDetail` – großes Bild, Bildpfad zum Kopieren, Eigenschaften, eingehende Verweise, Umbenennen, Löschen

### 10.8 Design und Tastatur

- **Klares Design:** wenige Farben, gute Typografie, keine Symbolfluten, Hell- und Dunkelmodus.
- **Schrift:** Dateibrowser, ls-Feld und Ausklapp-Editor in Monospace, passend zum Terminal-Charakter (Vorschlag).
- **Ausklapp-Editor:** Schatten an den Rändern, damit klar ist, was darüber liegt. Bei Halbtransparenz bleibt der Text deckend; der Weichzeichner verhindert, dass sich Code und gerenderter Text optisch vermischen.
- **Tastatur** (Vorschlag; vor der Umsetzung auf Konflikte mit Browser-Kürzeln prüfen):
  - `Strg+^` Ausklapp-Editor ein/aus
  - `Strg+Umschalt+^` Editor maximieren bzw. zurück zur eingestellten Größe
  - `Strg+S` speichern
  - `Strg+K` Seite suchen (Dateiname, später Volltext)
  - Im ls-Feld: `Enter` anwenden, `Esc` verwerfen, `↑`/`↓` Verlauf

### 10.9 Bibliotheken

Wenige, bewusst gewählte Abhängigkeiten (Vorschlag, D19). Versionen beim Projektstart prüfen.

| Zweck | Bibliothek | Begründung |
|---|---|---|
| Editor | CodeMirror 6 mit Markdown-Sprache, Autovervollständigung und Suche | modular und leicht, guter Markdown-Modus, funktioniert auch auf Touch-Geräten. Monaco (der Editor von VS Code) wäre deutlich schwerer und auf Mobilgeräten schwach. |
| Hilfsfunktionen | VueUse | fertige Composables für `localStorage`, Tastenkürzel, Größenänderungen und `EventSource`; spart viel eigenen Code |
| Vorschau-Typografie | `@tailwindcss/typography` | `prose`-Klassen für gerendertes Markdown |
| Symbole | Lucide (Vue-Paket) | wenige, schlichte Symbole, einzeln importiert |
| Tests | Vitest, Vue Test Utils | Standard im Vite-Umfeld |

Bewusst ohne Bibliothek:

- **Baum mit Drag & drop:** natives HTML5-Drag-and-drop. Fertige Baum-Komponenten verwalten meist eigenen Zustand, das widerspricht P2.
- **ls-Befehl:** Zerlegen der Eingabe (inkl. Anführungszeichen) und Muster (`*`, `?`, `[…]`) als eigenes kleines Modul, gut testbar.
- **Natürliche Sortierung:** `Intl.Collator` des Browsers.
- **Ausklapp-Editor, ziehbare Ränder, Dialoge, Meldungen:** eigene kleine Komponenten.
- **Markdown im Browser:** keine Bibliothek, gerendert wird im Backend (D9).

## 11. Projektstruktur

```
/
├── CLAUDE.md
├── README.md
├── .github/
│   └── pull_request_template.md
├── docs/
│   ├── konzept.md
│   ├── entscheidungen.md
│   └── begriffe.md
├── backend/
│   ├── Cargo.toml              ← Cargo-Workspace
│   └── crates/
│       ├── core/
│       └── server/
├── frontend/
│   ├── package.json
│   ├── vite.config.ts
│   └── src/
│       ├── api/                ← REST-Client, Typen
│       ├── components/
│       ├── composables/
│       ├── router/
│       ├── stores/
│       └── views/
├── fixtures/                   ← gemeinsame Testdaten für Backend und Frontend
└── examples/
    └── workspace/              ← Beispiel-Workspace für Entwicklung und Tests
```

## 12. Meilensteine

Jeder Meilenstein ist erst fertig, wenn seine Abnahmekriterien erfüllt und getestet sind. Umgesetzt wird jeder Meilenstein in mehreren kleinen Pull Requests, je einer pro abgeschlossenem Schritt (Arbeitsweise siehe `CLAUDE.md`).

**M0 – Gerüst**
- [ ] Cargo-Workspace mit `core` und `server`, `cargo test` läuft.
- [ ] Vite-Projekt mit Vue, Pinia, Router, Tailwind; Dev-Proxy auf den Server.
- [ ] Server lädt `docdump.toml` und liefert `/api/workspaces`.
- [ ] Beispiel-Workspace unter `examples/workspace`.

**M1 – Lesen**
- [ ] `WsPath` mit Tests für alle Ausbruchsversuche (`..`, absolute Pfade, Symlinks nach außen).
- [ ] Dateibrowser mit ls-Feld: alle Optionen aus 5.4; Parser, Sortierung und Filter mit Vitest getestet.
- [ ] Anzeigebereich im Modus Vorschau, Modus in der URL.
- [ ] Vorschau-HTML mit Quellzeilen (`data-sourcepos`), die die Bereinigung überstehen.
- [ ] Vorschau mit Bildern, internen Links (Router) und Markierung kaputter und ungültiger Verweise.

**M2 – Live**
- [ ] Änderung, Anlegen, Löschen, Umbenennen mit externem Editor oder `mv` erscheint ohne Neuladen.
- [ ] `git checkout` eines anderen Stands führt zu korrektem Baum (Bündelung bzw. `tree.resync`).
- [ ] Neu verbinden nach Serverneustart lädt den Baum neu.

**M3 – Schreiben**
- [ ] Ausklapp-Editor: Ein- und Ausblenden per Taste und Knopf; Breite, Höhe, Ausrichtung, Deckkraft und Weichzeichner einstellbar und gespeichert; Maximieren.
- [ ] Beim Öffnen Sprung zur sichtbaren Stelle; Vorschau folgt dem Cursor.
- [ ] Ein- und Ausblenden und Seitenwechsel verlieren keine ungespeicherten Änderungen.
- [ ] Speichern atomar und mit Version.
- [ ] Konfliktfall (Datei extern geändert) zeigt Hinweisleiste und `409`-Dialog.
- [ ] Seiten und Ordner anlegen.
- [ ] Bild per Upload, Zwischenablage oder Drag & drop: landet im Bildverzeichnis, Verweis wird an der Cursorposition eingefügt.
- [ ] Namensumwandlung (5.7) in Backend und Frontend, getestet gegen `fixtures/name-normalization.json`.
- [ ] Bilder-Browser: Wechsel über **Bilder**, Baum mit eigenem ls-Befehl, Raster, Details mit Bildpfad zum Kopieren, Hochladen.
- [ ] Autovervollständigung für Bildpfade und Seitenlinks im Editor.

**M4 – Umbauen**
- [ ] Drag & drop im Baum verschiebt Seiten und Ordner; Umbenennen per Kontextmenü.
- [ ] Alle eingehenden Verweise werden angepasst, inkl. Anker und Referenzdefinitionen.
- [ ] Ungültige Verweise (ohne `/`) bleiben unverändert (Test).
- [ ] Codeblöcke bleiben unberührt (Test).
- [ ] Nicht anpassbare Stellen werden gemeldet.
- [ ] Löschen mit Verweiswarnung.
- [ ] Bilder-Browser: Bilder umbenennen, verschieben und löschen; eingehende Verweise in den Details.

**M5 – Export**
- [ ] ZIP mit relativen Pfaden.
- [ ] Danach statisches HTML.

## 13. Offene Entscheidungen

E1, E2, E4, E7 und E10 sind entschieden (D17). Die Nummern der übrigen bleiben unverändert.

| Nr. | Frage | Optionen | Vorschlag |
|---|---|---|---|
| E3 | Andere Dateitypen (PDF, Office) | ausblenden · anzeigen und herunterladen · in eigenem Verzeichnis wie Bilder | anzeigen und herunterladen |
| E5 | Anmeldung | keine (nur localhost/Proxy) · ein Passwort · Benutzerkonten | MVP ohne, danach ein Passwort |
| E6 | Git | keine · Commit bei jedem Speichern · Commit auf Knopfdruck | später, Commit auf Knopfdruck |
| E8 | Unbenutzte Bilder | ignorieren · im Bilder-Browser markieren | markieren (mit der Verweisprüfung) |
| E9 | Sprache der Oberfläche | nur Deutsch · Deutsch mit Vorbereitung für Übersetzung | Deutsch, Texte von Anfang an zentral ablegen |
| E11 | Gespeicherte Voreinstellungen für den Editor (z. B. „oben halb“, „rechts halb“) mit eigenem Tastenkürzel | nein · ja | später, falls oft umgestellt wird |

## 14. Risiken

| Risiko | Gegenmaßnahme |
|---|---|
| Verschieben mit Verweisanpassung ist nicht atomar | Reihenfolge und Logging (7.6), später Verweisprüfung |
| inotify-Grenze erreicht | klare Fehlermeldung mit Anleitung; Polling als Ausweg |
| Netzlaufwerke melden keine Änderungen | `poll = true` pro Workspace |
| Externe Änderung während einer Umbau-Aktion | Anpassung prüft die Version jeder Datei vor dem Schreiben; bei Abweichung Datei auslassen und melden |
| Große Workspaces machen das Durchsuchen beim Verschieben langsam | messen; bei Bedarf Verweisindex im Speicher (P1-konform) |
| Viele große Bilder machen das Raster im Bilder-Browser langsam | verzögertes Laden; später Vorschaubilder im Speicher |
