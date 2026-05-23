use crate::engine;

#[derive(Default)]
pub struct Game {
}

impl engine::Game for Game {
  fn start(&self, _ctx: &mut engine::Context) {}
}
