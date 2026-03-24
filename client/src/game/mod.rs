use crate::framework;

mod windows;

pub struct GameImpl {}

impl framework::Game for GameImpl {
    fn start(&self, wm: &mut framework::window_manager::WindowManager) {
        let auth_window = windows::login::Window::new();
        wm.add(auth_window);
    }
}
