use std::collections::HashMap;

use crate::rendering::texture::{Id, Texture};

mod fonts;

#[derive(Default)]
pub struct Context {
  textures: Vec<Texture>,
  fonts: HashMap<Id, HashMap<char, math::Rect>>,
}

impl Context {
  pub fn load_texture(&mut self, data: &[u8]) -> Result<Id, String> {
    let tex = Texture::from_bytes(data)?;
    let id = *tex.id();
    self.textures.push(tex);
    Ok(id)
  }

  pub fn load_font(&mut self, data: &[u8]) -> Result<Id, String> {
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz _!12";
    let font = fontdue::Font::from_bytes(data, fontdue::FontSettings::default())
      .map_err(|e| e.to_string())?;
    let (data, meta) = fonts::rasterize_atlas(charset, &font);
    let tex = Texture::new(data, meta.height as u32, meta.width as u32);
    let id = *tex.id();
    self.textures.push(tex);
    self.fonts.insert(id, meta.locations);
    Ok(id)
  }

  pub const fn get(&self) -> &Vec<Texture> {
    &self.textures
  }

  pub const fn get_fonts(&self) -> &HashMap<Id, HashMap<char, math::Rect>> {
    &self.fonts
  }
}
