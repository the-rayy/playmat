use futures_util::{SinkExt, StreamExt, stream::SplitSink, TryStreamExt};
use reqwest_websocket::{Message, Upgrade, WebSocket};

use crate::engine::runtime;

pub struct Network {
  url: String,
  tx: Option<SplitSink<WebSocket, Message>>,
}

impl Network {
  pub fn new(url: impl Into<String>) -> Self {
    Self {
      url: url.into(),
      tx: None,
    }
  }

  pub async fn connect(&mut self) -> tokio::sync::mpsc::Receiver<Vec<u8>> {
    let response = reqwest::Client::default()
      .get(self.url.clone())
      .upgrade()
      .send()
      .await
      .unwrap();

    let (ws_sender, mut ws_receiver) = response.into_websocket().await.unwrap().split();
    self.tx = Some(ws_sender);

    let (mut tx, mut rx) = tokio::sync::mpsc::channel::<Vec<u8>>(10);
    runtime::_spawn_async(async move {
      while let Some(Message::Binary(binary)) = ws_receiver.try_next().await.unwrap() {
        let _ = tx.send(binary.into());
      }
    });

    rx
  }

  pub async fn send(&mut self, data: Vec<u8>) {
    if self.tx.is_none() {
      panic!("network not connected. please handle this gracefully");
    }

    let resp = self
      .tx
      .as_mut()
      .unwrap()
      .send(Message::Binary(data.into()))
      .await;
    log::warn!("{:?}", resp);
  }
}
