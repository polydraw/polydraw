extern crate polydraw;

use std::cmp::{Ordering, min, max};
use std::i64;

use polydraw::geom::point::Point;
use polydraw::{Application, Renderer, Frame};
use polydraw::raster::{Scene, Rasterizer, EdgeType, Poly, create_default_vec};


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


struct ClipRenderer {
   rasterizer: Rasterizer,
   div_per_pixel: i64,

   edges: Vec<Edge>,
   polys: Vec<Poly>,

   sections: Vec<Section>,
   sections_end: usize,

   sections_min_y: Vec<i64>,
   sections_max_y: Vec<i64>,
   sections_order: Vec<usize>,

   active: Vec<Section>,
   active_source: Vec<usize>,
   active_end: usize,
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
      let sections_order = create_default_vec(65536);

      let active = create_default_vec(65536);
      let active_source = create_default_vec(65536);

      ClipRenderer {
         rasterizer: Rasterizer::new(),
         div_per_pixel: div_per_pixel,

         edges: edges,
         polys: polys,

         sections: sections,
         sections_end: 0,

         sections_min_y: sections_min_y,
         sections_max_y: sections_max_y,
         sections_order: sections_order,

         active: active,
         active_source: active_source,
         active_end: 0,
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

         self.sections_min_y[section_index] = min_y;
         self.sections_max_y[section_index] = max_y;
         self.sections_order[section_index] = section_index;
      }

      self.sort_sections_order();

      println!("MIN Y {:?}", &self.sections_min_y[..self.sections_end]);
      println!("MAX Y {:?}", &self.sections_max_y[..self.sections_end]);
      println!("ORDER {:?}", &self.sections_order[..self.sections_end]);

      self.iterate_sections();
   }

   fn sort_sections_order(&mut self) {
      let sections_order = &mut self.sections_order[..self.sections_end];
      let sections_min_y = &self.sections_min_y;
      let sections_max_y = &self.sections_max_y;

      sections_order.sort_by(|a, b| {
         match sections_min_y[*a].cmp(&sections_min_y[*b]) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => sections_max_y[*a].cmp(&sections_max_y[*b])
         }
      });
   }

   fn iterate_sections(&self) {
      let mut prev_y = i64::MIN;
      for order_index in 0..self.sections_end {
         let section_index = self.sections_order[order_index];
         let min_y = self.sections_min_y[section_index];

         println!("I MIN Y {:?} {:?}", section_index, min_y);

         if min_y != prev_y {
            println!("HOR {:?}", min_y);
            prev_y = min_y;
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

            self.sections_end += 1;
         }
      }

      println!("SECTIONS {:?}", &self.sections[..self.sections_end]);
   }

   fn clip_sections(&mut self) {
      let sections_target = self.sections_order[0];
      let sections_clipper = self.sections_order[1];

      let active_target = self.copy_to_active(sections_target);
      let active_clipper = self.copy_to_active(sections_clipper);

      println!("TARGET {:?}", self.active[active_target]);
      println!("CLIPPER {:?}", self.active[active_clipper]);

      let intersection = self.intersect_sections(
         &self.active[active_target],
         &self.active[active_clipper]
      );
      println!("INTERSECTION {:?}", intersection);

      match intersection {
         Some(pt) => {
            self.sections[sections_target].set_bottom(&pt);
            self.sections[sections_clipper].set_bottom(&pt);

            self.active[active_target].set_top(&pt);
            self.active[active_clipper].set_top(&pt);
         },
         None => {}
      }

      for index in 0..self.sections_end {
         println!("SECTIONS [{}] - {:?}", index, self.sections[index]);
      }

      for index in 0..self.active_end {
         println!("ACTIVE   [{}] - {:?}", index, self.active[index]);
      }
   }

   fn copy_to_active(&mut self, sections_index: usize) -> usize {
      let active_index = self.active_end;

      self.active_source[active_index] = sections_index;
      self.active[active_index] = self.sections[sections_index];

      self.active_end += 1;

      active_index
   }

   fn intersect_sections(&self, s1: &Section, s2: &Section) -> Option<Point> {
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

      let x1 = max(min(s1.p1.x, s1.p2.x), min(s2.p1.x, s2.p2.x));
      let x2 = min(max(s1.p1.x, s1.p2.x), max(s2.p1.x, s2.p2.x));
      let y1 = max(min(s1.p1.y, s1.p2.y), min(s2.p1.y, s2.p2.y));
      let y2 = min(max(s1.p1.y, s1.p2.y), max(s2.p1.y, s2.p2.y));

      if x < x1 || x > x2 || y < y1 || y > y2 {
         return None;
      }

      return Some(Point::new(x, y));
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
