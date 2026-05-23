use std::sync::Arc;

use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};

use crate::{
  engine::{Engine, Game},
};

pub struct App<T: Game> {
  window: Option<Arc<Window>>,
  engine: Engine<T>,
}

impl<T: Game> App<T> {
  pub fn new(engine: Engine<T>) -> Self {
    Self { window: None, engine: engine }
  }
}

impl<T: Game> ApplicationHandler for App<T> {
  fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
    let window = event_loop
      .create_window(crate::engine::window::attributes())
      .expect("could not create window");
    let window = Arc::new(window);
    self.engine.init_rendering(window.clone());
    self.window = Some(window.clone());
  }

  fn window_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    _window_id: winit::window::WindowId,
    event: winit::event::WindowEvent,
  ) {
    match event {
      WindowEvent::CloseRequested => event_loop.exit(),
      // WindowEvent::Resized(size) => self.renderer.resize(size),
      WindowEvent::RedrawRequested => {
        self.engine.update();
        self.engine.render();
        self.window.as_ref().expect("requestind redraw on non-existent window").request_redraw();
      }
      _ => (),
    }
  }
}

