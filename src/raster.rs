use std::cmp::{PartialOrd, Ordering, min, max};
use std::iter::repeat;
use std::{usize, i64};

use frame::Frame;
use draw::RGB;

pub const HALF_MAX_ERR: i64  = i64::MAX / 2;

const DIV_PER_PIXEL: i64 = 1000;


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

#[derive(Debug, Clone, Copy)]
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
         EdgeType::CBR | EdgeType::CBL | EdgeType::ATR | EdgeType::ATL => {
            true
         },
         _ => {
            false
         }
      }
   }
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
   pub edge_type: EdgeType,
   pub segment: usize,
   pub circle: usize,
}

impl Edge {
   #[inline]
   pub fn new(edge_type: EdgeType, segment: usize, circle: usize) -> Self {
      Edge {
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

impl Default for Edge {
   fn default() -> Edge {
      Edge::new(EdgeType::LTR, 0, 0)
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
   pub edges: Vec<Edge>,
   pub polys: Vec<Poly>,
   pub colors: Vec<RGB>,
}

#[derive(Debug, Clone)]
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
   pub points: Vec<Point>,
   pub segments: Vec<Segment>,
   pub edges: Vec<Edge>,
   pub polys: Vec<PolyRef>,

   pub vert_intersections_ref: Vec<IntersectionRef>,
   pub hori_intersections_ref: Vec<IntersectionRef>,
   pub vert_intersections: Vec<i64>,
   pub hori_intersections: Vec<i64>,

   pub points_end: usize,
   pub segments_end: usize,
   pub edges_end: usize,
   pub polys_end: usize,
   pub vert_intersections_end: usize,
   pub hori_intersections_end: usize,

   pub polys_min_y: Vec<i64>,
   pub polys_max_y: Vec<i64>,
   pub polys_sorted_min_y: Vec<usize>,
}

impl Rasterizer {
   pub fn new() -> Self {
      let points = create_default_vec(65536);
      let segments = create_default_vec(65536);
      let edges = create_default_vec(65536);
      let polys = create_default_vec(65536);

      let vert_intersections_ref = create_default_vec(65536);
      let hori_intersections_ref = create_default_vec(65536);
      let vert_intersections = create_default_vec(65536);
      let hori_intersections = create_default_vec(65536);

      let polys_min_y = create_default_vec(65536);
      let polys_max_y = create_default_vec(65536);
      let polys_sorted_min_y = create_default_vec(65536);

      Rasterizer {
         points: points,
         segments: segments,
         edges: edges,
         polys: polys,

         vert_intersections_ref: vert_intersections_ref,
         hori_intersections_ref: hori_intersections_ref,
         vert_intersections: vert_intersections,
         hori_intersections: hori_intersections,

         points_end: 0,
         segments_end: 0,
         edges_end: 0,
         polys_end: 0,
         vert_intersections_end: 0,
         hori_intersections_end: 0,

         polys_min_y: polys_min_y,
         polys_max_y: polys_max_y,
         polys_sorted_min_y: polys_sorted_min_y,
      }
   }

   #[allow(unused_variables)]
   pub fn render(&mut self, scene: &Scene, frame: &mut Frame) {
      self.tranfer_scene(scene);

      self.check_scene_correctness(scene);

      self.intersect_edges();

      self.check_intersections();

      self.calc_poly_min_max_y();

      self.check_poly_min_max_y();
   }

   pub fn tranfer_scene(&mut self, scene: &Scene) {
      self.points_end = scene.points.len();
      for i in 0..self.points_end {
         self.points[i] = scene.points[i];
      }

      self.segments_end = scene.segments.len();
      for i in 0..self.segments_end {
         self.segments[i] = scene.segments[i];

         self.vert_intersections_ref[i].start = usize::MAX;
      }

      self.edges_end = scene.edges.len();
      for i in 0..self.edges_end {
         self.edges[i] = scene.edges[i];
      }

      self.polys_end = scene.polys.len();
      for i in 0..self.polys_end {
         self.polys[i].start = scene.polys[i].start;
         self.polys[i].end = scene.polys[i].end;
         self.polys[i].src = i;
      }
   }

   pub fn check_scene_correctness(&self, scene: &Scene) {
      for poly in &self.polys[0..self.polys_end] {
         self.check_poly_connected(poly);
         self.check_segments_orientation(poly);
         self.check_edges_orientation(poly);
      }

      self.check_index_coverage(scene);
   }

   fn check_poly_connected(&self, poly: &PolyRef) {
      let mut prev = self.edge_head(&self.edges[poly.end - 1]);

      for edge in &self.edges[poly.start..poly.end] {
         let current = self.edge_tail(edge);
         if current != prev {
            panic!("Unclosed poly : {:?}", poly);
         }

         prev = self.edge_head(edge);
      }
   }

   fn check_segments_orientation(&self, poly: &PolyRef) {
      for edge in &self.edges[poly.start..poly.end] {
         let ref segment = self.segments[edge.segment];

         let ref less = self.points[segment.p1];
         let ref greater = self.points[segment.p2];

         if greater <= less {
            panic!("Wrong segment orientation : {:?} / {:?}", edge, segment);
         }
      }
   }

   fn check_edges_orientation(&self, poly: &PolyRef) {
      for edge in &self.edges[poly.start..poly.end] {
         let tail = self.edge_tail(edge);
         let head = self.edge_head(edge);

         let reversed = edge.reversed();

         if (reversed && tail <= head) || (!reversed && head <= tail) {
            panic!("Wrong edge orientation : {:?}", edge);
         }
      }
   }

   fn check_index_coverage(&self, scene: &Scene) {
      self.check_edge_index_coverage();
      self.check_circle_index_coverage(scene);
      self.check_segment_index_coverage();
      self.check_point_index_coverage(scene);
   }

   fn check_edge_index_coverage(&self) {
      let len = self.edges_end;
      let mut coverage: Vec<bool> = repeat(false).take(len).collect();

      for poly in &self.polys[..self.polys_end] {
         for edge_index in poly.start..poly.end {
            coverage[edge_index] = true;
         }
      }

      if coverage.contains(&false) {
         panic!("Unreferenced edge found");
      }
   }

   fn check_circle_index_coverage(&self, scene: &Scene) {
      let len = scene.circles.len();
      let mut coverage: Vec<bool> = repeat(false).take(len).collect();

      for edge in &self.edges[..self.edges_end] {
         if edge.circle != usize::MAX {
            coverage[edge.circle] = true;
         }
      }

      if coverage.contains(&false) {
         panic!("Unreferenced circle found");
      }
   }

   fn check_segment_index_coverage(&self) {
      let len = self.segments_end;
      let mut coverage: Vec<bool> = repeat(false).take(len).collect();

      for edge in &self.edges[..self.edges_end] {
         coverage[edge.segment] = true;
      }

      if coverage.contains(&false) {
         panic!("Unreferenced segment found");
      }
   }

   fn check_point_index_coverage(&self, scene: &Scene) {
      let len = self.points_end;
      let mut coverage: Vec<bool> = repeat(false).take(len).collect();

      for circle in &scene.circles {
         coverage[circle.center] = true;
      }

      for segment in &self.segments[0..self.segments_end] {
         coverage[segment.p1] = true;
         coverage[segment.p2] = true;
      }

      if coverage.contains(&false) {
         panic!("Unreferenced segment found");
      }
   }

   // TODO: Combine into edge_head_tail method?

   fn edge_head(&self, edge: &Edge) -> &Point {
      let ref segment = self.segments[edge.segment];
      if edge.reversed() {
         &self.points[segment.p1]
      } else {
         &self.points[segment.p2]
      }
   }

   fn edge_tail(&self, edge: &Edge) -> &Point {
      let ref segment = self.segments[edge.segment];
      if edge.reversed() {
         &self.points[segment.p2]
      } else {
         &self.points[segment.p1]
      }
   }

   fn intersect_edges(&mut self) {
      self.vert_intersections_end = 0;
      self.hori_intersections_end = 0;

      for edge in &self.edges[..self.edges_end] {
         let i = edge.segment;

         let ref mut vert_ref = self.vert_intersections_ref[i];
         if vert_ref.start == usize::MAX {
            continue;
         }

         let ref mut hori_ref = self.hori_intersections_ref[i];

         let ref segment = self.segments[i];
         let ref p1 = self.points[segment.p1];
         let ref p2 = self.points[segment.p2];

         vert_ref.start = self.vert_intersections_end;
         hori_ref.start = self.hori_intersections_end;

         let (vert_end, x_first_px) = v_multi_intersect_fast(
            p1, p2, DIV_PER_PIXEL, self.vert_intersections_end, &mut self.vert_intersections
         );

         let (hori_end, y_first_px) = h_multi_intersect_fast(
            p1, p2, DIV_PER_PIXEL, self.hori_intersections_end, &mut self.hori_intersections
         );

         self.vert_intersections_end = vert_end;
         vert_ref.end = vert_end;
         vert_ref.first_px = x_first_px;

         self.hori_intersections_end = hori_end;
         hori_ref.end = hori_end;
         hori_ref.first_px = y_first_px;
      }
   }

   fn check_intersections(&self) {
      for edge in &self.edges[..self.edges_end] {
         let ref segment = self.segments[edge.segment];
         let ref p1 = self.points[segment.p1];
         let ref p2 = self.points[segment.p2];

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

   fn calc_poly_min_max_y(&mut self) {
      let mut sorted_min_y = Vec::with_capacity(self.polys_end);

      for i in 0..self.polys_end {
         let ref poly = self.polys[i];

         let mut poly_min_y = i64::MAX;
         let mut poly_max_y = i64::MIN;

         for edge in &self.edges[poly.start..poly.end] {
            let ref segment = self.segments[edge.segment];
            let ref p1 = self.points[segment.p1];
            let ref p2 = self.points[segment.p2];

            let min_y = min(p1.y, p2.y);
            let max_y = max(p1.y, p2.y);

            if min_y < poly_min_y  {
               poly_min_y = min_y;
            }

            if max_y > poly_max_y {
               poly_max_y = max_y;
            }
         }

         self.polys_min_y[i] = poly_min_y;
         self.polys_max_y[i] = poly_max_y;
         sorted_min_y.push(i);
      }

      sorted_min_y.sort_by(|a, b| self.polys_min_y[*a].cmp(&self.polys_min_y[*b]));

      for i in 0..self.polys_end {
         self.polys_sorted_min_y[i] = sorted_min_y[i];
      }
   }

   fn check_poly_min_max_y(&self) {
      let mut prev_min_y = i64::MIN;
      for i in 0..self.polys_end {
         let poly_i = self.polys_sorted_min_y[i];

         let min_y = self.polys_min_y[poly_i];
         let max_y = self.polys_max_y[poly_i];

         if min_y == i64::MAX {
            panic!("Bad poly min_y value");
         }
         if max_y == i64::MIN {
            panic!("Bad poly max_y value");
         }

         if prev_min_y > min_y  {
            panic!("Polys not sorted by min y");
         }

         prev_min_y = min_y;
      }
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
