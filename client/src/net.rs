use std::sync::{Arc, Mutex};

use futures_util::TryStreamExt;
use reqwest_websocket::{Message, Upgrade};
use shared::Envelope;

use crate::context::Context;

pub fn init(ctx: Arc<Mutex<Context>>) {
  crate::platform::runtime::_spawn_async(async move {
    let response = reqwest::Client::default()
      .get("ws://blackbook.local:8000/ws")
      .upgrade()
      .send()
      .await
      .unwrap();

    let mut ws = response.into_websocket().await.unwrap();

    while let Some(message) = ws.try_next().await.unwrap() {
      match message {
      Message::Text(text) => {
        log::info!("received: {text}");
        ctx.lock().unwrap().debug = text;
      },
      Message::Binary(binary) => {
        let env = Envelope::from_bytes(&binary).unwrap();
        log::info!("received: {:?}", env);
      },
      _ => (),
      }
    }
  });
}
