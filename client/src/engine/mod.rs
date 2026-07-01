mod event;
mod input;
mod platform;
pub mod rendering;

use std::sync::Arc;

pub use event::Event;
pub use event::MouseButton;
pub use platform::logger;
pub use platform::runtime;
pub use platform::window;
use winit::window::Window;

use crate::framework;

pub struct Engine<T: framework::Game> {
  game: T,
  context: framework::Context,

  renderer: Option<rendering::Renderer>,
  input: input::Input,
  frame_no: u64,
}

impl<T: framework::Game> Engine<T> {
  pub fn new(game: T) -> Self {
    runtime::init();
    logger::init();

    Self {
      game,
      context: framework::Context::default(),
      renderer: None,
      frame_no: 0,
      input: input::Input::default(),
    }
  }

  pub fn init_rendering(&mut self, window: Arc<Window>) {
    self.renderer = Some(platform::runtime::get().block_on(rendering::Renderer::new(window)));
  }

  pub fn update(&mut self) {
    self.context.gui.draw_list.clear();
    self.game.update(&mut self.context);
  }

  pub fn render(&mut self) {
    self
      .renderer
      .as_ref()
      .expect("renderer not initialized")
      .render(&self.context.gui.draw_list, self.frame_no);
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
      Event::CursorMoved { x, y } => self.input.set_cursor_pos(x, y),
      Event::MouseButtonPressed { button } => log::info!("mouse button pressed: {:?}", button),
      Event::MouseButtonReleased { button } => log::info!("mouse button released: {:?}", button),
    }
  }
}
