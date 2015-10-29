use std::i64;

use super::number::NumberOps;
use super::point::Point;
use super::ring::Ring;

pub const HALF_MAX_ERR: i64  = i64::MAX / 2;

#[inline]
pub fn h_multi_intersect(p1: Point, p2: Point, step_y: i64, inters: &mut Ring<i64>) {
   let start = 1 + p1.y / step_y;
   let end = 1 + (p2.y - 1) / step_y;

   for i in start..end {
      let x = h_intersect(p1, p2, i * step_y);
      inters.push(x);
   }
}

#[inline]
pub fn h_multi_intersect_fast(p1: Point, p2: Point, step_y: i64, inters: &mut Ring<i64>) {
   let start = 1 + p1.y / step_y;
   let end = 1 + (p2.y - 1) / step_y;

   let dy = p2.y - p1.y;
   let dx = p2.x - p1.x;
   let dx_signum = dx.signum();

   let step_x = dx * step_y / dy;

   let max_div_dy = i64::MAX / dy;

   let err_step = max_div_dy * (step_y * dx.abs() - step_x * dy);

   let fdy = start * step_y - p1.y;
   let fdx = dx * fdy / dy;

   let mut x = p1.x + fdx;

   let mut err = max_div_dy * (fdy * dx.abs() - fdx * dy) - HALF_MAX_ERR;

   for _ in start..end {
      if err > 0 {
         x += dx_signum;
         err -= i64::MAX;
      }

      inters.push(x);

      x += step_x;

      err += err_step;
   }
}

#[inline]
fn h_intersect(p1: Point, p2: Point, y: i64) -> i64 {
   p1.x + ((p2.x - p1.x) * (y - p1.y)).rounding_div(p2.y - p1.y)
}


#[cfg(test)]
mod tests {
   use test::Bencher;

   use super::super::point::Point;
   use super::super::ring::Ring;

   use super::*;

   #[test]
   fn bench_correct() {
      let mut inters = Ring::new(100_000);
      let mut inters_fast = Ring::new(100_000);
      let p1 = Point::new(2_135, 2_476);
      let p2 = Point::new(16_753, 1_534_398);

      h_multi_intersect(p1, p2, 1000, &mut inters);
      h_multi_intersect_fast(p1, p2, 1000, &mut inters_fast);

      for (correct, fast) in inters[..].iter().zip(inters_fast[..].iter()) {
         assert_eq!(correct, fast);
      }
   }

   #[bench]
   fn bench_lineinter_1(b: &mut Bencher) {
      let mut inters = Ring::new(100_000);
      let p1 = Point::new(2_135, 2_476);
      let p2 = Point::new(16_753, 1_534_398);

      b.iter(|| {
         for _ in 0..1000 {
            inters.clear();
            h_multi_intersect(p1, p2, 1000, &mut inters);
         }
      });
   }

   #[bench]
   fn bench_lineinter_2(b: &mut Bencher) {
      let mut inters = Ring::new(100_000);
      let p1 = Point::new(2_135, 2_476);
      let p2 = Point::new(16_753, 1_534_398);

      b.iter(|| {
         for _ in 0..1000 {
            inters.clear();
            h_multi_intersect_fast(p1, p2, 1000, &mut inters);
         }
      });
   }
}
