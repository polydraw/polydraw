use std::cmp::{PartialOrd, Ordering, min, max};
use std::iter::repeat;
use std::{usize, i64};

use frame::Frame;
use draw::RGB;

pub const HALF_MAX_ERR: i64  = i64::MAX / 2;

const DIV_PER_PIXEL: i64 = 1000;

#[inline]
fn to_px(v: i64) -> i64 {
   v / DIV_PER_PIXEL
}

#[inline]
fn from_px(v: i64) -> i64 {
   v as i64 * DIV_PER_PIXEL
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
   pub x: i64,
   pub y: i64,
}

impl Point {
   #[inline]
   pub fn new(x: i64, y: i64) -> Self {
      Point {
         x: x,
         y: y,
      }
   }
}

impl Default for Point {
   fn default() -> Point {
      Point::new(0, 0)
   }
}

impl PartialOrd for Point {
   fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
      Some(self.cmp(other))
   }
}

impl Ord for Point {
   fn cmp(&self, other: &Point) -> Ordering {
      if self.y < other.y {
         Ordering::Less
      } else if self.y > other.y {
         Ordering::Greater
      } else if self.x < other.x {
         Ordering::Less
      } else if self.x > other.x {
         Ordering::Greater
      } else {
         Ordering::Equal
      }
   }
}

#[derive(Debug, Clone, Copy)]
pub struct Segment {
   pub p1: usize,
   pub p2: usize,
}

impl Segment {
   #[inline]
   pub fn new(p1: usize, p2: usize) -> Self {
      Segment {
         p1: p1,
         p2: p2,
      }
   }
}

impl Default for Segment {
   fn default() -> Segment {
      Segment::new(0, 0)
   }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
   pub center: usize,
   pub radius: i64,
}

impl Circle {
   #[inline]
   pub fn new(center: usize, radius: i64) -> Self {
      Circle {
         center: center,
         radius: radius,
      }
   }
}

impl Default for Circle {
   fn default() -> Circle {
      Circle::new(0, 0)
   }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EdgeType {
   LTR,  // line top-right
   LTL,  // line top-left
   LBR,  // line bottom-right
   LBL,  // line bottom-left

   LHR,  // line horizontal-right
   LHL,  // line horizontal-left
   LVT,  // line vertical-top
   LVB,  // line vertical-bottom

   CTR, // clockwise arc top-right
   CTL, // clockwise arc top-left
   CBR, // clockwise arc bottom-right
   CBL, // clockwise arc bottom-left

   ATR, // anti-clockwise arc top-right
   ATL, // anti-clockwise arc top-left
   ABR, // anti-clockwise arc bottom-right
   ABL, // anti-clockwise arc bottom-left
}

impl EdgeType {
   #[inline]
   pub fn reversed(&self) -> bool {
      match *self {
         EdgeType::LBR | EdgeType::LBL | EdgeType::LHL | EdgeType::LVB |
         EdgeType::CBR | EdgeType::CBL | EdgeType::ABR | EdgeType::ABL => {
            true
         },
         _ => {
            false
         }
      }
   }
}

#[derive(Debug, Clone, Copy)]
pub struct EdgeSrc {
   pub edge_type: EdgeType,
   pub segment: usize,
   pub circle: usize,
}

impl EdgeSrc {
   #[inline]
   pub fn new(edge_type: EdgeType, segment: usize, circle: usize) -> Self {
      EdgeSrc {
         edge_type: edge_type,
         segment: segment,
         circle: circle,
      }
   }

   #[inline]
   pub fn reversed(&self) -> bool {
      self.edge_type.reversed()
   }
}

impl Default for EdgeSrc {
   fn default() -> EdgeSrc {
      EdgeSrc::new(EdgeType::LTR, 0, 0)
   }
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
   pub edge_type: EdgeType,
   pub segment: usize,
   pub p1: Point,
   pub p2: Point,
}

impl Edge {
   #[inline]
   pub fn new(edge_type: EdgeType, segment: usize, p1: Point, p2: Point) -> Self {
      Edge {
         edge_type: edge_type,
         segment: segment,
         p1: p1,
         p2: p2,
      }
   }

