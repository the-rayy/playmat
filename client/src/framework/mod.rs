use protocol::message::{client::ClientMessage, server::ServerMessage};

use crate::engine::{self, rendering::{Renderer, mesh::Mesh}};


pub trait Game {
  fn start(&mut self, ctx: &mut Context);
  fn handle(&mut self, msg: ServerMessage);
}

pub struct Context {
  pub renderer: Renderer,
  pub _tx: tokio::sync::mpsc::Sender<ClientMessage>,
  pub rx: tokio::sync::mpsc::Receiver<ServerMessage>,
}

impl Context {
  pub fn new(renderer: Renderer) -> Self {
    let (tx, rx) = engine::network::connect("ws://blackbook.local:8000/ws");
    Self { renderer, _tx: tx, rx }
  }

  pub fn create_debug_cube(&self) -> Mesh {
    Mesh::debug_cube(&self.renderer.device)
  }
}
