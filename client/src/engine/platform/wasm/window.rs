use winit::{dpi::PhysicalSize, window::{Window, WindowAttributes}};

fn window_attributes() -> WindowAttributes {
  let mut window_attributes = Window::default_attributes();
  let size = winit::dpi::Size::Physical(PhysicalSize::new(1920, 1080));
  window_attributes = window_attributes.with_inner_size(size);

    use wasm_bindgen::JsCast;
    use winit::platform::web::WindowAttributesExtWebSys;

    const CANVAS_ID: &str = "canvas";

    let window = wgpu::web_sys::window().expect("Unable to get window");
    let document = window.document().expect("Unable to get document");
    let canvas = document
      .get_element_by_id(CANVAS_ID)
      .expect("Unable to get canvas");
    let html_canvas_element = canvas.unchecked_into();
    window_attributes = window_attributes.with_canvas(Some(html_canvas_element));

  window_attributes
}
