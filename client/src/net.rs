use std::sync::{Arc, Mutex};

use futures_util::TryStreamExt;
use reqwest_websocket::{Message, Upgrade};

use crate::context::Context;

pub fn init(ctx: Arc<Mutex<Context>>) {
  crate::platform::runtime::_spawn_async(async move {
    let response = reqwest::Client::default()
      .get("ws://localhost:8000/ws")
      .upgrade()
      .send()
      .await
      .unwrap();

    let mut ws = response.into_websocket().await.unwrap();

    while let Some(message) = ws.try_next().await.unwrap() {
      if let Message::Text(text) = message {
        log::info!("received: {text}");
        ctx.lock().unwrap().debug = text;
      }
    }
  });
}
