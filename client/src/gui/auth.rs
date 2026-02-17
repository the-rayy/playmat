use std::sync::{Arc, Mutex};

use egui::{Context, Ui, Vec2};
use tokio::sync::mpsc;

use crate::{context, gui};

pub struct Window {
  ctx: Arc<Mutex<context::Context>>,

  email: String,
  password: String,
  net_tx: mpsc::Sender<i64>,
}

impl Window {
  pub fn new(ctx: Arc<Mutex<context::Context>>, net_tx: mpsc::Sender<i64>) -> Window {
    Window {
      ctx,

      email: Default::default(),
      password: Default::default(),
      net_tx,
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
        ui.label("email");
        ui.text_edit_singleline(&mut self.email);

        ui.label("password");
        ui.text_edit_singleline(&mut self.password);

        if ui.button("login").clicked() {
          let _ = self.net_tx.blocking_send(1);
        };
      });
  }
}
