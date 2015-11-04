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
   Inclined,
   InclinedRev,
   Horizontal,
   HorizontalRev,
   Vertical,
   VerticalRev,
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
   pub fn inclined(points: usize) -> Self {
      Edge::new(EdgeType::Inclined, points)
   }

   #[inline]
   pub fn inclined_rev(points: usize) -> Self {
      Edge::new(EdgeType::InclinedRev, points)
   }

   #[inline]
   pub fn horizontal(points: usize) -> Self {
      Edge::new(EdgeType::Horizontal, points)
   }

   #[inline]
   pub fn horizontal_rev(points: usize) -> Self {
      Edge::new(EdgeType::HorizontalRev, points)
   }

   #[inline]
   pub fn vertical(points: usize) -> Self {
      Edge::new(EdgeType::Vertical, points)
   }

   #[inline]
   pub fn vertical_rev(points: usize) -> Self {
      Edge::new(EdgeType::VerticalRev, points)
   }
}

impl Default for Edge {
   #[inline]
   fn default() -> Edge {
      Edge::inclined(0)
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
   pub fn new_ref(&self, i: usize) -> Self {
      EdgeRef::new(self.edge_type, self.src_points, i)
   }
}

impl Default for EdgeRef {
   #[inline]
   fn default() -> EdgeRef {
      EdgeRef::new(EdgeType::Inclined, 0, 0)
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
         Poly::new(RGB::new(243, 0, 149), 9, 12),
         // E
         Poly::new(RGB::new(170, 36, 14), 12, 15),
         // F
         Poly::new(RGB::new(219, 65, 18), 15, 18),
         // G
         Poly::new(RGB::new(254, 185, 21), 18, 21),
         // H
         Poly::new(RGB::new(244, 239, 114), 21, 24),
         // I
         Poly::new(RGB::new(109, 233, 158), 24, 27),
         // J
         Poly::new(RGB::new(66, 222, 241), 27, 30),
      ];

      let edges = vec![
         // 0: A
         Edge::vertical(13),
         Edge::inclined_rev(0),
         Edge::inclined_rev(1),

         // 1: B
         Edge::inclined(1),
         Edge::inclined_rev(2),
         Edge::inclined_rev(3),

         // 2: C
         Edge::inclined(3),
         Edge::inclined_rev(4),
         Edge::horizontal_rev(14),

         // 3: D
         Edge::inclined(4),
         Edge::inclined(5),
         Edge::inclined_rev(6),

         // 4: E
         Edge::inclined(6),
         Edge::inclined(7),
         Edge::vertical_rev(15),

         // 5: F
         Edge::inclined(2),
         Edge::inclined(8),
         Edge::inclined_rev(9),

         // 6: G
         Edge::inclined(9),
         Edge::inclined_rev(10),
         Edge::inclined_rev(5),

         // 7: H
         Edge::inclined(0),
         Edge::inclined_rev(11),
         Edge::inclined_rev(8),

         // 8: I
         Edge::inclined(10),
         Edge::inclined(12),
         Edge::inclined_rev(7),

         // 9: J
         Edge::inclined(11),
         Edge::horizontal(16),
         Edge::inclined_rev(12),
      ];

      let edge_points = vec![
         EdgePoints::new(2, 1),  // 0
         EdgePoints::new(0, 2),  // 1
         EdgePoints::new(3, 2),  // 2
         EdgePoints::new(0, 3),  // 3
         EdgePoints::new(4, 3),  // 4
         EdgePoints::new(3, 5),  // 5
         EdgePoints::new(4, 5),  // 6
         EdgePoints::new(5, 6),  // 7
         EdgePoints::new(2, 7),  // 8
         EdgePoints::new(3, 7),  // 9
         EdgePoints::new(5, 7),  // 10
         EdgePoints::new(7, 1),  // 11
         EdgePoints::new(7, 6),  // 12
         EdgePoints::new(0, 1),  // 13
         EdgePoints::new(0, 4),  // 14
         EdgePoints::new(4, 6),  // 15
         EdgePoints::new(1, 6),  // 16
      ];

