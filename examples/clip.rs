extern crate polydraw;

use std::cmp::Ordering;
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


struct ClipRenderer {
   rasterizer: Rasterizer,
   div_per_pixel: i64,

   edges: Vec<Edge>,
   polys: Vec<Poly>,

   edge_min_y: Vec<i64>,
   edge_max_y: Vec<i64>,
   edge_order: Vec<usize>,
}

impl ClipRenderer {
   fn new() -> Self {
      let edges = vec![
         Edge::top_left(5, 1, 1, 6),       // 0
         Edge::top_right(1, 6, 8, 9),      // 1
         Edge::bottom_right(8, 9, 9, 6),   // 2
         Edge::bottom_left(9, 6, 5, 1),    // 3
         Edge::top_right(1, 1, 3, 8),      // 4
         Edge::bottom_right(3, 8, 10, 4),  // 5
         Edge::bottom_left(10, 4, 1, 1),   // 6
      ];

      let polys = vec![
         Poly::new(0, 4, 0),    // 0
         Poly::new(4, 7, 1),    // 1
      ];

      let edges_len = edges.len();

      let edge_min_y = create_default_vec(edges_len);
      let edge_max_y = create_default_vec(edges_len);
      let edge_order = create_default_vec(edges_len);

      ClipRenderer {
         rasterizer: Rasterizer::new(),
         div_per_pixel: 1000,

         edges: edges,
         polys: polys,

         edge_min_y: edge_min_y,
         edge_max_y: edge_max_y,
         edge_order: edge_order,
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

   fn init_edge_min_max_y(&mut self) {
      for edge_index in 0..self.edges.len() {
         let ref edge = self.edges[edge_index];

         let mut min_y = i64::MAX;
         let mut max_y = i64::MIN;

         let (edge_min_y, edge_max_y) = if edge.edge_type.reversed() {
            (edge.p2.y, edge.p1.y)
         } else {
            (edge.p1.y, edge.p2.y)
         };

         if edge_min_y < min_y  {
            min_y = edge_min_y;
         }

         if edge_max_y > max_y {
            max_y = edge_max_y;
         }

         self.edge_min_y[edge_index] = min_y;
         self.edge_max_y[edge_index] = max_y;
         self.edge_order[edge_index] = edge_index;
      }

      self.sort_edge_order();

      println!("MIN Y {:?}", self.edge_min_y);
      println!("MAX Y {:?}", self.edge_max_y);
      println!("ORDER {:?}", self.edge_order);

      panic!("END");
   }

   fn sort_edge_order(&mut self) {
      let edge_order = &mut self.edge_order;
      let edge_min_y = &self.edge_min_y;
      let edge_max_y = &self.edge_max_y;

      edge_order.sort_by(|a, b| {
         match edge_min_y[*a].cmp(&edge_min_y[*b]) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => edge_max_y[*a].cmp(&edge_max_y[*b])
         }
      });
   }
}

impl Renderer for ClipRenderer {
   fn render(&mut self, frame: &mut Frame) {
      frame.clear();

      self.init_edge_min_max_y();

      let scene = self.create_scene();

      self.rasterizer.render(&scene, frame, self.div_per_pixel);
   }
}

fn main() {
   let mut renderer = ClipRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Clip")
      .run();
}
