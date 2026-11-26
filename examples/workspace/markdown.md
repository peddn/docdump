# Markdown

docdump versteht CommonMark mit den GitHub-Erweiterungen, ohne eigene Syntax.

## Text

Absatz mit *kursiv*, **fett**, ~~durchgestrichen~~ und `Code`.
Zeilenumbruch ohne Leerzeile gehört zum selben Absatz.

> Ein Zitat.
>
> Mit zwei Absätzen.

---

## Listen

- Punkt
  - eingerückter Punkt
- Punkt

1. Erstens
2. Zweitens

- [x] erledigt
- [ ] offen

## Tabelle

| links | Mitte | rechts |
|:---|:---:|---:|
| a | b | 1 |
| c | d | 22 |

## Code

```toml
image_dir = "img"
ls = "-v --group-directories-first"
```

    eingerückter Codeblock

## Fußnote

Ein Satz mit Fußnote.[^1]

[^1]: Die Fußnote.

## Autolink

https://example.org
