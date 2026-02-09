#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native::logger;
#[cfg(not(target_arch = "wasm32"))]
pub use native::runtime;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::logger;
#[cfg(target_arch = "wasm32")]
pub use wasm::runtime;

