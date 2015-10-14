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
use polydraw::geom::clip::{h_split_edge, v_split_edge, hv_split, Ring};


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
   colors: Vec<RGB>,
   up: Ring<Point>,
   right: Ring<Point>,
   left: Ring<Point>,
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
         up: Ring::new(131072),
         right: Ring::new(524288),
         left: Ring::new(524288),
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

      self.up.push(a); self.up.push(b); self.up.push(c);

      for y in min_y..max_y+1 {
         let y_split = from_px(y) + HALF_DIV_PER_PIXEL;

         hv_split(h_split_edge, y_split, &mut self.right, &mut self.up);

         let (min_x, max_x) = min_max_x(&self.right);

         for x in min_x..max_x {
            let x_split = from_px(x) + HALF_DIV_PER_PIXEL;

            hv_split(v_split_edge, x_split, &mut self.left, &mut self.right);

            plot_poly_pixel(frame, x, y, &self.left, &self.colors);

            self.left.consume();
         }

         plot_poly_pixel(frame, max_x, y, &self.right, &self.colors);

         self.right.consume();
      }
   }
}

#[inline]
fn print_points(name: &str, points: &Ring<Point>) {
   print!("{}: ", name);

   print!("s {} e {} ", points.start(), points.end());

   for p in points[..].iter() {
      print!("({:?}, {:?}) ", p.x, p.y);
   }

   println!("");
}

#[inline]
fn plot_poly_pixel(frame: &mut Frame, x: i64, y: i64, points: &Ring<Point>, colors: &Vec<RGB>) {
   let area = double_polygon_area(points);

   frame.put_pixel(x as i32, y as i32, &colors[(255 * area / DOUBLE_PIXEL_AREA) as usize]);
}

#[inline]
fn min_max_x(points: &Ring<Point>) -> (i64, i64) {
   let mut min_x = i64::MAX;
   let mut max_x = i64::MIN;
   for p in points[..].iter() {
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

#[inline]
pub fn double_polygon_area(points: &Ring<Point>) -> i64 {
   let mut p1 = points.last().unwrap();

   let mut area = 0;

   for p2 in points[..].iter() {
      area += p1.x * p2.y - p1.y * p2.x;

      p1 = p2;
   }

   area.abs()
}

fn main() {
   let mut renderer = TriangleRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Triangle")
      .run();
}
