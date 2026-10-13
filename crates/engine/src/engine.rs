use std::sync::Arc;

pub use crate::input::MouseButton;
pub use crate::platform::logger;
pub use crate::platform::runtime;
use winit::event::ElementState;
use winit::event::WindowEvent;
use winit::window::Window;

use crate::framework;

pub struct Engine<T: framework::Game> {
  game: T,
  context: framework::Context,

  renderer: Option<crate::rendering::Renderer>,
  input: crate::input::Input,
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
      input: crate::input::Input::default(),
    }
  }

  pub fn init_rendering(&mut self, window: Arc<Window>) {
    let renderer =
      crate::platform::runtime::get().block_on(crate::rendering::Renderer::new(window));
    let (w, h) = renderer.get_screen_size();

    self.context.gui.set_screen_dims(w, h);
    self.renderer = Some(renderer);
  }

  pub fn update(&mut self) {
    self.context.handle_input(&self.input);

    for (key, tex) in self.context.assets.get() {
      self
        .renderer
        .as_mut()
        .expect("renderer not initialized")
        .load_texture(key.clone(), tex);
    }

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

  pub fn handle(&mut self, ev: WindowEvent) {
    match ev {
      WindowEvent::Resized(_) => {
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
      WindowEvent::CursorMoved {
        device_id: _,
        position,
      } => self
        .input
        .set_cursor_pos(position.x as i32, position.y as i32),
      WindowEvent::MouseInput {
        device_id: _,
        state: ElementState::Pressed,
        button: winit::event::MouseButton::Left,
      } => self.input.set_mouse_button(MouseButton::Left),
      WindowEvent::MouseInput {
        device_id: _,
        state: ElementState::Pressed,
        button: winit::event::MouseButton::Right,
      } => self.input.set_mouse_button(MouseButton::Right),
      WindowEvent::MouseInput {
        device_id: _,
        state: ElementState::Released,
        button: winit::event::MouseButton::Left,
      } => self.input.reset_mouse_button(MouseButton::Left),
      WindowEvent::MouseInput {
        device_id: _,
        state: ElementState::Released,
        button: winit::event::MouseButton::Right,
      } => self.input.reset_mouse_button(MouseButton::Right),
      _ => (),
    }
  }
}
