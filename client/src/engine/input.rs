use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Input {
  cursor_pos: CursorPos,
  //TODO needs better impl
  mouse_buttons: HashMap<MouseButton, ButtonState>,
}

impl Input {
  pub const fn set_cursor_pos(&mut self, x: i32, y: i32) {
    self.cursor_pos.x = x;
    self.cursor_pos.y = y;
  }

  pub fn set_mouse_button(&mut self, button: MouseButton) {
    self
      .mouse_buttons
      .entry(button)
      .and_modify(|state| *state = ButtonState::Pressed)
      .or_insert(ButtonState::Pressed);
  }

  pub fn reset_mouse_button(&mut self, button: MouseButton) {
    self
      .mouse_buttons
      .entry(button)
      .and_modify(|state| *state = ButtonState::Released)
      .or_insert(ButtonState::Released);
  }

  pub fn end_of_frame(&mut self) {
    for state in self.mouse_buttons.values_mut() {
      match state {
        ButtonState::Pressed => *state = ButtonState::Down,
        ButtonState::Released => *state = ButtonState::Up,
        _ => (),
      }
    }
  }
}

#[derive(Default, Debug)]
pub struct CursorPos {
  pub x: i32,
  pub y: i32,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum MouseButton {
  Left,
  Right,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ButtonState {
  Up,
  Pressed,
  Down,
  Released,
}
