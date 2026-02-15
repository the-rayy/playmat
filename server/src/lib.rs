use std::time::Duration;

use axum::{
  Router,
  extract::{
    WebSocketUpgrade,
    ws::{Message, WebSocket},
  },
  response::IntoResponse,
  routing::get,
};
use shared::{Data, Envelope};
use tokio::time::sleep;

pub async fn run_server() {
  let router = Router::new()
    .route("/", get(handler))
    .route("/ws", get(ws_handler));

  let server_ipport = "0.0.0.0:8000";
  log::info!("Starting server on {server_ipport}");

  let listener = tokio::net::TcpListener::bind(server_ipport)
    .await
    .expect("unable to acquire tcp listener");

  axum::serve(listener, router)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("unable to start axum server");
}

async fn handler() -> impl IntoResponse {
  "ok"
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
  ws.on_upgrade(async |mut socket: WebSocket| {
    tokio::spawn(async move {
      let mut i: u64 = 0;

      loop {
        let env = Envelope { data: Data::Empty };
        let _ = socket.send(Message::Binary(env.to_bytes().into())).await;
        let _ = socket.send(Message::Text(format!("{}", i).into())).await;
        i += 1;
        sleep(Duration::from_secs(1)).await;
      }
    });
  })
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
