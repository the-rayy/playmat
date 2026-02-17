use std::sync::{Arc, Mutex};

use futures_util::{SinkExt, StreamExt, TryStreamExt};
use reqwest_websocket::{Message, Upgrade};
use shared::Envelope;
use tokio::sync::mpsc;

use crate::{context::Context, platform::runtime};

pub fn init(ctx: Arc<Mutex<Context>>) -> mpsc::Sender<i64> {
  let (mut tx, mut rx) = mpsc::channel::<i64>(10);

  crate::platform::runtime::_spawn_async(async move {
    let response = reqwest::Client::default()
      .get("ws://blackbook.local:8000/ws")
      .upgrade()
      .send()
      .await
      .unwrap();

    let (mut ws_sender, mut ws_receiver) = response.into_websocket().await.unwrap().split();

    runtime::_spawn_async(async move {
      loop { match rx.recv().await {
        Some(x) => {
            let envelope = Envelope::new();
            let _ = ws_sender.send(Message::Binary(envelope.to_bytes().into())).await;
        },
        None => return,
      }}
    });

    while let Some(message) = ws_receiver.try_next().await.unwrap() {
      match message {
        Message::Text(text) => {
          log::info!("received: {text}");
          ctx.lock().unwrap().debug = text;
        }
        Message::Binary(binary) => {
          let env = Envelope::from_bytes(&binary).unwrap();
          ctx.lock().unwrap().timestamp = Some(env.timestamp());
          log::info!("received: {:?}", env);
        }
        _ => (),
      }
    }
  });

  tx
}
