use std::cmp::{Ordering, min};
use std::iter::repeat;
use std::i64;

use frame::Frame;
use draw::RGB;
use num::NumberOps;
use geom::point::Point;

use super::intersection::{IntersectionRef, RasterizerIntersection};
use super::pool::RasterizerPool;
use super::edge::{Edge, EdgeType};
use super::scene::Scene;

pub const DIV_PER_PIXEL: i64 = 1000;

const DOUBLE_PIXEL_AREA: i64 = DIV_PER_PIXEL * DIV_PER_PIXEL * 2;


macro_rules! debug_check {
   ($expr:expr) => (if cfg!(debug_assertions) { $expr; })
}

#[inline]
pub fn to_px(v: i64) -> i64 {
   v / DIV_PER_PIXEL
}

#[inline]
pub fn from_px(v: i64) -> i64 {
   v as i64 * DIV_PER_PIXEL
}

pub struct Rasterizer {
   pub vert_intersections_ref: Vec<IntersectionRef>,
   pub hori_intersections_ref: Vec<IntersectionRef>,
   pub vert_intersections: Vec<i64>,
   pub hori_intersections: Vec<i64>,

   pub polys_len: usize,
   pub poly_to_pool: Vec<usize>,

   pub upper_edges: Vec<Edge>,
   pub upper_edges_len: Vec<usize>,

   pub upper_min_y: Vec<i64>,
   pub upper_max_y: Vec<i64>,

   pub upper_active: Vec<usize>,
   pub upper_active_start: usize,
   pub upper_active_end: usize,

   pub lower_edges: Vec<Edge>,
   pub lower_edges_len: Vec<usize>,

   pub lower_min_x: Vec<i64>,
   pub lower_max_x: Vec<i64>,

   pub lower_active: Vec<usize>,
   pub lower_active_start: usize,
   pub lower_active_end: usize,
   pub lower_active_full: usize,

   pub final_edges: Vec<Edge>,
   pub final_edges_len: Vec<usize>,

   pub final_active: Vec<usize>,
   pub final_active_full: usize,
}

impl Rasterizer {
   pub fn new() -> Self {
      let vert_intersections_ref = create_default_vec(65536);
      let hori_intersections_ref = create_default_vec(65536);
      let vert_intersections = create_default_vec(65536);
      let hori_intersections = create_default_vec(65536);

      let poly_to_pool = create_default_vec(65536);

      let upper_edges = create_default_vec(65536);
      let upper_edges_len = create_default_vec(65536);

      let upper_min_y = create_default_vec(65536);
      let upper_max_y = create_default_vec(65536);
      let upper_active = create_default_vec(65536);

      let lower_edges = create_default_vec(65536);
      let lower_edges_len = create_default_vec(65536);

      let lower_min_x = create_default_vec(65536);
      let lower_max_x = create_default_vec(65536);
      let lower_active = create_default_vec(65536);

      let final_edges = create_default_vec(65536);
      let final_edges_len = create_default_vec(65536);

      let final_active = create_default_vec(65536);

      Rasterizer {
         vert_intersections_ref: vert_intersections_ref,
         hori_intersections_ref: hori_intersections_ref,
         vert_intersections: vert_intersections,
         hori_intersections: hori_intersections,

         polys_len: 0,
         poly_to_pool: poly_to_pool,

         upper_edges: upper_edges,
         upper_edges_len: upper_edges_len,

         upper_min_y: upper_min_y,
         upper_max_y: upper_max_y,

         upper_active: upper_active,
         upper_active_start: 0,
         upper_active_end: 0,

         lower_edges: lower_edges,
         lower_edges_len: lower_edges_len,

         lower_min_x: lower_min_x,
         lower_max_x: lower_max_x,

         lower_active: lower_active,
         lower_active_start: 0,
         lower_active_end: 0,
         lower_active_full: 0,

         final_edges: final_edges,
         final_edges_len: final_edges_len,

         final_active: final_active,
         final_active_full: 0,
      }
   }

