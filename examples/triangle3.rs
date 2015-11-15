#![feature(augmented_assignments)]

#![allow(dead_code)]
extern crate polydraw;

use std::i64;
use std::usize;
use std::cmp::{min, max};
use std::iter::repeat;
use std::fmt::Debug;

use polydraw::{Application, Renderer, Frame};
use polydraw::geom::point::Point;
use polydraw::geom::ring::Ring;
use polydraw::geom::lineinter::{h_multi_intersect_fast, v_multi_intersect_fast};
use polydraw::draw::RGB;


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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Poly {
   color: RGB,
   start: usize,
   end: usize,
}

impl Poly {
   #[inline]
   pub fn new(color: RGB, start: usize, end: usize) -> Self {
      Poly {
         color: color,
         start: start,
         end: end,
      }
   }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum EdgeType {
   TR,  // top right
   TL,  // top left
   BR,  // bottom right
   BL,  // bottom left
   HR,  // horizontal right
   HL,  // horizontal left
   VT,  // vertical top
   VB,  // vertical bottom
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Edge {
   edge_type: EdgeType,
   points: usize,
}

impl Edge {
   #[inline]
   pub fn new(edge_type: EdgeType, points: usize) -> Self {
      Edge {
         edge_type: edge_type,
         points: points,
      }
   }

   #[inline]
   pub fn top_right(points: usize) -> Self {
      Edge::new(EdgeType::TR, points)
   }

   #[inline]
   pub fn top_left(points: usize) -> Self {
      Edge::new(EdgeType::TL, points)
   }

   #[inline]
   pub fn bottom_right(points: usize) -> Self {
      Edge::new(EdgeType::BR, points)
   }

   #[inline]
   pub fn bottom_left(points: usize) -> Self {
      Edge::new(EdgeType::BL, points)
   }

   #[inline]
   pub fn hori_right(points: usize) -> Self {
      Edge::new(EdgeType::HR, points)
   }

   #[inline]
   pub fn hori_left(points: usize) -> Self {
      Edge::new(EdgeType::HL, points)
   }

   #[inline]
   pub fn vert_top(points: usize) -> Self {
      Edge::new(EdgeType::VT, points)
   }

   #[inline]
   pub fn vert_bottom(points: usize) -> Self {
      Edge::new(EdgeType::VB, points)
   }
}

impl Default for Edge {
   #[inline]
   fn default() -> Edge {
      Edge::top_right(0)
   }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct EdgeRef {
   edge_type: EdgeType,
   src_points: usize,
   points: usize,
}

impl EdgeRef {
   #[inline]
   pub fn new(edge_type: EdgeType, src_points: usize, points: usize) -> Self {
      EdgeRef {
         edge_type: edge_type,
         src_points: src_points,
         points: points,
      }
   }

   #[inline]
   pub fn new_ref(&self, points: usize) -> Self {
      EdgeRef::new(self.edge_type, self.src_points, points)
   }
}

impl Default for EdgeRef {
   #[inline]
   fn default() -> EdgeRef {
      EdgeRef::new(EdgeType::TR, 0, 0)
   }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct EdgePoints {
   p1: usize,
   p2: usize,
}

impl EdgePoints {
   #[inline]
   pub fn new(p1: usize, p2: usize) -> Self {
      EdgePoints {
         p1: p1,
         p2: p2,
      }
   }
}

impl Default for EdgePoints {
   #[inline]
   fn default() -> EdgePoints {
      EdgePoints::new(0, 0)
   }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct PolyRef {
   src: usize,
   start: usize,
   end: usize,
}

impl PolyRef {
   #[inline]
   pub fn new(src: usize, start: usize, end: usize) -> Self {
      PolyRef {
         src: src,
         start: start,
         end: end,
      }
   }
}

impl Default for PolyRef {
   #[inline]
   fn default() -> PolyRef {
      PolyRef::new(0, 0, 0)
   }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct PolyMinRef {
   poly: usize,
   min: i64,
}

impl PolyMinRef {
   #[inline]
   pub fn new(poly: usize, min: i64) -> Self {
      PolyMinRef {
         poly: poly,
         min: min,
      }
   }
}

impl Default for PolyMinRef {
   #[inline]
   fn default() -> PolyMinRef {
      PolyMinRef::new(usize::MAX, i64::MAX)
   }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct IntersectRef {
   start_px: i64,
   start: usize,
   end: usize,
}

impl Default for IntersectRef {
   #[inline]
   fn default() -> IntersectRef {
      IntersectRef {
         start_px: i64::MAX,
         start: usize::MAX,
         end: usize::MAX,
      }
   }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct MinMaxXY {
   min_x: i64,
   min_y: i64,
   max_x: i64,
   max_y: i64,
}

impl MinMaxXY {
   #[inline]
   pub fn new(min_x: i64, min_y: i64, max_x: i64, max_y: i64) -> Self {
      MinMaxXY {
         min_x: min_x,
         min_y: min_y,
         max_x: max_x,
         max_y: max_y,
      }
   }
}

impl Default for MinMaxXY {
   #[inline]
   fn default() -> MinMaxXY {
      MinMaxXY::new(i64::MAX, i64::MAX, i64::MIN, i64::MIN)
   }
}

struct PolySource {
   polys: Vec<Poly>,
   edges: Vec<Edge>,
   edge_points: Vec<EdgePoints>,
   points: Vec<Point>,
}

impl PolySource {
   fn new() -> Self {
      let polys = vec![
         // A
         Poly::new(RGB::new(18, 78, 230), 0, 3),
         // B
         Poly::new(RGB::new(47, 11, 206), 3, 6),
         // C
         Poly::new(RGB::new(170, 44, 206), 6, 9),
         // D
         Poly::new(RGB::new(18, 78, 230), 0, 3),
         // E
         Poly::new(RGB::new(170, 36, 14), 12, 15),
         // F
         Poly::new(RGB::new(170, 44, 206), 6, 9),
         // G
         Poly::new(RGB::new(47, 11, 206), 3, 6),
         // H
         Poly::new(RGB::new(170, 44, 206), 6, 9),
         // I
         Poly::new(RGB::new(109, 233, 158), 24, 27),
         // J
         Poly::new(RGB::new(47, 11, 206), 3, 6),
         // K
         Poly::new(RGB::new(170, 44, 206), 6, 9),
         // L
         Poly::new(RGB::new(47, 11, 206), 3, 6),
      ];

      let edges = vec![
         // 0: A
         Edge::vert_top(4),
         Edge::top_right(14),
         Edge::bottom_right(11),
         Edge::bottom_left(5),
         Edge::hori_left(0),

         // 1: B
         Edge::top_right(5),
         Edge::bottom_right(9),
         Edge::vert_bottom(6),
         Edge::hori_left(1),

         // 2: C
         Edge::vert_top(6),
         Edge::top_right(10),
         Edge::bottom_right(7),
         Edge::hori_left(2),

         // 3: D
         Edge::top_left(7),
         Edge::top_right(12),
         Edge::bottom_right(15),
         Edge::vert_bottom(8),
         Edge::hori_left(3),

         // 4: E
         Edge::top_left(9),
         Edge::top_left(11),
         Edge::top_left(17),
         Edge::top_right(22),
         Edge::hori_right(29),
         Edge::bottom_right(23),
         Edge::bottom_left(18),
         Edge::bottom_left(12),
         Edge::bottom_left(10),

         // 5: F
         Edge::vert_top(13),
         Edge::hori_right(19),
         Edge::bottom_right(17),
         Edge::bottom_left(14),

         // 6: G
         Edge::vert_top(21),
         Edge::hori_right(25),
         Edge::vert_bottom(26),
         Edge::bottom_left(22),
         Edge::hori_left(19),

         // 7: H
         Edge::vert_top(26),
         Edge::hori_right(30),
         Edge::vert_bottom(27),
         Edge::hori_left(29),

         // 8: I
         Edge::vert_top(27),
         Edge::hori_right(28),
         Edge::vert_bottom(24),
         Edge::hori_left(20),
         Edge::top_left(23),

         // 9: J
         Edge::top_left(15),
         Edge::top_right(18),
         Edge::hori_right(20),
         Edge::vert_bottom(16),
      ];

      let edge_points = vec![
         EdgePoints::new(0, 1),    // 0
         EdgePoints::new(1, 2),    // 1
         EdgePoints::new(2, 3),    // 2
         EdgePoints::new(3, 4),    // 3
         EdgePoints::new(0, 8),    // 4
         EdgePoints::new(1, 5),    // 5
         EdgePoints::new(2, 6),    // 6
         EdgePoints::new(3, 7),    // 7
         EdgePoints::new(4, 9),    // 8
         EdgePoints::new(6, 5),    // 9
         EdgePoints::new(6, 7),    // 10
         EdgePoints::new(5, 10),   // 11
         EdgePoints::new(7, 11),   // 12
         EdgePoints::new(8, 12),   // 13
         EdgePoints::new(8, 10),   // 14
         EdgePoints::new(9, 11),   // 15
         EdgePoints::new(9, 15),   // 16
         EdgePoints::new(10, 13),  // 17
         EdgePoints::new(11, 14),  // 18
         EdgePoints::new(12, 13),  // 19
         EdgePoints::new(14, 15),  // 20
         EdgePoints::new(12, 18),  // 21
         EdgePoints::new(13, 16),  // 22
         EdgePoints::new(14, 17),  // 23
         EdgePoints::new(15, 21),  // 24
         EdgePoints::new(18, 19),  // 25
         EdgePoints::new(16, 19),  // 26
         EdgePoints::new(17, 20),  // 27
         EdgePoints::new(20, 21),  // 28
         EdgePoints::new(16, 17),  // 29
         EdgePoints::new(19, 20),  // 30
      ];

      let points = vec![
         Point::new(-22, -14), // 0
         Point::new(-6, -14),  // 1
         Point::new(0, -14),   // 2
         Point::new(6, -14),   // 3
         Point::new(22, -14),  // 4
         Point::new(-3, -11),  // 5
         Point::new(0, 12),    // 6
         Point::new(3, -11),   // 7
         Point::new(-22, 2),   // 8
         Point::new(22, 2),    // 9
         Point::new(-19, 5),   // 10
         Point::new(19, 5),    // 11
         Point::new(-22, 8),   // 12
         Point::new(-20, 8),   // 13
         Point::new(20, 8),    // 14
         Point::new(22, 8),    // 15
         Point::new(-16, 12),  // 16
         Point::new(16, 12),   // 17
         Point::new(-22, 14),  // 18
         Point::new(-16, 14),  // 19
         Point::new(16, 14),   // 20
         Point::new(22, 14),   // 21
      ];

      PolySource {
         polys: polys,
         edges: edges,
         edge_points: edge_points,
         points: points,
      }
   }
}

struct TriangleRenderer {
   src: PolySource,
   src_min_y: Vec<PolyMinRef>,
   src_poly_marker: usize,
   src_min_max: Vec<MinMaxXY>,

   scaled_points: Vec<Point>,

   edge_points_map: Vec<usize>,
   points_map: Vec<usize>,

   h_intersect_ref: Vec<IntersectRef>,
   v_intersect_ref: Vec<IntersectRef>,
   h_intersections: Ring<i64>,
   v_intersections: Ring<i64>,

   upper_polys: Ring<PolyRef>,
   upper_edges: Ring<EdgeRef>,

   lower_polys: Ring<PolyRef>,
   lower_edges: Ring<EdgeRef>,
   lower_min_x: Vec<i64>,
   lower_max_x: Vec<i64>,

   active_polys: Ring<PolyRef>,
   active_edges: Ring<EdgeRef>,

   edge_points: Ring<EdgePoints>,
   points: Ring<Point>,

   seed: u64,
   counter: i64,
}

impl TriangleRenderer {
   fn new() -> Self {
      let src = PolySource::new();

      let polys_len = src.polys.len();
      let edge_points_len = src.edge_points.len();
      let points_len = src.points.len();

      let src_min_y = repeat(PolyMinRef::default()).take(polys_len).collect();
      let src_min_max = repeat(MinMaxXY::default()).take(polys_len).collect();

      let scaled_points = repeat(Point::default()).take(points_len).collect();

      let edge_points_map = repeat(usize::MAX).take(edge_points_len).collect();
      let points_map = repeat(usize::MAX).take(points_len).collect();

      let h_intersect_ref = repeat(IntersectRef::default()).take(edge_points_len).collect();
      let v_intersect_ref = repeat(IntersectRef::default()).take(edge_points_len).collect();

      let lower_min_x = repeat(i64::MAX).take(polys_len).collect();
      let lower_max_x = repeat(i64::MIN).take(polys_len).collect();

      TriangleRenderer {
         src: src,
         src_min_y: src_min_y,
         src_poly_marker: 0,
         src_min_max: src_min_max,

         scaled_points: scaled_points,

         edge_points_map: edge_points_map,
         points_map: points_map,

         h_intersect_ref: h_intersect_ref,
         v_intersect_ref: v_intersect_ref,
         h_intersections: Ring::new(65536),
         v_intersections: Ring::new(65536),

         upper_polys: Ring::new(65536),
         upper_edges: Ring::new(262144),

         lower_polys: Ring::new(262144),
         lower_edges: Ring::new(262144),
         lower_min_x: lower_min_x,
         lower_max_x: lower_max_x,

         active_polys: Ring::new(262144),
         active_edges: Ring::new(262144),

         edge_points: Ring::new(4194304),
         points: Ring::new(1048576),

         seed: 0,
         counter: 0,
      }
   }

   pub fn rand_u8(&mut self) -> u8 {
       self.seed = self.seed.wrapping_mul(58321).wrapping_add(11113);
       (self.seed.wrapping_shr(16) % 256) as u8
   }

   pub fn calc_polys_min_y(&mut self) {
      for (poly_index, poly) in self.src.polys.iter().enumerate() {
         let mut min_y = i64::MAX;

         for edge_index in poly.start..poly.end {
            let edge_points = self.src.edge_points[self.src.edges[edge_index].points];
            let y = min(
               self.scaled_points[edge_points.p1].y,
               self.scaled_points[edge_points.p2].y
            );

            if y < min_y {
               min_y = y;
            }
         }

         self.src_min_y[poly_index] = PolyMinRef::new(poly_index, min_y);
      }

      self.src_min_y.sort_by(|a, b| a.min.cmp(&b.min));
   }


   pub fn calc_polys_min_max(&mut self) {
      for (poly_index, poly) in self.src.polys.iter().enumerate() {
         self.src_min_max[poly_index] = MinMaxXY::default();
         let poly_min_max = &mut self.src_min_max[poly_index];

         for edge_index in poly.start..poly.end {
            let edge_points = self.src.edge_points[self.src.edges[edge_index].points];

            let p1 = self.scaled_points[edge_points.p1];
            let p2 = self.scaled_points[edge_points.p2];

            let min_x;
            let min_y;
            let max_x;
            let max_y;

            if p1.x > p2.x {
               min_x = p2.x;
               max_x = p1.x;
            } else {
               min_x = p1.x;
               max_x = p2.x;
            }

            if p1.y > p2.y {
               min_y = p2.y;
               max_y = p1.y;
            } else {
               min_y = p1.y;
               max_y = p2.y;
            }

            if min_x < poly_min_max.min_x {
               poly_min_max.min_x = min_x;
            }

            if min_y < poly_min_max.min_y {
               poly_min_max.min_y = min_y;
            }

            if max_x > poly_min_max.max_x {
               poly_min_max.max_x = max_x;
            }

            if max_y > poly_min_max.max_y {
               poly_min_max.max_y = max_y;
            }
         }
      }
   }

   #[inline]
   fn min_max_x_y(&self) -> (i64, i64, i64, i64) {
      let mut min_x = i64::MAX;
      let mut min_y = i64::MAX;

      let mut max_x = i64::MIN;
      let mut max_y = i64::MIN;

      for p in &self.scaled_points {
         if p.x < min_x {
            min_x = p.x;
         }

         if p.y < min_y {
            min_y = p.y;
         }

         if p.x > max_x {
            max_x = p.x;
         }

         if p.y > max_y {
            max_y = p.y;
         }
      }

      (min_x, min_y, max_x, max_y)
   }

   fn clear(&mut self) {
      self.src_poly_marker = 0;

      self.upper_polys.clear();
      self.upper_edges.clear();

      self.lower_polys.clear();
      self.lower_edges.clear();

      self.active_polys.clear();
      self.active_edges.clear();

      self.edge_points.clear();
      self.points.clear();

      self.h_intersections.clear();
      self.v_intersections.clear();

      for intersect_ref in &mut self.h_intersect_ref {
         intersect_ref.start = usize::MAX;
      }

      for intersect_ref in &mut self.v_intersect_ref {
         intersect_ref.start = usize::MAX;
      }

      for map in self.edge_points_map.iter_mut() {
         *map = usize::MAX;
      }

      for map in self.points_map.iter_mut() {
         *map = usize::MAX;
      }
   }

   fn transfer(&mut self, y: i64) {
      let end = self.src_min_y.len();

      while self.src_poly_marker < end {
         let poly_min_y = self.src_min_y[self.src_poly_marker];

         if poly_min_y.min > y {
            break;
         }

         self.transfer_poly(poly_min_y.poly);

         self.src_poly_marker += 1;
      }
   }

   fn transfer_poly(&mut self, src_i: usize) {
      let poly = self.src.polys[src_i];

      let edge_start = self.upper_edges.end();

      for src_edge_i in poly.start..poly.end {
         self.transfer_edge(src_edge_i);
      }

      let edge_end = self.upper_edges.end();

      self.upper_polys.push(
         PolyRef::new(src_i, edge_start, edge_end)
      );
   }

   fn transfer_edge(&mut self, src_i: usize) {
      let edge = self.src.edges[src_i];

      let src_edge_points_i = edge.points;
      let mut edge_points_i = self.edge_points_map[src_edge_points_i];

      if edge_points_i == usize::MAX {
         let edge_points = self.src.edge_points[src_edge_points_i];

         let s1 = edge_points.p1;
         let s2 = edge_points.p2;

         let p1 = self.transfer_point(s1);
         let p2 = self.transfer_point(s2);

         edge_points_i = self.edge_points.end();

         self.edge_points.push(
            EdgePoints::new(p1, p2)
         );

         self.edge_points_map[src_edge_points_i] = edge_points_i;
      }

      let edge = EdgeRef::new(edge.edge_type, src_edge_points_i, edge_points_i);

      self.upper_edges.push(edge);
   }

   fn transfer_point(&mut self, src_i: usize) -> usize {
      let mut i = self.points_map[src_i];

      if i == usize::MAX {
         i = self.points.end();

         self.points_map[src_i] = i;

         self.points.push(
            self.scaled_points[src_i]
         );
      }

      i
   }

   #[inline]
   fn h_split(&mut self, y: i64, y_px: i64) {
      self.lower_polys.clear();
      self.lower_edges.clear();

      let start = self.upper_polys.start();
      let end = self.upper_polys.end();
      for i in start..end {
         let poly = self.upper_polys[i];
         if self.src_min_max[poly.src].max_y <= y {
            let lower_start = self.lower_edges.end();

            for edge_i in poly.start..poly.end {
               self.lower_edges.push(
                  self.upper_edges[edge_i].clone()
               );
            }

            let lower_end = self.lower_edges.end();

            self.lower_polys.push(
               PolyRef::new(poly.src, lower_start, lower_end)
            );
         } else {
            self.h_split_poly(i, y, y_px);
         }
      }

      self.upper_polys.consume_at(end);
   }

   #[inline]
   fn h_split_poly(&mut self, poly_i: usize, y: i64, y_px: i64) {
      let poly = self.upper_polys[poly_i];

      let upper_start = self.upper_edges.end();
      let lower_start = self.lower_edges.end();

      self.h_split_poly_edges(&poly, y, y_px);

      let upper_end = self.upper_edges.end();
      let lower_end = self.lower_edges.end();

      if upper_end > upper_start {
         self.upper_polys.push(
            PolyRef::new(poly.src, upper_start, upper_end)
         )
      }

      if lower_end > lower_start {
         let mut min_x = i64::MAX;
         let mut min_i = 0;

         for i in lower_start..lower_end {
            let edge = self.lower_edges[i];
            let edge_points = self.edge_points[edge.points];
            let x1 = self.edge_x1(&edge, &edge_points);
            if x1 < min_x {
               min_x = x1;
               min_i = i;
            }
         }

         if (min_i + 1) < lower_end {
            let edge = self.lower_edges[min_i + 1];
            let edge_points = self.edge_points[edge.points];
            let x1 = self.edge_x1(&edge, &edge_points);
            if x1 == min_x {
               min_i += 1;
            }
         }

         let (lower_start, lower_end) = if min_i != lower_start {
            for i in min_i..lower_end {
               let edge = self.lower_edges[i];
               self.lower_edges.push(edge);
            }

            for i in lower_start..min_i {
               let edge = self.lower_edges[i];
               self.lower_edges.push(edge);
            }

            (lower_end, 2 * lower_end - lower_start)
         } else {
            (lower_start, lower_end)
         };

         self.lower_polys.push(
            PolyRef::new(poly.src, lower_start, lower_end)
         )
      }
   }

   #[inline]
   fn h_split_poly_edges(&mut self, poly: &PolyRef, y: i64, y_px: i64) {
      let p1_index;

      let end = poly.end;
      let mut i = poly.start;

      loop { // Edge's first point below y
         let edge = self.upper_edges[i];
         let edge_points = self.edge_points[edge.points];

         match edge.edge_type {
            EdgeType::TL | EdgeType::TR | EdgeType::VT => {
               let y2 = self.points[edge_points.p2].y;
               if y2 < y {
                  self.lower_edges.push(edge);
               } else if y2 > y {
                  let x1_intersect = self.h_intersection(&edge, &edge_points, y_px);

                  let (edge_points_i, point_index) = self.divide_edge_at(&edge, &edge_points, x1_intersect, y);
                  p1_index = point_index;

                  let lower_edge = edge.new_ref(edge_points_i);

                  self.lower_edges.push(lower_edge);

                  let upper_edge = edge.new_ref(edge_points_i + 1);

                  self.upper_edges.push(upper_edge);

                  break;
               } else {
                  p1_index = edge_points.p2;

                  self.lower_edges.push(edge);

                  break;
               }
            },
            _ => {
               self.lower_edges.push(edge);
            },
         }

         i += 1;

         if i == end {
            return;
         }
      }

      i += 1;

      loop { // Edge's first point above y
         let edge = self.upper_edges[i];
         let edge_points = self.edge_points[edge.points];

         match edge.edge_type {
            EdgeType::BL | EdgeType::BR | EdgeType::VB => {
               let y2 = self.points[edge_points.p1].y;
               if y2 > y {
                  self.upper_edges.push(edge);
               } else if y2 < y {
                  let x2_intersect = self.h_intersection(&edge, &edge_points, y_px);

                  let (edge_points_i, p2_index) = self.divide_edge_at(&edge, &edge_points, x2_intersect, y);

                  self.edge_points.push(
                     EdgePoints::new(p1_index, p2_index)
                  );

                  let upper_edge = edge.new_ref(edge_points_i);

                  self.upper_edges.push(upper_edge);

                  self.add_h_split_h_edge_ref(edge_points_i + 2);

                  let lower_edge = edge.new_ref(edge_points_i + 1);

                  self.lower_edges.push(lower_edge);

                  break;
               } else {
                  let p2_index = edge_points.p1;

                  self.upper_edges.push(edge);

                  let edge_points_i = self.edge_points.end();

                  self.edge_points.push(
                     EdgePoints::new(p1_index, p2_index)
                  );

                  self.add_h_split_h_edge_ref(edge_points_i);

                  break;
               }
            },
            _ => {
               self.upper_edges.push(edge);
            }
         }

         i += 1;

         if i == end {
            return;
         }
      }

      i += 1;

      for j in i..end { // Edge's first point again below y
         let edge = self.upper_edges[j];
         self.lower_edges.push(edge);
      }
   }

   #[inline]
   fn add_h_split_h_edge_ref(&mut self, edge_points_i: usize) {
      self.upper_edges.push(EdgeRef::new(
         EdgeType::HL,
         usize::MAX,
         edge_points_i
      ));

      self.lower_edges.push(EdgeRef::new(
         EdgeType::HR,
         usize::MAX,
         edge_points_i
      ));
   }

   #[inline]
   fn add_v_split_v_edge_ref(&mut self, edge_points_i: usize) {
      self.lower_edges.push(EdgeRef::new(
         EdgeType::VT,
         usize::MAX,
         edge_points_i
      ));

      self.active_edges.push(EdgeRef::new(
         EdgeType::VB,
         usize::MAX,
         edge_points_i
      ));
   }

   #[inline]
   fn divide_edge_at(&mut self, edge: &EdgeRef, edge_points: &EdgePoints, x: i64, y: i64) -> (usize, usize) {
      let point_index = self.points.end();

      self.points.push(
         Point::new(x, y)
      );

      let edge_points_index = self.edge_points.end();

      let first = EdgePoints::new(edge_points.p1, point_index);
      let second = EdgePoints::new(point_index, edge_points.p2);

      match edge.edge_type {
         EdgeType::TL | EdgeType::TR | EdgeType::VT | EdgeType::HR => {
            self.edge_points.push(first);
            self.edge_points.push(second);
         },
         _ => {
            self.edge_points.push(second);
            self.edge_points.push(first);
         }
      }

      (edge_points_index, point_index)
   }

   #[inline]
   fn h_intersection(&self, edge: &EdgeRef, edge_points: &EdgePoints, y_px: i64) -> i64 {
      let y = y_px * DIV_PER_PIXEL;

      let p1 = self.points[edge_points.p1];
      let p2 = self.points[edge_points.p2];

      let min_y = min(p1.y, p2.y);
      let max_y = max(p1.y, p2.y);

      if y <= min_y || y >= max_y {
         println!("");
         println!("BAD INTERSECTION");
         self.print_edge_ref(edge);
      }

      match edge.edge_type {
         EdgeType::VT | EdgeType::VB => {
            self.points[edge_points.p1].x
         },
         _ => {
            let h_ref = self.h_intersect_ref[edge.src_points];

            self.h_intersections[
               h_ref.start + (y_px - h_ref.start_px) as usize
            ]
         }
      }
   }

   #[inline]
   fn v_intersection(&self, edge: &EdgeRef, edge_points: &EdgePoints, x_px: i64) -> i64 {
      let x = x_px * DIV_PER_PIXEL;

      let p1 = self.points[edge_points.p1];
      let p2 = self.points[edge_points.p2];

      let min_x = min(p1.x, p2.x);
      let max_x = max(p1.x, p2.x);

      if x <= min_x || x >= max_x {
         println!("");
         println!("BAD INTERSECTION");
         self.print_edge_ref(edge);
      }

      match edge.edge_type {
         EdgeType::HR | EdgeType::HL => {
            self.points[edge_points.p1].y
         },
         _ => {
            let v_ref = self.v_intersect_ref[edge.src_points];

            self.v_intersections[
               v_ref.start + (x_px - v_ref.start_px) as usize
            ]
         }
      }
   }

   fn intersect_edges(&mut self) {
      for edge in &self.src.edges {
         match edge.edge_type {
            EdgeType::TR | EdgeType::TL | EdgeType::BR | EdgeType::BL => {
               let mut h_ref = &mut self.h_intersect_ref[edge.points];

               if h_ref.start != usize::MAX {
                  continue;
               }

               let mut v_ref = &mut self.v_intersect_ref[edge.points];

               let edge_points = &self.src.edge_points[edge.points];

               let p1 = self.scaled_points[edge_points.p1];
               let p2 = self.scaled_points[edge_points.p2];

               h_ref.start = self.h_intersections.end();
               v_ref.start = self.v_intersections.end();

               if p1.y == p2.y || p1.x == p2.x {
                  panic!("{:?} - {:?}, {:?}", edge, p1, p2);
               }

               h_ref.start_px = h_multi_intersect_fast(
                  p1, p2, DIV_PER_PIXEL, &mut self.h_intersections
               ) / DIV_PER_PIXEL;

               v_ref.start_px = v_multi_intersect_fast(
                  p1, p2, DIV_PER_PIXEL, &mut self.v_intersections
               ) / DIV_PER_PIXEL;

               h_ref.end = self.h_intersections.end();
               v_ref.end = self.v_intersections.end();
            },
            _ => {}
         }
      }
   }

   fn recalc_lower_min_max_x(&mut self) {
      for poly in self.lower_polys[..].iter() {
         let mut min_x = i64::MAX;
         let mut max_x = i64::MIN;

         for edge_index in poly.start..poly.end {
            let edge_points = self.edge_points[self.lower_edges[edge_index].points];

            let x1 = self.points[edge_points.p1].x;
            let x2 = self.points[edge_points.p2].x;

            let (left, right) = if x1 < x2 {
               (x1, x2)
            } else {
               (x2, x1)
            };

            if left < min_x {
               min_x = left;
            }

            if right > max_x {
               max_x = right;
            }
         }

         self.lower_min_x[poly.src] = min_x;
         self.lower_max_x[poly.src] = max_x;
      }
   }

   #[inline]
   fn v_split(&mut self, x: i64, x_px: i64) {
      self.active_polys.clear();
      self.active_edges.clear();

      let start = self.lower_polys.start();
      let end = self.lower_polys.end();
      for i in start..end {
         let poly = self.lower_polys[i];
         if self.lower_max_x[poly.src] <= x {

            let active_start = self.active_edges.end();

            for edge_i in poly.start..poly.end {
               self.active_edges.push(
                  self.lower_edges[edge_i].clone()
               );
            }

            let active_end = self.active_edges.end();

            self.active_polys.push(
               PolyRef::new(poly.src, active_start, active_end)
            );

         } else if self.lower_min_x[poly.src] >= x {
            self.lower_polys.push(poly);
         } else {
            self.v_split_poly(i, x, x_px);
         }
      }

      self.lower_polys.consume_at(end);
   }

   #[inline]
   fn v_split_poly(&mut self, poly_i: usize, x: i64, x_px: i64) {
      let poly = self.lower_polys[poly_i];

      let lower_start = self.lower_edges.end();
      let active_start = self.active_edges.end();

      self.v_split_poly_edges(&poly, x, x_px);

      let lower_end = self.lower_edges.end();
      let active_end = self.active_edges.end();

      if lower_end > lower_start {
         self.lower_polys.push(
            PolyRef::new(poly.src, lower_start, lower_end)
         );
         self.lower_min_x[poly.src] = x;
      }

      if active_end > active_start {
         self.active_polys.push(
            PolyRef::new(poly.src, active_start, active_end)
         );
      }
   }

   #[inline]
   fn check_edge_bounds(&self, edge: &EdgeRef) -> bool {
      if edge.src_points != usize::MAX {
         let edge_points = self.edge_points[edge.points];
         let src_points = self.src.edge_points[edge.src_points];
         let p1 = self.points[edge_points.p1];
         let p2 = self.points[edge_points.p2];
         let sp1 = self.scaled_points[src_points.p1];
         let sp2 = self.scaled_points[src_points.p2];
         let min_x = min(sp1.x, sp2.x);
         let max_x = max(sp1.x, sp2.x);
         let min_y = min(sp1.y, sp2.y);
         let max_y = max(sp1.y, sp2.y);

         if p1.x < min_x || p2.x < min_x || p1.x > max_x || p2.x > max_x ||
               p1.y < min_y || p2.y < min_y || p1.y > max_y || p2.y > max_y {
            return false;
         }
      }

      true
   }

   #[inline]
   fn v_split_poly_edges(&mut self, poly: &PolyRef, x: i64, x_px: i64) {
      // self.print_poly_ref(&poly, &self.lower_edges);

      let p1_index;

      let end = poly.end;
      let mut i = poly.start;

      loop { // Edge's first point below y
         let edge = self.lower_edges[i];
         let edge_points = self.edge_points[edge.points];

         match edge.edge_type {
            EdgeType::TR | EdgeType::BR | EdgeType::HR => {
               let x2 = self.edge_x2(&edge, &edge_points);
               if x2 < x {
                  self.active_edges.push(edge);
               } else if x2 > x {
                  let y1_intersect = self.v_intersection(&edge, &edge_points, x_px);

                  let (edge_points_i, point_index) = self.divide_edge_at(&edge, &edge_points, x, y1_intersect);
                  p1_index = point_index;

                  let active_edge = edge.new_ref(edge_points_i);

                  self.active_edges.push(active_edge);

                  let lower_edge = edge.new_ref(edge_points_i + 1);

                  self.lower_edges.push(lower_edge);

                  break;
               } else {
                  p1_index = self.edge_p2(&edge, &edge_points);

                  self.active_edges.push(edge);

                  break;
               }
            },
            _ => {
               self.active_edges.push(edge);
            },
         }

         i += 1;

         if i == end {
            return;
         }
      }

      i += 1;

      loop { // Edge's first point above y
         let edge = self.lower_edges[i];
         let edge_points = self.edge_points[edge.points];

         match edge.edge_type {
            EdgeType::BL | EdgeType::TL | EdgeType::HL => {
               let x2 = self.edge_x2(&edge, &edge_points);
               if x2 > x {
                  self.lower_edges.push(edge);
               } else if x2 < x {
                  let y2_intersect = self.v_intersection(&edge, &edge_points, x_px);

                  let (edge_points_i, p2_index) = self.divide_edge_at(&edge, &edge_points, x, y2_intersect);

                  self.push_edge_points(p1_index, p2_index);

                  let lower_edge = edge.new_ref(edge_points_i);

                  self.lower_edges.push(lower_edge);

                  self.add_v_split_v_edge_ref(edge_points_i + 2);

                  let active_edge = edge.new_ref(edge_points_i + 1);

                  self.active_edges.push(active_edge);

                  break;
               } else {
                  let p2_index = self.edge_p2(&edge, &edge_points);

                  self.lower_edges.push(edge);

                  let edge_points_i = self.edge_points.end();

                  self.push_edge_points(p1_index, p2_index);

                  self.add_v_split_v_edge_ref(edge_points_i);

                  break;
               }
            },
            _ => {
               self.lower_edges.push(edge);
            }
         }

         i += 1;

         if i == end {
            return;
         }
      }

      i += 1;

      for j in i..end { // Edge's first point again below y
         let edge = self.lower_edges[j];
         self.active_edges.push(edge);
      }
   }

   #[inline]
   fn edge_x1(&self, edge: &EdgeRef, edge_points: &EdgePoints) -> i64 {
      self.points[self.edge_p1(edge, edge_points)].x
   }

   #[inline]
   fn edge_x2(&self, edge: &EdgeRef, edge_points: &EdgePoints) -> i64 {
      self.points[self.edge_p2(edge, edge_points)].x
   }

   #[inline]
   fn edge_p1(&self, edge: &EdgeRef, edge_points: &EdgePoints) -> usize {
      match edge.edge_type {
         EdgeType::TL | EdgeType::TR | EdgeType::VT | EdgeType::HR => {
            edge_points.p1
         },
         _ => {
            edge_points.p2
         }
      }
   }

   #[inline]
   fn edge_p2(&self, edge: &EdgeRef, edge_points: &EdgePoints) -> usize {
      match edge.edge_type {
         EdgeType::TL | EdgeType::TR | EdgeType::VT | EdgeType::HR => {
            edge_points.p2
         },
         _ => {
            edge_points.p1
         }
      }
   }

   #[inline]
   fn check_equal_points(&self, name: &str, edge_points_i: usize, poly: &PolyRef, x: i64, x_px: i64) {
      let edge_points = self.edge_points[edge_points_i];
      if self.points[edge_points.p1] == self.points[edge_points.p2] {
         println!("");
         println!("{} EQUAL POINTS {:?} {} {}", name, self.points[edge_points.p1], x, x_px);
         self.print_poly_ref(poly, &self.lower_edges);
         panic!("EXIT");
      }
   }

   #[inline]
   fn push_edge_points(&mut self, p1_index: usize, p2_index: usize) {
      let p1 = self.points[p1_index];
      let p2 = self.points[p2_index];

      self.edge_points.push(
         if p2.y > p1.y || (p2.y == p1.y && p2.x > p1.x) {
            EdgePoints::new(p1_index, p2_index)
         } else {
            EdgePoints::new(p2_index, p1_index)
         }
      );
   }

   #[inline]
   fn advance(&mut self, x: i64, y: i64) -> Option<(i64, usize)> {
      if self.lower_polys.len() == 0 {
         // println!("NO POLYS");
         return None;
      }

      let first = self.lower_polys.start();
      let end = self.lower_polys.end();
      let mut min_x = i64::MAX;
      let mut min_i = 0;

      for i in first..end {
         let poly = self.lower_polys[i];
         let i_x = self.lower_min_x[poly.src];
         if min_x > i_x {
            min_x = i_x;
            min_i = i;
         }
      }

      let poly = self.lower_polys[min_i];

      // println!("");
      // println!("=== ADVANCE {}, {}", x, y);
      // self.print_poly_ref(&poly, &self.lower_edges);

      let top = self.lower_edges[poly.start];
      if top.edge_type != EdgeType::HR {
         // println!("TOP NOT HORIZONTAL");
         return None;
      }

      let bottom = self.lower_edges[poly.end - 2];
      if bottom.edge_type != EdgeType::HL {
         // println!("BOTTOM NOT HORIZONTAL REV");
         return None;
      }

      let side = self.lower_edges[poly.end - 1];
      if side.edge_type != EdgeType::VT {
         // println!("SIDE NOT VERTICAL");
         return None;
      }

      let side_points = self.edge_points[side.points];
      let p1 = self.points[side_points.p1];
      let p2 = self.points[side_points.p2];

      if p1.x != x || p2.x != x || p1.y != y || p2.y != y + DIV_PER_PIXEL {
         // println!("SIDE NOT GOOD VERTICAL");
         return None;
      }

      let top_points = self.edge_points[top.points];
      let top_x = self.points[top_points.p2].x;

      let bottom_points = self.edge_points[bottom.points];
      let bottom_x = self.points[bottom_points.p2].x;

      let dx = (min(top_x, bottom_x) - x) / DIV_PER_PIXEL;

      // println!("ADVANCED - {}", dx);

      Some((dx, poly.src))
   }

   #[inline]
   pub fn active_color(&self) -> RGB {
      let mut r: i64 = 0;
      let mut g: i64 = 0;
      let mut b: i64 = 0;

      let mut total_area: i64 = 0;

      let start = self.active_polys.start();
      let end = self.active_polys.end();

      for i in start..(end-1) {
         let poly = self.src.polys[self.active_polys[i].src];
         let area = self.double_area(i);
         r += (poly.color.r as i64) * area;
         g += (poly.color.g as i64) * area;
         b += (poly.color.b as i64) * area;
         total_area += area;
      }

      let poly = self.src.polys[self.active_polys[end-1].src];
      let area = DOUBLE_PIXEL_AREA - total_area;
      r += (poly.color.r as i64) * area;
      g += (poly.color.g as i64) * area;
      b += (poly.color.b as i64) * area;

      r /= DOUBLE_PIXEL_AREA;
      g /= DOUBLE_PIXEL_AREA;
      b /= DOUBLE_PIXEL_AREA;

      RGB::new(r as u8, g as u8, b as u8)
   }

   #[inline]
   pub fn double_area(&self, poly_i: usize) -> i64 {
      let mut area = 0;

      let poly = self.active_polys[poly_i];

      for i in poly.start..poly.end {
         let edge = self.active_edges[i];
         let edge_points = self.edge_points[edge.points];

         let p1 = self.points[edge_points.p1];
         let p2 = self.points[edge_points.p2];

         match edge.edge_type {
            EdgeType::TR | EdgeType::TL => {
               area += p2.x * p1.y - p1.x * p2.y;
            },
            EdgeType::BR | EdgeType::BL => {
               area += p1.x * p2.y - p2.x * p1.y;
            },
            EdgeType::HR => {
               area += (p2.x - p1.x) * p1.y;
            },
            EdgeType::HL => {
               area += (p1.x - p2.x) * p1.y;
            },
            EdgeType::VT => {
               area += (p1.y - p2.y) * p1.x;
            },
            EdgeType::VB => {
               area += (p2.y - p1.y) * p1.x;
            },
         }
      }

      assert!(area >= 0);

      area
   }

   #[inline]
   pub fn scale_src_points(&mut self, frame: &Frame) {
      let scale_x = DIV_PER_PIXEL * 10;//frame.width as i64 / 44;
      let scale_y = DIV_PER_PIXEL * 10;//frame.height as i64 / 28;

      for i in 0..self.scaled_points.len() {
         let point = self.src.points[i];
         let point = Point::new(point.x + 22, point.y + 14);
         let dest = &mut self.scaled_points[i];
         dest.x = point.x * scale_x;
         dest.y = point.y * scale_y;
      }
   }

   fn print_edge_ref(&self, edge: &EdgeRef) {
      let edge_points = self.edge_points[edge.points];
      if edge.src_points == usize::MAX {
         let p1;
         let p2;

         if edge.edge_type == EdgeType::TL ||
            edge.edge_type == EdgeType::TR ||
            edge.edge_type == EdgeType::VT ||
            edge.edge_type == EdgeType::HR
         {
            p1 = self.points[edge_points.p1];
            p2 = self.points[edge_points.p2];
         } else {
            p1 = self.points[edge_points.p2];
            p2 = self.points[edge_points.p1];
         }

         println!("E {:?} : ({}, {}) -> ({}, {})", edge.edge_type,
            p1.x, p1.y, p2.x, p2.y);

      } else {
         let src_points = self.src.edge_points[edge.src_points];

         let p1;
         let p2;

         let sp1;
         let sp2;

         if edge.edge_type == EdgeType::TL ||
            edge.edge_type == EdgeType::TR ||
            edge.edge_type == EdgeType::VT ||
            edge.edge_type == EdgeType::HR
         {
            p1 = self.points[edge_points.p1];
            p2 = self.points[edge_points.p2];
            sp1 = self.scaled_points[src_points.p1];
            sp2 = self.scaled_points[src_points.p2];
         } else {
            p1 = self.points[edge_points.p2];
            p2 = self.points[edge_points.p1];
            sp1 = self.scaled_points[src_points.p2];
            sp2 = self.scaled_points[src_points.p1];
         }

         println!("E {:?} : ({}, {}) -> ({}, {}); ({}, {}) -> ({}, {})", edge.edge_type,
            p1.x, p1.y, p2.x, p2.y,
            sp1.x, sp1.y, sp2.x, sp2.y);
      }
   }

   fn print_poly_ref(&self, poly: &PolyRef, edges: &Ring<EdgeRef>) {
      println!("P src: {} size: {}", poly.src, poly.end - poly.start);

      for i in poly.start..poly.end {
         self.print_edge_ref(&edges[i]);
      }
   }
}

#[inline]
fn print_ring<T>(name: &str, ring: &Ring<T>) where T: Default + Clone + Debug {
   println!("{}: s {} e {} :", name, ring.start(), ring.end());

   for item in ring[..].iter() {
      println!("{:?}", item);
   }
}

impl Renderer for TriangleRenderer {
   fn render(&mut self, frame: &mut Frame) {
      frame.clear();

      self.clear();

      self.scale_src_points(frame);

      self.calc_polys_min_y();
      self.calc_polys_min_max();

      self.intersect_edges();

      let (min_x, min_y, max_x, max_y) = self.min_max_x_y();

      let min_x = max(to_px(min_x), 0);
      let min_y = max(to_px(min_y), 0);
      let max_x = min(to_px(max_x), frame.width as i64 - 1);
      let max_y = min(to_px(max_y), frame.height as i64 - 1);

      for y in min_y..max_y {
         let y_world = from_px(y);
         let y_split = y_world + DIV_PER_PIXEL;

         self.transfer(y_world);

         self.h_split(y_split, y + 1);

         self.recalc_lower_min_max_x();

         let mut x = min_x;

         while x < max_x {
            let mut x_world = from_px(x);
            let mut x_split = x_world + DIV_PER_PIXEL;

            match self.advance(x_world, y_world) {
               Some((dx, poly_i)) => {
                  let src_poly = self.src.polys[poly_i];

                  for ix in x..x+dx {
                     frame.put_pixel(ix as i32, y as i32, &src_poly.color);
                  }

                  x += dx;
                  x_world = from_px(x);
                  x_split = x_world + DIV_PER_PIXEL;

                  self.v_split(x_world, x);
               },
               None => {}
            }

            if x < max_x {
               self.v_split(x_split, x + 1);

               assert!(self.active_polys.len() > 0);

               let color = self.active_color();

               frame.put_pixel(x as i32, y as i32, &color);
            }

            x += 1;
         }
      }
   }
}

fn main() {
   let mut renderer = TriangleRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Triangle3")
      .run();
}
