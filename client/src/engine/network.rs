use futures_util::{SinkExt, StreamExt, TryStreamExt};
use protocol::message::{ServerMessageEnvelope, client::ClientMessage, server::ServerMessage};
use reqwest_websocket::{Message, Upgrade};

use crate::engine::runtime;

pub fn connect(
  url: impl Into<String>,
) -> (
  tokio::sync::mpsc::Sender<ClientMessage>,
  tokio::sync::mpsc::Receiver<ServerMessage>,
) {
  let url = url.into();
  let (server_tx, server_rx) = tokio::sync::mpsc::channel::<ServerMessage>(10);
  let (client_tx, mut client_rx) = tokio::sync::mpsc::channel::<ClientMessage>(10);
  runtime::_spawn_async(async move {
    let response = reqwest::Client::default()
      .get(url)
      .upgrade()
      .send()
      .await
      .expect("unable to connect to server");

    let (mut ws_sender, mut ws_receiver) = response
      .into_websocket()
      .await
      .expect("unable to open websocket")
      .split();
    runtime::_spawn_async(async move {
      loop {
        match ws_receiver.try_next().await {
          Err(e) => {
            log::error!("Websocket receiver error: {}", e);
            break;
          }
          Ok(None) => {
            log::warn!("Websocket receiver stream ended");
            break;
          }
          Ok(Some(Message::Binary(binary))) => {
            let env = match ServerMessageEnvelope::from_bytes(&binary) {
              Ok(e) => e,
              Err(e) => {
                log::error!("Failed to decode server message: {}", e);
                continue;
              }
            };

            if let Err(e) = server_tx.send(env.unpack()).await {
              log::error!("Failed to forward server message: {}", e);
            }
          }
          Ok(Some(_)) => {
            log::warn!("Received non-binary websocket message, ignoring");
          }
        }
      }
    });

    runtime::_spawn_async(async move {
      while let Some(msg) = client_rx.recv().await {
        if let Err(e) = ws_sender
          .send(Message::Binary(msg.pack().to_bytes().into()))
          .await
        {
          log::error!("Failed to send websocket message: {}", e);
          break;
        }
      }
      log::warn!("Client message channel closed");
    });
  });

  (client_tx, server_rx)
}
