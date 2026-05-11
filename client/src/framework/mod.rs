use protocol::message::{client::ClientMessage, server::ServerMessage};

use crate::engine::{self, rendering::{Renderer, math, mesh::Mesh}};


pub trait Game {
  fn start(&mut self, ctx: &mut Context);
  fn handle(&mut self, msg: ServerMessage);
  fn render(&self, ctx: &mut Context);
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

  pub fn draw(&self, mesh: &Mesh, angle: f32) {
    let mvp = math::compute_mvp(angle, 1920.0/1080.0);
    let test_mvp: [f32; 16] = [
    1.0,  0.0,  0.0,  0.0,
    0.0,  1.0,  0.0,  0.0,
    0.0,  0.0, -1.0, -1.0,
    0.0,  0.0, -0.2,  0.0,
];
    self.renderer.render(test_mvp, mesh);
  }
}
