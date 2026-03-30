use futures_util::{SinkExt, StreamExt, TryStreamExt};
use protocol::message::{
  ClientMessageEnvelope, ServerMessageEnvelope, client::ClientMessage, server::ServerMessage,
};
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
      .unwrap();

    let (mut ws_sender, mut ws_receiver) = response.into_websocket().await.unwrap().split();

    runtime::_spawn_async(async move {
      while let Some(Message::Binary(binary)) = ws_receiver.try_next().await.unwrap() {
        let env = ServerMessageEnvelope::from_bytes(&binary).unwrap();
        let _ = server_tx.send(env.msg);
      }
    });

    runtime::_spawn_async(async move {
      while let Some(msg) = client_rx.recv().await {
        let env = ClientMessageEnvelope::new(msg);
        let _ = ws_sender.send(Message::Binary(env.to_bytes().into())).await;
      }
    });
  });

  (client_tx, server_rx)
}
