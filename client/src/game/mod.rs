use crate::framework;

mod windows;

pub struct GameImpl {}

impl framework::Game for GameImpl {
  fn start(&self, ctx: &mut framework::Context) {
    let auth_window = windows::login::Window::new();
    ctx.window_manager.add(auth_window);
  }
}