   #[inline]
   pub fn reversed(&self) -> bool {
      self.edge_type.reversed()
   }
}

impl Default for Edge {
   fn default() -> Edge {
      Edge::new(EdgeType::LTR, 0, Point::default(), Point::default())
   }
}

#[derive(Debug, Clone)]
pub struct Poly {
   pub start: usize,
   pub end: usize,
   pub color: usize,
}

impl Poly {
   #[inline]
   pub fn new(start: usize, end: usize, color: usize) -> Self {
      Poly {
         start: start,
         end: end,
         color: color,
      }
   }
}

impl Default for Poly {
   fn default() -> Poly {
      Poly::new(0, 0, 0)
   }
}

pub struct Scene {
   pub points: Vec<Point>,
   pub segments: Vec<Segment>,
   pub circles: Vec<Circle>,
   pub edges: Vec<EdgeSrc>,
   pub polys: Vec<Poly>,
   pub colors: Vec<RGB>,
}

#[derive(Debug, Clone, Copy)]
pub struct PolyRef {
   pub start: usize,
   pub end: usize,
   pub src: usize,
}

impl PolyRef {
   #[inline]
   pub fn new(start: usize, end: usize, src: usize) -> Self {
      PolyRef {
         start: start,
         end: end,
         src: src,
      }
   }
}

impl Default for PolyRef {
   fn default() -> PolyRef {
      PolyRef::new(0, 0, 0)
   }
}

#[derive(Debug, Clone)]
pub struct IntersectionRef {
   first_px: i64,
   start: usize,
   end: usize,
}

impl IntersectionRef {
   #[inline]
   pub fn new(first_px: i64, start: usize, end: usize) -> Self {
      IntersectionRef {
         first_px: first_px,
         start: start,
         end: end,
      }
   }
}

impl Default for IntersectionRef {
   #[inline]
   fn default() -> IntersectionRef {
      IntersectionRef::new(0, 0, 0)
   }
}

pub struct Rasterizer {
   pub vert_intersections_ref: Vec<IntersectionRef>,
   pub hori_intersections_ref: Vec<IntersectionRef>,
   pub vert_intersections: Vec<i64>,
   pub hori_intersections: Vec<i64>,

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

      let lower_edges = create_default_vec(65536);
      let lower_edges_len = create_default_vec(65536);

      let upper_min_y = create_default_vec(65536);
      let upper_max_y = create_default_vec(65536);
      let upper_active = create_default_vec(65536);

      Rasterizer {
         vert_intersections_ref: vert_intersections_ref,
         hori_intersections_ref: hori_intersections_ref,
         vert_intersections: vert_intersections,
         hori_intersections: hori_intersections,

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
      }
   }

   #[allow(unused_variables)]
   pub fn render(&mut self, scene: &Scene, frame: &mut Frame) {
      self.transfer_scene(scene);

      self.check_pool(scene, &self.upper_edges, &self.upper_edges_len);

      self.intersect_edges(scene);

      self.check_intersections(scene);

      let (min_x, min_y, max_x, max_y) = self.min_max_x_y(scene);

      self.check_min_max_x_y(min_x, min_y, max_x, max_y);

      self.calc_poly_min_max_y(scene);

      self.check_poly_min_max_y(scene, min_y, max_y);

      let x_start = to_px(min_x);
      let x_end = to_px(max_x - 1) + 1;
      let y_start = to_px(min_y);
      let y_end = to_px(max_y - 1) + 1;

      for y in y_start..y_end {
         let y_world = from_px(y);
         let y_split = y_world + DIV_PER_PIXEL;

         self.h_split(y_split, y + 1);

         let mut x = x_start;

         while x < x_end {
            let x_world = from_px(x);
            let x_split = x_world + DIV_PER_PIXEL;

            x += 1;
         }

         panic!("END");
      }
   }

