use protocol::message::{client::ClientMessage, server::ServerMessage};

use crate::engine::{self};

pub mod window_manager;

pub trait Game {
  fn start(&self, ctx: &mut Context);
  fn handle(&mut self, msg: ServerMessage);
}

pub struct Context {
  pub window_manager: window_manager::WindowManager,
  pub tx: tokio::sync::mpsc::Sender<ClientMessage>,
  pub rx: tokio::sync::mpsc::Receiver<ServerMessage>,
}

impl Context {
  pub fn new() -> Self {
    let (tx, rx) = engine::network::connect("ws://blackbook.local:8000/ws");
    Self {
      window_manager: Default::default(),
      tx,
      rx,
    }
  }
}
