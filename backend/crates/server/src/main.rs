//! Programmdatei `docdump`: liest Kommandozeile und Serverkonfiguration und
//! startet den HTTP-Server.

use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use docdump_core::config::ServerConfig;
use docdump_core::names::{APP_NAME, SERVER_CONFIG_FILE};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = APP_NAME, version, about = "Self-hosted documentation server for Markdown folders")]
struct Cli {
    /// Path to the server config file
    #[arg(long, value_name = "FILE", default_value = SERVER_CONFIG_FILE)]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();
    let config = ServerConfig::load(&cli.config)
        .with_context(|| format!("loading {}", cli.config.display()))?;

    for ws in &config.workspaces {
        tracing::info!(id = %ws.id, root = %ws.root.display(), poll = ws.poll, "workspace");
    }

    let listener = tokio::net::TcpListener::bind(config.listen)
        .await
        .with_context(|| format!("binding {}", config.listen))?;
    tracing::info!(address = %listener.local_addr()?, "listening");

    axum::serve(listener, docdump_server::router(config))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("running server")?;

    tracing::info!("stopped");
    Ok(())
}

/// Wartet auf Strg+C oder SIGTERM, damit laufende Anfragen sauber enden.
async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(error) = tokio::signal::ctrl_c().await {
            tracing::error!(%error, "cannot listen for Ctrl+C");
            std::future::pending::<()>().await;
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
            }
            Err(error) => {
                tracing::error!(%error, "cannot listen for SIGTERM");
                std::future::pending::<()>().await;
            }
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {}
        () = terminate => {}
    }
    tracing::info!("shutting down");
}
