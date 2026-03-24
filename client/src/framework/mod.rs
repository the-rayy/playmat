pub mod window_manager;

pub trait Game {
  fn start(&self, wm: &mut window_manager::WindowManager);
}
