use crate::framework;

pub struct Game {
  initialized: bool,
}

impl Game {
  pub const fn new() -> Self {
    Self { initialized: false }
  }
}

impl framework::Game for Game {
  fn update(&mut self, ctx: &mut framework::Context) {
    if self.initialized {
      return;
    }

    let debug_button = framework::gui::Button::new(crate::math::Rect::new(-0.5, -0.5, 1.0, 1.0));
    ctx.gui.add_button(String::from("debug"), debug_button);
    self.initialized = true;
  }
}
