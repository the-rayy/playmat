mod event;
mod input;
mod platform;
pub mod rendering;

use std::sync::Arc;

pub use event::Event;
pub use input::ButtonState;
pub use input::Input;
pub use input::MouseButton;
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
      context: framework::Context::new(),
      renderer: None,
      frame_no: 0,
      input: input::Input::default(),
    }
  }

  pub fn init_rendering(&mut self, window: Arc<Window>) {
    let renderer = platform::runtime::get().block_on(rendering::Renderer::new(window));
    let (w, h) = renderer.get_screen_size();

    self.context.gui.set_screen_dims(w, h);
    self.renderer = Some(renderer);
  }

  pub fn update(&mut self) {
    self.context.handle_input(&self.input);

    self.game.update(&mut self.context);

    self.input.end_of_frame();
  }

  pub fn render(&mut self) {
    self
      .renderer
      .as_ref()
      .expect("renderer not initialized")
      .render(&self.context.gui.get_draw_list(), self.frame_no);
    self.frame_no += 1;
  }

  pub fn handle(&mut self, ev: Event) {
    match ev {
      Event::Noop => (),
      Event::WindowResized => {
        self
          .renderer
          .as_ref()
          .expect("renderer not initialized")
          .resize();
        let (w, h) = self
          .renderer
          .as_ref()
          .expect("renderer not initialized")
          .get_screen_size();
        self.context.gui.set_screen_dims(w, h);
      }
      Event::CursorMoved { x, y } => self.input.set_cursor_pos(x, y),
      Event::MouseButtonPressed { button } => self.input.set_mouse_button(button),
      Event::MouseButtonReleased { button } => self.input.reset_mouse_button(button),
    }
  }
}
