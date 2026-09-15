use std::sync::Arc;

use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};

use framework::{self, Game};

pub struct App<T: Game> {
  window: Option<Arc<Window>>,
  engine: framework::engine::Engine<T>,
}

impl<T: Game> App<T> {
  pub const fn new(engine: framework::engine::Engine<T>) -> Self {
    Self {
      window: None,
      engine,
    }
  }
}

impl<T: Game> ApplicationHandler for App<T> {
  fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
    let window = event_loop
      .create_window(engine::platform::window::attributes())
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
