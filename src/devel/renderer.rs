use geom::point::Point;
use renderer::Renderer;
use frame::Frame;
use draw::RGB;

use super::Scene;

pub struct DevelRenderer {
   scene: Scene,
   aliased: Vec<RGB>,
   rendered: Vec<RGB>,
}

const SUBDIVISIONS: i64 = 3;

impl DevelRenderer {
   #[inline]
   pub fn new(scene: Scene) -> Self {
      DevelRenderer {
         scene: scene,

         aliased: vec!(),
         rendered: vec!(),
      }
   }

   fn check_resize(&mut self, width: u32, height: u32) {
      if self.rendered.len() == (width * height) as usize {
         return;
      }

      self.aliased.resize((width * height) as usize * (SUBDIVISIONS * SUBDIVISIONS) as usize, RGB::new(0, 0, 0));
      self.rendered.resize((width * height) as usize, RGB::new(0, 0, 0));
   }

   #[inline]
   fn clear(&mut self) {
      let mut aliased: &mut Vec<RGB> = self.aliased.as_mut();

      for i in 0..aliased.len() {
         aliased[i] = RGB::default();
      }
   }

   #[inline]
   fn render_aliased(&mut self, frame: &mut Frame) {
      let mut aliased: &mut Vec<RGB> = self.aliased.as_mut();

      for poly in &self.scene.polys {
         let points = &poly.points;

         let (left, right) = get_left_right_edges(points);

         let mut left_i = 0;
         let mut right_i = 0;

         let mut left_edge = left[left_i];
         let mut right_edge = right[right_i];

         let mut left_x = left_edge.p1.x;
         let mut right_x = right_edge.p1.x;

         let left_last_i = left.len() - 1;
         let right_last_i = right.len() - 1;

         let mut left_advancer = YAxisAdvancer::new(&left_edge);
         let mut right_advancer = YAxisAdvancer::new(&right_edge);

         let min_y = left_edge.p1.y;
         let max_y = left[left_last_i].p2.y;

         for y in min_y..max_y + 1 {
            if left_edge.p2.y == y && left_i != left_last_i {
               left_i += 1;
               left_edge = left[left_i];
               left_advancer = YAxisAdvancer::new(&left_edge);
            }

            if right_edge.p2.y == y && right_i != right_last_i  {
               right_i += 1;
               right_edge = right[right_i];
               right_advancer = YAxisAdvancer::new(&right_edge);
            }

            for x in left_x..right_x {
               aliased[y as usize * frame.width as usize * SUBDIVISIONS as usize + x as usize] = poly.color;
            }

            left_x = left_advancer.advance();
            right_x = right_advancer.advance();
         }
      }
   }

   #[inline]
   fn downsample(&mut self, frame: &mut Frame) {
      let aliased: &mut Vec<RGB> = self.aliased.as_mut();

      let rendered: &mut Vec<RGB> = self.rendered.as_mut();

      let divisor = (SUBDIVISIONS * SUBDIVISIONS) as u16;

      for y in 0..frame.height as usize {
         for x in 0..frame.width as usize {
            let mut r: u16 = 0;
            let mut g: u16 = 0;
            let mut b: u16 = 0;
            for box_y in 0..SUBDIVISIONS as usize {
               for box_x in 0..SUBDIVISIONS as usize {
                  let pos = (box_y + y * SUBDIVISIONS as usize * frame.width as usize * SUBDIVISIONS as usize) + box_x + x * SUBDIVISIONS as usize;
                  let color = &aliased[pos];
                  r += color.r as u16;
                  g += color.g as u16;
                  b += color.b as u16;
               }
            }

            r /= divisor;
            g /= divisor;
            b /= divisor;

            let color = RGB::new(r as u8, g as u8, b as u8);

            frame.put_pixel(x as i32, y as i32, &color);

            rendered[y * frame.width as usize + x] = color;
         }
      }
   }
}


impl Renderer for DevelRenderer {
   #[inline]
   fn init(&mut self, width: u32, height: u32) {
      self.check_resize(width, height);
   }

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

   fn advance(&mut self) -> i64 {
      self.f64x += self.delta;
      self.f64x.round() as i64
   }
}


fn get_left_right_edges(points: &Vec<Point>) -> (Vec<Edge>, Vec<Edge>){
   let (min_i, max_i) = find_min_max_y_index(points);

   let left = get_left_edges(points, min_i, max_i);
   let right = get_right_edges(points, min_i, max_i);

   (left, right)
}

fn get_left_edges(points: &Vec<Point>, min_i: usize, max_i: usize) -> Vec<Edge> {
   let mut edges = Vec::new();

   let mut curr_i = min_i;
   let mut prev_i = curr_i;

   loop {
      curr_i += 1;

      if curr_i == points.len() {
         curr_i = 0;
      }

      edges.push(Edge::new(points[prev_i], points[curr_i]));

      prev_i = curr_i;

      if curr_i == max_i {
         break;
      }
   }

   edges
}

fn get_right_edges(points: &Vec<Point>, min_i: usize, max_i: usize) -> Vec<Edge> {
   let mut edges = Vec::new();

   let mut curr_i = min_i;
   let mut prev_i = curr_i;

   loop {
      if curr_i == 0 {
         curr_i = points.len() - 1;
      } else {
         curr_i -= 1;
      }

      edges.push(Edge::new(points[prev_i], points[curr_i]));

      prev_i = curr_i;

      if curr_i == max_i {
         break;
      }
   }

   edges
}

fn find_min_max_y_index(points: &Vec<Point>) -> (usize, usize) {
   let (first, rest) = points.split_first().unwrap();

   let mut min_y = first.y;
   let mut max_y = min_y;
   let mut min_i = 0;
   let mut max_i = 0;

   for (i, point) in rest.iter().enumerate() {
      if point.y < min_y {
         min_i = i + 1;
         min_y = point.y;
      } else if point.y > max_y {
         max_i = i + 1;
         max_y = point.y;
      }
   }

   (min_i, max_i)
}

