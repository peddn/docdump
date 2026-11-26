# Beispiel-Workspace

Dieser Workspace dient zum Entwickeln und Testen von docdump. Er folgt dem Aufbau aus dem Konzept (Kapitel 5).

- [Netzwerk](/netzwerk.md)
- [Server](/server/index.md)
- [Markdown](/markdown.md): CommonMark mit den GitHub-Erweiterungen
- [Verweise](/verweise.md): jede Art von Verweis auf einer Seite

![Netzwerkplan](/netzwerk.png)

## Für den Dateibrowser

| Was | Wo | Sichtbar |
|---|---|---|
| Nummern im Namen | `/server/dienste/` | `ls -v` sortiert `2-dns.md` vor `10-mail.md` |
| Ordnerseite | `/server/index.md` | Klick auf den Ordner `server` |
| versteckte Seite | `/.entwurf.md` | nur mit `ls -A` |
| ausgeblendet über `ignore` | `/entwurf-gliederung.md`, `/archiv/` | nie |
| Datei, die keine Seite ist | `/notizen.txt` | offen (E3) |
| Name mit Leerzeichen | `/mein dokument.md` | immer |
| Bildverzeichnis | `/img/` | nur im Bilder-Browser |
