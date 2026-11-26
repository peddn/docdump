//! HTTP-Server von docdump: Routen, Fehlerformat und gemeinsamer Zustand.
//!
//! Die Programmdatei (`main.rs`) liest nur die Kommandozeile und die Konfiguration
//! und startet den Server. Alles andere steht hier, damit es ohne Netzwerk
//! testbar ist.

mod api;
mod error;

use std::sync::Arc;

use axum::Router;
use docdump_core::config::ServerConfig;

pub use error::ApiError;

/// Gemeinsamer Zustand aller Routen.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<ServerConfig>,
}

/// Baut den Router mit allen Routen.
pub fn router(config: ServerConfig) -> Router {
    let state = AppState {
        config: Arc::new(config),
    };
    Router::new()
        .nest("/api", api::router())
        .fallback(error::not_found)
        .with_state(state)
}
