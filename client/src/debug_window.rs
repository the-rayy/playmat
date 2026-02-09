use std::sync::{Arc, atomic::AtomicBool};

use egui::{Context, Ui, Vec2};

use crate::gui;

#[derive(Default)]
pub struct Window {
  connected: Arc<AtomicBool>,
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
