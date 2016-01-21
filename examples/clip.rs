extern crate polydraw;

use std::cmp::{Ordering, min, max};
use std::i64;

use polydraw::geom::point::Point;
use polydraw::{Application, Renderer, Frame};
use polydraw::raster::{Scene, Rasterizer, EdgeType, Poly, create_default_vec};
use polydraw::num::NumberOps;


#[derive(Debug, Clone, Copy)]
pub struct Edge {
   pub edge_type: EdgeType,
   pub p1: Point,
   pub p2: Point,
}

impl Edge {
   #[inline]
   pub fn new(edge_type: EdgeType, x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
      Edge {
         edge_type: edge_type,
         p1: Point::new(x1, y1),
         p2: Point::new(x2, y2),
      }
   }

   #[inline]
   pub fn vert_top(x1: i64, y1: i64, x2: i64, y2: i64) -> Edge {
      Edge::new(EdgeType::LVT, x1, y1, x2, y2)
   }

   #[inline]
   pub fn vert_bottom(x1: i64, y1: i64, x2: i64, y2: i64) -> Edge {
      Edge::new(EdgeType::LVB, x1, y1, x2, y2)
   }

   #[inline]
   pub fn hori_right(x1: i64, y1: i64, x2: i64, y2: i64) -> Edge {
      Edge::new(EdgeType::LHR, x1, y1, x2, y2)
   }

   #[inline]
   pub fn hori_left(x1: i64, y1: i64, x2: i64, y2: i64) -> Edge {
      Edge::new(EdgeType::LHL, x1, y1, x2, y2)
   }

   #[inline]
   pub fn top_right(x1: i64, y1: i64, x2: i64, y2: i64) -> Edge {
      Edge::new(EdgeType::LTR, x1, y1, x2, y2)
   }

   #[inline]
   pub fn top_left(x1: i64, y1: i64, x2: i64, y2: i64) -> Edge {
      Edge::new(EdgeType::LTL, x1, y1, x2, y2)
   }

   #[inline]
   pub fn bottom_right(x1: i64, y1: i64, x2: i64, y2: i64) -> Edge {
      Edge::new(EdgeType::LBR, x1, y1, x2, y2)
   }

   #[inline]
   pub fn bottom_left(x1: i64, y1: i64, x2: i64, y2: i64) -> Edge {
      Edge::new(EdgeType::LBL, x1, y1, x2, y2)
   }
}


#[derive(Debug, Clone, Copy, Default)]
pub struct Section {
   pub edge_type: EdgeType,
   pub p1: Point,
   pub p2: Point,
   pub edge: usize,
   pub poly: usize,
}

impl Section {
   fn set_bottom(&mut self, point: &Point) {
      if self.edge_type.reversed() {
         self.p2 = *point;
      } else {
         self.p1 = *point;
      }
   }

   fn set_top(&mut self, point: &Point) {
      if self.edge_type.reversed() {
         self.p1 = *point;
      } else {
         self.p2 = *point;
      }
   }
}


pub enum ClipEvent {
   Section(usize),
   Horizontal(i64),
}


struct ClipRenderer {
   rasterizer: Rasterizer,
   div_per_pixel: i64,

   edges: Vec<Edge>,
   polys: Vec<Poly>,

   sections: Vec<Section>,
   sections_end: usize,

   absolute_min_y: i64,

   sections_min_y: Vec<i64>,
   sections_max_y: Vec<i64>,
   sections_active: Vec<bool>,

   order_to_section: Vec<usize>,
   section_to_order: Vec<usize>,

   active: Vec<Section>,
   active_source: Vec<usize>,
   active_end: usize,
   active_max_y: Vec<i64>,

   active_shift_index: Vec<usize>,
   active_shift_y: Vec<i64>,
   active_shift_end: usize,

   order_to_active: Vec<usize>,
}

