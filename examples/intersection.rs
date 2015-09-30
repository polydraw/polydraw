extern crate polydraw;

use std::cmp::max;

use polydraw::{Application, Renderer, RenderFrame};
use polydraw::geom::line::{LineSegment, LineIntersection};
use polydraw::geom::point::Point;
use polydraw::draw::{RGB, bresenham, hline, vline};


const HALF_RECT: i32 = 12;

#[derive (Debug)]
enum ActivePoint {
   Line1Point1,
   Line1Point2,
   Line2Point1,
   Line2Point2,
}

struct IntersectionRenderer {
   intersection: Option<Point<f64>>,

   l1: LineSegment<f64>,
   l2: LineSegment<f64>,

   mouse_x: i32,
   mouse_y: i32,

   width: u32,
   height: u32,

   line_color: RGB,
   default_color: RGB,
   hover_color: RGB,
   intersection_color: RGB,

   dragged_point: Option<ActivePoint>,
}

impl IntersectionRenderer {
   fn new() -> Self {
      IntersectionRenderer {
         intersection: None,

         l1: LineSegment::default(),
         l2: LineSegment::default(),

         mouse_x: -10000,
         mouse_y: -10000,

         width: 0,
         height: 0,

         line_color: RGB::new(93, 181, 241),
         default_color: RGB::new(127, 191, 63),
         hover_color: RGB::new(255, 255, 255),
         intersection_color: RGB::new(127, 127, 255),

         dragged_point: None,
      }
   }

   fn line_point_rect(&self, frame: &mut RenderFrame, x: i32, y: i32) {
      self.point_rect(frame, x, y, self.hit_test_color(x, y));
   }

   fn point_rect(&self, frame: &mut RenderFrame, x: i32, y: i32, color: &RGB) {
      let right_x = x + HALF_RECT;
      let left_x = x - HALF_RECT;
      let top_y = y + HALF_RECT;
      let bottom_y = y - HALF_RECT;

      hline(frame, left_x, right_x, top_y, color);
      hline(frame, left_x, right_x, bottom_y, color);

      vline(frame, left_x, bottom_y, top_y, color);
      vline(frame, right_x, bottom_y, top_y, color);
   }

   fn hit_test_color(&self, x: i32, y: i32) -> &RGB {
      if self.hit_test(x, y) {
         &self.hover_color
      } else {
         &self.default_color
      }
   }

   fn hit_test(&self, x: i32, y: i32) -> bool {
      let distance = max((x - self.mouse_x).abs(), (y - self.mouse_y).abs());
      distance <= HALF_RECT
   }

   fn active_point(&self) -> Option<ActivePoint> {
      let l1_p1_x = self.l1.x1() as i32;
      let l1_p1_y = self.l1.y1() as i32;

      if self.hit_test(l1_p1_x, l1_p1_y) {
         return Some(ActivePoint::Line1Point1);
      }

      let l1_p2_x = self.l1.x2() as i32;
      let l1_p2_y = self.l1.y2() as i32;

      if self.hit_test(l1_p2_x, l1_p2_y) {
         return Some(ActivePoint::Line1Point2);
      }

      let l2_p1_x = self.l2.x1() as i32;
      let l2_p1_y = self.l2.y1() as i32;

      if self.hit_test(l2_p1_x, l2_p1_y) {
         return Some(ActivePoint::Line2Point1);
      }

      let l2_p2_x = self.l2.x2() as i32;
      let l2_p2_y = self.l2.y2() as i32;

      if self.hit_test(l2_p2_x, l2_p2_y) {
         return Some(ActivePoint::Line2Point2);
      }

      None
   }

   fn recalc_intersection(&mut self) {
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
   fn init(&mut self, width: u32, height: u32) {
      self.l1.update(
         100_f64, 120_f64,
         width as f64 - 100_f64, height as f64 - 100_f64
      );

      self.l2.update(
         140_f64, height as f64 - 90_f64,
         width as f64 - 140_f64, 100_f64
      );

      self.width = width;
      self.height = height;

      self.recalc_intersection();
   }

   fn resized(&mut self, width: u32, height: u32) {
      let dx: f64 = width as f64 - self.width as f64;
      let dy: f64 = height as f64 - self.height as f64;

      let l1_p1_x = self.l1.x1();
      let l1_p1_y = self.l1.y1();

      let l1_p2_x = self.l1.x2();
      let l1_p2_y = self.l1.y2();

      let l2_p1_x = self.l2.x1();
      let l2_p1_y = self.l2.y1();

      let l2_p2_x = self.l2.x2();
      let l2_p2_y = self.l2.y2();

      self.l1.update(l1_p1_x, l1_p1_y, l1_p2_x + dx, l1_p2_y + dy);

      self.l2.update(l2_p1_x, l2_p1_y + dy, l2_p2_x + dx, l2_p2_y);

      self.width = width;
      self.height = height;

      self.recalc_intersection();
   }

   fn mouse_moved(&mut self, x: i32, y: i32) {
      self.mouse_x = x;
      self.mouse_y = y;

      let x_f64 = x as f64;
      let y_f64 = y as f64;

      match self.dragged_point {
         Some(ActivePoint::Line1Point1) => {
            let x2 = self.l1.x2();
            let y2 = self.l1.y2();
            self.l1.update(x_f64, y_f64, x2, y2);
         },

         Some(ActivePoint::Line1Point2) => {
            let x1 = self.l1.x1();
            let y1 = self.l1.y1();
            self.l1.update(x1, y1, x_f64, y_f64);
         },

         Some(ActivePoint::Line2Point1) => {
            let x2 = self.l2.x2();
            let y2 = self.l2.y2();
            self.l2.update(x_f64, y_f64, x2, y2);
         },

         Some(ActivePoint::Line2Point2) => {
            let x1 = self.l2.x1();
            let y1 = self.l2.y1();
            self.l2.update(x1, y1, x_f64, y_f64);
         },

         _ => {
            return;
         }
      }

      self.recalc_intersection();
   }

   fn mouse_left_button_pressed(&mut self) {
      self.dragged_point = self.active_point();
   }

   fn mouse_left_button_released(&mut self) {
      self.dragged_point = None;
   }

   fn render(&mut self, frame: &mut RenderFrame) {
      frame.clear();

      let l1_p1_x = self.l1.x1() as i32;
      let l1_p1_y = self.l1.y1() as i32;

      let l1_p2_x = self.l1.x2() as i32;
      let l1_p2_y = self.l1.y2() as i32;

      let l2_p1_x = self.l2.x1() as i32;
      let l2_p1_y = self.l2.y1() as i32;

      let l2_p2_x = self.l2.x2() as i32;
      let l2_p2_y = self.l2.y2() as i32;

      bresenham(frame, l1_p1_x, l1_p1_y, l1_p2_x, l1_p2_y, &self.line_color);
      bresenham(frame, l2_p1_x, l2_p1_y, l2_p2_x, l2_p2_y, &self.line_color);

      self.line_point_rect(frame, l1_p1_x, l1_p1_y);
      self.line_point_rect(frame, l1_p2_x, l1_p2_y);
      self.line_point_rect(frame, l2_p1_x, l2_p1_y);
      self.line_point_rect(frame, l2_p2_x, l2_p2_y);

      match self.intersection {
         Some(ref p) => {
            self.point_rect(frame, p.x as i32, p.y as i32, &self.intersection_color);
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
