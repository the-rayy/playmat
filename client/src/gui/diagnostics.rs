use std::sync::{Arc, Mutex};

use egui::{Context, Ui};

use crate::{context, gui};

pub struct Window {
  ctx: Arc<Mutex<context::Context>>,
}

impl Window {
  pub fn new(ctx: Arc<Mutex<context::Context>>) -> Window {
    Window {
      ctx,
    }
  }
}

impl gui::Draw for Window {
  fn draw(&mut self, ctx: &Context) {
    egui::Window::new("Diagnostics")
      .auto_sized()
      .show(ctx, |ui: &mut Ui| {
        match self.ctx.lock().unwrap().timestamp {
          Some(x) => ui.label(format!("Server time: {x}")),
          None => ui.label("Not connected to server"),
        }
      });
  }
}
