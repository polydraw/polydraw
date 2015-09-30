extern crate polydraw;

use polydraw::{Application, Renderer, RenderFrame};
use polydraw::geom::line::{LineSegment, LineIntersection};
use polydraw::geom::point::Point;
use polydraw::draw::{RGB, bresenham, hline, vline};

struct IntersectionRenderer {
   intersection: Option<Point<f64>>,

   l1: LineSegment<f64>,
   l2: LineSegment<f64>,

   top_left: Point<f64>,
   top_right: Point<f64>,
   bottom_left: Point<f64>,
   bottom_right: Point<f64>,

   width: u32,
   height: u32,
}

impl IntersectionRenderer {
   fn new() -> Self {
      IntersectionRenderer {
         intersection: None,

         l1: LineSegment::default(),
         l2: LineSegment::default(),

         top_left: Point::default(),
         top_right: Point::default(),
         bottom_left: Point::default(),
         bottom_right: Point::default(),

         width: 0,
         height: 0,
      }
   }

   fn point_rect(&self, frame: &mut RenderFrame, x: i32, y: i32, color: &RGB) {
      let half = 12;
      let right_x = x + half;
      let left_x = x - half;
      let top_y = y + half;
      let bottom_y = y - half;

      hline(frame, left_x, right_x, top_y, color);
      hline(frame, left_x, right_x, bottom_y, color);

      vline(frame, left_x, bottom_y, top_y, color);
      vline(frame, right_x, bottom_y, top_y, color);
   }

   fn recalc(&mut self) {
      self.l1.update(
         self.bottom_left.x, self.bottom_left.y,
         self.width as f64 - self.top_right.x, self.height as f64 - self.top_right.y
      );

      self.l2.update(
         self.top_left.x, self.height as f64 - self.top_left.y,
         self.width as f64 - self.bottom_right.x, self.bottom_right.y
      );

      let intersection = self.l1.line().intersect(self.l2.line());
      match intersection {
         LineIntersection::Point(p) => {
            self.intersection = Some(p);
         },
         _ => {
            self.intersection = None;
         }
      }
   }
}

impl Renderer for IntersectionRenderer {
   fn init(&mut self, frame: &RenderFrame) {
      self.bottom_left.update(100_f64, 120_f64);
      self.top_right.update(100_f64, 100_f64);

      self.top_left.update(140_f64, 90_f64);
      self.bottom_right.update(140_f64, 100_f64);

      self.width = frame.width;
      self.height = frame.height;

      self.recalc();
   }

   fn resized(&mut self, width: u32, height: u32) {
      self.width = width;
      self.height = height;

      self.recalc();
   }

   fn render(&mut self, frame: &mut RenderFrame) {
      frame.clear();

      let l1_color = RGB::new(127, 223, 255);
      let l2_color = RGB::new(127, 255, 223);
      let intersection_color = RGB::new(255, 223, 191);

      let l1_p1_x = self.l1.x1() as i32;
      let l1_p1_y = self.l1.y1() as i32;

      let l1_p2_x = self.l1.x2() as i32;
      let l1_p2_y = self.l1.y2() as i32;

      let l2_p1_x = self.l2.x1() as i32;
      let l2_p1_y = self.l2.y1() as i32;

      let l2_p2_x = self.l2.x2() as i32;
      let l2_p2_y = self.l2.y2() as i32;

      bresenham(frame, l1_p1_x, l1_p1_y, l1_p2_x, l1_p2_y, &l1_color);
      bresenham(frame, l2_p1_x, l2_p1_y, l2_p2_x, l2_p2_y, &l2_color);

      self.point_rect(frame, l1_p1_x, l1_p1_y, &l1_color);
      self.point_rect(frame, l1_p2_x, l1_p2_y, &l1_color);
      self.point_rect(frame, l2_p1_x, l2_p1_y, &l2_color);
      self.point_rect(frame, l2_p2_x, l2_p2_y, &l2_color);

      match self.intersection {
         Some(ref p) => {
            self.point_rect(frame, p.x as i32, p.y as i32, &intersection_color);
         },
         _ => {}
      }
   }
}

fn main() {
   let mut renderer = IntersectionRenderer::new();

   Application::new()
      .renderer(&mut renderer)
      .title("Intersection")
      .run();
}
