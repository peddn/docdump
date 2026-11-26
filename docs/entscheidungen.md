# Entscheidungen

Kurze Entscheidungsprotokolle. Jede Entscheidung hat einen Status:

- **Festgelegt:** vom Projektinhaber entschieden. Nur nach Rücksprache ändern.
- **Vorgeschlagen:** im Konzept als sinnvoller Standard gesetzt. Darf begründet geändert werden; dann hier nachtragen.

Offene Fragen stehen in [konzept.md, Kapitel 13](konzept.md#13-offene-entscheidungen).

---

## D1 – Dateien sind die einzige Wahrheit

**Status:** Festgelegt

**Kontext:** Bisherige Werkzeuge (MkDocs, TiddlyWiki) passten nicht. Andere Wikis legen Struktur oder Inhalte in Datenbanken ab.

**Entscheidung:** Inhalte sind Markdown-Dateien in einem Ordner. Die Oberfläche spiegelt den Ordner; jede Aktion in der Oberfläche ist eine Dateioperation. Es gibt keine Datenbank und keinen dauerhaften Cache.

**Folgen:**
- Der Ordner ist ohne die App vollständig nutzbar (Editor, Git, Backup).
- Abgeleiteter Zustand (Baum, Verweise) wird nur im Speicher gehalten und muss jederzeit neu aufbaubar sein.

## D2 – Server-Anwendung in Rust

**Status:** Festgelegt

**Kontext:** PHP kommt nicht in Frage. Gewünscht ist eine Server-Anwendung, keine Desktop-App.

**Entscheidung:**
- Backend in Rust, Zugriff über den Browser (festgelegt).
- Auslieferung als eine Programmdatei mit eingebettetem Frontend und HTTP-Framework axum (vorgeschlagen).

**Folgen:** Einfaches Deployment (eine Datei plus Konfiguration). Mehrere Geräte greifen auf denselben Stand zu.

## D3 – Frontend mit Vue 3, Pinia, Vue Router und Tailwind CSS

**Status:** Festgelegt

**Ergänzungen:** TypeScript, Vite und CodeMirror 6 als Editor sind **vorgeschlagen**.

## D4 – Pfade mit führendem `/`, Bildverzeichnis nur in der Konfiguration

**Status:** Festgelegt (Detailregel unten: Vorgeschlagen)

**Kontext:** Bilder liegen in einem eigenen Verzeichnis. Relative Pfade (`../../img/x.png`) hängen von der Lage der Seite ab und müssten beim Verschieben angepasst werden.

**Entscheidung:**
- Das Bildverzeichnis steht in der Workspace-Konfiguration (`image_dir`, Standard `img`) und wird in Pfaden nie angegeben.
- Bildeinbindungen beziehen sich mit führendem `/` auf das Bildverzeichnis: `![Plan](/netzwerk.png)`.
- Seitenlinks beziehen sich mit führendem `/` auf die Workspace-Wurzel: `[Backup](/server/backup.md)`.
- **Interne Verweise müssen immer mit `/` beginnen.** Relative Pfade gelten als ungültige Verweise: Die Vorschau markiert sie, die Verweisanpassung lässt sie unverändert. Relative Pfade entstehen nur im Export.

**Detailregel (Vorgeschlagen):** Welcher Bezugspunkt gilt, entscheidet allein die Syntax (`![…]()` oder `[…]()`), nicht die Dateiendung. Ein Link auf eine Bilddatei zum Herunterladen nennt deshalb den Ordner: `[Plan](/img/netzwerk.png)`. Grund: Eine Regel nach Dateiendung wäre schwerer vorherzusehen und bräuchte eine Liste von Endungen.

**Folgen:**
- Eine verschobene Seite behält gültige Verweise; anzupassen sind nur Seitenlinks, die auf sie zeigen.
- Die Auflösung eines Verweises ist einfach und eindeutig: Es gibt keine Pfade, die von der Lage der Seite abhängen.
- Bestehende Dokumentation mit relativen Pfaden (z. B. aus MkDocs) muss beim Übernehmen umgewandelt werden. Dafür ist eine Umwandlungsfunktion vorgesehen (Konzept 6.2).
- Bildeinbindungen brechen beim Umbau von Seiten nie. Umbenennen des Bildverzeichnisses ändert keine Bildeinbindung, nur die Konfiguration.
- Andere Programme (GitHub, VS Code) zeigen die Bilder nicht an. Bewusst in Kauf genommen; der Export stellt die Kompatibilität her (D5).

## D5 – Kompatibilität über Export

**Status:** Festgelegt

**Entscheidung:** Ein Export erzeugt eine Kopie mit relativen Pfaden (ZIP), später statisches HTML und einen Zensical/MkDocs-Ordner. Der Workspace selbst wird dabei nie verändert.

## D6 – Dateibrowser über einen ls-Befehl

**Status:** Festgelegt (Ansatz) / Vorgeschlagen (Umfang der Optionen, Pfad-Argument, Standard)

**Kontext:** Es soll keine gespeicherte Reihenfolge geben. Sortiert wird nach Regeln, so flexibel wie bei `ls`.

**Entscheidung:**
- Der Dateibrowser wird über ein Eingabefeld eingestellt, in das ein `ls`-Befehl getippt wird. Es gibt keine Reihenfolgedatei und kein Sortiermenü.
- Die unterstützten Optionen haben die Bedeutung von GNU `ls` (Konzept 5.4).
- Der Standard pro Workspace steht in derselben Schreibweise in `.docdump.toml`: `ls = "-v --group-directories-first"`.

**Folgen:**
- Eine feste Reihenfolge entsteht bei Bedarf über Nummern im Dateinamen zusammen mit `-v`.
- Wer `ls` kennt, muss nichts Neues lernen.
- Die Sortierung entspricht GNU `ls` in der Wirkung, aber nicht zeichengenau, weil sie die Sortierregeln des Browsers nutzt.

## D7 – Dateiüberwachung über inotify, Oberfläche nur über Ereignisse

**Status:** Festgelegt (inotify) / Vorgeschlagen (nur über Ereignisse)

**Entscheidung:**
- Das Backend überwacht die Workspaces mit `notify` (unter Linux inotify).
- Die Oberfläche ändert den Baum ausschließlich aufgrund dieser Ereignisse, auch bei eigenen Aktionen.

**Folgen:**
- Es gibt nur einen Weg, auf dem sich die Anzeige ändert; sie kann nicht vom Ordner abweichen.
- Für Netzlaufwerke ist Polling nötig.

## D8 – Server-Sent Events statt WebSocket

**Status:** Vorgeschlagen

**Begründung:** Ereignisse fließen nur vom Server zum Browser. SSE ist einfacher, läuft über normales HTTP und verbindet sich im Browser selbst neu. Aktionen laufen über REST.

## D9 – Ein Markdown-Parser im Backend

**Status:** Vorgeschlagen

**Entscheidung:** `pulldown-cmark` sowohl für das Finden und Umschreiben von Verweisen (über Quellpositionen) als auch für die Vorschau. Die Vorschau wird im Backend gerendert.

**Begründung:** Zwei Parser (z. B. einer im Browser, einer im Backend) könnten Verweise unterschiedlich erkennen. Dann zeigt die Vorschau etwas anderes, als die Verweisanpassung bearbeitet.

**Folgen:**
- Vorschau braucht eine Serveranfrage; das Frontend sendet gebündelt (ca. 250 ms).
- Der Parser muss zwei Dinge können: Quellpositionen von Verweisen liefern und Blockelemente im HTML mit Quellzeilen versehen (für den Ausklapp-Editor, D16). `comrak` bietet Quellzeilen im HTML als Option; mit `pulldown-cmark` ist dafür ein eigener HTML-Writer nötig. Die Wahl zwischen beiden wird vor M1 mit einem kleinen Versuch getroffen.

## D10 – Sortieren im Frontend

**Status:** Vorgeschlagen

**Begründung:** Umschalten der Sortierung soll ohne Serveranfrage sofort wirken. Das Backend liefert dafür Typ, Größe und Änderungsdatum jedes Eintrags.

## D11 – Mehrere Workspaces

**Status:** Vorgeschlagen

**Kontext:** Dokumentation für Beruf und Hobby.

**Entscheidung:** Die Serverkonfiguration kennt mehrere Workspaces mit eigener ID. API und Routen enthalten die Workspace-ID von Anfang an, auch wenn nur einer konfiguriert ist.

## D12 – Sprachen im Projekt

**Status:** Festgelegt (Code, Kommentare, Doku, Begriffsliste) / Vorgeschlagen (Commits, Pull Requests, Logmeldungen)

**Entscheidung:**

| Bereich | Sprache |
|---|---|
| Code: Bezeichner, Typen, Funktionen, Dateinamen, API-Felder, Ereignisnamen | Englisch |
| Kommentare im Code, auch Doc-Comments (`///`, `/** */`) | Deutsch |
| Dokumentation (`docs/`, `README.md`, `CLAUDE.md`) | Deutsch |
| Oberflächentexte | Deutsch, zentral abgelegt |
| Commit-Nachrichten und Pull Requests | Deutsch, Typ-Präfix englisch (`feat:`, `fix:` …) |
| Logmeldungen und technische Fehlermeldungen der API (`message`) | Englisch; das Frontend zeigt deutsche Texte anhand von `code` |

**Begriffe:** [begriffe.md](begriffe.md) legt für jeden Fachbegriff die deutsche und die englische Bezeichnung fest. Im Code wird nur die englische verwendet, in Texten nur die deutsche. Neue Begriffe werden dort eingetragen, bevor sie im Code auftauchen.

## D13 – Arbeit in Branches und Pull Requests

**Status:** Festgelegt

**Entscheidung:**
- Claude Code arbeitet pro Feature oder Schritt in einem eigenen Branch und öffnet danach einen Pull Request.
- Der Projektinhaber prüft und merged. Claude Code merged nie selbst und committet nie direkt auf `main`.
- Details (Branch-Namen, Prüfungen vor dem PR, Vorlage) stehen in `CLAUDE.md` und `.github/pull_request_template.md`.

## D14 – Name „docdump“

**Status:** Festgelegt

**Entscheidung:** Das Projekt heißt „docdump“ (zuvor Arbeitstitel „Halde“; *dump* ist die englische Entsprechung). Daraus abgeleitet: Programmname `docdump`, Serverkonfiguration `docdump.toml`, Workspace-Konfiguration `.docdump.toml`.

**Folgen:** Bei einer späteren Umbenennung müssen diese Namen mitgeändert werden. Deshalb im Code nur an einer Stelle als Konstante definieren.

## D15 – Anzeigebereich mit drei Modi

**Status:** Festgelegt (drei Darstellungen) / Vorgeschlagen (Namen, Modus in der URL)

**Entscheidung:**
- Drei Ansichtsmodi:
  - **Vorschau:** nur gerendertes Markdown
  - **Überlagert:** Code und gerenderte Anzeige gleichzeitig, der Editor liegt über der Vorschau
  - **Code:** nur Code
- Bearbeitet wird in Überlagert und Code. Getrennte Routen für Lesen und Bearbeiten gibt es nicht.
- Umgesetzt werden die Modi über den Ausklapp-Editor (D16).

**Folgen:** Eine Route pro Seite. Der Modus ist reine Darstellung und steht als Query-Parameter (`?view=…`) in der URL.

## D16 – Ausklapp-Editor als Overlay nach Art von Yakuake/Guake

**Status:** Festgelegt (Overlay, Breite und Höhe einstellbar, Tastenkombination und kleiner Knopf, Halbtransparenz möglich) / Vorgeschlagen (Tastenkürzel, Ausrichtung, Standardwerte, Weichzeichner)

**Kontext:** Meist wird gelesen, gelegentlich schnell etwas korrigiert. Der Code soll dann ohne Moduswechsel erreichbar sein, wie ein Drop-down-Terminal.

**Entscheidung:**
- Grundfläche des Anzeigebereichs ist die Vorschau. Der Editor legt sich darüber und verdrängt sie nicht.
- Breite und Höhe stellt man ein, ebenso Ausrichtung und Deckkraft. Die Einstellungen gelten pro Browser.
- Aktivieren per `Strg+^` (Taste links neben der `1`) oder über einen kleinen Knopf in der Kopfzeile. `F12` scheidet aus, weil Browser damit die Entwicklerwerkzeuge öffnen.
- Bei Halbtransparenz wird nur der Hintergrund durchscheinend, der Code-Text bleibt deckend.
- Beim Öffnen springt der Editor an die gerade gelesene Stelle. Beim Tippen folgt die Vorschau dem Cursor.

**Folgen:**
- Die Vorschau braucht Quellzeilen im HTML (D9).
- Code und Vorschau nebeneinander ergeben sich über die Einstellungen (Breite 50 %, Höhe 100 %, Ausrichtung rechts); ein eigener Modus dafür ist nicht nötig.

## D17 – Namen, Ordnerseiten, Dateinamen, Speichern

**Status:** Festgelegt (Namensumwandlung beim Hochladen und Umbenennen: Vorgeschlagen)

Entscheidet die früheren offenen Punkte E1, E2, E4, E7 und E10.

**Entscheidung:**
- **Namen im Dateibrowser (E1):** immer der echte, vollständige Dateiname, inklusive Endung und Nummern.
- **Ordnerseiten (E2):** Ein Klick auf einen Ordner zeigt seine `index.md`.
- **Dateinamen (E7):**
  - Beim Anlegen in der App werden Namen umgewandelt (Konzept 5.7).
  - Von außen angelegte Dateien behalten ihren Namen und werden voll unterstützt.
  - Vorgeschlagen: dieselbe Umwandlung auch beim Hochladen und beim Umbenennen in der App.
- **Speichern (E4, E10):** nur ausdrücklich (`Strg+S` oder Knopf). Ausblenden des Editors speichert nicht. Automatisches Speichern kommt später als Einstellung.

## D18 – Bilder-Browser

**Status:** Festgelegt (Wechsel über „Bilder“ im linken Bereich) / Vorgeschlagen (Aufteilung, eigener ls-Befehl, Autovervollständigung)

**Entscheidung:**
- Ein Klick auf den festen Eintrag **Bilder** links wechselt in den Bilder-Browser.
- Der Dateibrowser zeigt dann das Bildverzeichnis als Baum. Der Anzeigebereich zeigt ein Raster der Bilder des gewählten Ordners bzw. die Details eines Bildes.
- Der Bilder-Browser hat einen eigenen ls-Befehl (`ls_images`).
- Im Editor schlägt die Autovervollständigung Bildpfade vor.

**Folgen:** Die bisher geplante Medienansicht entfällt; ihre Aufgaben übernimmt der Bilder-Browser.

## D19 – Frontend-Bibliotheken

**Status:** Vorgeschlagen

**Entscheidung:** CodeMirror 6 als Editor, dazu VueUse, `@tailwindcss/typography` und Lucide. Baum, ls-Modul, Ausklapp-Editor und Dialoge entstehen ohne weitere Bibliotheken (Konzept 10.9).

**Begründung:**
- CodeMirror ist modular und leicht und läuft auch auf Touch-Geräten. Monaco wäre schwerer und auf Mobilgeräten schwach.
- Komponenten mit eigenem Zustand (z. B. fertige Baum-Komponenten) vertragen sich schlecht mit P2: Der Baum ändert sich nur über Ereignisse.
- Jede weitere Abhängigkeit braucht eine Begründung im Pull Request.
