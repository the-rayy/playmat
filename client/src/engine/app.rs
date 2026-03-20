use std::sync::Arc;

use winit::{
  application::ApplicationHandler,
  event::WindowEvent,
  window::Window,
};

use crate::engine::rendering::Renderer;

pub struct App {
  window: Option<Arc<Window>>,
  renderer: Option<Renderer>,
}

impl App {
  pub fn new() -> App {
    App {
      window: None,
      renderer: None,
    }
  }
}

impl ApplicationHandler for App {
  fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
    let window = event_loop
      .create_window(super::platform::window::attributes())
      .expect("could not create window");
    let window = Arc::new(window);
    let renderer = super::runtime::get().block_on(Renderer::new(window.clone()));

    self.window = Some(window);
    self.renderer = Some(renderer);
  }

  fn window_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    _window_id: winit::window::WindowId,
    event: winit::event::WindowEvent,
  ) {
    match event {
      WindowEvent::CloseRequested => event_loop.exit(),
      WindowEvent::Resized(size) => self.renderer.as_mut().unwrap().resize(size),
      WindowEvent::RedrawRequested => {
        self.renderer.as_mut().unwrap().render();
        self.window.as_mut().unwrap().request_redraw();
      }
      _ => (),
    }
  }
}