   pub fn transfer_scene(&mut self, scene: &Scene) {
      let mut pool_index = 0;
      for i in 0..scene.polys.len() {
         let ref poly = &scene.polys[i];
         self.poly_to_pool[i] = pool_index;
         self.upper_edges_len[i] = poly.end - poly.start;

         for edge in &scene.edges[poly.start..poly.end] {
            let ref mut edge_ref = self.upper_edges[pool_index];
            edge_ref.segment = edge.segment;
            edge_ref.edge_type = edge.edge_type;

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

         // Leave 4 position for further hori and vert splitting
         pool_index += 4;
      }

      self.upper_active_end = scene.polys.len();
   }

   fn check_pool(&self, scene: &Scene, pool: &Vec<Edge>, pool_lens: &Vec<usize>) {
      for poly_index in 0..scene.polys.len() {
         let edge_start = self.poly_to_pool[poly_index];
         let poly_len = pool_lens[poly_index];

         if poly_len < 3 {
            panic!("Insufficient edge count: {}", poly_len);
         }

         let mut p2_prev = pool[edge_start + poly_len - 1].p2;
         for edge_index in edge_start..edge_start + poly_len {
            let edge = pool[edge_index];

            if edge.edge_type.reversed() != (edge.p1 > edge.p2) {
               panic!("Wrong edge points ordering");
            }

            if edge.p1 != p2_prev {
               panic!("Unconnected poly");
            }

            p2_prev = edge.p2;
         }
      }
   }

   fn intersect_edges(&mut self, scene: &Scene) {
      self.reset_intersections(scene);

      let mut vert_prev_end = 0;
      let mut hori_prev_end = 0;

      for edge in &scene.edges {
         let i = edge.segment;

         let ref mut vert_ref = self.vert_intersections_ref[i];
         if vert_ref.start == usize::MAX {
            continue;
         }

         let ref mut hori_ref = self.hori_intersections_ref[i];

         let ref segment = scene.segments[i];
         let ref p1 = scene.points[segment.p1];
         let ref p2 = scene.points[segment.p2];

         vert_ref.start = vert_prev_end;
         hori_ref.start = hori_prev_end;

         let (vert_end, x_first_px) = v_multi_intersect_fast(
            p1, p2, DIV_PER_PIXEL, vert_ref.start, &mut self.vert_intersections
         );

         let (hori_end, y_first_px) = h_multi_intersect_fast(
            p1, p2, DIV_PER_PIXEL, hori_ref.start, &mut self.hori_intersections
         );

         vert_prev_end = vert_end;
         vert_ref.end = vert_end;
         vert_ref.first_px = x_first_px;

         hori_prev_end = hori_end;
         hori_ref.end = hori_end;
         hori_ref.first_px = y_first_px;
      }
   }

   fn reset_intersections(&mut self, scene: &Scene) {
      for i in 0..scene.segments.len() {
         self.vert_intersections_ref[i].start = usize::MAX;
      }
   }

   fn check_intersections(&self, scene: &Scene) {
      for edge in &scene.edges {
         let ref segment = scene.segments[edge.segment];
         let ref p1 = scene.points[segment.p1];
         let ref p2 = scene.points[segment.p2];

         let min_x = min(p1.x, p2.x);
         let max_x = max(p1.x, p2.x);
         let min_y = min(p1.y, p2.y);
         let max_y = max(p1.y, p2.y);

         let ref vert_ref = self.vert_intersections_ref[edge.segment];

         let mut prev_x = i64::MIN;
         for i in vert_ref.start..vert_ref.end {
            let x = self.vert_intersections[i];
            assert!(min_x <= x);
            assert!(max_x >= x);
            assert!(prev_x < x);
            prev_x = x;
         }

         let ref hori_ref = self.hori_intersections_ref[edge.segment];

         let mut prev_y = i64::MIN;
         for i in hori_ref.start..hori_ref.end {
            let y = self.hori_intersections[i];
            assert!(min_y <= y);
            assert!(max_y >= y);
            assert!(prev_y < y);
            prev_y = y;
         }
      }
   }

   fn calc_poly_min_max_y(&mut self, scene: &Scene) {
      for i in 0..scene.polys.len() {
         let poly_start = self.poly_to_pool[i];
         let poly_end = poly_start + self.upper_edges_len[i];

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

         self.upper_min_y[i] = poly_min_y;
         self.upper_max_y[i] = poly_max_y;
         self.upper_active[i] = i;
      }

      self.sort_upper_min_y(scene);
   }

   fn sort_upper_min_y(&mut self, scene: &Scene) {
      let upper_active = &mut self.upper_active;
      let upper_min_y = &self.upper_min_y;
      let upper_max_y = &self.upper_max_y;

      upper_active[..scene.polys.len()].sort_by(|a, b| {
         match upper_min_y[*a].cmp(&upper_min_y[*b]) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => upper_max_y[*a].cmp(&upper_max_y[*b])
         }
      });
   }

