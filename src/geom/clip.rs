use super::point::Point;
use super::number::NumberOps;
use super::ring::Ring;

#[inline]
pub fn h_down_intersect(y: i64, p1: Point, p2: Point, points: &mut Ring<Point>) {
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
pub fn h_up_intersect(y: i64, p1: Point, p2: Point, points: &mut Ring<Point>) {
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
pub fn v_left_intersect(x: i64, p1: Point, p2: Point, points: &mut Ring<Point>) {
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
pub fn v_right_intersect(x: i64, p1: Point, p2: Point, points: &mut Ring<Point>) {
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

pub fn hv_clip<F>(intersect: F, at: i64, points: &mut Ring<Point>) where
   F: Fn(i64, Point, Point, &mut Ring<Point>),
{
   let start = points.start();
   let end = points.end();

   points.rewind(2 * (end - start));

   let rewinded_end = points.end();

   let mut p1 = points[end-1];

   for i in start..end {
      let p2 = points[i];

      intersect(at, p1, p2, points);

      p1 = p2;
   }

   points.consume_at(rewinded_end);
}

pub fn hv_split<F>(split: F, at: i64, write: &mut Ring<Point>, read_write: &mut Ring<Point>) where
   F: Fn(i64, Point, Point, &mut Ring<Point>, &mut Ring<Point>),
{
   let start = read_write.start();
   let end = read_write.end();

   if end - start <= 2 {
      read_write.consume();
      return;
   }

   let double = 2 * (end - start);
   read_write.rewind(double);
   write.rewind(double);

   let rewinded_end = read_write.end();

   let mut p1 = read_write[end-1];

   for i in start..end {
      let p2 = read_write[i];

      split(at, p1, p2, write, read_write);

      p1 = p2;
   }

   read_write.consume_at(rewinded_end);
}

#[inline]
pub fn h_split_edge(y: i64, p1: Point, p2: Point, down: &mut Ring<Point>, up: &mut Ring<Point>) {
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
pub fn v_split_edge(x: i64, p1: Point, p2: Point, left: &mut Ring<Point>, right: &mut Ring<Point>) {
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
   use super::super::ring::Ring;

   use super::*;

   #[bench]
   fn bench_clip(b: &mut Bencher) {
      let mut points = Ring::new(1024 * 256);

      b.iter(|| {
         for _ in 0..1000 {
            points.consume();

            points.push(Point::new(50, 50));
            points.push(Point::new(100, 200));
            points.push(Point::new(150, 100));

            hv_clip(h_down_intersect, 150, &mut points);

            hv_clip(h_up_intersect, 80, &mut points);

            hv_clip(v_right_intersect, 70, &mut points);

            hv_clip(v_left_intersect, 140, &mut points);
         }
      });
   }

   #[test]
   fn test_clip() {
      let mut points = Ring::new(30);

      points.push(Point::new(50, 50));
      points.push(Point::new(100, 200));
      points.push(Point::new(150, 100));

      hv_clip(h_down_intersect, 150, &mut points);

      hv_clip(h_up_intersect, 80, &mut points);

      hv_clip(v_right_intersect, 70, &mut points);

      hv_clip(v_left_intersect, 140, &mut points);

      assert_eq!(points[..].len(), 7);

      assert_eq!(&points[..], [
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
