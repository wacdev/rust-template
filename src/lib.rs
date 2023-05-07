use image::EncodableLayout;
use napi::bindgen_prelude::Buffer;
use thiserror::Error;
use tiny_skia::PremultipliedColorU8;
use usvg::TreeParsing;
use webp::Encoder;

#[macro_use]
extern crate napi_derive;

#[derive(Error, Debug)]
pub enum Error {
  #[error("tiny_skia::Pixmap::new return None")]
  PIXMAP,

  #[error("resvg::render return None")]
  RESVG,
}

#[napi]
fn svg_webp(svg: Buffer, quality: f64) -> anyhow::Result<Buffer> {
  let opt = usvg::Options::default();
  let rtree = usvg::Tree::from_data(svg.as_ref(), &opt)?;
  let pixmap_size = rtree.size.to_screen_size();
  let width = pixmap_size.width();
  let height = pixmap_size.height();
  if let Some(mut pixmap) = tiny_skia::Pixmap::new(width, height) {
    // 去除透明度（默认是黑底，255-颜色会改为用白底）
    for px in pixmap.pixels_mut() {
      *px = PremultipliedColorU8::from_rgba(255 - px.red(), 255 - px.green(), 255 - px.blue(), 255)
        .unwrap();
    }
    if resvg::render(
      &rtree,
      resvg::FitTo::Original,
      tiny_skia::Transform::default(),
      pixmap.as_mut(),
    )
    .is_some()
    {
      let img = pixmap.data();

      let encoder = Encoder::from_rgba(img, width, height);
      let encoded_webp = encoder.encode(quality as f32);
      let b = encoded_webp.as_bytes();
      return Ok(b.into());
    } else {
      return Err(Error::RESVG)?;
    }
  }
  Err(Error::PIXMAP)?
}
