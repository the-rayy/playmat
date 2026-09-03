use std::{collections::HashMap, io::Cursor};

use image::ImageReader;

use crate::{engine::rendering::texture::{Texture, TextureKey}, math};

mod fonts;

#[derive(Default)]
pub struct Context {
  textures: HashMap<TextureKey, Texture>,
  fonts: HashMap<TextureKey, HashMap<char, math::Rect>>
}

impl Context {
  pub fn load_texture(&mut self, key: TextureKey, data: &[u8]) -> Result<(), String> {
    let img = ImageReader::new(Cursor::new(data))
      .with_guessed_format()
      .map_err(|e| e.to_string())?
      .decode()
      .map_err(|e| e.to_string())?
      .into_rgba8();

    let (width, height) = img.dimensions();
    let pixels = img.into_raw();
    let tex = Texture::new(pixels, height, width);
    self.textures.insert(key, tex);
    Ok(())
  }

  pub fn load_font(&mut self, key: TextureKey, data: &[u8]) -> Result<(), String> {
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let font = fontdue::Font::from_bytes(data, fontdue::FontSettings::default()).unwrap();
    let (data, meta) = fonts::rasterize_atlas(charset, &font);
    let tex = Texture::new(data, meta.height as u32, meta.width as u32);
    self.textures.insert(key.clone(), tex);
    self.fonts.insert(key, meta.locations);
    Ok(())
  }

  pub const fn get(&self) -> &HashMap<TextureKey, Texture> {
    &self.textures
  }

  pub fn get_font_glyph_uv(&self, key: &TextureKey, c: char) -> &math::Rect {
    self.fonts.get(key).unwrap().get(&c).unwrap()
  }
}
