#[derive(Debug)]
pub enum Event {
  Noop,
  WindowResized,
  CursorMoved { x: i32, y: i32},
  MouseButtonPressed { button: MouseButton },
  MouseButtonReleased { button: MouseButton }
}

#[derive(Debug)]
pub enum MouseButton {
  Left,
  Right,
}
