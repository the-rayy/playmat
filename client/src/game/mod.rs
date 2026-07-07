use crate::framework::{self, Event, gui};

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
      for ev in ctx.events() {
        match ev {
          Event::Gui(gui::Event::ButtonClicked { id }) => log::info!("button {} clicked", id),
        }
      }

      return;
    }

    let debug_button = framework::gui::Button::new(crate::math::Rect::new(-0.5, -0.5, 1.0, 1.0));
    ctx.gui.add_button(String::from("debug"), debug_button);
    self.initialized = true;
  }
}
