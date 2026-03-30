use protocol::message::{ClientMessageEnvelope, client::ClientMessage};

use crate::engine::network::Network;

pub struct ServerConnection {
  net: Network,
}

impl ServerConnection {
  pub fn new() -> Self {
    

    Self {
      net: Network::new("ws://blackbook.local:8000/ws"),
    }
  }

  pub async fn send(&mut self, msg: ClientMessage) {
    let envelope = ClientMessageEnvelope::new(msg);
    self.net.send(envelope.to_bytes()).await;
  }
}
