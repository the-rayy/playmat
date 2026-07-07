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

    let btn = framework::gui::Button::new(crate::math::Rect::new(-0.1, -0.1, 0.2, 0.2), crate::math::Color::new(0.5, 0.5, 0.5, 1.0));
    ctx.gui.add_button(String::from("cc"), btn);

    let btn = framework::gui::Button::new(crate::math::Rect::new(-0.1, 0.15, 0.2, 0.2), crate::math::Color::new(0.5, 0.5, 0.5, 1.0));
    ctx.gui.add_button(String::from("tc"), btn);

    let btn = framework::gui::Button::new(crate::math::Rect::new(-0.1, -0.35, 0.2, 0.2), crate::math::Color::new(0.5, 0.5, 0.5, 1.0));
    ctx.gui.add_button(String::from("bc"), btn);


    let btn = framework::gui::Button::new(crate::math::Rect::new(0.15, -0.1, 0.2, 0.2), crate::math::Color::new(0.5, 0.5, 0.5, 1.0));
    ctx.gui.add_button(String::from("cr"), btn);

    let btn = framework::gui::Button::new(crate::math::Rect::new(0.15, 0.15, 0.2, 0.2), crate::math::Color::new(0.5, 0.5, 0.5, 1.0));
    ctx.gui.add_button(String::from("tr"), btn);

    let btn = framework::gui::Button::new(crate::math::Rect::new(0.15, -0.35, 0.2, 0.2), crate::math::Color::new(0.5, 0.5, 0.5, 1.0));
    ctx.gui.add_button(String::from("br"), btn);


    let btn = framework::gui::Button::new(crate::math::Rect::new(-0.35, -0.1, 0.2, 0.2), crate::math::Color::new(0.5, 0.5, 0.5, 1.0));
    ctx.gui.add_button(String::from("cl"), btn);

    let btn = framework::gui::Button::new(crate::math::Rect::new(-0.35, 0.15, 0.2, 0.2), crate::math::Color::new(0.5, 0.5, 0.5, 1.0));
    ctx.gui.add_button(String::from("tl"), btn);

    let btn = framework::gui::Button::new(crate::math::Rect::new(-0.35, -0.35, 0.2, 0.2), crate::math::Color::new(0.5, 0.5, 0.5, 1.0));
    ctx.gui.add_button(String::from("bl"), btn);

    self.initialized = true;
  }
}
