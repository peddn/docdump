//! Routen unter `/api` (Konzept 8.1).

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;

use crate::AppState;

pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/workspaces", get(list_workspaces))
}

/// Eintrag in der Liste der Workspaces. Der Pfad auf dem Server bleibt intern.
#[derive(Serialize)]
struct WorkspaceInfo {
    id: String,
    name: String,
}

/// `GET /api/workspaces`: Workspaces in der Reihenfolge der Konfiguration.
async fn list_workspaces(State(state): State<AppState>) -> Json<Vec<WorkspaceInfo>> {
    let list = state
        .config
        .workspaces
        .iter()
        .map(|ws| WorkspaceInfo {
            id: ws.id.clone(),
            name: ws.name.clone(),
        })
        .collect();
    Json(list)
}
