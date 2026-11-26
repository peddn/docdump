//! Programmdatei `docdump`.
//!
//! Im Gerüst (M0) gibt sie nur Namen und Version aus. Konfiguration, Routen
//! und Ereignisstrom folgen in eigenen Schritten.

use docdump_core::names::APP_NAME;

fn main() {
    println!("{APP_NAME} {}", env!("CARGO_PKG_VERSION"));
}
