use winit::event::{ElementState, MouseButton, WindowEvent};

use crate::input;

#[derive(Debug)]
pub enum Event {
  Noop,
  WindowResized,
  CursorMoved { x: i32, y: i32 },
  MouseButtonPressed { button: input::MouseButton },
  MouseButtonReleased { button: input::MouseButton },
}

impl From<WindowEvent> for Event {
  fn from(value: winit::event::WindowEvent) -> Self {
    match value {
      WindowEvent::Resized(_) => Self::WindowResized,
      WindowEvent::CursorMoved {
        device_id: _,
        position,
      } => Self::CursorMoved {
        x: position.x as i32,
        y: position.y as i32,
      },
      WindowEvent::MouseInput {
        device_id: _,
        state: ElementState::Pressed,
        button: MouseButton::Left,
      } => Self::MouseButtonPressed {
        button: input::MouseButton::Left,
      },
      WindowEvent::MouseInput {
        device_id: _,
        state: ElementState::Pressed,
        button: MouseButton::Right,
      } => Self::MouseButtonPressed {
        button: input::MouseButton::Right,
      },
      WindowEvent::MouseInput {
        device_id: _,
        state: ElementState::Released,
        button: MouseButton::Left,
      } => Self::MouseButtonReleased {
        button: input::MouseButton::Left,
      },
      WindowEvent::MouseInput {
        device_id: _,
        state: ElementState::Released,
        button: MouseButton::Right,
      } => Self::MouseButtonReleased {
        button: input::MouseButton::Right,
      },
      _ => Self::Noop,
    }
  }
}
