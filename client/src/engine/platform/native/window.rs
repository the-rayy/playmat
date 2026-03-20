use winit::{dpi::PhysicalSize, window::{Window, WindowAttributes}};

pub fn attributes() -> WindowAttributes {
  let mut window_attributes = Window::default_attributes();
  let size = winit::dpi::Size::Physical(PhysicalSize::new(1920, 1080));
  window_attributes = window_attributes.with_inner_size(size);
  
  window_attributes
}
