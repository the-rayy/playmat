mod game;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
  let game = game::Game::new();
  playmat_engine::run(game);
}
