use std::sync::{Arc, Mutex};

use protocol::message::server::ServerMessage;

use crate::{engine::rendering::mesh::Mesh, framework};

#[derive(Default)]
pub struct GameStateData {
  user_token: Option<String>,
}

#[derive(Default)]
pub struct GameImpl {
  game_state: Arc<Mutex<GameStateData>>,

  debug_cube: Option<Mesh>
}

impl framework::Game for GameImpl {
  fn start(&mut self, ctx: &mut framework::Context) {
    self.debug_cube = Some(ctx.create_debug_cube());
  }

  fn handle(&mut self, msg: ServerMessage) {
    match msg {
      ServerMessage::Empty => {}
      ServerMessage::SignIn(sign_in_token) => {
        self
          .game_state
          .lock()
          .as_mut()
          .expect("unable to acquire lock")
          .user_token = Some(sign_in_token.token)
      }
    }
  }

  fn render(&self, ctx: &mut framework::Context) {
    ctx.draw(self.debug_cube.as_ref().unwrap(), 0.2);
  }
}
