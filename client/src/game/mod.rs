use crate::engine;

#[derive(Default)]
pub struct Game {}

impl engine::Game for Game {
  fn update(&mut self, ctx: &mut engine::context::Context) {
    ctx.gui_draw_list.debug_push();
  }
}
