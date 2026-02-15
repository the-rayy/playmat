use std::sync::{Arc, Mutex};

use winit::event_loop::EventLoop;

use crate::context::Context;

mod app;
mod debug_window;
mod gui;
mod net;
mod platform;
mod renderer;
mod context;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
  let ctx = Arc::new(Mutex::new(Context::default()));
  platform::logger::init();
  platform::runtime::init();
  net::init(ctx.clone());

  let mut winit_app = app::App::new(ctx.clone());

  let event_loop = EventLoop::new().expect("unable to initialize winit EventLoop");
  event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
  event_loop
    .run_app(&mut winit_app)
    .expect("EventLoop run failed");

  log::info!("Hello world!");
}
