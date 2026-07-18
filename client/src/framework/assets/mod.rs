use std::{
  collections::HashMap,
  io::Cursor,
};

use image::ImageReader;

use crate::engine::rendering::texture::{Texture, TextureKey};

#[derive(Default)]
pub struct Context {
  textures: HashMap<TextureKey, Texture>,
}

impl Context {
  pub fn load_texture_png(&mut self, key: TextureKey, png: &[u8]) {
    let img = ImageReader::new(Cursor::new(png))
      .with_guessed_format()
      .unwrap()
      .decode()
      .unwrap()
      .into_rgba8();

    let (width, height) = img.dimensions();
    let pixels = img.into_raw();
    let tex = Texture::new(pixels, height, width);
    self.textures.insert(key, tex);
  }

  pub fn load_needed(&self) -> bool {
    !self.textures.is_empty()
  }

  pub const fn get(&self) -> &HashMap<TextureKey, Texture> {
    &self.textures
  }
}
