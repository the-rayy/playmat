use crate::framework;

pub struct Game {
  debug_button: framework::gui::Button,
}

impl Game {
  pub fn new() -> Self {
    Game {
      debug_button: framework::gui::Button::new(crate::math::Rect::new(-0.5, -0.5, 1.0, 1.0)),
    }
  }
}

impl framework::Game for Game {
  fn update(&mut self, ctx: &mut framework::Context) {
    ctx.gui.draw_button(&self.debug_button);
  }
}