   pub fn render(&mut self, scene: &Scene, frame: &mut Frame) {
      self.transfer_scene(scene);

      debug_check!(self.check_upper_initial_pool());

      self.intersect_edges(scene);

      debug_check!(self.check_intersections(scene));

      let (min_x, min_y, max_x, max_y) = self.min_max_x_y(scene);

      debug_check!(self.check_min_max_x_y(min_x, min_y, max_x, max_y));

      self.update_upper_min_max_y();

      debug_check!(self.check_upper_min_max_y(min_y, max_y));

      let x_start = to_px(min_x);
      let x_end = to_px(max_x - 1) + 1;
      let y_start = to_px(min_y);
      let y_end = to_px(max_y - 1) + 1;

      for y in y_start..y_end {
         let y_world = from_px(y);
         let y_slice = y_world + DIV_PER_PIXEL;

         self.lower_active_start = 0;
         self.lower_active_end = 0;
         self.lower_active_full = 0;

         self.advance_upper_range(y_world, y_slice);

         debug_check!(self.check_upper_range(y_slice));

         debug_check!(self.check_upper_pool());

         self.h_slice(y_slice, y + 1);

         debug_check!(self.check_upper_bounds(y_slice));

         debug_check!(self.check_lower_initial_pool());

         debug_check!(self.check_lower_initial_bounds(y_slice));

         self.update_lower_min_max_x();

         debug_check!(self.check_lower_min_max_x(min_x, max_x));

         let mut x = x_start;

         while x < x_end {
            let x_world = from_px(x);
            let x_slice = x_world + DIV_PER_PIXEL;

            self.final_active_full = 0;

            self.advance_lower_range(x_world, x_slice);

            debug_check!(self.check_lower_range(x_slice));

            debug_check!(self.check_lower_pool());

            match self.can_advance_stripe(x_end) {
               Some(x_delta) => {
                  let poly_index = self.lower_active[self.lower_active_start];

                  self.v_slice_poly(poly_index, from_px(x_delta), x_delta);

                  let ref color = scene.colors[scene.polys[poly_index].color];

                  for fill_x in x..x_delta {
                     frame.put_pixel(fill_x as i32, y as i32, &color);
                  }

                  x = x_delta;
               },
               None => {
                  self.v_slice(x_slice, x + 1);

                  debug_check!(self.check_lower_bounds(x_slice));

                  debug_check!(self.check_final_pool());

                  debug_check!(self.check_final_bounds(x_slice));

                  if self.final_active_full != 0 {
                     let color = self.active_color(scene);

                     frame.put_pixel(x as i32, y as i32, &color);
                  }

                  x += 1;
               }
            }
         }
      }
   }

   pub fn transfer_scene(&mut self, scene: &Scene) {
      self.polys_len = scene.polys.len();

      let mut pool_index = 0;
      for i in 0..self.polys_len {
         let ref poly = &scene.polys[i];
         self.poly_to_pool[i] = pool_index;
         self.upper_edges_len[i] = poly.end - poly.start;

         for edge in &scene.edges[poly.start..poly.end] {
            let ref mut edge_ref = self.upper_edges[pool_index];
            edge_ref.edge_type = edge.edge_type;
            edge_ref.segment = edge.segment;
            edge_ref.circle = edge.circle;

            let ref segment = scene.segments[edge.segment];
            let (ref p1, ref p2) = if edge.reversed() {
               (scene.points[segment.p2], scene.points[segment.p1])
            } else {
               (scene.points[segment.p1], scene.points[segment.p2])
            };

            edge_ref.p1 = *p1;
            edge_ref.p2 = *p2;

            pool_index += 1;
         }

         // 4 extra positions for additional horizontal and vertical edges
         // added in during the slice passes
         pool_index += 4;

         self.upper_active[i] = i;
      }

      self.upper_active_start = 0;
      self.upper_active_end = 0;
   }

