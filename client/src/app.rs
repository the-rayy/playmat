use std::{cell::RefCell, rc::Rc, sync::Arc};

use tokio::sync::Mutex;
use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};

use crate::{
  engine::{self, gui::Gui, rendering::Renderer},
  framework::{self, Context, Game},
};

pub struct App<T: Game> {
  //engine
  window: Option<Arc<Window>>,
  renderer: Option<Renderer>,
  gui: Option<Gui>,

  //framework
  framework_context: Option<framework::Context>,

  //game
  game: T,
}

impl<T: Game> App<T> {
  pub fn new(game: T) -> App<T> {
    engine::runtime::init();

    App {
      window: None,
      renderer: None,
      gui: None,

      framework_context: None,

      game,
    }
  }
}

impl<T: Game> ApplicationHandler for App<T> {
  fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
    let window = event_loop
      .create_window(engine::window::attributes())
      .expect("could not create window");
    let window = Arc::new(window);
    let renderer = engine::runtime::get().block_on(Renderer::new(window.clone()));
    let framework_context = Context::new();

    self.window = Some(window.clone());
    self.renderer = Some(renderer);
    self.gui = Some(Gui::new(window));
    self.framework_context = Some(framework_context);

    self.game.start(&mut self.framework_context.as_mut().unwrap());
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
          .update(self.framework_context.as_mut().unwrap().window_manager.get_current());
        self.renderer.as_mut().unwrap().render(renderable_gui);
        self.window.as_mut().unwrap().request_redraw();
      }
      _ => (),
    }
  }
}
