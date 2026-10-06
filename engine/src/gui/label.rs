use std::collections::HashMap;

use crate::gui::Quad;
use crate::rendering::texture::Id;

#[derive(Debug)]
pub struct Label {
  pub label: String,
  pub rect: math::Rect,
  pub color: math::Color,
  pub texture_key: Id,
}

impl Label {
  pub const fn new(s: String, rect: math::Rect, color: math::Color, texture_key: Id) -> Self {
    Self {
      label: s,
      rect,
      color,
      texture_key,
    }
  }

  pub fn quads(&self, fonts: &HashMap<Id, HashMap<char, math::Rect>>) -> Vec<Quad> {
    self
      .label
      .chars()
      .enumerate()
      .map(|(i, c)| {
        Quad::new(
          math::Rect {
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
          self.texture_key,
        )
      })
      .collect()
  }
}
