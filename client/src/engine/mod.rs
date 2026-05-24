mod event;
pub mod network;
mod platform;
pub mod rendering;

use std::sync::Arc;

pub use event::Event;
pub use platform::logger;
pub use platform::runtime;
pub use platform::window;
use winit::window::Window;

pub trait Game {}

#[derive(Default)]
pub struct Context {}

pub struct Engine<T: Game> {
  game: T,
  context: Context,

  renderer: Option<rendering::Renderer>,
  frame_no: u64,
}

impl<T: Game> Engine<T> {
  pub fn new(game: T) -> Self {
    runtime::init();
    logger::init();

    Self {
      game,
      context: Context::default(),
      renderer: None,
      frame_no: 0,
    }
  }

  pub fn init_rendering(&mut self, window: Arc<Window>) {
    self.renderer = Some(platform::runtime::get().block_on(rendering::Renderer::new(window)));
  }

  pub fn update(&mut self) {}
  pub fn render(&mut self) {
    self
      .renderer
      .as_ref()
      .expect("renderer not initialized")
      .render(self.frame_no);
    self.frame_no += 1;
  }

  pub fn handle(&mut self, ev: Event) {
    match ev {
      Event::Noop => (),
      Event::WindowResized => self
        .renderer
        .as_ref()
        .expect("renderer not initialized")
        .resize(),
    }
  }
}
