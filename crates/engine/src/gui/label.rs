use std::collections::HashMap;

use crate::{
  gui::{self, Quad},
  math::{self, Color, Rect},
  rendering::texture::TextureKey,
};

#[derive(Debug)]
pub struct Label {
  pub label: String,
  pub rect: Rect,
  pub color: Color,
  pub texture_key: TextureKey,
  pub layer: u8,

  quads: Vec<Quad>,
}

impl Label {
  pub const fn new(
    label: String,
    rect: Rect,
    color: Color,
    texture_key: TextureKey,
    layer: u8,
  ) -> Self {
    Self {
      label,
      rect,
      color,
      texture_key,
      layer,
      quads: vec![],
    }
  }

  //TODO this feels like it should be somewhere else
  pub fn build(&mut self, fonts: &HashMap<TextureKey, HashMap<char, math::Rect>>) {
    self.quads = self
      .label
      .chars()
      .enumerate()
      .map(|(i, c)| {
        Quad::new(
          Rect {
            x: (1.1 * self.rect.w).mul_add(i as f32, self.rect.x),
            y: self.rect.y,
            w: self.rect.w,
            h: self.rect.h,
          },
          *fonts
            .get(&self.texture_key)
            .expect("texture not loaded")
            .get(&c)
            .expect("char not in charset"),
          self.color,
          self.texture_key.clone(),
          self.layer,
        )
      })
      .collect();
  }
}

impl gui::Widget for Label {
  fn quads(&self) -> Vec<Quad> {
    self.quads.clone()
  }

  fn rect(&self) -> Rect {
    self.rect
  }
}
