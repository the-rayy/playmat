use crate::framework;

mod windows;

pub struct GameImpl {}

impl framework::Game for GameImpl {
  fn start(&self, ctx: &mut framework::Context) {
    let tx = ctx.get_client_message_tx();

    let auth_window = windows::login::Window::new(tx);
    ctx.window_manager.add(auth_window);
  }
}
