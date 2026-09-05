use std::collections::HashMap;

use crate::math;

pub struct Meta {
  pub locations: HashMap<char, math::Rect>,
  pub width: usize,
  pub height: usize,
}

pub fn rasterize_atlas(charset: &str, font: &fontdue::Font) -> (Vec<u8>, Meta) {
  let rasterized = charset
    .chars()
    .map(|c| {
      let (met, glyph) = font.rasterize(c, 32.0);
      (c, met, glyph)
    })
    .collect::<Vec<(char, fontdue::Metrics, Vec<u8>)>>();

  let total_width = rasterized
    .iter()
    .map(|(_, met, _)| met.width)
    .sum::<usize>();
  let height = rasterized
    .iter()
    .map(|(_, met, _)| met.height)
    .max()
    .unwrap();

  let mut bmp = vec![0; total_width * height];
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

    meta.insert(
      *c,
      math::Rect::new(
        x_offset as f32 / total_width as f32,
        0.0,
        w as f32 / total_width as f32,
        h as f32 / height as f32,
      ),
    );
    x_offset += w;
  }

  let bmp = bmp
    .into_iter()
    .flat_map(|x| [255, 255, 255, x])
    .collect();

  let meta = Meta {
    locations: meta,
    width: total_width,
    height,
  };

  (bmp, meta)
}
