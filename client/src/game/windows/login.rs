use protocol::message::client::{ClientMessage, SignInCredentials};
use tokio::sync::mpsc::Sender;

use crate::{engine::gui, game::GameState};

pub struct Window {
  username: String,
  tx: Sender<ClientMessage>,
  game_state: GameState,
}

impl Window {
  pub fn new(tx: Sender<ClientMessage>, game_state: GameState) -> Window {
    Window {
      username: Default::default(),
      tx,
      game_state,
    }
  }
}

impl gui::Draw for Window {
  fn draw(&mut self, ctx: &egui::Context) {
    egui::Window::new("Login")
      .default_open(true)
      .movable(false)
      .auto_sized()
      .title_bar(false)
      .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
      .show(ctx, |ui: &mut egui::Ui| {
        if let Some(token) = &self.game_state.lock().unwrap().user_token {
          ui.label(format!("Logged in as {token}"));
        } else {
          ui.label("username");
          ui.text_edit_singleline(&mut self.username);

          if ui.button("login").clicked() {
            let msg = ClientMessage::SignIn(SignInCredentials {
              username: self.username.clone(),
            });
            let _ = self.tx.blocking_send(msg);
          };
        }
      });
  }
}
