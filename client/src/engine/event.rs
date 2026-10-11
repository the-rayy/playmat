use crate::engine::input;

#[derive(Debug)]
pub enum Event {
  Noop,
  WindowResized,
  CursorMoved { x: i32, y: i32 },
  MouseButtonPressed { button: input::MouseButton },
  MouseButtonReleased { button: input::MouseButton },
}