   fn min_max_x_y(&self, scene: &Scene) -> (i64, i64, i64, i64) {
      let mut min_x = i64::MAX;
      let mut min_y = i64::MAX;

      let mut max_x = i64::MIN;
      let mut max_y = i64::MIN;

      for segment in &scene.segments {
         let p1 = &scene.points[segment.p1];
         let p2 = &scene.points[segment.p2];

         let s_min_x = min(p1.x, p2.x);
         let s_min_y = min(p1.y, p2.y);

         let s_max_x = min(p1.x, p2.x);
         let s_max_y = min(p1.y, p2.y);

         if s_min_x < min_x {
            min_x = s_min_x;
         }

         if s_min_y < min_y {
            min_y = s_min_y;
         }

         if s_max_x > max_x {
            max_x = s_max_x;
         }

         if s_max_y > max_y {
            max_y = s_max_y;
         }
      }

      (min_x, min_y, max_x, max_y)
   }

   fn check_min_max_x_y(&self, min_x: i64, min_y: i64, max_x: i64, max_y: i64) {
      if min_x == i64::MAX {
         panic!("Unitialized min_x");
      }

      if min_y == i64::MAX {
         panic!("Unitialized min_y");
      }

      if max_x == i64::MIN {
         panic!("Unitialized max_x");
      }

      if max_y == i64::MIN {
         panic!("Unitialized max_y");
      }

      if min_x > max_x {
         panic!("Wrong min_x max_x");
      }

      if min_y > max_y {
         panic!("Wrong min_y max_y");
      }
   }

   fn update_upper_min_max_y(&mut self) {
      for poly_index in 0..self.polys_len {
         let poly_start = self.poly_to_pool[poly_index];
         let poly_end = poly_start + self.upper_edges_len[poly_index];

         let mut poly_min_y = i64::MAX;
         let mut poly_max_y = i64::MIN;

         for edge_i in poly_start..poly_end {
            let ref edge = self.upper_edges[edge_i];

            if edge.p1.y < poly_min_y  {
               poly_min_y = edge.p1.y;
            }

            if edge.p1.y > poly_max_y {
               poly_max_y = edge.p1.y;
            }
         }

         self.upper_min_y[poly_index] = poly_min_y;
         self.upper_max_y[poly_index] = poly_max_y;
      }

      self.sort_upper_active();
   }

