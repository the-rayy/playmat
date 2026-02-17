use std::sync::{Arc, Mutex};

use futures_util::{SinkExt, StreamExt, TryStreamExt};
use reqwest_websocket::{Message, Upgrade};
use shared::{ClientMessage, ClientMessageEnvelope, ServerMessageEnvelope};
use tokio::sync::mpsc;

use crate::{context::Context, platform::runtime};

pub fn init(ctx: Arc<Mutex<Context>>) -> mpsc::Sender<ClientMessage> {
  let (tx, mut rx) = mpsc::channel::<ClientMessage>(10);

  crate::platform::runtime::_spawn_async(async move {
    let response = reqwest::Client::default()
      .get("ws://blackbook.local:8000/ws")
      .upgrade()
      .send()
      .await
      .unwrap();

    let (mut ws_sender, mut ws_receiver) = response.into_websocket().await.unwrap().split();

    runtime::_spawn_async(async move {
      loop {
        match rx.recv().await {
          Some(x) => {
            let envelope = ClientMessageEnvelope::new(x);
            let _ = ws_sender
              .send(Message::Binary(envelope.to_bytes().into()))
              .await;
          }
          None => return,
        }
      }
    });

    while let Some(message) = ws_receiver.try_next().await.unwrap() {
      match message {
        Message::Text(text) => {
          log::info!("received: {text}");
          ctx.lock().unwrap().debug = text;
        }
        Message::Binary(binary) => {
          let env = ServerMessageEnvelope::from_bytes(&binary).unwrap();
          ctx.lock().unwrap().timestamp = Some(env.timestamp());
          match env.msg {
            shared::ServerMessage::Empty => (),
            shared::ServerMessage::SignIn(sign_in_token) => {
              ctx.lock().unwrap().token = Some(sign_in_token.token);
            }
          }
        }
        _ => (),
      }
    }
  });

  tx
}
