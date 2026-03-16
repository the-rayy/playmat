use std::{collections::HashMap, sync::{Arc, Mutex}};

use egui::{ClippedPrimitive, Context, TexturesDelta};
use shared::message::client::ClientMessage;
use tokio::sync::mpsc;
use winit::{event::WindowEvent, window::Window};

use crate::context::{self, Scene};

pub mod auth;
pub mod diagnostics;

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

pub struct WindowManager {
  w: HashMap<Scene, Vec<Box<dyn Draw>>>,
}

impl WindowManager {
  fn new(ctx: Arc<Mutex<context::Context>>, net_tx: mpsc::Sender<ClientMessage>) -> Self {
    let mut windows = Vec::<Box<dyn Draw>>::new();
    let diag_window = diagnostics::Window::new(ctx.clone());
    let auth_window = auth::Window::new(ctx.clone(), net_tx);

    windows.push(Box::new(diag_window));
    windows.push(Box::new(auth_window));

    let mut scenes = HashMap::new();
    scenes.insert(Scene::Login, windows);

    Self { w: scenes }
  }
}
