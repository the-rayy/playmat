use std::collections::HashMap;

use crate::math::Point;

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

  pub fn get_cursor_pos(&self) -> &CursorPos {
    &self.cursor_pos
  }

  pub fn get_mouse_button(&self, button: MouseButton) -> &ButtonState {
    self.mouse_buttons.get(&button).unwrap_or(&ButtonState::Up)
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

impl CursorPos {
  pub fn into_screen_space(&self, w: u32, h: u32) -> Point {
    Point {
      x: 2_f32 * self.x as f32 / w as f32 - 1_f32,
      y: 2_f32 * self.y as f32 / h as f32 - 1_f32,
    }
  }
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
