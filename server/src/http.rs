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
  let router = Router::new().route("/ws", get(ws_handler));

  let listener = tokio::net::TcpListener::bind(ipport).await?;

  axum::serve(listener, router)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("axum serve failed, but this should never happen");

  Ok(())
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
  log::info!("Websocket connected");
  ws.on_upgrade(async |socket: WebSocket| {
    log::info!("Websocket upgraded");
    let (mut tx, mut rx) = socket.split();

    tokio::spawn(async move {
      log::info!("Websocket thread starting");
      while let Some(msg) = rx.next().await {
        log::debug!("Websocket message received");
        if let Message::Binary(x) = msg.unwrap() {
          let env = ClientMessageEnvelope::from_bytes(&x).unwrap();
          let resp = match env.msg {
            ClientMessage::SignIn(data) => handlers::signin::handler(data).await,
          };

          let env = ServerMessageEnvelope::new(resp);
          if let Err(e) = tx.send(Message::Binary(env.to_bytes().into())).await {
            log::error!("Websocket message sending error: {}", e);
          }
          log::debug!("Websocket message handled");
        } else {
          log::warn!("Websocket message was not binary!");
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
