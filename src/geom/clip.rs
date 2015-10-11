use super::point::Point;
use super::number::NumberOps;

#[inline]
pub fn h_down_intersect(y: i64, p1: Point, p2: Point) -> EdgeIntersection {
   if p2.y > y {
      if p1.y < y {
         EdgeIntersection::Single(
            Point::new(h_intersect(y, p1, p2), y)
         )
      } else {
         EdgeIntersection::None
      }
   } else {
      if p1.y > y && p2.y < y {
         EdgeIntersection::Double(
            Point::new(h_intersect(y, p1, p2), y),
            Point::new(p2.x, p2.y)
         )
      } else {
         EdgeIntersection::Single(
            Point::new(p2.x, p2.y)
         )
      }
   }
}

#[inline]
pub fn h_up_intersect(y: i64, p1: Point, p2: Point) -> EdgeIntersection {
   if p2.y < y {
      if p1.y > y {
         EdgeIntersection::Single(
            Point::new(h_intersect(y, p1, p2), y)
         )
      } else {
         EdgeIntersection::None
      }
   } else {
      if p1.y < y && p2.y > y {
         EdgeIntersection::Double(
            Point::new(h_intersect(y, p1, p2), y),
            Point::new(p2.x, p2.y)
         )
      } else {
         EdgeIntersection::Single(
            Point::new(p2.x, p2.y)
         )
      }
   }
}

#[inline]
pub fn v_left_intersect(x: i64, p1: Point, p2: Point) -> EdgeIntersection {
   if p2.x > x {
      if p1.x < x {
         EdgeIntersection::Single(
            Point::new(x, v_intersect(x, p1, p2))
         )
      } else {
         EdgeIntersection::None
      }
   } else {
      if p1.x > x && p2.x < x {
         EdgeIntersection::Double(
            Point::new(x, v_intersect(x, p1, p2)),
            Point::new(p2.x, p2.y)
         )
      } else {
         EdgeIntersection::Single(
            Point::new(p2.x, p2.y)
         )
      }
   }
}

#[inline]
pub fn v_right_intersect(x: i64, p1: Point, p2: Point) -> EdgeIntersection {
   if p2.x < x {
      if p1.x > x {
         EdgeIntersection::Single(
            Point::new(x, v_intersect(x, p1, p2))
         )
      } else {
         EdgeIntersection::None
      }
   } else {
      if p1.x < x && p2.x > x {
         EdgeIntersection::Double(
            Point::new(x, v_intersect(x, p1, p2)),
            Point::new(p2.x, p2.y)
         )
      } else {
         EdgeIntersection::Single(
            Point::new(p2.x, p2.y)
         )
      }
   }
}

#[inline]
pub fn h_intersect(y: i64, p1: Point, p2: Point) -> i64 {
   p1.x + ((p2.x - p1.x) * (y - p1.y)).rounding_div(p2.y - p1.y)
}

#[inline]
pub fn v_intersect(x: i64, p1: Point, p2: Point) -> i64 {
   p1.y + ((p2.y - p1.y) * (x - p1.x)).rounding_div(p2.x - p1.x)
}

#[derive(Debug)]
pub enum EdgeIntersection {
   Double(Point, Point),
   Single(Point),
   None,
}

pub fn hv_clip<F>(intersect: F, y: i64, points: &mut Vec<Point>, start: usize) where
   F: Fn(i64, Point, Point) -> EdgeIntersection,
{
   let end = points.len();

   if end - start < 2 {
      return;
   }

   let mut p1 = points[end-1];

   for i in start..end {
      let p2 = points[i];
      match intersect(y, p1, p2) {
         EdgeIntersection::Double(r1, r2) => {
            points.push(r1);
            points.push(r2);
         },
         EdgeIntersection::Single(r) => {
            points.push(r);
         },
         EdgeIntersection::None => {}
      }

      p1 = p2;
   }
}

#[cfg(test)]
mod tests {
   use test::Bencher;

   use super::super::point::Point;

   use super::*;

   #[bench]
   fn bench_clip(b: &mut Bencher) {
      b.iter(|| {
         for _ in 0..1000 {
            let mut points = vec![
               Point::new(50, 50),
               Point::new(100, 200),
               Point::new(150, 100),
            ];
            points.reserve(30);

            let start = 0;
            let end = points.len();
            hv_clip(h_down_intersect, 150, &mut points, start);

            let start = end;
            let end = points.len();
            hv_clip(h_up_intersect, 80, &mut points, start);

            let start = end;
            let end = points.len();
            hv_clip(v_right_intersect, 70, &mut points, start);

            let start = end;
            hv_clip(v_left_intersect, 140, &mut points, start);
         }
      });
   }

   #[test]
   fn test_clip() {
      let mut points = vec![
         Point::new(50, 50),
         Point::new(100, 200),
         Point::new(150, 100),
      ];

      let start = 0;
      let end = points.len();
      hv_clip(h_down_intersect, 150, &mut points, start);

      let start = end;
      let end = points.len();
      hv_clip(h_up_intersect, 80, &mut points, start);

      let start = end;
      let end = points.len();
      hv_clip(v_right_intersect, 70, &mut points, start);

      let start = end;
      hv_clip(v_left_intersect, 140, &mut points, start);

      assert_eq!(points.len(), 25);
   }
}
