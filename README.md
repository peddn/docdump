# docdump

> Status: Konzeptphase

Ein selbst gehosteter Dokumentations-Server für Markdown. Die Weboberfläche spiegelt einen Ordner:

- **Dateien sind die einzige Wahrheit.** Keine Datenbank. Der Ordner bleibt ohne die App voll nutzbar.
- **Umbauen ohne Reue.** Seiten und Ordner per Drag & drop verschieben; alle Verweise werden angepasst.
- **Live gespiegelt.** Änderungen mit anderen Programmen oder per `git pull` erscheinen sofort.
- **Editor mit Vorschau** direkt im Browser. Bilder liegen in einem eigenen Verzeichnis und werden kurz als `![](/bild.png)` eingebunden.

## Technik

- Backend: Rust (axum), eine Programmdatei
- Frontend: Vue 3, Pinia, Vue Router, Tailwind CSS

## Dokumentation

- [Konzept](docs/konzept.md)
- [Entscheidungen](docs/entscheidungen.md)
- [Begriffe](docs/begriffe.md)

## Mitarbeit

Änderungen nur über Branches und Pull Requests. Regeln in [CLAUDE.md](CLAUDE.md).
