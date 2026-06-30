pub mod gui;

pub trait Game {
  fn update(&mut self, ctx: &mut Context);
}

#[derive(Default)]
pub struct Context {
  pub gui: gui::Context,
}
