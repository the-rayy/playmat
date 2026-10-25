mod button;
mod label;
mod quad;

use std::{any::Any, collections::HashMap, sync::mpsc};

pub use button::Button;
pub use label::Label;
pub use quad::Quad;

use crate::{
  input::{ButtonState, Input, MouseButton},
  math::Rect,
  rendering::canvas::draw_list::DrawList,
};

#[derive(Debug)]
pub enum Event {
  ButtonClicked { id: String },
}

#[derive(Debug)]
pub enum InternalEvent {
  Click,
  Hover,
  Clear,
}

pub trait Widget: Any {
  fn rect(&self) -> Rect;
  fn quads(&self) -> Vec<Quad>;
  fn on_event(&mut self, _event: &InternalEvent) {}
}

pub struct Context {
  screen_width: u32,
  screen_height: u32,
  widgets: HashMap<String, Box<dyn Widget>>,

  tx: mpsc::Sender<crate::event::Event>,
}

impl Context {
  pub fn new(tx: mpsc::Sender<crate::event::Event>) -> Self {
    Self {
      screen_width: 0,
      screen_height: 0,
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

  pub fn reset(&mut self) {
    self.widgets.clear();
  }

  pub fn get_mut<T: Widget>(&mut self, id: &String) -> &mut T {
    let widget: &mut dyn Widget = self
      .widgets
      .get_mut(id)
      .expect("trying to get nonexistent widget")
      .as_mut();
    let a: &mut dyn Any = widget;
    a.downcast_mut::<T>()
      .expect("trying to get wrong type of widget")
  }

  pub fn get_draw_list(&self) -> DrawList {
    let mut draw_list = DrawList::default();
    self.widgets.values().for_each(|w| {
      w.quads()
        .iter()
        .for_each(|q| draw_list.push_rect(&q.rect, &q.uv, q.color, q.texture_key.clone()))
    });
    draw_list
  }

  pub fn handle_input(&mut self, input: &Input) {
    if self.screen_width == 0 || self.screen_height == 0 {
      return;
    }

    self.widgets.iter_mut().for_each(|(id, b)| {
      let hover = b.rect().contains(
        &input
          .get_cursor_pos()
          .in_screen_space(self.screen_width, self.screen_height),
      );

      if hover && input.get_mouse_button(MouseButton::Left) == &ButtonState::Pressed {
        b.on_event(&InternalEvent::Click);

        //TODO this should probably not be here, but in button...
        let res = self.tx.send(crate::event::Event::Gui(Event::ButtonClicked {
          id: id.clone(),
        }));
        log::info!("{:?}", res); //FIXME without logging res, send gets optimized away by compiler
      //on wasm. weird
      } else if hover {
        b.on_event(&InternalEvent::Hover);
      } else {
        b.on_event(&InternalEvent::Clear);
      };
    });
  }
}
