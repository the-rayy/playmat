use crate::{engine::rendering::canvas::draw_list::DrawList, framework::gui::Button};
pub mod gui;

pub trait Game {
  fn update(&mut self, ctx: &mut Context);
}

#[derive(Default)]
pub struct Context {
  pub gui: GuiContext,
}

#[derive(Default)]
pub struct GuiContext {
  pub draw_list: DrawList,
}

impl GuiContext {
  pub fn draw_button(&mut self, button: &Button) {
    self.draw_list.push_rect(&button.rect);
  }
}
