//! Tests der HTTP-Routen ohne Netzwerk: Anfragen gehen direkt an den Router.

use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use docdump_core::config::ServerConfig;
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tempfile::TempDir;
use tower::ServiceExt;

/// Konfiguration mit den Workspaces `beruf` und `hobby` in einem temporären Ordner.
fn config() -> (TempDir, ServerConfig) {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("beruf")).unwrap();
    std::fs::create_dir(dir.path().join("hobby")).unwrap();
    let config = ServerConfig::from_toml(
        r#"
        [[workspace]]
        id = "beruf"
        name = "Beruf"
        path = "beruf"

        [[workspace]]
        id = "hobby"
        name = "Hobby"
        path = "hobby"
        "#,
        dir.path(),
    )
    .unwrap();
    (dir, config)
}

async fn get(uri: &str) -> (StatusCode, Option<String>, Value) {
    let (_dir, config) = config();
    let response = docdump_server::router(config)
        .oneshot(Request::get(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().to_owned());
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        content_type,
        serde_json::from_slice(&bytes).unwrap(),
    )
}

#[tokio::test]
async fn list_workspaces() {
    let (status, content_type, body) = get("/api/workspaces").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(content_type.as_deref(), Some("application/json"));
    // Reihenfolge wie in der Konfiguration, ohne Pfad auf dem Server
    assert_eq!(
        body,
        json!([
            {"id": "beruf", "name": "Beruf"},
            {"id": "hobby", "name": "Hobby"},
        ])
    );
}

#[tokio::test]
async fn unknown_api_route_is_json_error() {
    let (status, content_type, body) = get("/api/gibt-es-nicht").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(content_type.as_deref(), Some("application/json"));
    assert_eq!(body["error"]["code"], "not_found");
    assert_eq!(body["error"]["message"], "no route for /api/gibt-es-nicht");
}

#[tokio::test]
async fn unknown_route_outside_api_is_json_error() {
    let (status, _, body) = get("/irgendwas").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "not_found");
}
