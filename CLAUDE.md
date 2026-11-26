# CLAUDE.md

Hinweise für die Arbeit an diesem Repository.

## Projekt

docdump ist ein selbst gehosteter Dokumentations-Server. Die Weboberfläche spiegelt einen Ordner mit Markdown-Dateien; jede Aktion in der Oberfläche ist eine Dateioperation. Beim Verschieben und Umbenennen passt die App alle Verweise an.

- Gesamtkonzept: `docs/konzept.md` (vor jeder größeren Arbeit lesen)
- Entscheidungen und ihr Status: `docs/entscheidungen.md`
- Begriffe Deutsch/Englisch: `docs/begriffe.md`

## Grundprinzipien (nicht verletzen)

1. **Dateien sind die einzige Wahrheit.** Keine Datenbank, kein dauerhafter Cache. Zustand im Speicher muss jederzeit aus den Dateien neu aufbaubar sein.
2. **Ein Weg für Änderungen.** Das Frontend ändert den Baum nur aufgrund von SSE-Ereignissen, nie optimistisch nach einer eigenen Aktion.
3. **Der Ordner bleibt gültig.** Verschieben und Umbenennen passen eingehende Verweise an. Was nicht angepasst werden kann, wird gemeldet, nie still übergangen.
4. **Standard-Markdown.** CommonMark + GFM. Keine eigene Syntax.
5. **Nichts außerhalb des Workspace.** Jeder Pfad aus einer Anfrage wird über `WsPath` geprüft. Keine Dateioperation mit ungeprüften Pfaden.

## Pfade in Markdown

- Bildeinbindung `![Alt](/netzwerk.png)`: führender `/` bezieht sich auf das Bildverzeichnis. Dessen Name steht nur in `.docdump.toml` (`image_dir`), nie im Markdown.
- Seitenlink `[Text](/server/backup.md)`: führender `/` bezieht sich auf die Workspace-Wurzel.
- Den Bezugspunkt bestimmt allein die Syntax, nicht die Dateiendung.
- **Interne Verweise beginnen immer mit `/`.** Relative Pfade sind ungültige Verweise: nicht auflösen, in der Vorschau markieren, bei der Verweisanpassung unverändert lassen. Relative Pfade entstehen nur im Export.
- In der API sind Pfade Workspace-Pfade ohne führenden `/` (Ausnahme: Antwort des Bild-Uploads).

## Sprachen

| Bereich | Sprache |
|---|---|
| Code: Bezeichner, Typen, Funktionen, Dateinamen, API-Felder, Ereignisnamen | Englisch |
| Kommentare im Code, auch Doc-Comments | Deutsch |
| Dokumentation (`docs/`, `README.md`, diese Datei) | Deutsch |
| Oberflächentexte | Deutsch, zentral abgelegt, nicht verstreut in Komponenten |
| Commit-Nachrichten und Pull Requests | Deutsch, Typ-Präfix englisch |
| Logmeldungen, `message` in API-Fehlern | Englisch |

**Begriffe:** Für Fachbegriffe gilt `docs/begriffe.md`. Im Code nur die englische Bezeichnung, in Texten nur die deutsche. Keine Synonyme. Ein neuer Begriff wird im selben Pull Request dort eingetragen, in dem er im Code auftaucht.

## Git-Arbeitsweise

- **Nie direkt auf `main` committen. Nie selbst mergen.** Der Projektinhaber prüft jeden Pull Request und merged.
- Pro Feature oder Schritt ein Branch vom aktuellen `main`:
  - Schema `<typ>/<kurzbeschreibung>`, bei Meilensteinarbeit mit Kürzel: `feat/m1-ws-path`, `docs/begriffe-export`.
  - Typen: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`.
- Pull Requests klein halten: ein abgeschlossener Schritt, der sich in einem Durchgang prüfen lässt. Lieber zwei PRs als einer, der Unabhängiges mischt.
- Vor dem Öffnen eines PR müssen grün sein:
  - `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`
  - Frontend: Lint, Typprüfung, Tests
- PR-Beschreibung nach `.github/pull_request_template.md`, mit Bezug auf Meilenstein und Abnahmekriterien.
- Commit-Nachrichten: `feat: WsPath mit Prüfung auf Ausbruchsversuche`.
- Review-Rückmeldungen als neue Commits auf demselben Branch umsetzen. Kein Force-Push, sobald das Review begonnen hat.
- Nach dem Merge den nächsten Branch vom aktualisierten `main` abzweigen. Hängt ein Schritt von einem ungemergten PR ab: im PR erwähnen und auf dessen Merge warten oder ausdrücklich darauf aufbauen.

## Stack

- **Backend:** Rust, Cargo-Workspace in `backend/` mit den Crates `core` (Fachlogik, ohne HTTP) und `server` (axum, SSE, Konfiguration, eingebettetes Frontend).
- **Frontend:** `frontend/` mit Vue 3 (`<script setup>`, TypeScript), Vite, Pinia, Vue Router, Tailwind CSS, CodeMirror 6.
- Neue Abhängigkeiten nur mit kurzer Begründung im PR. Bei der Versionswahl die aktuelle stabile Version prüfen.

## Konventionen

- **Name:** `docdump`, `docdump.toml`, `.docdump.toml` im Code nur an einer Stelle als Konstanten definieren, damit eine Umbenennung billig bleibt.
- **Schreiben:** immer atomar (temporäre Datei `.docdump-tmp-*` im selben Verzeichnis, dann umbenennen).
- **Verweise ändern:** nur über Quellpositionen des Parsers, nie per Suchen/Ersetzen im Gesamttext.
- **Fehler:** `thiserror` in `core`, `anyhow` nur in `server`. API-Fehler als `{error: {code, message}}`; das Frontend zeigt deutsche Texte anhand von `code`.

## Arbeitsweise

- Meilenstein für Meilenstein nach `docs/konzept.md` Kapitel 12, in mehreren kleinen PRs. Abnahmekriterien abhaken, wenn erfüllt und getestet.
- Bei offenen Entscheidungen (Konzept Kapitel 13) nicht raten: nachfragen, oder den Vorschlag umsetzen und das im PR deutlich als Frage an den Prüfer markieren.
- Entscheidungen mit Status „Festgelegt“ nur nach Rücksprache ändern.
- Wenn sich beim Bauen etwas am Konzept als falsch herausstellt: Konzept im selben PR anpassen, nicht stillschweigend abweichen.

## Tests

- `core` wird gründlich getestet, mit temporären Verzeichnissen (`tempfile`). Pflicht für:
  - `WsPath`: alle Ausbruchsversuche
  - Verweisanpassung: Anker, Referenzdefinitionen, Bildeinbindung vs. Seitenlink, ungültige Verweise unverändert, Codeblöcke unverändert, nicht anpassbare Stellen
  - atomares Schreiben und Versionskonflikte
  - Namensumwandlung gegen `fixtures/name-normalization.json` (dieselbe Tabelle testet auch das Frontend)
- Testdaten für die Verweisanpassung als Vorher/Nachher-Dateien unter `backend/crates/core/tests/fixtures/`.
- Frontend mit Vitest:
  - ls-Modul: alle Optionen, kombinierte Kurzoptionen, letzte Sortieroption gewinnt, Muster, Pfad-Argument, Fehlermeldungen
  - Stores, insbesondere das Anwenden von Ereignissen im `tree`-Store

## Befehle

Werden in Meilenstein M0 angelegt. Danach hier eintragen (Bauen, Testen, Entwicklungsserver starten, Lint).
