//! Fehlerantworten der API im Format `{error: {code, message}}`.
//!
//! `code` ist ein fester, maschinenlesbarer Schlüssel; das Frontend zeigt anhand
//! dessen einen deutschen Text. `message` ist ein englischer Hinweis für Logs und
//! Fehlersuche.

use axum::Json;
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use serde::Serialize;

/// Fehler, den ein Handler als Antwort zurückgibt.
#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: String,
}

impl ApiError {
    pub fn new(status: StatusCode, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status,
            code,
            message: message.into(),
        }
    }

    /// Ressource oder Route existiert nicht.
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, "not_found", message)
    }

    /// Unerwarteter Fehler im Server. Die Ursache wird geloggt, nicht ausgeliefert.
    pub fn internal(error: &dyn std::error::Error) -> Self {
        tracing::error!(error = %error, "internal error");
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal",
            "internal server error",
        )
    }
}

#[derive(Serialize)]
struct ErrorBody<'a> {
    error: ErrorDetail<'a>,
}

#[derive(Serialize)]
struct ErrorDetail<'a> {
    code: &'a str,
    message: &'a str,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = ErrorBody {
            error: ErrorDetail {
                code: self.code,
                message: &self.message,
            },
        };
        (self.status, Json(body)).into_response()
    }
}

/// Antwort für alle Pfade ohne Route.
pub(crate) async fn not_found(uri: Uri) -> ApiError {
    ApiError::not_found(format!("no route for {}", uri.path()))
}
