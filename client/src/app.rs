use std::sync::Arc;

use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};

use crate::{
  engine::{self, rendering::Renderer},
  framework::{self, Context, Game},
};

pub struct App<T: Game> {
  game: Option<T>, //store game only between new() and resumed() calls
  running_app: Option<RunningApp<T>>,
}

impl<T: Game> App<T> {
  pub fn new(game: T) -> Self {
    engine::runtime::init();

    Self {
      game: Some(game),
      running_app: None
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

    let mut running_app = RunningApp {
      window: window,
      renderer: renderer,
      framework_context: framework_context,
      game: self.game.take().expect("resume called twice"),
    };

    running_app.start();
    self.running_app = Some(running_app);
  }

  fn window_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    _window_id: winit::window::WindowId,
    event: winit::event::WindowEvent,
  ) {
    self.running_app.as_mut().expect("app not running").window_event(event_loop, _window_id, event);
  }
}

struct RunningApp<T: Game> {
  window: Arc<Window>,
  renderer: Renderer,
  framework_context: framework::Context,
  game: T,
}

impl<T: Game> RunningApp<T> {
  fn start(&mut self) {
    self.game.start(&mut self.framework_context);
  }

  fn window_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    _window_id: winit::window::WindowId,
    event: winit::event::WindowEvent,
  ) {
    while let Ok(msg) = self.framework_context.rx.try_recv() {
      self.game.handle(msg);
    }
    match event {
      WindowEvent::CloseRequested => event_loop.exit(),
      WindowEvent::Resized(size) => self.renderer.resize(size),
      WindowEvent::RedrawRequested => {
        self.renderer.render();
        self.window.request_redraw();
      }
      _ => (),
    }
  }

}
