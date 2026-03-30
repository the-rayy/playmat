use protocol::message::client::ClientMessage;
use tokio::sync::mpsc::{self, Sender};

use crate::engine::runtime;

pub mod network;
pub mod window_manager;

pub trait Game {
  fn start(&self, ctx: &mut Context);
}

pub struct Context {
  pub window_manager: window_manager::WindowManager,

  client_message_tx: Sender<ClientMessage>,
}

impl Context {
  pub fn new() -> Self {
    let (tx, mut rx) = mpsc::channel::<ClientMessage>(10);

    let mut net = network::ServerConnection::new();

    runtime::_spawn_async(async move {
      net.connect().await;
      loop {
        match rx.recv().await {
          Some(x) => {
            let _ = net.send(x).await;
          }
          None => return,
        }
      }
    });

    Self {
      window_manager: Default::default(),
      client_message_tx: tx,
    }
  }

  pub fn get_client_message_tx(&self) -> Sender<ClientMessage> {
    self.client_message_tx.clone()
  }
}
