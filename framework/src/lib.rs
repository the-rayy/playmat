use std::sync::mpsc::{self, TryRecvError};

use ::engine::input::Input;

pub mod assets;
pub mod engine;
pub mod gui;

pub trait Game {
  fn update(&mut self, ctx: &mut Context);
}

#[derive(Debug)]
pub enum Event {
  Gui(gui::Event),
}

pub struct Context {
  pub gui: gui::Context,
  pub assets: assets::Context,

  rx: mpsc::Receiver<Event>,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
  pub fn new() -> Self {
    let (tx, rx) = mpsc::channel::<Event>();
    Self {
      gui: gui::Context::new(tx),
      assets: assets::Context::default(),
      rx,
    }
  }

  pub fn handle_input(&mut self, input: &Input) {
    self.gui.handle_input(input);
  }

  pub fn events(&self) -> Vec<Event> {
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
