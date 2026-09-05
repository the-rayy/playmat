use std::collections::HashMap;

use crate::{
  engine::rendering::texture::TextureKey,
  framework::gui::Quad,
  math::{self, Color, Rect},
};

#[derive(Debug)]
pub struct Label {
  pub label: String,
  pub rect: Rect,
  pub color: Color,
  pub texture_key: TextureKey,
}

impl Label {
  pub const fn new(s: String, rect: Rect, color: Color, texture_key: TextureKey) -> Self {
    Self {
      label: s,
      rect,
      color,
      texture_key,
    }
  }

  pub fn quads(&self, fonts: &HashMap<TextureKey, HashMap<char, math::Rect>>) -> Vec<Quad> {
    self
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
            .unwrap()
            .get(&c)
            .unwrap(),
          self.color,
          self.texture_key.clone(),
        )
      })
      .collect()
  }
}
