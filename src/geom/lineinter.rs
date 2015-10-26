use super::number::NumberOps;
use super::point::Point;
use super::ring::Ring;

#[inline]
pub fn h_multi_intersect(p1: Point, p2: Point, step: i64, inters: &mut Ring<i64>)  {
   let start = 1 + p1.y / step;
   let end = 1 + (p2.y - 1) / step;

   for i in start..end {
      let x = h_intersect(p1, p2, i * step);
      inters.push(x);
   }
}

#[inline]
pub fn h_multi_intersect_fast(p1: Point, p2: Point, step: i64, inters: &mut Ring<i64>)  {
   let start = 1 + p1.y / step;
   let end = 1 + (p2.y - 1) / step;

   let dy = p2.y - p1.y;
   let dx = p2.x - p1.x;

   let step_x = dx * step / dy;

   let mut x = p1.x + (dx * (start * step - p1.y)).rounding_div(dy);
   inters.push(x);

   for i in start+1..end {
      x += step_x;
      inters.push(x);
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

   #[bench]
   fn bench_lineinter_1(b: &mut Bencher) {
      let mut inters = Ring::new(10_000);
      let p1 = Point::new(2135, 2476);
      let p2 = Point::new(16753, 72398);

      b.iter(|| {
         for _ in 0..1000 {
            inters.rewind(100);
            h_multi_intersect(p1, p2, 1000, &mut inters);
            inters.consume();
         }
      });
   }

   #[bench]
   fn bench_lineinter_2(b: &mut Bencher) {
      let mut inters = Ring::new(10_000);
      let p1 = Point::new(2135, 2476);
      let p2 = Point::new(16753, 72398);

      b.iter(|| {
         for _ in 0..1000 {
            inters.rewind(100);
            h_multi_intersect_fast(p1, p2, 1000, &mut inters);
            inters.consume();
         }
      });
   }
}
