use winit::event_loop::EventLoop;

mod app;
mod engine;
mod game;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
  let game = game::Game::default();
  let engine = engine::Engine::new(game);
  let mut winit_app = app::App::new(engine);

  let event_loop = EventLoop::new().expect("unable to initialize winit EventLoop");
  event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
  event_loop
    .run_app(&mut winit_app)
    .expect("EventLoop run failed");
}
