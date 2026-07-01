use std::sync::Arc;

use winit::{
  application::ApplicationHandler,
  event::{ElementState, MouseButton, WindowEvent},
  window::Window,
};

use crate::{
  engine::{self, Engine},
  framework::Game,
};

pub struct App<T: Game> {
  window: Option<Arc<Window>>,
  engine: Engine<T>,
}

impl<T: Game> App<T> {
  pub const fn new(engine: Engine<T>) -> Self {
    Self {
      window: None,
      engine,
    }
  }
}

impl<T: Game> ApplicationHandler for App<T> {
  fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
    let window = event_loop
      .create_window(crate::engine::window::attributes())
      .expect("could not create window");
    let window = Arc::new(window);
    self.engine.init_rendering(window.clone());
    self.window = Some(window);
  }

  fn window_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    _window_id: winit::window::WindowId,
    event: winit::event::WindowEvent,
  ) {
    match event {
      WindowEvent::CloseRequested => event_loop.exit(),
      WindowEvent::RedrawRequested => {
        self.engine.update();
        self.engine.render();
        self
          .window
          .as_ref()
          .expect("requestind redraw on non-existent window")
          .request_redraw();
      }
      event => self.engine.handle(event.into()),
    }
  }
}

impl From<winit::event::WindowEvent> for engine::Event {
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
        button: engine::MouseButton::Left,
      },
      WindowEvent::MouseInput {
        device_id: _,
        state: ElementState::Pressed,
        button: MouseButton::Right,
      } => Self::MouseButtonPressed {
        button: engine::MouseButton::Right,
      },
      WindowEvent::MouseInput {
        device_id: _,
        state: ElementState::Released,
        button: MouseButton::Left,
      } => Self::MouseButtonReleased {
        button: engine::MouseButton::Left,
      },
      WindowEvent::MouseInput {
        device_id: _,
        state: ElementState::Released,
        button: MouseButton::Right,
      } => Self::MouseButtonReleased {
        button: engine::MouseButton::Right,
      },
      _ => Self::Noop,
    }
  }
}
