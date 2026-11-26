# Einrichtung

Zuerst das Netz, dann der Server.

![Rack](/server/rack.jpg)

## Betriebssystem

1. Image herunterladen und Prüfsumme vergleichen.
2. Installieren, Hostname `server` setzen.
3. Feste Adresse aus [Netzwerk › Adressen](/netzwerk.md#adressen) eintragen.

## Zugang

```bash
ssh-copy-id admin@192.168.1.10
```

Danach das [Backup](/server/02-backup.md) einrichten.
