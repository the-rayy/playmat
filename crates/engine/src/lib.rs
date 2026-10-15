use winit::event_loop::EventLoop;

use crate::framework::Game;

pub mod app; //TODO change to pub(crate)
pub mod assets; //TODO change to pub(crate)
pub mod engine; //TODO change to pub(crate)
pub mod event; //TODO change to pub(crate)
pub mod framework;
pub mod gui; //TODO change to pub(crate)
pub mod input; //TODO change to pub(crate)
pub mod math;
pub mod platform; //TODO change to pub(crate)
pub mod rendering; //TODO change to pub(crate)

pub fn run<T: Game>(game: T) {
  let engine = engine::Engine::new(game);
  let mut winit_app = app::App::new(engine);

  let event_loop = EventLoop::new().expect("unable to initialize winit EventLoop");
  event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
  event_loop
    .run_app(&mut winit_app)
    .expect("EventLoop run failed");
}
