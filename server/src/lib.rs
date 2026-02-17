use std::{sync::Arc, time::Duration};
use tokio::sync::{Mutex};

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
use shared::{ClientMessage, ClientMessageEnvelope, ServerMessage, ServerMessageEnvelope, SignInCredentials, SignInToken};
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

async fn handle_signin(data: SignInCredentials) -> ServerMessage {
  sleep(Duration::from_secs(3)).await;
  ServerMessage::SignIn(SignInToken{
    token: String::from("asd"),
  })
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
  ws.on_upgrade(async |socket: WebSocket| {
    let (mut ws_sender, mut ws_receiver) = socket.split();

    let sender = Arc::new(Mutex::new(ws_sender));

    let ws_sender = sender.clone();
    tokio::spawn(async move {
      while let Some(msg) = ws_receiver.next().await {
        if let Message::Binary(x) = msg.unwrap() {
          let env = ClientMessageEnvelope::from_bytes(&x).unwrap();
          log::debug!("Received: {:?}", env);

          let resp = match env.msg {
            ClientMessage::SignIn(data) => handle_signin(data).await,
            _ => unreachable!(),
          };

          let env = ServerMessageEnvelope::new(resp);
          let _ = ws_sender.lock().await.send(Message::Binary(env.to_bytes().into())).await;
        }
      }
    });

    let ws_sender = sender.clone();
    tokio::spawn(async move {
      let mut i: u64 = 0;

      loop {
        let env = ServerMessageEnvelope::new(ServerMessage::Empty);
        let _ = ws_sender.lock().await.send(Message::Binary(env.to_bytes().into())).await;
        let _ = ws_sender.lock().await.send(Message::Text(format!("{}", i).into())).await;
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
