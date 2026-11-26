//! Namen, die sich vom Projektnamen ableiten.
//!
//! Der Projektname steht nur hier, damit eine Umbenennung billig bleibt (D14).
//! Alle abgeleiteten Namen werden zur Übersetzungszeit aus ihm gebildet.

/// Liefert den Projektnamen als Literal, damit er in `concat!` nutzbar ist.
macro_rules! app_name {
    () => {
        "docdump"
    };
}

/// Name des Projekts und der Programmdatei.
pub const APP_NAME: &str = app_name!();

/// Dateiname der Serverkonfiguration.
pub const SERVER_CONFIG_FILE: &str = concat!(app_name!(), ".toml");

/// Dateiname der Workspace-Konfiguration in der Workspace-Wurzel.
pub const WORKSPACE_CONFIG_FILE: &str = concat!(".", app_name!(), ".toml");

/// Präfix temporärer Dateien beim atomaren Schreiben.
pub const TEMP_FILE_PREFIX: &str = concat!(".", app_name!(), "-tmp-");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derived_names() {
        assert_eq!(APP_NAME, "docdump");
        assert_eq!(SERVER_CONFIG_FILE, "docdump.toml");
        assert_eq!(WORKSPACE_CONFIG_FILE, ".docdump.toml");
        assert_eq!(TEMP_FILE_PREFIX, ".docdump-tmp-");
    }

    #[test]
    fn hidden_files_start_with_dot() {
        // Workspace-Konfiguration und temporäre Dateien müssen versteckt sein,
        // damit der Dateibrowser sie ohne `ls -A` nicht zeigt.
        assert!(WORKSPACE_CONFIG_FILE.starts_with('.'));
        assert!(TEMP_FILE_PREFIX.starts_with('.'));
    }
}
