# Begriffe

Verbindliche Fachbegriffe des Projekts.

- **Im Code** (Bezeichner, Typen, API-Felder, Ereignisnamen) gilt nur die englische Bezeichnung.
- **In Dokumentation, Kommentaren und Oberflächentexten** gilt nur die deutsche Bezeichnung.
- **Neue Begriffe** werden hier eingetragen, bevor oder während sie im Code auftauchen, im selben Pull Request.
- **Synonyme vermeiden:** Für jede Sache gibt es genau einen Begriff pro Sprache.

## Inhalt und Struktur

| Deutsch | Englisch (Code) | Erklärung |
|---|---|---|
| Workspace | `workspace` | Ein Wurzelordner mit Dokumentation, z. B. „Beruf“ oder „Hobby“. Der Server bedient einen oder mehrere. |
| Workspace-ID | `workspace_id`, in URLs `ws` | Kurzname eines Workspace aus der Serverkonfiguration. Teil aller API-Pfade und Routen. |
| Workspace-Wurzel | `root` | Ordner eines Workspace im Dateisystem, in der Serverkonfiguration als `path` angegeben. Bezugspunkt für Workspace-Pfade und Wurzelpfade. |
| Eintrag | `entry` | Eine Datei oder ein Ordner im Workspace. |
| Art des Eintrags | `kind` | `page`, `dir` oder `file`. |
| Seite | `page` | Eine `.md`-Datei. |
| Ordner | `dir` | Ein Verzeichnis im Workspace. |
| Datei | `file` | Ein Eintrag, der weder Seite noch Ordner ist (z. B. PDF, Bild). |
| Ordnerseite | `index_page` | Die `index.md` eines Ordners, die beim Klick auf den Ordner gezeigt wird. |
| Baum | `tree` | Alle Einträge eines Workspace in hierarchischer Darstellung. |
| Bildverzeichnis | `image_dir` | In der Workspace-Konfiguration festgelegtes Verzeichnis für Bilder, Standard `img`. Wird in Bildpfaden nie angegeben. |
| Bild | `image` | Eine Datei im Bildverzeichnis. |
| Bilder-Browser | `image_browser` | Ansicht des Bildverzeichnisses: links als Baum, rechts als Raster bzw. Details eines Bildes. |
| Bereich | `section` | Was der Dateibrowser zeigt: Seiten (`pages`) oder Bilder (`images`). |
| Vorschaubild | `thumbnail` | Verkleinerte Darstellung eines Bildes im Raster. |
| Bilddetails | `image_detail` | Großes Bild mit Bildpfad, Eigenschaften und eingehenden Verweisen. |

## Pfade

| Deutsch | Englisch (Code) | Erklärung |
|---|---|---|
| Workspace-Pfad | `WsPath` (Typ), `path` (Feld) | Geprüfter Pfad relativ zur Workspace-Wurzel, ohne führenden `/`, Trenner `/`. Interne Form und Form in der API. Einziger Weg, aus Benutzereingaben Dateipfade zu machen. |
| Wurzelpfad | `root_path` | Pfad mit führendem `/` in einem Seitenlink, bezogen auf die Workspace-Wurzel: `/server/backup.md`. |
| Bildpfad | `image_path` | Pfad mit führendem `/` in einer Bildeinbindung, bezogen auf das Bildverzeichnis: `/netzwerk.png`. |
| Relativer Pfad | `relative_path` | Pfad ohne führenden `/`. In Verweisen nicht erlaubt (siehe ungültiger Verweis); entsteht nur im Export. |
| Pfadabbildung | `path_mapping` | Zuordnung alter Pfad → neuer Pfad bei einem Verschieben; bei Ordnern für alle Pfade mit dem alten Präfix. |

## Verweise

