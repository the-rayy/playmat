use std::collections::HashMap;

use crate::engine::gui::Draw;

#[derive(Default)]
pub struct WindowManager {
  windows: HashMap<String, Box<dyn Draw + 'static>>,
}

impl WindowManager {
  pub fn add<T: Draw + 'static>(&mut self, id: String, window: T) {
    self.windows.insert(id, Box::new(window));
  }
}

impl Draw for WindowManager {
  fn draw(&mut self, ctx: &egui::Context) {
    self.windows.values_mut().for_each(|w| w.draw(ctx));
  }
}
