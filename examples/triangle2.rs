extern crate polydraw;

use std::cmp::{min, max};
use std::i64;

use polydraw::{Application, Renderer, Frame};
use polydraw::draw::RGB;
use polydraw::geom::point::Point;
use polydraw::geom::triangle::Triangle;
use polydraw::geom::ring::Ring;
use polydraw::geom::clip2::{h_split, v_split, Edge, InclinedEdge};


const DIV_PER_PIXEL: i64 = 1000;
const DOUBLE_PIXEL_AREA: i64 = DIV_PER_PIXEL * DIV_PER_PIXEL * 2;

#[inline]
fn to_px(v: i64) -> i64 {
   v / DIV_PER_PIXEL
}

#[inline]
fn from_px(v: i64) -> i64 {
   v as i64 * DIV_PER_PIXEL
}

struct TriangleRenderer {
   tr: Triangle,
   colors: Vec<RGB>,
   up: Ring<Edge>,
   right: Ring<Edge>,
   left: Ring<Edge>,
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


   fn solid_follow(&mut self, x_split: i64, y_world: i64, y_split: i64) -> Option<i64> {
      let start = self.right.start();
      let vertical_index = self.right.prev_index(start);

      match self.right[vertical_index] {
         Edge::Vertical(vertical) => {
            if vertical.x != x_split || vertical.y1 != y_split || vertical.y2 != y_world {
               return None;
            }

            let top_index = self.right.next_index(vertical_index);

            match self.right[top_index] {
               Edge::Horizontal(top) => {
                  let bottom_index = self.right.prev_index(vertical_index);

                  match self.right[bottom_index] {
                     Edge::Horizontal(bottom) => {
                        let delta_min = min(top.x2, bottom.x1) - x_split;

                        if delta_min >= DIV_PER_PIXEL {
                           return Some(to_px(delta_min));
                        }
                     },
                     _ => {}
                  }
               },
               _ => {}
            }

         },
         _ => {}
      }

      None
   }

}

impl Renderer for TriangleRenderer {
   fn render(&mut self, frame: &mut Frame) {
      frame.clear();

      self.tr.a.x += DIV_PER_PIXEL;
      if self.tr.a.x >= from_px(frame.width as i64) {
         self.tr.a.x = 0;
      }

      if self.tr.clockwise() {
         self.tr.change_orientation();
      }

      let a = self.tr.a;
      let b = self.tr.b;
      let c = self.tr.c;

      let min_y = max(to_px(min3(a.y, b.y, c.y)), 0);
      let max_y = min(to_px(max3(a.y, b.y, c.y)), frame.height as i64 - 1);

      self.up.push(Edge::Inclined(
         InclinedEdge::new(a.x, a.y, b.x, b.y)
      ));
      self.up.push(Edge::Inclined(
         InclinedEdge::new(b.x, b.y, c.x, c.y)
      ));
      self.up.push(Edge::Inclined(
         InclinedEdge::new(c.x, c.y, a.x, a.y)
      ));

      shift_to_min_y1(&mut self.up);

      for y in min_y..max_y+1 {
         let y_world = from_px(y);
         let y_split = y_world + DIV_PER_PIXEL;

         h_split(y_split, &mut self.right, &mut self.up);

         shift_to_min_x1(&mut self.right);

         let (min_x, max_x) = min_max_x(&self.right);

         let mut x = min_x;

         while x < max_x {
            let x_world = from_px(x);
            let x_split = x_world + DIV_PER_PIXEL;

            v_split(x_split, &mut self.left, &mut self.right);

            plot_poly_pixel(frame, x, y, &self.left, &self.colors);

            self.left.consume();

            x += 1;

            match self.solid_follow(x_split, y_world, y_split) {
               Some(delta) => {
                  let x_split_right = x_split + from_px(delta);

                  v_split(x_split_right, &mut self.left, &mut self.right);

                  for solid_x in x..x + delta {
                     frame.put_pixel(solid_x as i32, y as i32, &self.colors[255]);
                  }

                  self.left.consume();

                  x += delta;
               },
               None => {}
            }

         }

         plot_poly_pixel(frame, max_x, y, &self.right, &self.colors);

         self.right.consume();
      }
   }
}

#[inline]
fn plot_poly_pixel(frame: &mut Frame, x: i64, y: i64, poly: &Ring<Edge>, colors: &Vec<RGB>) {
   let area = double_area(poly);

   assert!(area >= 0);

   frame.put_pixel(x as i32, y as i32, &colors[(255 * area / DOUBLE_PIXEL_AREA) as usize]);
}

#[inline]
fn min_max_x(edges: &Ring<Edge>) -> (i64, i64) {
   let mut min_x = i64::MAX;
   let mut max_x = i64::MIN;
   for e in edges[..].iter() {
      let x = e.x1();

      if x > max_x {
         max_x = x;
      }

      if x < min_x {
         min_x = x;
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
pub fn double_area(poly: &Ring<Edge>) -> i64 {
   if poly.len() <= 2 {
//      panic!("poly.len() <= 2");
      return 0;
   }

   let mut area = 0;

   for e in poly[..].iter() {
      area += e.x1() * e.y2() - e.y1() * e.x2();
   }

   area
}

#[allow(dead_code)]
#[inline]
fn print_edges(name: &str, edges: &Ring<Edge>) {
   print!("{}: ", name);

   println!("s {} e {} :", edges.start(), edges.end());

   for edge in edges[..].iter() {
      println!("{:?}", edge);
   }
}

#[inline]
pub fn shift_to_min_x1(edges: &mut Ring<Edge>) {
   let len = edges.len();

   edges.rewind(len);

   let start = edges.start();
   let end = edges.end();

   let mut min_x = i64::MAX;
   let mut min_i = start;

   for i in start..end {
      let x1 = edges[i].x1();

      if min_x > x1 {
         min_x = x1;
         min_i = i;
      }
   }

   if min_i == start {
      return;
   }

   for i in min_i..end {
      let edge = edges[i];
      edges.push(edge);
   }

   for i in start..min_i {
      let edge = edges[i];
      edges.push(edge);
   }

   edges.consume_at(end);
}

#[inline]
pub fn shift_to_min_y1(edges: &mut Ring<Edge>) {
   let len = edges.len();

   edges.rewind(len);

   let start = edges.start();
   let end = edges.end();

   let mut min_y = i64::MAX;
   let mut min_i = start;

   for i in start..end {
      let y1 = edges[i].y1();

      if min_y > y1 {
         min_y = y1;
         min_i = i;
      }
   }

   if min_i == start {
      return;
   }

   for i in min_i..end {
      let edge = edges[i];
      edges.push(edge);
   }

   for i in start..min_i {
      let edge = edges[i];
      edges.push(edge);
   }

   edges.consume_at(end);
}

fn main() {
   let mut renderer = TriangleRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Triangle2")
      .run();
}
