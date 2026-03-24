use crate::engine::gui::Draw;

#[derive(Default)]
pub struct WindowManager {
  windows: Vec<Box<dyn Draw>>,
}

impl WindowManager {
  pub fn add<T: Draw + 'static>(&mut self, window: T) {
    self.windows.push(Box::new(window))
  }

  pub fn get_current(&mut self) -> &mut Vec<Box<dyn Draw>> {
    &mut self.windows
  }
}