| Deutsch | Englisch (Code) | Erklärung |
|---|---|---|
| Verweis | `link` | Oberbegriff für Seitenlink und Bildeinbindung. |
| Seitenlink | `page_link` | Markdown-Link `[Text](/pfad.md)`. Kann auch auf andere Dateien zeigen. |
| Bildeinbindung | `image_embed` | Markdown-Bild `![Alt](/bild.png)`. |
| Ziel | `target` | Der Eintrag, auf den ein Verweis zeigt. |
| Anker | `anchor` | Sprungmarke hinter `#`, z. B. `/netzwerk.md#ports`. |
| Referenzdefinition | `reference_definition` | Markdown-Definition `[ref]: /pfad.md`, auf die Links mit `[Text][ref]` verweisen. |
| Quellposition | `source_span` | Byte-Bereich eines Verweises im Markdown-Text, geliefert vom Parser. |
| Eingehender Verweis | `backlink` | Verweis aus einer anderen Seite auf einen Eintrag. |
| Kaputter Verweis | `broken_link` | Gültig geschriebener Verweis, dessen Ziel nicht existiert. |
| Ungültiger Verweis | `invalid_link` | Interner Verweis, der nicht mit `/` beginnt. Wird nicht aufgelöst, sondern markiert und gemeldet. |
| Externer Verweis | `external_link` | Verweis mit Schema (`https:`, `mailto:` …). Wird nie geprüft oder angepasst. |
| Verweisanpassung | `link_rewrite` | Umschreiben von Verweisen nach einem Verschieben oder Umbenennen. |
| Nicht anpassbare Stelle | `unresolved_link` | Verweis, den die Verweisanpassung nicht sicher ändern konnte. Wird immer gemeldet. |

## Operationen

| Deutsch | Englisch (Code) | Erklärung |
|---|---|---|
| Anlegen | `create` | Neue Seite oder neuen Ordner erzeugen. |
| Speichern | `save` | Inhalt einer Seite schreiben, mit Prüfung der Basisversion. |
| Verschieben | `move` | Eintrag an einen anderen Pfad bringen. Umbenennen ist dieselbe Operation. |
| Umbenennen | `move` (kein eigener Begriff im Code) | Verschieben innerhalb desselben Ordners. Im Text „Umbenennen“, im Code `move`. |
| Löschen | `delete` | Eintrag entfernen, mit Warnung bei eingehenden Verweisen. |
| Hochladen | `upload` | Bild über die Oberfläche ins Bildverzeichnis bringen. |
| Namensumwandlung | `normalize_name` | Umwandlung eines eingegebenen Namens in einen Dateinamen: Kleinbuchstaben, `-` statt Leerzeichen, Umlaute ausgeschrieben (Konzept 5.7). |
| Atomares Schreiben | `atomic_write` | Erst in eine temporäre Datei im selben Verzeichnis schreiben, dann umbenennen. |
| Version | `version` | BLAKE3-Hash des Dateiinhalts. |
| Basisversion | `base_version` | Version, auf der eine Bearbeitung beruht; wird beim Speichern mitgeschickt. |
| Konflikt | `conflict` | Datei wurde seit dem Lesen von außen geändert; Speichern wird mit `409` abgelehnt. |

## Überwachung und Ereignisse

| Deutsch | Englisch (Code) | Erklärung |
|---|---|---|
| Dateiüberwachung | `watcher` | Meldet Änderungen im Workspace (unter Linux über inotify). |
| Ereignis | `event` | Meldung über eine Änderung, z. B. `entry.renamed`. |
| Ereignisstrom | `event_stream` | SSE-Verbindung, über die das Frontend Ereignisse erhält. |
| Bündelung | `debounce` | Zusammenfassen schnell aufeinanderfolgender Ereignisse. |
| Neusynchronisierung | `resync` | Vollständiges Neueinlesen des Baums, z. B. nach verlorenen Ereignissen. |
| Abfragen | `poll` | Regelmäßiges Prüfen statt Benachrichtigung, für Netzlaufwerke. |

## Darstellung

