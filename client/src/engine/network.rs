use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use reqwest_websocket::{Message, Upgrade, WebSocket};

use crate::engine;

pub struct Network {
  tx: SplitSink<WebSocket, Message>,
}

impl Network {
  pub fn new(url: impl Into<String>) -> Self {
    let ws_sender = engine::runtime::get().block_on(async {
      let response = reqwest::Client::default()
        .get(url.into())
        .upgrade()
        .send()
        .await
        .unwrap();

      let (ws_sender, _ws_receiver) = response.into_websocket().await.unwrap().split();
      ws_sender
    });

    Self { tx: ws_sender }
  }

  pub async fn send(&mut self, data: Vec<u8>) {
    let resp = self.tx.send(Message::Binary(data.into())).await;
    log::warn!("{:?}", resp);
  }
}
