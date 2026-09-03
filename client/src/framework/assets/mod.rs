use std::{collections::HashMap, io::Cursor};

use image::ImageReader;

use crate::{engine::rendering::texture::{Texture, TextureKey}, math};

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
      .map(|c| {
        let (met, glyph) = font.rasterize(c, 200.0);
        (c, met, glyph)
      })
      .collect::<Vec<(char, fontdue::Metrics, Vec<u8>)>>();

    let total_width = rasterized.iter().map(|(_, met, _)| met.width).sum::<usize>();
    let height = rasterized.iter().map(|(_, met, _)| met.height).max().unwrap();

    let mut bmp = Vec::new();
    bmp.resize(total_width * height, 0_u8);
    let mut meta = HashMap::<char, math::Rect>::new();

    let mut x_offset = 0;

    for (c, met, glyph) in &rasterized {
      let w = met.width;
      let h = met.height;

      for y in 0..h {
        for x in 0..w {
          let src = x + y * w;
          let dst = x_offset + x + y * total_width;

          bmp[dst] = glyph[src];
        }
      }

      meta.insert(*c, math::Rect::new(x_offset as f32, 0.0, w as f32, h as f32));
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