| Deutsch | Englisch (Code) | Erklärung |
|---|---|---|
| Dateibrowser | `file_browser` | Linker Bereich der Oberfläche: Workspace-Wahl, ls-Feld und Baum. |
| ls-Befehl | `ls_command` | Eingabe im ls-Feld, z. B. `ls -v --group-directories-first`. Legt Sortierung, Filter und Anzeige des Dateibrowsers fest. |
| ls-Option | `ls_option` | Einzelne Option des ls-Befehls, z. B. `-v`. |
| Detailansicht | `long_format` | Dateibrowser mit Größe und Änderungsdatum (`-l`). |
| Verlauf | `history` | Frühere Eingaben im ls-Feld. |
| Autovervollständigung | `autocomplete` | Vorschläge für Bildpfade und Seitenlinks beim Tippen im Editor. |
| Anzeigebereich | `doc_pane` | Rechter Bereich der Oberfläche mit der geöffneten Seite. |
| Vorschau | `preview` | Gerenderte Ansicht einer Seite; auch Name des Ansichtsmodus mit nur gerendertem Markdown. |
| Code (Anzeigebereich) | `code` | Markdown-Quelltext einer Seite im Editor; auch Name des Ansichtsmodus mit nur Code. |
| Rendern | `render` | Markdown in HTML umwandeln. |
| Sortierung | `sort` | Reihenfolge im Dateibrowser, festgelegt über den ls-Befehl. |
| Natürliche Sortierung | `natural_sort` | Zahlen im Namen werden als Zahlen verglichen: `2` vor `10`. |
| Ansichtsmodus | `view_mode` | Darstellung des Anzeigebereichs: Vorschau (`preview`, Editor ausgeblendet), Überlagert (`overlay`, Editor über der Vorschau) oder Code (`code`, Editor maximiert). |
| Ausklapp-Editor | `drop_down_editor` | Editor für den Code einer Seite, der per Taste oder Knopf von oben hereingleitet und sich über die Vorschau legt, nach dem Vorbild von Yakuake und Guake. |
| Rand (Ausklapp-Editor) | `resize_edge` | Ziehbarer Rand des Ausklapp-Editors: unten für die Höhe, seitlich für die Breite. |
| Ausrichtung | `align` | Lage des Ausklapp-Editors am oberen Rand: links, Mitte oder rechts. |
| Deckkraft | `opacity` | Wie stark der Hintergrund des Ausklapp-Editors die Vorschau verdeckt (50–100 %). Der Code-Text bleibt immer deckend. |
| Weichzeichner | `backdrop_blur` | Optionales Verwischen der durchscheinenden Vorschau, damit sich Code und gerenderter Text nicht vermischen. |
| Maximieren | `maximize` | Ausklapp-Editor vorübergehend auf volle Größe und volle Deckkraft; entspricht dem Ansichtsmodus Code. |
| Quellzeile | `source_line` | Zeile im Markdown, aus der ein Element der Vorschau stammt (`data-sourcepos`). Grundlage dafür, dass Editor und Vorschau einander folgen. |
| Hinweisleiste | `banner` | Dauerhafte Hinweiszeile, z. B. bei externer Änderung. |
| Meldung | `toast` | Kurze, selbst verschwindende Meldung, z. B. „3 Verweise angepasst“. |

## Konfiguration

| Deutsch | Englisch (Code) | Erklärung |
|---|---|---|
| Serverkonfiguration | `server_config` | Datei `docdump.toml`: Adresse, Workspaces. Liegt außerhalb der Workspaces. |
| Workspace-Konfiguration | `workspace_config` | Datei `.docdump.toml` in der Workspace-Wurzel: Bildverzeichnis, Standardsortierung, Ausnahmen. |
| Ausnahmen | `ignore` | Muster für Einträge, die nicht angezeigt und nicht überwacht werden. |

## Projekt

| Deutsch | Englisch (Code) | Erklärung |
|---|---|---|
| Meilenstein | `milestone` | Abschnitt der Umsetzung (M0–M5), siehe Konzept Kapitel 12. |
| Abnahmekriterium | `acceptance_criterion` | Prüfbare Bedingung, die ein Meilenstein erfüllen muss. |
| Export | `export` | Kopie des Workspace in einem anderen Format, z. B. ZIP mit relativen Pfaden. |
