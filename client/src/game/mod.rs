use std::sync::{Arc, Mutex};

use protocol::message::server::ServerMessage;

use crate::framework;

mod windows;

#[derive(Default)]
pub struct GameStateData {
  user_token: Option<String>,
}

pub type GameState = Arc<Mutex<GameStateData>>;

#[derive(Default)]
pub struct GameImpl {
  game_state: Arc<Mutex<GameStateData>>,
}

impl framework::Game for GameImpl {
  fn start(&self, ctx: &mut framework::Context) {

    let auth_window = windows::login::Window::new(ctx.tx.clone(), self.game_state.clone());
    ctx.window_manager.add(auth_window);
  }

  fn handle(&mut self, msg: ServerMessage) {
  }
}
