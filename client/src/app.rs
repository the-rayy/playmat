use std::sync::Arc;

use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};

use crate::{
  engine::{self, gui::Gui, rendering::Renderer},
  framework::window_manager::WindowManager,
  game::Game,
};

pub struct App<T: Game> {
  //engine
  window: Option<Arc<Window>>,
  renderer: Option<Renderer>,
  gui: Option<Gui>,

  //framework
  window_manager: WindowManager,

  //game
  game: T,
}

impl<T: Game> App<T> {
  pub fn new(game: T) -> App<T> {
    App {
      window: None,
      renderer: None,
      gui: None,

      window_manager: WindowManager::default(),

      game,
    }
  }
}

impl<T: Game> ApplicationHandler for App<T> {
  fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
    engine::runtime::init();
    let window = event_loop
      .create_window(engine::window::attributes())
      .expect("could not create window");
    let window = Arc::new(window);
    let renderer = engine::runtime::get().block_on(Renderer::new(window.clone()));

    self.window = Some(window.clone());
    self.renderer = Some(renderer);
    self.gui = Some(Gui::new(window));
  }

  fn window_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    _window_id: winit::window::WindowId,
    event: winit::event::WindowEvent,
  ) {
    self.gui.as_mut().unwrap().handle_event(&event);
    match event {
      WindowEvent::CloseRequested => event_loop.exit(),
      WindowEvent::Resized(size) => self.renderer.as_mut().unwrap().resize(size),
      WindowEvent::RedrawRequested => {
        let renderable_gui = self
          .gui
          .as_mut()
          .unwrap()
          .update(self.window_manager.get_current());
        self.renderer.as_mut().unwrap().render(renderable_gui);
        self.window.as_mut().unwrap().request_redraw();
      }
      _ => (),
    }
  }
}
