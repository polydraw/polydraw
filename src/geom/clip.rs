use super::point::Point;
use super::number::NumberOps;

#[inline]
pub fn h_down_intersect(y: i64, p1: Point, p2: Point, points: &mut Vec<Point>) {
   if p2.y > y {
      if p1.y < y {
         points.push(Point::new(h_intersect(y, p1, p2), y));
      }
   } else {
      if p1.y > y && p2.y < y {
         points.push(Point::new(h_intersect(y, p1, p2), y));
      }
      points.push(p2);
   }
}

#[inline]
pub fn h_up_intersect(y: i64, p1: Point, p2: Point, points: &mut Vec<Point>) {
   if p2.y < y {
      if p1.y > y {
         points.push(Point::new(h_intersect(y, p1, p2), y));
      }
   } else {
      if p1.y < y && p2.y > y {
         points.push(Point::new(h_intersect(y, p1, p2), y));
      }
      points.push(p2);
   }
}

#[inline]
pub fn v_left_intersect(x: i64, p1: Point, p2: Point, points: &mut Vec<Point>) {
   if p2.x > x {
      if p1.x < x {
         points.push(Point::new(x, v_intersect(x, p1, p2)));
      }
   } else {
      if p1.x > x && p2.x < x {
         points.push(Point::new(x, v_intersect(x, p1, p2)));
      }
      points.push(p2);
   }
}

#[inline]
pub fn v_right_intersect(x: i64, p1: Point, p2: Point, points: &mut Vec<Point>) {
   if p2.x < x {
      if p1.x > x {
         points.push(Point::new(x, v_intersect(x, p1, p2)));
      }
   } else {
      if p1.x < x && p2.x > x {
         points.push(Point::new(x, v_intersect(x, p1, p2)));
      }
      points.push(p2);
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

pub fn hv_clip<F>(intersect: F, y: i64, points: &mut Vec<Point>, start: usize) where
   F: Fn(i64, Point, Point, &mut Vec<Point>),
{
   let end = points.len();

   if end - start < 2 {
      return;
   }

   let mut p1 = points[end-1];

   for i in start..end {
      let p2 = points[i];
      intersect(y, p1, p2, points);

      p1 = p2;
   }
}

pub fn h_split(y: i64, start: usize, down: &mut Vec<Point>, up: &mut Vec<Point>) {
   let end = up.len();

   assert!(end - start > 2);

   let mut p1 = up[end-1];

   for i in start..end {
      let p2 = up[i];

      h_split_edge(y, p1, p2, down, up);

      p1 = p2;
   }
}

pub fn v_split(x: i64, start: usize, left: &mut Vec<Point>, right: &mut Vec<Point>) {
   let end = right.len();

   assert!(end - start > 2);

   let mut p1 = right[end-1];

   for i in start..end {
      let p2 = right[i];

      v_split_edge(x, p1, p2, left, right);

      p1 = p2;
   }
}

#[inline]
pub fn h_split_edge(y: i64, p1: Point, p2: Point, down: &mut Vec<Point>, up: &mut Vec<Point>) {
   if p2.y > y {
      if p1.y < y {
         let intersection = Point::new(h_intersect(y, p1, p2), y);
         down.push(intersection);
         up.push(intersection);
      }
      up.push(p2);
   } else if p2.y < y {
      if p1.y > y {
         let intersection = Point::new(h_intersect(y, p1, p2), y);
         down.push(intersection);
         up.push(intersection);
      }
      down.push(p2);
   } else {
      down.push(p2);
      up.push(p2);
   }
}

#[inline]
pub fn v_split_edge(x: i64, p1: Point, p2: Point, left: &mut Vec<Point>, right: &mut Vec<Point>) {
   if p2.x > x {
      if p1.x < x {
         let intersection = Point::new(x, v_intersect(x, p1, p2));
         left.push(intersection);
         right.push(intersection);
      }
      right.push(p2);
   } else if p2.x < x {
      if p1.x > x {
         let intersection = Point::new(x, v_intersect(x, p1, p2));
         left.push(intersection);
         right.push(intersection);
      }
      left.push(p2);
   } else {
      left.push(p2);
      right.push(p2);
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
      let end = points.len();
      hv_clip(v_left_intersect, 140, &mut points, start);

      assert_eq!(points.len(), 25);

      let (_, result) = points.split_at(end);
      assert_eq!(result, [
         Point::new(140, 95),
         Point::new(110, 80),
         Point::new(70, 80),
         Point::new(70, 110),
         Point::new(83, 150),
         Point::new(125, 150),
         Point::new(140, 120),
      ]);
   }
}
