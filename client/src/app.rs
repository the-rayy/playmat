use std::sync::Arc;

use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};

use crate::{
  engine::{self, rendering::Renderer},
  framework::{self, Context, Game},
};

pub struct App<T: Game> {
  //engine
  window: Option<Arc<Window>>,
  renderer: Option<Renderer>,

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
    self.framework_context = Some(framework_context);

    self.game.start(self.framework_context.as_mut().unwrap());
  }

  fn window_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    _window_id: winit::window::WindowId,
    event: winit::event::WindowEvent,
  ) {
    while let Ok(msg) = self.framework_context.as_mut().unwrap().rx.try_recv() {
      self.game.handle(msg);
    }
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
