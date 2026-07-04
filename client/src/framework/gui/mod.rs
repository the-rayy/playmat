mod button;

use std::collections::HashMap;

pub use button::Button;

use crate::engine::rendering::canvas::draw_list::DrawList;

#[derive(Default)]
pub struct Context {
  buttons: HashMap<String, Button>,
}

impl Context {
  pub fn add_button(&mut self, id: String, button: Button) {
    self.buttons.insert(id, button);
  }

  pub fn get_draw_list(&self) -> DrawList {
    let mut draw_list = DrawList::default();
    self
      .buttons
      .values()
      .for_each(|b| draw_list.push_rect(&b.rect));
    draw_list
  }
}
