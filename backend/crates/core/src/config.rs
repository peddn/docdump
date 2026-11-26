//! Serverkonfiguration (`docdump.toml`, Konzept 5.6).
//!
//! Die Workspace-Konfiguration (`.docdump.toml`) kommt mit M1 dazu.

use std::collections::HashSet;
use std::io;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::path::{Path, PathBuf};

use serde::Deserialize;

/// Standardadresse des Servers: nur lokal erreichbar (Konzept 7.8).
pub const DEFAULT_LISTEN: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080));

/// Geprüfte Serverkonfiguration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerConfig {
    /// Adresse, an der der Server lauscht.
    pub listen: SocketAddr,
    /// Workspaces in der Reihenfolge der Konfigurationsdatei. Mindestens einer.
    pub workspaces: Vec<Workspace>,
}

/// Ein Workspace aus der Serverkonfiguration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workspace {
    /// Workspace-ID, Teil aller API-Pfade und Routen. Siehe [`is_valid_workspace_id`].
    pub id: String,
    /// Anzeigename in der Oberfläche.
    pub name: String,
    /// Kanonischer, absoluter Pfad der Workspace-Wurzel. Symlinks sind aufgelöst.
    pub root: PathBuf,
    /// Abfragen statt Benachrichtigung, für Netzlaufwerke (Konzept 7.4).
    pub poll: bool,
}

/// Fehler beim Laden der Serverkonfiguration.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("cannot read config file {path}")]
    Read {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("invalid config: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("no workspace configured, add at least one [[workspace]]")]
    NoWorkspace,
    #[error(
        "invalid workspace id {id:?}: use lowercase letters, digits, '-' and '_', \
         starting with a letter or digit"
    )]
    InvalidWorkspaceId { id: String },
    #[error("workspace id {id:?} is used more than once")]
    DuplicateWorkspaceId { id: String },
    #[error("workspace {id:?} has an empty name")]
    EmptyWorkspaceName { id: String },
    #[error("cannot access path {path} of workspace {id:?}")]
    WorkspacePath {
        id: String,
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("path {path} of workspace {id:?} is not a directory")]
    WorkspaceNotADirectory { id: String, path: PathBuf },
}

/// Rohform der Datei, wie sie in TOML steht.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawServerConfig {
    #[serde(default = "default_listen")]
    listen: SocketAddr,
    #[serde(default, rename = "workspace")]
    workspaces: Vec<RawWorkspace>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawWorkspace {
    id: String,
    name: String,
    path: PathBuf,
    #[serde(default)]
    poll: bool,
}

fn default_listen() -> SocketAddr {
    DEFAULT_LISTEN
}

impl ServerConfig {
    /// Liest und prüft die Serverkonfiguration aus einer Datei.
    ///
    /// Relative Workspace-Pfade gelten relativ zum Ordner der Konfigurationsdatei.
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        let text = std::fs::read_to_string(path).map_err(|source| ConfigError::Read {
            path: path.to_path_buf(),
            source,
        })?;
        let base_dir = path.parent().unwrap_or(Path::new(""));
        Self::from_toml(&text, base_dir)
    }

    /// Prüft die Serverkonfiguration aus TOML-Text.
    ///
    /// Relative Workspace-Pfade gelten relativ zu `base_dir`. Jeder Workspace-Pfad
    /// muss ein vorhandener Ordner sein; gespeichert wird der kanonische Pfad.
    pub fn from_toml(text: &str, base_dir: &Path) -> Result<Self, ConfigError> {
        let raw: RawServerConfig = toml::from_str(text)?;
        if raw.workspaces.is_empty() {
            return Err(ConfigError::NoWorkspace);
        }

        let mut seen = HashSet::new();
        let mut workspaces = Vec::with_capacity(raw.workspaces.len());
        for ws in raw.workspaces {
            if !is_valid_workspace_id(&ws.id) {
                return Err(ConfigError::InvalidWorkspaceId { id: ws.id });
            }
            if !seen.insert(ws.id.clone()) {
                return Err(ConfigError::DuplicateWorkspaceId { id: ws.id });
            }
            let name = ws.name.trim();
            if name.is_empty() {
                return Err(ConfigError::EmptyWorkspaceName { id: ws.id });
            }
            let root = resolve_root(&ws.id, base_dir, &ws.path)?;
            workspaces.push(Workspace {
                name: name.to_owned(),
                id: ws.id,
                root,
                poll: ws.poll,
            });
        }

        Ok(Self {
            listen: raw.listen,
            workspaces,
        })
    }

    /// Workspace mit der angegebenen ID.
    pub fn workspace(&self, id: &str) -> Option<&Workspace> {
        self.workspaces.iter().find(|ws| ws.id == id)
    }
}

