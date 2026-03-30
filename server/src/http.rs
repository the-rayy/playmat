use axum::{
  Router,
  extract::{
    WebSocketUpgrade,
    ws::{Message, WebSocket},
  },
  response::IntoResponse,
  routing::get,
};

use futures_util::{SinkExt, StreamExt};
use protocol::message::{ClientMessageEnvelope, ServerMessageEnvelope, client::ClientMessage};
use thiserror::Error as Thiserror;

use crate::handlers;

#[derive(Thiserror, Debug)]
pub enum Error {
  #[error("")]
  InitializationError(#[from] std::io::Error),
}

pub async fn run(ipport: &str) -> Result<(), Error> {
  let router = Router::new()
    .route("/", get(handler))
    .route("/ws", get(ws_handler));

  let listener = tokio::net::TcpListener::bind(ipport).await?;

  axum::serve(listener, router)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("axum serve failed, but this should never happen");

  Ok(())
}

async fn handler() -> impl IntoResponse {
  "ok"
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
  log::debug!("Running websocket");
  ws.on_upgrade(async |socket: WebSocket| {
    log::debug!("Upgrading websocket");
    let (mut tx, mut rx) = socket.split();
    tokio::spawn(async move {
      while let Some(msg) = rx.next().await {
        if let Message::Binary(x) = msg.unwrap() {
          let env = ClientMessageEnvelope::from_bytes(&x).unwrap();
          let resp = match env.msg {
            ClientMessage::SignIn(data) => handlers::signin::handler(data).await,
          };

          let env = ServerMessageEnvelope::new(resp);
          let _ = tx.send(Message::Binary(env.to_bytes().into())).await;
        }
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