      let mut points = vec![
         Point::new(0, 0),   // 0
         Point::new(0, 10),  // 1
         Point::new(2, 5),   // 2
         Point::new(3, 1),   // 3
         Point::new(10, 0),  // 4
         Point::new(8, 5),   // 5
         Point::new(10, 10), // 6
         Point::new(7, 9),   // 7
      ];

      for point in &mut points {
         *point *= 100 * DIV_PER_PIXEL;
      }

      PolySource {
         polys: polys,
         edges: edges,
         edge_points: edge_points,
         points: points,
      }
   }

   pub fn polys_min_y(&self) -> Vec<PolyMinRef> {
      let mut v = Vec::new();

      for (poly_index, poly) in self.polys.iter().enumerate() {
         let mut min_y = i64::MAX;

         for edge_index in poly.start..poly.end {
            let edge_points = self.edge_points[self.edges[edge_index].points];
            let y = min(
               self.points[edge_points.p1].y,
               self.points[edge_points.p2].y
            );

            if y < min_y {
               min_y = y;
            }
         }

         v.push(PolyMinRef::new(poly_index, min_y));
      }

      v.sort_by(|a, b| a.min.cmp(&b.min));

      v
   }

   pub fn polys_min_max(&self) -> Vec<MinMaxXY> {
      let mut v: Vec<MinMaxXY> = repeat(MinMaxXY::default()).take(self.polys.len()).collect();

      for (poly_index, poly) in self.polys.iter().enumerate() {
         let poly_min_max = &mut v[poly_index];

         for edge_index in poly.start..poly.end {
            let edge_points = self.edge_points[self.edges[edge_index].points];

            let p1 = self.points[edge_points.p1];
            let p2 = self.points[edge_points.p2];

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

      v
   }

   #[inline]
   fn min_max_x_y(&self) -> (i64, i64, i64, i64) {
      let mut min_x = i64::MAX;
      let mut min_y = i64::MAX;

      let mut max_x = i64::MIN;
      let mut max_y = i64::MIN;

      for p in &self.points {
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
}

struct TriangleRenderer {
   src: PolySource,
   src_min_y: Vec<PolyMinRef>,
   src_poly_marker: usize,
   src_min_max: Vec<MinMaxXY>,

   h_sorted: Vec<usize>,
   v_sorted: Vec<usize>,

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

   active_polys: Ring<PolyRef>,
   active_edges: Ring<EdgeRef>,

   edge_points: Ring<EdgePoints>,
   points: Ring<Point>,
}

impl TriangleRenderer {
   fn new() -> Self {
      let src = PolySource::new();
      let src_min_y = src.polys_min_y();

      let src_min_max = src.polys_min_max();

      let polys_len = src.polys.len();
      let edge_points_len = src.edge_points.len();
      let points_len = src.points.len();

      let h_sorted = (1..polys_len).collect();
      let v_sorted = (1..polys_len).collect();

      let edge_points_map = repeat(usize::MAX).take(edge_points_len).collect();
      let points_map = repeat(usize::MAX).take(points_len).collect();

      let h_intersect_ref = repeat(IntersectRef::default()).take(edge_points_len).collect();
      let v_intersect_ref = repeat(IntersectRef::default()).take(edge_points_len).collect();

      let lower_min_x = repeat(i64::MAX).take(polys_len).collect();

      TriangleRenderer {
         src: src,
         src_min_y: src_min_y,
         src_poly_marker: 0,
         src_min_max: src_min_max,

         h_sorted: h_sorted,
         v_sorted: v_sorted,

         edge_points_map: edge_points_map,
         points_map: points_map,

         h_intersect_ref: h_intersect_ref,
         v_intersect_ref: v_intersect_ref,
         h_intersections: Ring::new(65536),
         v_intersections: Ring::new(65536),

         upper_polys: Ring::new(65536),
         upper_edges: Ring::new(262144),

         lower_polys: Ring::new(65536),
         lower_edges: Ring::new(262144),
         lower_min_x: lower_min_x,

         active_polys: Ring::new(16384),
         active_edges: Ring::new(65536),

         edge_points: Ring::new(262144),
         points: Ring::new(262144),
      }
   }

   fn clear(&mut self) {
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

      self.upper_edges.push(
         EdgeRef::new(edge.edge_type, src_edge_points_i, edge_points_i)
      );
   }

   fn transfer_point(&mut self, src_i: usize) -> usize {
      let mut i = self.points_map[src_i];

      if i == usize::MAX {
         i = self.points.end();

         self.points_map[src_i] = i;

         self.points.push(
            self.src.points[src_i]
         );
      }

      i
   }

   #[inline]
   fn h_split(&mut self, y: i64, y_px: i64) {
      self.lower_polys.consume();

      let start = self.upper_polys.start();
      let end = self.upper_polys.end();
      for i in start..end {
         self.h_split_poly(i, y, y_px);
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
            EdgeType::Inclined => {
               let y2 = self.points[edge_points.p2].y;
               if y2 < y {
                  self.lower_edges.push(edge);
               } else if y2 > y {
                  let x1_intersect = self.h_intersection(&edge, y_px);

                  let (edge_points_i, point_index) = self.divide_edge_at(&edge_points, x1_intersect, y);
                  p1_index = point_index;

                  self.lower_edges.push(edge.new_ref(edge_points_i));

                  self.upper_edges.push(edge.new_ref(edge_points_i + 1));

                  break;
               } else {
                  p1_index = edge_points.p2;

                  self.lower_edges.push(edge);

                  break;
               }
            },
            EdgeType::Vertical => {
               let y2 = self.points[edge_points.p2].y;
               if y2 < y {
                  self.lower_edges.push(edge);
               } else if y2 > y {
                  let x1_intersect = self.points[edge_points.p2].x;

                  let (edge_points_i, point_index) = self.divide_edge_at(&edge_points, x1_intersect, y);
                  p1_index = point_index;

                  self.lower_edges.push(edge.new_ref(edge_points_i));

                  self.upper_edges.push(edge.new_ref(edge_points_i + 1));

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
            EdgeType::InclinedRev => {
               let y2 = self.points[edge_points.p1].y;
               if y2 > y {
                  self.upper_edges.push(edge);
               } else if y2 < y {
                  let x2_intersect = self.h_intersection(&edge, y_px);

                  let (edge_points_i, p2_index) = self.divide_edge_at(&edge_points, x2_intersect, y);

                  self.edge_points.push(
                     EdgePoints::new(p1_index, p2_index)
                  );

                  self.upper_edges.push(edge.new_ref(edge_points_i + 1));

                  self.add_h_split_h_edge_ref(edge_points_i + 2);

                  self.lower_edges.push(edge.new_ref(edge_points_i));

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
            EdgeType::VerticalRev => {
               let y2 = self.points[edge_points.p1].y;
               if y2 > y {
                  self.upper_edges.push(edge);
               } else if y2 < y {
                  let x2_intersect = self.points[edge_points.p1].x;

                  let (edge_points_i, p2_index) = self.divide_edge_at(&edge_points, x2_intersect, y);

                  self.edge_points.push(
                     EdgePoints::new(p1_index, p2_index)
                  );

                  self.upper_edges.push(edge.new_ref(edge_points_i + 1));

                  self.add_h_split_h_edge_ref(edge_points_i + 2);

                  self.lower_edges.push(edge.new_ref(edge_points_i));

                  break;
               } else {
                  let p2_index = edge_points.p1;

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
         EdgeType::HorizontalRev,
         usize::MAX,
         edge_points_i
      ));

      self.lower_edges.push(EdgeRef::new(
         EdgeType::Horizontal,
         usize::MAX,
         edge_points_i
      ));
   }

   #[inline]
   fn add_v_split_v_edge_ref(&mut self, edge_points_i: usize) {
      self.lower_edges.push(EdgeRef::new(
         EdgeType::Vertical,
         usize::MAX,
         edge_points_i
      ));

      self.active_edges.push(EdgeRef::new(
         EdgeType::VerticalRev,
         usize::MAX,
         edge_points_i
      ));
   }

   #[inline]
   fn divide_edge_at(&mut self, edge_points: &EdgePoints, x: i64, y: i64) -> (usize, usize) {
      let point_index = self.points.end();

      self.points.push(
         Point::new(x, y)
      );

      let edge_points_index = self.edge_points.end();

      self.edge_points.push(
         EdgePoints::new(edge_points.p1, point_index)
      );

      self.edge_points.push(
         EdgePoints::new(point_index, edge_points.p2)
      );

      (edge_points_index, point_index)
   }

   #[inline]
   fn h_intersection(&self, edge: &EdgeRef, y_px: i64) -> i64 {
      let h_ref = self.h_intersect_ref[edge.src_points];

      self.h_intersections[
         h_ref.start + (y_px - h_ref.start_px) as usize
      ]
   }

   #[inline]
   fn v_intersection(&self, edge: &EdgeRef, x_px: i64) -> i64 {
      let v_ref = self.v_intersect_ref[edge.src_points];

      self.v_intersections[
         v_ref.start + (x_px - v_ref.start_px) as usize
      ]
   }

   fn intersect_edges(&mut self) {
      for edge in &self.src.edges {
         if edge.edge_type == EdgeType::Inclined || edge.edge_type == EdgeType::InclinedRev {
            let mut h_ref = &mut self.h_intersect_ref[edge.points];

            if h_ref.start != usize::MAX {
               continue;
            }

            let mut v_ref = &mut self.v_intersect_ref[edge.points];

            let edge_points = &self.src.edge_points[edge.points];

            let p1 = self.src.points[edge_points.p1];
            let p2 = self.src.points[edge_points.p2];

            h_ref.start = self.h_intersections.end();
            v_ref.start = self.v_intersections.end();

            h_ref.start_px = h_multi_intersect_fast(
               p1, p2, DIV_PER_PIXEL, &mut self.h_intersections
            ) / DIV_PER_PIXEL;

            v_ref.start_px = v_multi_intersect_fast(
               p1, p2, DIV_PER_PIXEL, &mut self.v_intersections
            ) / DIV_PER_PIXEL;

            h_ref.end = self.h_intersections.end();
            v_ref.end = self.v_intersections.end();
         }
      }
   }

   fn recalc_lower_min_x(&mut self) {
      for poly in self.lower_polys[..].iter() {
         let mut min_x = i64::MAX;

         for edge_index in poly.start..poly.end {
            let edge_points = self.edge_points[self.lower_edges[edge_index].points];
            let x = min(
               self.points[edge_points.p1].x,
               self.points[edge_points.p2].x
            );

            if x < min_x {
               min_x = x;
            }
         }

         self.lower_min_x[poly.src] = min_x;
      }
   }

   #[inline]
   fn v_split(&mut self, x: i64, x_px: i64) {
      self.active_polys.consume();

      println!("");
      println!("== LOWER ==");

      for poly in self.lower_polys[..].iter() {
         self.print_poly_ref(&poly, &self.lower_edges);
      }

      println!("");
      println!("== SPLITTING ==");

      let start = self.lower_polys.start();
      let end = self.lower_polys.end();
      for i in start..end {
         let poly = self.lower_polys[i];
         if self.lower_min_x[poly.src] < x {
            self.v_split_poly(i, x, x_px);
         } else {
            self.lower_polys.push(poly);
         }
      }

      println!("");
      println!("== ACTIVE ==");

      for poly in self.active_polys[..].iter() {
         self.print_poly_ref(&poly, &self.active_edges);
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
   fn v_split_poly_edges(&mut self, poly: &PolyRef, x: i64, x_px: i64) {
      self.print_poly_ref(&poly, &self.lower_edges);

      let p1_index;

      let end = poly.end;
      let mut i = poly.start;

      loop { // Edge's first point below y
         let edge = self.lower_edges[i];
         let edge_points = self.edge_points[edge.points];

         match edge.edge_type {
            EdgeType::Inclined | EdgeType::InclinedRev => {
               let x2 = self.points[edge_points.p2].x;
               if x2 < x {
                  self.active_edges.push(edge);
               } else if x2 > x {
                  let y1_intersect = self.v_intersection(&edge, x_px);

                  let (edge_points_i, point_index) = self.divide_edge_at(&edge_points, x, y1_intersect);
                  p1_index = point_index;

                  self.active_edges.push(edge.new_ref(edge_points_i));

                  self.lower_edges.push(edge.new_ref(edge_points_i + 1));

                  break;
               } else {
                  p1_index = edge_points.p2;

                  self.active_edges.push(edge);

                  break;
               }
            },
            EdgeType::Horizontal | EdgeType::HorizontalRev => {
               let x2 = self.points[edge_points.p2].x;
               if x2 < x {
                  self.active_edges.push(edge);
               } else if x2 > x {
                  let y1_intersect = self.points[edge_points.p2].y;

                  let (edge_points_i, point_index) = self.divide_edge_at(&edge_points, x, y1_intersect);
                  p1_index = point_index;

                  self.active_edges.push(edge.new_ref(edge_points_i));

                  self.lower_edges.push(edge.new_ref(edge_points_i + 1));

                  break;
               } else {
                  p1_index = edge_points.p2;

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
            EdgeType::Inclined | EdgeType::InclinedRev => {
               let x2 = self.points[edge_points.p1].x;
               if x2 > x {
                  self.lower_edges.push(edge);
               } else if x2 < x {
                  let y2_intersect = self.v_intersection(&edge, x_px);

                  let (edge_points_i, p2_index) = self.divide_edge_at(&edge_points, x, y2_intersect);

                  self.push_edge_points(p1_index, p2_index);

                  self.lower_edges.push(edge.new_ref(edge_points_i + 1));

                  self.add_v_split_v_edge_ref(edge_points_i + 2);

                  self.active_edges.push(edge.new_ref(edge_points_i));

                  break;
               } else {
                  let p2_index = edge_points.p1;

                  self.lower_edges.push(edge);

                  let edge_points_i = self.edge_points.end();

                  self.push_edge_points(p1_index, p2_index);

                  self.add_v_split_v_edge_ref(edge_points_i);

                  break;
               }
            },
            EdgeType::Horizontal | EdgeType::HorizontalRev => {
               let x2 = self.points[edge_points.p1].x;
               if x2 > x {
                  self.lower_edges.push(edge);
               } else if x2 < x {
                  let y2_intersect = self.points[edge_points.p1].y;

                  let (edge_points_i, p2_index) = self.divide_edge_at(&edge_points, x, y2_intersect);

                  self.push_edge_points(p1_index, p2_index);

                  self.lower_edges.push(edge.new_ref(edge_points_i + 1));

                  self.add_v_split_v_edge_ref(edge_points_i + 2);

                  self.active_edges.push(edge.new_ref(edge_points_i));

                  break;
               } else {
                  let p2_index = edge_points.p1;

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

   fn print_edge_ref(&self, edge: &EdgeRef) {
      let edge_points = self.edge_points[edge.points];
      let p1;
      let p2;
      if edge.edge_type == EdgeType::Inclined ||
         edge.edge_type == EdgeType::Horizontal ||
         edge.edge_type == EdgeType::Vertical
      {
         p1 = self.points[edge_points.p1];
         p2 = self.points[edge_points.p2];
      } else {
         p1 = self.points[edge_points.p2];
         p2 = self.points[edge_points.p1];
      }

      println!("E {:?} : ({}, {}) -> ({}, {})", edge.edge_type, p1.x, p1.y, p2.x, p2.y)
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

      self.intersect_edges();

      let (min_x, min_y, max_x, max_y) = self.src.min_max_x_y();

      let min_x = max(to_px(min_x), 0);
      let min_y = max(to_px(min_y), 0);
      let max_x = min(to_px(max_x), frame.width as i64 - 1);
      let max_y = min(to_px(max_y), frame.height as i64 - 1);

      let back = RGB::new(1, 1, 1);

      for y in min_y..max_y + 1 {
         let y_world = from_px(y);
         let y_split = y_world + DIV_PER_PIXEL;

         self.transfer(y_world);

         self.h_split(y_split, y + 1);

         self.recalc_lower_min_x();

         for x in min_x..max_x + 1 {
            let x_world = from_px(x);
            let x_split = x_world + DIV_PER_PIXEL;

            self.v_split(x_split, x + 1);

            frame.put_pixel(x as i32, y as i32, &back);
         }

         panic!("COMPLETE");
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
