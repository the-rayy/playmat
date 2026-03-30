use protocol::message::{ClientMessageEnvelope, ServerMessageEnvelope, client::ClientMessage, server::ServerMessage};

use futures_util::{SinkExt, StreamExt, stream::SplitSink, TryStreamExt};
use crate::engine::{network::Network, runtime};

pub struct ServerConnection {
  net: Network,
}

impl ServerConnection {
  pub fn new() -> Self {
    Self {
      net: Network::new("ws://blackbook.local:8000/ws"),
    }
  }

  pub async fn connect(&mut self) -> tokio::sync::mpsc::Receiver<ServerMessage> {
    let mut ws_rx = self.net.connect().await;
    let (tx, rx) = tokio::sync::mpsc::channel::<ServerMessage>(10);
    
    runtime::_spawn_async(async move {
      while let Some(binary) = ws_rx.recv().await {
        let env = ServerMessageEnvelope::from_bytes(&binary).unwrap();
        let _ = tx.send(env.msg);
      }
    });

    rx
  }

  pub async fn send(&mut self, msg: ClientMessage) {
    let envelope = ClientMessageEnvelope::new(msg);
    self.net.send(envelope.to_bytes()).await;
  }
}
