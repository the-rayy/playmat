use std::sync::{Arc, Mutex, atomic::AtomicBool};

use egui::{Context, Ui, Vec2};

use crate::{context, gui};

pub struct Window {
  ctx: Arc<Mutex<context::Context>>,
  connected: Arc<AtomicBool>,
}

impl Window {
  pub fn new(ctx: Arc<Mutex<context::Context>>) -> Window {
    Window {
      ctx,
      connected: Default::default(),
    }
  }
}

impl gui::Draw for Window {
  fn draw(&mut self, ctx: &Context) {
    egui::Window::new("Login")
      .default_open(true)
      .movable(false)
      .auto_sized()
      .title_bar(false)
      .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
      .show(ctx, |ui: &mut Ui| {
        ui.label(self.ctx.lock().unwrap().debug.clone());
        ui.spacing_mut().item_spacing = Vec2::new(5.0, 5.0);
        if self.connected.load(std::sync::atomic::Ordering::SeqCst) {
          ui.label("connected");
        } else {
          ui.label("not connected");
        }
        if ui.button("connect").clicked() {};
      });
  }
}
