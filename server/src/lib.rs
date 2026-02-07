use axum::{Router, response::IntoResponse, routing::get};

pub async fn run_server() {
  let router = Router::new()
    .route("/", get(handler));

  let server_ipport = "0.0.0.0:8000";
  log::info!("Starting server on {server_ipport}");

  let listener = tokio::net::TcpListener::bind(server_ipport)
    .await
  .expect("unable to acquire tcp listener");

  axum::serve(listener, router)
  .with_graceful_shutdown(shutdown_signal())
    .await.expect("unable to start axum server");
}

async fn handler() -> impl IntoResponse { "ok" }

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
