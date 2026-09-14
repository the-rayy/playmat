use engine::event::Event;
use std::sync::Arc;
use winit::window::Window;

use crate::framework::Game;

pub struct Engine<T: Game> {
  game: T,
  context: crate::framework::Context,

  renderer: Option<engine::rendering::Renderer>,
  input: engine::input::Input,
  frame_no: u64,
}

impl<T: Game> Engine<T> {
  pub fn new(game: T) -> Self {
    engine::platform::runtime::init();
    engine::platform::logger::init();

    Self {
      game,
      context: crate::framework::Context::new(),
      renderer: None,
      frame_no: 0,
      input: engine::input::Input::default(),
    }
  }

  pub fn init_rendering(&mut self, window: Arc<Window>) {
    let renderer =
      engine::platform::runtime::get().block_on(engine::rendering::Renderer::new(window));
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
