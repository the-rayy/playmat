pub mod event;
pub mod input;
pub mod platform;
pub mod rendering;
pub mod assets;
pub mod gui;

#[derive(Debug)]
pub enum EngineEvent {
  Gui(gui::Event),
}


