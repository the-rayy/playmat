use std::{collections::HashMap, io::Cursor};

use image::ImageReader;

use crate::engine::rendering::texture::{Texture, TextureKey};

#[derive(Default)]
pub struct Context {
  textures: HashMap<TextureKey, Texture>,
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

  pub fn foo(&self) {
    let font = include_bytes!("Kenney Pixel.ttf") as &[u8];
    let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
    let (met, bmp) = font.rasterize('A', 200.0);
    image::save_buffer("foo.png", &bmp, met.width as u32, met.height as u32, image::ExtendedColorType::L8).unwrap();


  }

  pub const fn get(&self) -> &HashMap<TextureKey, Texture> {
    &self.textures
  }
}
