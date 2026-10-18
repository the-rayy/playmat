use std::sync::mpsc::{self, TryRecvError};

use crate::input::Input;

pub trait Game {
  fn init(&mut self, ctx: &mut Context);
  fn update(&mut self, ctx: &mut Context);
}

pub struct Context {
  pub gui: crate::gui::Context,
  pub assets: crate::assets::Context,

  rx: mpsc::Receiver<crate::event::Event>,
}

impl Context {
  pub fn new() -> Self {
    let (tx, rx) = mpsc::channel::<crate::event::Event>();
    Self {
      gui: crate::gui::Context::new(tx),
      assets: crate::assets::Context::default(),
      rx,
    }
  }

  pub fn handle_input(&mut self, input: &Input) {
    self.gui.handle_input(input);
  }

  pub fn events(&self) -> Vec<crate::event::Event> {
    let mut items = Vec::new();
    loop {
      match self.rx.try_recv() {
        Ok(item) => items.push(item),
        Err(TryRecvError::Empty) => break,
        Err(TryRecvError::Disconnected) => break,
      }
    }
    items
  }
}

impl Default for Context {
  fn default() -> Self {
    Self::new()
  }
}
