use std::sync::{Arc, Mutex};

use winit::{
  application::ApplicationHandler,
  dpi::PhysicalSize,
  event::WindowEvent,
  window::{Window, WindowAttributes},
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
        self.window.as_mut().unwrap().request_redraw();
      }
      _ => (),
    }
  }
}

fn window_attributes() -> WindowAttributes {
  let mut window_attributes = Window::default_attributes();
  let size = winit::dpi::Size::Physical(PhysicalSize::new(1920, 1080));
  window_attributes = window_attributes.with_inner_size(size);
  #[cfg(target_arch = "wasm32")]
  {
    use wasm_bindgen::JsCast;
    use winit::platform::web::WindowAttributesExtWebSys;

    const CANVAS_ID: &str = "canvas";

    let window = wgpu::web_sys::window().expect("Unable to get window");
    let document = window.document().expect("Unable to get document");
    let canvas = document
      .get_element_by_id(CANVAS_ID)
      .expect("Unable to get canvas");
    let html_canvas_element = canvas.unchecked_into();
    window_attributes = window_attributes.with_canvas(Some(html_canvas_element));
  }

  window_attributes
}
