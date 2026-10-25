mod button;
mod label;
mod quad;

use std::{collections::HashMap, sync::mpsc};

pub use button::Button;
pub use label::Label;
pub use quad::Quad;

use crate::{
  input::{ButtonState, Input, MouseButton},
  math::{Color, Rect},
  rendering::{canvas::draw_list::DrawList, texture::TextureKey},
};

#[derive(Debug)]
pub enum Event {
  ButtonClicked { id: String },
}

pub trait Widget {
  fn quads(&self) -> Vec<Quad>;
}

pub struct Context {
  screen_width: u32,
  screen_height: u32,
  buttons: HashMap<String, Button>,

  widgets: HashMap<String, Box<dyn Widget>>,

  tx: mpsc::Sender<crate::event::Event>,
}

impl Context {
  pub fn new(tx: mpsc::Sender<crate::event::Event>) -> Self {
    Self {
      screen_width: 0,
      screen_height: 0,
      buttons: HashMap::default(),
      widgets: HashMap::default(),
      tx,
    }
  }

  pub const fn set_screen_dims(&mut self, w: u32, h: u32) {
    self.screen_width = w;
    self.screen_height = h;
  }

  pub fn add<T: Widget + 'static>(&mut self, id: String, widget: T) {
    let widget = Box::new(widget);
    self.widgets.insert(id, widget);
  }

  pub fn add_button(&mut self, id: String, button: Button) {
    self.buttons.insert(id, button);
  }

  pub fn reset(&mut self) {
    self.buttons.clear();
    self.widgets.clear();
  }

  pub fn get_mut_button(&mut self, id: &String) -> &mut Button {
    self
      .buttons
      .get_mut(id)
      .expect("trying to get nonexistent button")
  }

  pub fn get_draw_list(&self) -> DrawList {
    let mut draw_list = DrawList::default();
    self
      .buttons
      .values()
      .for_each(|b| draw_list.push_rect(&b.rect, &Rect::unit(), b.color(), b.texture_key()));
    self.widgets.values().for_each(|w| {
      w.quads()
        .iter()
        .for_each(|q| draw_list.push_rect(&q.rect, &q.uv, q.color.clone(), q.texture_key.clone()))
    });
    draw_list
  }

  pub fn handle_input(&mut self, input: &Input) {
    if self.screen_width == 0 || self.screen_height == 0 {
      return;
    }

    self.buttons.iter_mut().for_each(|(id, b)| {
      let hover = b.rect.contains(
        &input
          .get_cursor_pos()
          .in_screen_space(self.screen_width, self.screen_height),
      );

      b.state = if hover && input.get_mouse_button(MouseButton::Left) == &ButtonState::Pressed {
        let res = self.tx.send(crate::event::Event::Gui(Event::ButtonClicked {
          id: id.clone(),
        }));
        log::info!("{:?}", res); //FIXME without logging res, send gets optimized away by compiler
        //on wasm. weird

        button::State::Down
      } else if hover {
        button::State::Hovered
      } else {
        button::State::Neutral
      };
    });
  }
}
