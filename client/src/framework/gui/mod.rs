mod button;

pub use button::Button;

use crate::engine::rendering::canvas::draw_list::DrawList;

#[derive(Default)]
pub struct Context {
  pub draw_list: DrawList,
}

impl Context {
  pub fn draw_button(&mut self, button: &Button) {
    self.draw_list.push_rect(&button.rect);
  }
}