   fn check_poly_min_max_y(&self, scene: &Scene, all_min_y: i64, all_max_y: i64) {
      let polys_end = scene.polys.len();
      let mut prev_min_y = i64::MIN;
      for i in 0..polys_end {
         let poly_i = self.upper_active[i];

         let min_y = self.upper_min_y[poly_i];
         let max_y = self.upper_max_y[poly_i];

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

   fn h_split(&mut self, y: i64, y_px: i64) {
      for i in self.upper_active_start .. self.upper_active_end {
         let poly_index = self.upper_active[i];

         if self.upper_max_y[poly_index] <= y {
            let poly_start = self.poly_to_pool[poly_index];
            let poly_len = self.upper_edges_len[poly_index];

            for edge_i in poly_start..poly_start + poly_len {
               self.lower_edges[edge_i] = self.upper_edges[edge_i];
            }

            self.lower_edges_len[poly_index] = poly_len;

            // ADD poly to some lower list
         } else {
            self.h_split_poly(poly_index, y, y_px);
         }
      }
   }

   fn h_split_poly(&mut self, poly_index: usize, y: i64, y_px: i64) {
      let p1;
      let p2;

      let poly_start = self.poly_to_pool[poly_index];
      let poly_len = self.upper_edges_len[poly_index];

      let mut i = poly_start;
      let mut upper_i = poly_start;
      let mut lower_i = poly_start;
      let end = poly_start + poly_len;

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

         if i == end {
            return;
         }
      }

      i += 1;

      loop { // Edge's first point above y
         let mut edge = self.upper_edges[i];

         match edge.edge_type {
            EdgeType::LBR | EdgeType::LBL | EdgeType::LVB | EdgeType::CBR |
            EdgeType::CBL | EdgeType::ABR | EdgeType::ABL => {
               let y2 = edge.p1.y;
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

                  self.lower_edges[lower_i] = Edge::new(EdgeType::LHR, usize::MAX, p1, p2);
                  lower_i += 1;

                  lower_edge.p1 = p2;
                  self.lower_edges[lower_i] = lower_edge;
                  lower_i += 1;

                  break;
               } else {
                  p2 = edge.p1;

                  self.upper_edges[upper_i] = edge;
                  upper_i += 1;

                  self.lower_edges[lower_i] = Edge::new(EdgeType::LHR, usize::MAX, p1, p2);
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

         if i == end {
            return;
         }
      }

      i += 1;

      for j in i..end { // Edge's first point again below y
         self.lower_edges[lower_i] = self.upper_edges[j];
         lower_i += 1;
      }

      self.upper_edges[upper_i] = Edge::new(EdgeType::LHL, usize::MAX, p2, p1);
      upper_i += 1;

      self.upper_edges_len[poly_index] = upper_i - poly_start;
      self.lower_edges_len[poly_index] = lower_i - poly_start;
   }

   #[inline]
   fn h_intersection(&self, edge: &Edge, y_px: i64) -> i64 {
      if edge.edge_type == EdgeType::LVT || edge.edge_type == EdgeType::LVB {
         return edge.p1.x;
      }

      let ref h_ref = self.hori_intersections_ref[edge.segment];

      self.hori_intersections[
         h_ref.start + (y_px - h_ref.first_px) as usize
      ]
   }
}

fn create_default_vec<T>(capacity: usize) -> Vec<T> where T: Default + Clone {
   repeat(T::default()).take(capacity).collect()
}

fn h_multi_intersect_fast(p1: &Point, p2: &Point, step_y: i64, mut vec_start: usize, inters: &mut Vec<i64>) -> (usize, i64) {
   let (p1, p2) = if p1.y > p2.y {
      (p2, p1)
   } else {
      (p1, p2)
   };

   let start = 1 + p1.y / step_y;
   let end = 1 + (p2.y - 1) / step_y;

   let dy = p2.y - p1.y;
   let dx = p2.x - p1.x;
   let dx_signum = dx.signum();

   let step_x = dx * step_y / dy;

   let max_div_dy = i64::MAX / dy;

   let err_step = max_div_dy * (step_y * dx * dx_signum - step_x * dx_signum * dy);

   let first_y = start * step_y;

   let fdy = first_y - p1.y;
   let fdx = dx * fdy / dy;

   let mut x = p1.x + fdx;

   if err_step == 0 {
      for _ in start..end {
         inters[vec_start] = x;
         vec_start += 1;

         x += step_x;
      }

      return (vec_start, first_y);
   }

   let mut err = max_div_dy * (fdy * dx * dx_signum - fdx * dx_signum * dy) - HALF_MAX_ERR;

   for _ in start..end {
      if err > 0 {
         x += dx_signum;
         err -= i64::MAX;
      }

      inters[vec_start] = x;
      vec_start += 1;

      x += step_x;

      err += err_step;
   }

   (vec_start, first_y)
}

fn v_multi_intersect_fast(p1: &Point, p2: &Point, step_x: i64, mut vec_start: usize, inters: &mut Vec<i64>) -> (usize, i64) {
   let (p1, p2) = if p1.x > p2.x {
      (p2, p1)
   } else {
      (p1, p2)
   };

   let start = 1 + p1.x / step_x;
   let end = 1 + (p2.x - 1) / step_x;

   let dx = p2.x - p1.x;
   let dy = p2.y - p1.y;
   let dy_signum = dy.signum();

   let step_y = dy * step_x / dx;

   let max_div_dx = i64::MAX / dx;

   let err_step = max_div_dx * (step_x * dy * dy_signum - step_y * dy_signum * dx);

   let first_x = start * step_x;

   let fdx = first_x - p1.x;
   let fdy = dy * fdx / dx;

   let mut y = p1.y + fdy;

   if err_step == 0 {
      for _ in start..end {
         inters[vec_start] = y;
         vec_start += 1;

         y += step_y;
      }

      return (vec_start, first_x);
   }

   let mut err = max_div_dx * (fdx * dy * dy_signum - fdy * dy_signum * dx) - HALF_MAX_ERR;

   for _ in start..end {
      if err > 0 {
         y += dy_signum;
         err -= i64::MAX;
      }

      inters[vec_start] = y;
      vec_start += 1;

      y += step_y;

      err += err_step;
   }

   (vec_start, first_x)
}