/// Prüft eine Workspace-ID: nicht leer, nur `a–z`, `0–9`, `-` und `_`,
/// erstes Zeichen Buchstabe oder Ziffer. So kann sie ohne Kodierung in URLs stehen.
pub fn is_valid_workspace_id(id: &str) -> bool {
    let mut chars = id.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    let allowed = |c: char| c.is_ascii_lowercase() || c.is_ascii_digit();
    allowed(first) && chars.all(|c| allowed(c) || c == '-' || c == '_')
}

/// Bildet den kanonischen Pfad der Workspace-Wurzel und prüft, dass er ein Ordner ist.
fn resolve_root(id: &str, base_dir: &Path, path: &Path) -> Result<PathBuf, ConfigError> {
    let joined = base_dir.join(path);
    let root = joined
        .canonicalize()
        .map_err(|source| ConfigError::WorkspacePath {
            id: id.to_owned(),
            path: joined.clone(),
            source,
        })?;
    if !root.is_dir() {
        return Err(ConfigError::WorkspaceNotADirectory {
            id: id.to_owned(),
            path: root,
        });
    }
    Ok(root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// Temporärer Ordner mit den Unterordnern `beruf` und `hobby`.
    fn base() -> TempDir {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join("beruf")).unwrap();
        std::fs::create_dir(dir.path().join("hobby")).unwrap();
        dir
    }

    fn parse(text: &str, base: &TempDir) -> Result<ServerConfig, ConfigError> {
        ServerConfig::from_toml(text, base.path())
    }

    #[test]
    fn minimal_config_uses_defaults() {
        let dir = base();
        let config = parse(
            r#"
            [[workspace]]
            id = "beruf"
            name = "Beruf"
            path = "beruf"
            "#,
            &dir,
        )
        .unwrap();

        assert_eq!(config.listen, DEFAULT_LISTEN);
        assert_eq!(config.listen.to_string(), "127.0.0.1:8080");
        assert_eq!(config.workspaces.len(), 1);
        let ws = &config.workspaces[0];
        assert_eq!(ws.id, "beruf");
        assert_eq!(ws.name, "Beruf");
        assert!(!ws.poll);
    }

    #[test]
    fn full_config_keeps_order() {
        let dir = base();
        let config = parse(
            r#"
            listen = "0.0.0.0:9000"

            [[workspace]]
            id = "hobby"
            name = "Hobby"
            path = "hobby"
            poll = true

            [[workspace]]
            id = "beruf"
            name = "Beruf"
            path = "beruf"
            "#,
            &dir,
        )
        .unwrap();

        assert_eq!(config.listen.to_string(), "0.0.0.0:9000");
        let ids: Vec<_> = config.workspaces.iter().map(|ws| ws.id.as_str()).collect();
        assert_eq!(ids, ["hobby", "beruf"]);
        assert!(config.workspaces[0].poll);
        assert_eq!(config.workspace("beruf").unwrap().name, "Beruf");
        assert!(config.workspace("gibt-es-nicht").is_none());
    }

    #[test]
    fn relative_path_is_resolved_against_base_dir() {
        let dir = base();
        let config = parse(
            r#"
            [[workspace]]
            id = "beruf"
            name = "Beruf"
            path = "./hobby/../beruf"
            "#,
            &dir,
        )
        .unwrap();

        let expected = dir.path().join("beruf").canonicalize().unwrap();
        assert_eq!(config.workspaces[0].root, expected);
        assert!(config.workspaces[0].root.is_absolute());
    }

    #[test]
    fn absolute_path_is_kept() {
        let dir = base();
        let other = tempfile::tempdir().unwrap();
        let text = format!(
            "[[workspace]]\nid = \"beruf\"\nname = \"Beruf\"\npath = {:?}\n",
            other.path()
        );
        let config = parse(&text, &dir).unwrap();
        assert_eq!(
            config.workspaces[0].root,
            other.path().canonicalize().unwrap()
        );
    }

    #[test]
    fn name_is_trimmed() {
        let dir = base();
        let config = parse(
            "[[workspace]]\nid = \"beruf\"\nname = \"  Beruf \"\npath = \"beruf\"\n",
            &dir,
        )
        .unwrap();
        assert_eq!(config.workspaces[0].name, "Beruf");
    }

    #[test]
    fn load_resolves_relative_to_config_file() {
        let dir = base();
        let file = dir.path().join("docdump.toml");
        std::fs::write(
            &file,
            "[[workspace]]\nid = \"beruf\"\nname = \"Beruf\"\npath = \"beruf\"\n",
        )
        .unwrap();

        let config = ServerConfig::load(&file).unwrap();
        assert_eq!(
            config.workspaces[0].root,
            dir.path().join("beruf").canonicalize().unwrap()
        );
    }

    #[test]
    fn load_missing_file() {
        let dir = base();
        let err = ServerConfig::load(&dir.path().join("fehlt.toml")).unwrap_err();
        assert!(matches!(err, ConfigError::Read { .. }), "{err:?}");
    }

    #[test]
    fn no_workspace() {
        let dir = base();
        let err = parse("listen = \"127.0.0.1:8080\"\n", &dir).unwrap_err();
        assert!(matches!(err, ConfigError::NoWorkspace), "{err:?}");
    }

    #[test]
    fn invalid_toml() {
        let dir = base();
        let err = parse("[[workspace]\n", &dir).unwrap_err();
        assert!(matches!(err, ConfigError::Parse(_)), "{err:?}");
    }

    #[test]
    fn unknown_keys_are_rejected() {
        let dir = base();
        // Tippfehler im Tabellennamen
        let err = parse(
            "[[workspaces]]\nid = \"beruf\"\nname = \"Beruf\"\npath = \"beruf\"\n",
            &dir,
        )
        .unwrap_err();
        assert!(matches!(err, ConfigError::Parse(_)), "{err:?}");

        // unbekannter Schlüssel im Workspace
        let err = parse(
            "[[workspace]]\nid = \"beruf\"\nname = \"Beruf\"\npath = \"beruf\"\npoling = true\n",
            &dir,
        )
        .unwrap_err();
        assert!(matches!(err, ConfigError::Parse(_)), "{err:?}");
    }

    #[test]
    fn missing_required_key() {
        let dir = base();
        let err = parse("[[workspace]]\nid = \"beruf\"\npath = \"beruf\"\n", &dir).unwrap_err();
        assert!(matches!(err, ConfigError::Parse(_)), "{err:?}");
    }

    #[test]
    fn invalid_listen_address() {
        let dir = base();
        let err = parse(
            "listen = \"localhost\"\n[[workspace]]\nid = \"beruf\"\nname = \"Beruf\"\npath = \"beruf\"\n",
            &dir,
        )
        .unwrap_err();
        assert!(matches!(err, ConfigError::Parse(_)), "{err:?}");
    }

    #[test]
    fn invalid_workspace_ids() {
        let dir = base();
        for id in [
            "", "Beruf", "be ruf", "be/ruf", "..", ".beruf", "-beruf", "_beruf", "bérüf", "be%20",
        ] {
            let text = format!("[[workspace]]\nid = {id:?}\nname = \"Beruf\"\npath = \"beruf\"\n");
            let err = parse(&text, &dir).unwrap_err();
            assert!(
                matches!(err, ConfigError::InvalidWorkspaceId { .. }),
                "{id:?}: {err:?}"
            );
        }
    }

    #[test]
    fn valid_workspace_ids() {
        for id in ["beruf", "b", "2026", "beruf-alt", "beruf_alt", "a-b_c-1"] {
            assert!(is_valid_workspace_id(id), "{id:?}");
        }
    }

    #[test]
    fn duplicate_workspace_id() {
        let dir = base();
        let err = parse(
            r#"
            [[workspace]]
            id = "beruf"
            name = "Beruf"
            path = "beruf"

            [[workspace]]
            id = "beruf"
            name = "Hobby"
            path = "hobby"
            "#,
            &dir,
        )
        .unwrap_err();
        assert!(
            matches!(err, ConfigError::DuplicateWorkspaceId { ref id } if id == "beruf"),
            "{err:?}"
        );
    }

    #[test]
    fn empty_name() {
        let dir = base();
        let err = parse(
            "[[workspace]]\nid = \"beruf\"\nname = \"   \"\npath = \"beruf\"\n",
            &dir,
        )
        .unwrap_err();
        assert!(
            matches!(err, ConfigError::EmptyWorkspaceName { .. }),
            "{err:?}"
        );
    }

    #[test]
    fn missing_workspace_dir() {
        let dir = base();
        let err = parse(
            "[[workspace]]\nid = \"beruf\"\nname = \"Beruf\"\npath = \"fehlt\"\n",
            &dir,
        )
        .unwrap_err();
        assert!(matches!(err, ConfigError::WorkspacePath { .. }), "{err:?}");
    }

    #[test]
    fn workspace_path_is_a_file() {
        let dir = base();
        std::fs::write(dir.path().join("datei.md"), "# Datei\n").unwrap();
        let err = parse(
            "[[workspace]]\nid = \"beruf\"\nname = \"Beruf\"\npath = \"datei.md\"\n",
            &dir,
        )
        .unwrap_err();
        assert!(
            matches!(err, ConfigError::WorkspaceNotADirectory { .. }),
            "{err:?}"
        );
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_root_is_resolved() {
        let dir = base();
        std::os::unix::fs::symlink(dir.path().join("beruf"), dir.path().join("link")).unwrap();
        let config = parse(
            "[[workspace]]\nid = \"beruf\"\nname = \"Beruf\"\npath = \"link\"\n",
            &dir,
        )
        .unwrap();
        assert_eq!(
            config.workspaces[0].root,
            dir.path().join("beruf").canonicalize().unwrap()
        );
    }
}
