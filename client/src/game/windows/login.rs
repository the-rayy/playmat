use crate::engine::gui;

pub struct Window {
  username: String,
}

impl Window {
  pub fn new() -> Window {
    Window {
      username: Default::default(),
    }
  }
}

impl gui::Draw for Window {
  fn draw(&mut self, ctx: &egui::Context) {
    egui::Window::new("Login")
      .default_open(true)
      .movable(false)
      .auto_sized()
      .title_bar(false)
      .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
      .show(ctx, |ui: &mut egui::Ui| {
        ui.label("username");
        ui.text_edit_singleline(&mut self.username);

        if ui.button("login").clicked() {};
      });
  }
}
