#![feature(test)]

#![allow(dead_code)]

extern crate test;

extern crate polydraw;

use std::cmp::{min, max};
use std::i64;

use polydraw::{Application, Renderer, Frame};
use polydraw::draw::RGB;
use polydraw::geom::point::Point;
use polydraw::geom::triangle::Triangle;
use polydraw::geom::number::NumberOps;
use polydraw::geom::polygon::double_polygon_area;
use polydraw::geom::clip::{h_split, v_split};


const DIV_PER_PIXEL: i64 = 1000;
const HALF_DIV_PER_PIXEL: i64 = DIV_PER_PIXEL / 2;
const DOUBLE_PIXEL_AREA: i64 = DIV_PER_PIXEL * DIV_PER_PIXEL * 2;

#[inline]
fn to_px(v: i64) -> i64 {
   v.rounding_div(DIV_PER_PIXEL)
}

#[inline]
fn from_px(v: i64) -> i64 {
   v as i64 * DIV_PER_PIXEL
}

struct TriangleRenderer {
   tr: Triangle,
   colors: Vec<RGB>
}

impl TriangleRenderer {
   fn new() -> Self {
      let tr = Triangle::new(
         Point::new(from_px(100), from_px(450)),
         Point::new(from_px(600), from_px(600)),
         Point::new(from_px(350), from_px(100)),
      );

      let mut colors = Vec::with_capacity(256);
      for i in 0..256_usize {
         colors.push(
            RGB::new(
               (i * 127 / 255) as u8, (i * 223 / 255) as u8, i as u8
            )
         );
      }

      TriangleRenderer {
         tr: tr,
         colors: colors,
      }
   }
}

impl Renderer for TriangleRenderer {
   fn render(&mut self, frame: &mut Frame) {
      frame.clear();

      self.tr.a.x += DIV_PER_PIXEL;
      if self.tr.a.x >= from_px(frame.width as i64) {
         self.tr.a.x = 0;
      }

      let a = self.tr.a;
      let b = self.tr.b;
      let c = self.tr.c;

      let min_y = max(to_px(min3(a.y, b.y, c.y)), 0);
      let max_y = min(to_px(max3(a.y, b.y, c.y)), frame.height as i64 - 1);

      let mut up = Vec::with_capacity(100);
      up.push(a); up.push(b); up.push(c);

      let mut left = Vec::with_capacity(100);
      let mut right = Vec::with_capacity(100);

      let mut up_start = 0;
      let mut right_start;

      for y in min_y..max_y+1 {
         let y_split = from_px(y) + HALF_DIV_PER_PIXEL;

         right_start = right.len();
         let up_end = up.len();

         h_split(y_split, up_start, &mut right, &mut up);

         up_start = up_end;

         let (min_x, max_x) = min_max_x(0, &right);

         for x in min_x..max_x {
            let left_start = left.len();
            let right_end = right.len();

            let x_split = from_px(x) + HALF_DIV_PER_PIXEL;

            v_split(x_split, right_start, &mut left, &mut right);

            right_start = right_end;

            plot_poly_pixel(frame, x, y, left_start, &left, &self.colors);
         }

         plot_poly_pixel(frame, max_x, y, right_start, &right, &self.colors);
      }
   }
}

#[inline]
fn print_points(name: &str, start: usize, points: &Vec<Point>) {
   print!("{}: ", name);

   for p in points[start..].iter() {
      print!("({:?}, {:?}) ", p.x, p.y);
   }

   println!("");
}

#[inline]
fn plot_poly_pixel(frame: &mut Frame, x: i64, y: i64, start: usize, points: &Vec<Point>, colors: &Vec<RGB>) {
   let area = double_polygon_area(start, &points);

   assert!(area <= DOUBLE_PIXEL_AREA);

   frame.put_pixel(x as i32, y as i32, &colors[(255 * area / DOUBLE_PIXEL_AREA) as usize]);
}

#[inline]
fn min_max_x(start: usize, points: &Vec<Point>) -> (i64, i64) {
   let mut min_x = i64::MAX;
   let mut max_x = i64::MIN;
   for p in points[start..].iter() {
      if p.x > max_x {
         max_x = p.x;
      }

      if p.x < min_x {
         min_x = p.x;
      }
   }

   (to_px(min_x), to_px(max_x))
}

#[inline]
pub fn min3<T: Ord>(v1: T, v2: T, v3: T) -> T {
   min(min(v1, v2), v3)
}

#[inline]
pub fn max3<T: Ord>(v1: T, v2: T, v3: T) -> T {
   max(max(v1, v2), v3)
}


fn main() {
   let mut renderer = TriangleRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Triangle")
      .run();
}
