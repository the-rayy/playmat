use axum::{Router, response::IntoResponse, routing::get};

use thiserror::Error as Thiserror;

#[derive(Thiserror, Debug)]
pub enum Error {
  #[error("")]
  InitializationError(#[from] std::io::Error)
}

pub async fn run(ipport: &str) -> Result<(), Error> {
  let router = Router::new()
  .route("/", get(handler));

  let listener = tokio::net::TcpListener::bind(ipport)
    .await?;

  axum::serve(listener, router)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("axum serve failed, but this should never happen");

  Ok(())
}

async fn handler() -> impl IntoResponse {
  "ok"
}

async fn shutdown_signal() {
  use tokio::signal::unix::{SignalKind, signal};
  let mut sigint = signal(SignalKind::interrupt()).unwrap();
  let mut sigterm = signal(SignalKind::terminate()).unwrap();

  tokio::select! {
      _ = sigint.recv() => {},
      _ = sigterm.recv() => {},
  }
  log::info!("Shutdown signal received, starting graceful shutdown");
}
