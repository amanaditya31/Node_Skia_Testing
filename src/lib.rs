#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;


use std::mem;

use skia_safe::{surfaces, Color, Data, EncodedImageFormat, Paint, PaintStyle, Path, Surface};

#[napi]
pub struct SkiaCanvas {
    surface: Surface,
    path: Path,
    paint: Paint,
}

#[napi]
impl SkiaCanvas {
    #[napi(constructor)]
    pub fn new(width: i32, height: i32) -> SkiaCanvas {
        let mut surface = surfaces::raster_n32_premul((width, height)).expect("surface");
        let path = Path::new();
        let mut paint = Paint::default();
        paint.set_color(Color::BLACK);
        paint.set_anti_alias(true);
        paint.set_stroke_width(1.0);
        surface.canvas().clear(Color::WHITE);
        SkiaCanvas {
            surface,
            path,
            paint,
        }
    }
    #[napi]
    #[inline]
    pub fn save(&mut self) {
        self.canvas().save();
    }

    #[napi]
    #[inline]
    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.canvas().translate((dx as f32, dy as f32));
    }
    #[napi]
    #[inline]
    pub fn scale(&mut self, sx: f64, sy: f64) {
        self.canvas().scale((sx as f32, sy as f32));
    }
    #[napi]
    #[inline]
    pub fn move_to(&mut self, x: f64, y: f64) {
        self.begin_path();
        self.path.move_to((x as f32, y as f32));
    }
    #[napi]
    #[inline]
    pub fn line_to(&mut self, x: f64, y: f64) {
        self.path.line_to((x  as f32, y as f32));
    }
    #[napi]
    #[inline]
    pub fn quad_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        self.path.quad_to((cpx  as f32 , cpy as f32), (x as f32, y as f32));
    }

    #[allow(dead_code)]
    #[napi]
    #[inline]
    pub fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.path.cubic_to((cp1x as f32, cp1y as f32), (cp2x as f32, cp2y as f32), (x as f32, y as f32));
    }

    #[allow(dead_code)]
    #[napi]
    #[inline]
    pub fn close_path(&mut self) {
        self.path.close();
    }
    #[napi]
    #[inline]
    pub fn begin_path(&mut self) {
        let new_path = Path::new();
        self.surface.canvas().draw_path(&self.path, &self.paint);
        let _ = mem::replace(&mut self.path, new_path);
    }
    #[napi]
    #[inline]
    pub fn stroke(&mut self) {
        self.paint.set_style(PaintStyle::Stroke);
        self.surface.canvas().draw_path(&self.path, &self.paint);
    }
    #[napi]
    #[inline]
    pub fn fill(&mut self) {
        self.paint.set_style(PaintStyle::Fill);
        self.surface.canvas().draw_path(&self.path, &self.paint);
    }
    #[napi]
    #[inline]
    pub fn set_line_width(&mut self, width: f64) {
        self.paint.set_stroke_width(width as f32);
    }
    #[napi]
    #[inline]
    pub fn save_to(&mut self, file:String){
      
      let image = self.surface.image_snapshot();
      let mut context = self.surface.direct_context();
      let encoded = image
          .encode(context.as_mut(), EncodedImageFormat::PNG, None)
          .unwrap();
      std::fs::write(file, encoded.as_bytes()).expect("Could not save");
    }

    #[inline]
     pub fn canvas(&mut self) -> &skia_safe::Canvas {
        self.surface.canvas()
    }
}


#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}
