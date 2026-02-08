mod platform;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
  platform::logger::init();

  log::info!("Hello world!");
}
