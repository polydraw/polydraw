use std::ptr;

use geom::point::Point;
use renderer::Renderer;
use frame::Frame;
use draw::RGB;
use raster::create_default_vec;

use super::Scene;
use super::Poly;

pub struct DevelRenderer {
   scene: Scene,
   aliased: Vec<RGB>,
}

pub const SUBDIVISIONS: usize = 3;

impl DevelRenderer {
   #[inline]
   pub fn new(scene: Scene) -> Self {
      DevelRenderer {
         scene: scene,

         aliased: vec!(),
      }
   }

   fn check_resize(&mut self, width: u32, height: u32) {
      if self.aliased.len() == (width * height) as usize * SUBDIVISIONS * SUBDIVISIONS {
         return;
      }

      self.aliased.resize((width * height) as usize * SUBDIVISIONS * SUBDIVISIONS, RGB::new(0, 0, 0));
   }

   #[inline]
   fn clear(&mut self) {
      let len = self.aliased.len();
      let vp = self.aliased.as_mut_ptr();

      unsafe {
         ptr::write_bytes(vp, 0, len);
      }
   }

   #[inline]
   fn render_aliased(&mut self, frame: &mut Frame) {
      let mut aliased: &mut Vec<RGB> = self.aliased.as_mut();

      for poly in &self.scene.polys {
         let edges = get_poly_edges(poly);

         let mut sorted_y = create_default_vec(edges.len());

         sorted_edges(&edges, &mut sorted_y);

         let mut advancers = Vec::new();

         for edge in &*edges {
            let advancer = YAxisAdvancer::new(edge);
            advancers.push(advancer);
         }

         let mut y = edges[sorted_y[0]].p1.y;

         for i in 0..sorted_y.len() {
            let edge_index: usize = sorted_y[i];

            if edges[edge_index].p1.y != y {
               break;
            }

            let x = edges[edge_index].p1.x;

            aliased[y as usize * frame.width as usize * SUBDIVISIONS + x as usize] = poly.color;
         }

         let mut next_y_index = 0;

         let mut active = Vec::new();

         let mut sorted_end_y = create_default_vec(sorted_y.len());

         for i in 0..sorted_y.len() {
            sorted_end_y[i] = i;
         }

         loop {
            let mut removed = 0;

            loop {
               if removed == active.len() {
                  break;
               }

               let edge_index: usize = active[removed];

               if edges[edge_index].p2.y != y {
                  break;
               }

               removed += 1;
            }

            if removed != 0 {
               active.drain(0..removed);
            }

            loop {
               if next_y_index >= sorted_y.len() {
                  break;
               }

               let edge_index = sorted_y[next_y_index];

               if edges[edge_index].p1.y != y {
                  break;
               }

               active.push(edge_index);

               active.sort_by(|a, b| {
                  edges[*a].p2.y.cmp(&edges[*b].p2.y)
               });

               next_y_index += 1;
            }

            if active.len() == 0 {
               break;
            }

            y += 1;

            let mut xs = Vec::new();

            for i in 0..active.len() {
               let edge_index = active[i];
               xs.push(
                  advancers[edge_index].advance()
               );
            }

            xs.sort();

            assert!(xs.len() % 2 == 0);

            for i in 0..xs.len() / 2 {
               let left_x = xs[i * 2];
               let right_x = xs[i * 2 + 1];
               for x in left_x..right_x {
                  aliased[y as usize * frame.width as usize * SUBDIVISIONS + x as usize] = poly.color;
               }
            }
         }
      }
   }

   #[inline]
   fn downsample(&mut self, frame: &mut Frame) {
      let aliased: &mut Vec<RGB> = self.aliased.as_mut();

      let (width, height) = (frame.width as usize, frame.height as usize);

      for y in 0..height {
         for x in 0..width {
            let color = calc_pixel_color(aliased, width, x, y);

            frame.put_pixel(x as i32, y as i32, &color);
         }
      }
   }
}


impl Renderer for DevelRenderer {
   #[inline]
   fn init(&mut self, width: u32, height: u32) {
      self.check_resize(width, height);
   }

   #[inline]
   fn render(&mut self, frame: &mut Frame) {
      self.check_resize(frame.width, frame.height);

      self.clear();

      self.render_aliased(frame);

      self.downsample(frame);
   }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Edge {
   p1: Point,
   p2: Point,
}

impl Edge {
   #[inline]
   fn new(p1: Point, p2: Point) -> Self {
      Edge {
         p1: p1,
         p2: p2,
      }
   }
}


struct YAxisAdvancer {
   delta: f64,
   f64x: f64,
}

impl YAxisAdvancer {
   #[inline]
   fn new(edge: &Edge) -> Self {
      let dx = edge.p2.x - edge.p1.x;
      let dy = edge.p2.y - edge.p1.y;

      let delta = dx as f64 / dy as f64;
      let f64x = edge.p1.x as f64;

      YAxisAdvancer {
         delta: delta,
         f64x: f64x,
      }
   }

   #[inline]
   fn advance(&mut self) -> i64 {
      self.f64x += self.delta;
      self.f64x.round() as i64
   }
}

#[inline]
fn calc_pixel_color(aliased: &mut Vec<RGB>, width: usize, x: usize, y: usize) -> RGB {
   let mut r: u16 = 0;
   let mut g: u16 = 0;
   let mut b: u16 = 0;
   for box_y in 0..SUBDIVISIONS {
      for box_x in 0..SUBDIVISIONS {
         let pos = (box_y + y * SUBDIVISIONS) * width * SUBDIVISIONS + box_x + x * SUBDIVISIONS;
         let color = &aliased[pos];
         r += color.r as u16;
         g += color.g as u16;
         b += color.b as u16;
      }
   }

   let divisor = (SUBDIVISIONS * SUBDIVISIONS) as u16;

   r /= divisor;
   g /= divisor;
   b /= divisor;

   RGB::new(r as u8, g as u8, b as u8)
}

fn get_poly_edges(poly: &Poly) -> Vec<Edge> {
   let mut edges = Vec::new();

   fill_edges_from_points(&mut edges, &poly.points);

   for points in &poly.holes {
      fill_edges_from_points(&mut edges, points);
   }

   edges
}

#[inline]
fn fill_edges_from_points(edges: &mut Vec<Edge>, points: &Vec<Point>) {
   if points.len() < 2 {
      return;
   }

   for slice in points.windows(2) {
      match to_renderable_edge(slice[0], slice[1]) {
         Some(edge) => edges.push(edge),
         _ => {}
      }
   }

   match to_renderable_edge(points[points.len() - 1], points[0]) {
      Some(edge) => edges.push(edge),
      _ => {}
   }
}

#[inline]
fn to_renderable_edge(p1: Point, p2: Point) -> Option<Edge> {
   if p1.y == p2.y {
      None
   } else if p2.y > p1.y {
      Some(Edge::new(p1, p2))
   } else {
      Some(Edge::new(p2, p1))
   }
}

#[inline]
fn sorted_edges(edges: &Vec<Edge>, sorted: &mut Vec<usize>) {
   for i in 0..edges.len() {
      sorted[i] = i;
   }

   sorted.sort_by(|a, b| {
      edges[*a].p1.y.cmp(&edges[*b].p1.y)
   });
}

