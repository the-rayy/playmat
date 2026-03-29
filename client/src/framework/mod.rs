pub mod network;
pub mod window_manager;

pub trait Game {
  fn start(&self, ctx: &mut Context);
}

pub struct Context {
  pub window_manager: window_manager::WindowManager,
  pub network: network::ServerConnection,
}

impl Context {
  pub fn new() -> Self {
    Self {
      window_manager: Default::default(),
      network: network::ServerConnection::new(),
    }
  }
}
