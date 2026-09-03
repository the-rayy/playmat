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
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let font = include_bytes!("Kenney Pixel.ttf") as &[u8];
    let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();

    let rasterized = charset
      .chars()
      .map(|c| font.rasterize(c, 200.0))
      .collect::<Vec<(fontdue::Metrics, Vec<u8>)>>();

    let total_width = rasterized.iter().map(|(met, _)| met.width).sum::<usize>();
    let height = rasterized.iter().map(|(met, _)| met.height).max().unwrap();

    let mut bmp = Vec::new();
    bmp.resize(total_width * height, 0_u8);

    let mut x_offset = 0;

    for (met, glyph) in &rasterized {
      let w = met.width;
      let h = met.height;

      for y in 0..h {
        for x in 0..w {
          let src = x + y * w;
          let dst = x_offset + x + y * total_width;

          bmp[dst] = glyph[src];
        }
      }

      x_offset += w;
    }

    image::save_buffer(
      "foo.png",
      &bmp,
      total_width as u32,
      height as u32,
      image::ExtendedColorType::L8,
    )
    .unwrap();
  }

  pub const fn get(&self) -> &HashMap<TextureKey, Texture> {
    &self.textures
  }
}
