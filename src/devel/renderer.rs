use geom::point::Point;
use renderer::Renderer;
use frame::Frame;

use super::Scene;

pub struct DevelRenderer {
   scene: Scene,
   width: u32,
   height: u32,
}


impl DevelRenderer {
   #[inline]
   pub fn new(scene: Scene) -> Self {
      DevelRenderer {
         scene: scene,
         width: 0,
         height: 0,
      }
   }
}


impl Renderer for DevelRenderer {
   fn init(&mut self, width: u32, height: u32) {
      self.width = width;
      self.height = height;
   }

   fn render(&mut self, frame: &mut Frame) {
      let (min_y, max_y, min_i, max_i) = min_max_y_index(&self.scene.polys[0].points);
   }
}


pub fn min_max_y_index(points: &Vec<Point>) -> (i64, i64, usize, usize) {
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

   (min_y, max_y, min_i, max_i)
}
