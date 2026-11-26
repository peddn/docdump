# Verweise

Jede Art von Verweis auf einer Seite (Konzept 5.2). Zum Prüfen der Vorschau (7.7) und der Verweisanpassung (7.6).

## Seitenlinks

- Seite: [Netzwerk](/netzwerk.md)
- Seite in einem Ordner: [Backup](/server/02-backup.md)
- Ordnerseite: [Server](/server/index.md)
- mit Anker: [Ports](/netzwerk.md#ports)
- Name mit Leerzeichen, URL-kodiert: [Mein Dokument](/mein%20dokument.md)
- Datei zum Herunterladen; Seitenlink, daher mit Bildverzeichnis: [Netzwerkplan als PNG](/img/netzwerk.png)
- Referenzlink: [Einrichtung][einrichtung]
- Referenzlink, Kurzform, mit Anker: [zeitplan]

[einrichtung]: /server/01-einrichtung.md
[zeitplan]: /server/02-backup.md#zeitplan

## Bildeinbindungen

Ab dem Bildverzeichnis, ohne dessen Namen:

![Netzwerkplan](/netzwerk.png)

![Rack](/server/rack.jpg)

![Lageplan](/plan.svg)

## Externe Verweise und reine Anker

Werden nie geprüft oder angepasst:

- [Beispielseite](https://example.org)
- [E-Mail](mailto:admin@example.org)
- [nach oben](#verweise)

## Kaputte Verweise

Gültig geschrieben, aber das Ziel fehlt (`link-broken`):

- [fehlende Seite](/gibt-es-nicht.md)
- ![fehlendes Bild](/fehlt.png)

## Ungültige Verweise

Ohne führenden `/` (`link-invalid`). Werden nicht aufgelöst und nie angepasst:

- [relativ](server/01-einrichtung.md)
- [relativ nach oben](../netzwerk.md)
- ![relatives Bild](netzwerk.png)

## Kein Verweis

Inline-Code bleibt unverändert: `[Netzwerk](/netzwerk.md)`

```markdown
[Netzwerk](/netzwerk.md)
![Netzwerkplan](/netzwerk.png)
```

## Rohes HTML

Wird im MVP nicht angepasst, aber gemeldet:

<img src="/netzwerk.png" alt="Netzwerkplan in rohem HTML">
