use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use reqwest_websocket::{Message, Upgrade, WebSocket};


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

  pub async fn connect(&mut self) {
    let response = reqwest::Client::default()
      .get(self.url.clone())
      .upgrade()
      .send()
      .await
      .unwrap();

    let (ws_sender, _ws_receiver) = response.into_websocket().await.unwrap().split();
    self.tx = Some(ws_sender)
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