   fn sort_upper_active(&mut self) {
      let upper_active = &mut self.upper_active;
      let upper_min_y = &self.upper_min_y;
      let upper_max_y = &self.upper_max_y;

      upper_active[..self.polys_len].sort_by(|a, b| {
         match upper_min_y[*a].cmp(&upper_min_y[*b]) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => upper_max_y[*a].cmp(&upper_max_y[*b])
         }
      });
   }

   fn check_upper_min_max_y(&self, all_min_y: i64, all_max_y: i64) {
      let mut prev_min_y = i64::MIN;
      for active_index in 0..self.polys_len {
         let poly_index = self.upper_active[active_index];

         let min_y = self.upper_min_y[poly_index];
         let max_y = self.upper_max_y[poly_index];

         if min_y == i64::MAX || min_y < all_min_y {
            panic!("Bad poly min_y value");
         }
         if max_y == i64::MIN || max_y > all_max_y {
            panic!("Bad poly max_y value");
         }

         if prev_min_y > min_y  {
            panic!("Polys not sorted by min y");
         }

         prev_min_y = min_y;
      }
   }

   fn update_lower_min_max_x(&mut self) {
      for active_index in 0..self.lower_active_full {
         let poly_index = self.lower_active[active_index];

         let poly_start = self.poly_to_pool[poly_index];
         let poly_end = poly_start + self.lower_edges_len[poly_index];

         let mut poly_min_x = i64::MAX;
         let mut poly_max_x = i64::MIN;

         for edge_i in poly_start..poly_end {
            let ref edge = self.lower_edges[edge_i];

            if edge.p1.x < poly_min_x  {
               poly_min_x = edge.p1.x;
            }

            if edge.p1.x > poly_max_x {
               poly_max_x = edge.p1.x;
            }
         }

         self.lower_min_x[poly_index] = poly_min_x;
         self.lower_max_x[poly_index] = poly_max_x;
      }

      self.sort_lower_active();
   }

   fn sort_lower_active(&mut self) {
      let lower_active = &mut self.lower_active;
      let lower_min_x = &self.lower_min_x;
      let lower_max_x = &self.lower_max_x;

      lower_active[..self.lower_active_full].sort_by(|a, b| {
         match lower_min_x[*a].cmp(&lower_min_x[*b]) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => lower_max_x[*a].cmp(&lower_max_x[*b])
         }
      });
   }

   fn check_lower_min_max_x(&self, all_min_x: i64, all_max_x: i64) {
      let mut prev_min_x = i64::MIN;
      for active_index in 0..self.lower_active_full {
         let poly_index = self.lower_active[active_index];

         let min_x = self.lower_min_x[poly_index];
         let max_x = self.lower_max_x[poly_index];

         if min_x == i64::MAX || min_x < all_min_x {
            panic!("Bad poly min_x value");
         }
         if max_x == i64::MIN || max_x > all_max_x {
            panic!("Bad poly max_x value");
         }

         if prev_min_x > min_x  {
            panic!("Polys not sorted by min x");
         }

         prev_min_x = min_x;
      }
   }

   fn advance_upper_range(&mut self, y_world: i64, y_slice: i64) {
      self.advance_upper_range_end(y_slice);

      self.advance_upper_range_start(y_world, y_slice);
   }

   fn advance_upper_range_start(&mut self, y_world: i64, y_slice: i64) {
      while self.upper_active_start < self.upper_active_end {
         let poly_index = self.upper_active[self.upper_active_start];

         let max_y = self.upper_max_y[poly_index];
         if max_y > y_slice {
            break;
         }

         if max_y > y_world {
            self.copy_to_lower(poly_index);
         }

         self.upper_active_start += 1;
      }
   }

   fn advance_upper_range_end(&mut self, y_slice: i64) {
      while self.upper_active_end < self.polys_len {
         let poly_index = self.upper_active[self.upper_active_end];

         let min_y = self.upper_min_y[poly_index];
         if min_y >= y_slice {
            break;
         }

         self.upper_active_end += 1;

         self.sort_sink_upper_last_poly();
      }
   }

   fn copy_to_lower(&mut self, poly_index: usize) {
      let poly_start = self.poly_to_pool[poly_index];

      let poly_len = self.upper_edges_len[poly_index];
      self.lower_edges_len[poly_index] = poly_len;

      let poly_end = poly_start + poly_len;
      for edge_i in poly_start..poly_end {
         self.lower_edges[edge_i] = self.upper_edges[edge_i];
      }

      self.add_lower_active(poly_index);
   }

   fn add_lower_active(&mut self, poly_index: usize) {
      self.lower_active[self.lower_active_full] = poly_index;
      self.lower_active_full += 1;
   }

   fn sort_sink_upper_last_poly(&mut self) {
      let mut active_this = self.upper_active_end - 1;
      if active_this <= self.upper_active_start {
         return;
      }

      let mut active_prev = active_this - 1;

      while active_prev >= self.upper_active_start {
         let poly_this = self.upper_active[active_this];
         let poly_prev = self.upper_active[active_prev];

         let max_y_this = self.upper_max_y[poly_this];
         let max_y_prev = self.upper_max_y[poly_prev];

         if max_y_prev <= max_y_this {
            return;
         }

         self.upper_active[active_this] = poly_prev;
         self.upper_active[active_prev] = poly_this;

         active_this -= 1;
         if active_this <= self.upper_active_start {
            return;
         }
         active_prev = active_this - 1;
      }
   }

   fn check_upper_range(&self, y_slice: i64) {
      for i in 0..self.upper_active_start {
         let poly_index = self.upper_active[i];

         let max_y = self.upper_max_y[poly_index];
         if max_y > y_slice {
            panic!("Discarded upper poly above slice point: {}", poly_index);
         }
      }

      let mut prev_max_y = i64::MIN;

      for i in self.upper_active_start..self.upper_active_end {
         let poly_index = self.upper_active[i];

         let min_y = self.upper_min_y[poly_index];
         let max_y = self.upper_max_y[poly_index];

         if max_y < prev_max_y {
            panic!(
               "Active poly max y smaller than previous: [{}] {} / {} / {}",
               poly_index, max_y, prev_max_y, y_slice
            );
         }

         if max_y <= y_slice {
            panic!("Active poly max y too low: {}", poly_index);
         }

         if min_y >= y_slice {
            panic!("Active poly min y too high: {}", poly_index);
         }

         prev_max_y = max_y;
      }

      for i in self.upper_active_end..self.polys_len {
         let poly_index = self.upper_active[i];

         let min_y = self.upper_min_y[poly_index];
         if min_y < y_slice {
            panic!("Upcoming poly below slice point: {}", poly_index);
         }
      }
   }

   fn advance_lower_range(&mut self, x_world: i64, x_slice: i64) {
      self.advance_lower_range_end(x_slice);

      self.advance_lower_range_start(x_world, x_slice);
   }

   fn advance_lower_range_start(&mut self, x_world: i64, x_slice: i64) {
      while self.lower_active_start < self.lower_active_end {
         let poly_index = self.lower_active[self.lower_active_start];

         let max_x = self.lower_max_x[poly_index];
         if max_x > x_slice {
            break;
         }

         if max_x > x_world {
            self.copy_to_final(poly_index);
         }

         self.lower_active_start += 1;
      }
   }

   fn advance_lower_range_end(&mut self, x_slice: i64) {
      while self.lower_active_end < self.lower_active_full {
         let poly_index = self.lower_active[self.lower_active_end];

         let min_x = self.lower_min_x[poly_index];
         if min_x >= x_slice {
            break;
         }

         self.lower_active_end += 1;

         self.sort_sink_lower_last_poly();
      }
   }

   fn copy_to_final(&mut self, poly_index: usize) {
      let poly_start = self.poly_to_pool[poly_index];

      let poly_len = self.lower_edges_len[poly_index];
      self.final_edges_len[poly_index] = poly_len;

      let poly_end = poly_start + poly_len;
      for edge_i in poly_start..poly_end {
         self.final_edges[edge_i] = self.lower_edges[edge_i];
      }

      self.add_final_active(poly_index);
   }

   fn add_final_active(&mut self, poly_index: usize) {
      self.final_active[self.final_active_full] = poly_index;
      self.final_active_full += 1;
   }

   fn sort_sink_lower_last_poly(&mut self) {
      let mut active_this = self.lower_active_end - 1;
      if active_this <= self.lower_active_start {
         return;
      }

      let mut active_prev = active_this - 1;

      while active_prev >= self.lower_active_start {
         let poly_this = self.lower_active[active_this];
         let poly_prev = self.lower_active[active_prev];

         let max_x_this = self.lower_max_x[poly_this];
         let max_x_prev = self.lower_max_x[poly_prev];

         if max_x_prev <= max_x_this {
            return;
         }

         self.lower_active[active_this] = poly_prev;
         self.lower_active[active_prev] = poly_this;

         active_this -= 1;
         if active_this <= self.lower_active_start {
            return;
         }
         active_prev = active_this - 1;
      }
   }

   fn check_lower_range(&self, x_slice: i64) {
      for i in 0..self.lower_active_start {
         let poly_index = self.lower_active[i];

         let max_x = self.lower_max_x[poly_index];
         if max_x > x_slice {
            panic!("Discarded lower poly after slice point: [{}] {} / {}", poly_index, max_x, x_slice);
         }
      }

      let mut prev_max_x = i64::MIN;

      for i in self.lower_active_start..self.lower_active_end {
         let poly_index = self.lower_active[i];

         let min_x = self.lower_min_x[poly_index];
         let max_x = self.lower_max_x[poly_index];

         if max_x < prev_max_x {
            panic!(
               "Active poly max x smaller than previous: [{}] {} / {} / {}",
               poly_index, max_x, prev_max_x, x_slice
            );
         }

         if max_x <= x_slice {
            panic!("Active poly max x too small: [{}] {} / {}", poly_index, max_x, x_slice);
         }

         if min_x >= x_slice {
            panic!("Active poly min x too big: [{}] {} / {}", poly_index, min_x, x_slice);
         }

         prev_max_x = max_x;
      }

      for i in self.lower_active_end..self.lower_active_full {
         let poly_index = self.lower_active[i];

         let min_x = self.lower_min_x[poly_index];
         if min_x < x_slice {
            panic!("Upcoming poly before slice point: [{}] {} / {}", poly_index, min_x, x_slice);
         }
      }
   }

   fn h_slice(&mut self, y: i64, y_px: i64) {
      for i in self.upper_active_start..self.upper_active_end {
         let poly_index = self.upper_active[i];

         debug_assert!(self.upper_max_y[poly_index] > y);

         self.h_slice_poly(poly_index, y, y_px);

         self.add_lower_active(poly_index);
      }
   }

   fn h_slice_poly(&mut self, poly_index: usize, y: i64, y_px: i64) {
      let p1;
      let mut p2 = Point::default();

      let poly_start = self.poly_to_pool[poly_index];
      let poly_end = poly_start + self.upper_edges_len[poly_index];

      let mut i = poly_start;
      let mut upper_i = poly_start;
      let mut lower_i = poly_start;

      loop { // Edge's first point below y
         let mut edge = self.upper_edges[i];

         match edge.edge_type {
            EdgeType::LTR | EdgeType::LTL | EdgeType::LVT | EdgeType::CTR |
            EdgeType::CTL | EdgeType::ATR | EdgeType::ATL => {
               let y2 = edge.p2.y;
               if y2 < y {
                  self.lower_edges[lower_i] = edge;
                  lower_i += 1;
               } else if y2 > y {
                  let x1_intersect = self.h_intersection(&edge, y_px);

                  p1 = Point::new(x1_intersect, y);

                  let mut upper_edge = edge.clone();

                  edge.p2 = p1;
                  self.lower_edges[lower_i] = edge;
                  lower_i += 1;

                  upper_edge.p1 = p1;
                  self.upper_edges[upper_i] = upper_edge;
                  upper_i += 1;

                  break;
               } else {
                  p1 = edge.p2;

                  self.lower_edges[lower_i] = edge;
                  lower_i += 1;

                  break;
               }
            },
            _ => {
               self.lower_edges[lower_i] = edge;
               lower_i += 1;
            }
         }

         i += 1;

         debug_assert!(i != poly_end, "Polygon should not end before Y slice line");
      }

      i += 1;

      loop { // Edge's first point above y
         let mut edge = self.upper_edges[i];

         match edge.edge_type {
            EdgeType::LBR | EdgeType::LBL | EdgeType::LVB | EdgeType::CBR |
            EdgeType::CBL | EdgeType::ABR | EdgeType::ABL => {
               let y2 = edge.p2.y;
               if y2 > y {
                  self.upper_edges[upper_i] = edge;
                  upper_i += 1;
               } else if y2 < y {
                  let x2_intersect = self.h_intersection(&edge, y_px);

                  p2 = Point::new(x2_intersect, y);

                  let mut lower_edge = edge.clone();

                  edge.p2 = p2;
                  self.upper_edges[upper_i] = edge;
                  upper_i += 1;

                  self.lower_edges[lower_i] = Edge::hori_right(p1, p2);
                  lower_i += 1;

                  lower_edge.p1 = p2;
                  self.lower_edges[lower_i] = lower_edge;
                  lower_i += 1;

                  break;
               } else {
                  p2 = edge.p2;

                  self.upper_edges[upper_i] = edge;
                  upper_i += 1;

                  self.lower_edges[lower_i] = Edge::hori_right(p1, p2);
                  lower_i += 1;

                  break;
               }
            }
            _ => {
               self.upper_edges[upper_i] = edge;
               upper_i += 1;
            }

         }

         i += 1;

         if i == poly_end {
            break;
         }
      }

      i += 1;

      for j in i..poly_end { // Edge's first point again below y
         self.lower_edges[lower_i] = self.upper_edges[j];
         lower_i += 1;
      }

      let last_upper = Edge::hori_left(p2, p1);
      self.upper_edges[upper_i] = last_upper;
      upper_i += 1;

      self.upper_edges_len[poly_index] = upper_i - poly_start;
      self.lower_edges_len[poly_index] = lower_i - poly_start;
   }

   fn v_slice(&mut self, x: i64, x_px: i64) {
      for i in self.lower_active_start..self.lower_active_end {
         let poly_index = self.lower_active[i];

         debug_assert!(self.lower_max_x[poly_index] > x);

         self.v_slice_poly(poly_index, x, x_px);

         self.add_final_active(poly_index);
      }
   }

   fn v_slice_poly(&mut self, poly_index: usize, x: i64, x_px: i64) {
      let p1;
      let mut p2 = Point::default();

      let poly_start = self.poly_to_pool[poly_index];
      let poly_end = poly_start + self.lower_edges_len[poly_index];

      let mut i = poly_start;
      let mut lower_i = poly_start;
      let mut final_i = poly_start;

      loop { // Edge's first point to the left of x
         let mut edge = self.lower_edges[i];

         match edge.edge_type {
            EdgeType::LTR | EdgeType::LBR | EdgeType::LHR | EdgeType::CTR |
            EdgeType::CBR | EdgeType::ATR | EdgeType::ABR => {
               let x2 = edge.p2.x;
               if x2 < x {
                  self.final_edges[final_i] = edge;
                  final_i += 1;
               } else if x2 > x {
                  let y1_intersect = self.v_intersection(&edge, x_px);

                  p1 = Point::new(x, y1_intersect);

                  let mut lower_edge = edge.clone();

                  edge.p2 = p1;
                  self.final_edges[final_i] = edge;
                  final_i += 1;

                  lower_edge.p1 = p1;
                  self.lower_edges[lower_i] = lower_edge;
                  lower_i += 1;

                  break;
               } else {
                  p1 = edge.p2;

                  self.final_edges[final_i] = edge;
                  final_i += 1;

                  break;
               }
            },
            _ => {
               self.final_edges[final_i] = edge;
               final_i += 1;
            }
         }

         i += 1;

         if i == poly_end {
            panic!("Polygon ends before X slice line");
         }
      }

      i += 1;

      loop { // Edge's first point to the right of x
         let mut edge = self.lower_edges[i];

         match edge.edge_type {
            EdgeType::LTL | EdgeType::LBL | EdgeType::LHL | EdgeType::CTL |
            EdgeType::CBL | EdgeType::ATL | EdgeType::ABL => {
               let x2 = edge.p2.x;
               if x2 > x {
                  self.lower_edges[lower_i] = edge;
                  lower_i += 1;
               } else if x2 < x {
                  let y2_intersect = self.v_intersection(&edge, x_px);

                  p2 = Point::new(x, y2_intersect);

                  let mut final_edge = edge.clone();

                  edge.p2 = p2;
                  self.lower_edges[lower_i] = edge;
                  lower_i += 1;

                  self.final_edges[final_i] = Edge::vert_bottom(p1, p2);
                  final_i += 1;

                  final_edge.p1 = p2;
                  self.final_edges[final_i] = final_edge;
                  final_i += 1;

                  break;
               } else {
                  p2 = edge.p2;

                  self.lower_edges[lower_i] = edge;
                  lower_i += 1;

                  self.final_edges[final_i] = Edge::vert_bottom(p1, p2);
                  final_i += 1;

                  break;
               }
            }
            _ => {
               self.lower_edges[lower_i] = edge;
               lower_i += 1;
            }

         }

         i += 1;

         if i == poly_end {
            break;
         }
      }

      i += 1;

      for j in i..poly_end { // Edge's first point again to the left of x
         self.final_edges[final_i] = self.lower_edges[j];
         final_i += 1;
      }

      let last_lower = Edge::vert_top(p2, p1);
      self.lower_edges[lower_i] = last_lower;
      lower_i += 1;

      self.lower_edges_len[poly_index] = lower_i - poly_start;
      self.final_edges_len[poly_index] = final_i - poly_start;
   }

   #[inline]
   fn can_advance_stripe(&self, x_end: i64) -> Option<i64> {
      if self.final_active_full != 0 {
         return None;
      }

      let active_len = self.lower_active_end - self.lower_active_start;

      if active_len != 1 {
         return None;
      }

      if self.lower_active_end == self.lower_active_full {
         return Some(x_end);
      }

      let poly_index = self.lower_active[self.lower_active_end];

      Some(self.lower_min_x[poly_index] / DIV_PER_PIXEL)
   }

   #[inline]
   pub fn active_color(&self, scene: &Scene) -> RGB {
      let mut r: i64 = 0;
      let mut g: i64 = 0;
      let mut b: i64 = 0;

      let mut total_area: i64 = 0;

      for active_index in 0..self.final_active_full - 1 {
         let poly_index = self.final_active[active_index];

         let area = self.double_area(poly_index, scene);

         let ref color = scene.colors[scene.polys[poly_index].color];

         r += (color.r as i64) * area;
         g += (color.g as i64) * area;
         b += (color.b as i64) * area;

         total_area += area;
      }

      let poly_index = self.final_active[self.final_active_full - 1];

      let area = DOUBLE_PIXEL_AREA - total_area;

      let ref color = scene.colors[scene.polys[poly_index].color];

      r += (color.r as i64) * area;
      g += (color.g as i64) * area;
      b += (color.b as i64) * area;

      r /= DOUBLE_PIXEL_AREA;
      g /= DOUBLE_PIXEL_AREA;
      b /= DOUBLE_PIXEL_AREA;

      RGB::new(r as u8, g as u8, b as u8)
   }

   #[inline]
   fn double_area(&self, poly_index: usize, scene: &Scene) -> i64 {
      let poly_start = self.poly_to_pool[poly_index];
      let poly_end = poly_start + self.final_edges_len[poly_index];

      let mut area = 0;

      for edge_i in poly_start..poly_end {
         let ref edge = self.final_edges[edge_i];

         match edge.edge_type {
            EdgeType::LHR | EdgeType::LHL => {
               area += (edge.p2.x - edge.p1.x) * 2 * edge.p1.y;
            },
            EdgeType::LTR | EdgeType::LBR | EdgeType::CTR => {
               area += (edge.p2.x - edge.p1.x) * (edge.p1.y + edge.p2.y);
            },
            EdgeType::CBR | EdgeType::ATR | EdgeType::ABR |
            EdgeType::LTL | EdgeType::LBL | EdgeType::CTL |
            EdgeType::CBL | EdgeType::ATL | EdgeType::ABL => {
               let dx = edge.p2.x - edge.p1.x;
               let dy = edge.p2.y - edge.p1.y;

               area += dx * (edge.p1.y + edge.p2.y);

               let side = (dx * dx + dy * dy).sqrt();

               let ref circle = scene.circles[edge.circle];
               let radius = circle.radius;

               let alpha = 2. * (side as f64 / (2 * radius) as f64).asin();

               let segment_area = ((radius * radius) as f64 * (alpha - alpha.sin())) as i64;

               match edge.edge_type {
                  EdgeType::CTR | EdgeType::CTL | EdgeType::CBR | EdgeType::CBL => area += segment_area,
                  _ => area -= segment_area
               }
            },
            _ => {}
         }
      }

      debug_assert!(area >= 0);

      area
   }
}

fn create_default_vec<T>(capacity: usize) -> Vec<T> where T: Default + Clone {
   repeat(T::default()).take(capacity).collect()
}