impl ClipRenderer {
   fn new() -> Self {
      let div_per_pixel = 1000;

      let mut edges = vec![
         Edge::top_left(5, 1, 1, 6),       // 0
         Edge::top_right(1, 6, 8, 9),      // 1
         Edge::bottom_right(8, 9, 9, 6),   // 2
         Edge::bottom_left(9, 6, 5, 1),    // 3
         Edge::top_right(1, 1, 3, 8),      // 4
         Edge::bottom_right(3, 8, 10, 4),  // 5
         Edge::bottom_left(10, 4, 1, 1),   // 6
      ];

      for edge in edges.iter_mut() {
         edge.p1.x *= div_per_pixel;
         edge.p1.y *= div_per_pixel;
         edge.p2.x *= div_per_pixel;
         edge.p2.y *= div_per_pixel;
      }

      let polys = vec![
         Poly::new(0, 4, 0),    // 0
         Poly::new(4, 7, 1),    // 1
      ];

      let sections = create_default_vec(65536);

      let sections_min_y = create_default_vec(65536);
      let sections_max_y = create_default_vec(65536);
      let sections_active = create_default_vec(65536);

      let order_to_section = create_default_vec(65536);
      let section_to_order = create_default_vec(65536);

      let active = create_default_vec(65536);
      let active_source = create_default_vec(65536);
      let active_max_y = create_default_vec(65536);
      let active_shift_index = create_default_vec(65536);
      let active_shift_y = create_default_vec(65536);

      let order_to_active = create_default_vec(65536);

      ClipRenderer {
         rasterizer: Rasterizer::new(),
         div_per_pixel: div_per_pixel,

         edges: edges,
         polys: polys,

         sections: sections,
         sections_end: 0,

         absolute_min_y: 0,

         sections_min_y: sections_min_y,
         sections_max_y: sections_max_y,
         sections_active: sections_active,

         order_to_section: order_to_section,
         section_to_order: section_to_order,

         active: active,
         active_source: active_source,
         active_end: 0,
         active_max_y: active_max_y,

         active_shift_index: active_shift_index,
         active_shift_y: active_shift_y,
         active_shift_end: 0,

         order_to_active: order_to_active,
      }
   }

   fn create_scene(&self) -> Scene {
      let points = vec![];

      let segments = vec![];

      let circles = vec![];

      let edges = vec![];

      let polys = vec![];

      let colors = vec![];

      Scene {
         points: points,
         segments: segments,
         circles: circles,
         edges: edges,
         polys: polys,
         colors: colors,
      }
   }

   fn eval_sections_min_max_y(&mut self) {
      self.absolute_min_y = i64::MAX;

      for section_index in 0..self.sections_end {
         let ref section = self.sections[section_index];

         let mut min_y = i64::MAX;
         let mut max_y = i64::MIN;

         let (section_min_y, section_max_y) = if section.edge_type.reversed() {
            (section.p2.y, section.p1.y)
         } else {
            (section.p1.y, section.p2.y)
         };

         if section_min_y < min_y  {
            min_y = section_min_y;
         }

         if section_max_y > max_y {
            max_y = section_max_y;
         }

         if min_y < self.absolute_min_y {
            self.absolute_min_y = min_y;
         }

         self.sections_min_y[section_index] = min_y;
         self.sections_max_y[section_index] = max_y;
         self.order_to_section[section_index] = section_index;
      }

      self.sort_sections_order();
      self.init_section_to_order();

      self.iterate_sections();
   }

