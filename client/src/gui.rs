use std::sync::Arc;

use egui::{ClippedPrimitive, Context, TexturesDelta};
use winit::{event::WindowEvent, window::Window};

pub trait Draw: Send + Sync {
  fn draw(&mut self, ctx: &Context);
}

pub struct Gui {
  context: egui::Context,
  state: egui_winit::State,
  window: Arc<Window>,
}

impl Gui {
  pub fn new(window: Arc<Window>) -> Self {
    let context = egui::Context::default();
    let state = egui_winit::State::new(
      context.clone(),
      context.clone().viewport_id(),
      window.clone().as_ref(),
      None,
      None,
      None,
    );

    Self {
      context,
      state,
      window,
    }
  }

  pub fn handle_event(&mut self, event: &WindowEvent) {
    let _ = self.state.on_window_event(self.window.as_ref(), event);
  }

  pub fn update(
    &mut self,
    windows: &mut Vec<Box<dyn Draw>>,
  ) -> (Vec<ClippedPrimitive>, TexturesDelta) {
    let input = self.state.take_egui_input(self.window.as_ref());
    let output = self.context.run(input, |ui| {
      windows.iter_mut().for_each(|w| w.draw(ui));
    });
    self
      .state
      .handle_platform_output(self.window.as_ref(), output.platform_output);

    let primitives = self
      .context
      .tessellate(output.shapes, output.pixels_per_point);
    let textures = output.textures_delta;

    (primitives, textures)
  }
}
