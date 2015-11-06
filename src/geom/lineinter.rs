use std::i64;

use super::number::NumberOps;
use super::point::Point;
use super::ring::Ring;

pub const HALF_MAX_ERR: i64  = i64::MAX / 2;

#[inline]
pub fn h_multi_intersect(p1: Point, p2: Point, step_y: i64, inters: &mut Ring<i64>) {
   let (p1, p2) = if p1.y > p2.y {
      (p2, p1)
   } else {
      (p1, p2)
   };

   let start = 1 + p1.y / step_y;
   let end = 1 + (p2.y - 1) / step_y;

   for i in start..end {
      let x = h_intersect(p1, p2, i * step_y);
      inters.push(x);
   }
}

#[inline]
pub fn v_multi_intersect(p1: Point, p2: Point, step_x: i64, inters: &mut Ring<i64>) {
   let (p1, p2) = if p1.x > p2.x {
      (p2, p1)
   } else {
      (p1, p2)
   };

   let start = 1 + p1.x / step_x;
   let end = 1 + (p2.x - 1) / step_x;

   for i in start..end {
      let y = v_intersect(p1, p2, i * step_x);
      inters.push(y);
   }
}

#[inline]
pub fn h_multi_intersect_fast(p1: Point, p2: Point, step_y: i64, inters: &mut Ring<i64>) -> i64 {
   let (p1, p2) = if p1.y > p2.y {
      (p2, p1)
   } else {
      (p1, p2)
   };

   let start = 1 + p1.y / step_y;
   let end = 1 + (p2.y - 1) / step_y;

   let dy = p2.y - p1.y;
   let dx = p2.x - p1.x;
   let dx_signum = dx.signum();

   let step_x = dx * step_y / dy;

   let max_div_dy = i64::MAX / dy;

   let err_step = max_div_dy * (step_y * dx * dx_signum - step_x * dx_signum * dy);

   let first_y = start * step_y;

   let fdy = first_y - p1.y;
   let fdx = dx * fdy / dy;

   let mut x = p1.x + fdx;

   if err_step == 0 {
      for _ in start..end {
         inters.push(x);

         x += step_x;
      }

      return first_y;
   }

   let mut err = max_div_dy * (fdy * dx * dx_signum - fdx * dx_signum * dy) - HALF_MAX_ERR;

   for _ in start..end {
      if err > 0 {
         x += dx_signum;
         err -= i64::MAX;
      }

      inters.push(x);

      x += step_x;

      err += err_step;
   }

   first_y
}

#[inline]
pub fn v_multi_intersect_fast(p1: Point, p2: Point, step_x: i64, inters: &mut Ring<i64>) -> i64 {
   let (p1, p2) = if p1.x > p2.x {
      (p2, p1)
   } else {
      (p1, p2)
   };

   let start = 1 + p1.x / step_x;
   let end = 1 + (p2.x - 1) / step_x;

   let dx = p2.x - p1.x;
   let dy = p2.y - p1.y;
   let dy_signum = dy.signum();

   let step_y = dy * step_x / dx;

   let max_div_dx = i64::MAX / dx;

   let err_step = max_div_dx * (step_x * dy * dy_signum - step_y * dy_signum * dx);

   let first_x = start * step_x;

   let fdx = first_x - p1.x;
   let fdy = dy * fdx / dx;

   let mut y = p1.y + fdy;

   if err_step == 0 {
      for _ in start..end {
         inters.push(y);

         y += step_y;
      }

      return first_x;
   }

   let mut err = max_div_dx * (fdx * dy * dy_signum - fdy * dy_signum * dx) - HALF_MAX_ERR;

   for _ in start..end {
      if err > 0 {
         y += dy_signum;
         err -= i64::MAX;
      }

      inters.push(y);

      y += step_y;

      err += err_step;
   }

   first_x
}

#[inline]
pub fn h_intersect(p1: Point, p2: Point, y: i64) -> i64 {
   p1.x + ((p2.x - p1.x) * (y - p1.y)).rounding_div(p2.y - p1.y)
}

#[inline]
pub fn v_intersect(p1: Point, p2: Point, x: i64) -> i64 {
   p1.y + ((p2.y - p1.y) * (x - p1.x)).rounding_div(p2.x - p1.x)
}


#[cfg(test)]
mod tests {
   use test::Bencher;

   use super::super::point::Point;
   use super::super::ring::Ring;

   use super::*;

   #[test]
   fn test_correct_h() {
      let mut inters = Ring::new(100_000);
      let mut inters_fast = Ring::new(100_000);
      let p1 = Point::new(0, 0);
      let p2 = Point::new(500_000, 2_000_000);

      h_multi_intersect(p1, p2, 1000, &mut inters);
      h_multi_intersect_fast(p1, p2, 1000, &mut inters_fast);

      for (correct, fast) in inters[..].iter().zip(inters_fast[..].iter()) {
         assert_eq!(correct, fast);
      }
   }

   #[test]
   fn test_correct_h_rev() {
      let mut inters = Ring::new(100_000);
      let mut inters_fast = Ring::new(100_000);
      let p1 = Point::new(500_000, 0);
      let p2 = Point::new(0, 2_000_000);

      h_multi_intersect(p1, p2, 1000, &mut inters);
      h_multi_intersect_fast(p1, p2, 1000, &mut inters_fast);

      for (correct, fast) in inters[..].iter().zip(inters_fast[..].iter()) {
         assert_eq!(correct, fast);
      }
   }

   #[test]
   fn test_correct_v() {
      let mut inters = Ring::new(100_000);
      let mut inters_fast = Ring::new(100_000);
      let p1 = Point::new(0, 0);
      let p2 = Point::new(500_000, 2_000_000);

      v_multi_intersect(p1, p2, 1000, &mut inters);
      v_multi_intersect_fast(p1, p2, 1000, &mut inters_fast);

      for (correct, fast) in inters[..].iter().zip(inters_fast[..].iter()) {
         assert_eq!(correct, fast);
      }
   }

   #[test]
   fn test_correct_v_rev() {
      let mut inters = Ring::new(100_000);
      let mut inters_fast = Ring::new(100_000);
      let p1 = Point::new(500_000, 0);
      let p2 = Point::new(0, 2_000_000);

      v_multi_intersect(p1, p2, 1000, &mut inters);
      v_multi_intersect_fast(p1, p2, 1000, &mut inters_fast);

      for (correct, fast) in inters[..].iter().zip(inters_fast[..].iter()) {
         assert_eq!(correct, fast);
      }
   }

   #[bench]
   fn bench_lineinter(b: &mut Bencher) {
      let mut inters = Ring::new(100_000);
      let p1 = Point::new(0, 0);
      let p2 = Point::new(500_000, 2_000_000);

      b.iter(|| {
         for _ in 0..1000 {
            inters.clear();
            h_multi_intersect(p1, p2, 1000, &mut inters);
         }
      });
   }

   #[bench]
   fn bench_lineinter_fast(b: &mut Bencher) {
      let mut inters = Ring::new(100_000);
      let p1 = Point::new(0, 0);
      let p2 = Point::new(500_000, 2_000_000);

      b.iter(|| {
         for _ in 0..1000 {
            inters.clear();
            h_multi_intersect_fast(p1, p2, 1000, &mut inters);
         }
      });
   }
}
