pub mod window_manager;

pub trait Game {
  fn start(&self, ctx: &mut Context);
}

#[derive(Default)]
pub struct Context {
  pub window_manager: window_manager::WindowManager,
}
