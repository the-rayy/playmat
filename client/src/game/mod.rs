use std::sync::{Arc, Mutex};

use protocol::message::server::ServerMessage;

use crate::framework;


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
  }

  fn handle(&mut self, msg: ServerMessage) {
    match msg {
      ServerMessage::Empty => todo!(),
      ServerMessage::SignIn(sign_in_token) => {
        self.game_state.lock().as_mut().unwrap().user_token = Some(sign_in_token.token)
      }
    }
  }
}
