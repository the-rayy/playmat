use winit::event_loop::EventLoop;

mod engine;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
  engine::logger::init();
  engine::runtime::init();

  let mut winit_app = engine::app::App::new();

  let event_loop = EventLoop::new().expect("unable to initialize winit EventLoop");
  event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
  event_loop
    .run_app(&mut winit_app)
    .expect("EventLoop run failed");

  log::info!("Hello world!");
}
