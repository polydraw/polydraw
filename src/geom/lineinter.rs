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

   // println!("START  : {}", start);
   // println!("END    : {}", end);

   let dy = p2.y - p1.y;
   let dx = p2.x - p1.x;

   let step_x = dx * step_y / dy;

   // println!("STEP X : {}", step_x);

   let max_div_dy = i64::MAX / dy;

   // println!("MAX/DY : {}", max_div_dy);

   let err_step = max_div_dy * (step_y * dx - step_x * dy);

   // println!("ERR ST : {}", err_step);

   // let float_err_step = err_step as f64 / i64::MAX as f64;

   // println!("ERR SF : {}", float_err_step);

   let first_y = start * step_y;

   // println!("FIRS Y : {}", first_y);

   let fdy = first_y - p1.y;
   let fdx = dx * fdy / dy;

   // println!("FDY Y  : {}", fdy);
   // println!("FDY X  : {}", fdx);

   let mut x = p1.x + fdx;

   // println!("X      : {}", x);

   let mut err = max_div_dy * (fdy * dx - fdx * dy) - HALF_MAX_ERR;

   // println!("ERR    : {}", err);

   // println!("X      : {}", x);

   // println!("MAX    : {}", i64::MAX);

   // println!("HALF M : {}", HALF_MAX_ERR);

   for _ in start..end {
      // let split_y = i * step_y;
      // println!("SPLI-Y : {}", split_y);

      if err > 0 {
         x += 1;
         err -= i64::MAX;
      }

      // println!("PUSH   : {}", x);
      inters.push(x);


      // let correct_x = h_intersect(p1, p2, split_y);

      // println!("CORR X : {}", correct_x);

      // if correct_x != x {
      //    println!("p1 = {:?}, p2 = {:?}, slit_y = {:?}", p1, p2, split_y);
      //    panic!("NOT EQUAL");
      // }


      x += step_x;

      // println!("X      : {}", x);

      err += err_step;

      // println!("ERR    : {}", err);

      // let float_err = err as f64 / i64::MAX as f64;

      // println!("FL ERR : {}", float_err);
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