   fn sort_sections_order(&mut self) {
      let order_to_section = &mut self.order_to_section[..self.sections_end];
      let sections_min_y = &self.sections_min_y;
      let sections_max_y = &self.sections_max_y;

      order_to_section.sort_by(|a, b| {
         match sections_min_y[*a].cmp(&sections_min_y[*b]) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => sections_max_y[*a].cmp(&sections_max_y[*b])
         }
      });
   }

   fn init_section_to_order(&mut self) {
      for order_index in 0..self.sections_end {
         let section_index = self.order_to_section[order_index];
         self.section_to_order[section_index] = order_index;
      }
   }

   fn iterate_sections(&self) {
      let mut prev_min_y = i64::MIN;
      for order_index in 0..self.sections_end {
         let section_index = self.order_to_section[order_index];
         let min_y = self.sections_min_y[section_index];

         if min_y != prev_min_y {
            prev_min_y = min_y;
         }
      }
   }

   fn transfer_sections(&mut self) {
      self.sections_end = 0;

      for poly_index in 0..self.polys.len() {
         let ref poly = self.polys[poly_index];

         for edge_index in poly.start..poly.end {
            let ref edge = self.edges[edge_index];

            self.sections[self.sections_end] = Section {
               edge_type: edge.edge_type,
               p1: edge.p1,
               p2: edge.p2,
               edge: edge_index,
               poly: poly_index,
            };

            self.sections_active[self.sections_end] = false;

            self.sections_end += 1;
         }
      }
   }

   fn clip_sections(&mut self) {
      let mut prev_min_y = self.absolute_min_y;

      let mut active_start = 0;

      self.active_end = 0;
      self.active_shift_end = 0;

      while let Some(clip_event) = self.next_event(prev_min_y, active_start) {
         match clip_event {
            ClipEvent::Horizontal(clip_y) => {
               println!("NEW MIN Y {:?}", clip_y);

               self.active_shift_index[self.active_shift_end] = self.active_end;
               self.active_shift_y[self.active_shift_end] = clip_y;

               self.active_shift_end += 1;

               let active_end = self.active_end;

               for active_order_target in active_start..self.active_end {
                  println!("target active order {}", active_order_target);

                  let active_target = self.order_to_active[active_order_target];

                  println!("target active index {}", active_target);

                  let sections_target = self.active_source[active_target];

                  println!("target sections index {}", sections_target);

                  let target_order_index = self.section_to_order[sections_target];

                  println!("target order index {}", target_order_index);

                  println!("target {:?}", self.active[active_target]);

                  let intersection = self.hori_intersect_active(
                     &self.active[active_target], clip_y
                  );

                  if let Some(point) = intersection {
                     println!("intersection {:?}", point);

                     self.split_active(active_target, &point);
                  }
               }

               active_start = active_end;

               self.check_sections_order();
               self.check_active_max_y();

               prev_min_y = clip_y;

            },
            ClipEvent::Section(clipper_order_index) => {
               println!("---------------------");

               println!("ORD {:?}", &self.order_to_section[0..self.sections_end]);

               println!("cliper order index {:?}", clipper_order_index);

               let sections_clipper = self.order_to_section[clipper_order_index];

               println!("cliper sections index {:?}", sections_clipper);

               let clipper_min_y = self.sections_min_y[sections_clipper];

               println!("cliper min y {}", clipper_min_y);

               let active_clipper = self.copy_to_active(sections_clipper);

               println!("cliper active index {:?}", active_clipper);

               println!("cliper {:?}", self.active[active_clipper]);

               while active_start < self.active_end - 1 {
                  let active_index = self.order_to_active[active_start];
                  let start_max_y = self.active_max_y[active_index];

                  if start_max_y > clipper_min_y {
                     break;
                  }

                  active_start += 1;
                  println!("active start +1 {}", active_start);
               }

               for active_order_target in active_start..self.active_end - 1 {
                  println!("target active order {}", active_order_target);

                  let active_target = self.order_to_active[active_order_target];

                  println!("target active index {}", active_target);

                  let sections_target = self.active_source[active_target];

                  println!("target sections index {}", sections_target);

                  let target_order_index = self.section_to_order[sections_target];

                  println!("target order index {}", target_order_index);

                  println!("target {:?}", self.active[active_target]);

                  let intersection = self.intersect_sections(
                     &self.active[active_target],
                     &self.active[active_clipper]
                  );

                  if let Some(point) = intersection {
                     println!("intersection {:?}", point);

                     self.sections[sections_clipper].set_bottom(&point);
                     self.change_section_order(clipper_order_index, point.y);

                     self.sections_active[sections_clipper] = false;

                     self.active[active_clipper].set_top(&point);
                     self.active_max_y[active_clipper] = point.y;

                     println!("checking order #1");

                     self.check_sections_order();
                     self.check_active_max_y();

                     self.sections[sections_target].set_bottom(&point);
                     self.change_section_order(target_order_index, point.y);

                     self.sections_active[sections_target] = false;

                     self.active[active_target].set_top(&point);
                     self.active_max_y[active_target] = point.y;

                     self.reorder_active(active_order_target);

                     println!("checking order #2");

                     self.check_sections_order();
                     self.check_active_max_y();

                     break;
                  }
               }

               self.reorder_active(active_clipper);
            }
         }

         self.check_active_order();

         let mut active_ones = vec![];
         for i in 0..self.sections_end {
            if self.sections_active[i] == true {
               active_ones.push(1);
            } else {
               active_ones.push(0);
            }
         }
         println!("ACT {:?}", &active_ones);

         for active_index in 0..self.active_end {
            println!("A [{}] {:?}", active_index, &self.active[active_index]);
         }

         println!("*");

         for section_index in 0..self.sections_end {
            println!("S [{}] {:?}", section_index, &self.sections[section_index]);
         }
      }

      let last_index = self.order_to_active[self.active_end - 1];
      let max_y = self.active_max_y[last_index];

      self.active_shift_index[self.active_shift_end] = self.active_end;
      self.active_shift_y[self.active_shift_end] = max_y;

      self.active_shift_end += 1;

      self.check_active_shift();
   }

   fn next_event(&self, prev_y: i64, active_start: usize) -> Option<ClipEvent> {
      for order_index in 0..self.sections_end {
         let section_index = self.order_to_section[order_index];
         if self.sections_active[section_index] == false {
            let min_y = self.sections_min_y[section_index];

            if min_y == prev_y {
               return Some(ClipEvent::Section(order_index));
            } else {
               break;
            }
         }
      }

      if active_start == self.active_end {
         return None;
      }

      let active_index = self.order_to_active[active_start];
      let max_y = self.active_max_y[active_index];
      Some(ClipEvent::Horizontal(max_y))
   }

   fn copy_to_active(&mut self, section_index: usize) -> usize {
      let active_index = self.active_end;

      self.active_source[active_index] = section_index;
      self.active[active_index] = self.sections[section_index];
      self.active_max_y[active_index] = self.sections_max_y[section_index];

      self.sections_active[section_index] = true;

      self.order_to_active[active_index] = active_index;

      self.active_end += 1;

      active_index
   }

   fn split_active(&mut self, active_index: usize, point: &Point) {
      let new_index = self.active_end;

      self.active_source[new_index] = self.active_source[active_index];
      self.active[new_index] = self.active[active_index];
      self.active_max_y[new_index] = self.active_max_y[active_index];

      self.active[new_index].set_bottom(&point);

      self.active[active_index].set_top(&point);
      self.active_max_y[active_index] = point.y;

      self.order_to_active[new_index] = new_index;

      self.active_end += 1;
   }

   fn reorder_active(&mut self, order_index: usize) {
      let active_index = self.order_to_active[order_index];
      let max_y = self.active_max_y[active_index];

      let mut curr_index = order_index;

      if curr_index == 0 {
         return;
      }

      let mut prev_index = curr_index - 1;

      loop {
         let prev_active_index = self.order_to_active[prev_index];
         let prev_max_y = self.active_max_y[prev_active_index];

         if prev_max_y <= max_y {
            break;
         }

         self.order_to_active[curr_index] = prev_active_index;

         curr_index = prev_index;

         if curr_index == 0 {
            break;
         }

         prev_index -= 1;
      }

      if curr_index != order_index {
         self.order_to_active[curr_index] = active_index;
      }
   }

   fn change_section_order(&mut self, order_index: usize, new_min_y: i64) {
      let section_index = self.order_to_section[order_index];
      self.sections_min_y[section_index] = new_min_y;
      let new_max_y = self.sections_max_y[section_index];

      let mut current_order_index = order_index;
      let mut next_order_index = order_index + 1;

      while next_order_index < self.sections_end {
         let next_index = self.order_to_section[next_order_index];

         let min_y = self.sections_min_y[next_index];
         let max_y = self.sections_max_y[next_index];

         if !(new_min_y > min_y || (new_min_y == min_y && new_max_y > max_y)) {
            break;
         }

         self.order_to_section[current_order_index] = next_index;
         self.section_to_order[next_index] = current_order_index;

         current_order_index = next_order_index;
         next_order_index += 1;
      }

      if current_order_index != order_index {
         self.order_to_section[current_order_index] = section_index;
         self.section_to_order[section_index] = current_order_index;
      }
   }

   fn intersect_sections(&self, s1: &Section, s2: &Section) -> Option<Point> {
      let s1_x_min = min(s1.p1.x, s1.p2.x);
      let s2_x_min = min(s2.p1.x, s2.p2.x);

      let s1_x_max = max(s1.p1.x, s1.p2.x);
      let s2_x_max = max(s2.p1.x, s2.p2.x);

      if s1_x_min >= s2_x_max || s2_x_min >= s1_x_max {
         return None;
      }

      let s1_a = (s1.p2.y - s1.p1.y) as f64;
      let s1_b = (s1.p1.x - s1.p2.x) as f64;
      let s1_c = (s1.p2.x * s1.p1.y - s1.p1.x * s1.p2.y) as f64;

      let s2_a = (s2.p2.y - s2.p1.y) as f64;
      let s2_b = (s2.p1.x - s2.p2.x) as f64;
      let s2_c = (s2.p2.x * s2.p1.y - s2.p1.x * s2.p2.y) as f64;

      let denominator = s1_a * s2_b - s2_a * s1_b;

      if denominator == 0. {
         return None;
      }

      let x = (s2_c * s1_b - s1_c * s2_b) / denominator;
      let y = (s2_a * s1_c - s1_a * s2_c) / denominator;

      let x = x.round() as i64;
      let y = y.round() as i64;

      let x1 = max(s1_x_min, s2_x_min);
      let x2 = min(s1_x_max, s2_x_max);
      let y1 = max(min(s1.p1.y, s1.p2.y), min(s2.p1.y, s2.p2.y));
      let y2 = min(max(s1.p1.y, s1.p2.y), max(s2.p1.y, s2.p2.y));

      if x <= x1 || x >= x2 || y <= y1 || y >= y2 {
         return None;
      }

      return Some(Point::new(x, y));
   }

   fn hori_intersect_active(&self, active: &Section, y: i64) -> Option<Point> {
      // TODO: Use original edge as an intersection segment

      let x1 = active.p1.x;
      let y1 = active.p1.y;

      let x2 = active.p2.x;
      let y2 = active.p2.y;

      let y_max = max(y1, y2);

      if y_max <= y {
         return None;
      }

      let x = x1 + ((x2 - x1) * (y - y1)).rounding_div(y2 - y1);

      assert!(x >= min(x1, x2) && x <= max(x1, x2));

      Some(Point::new(x, y))
   }

   fn check_sections_order(&self) {
      let mut prev_min_y = i64::MIN;
      let mut prev_max_y = i64::MAX;

      for order_index in 0..self.sections_end {
         let section_index = self.order_to_section[order_index];

         let min_y = self.sections_min_y[section_index];
         let max_y = self.sections_max_y[section_index];

         if min_y < prev_min_y || (min_y == prev_min_y && max_y < prev_max_y) {
            panic!(
               "Wrong sections order [{}/{}], min/max y {} / {}, prev min/max y {} / {}",
               order_index, section_index, min_y, max_y, prev_min_y, prev_max_y
            );
         }

         prev_min_y = min_y;

         prev_max_y = self.sections_max_y[section_index];
      }

      println!("SORD {:?}", &self.order_to_section[0..self.sections_end]);
   }

   fn check_active_max_y(&self) {
      for active_index in 0..self.active_end {
         let ref active_section = self.active[active_index];

         let section_index = self.active_source[active_index];

         println!("section_index {:?}", section_index);

         let ref section = self.sections[section_index];

         let active_max_y = max(active_section.p1.y, active_section.p2.y);

         if active_max_y != self.active_max_y[active_index] {
            panic!(
               "Wrong active max y [{}] {} != {}",
               active_index, active_max_y, self.active_max_y[active_index]
            );
         }

         if self.sections_active[section_index] == false {
            let section_min_y = min(section.p1.y, section.p2.y);

            if active_max_y > section_min_y {
               println!("{:?}", active_section);
               panic!(
                  "Active max y [{}] {} > Section min y [{}] {}",
                  active_index, active_max_y, section_index, section_min_y
               );
            }
         }
      }
   }

   fn check_active_order(&self) {
      let mut prev_max_y = i64::MIN;
      for order_index in 0..self.active_end {
         let active_index = self.order_to_active[order_index];
         let max_y = self.active_max_y[active_index];

         if prev_max_y > max_y {
            panic!("Wrong active order [{}]", active_index)
         }

         prev_max_y = max_y;
      }
   }

   fn check_active_shift(&self) {
      let mut shift_start = 0;

      for shift_index in 0..self.active_shift_end {
         let shift_end = self.active_shift_index[shift_index];

         for active_index in shift_start..shift_end {
            let ref section = self.active[active_index];

            let (p1, p2) = if section.edge_type.reversed() {
               (section.p2, section.p1)
            } else {
               (section.p1, section.p2)
            };

            println!("A {:?} -> {:?}", p1, p2);
         }

         shift_start = shift_end;
      }
   }
}

impl Renderer for ClipRenderer {
   fn render(&mut self, frame: &mut Frame) {
      frame.clear();

      self.transfer_sections();

      self.eval_sections_min_max_y();

      self.clip_sections();

      let scene = self.create_scene();

      self.rasterizer.render(&scene, frame, self.div_per_pixel);

      panic!("END");
   }
}

fn main() {
   let mut renderer = ClipRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Clip")
      .run();
}
