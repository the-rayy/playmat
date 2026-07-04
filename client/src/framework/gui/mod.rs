mod button;

use std::collections::HashMap;

pub use button::Button;

use crate::engine::{ButtonState, CursorPos, Input, rendering::canvas::draw_list::DrawList};

#[derive(Default)]
pub struct Context {
  screen_width: u32,
  screen_height: u32,
  buttons: HashMap<String, Button>,
}

impl Context {
  pub fn set_screen_dims(&mut self, w: u32, h: u32) {
    self.screen_width = w;
    self.screen_height = h;
  }

  pub fn add_button(&mut self, id: String, button: Button) {
    self.buttons.insert(id, button);
  }

  pub fn get_draw_list(&self) -> DrawList {
    let mut draw_list = DrawList::default();
    self
      .buttons
      .values()
      .for_each(|b| draw_list.push_rect(&b.rect, b.color()));
    draw_list
  }

  pub fn handle_input(&mut self, input: &Input) {
    if self.screen_width == 0 || self.screen_height == 0 {
      return;
    }

    self.buttons.values_mut().for_each(|b| {
      let hover = b.rect.contains(
        &input
          .get_cursor_pos()
          .into_screen_space(self.screen_width, self.screen_height),
      );

      b.state = if hover
        && input.get_mouse_button(crate::engine::MouseButton::Left) == &ButtonState::Down
      {
        button::State::Down
      } else if hover {
        button::State::Hovered
      } else {
        button::State::Neutral
      };
    });

    log::info!("{:?}", &self.buttons);
  }
}
