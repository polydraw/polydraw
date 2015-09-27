use super::coordinate::Coordinate;
use super::point::Point;

pub struct Line<T> {
   pub p1: Point<T>,
   pub p2: Point<T>
}

impl<T> Line<T> where T: Coordinate {
   pub fn new(p1: Point<T>, p2: Point<T>) -> Self {
      Line {
         p1: p1,
         p2: p2
      }
   }
}

#[cfg(test)]
mod tests {
   use super::Line;
   use super::super::point::Point;

   #[test]
   fn test_new_f64() {
      let line = Line::new(
         Point::new(10_f64, 20_f64),
         Point::new(-100_f64, 0_f64)
      );
      assert_eq!(line.p1.x, 10_f64);
      assert_eq!(line.p1.y, 20_f64);
      assert_eq!(line.p2.x, -100_f64);
      assert_eq!(line.p2.y, 0_f64);
   }

   #[test]
   fn test_new_f32() {
      let line = Line::new(
         Point::new(10_f32, 20_f32),
         Point::new(-100_f32, 0_f32)
      );
      assert_eq!(line.p1.x, 10_f32);
      assert_eq!(line.p1.y, 20_f32);
      assert_eq!(line.p2.x, -100_f32);
      assert_eq!(line.p2.y, 0_f32);
   }
}
