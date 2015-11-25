use std::cmp::{min, max};

use super::frame::Frame;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RGB {
   pub r: u8,
   pub g: u8,
   pub b: u8
}

impl RGB {
   pub fn new(r: u8, g: u8, b: u8) -> Self {
      RGB {
         r: r,
         g: g,
         b: b
      }
   }
}

impl Default for RGB {
   fn default() -> RGB {
      RGB::new(0, 0, 0)
   }
}

pub fn bresenham(frame: &mut Frame, x1: i32, y1: i32, x2: i32, y2: i32, color: &RGB) {
   let dx = x2 - x1;
   let dy = y2 - y1;

   let ix = ((dx > 0) as i32) - ((dx < 0) as i32);
   let iy = ((dy > 0) as i32) - ((dy < 0) as i32);

   let dabs2x = dx.abs() * 2;
   let dabs2y = dy.abs() * 2;

   let (mut x, mut y) = (x1, y1);

   frame.put_pixel(x, y, color);

   if dabs2x >= dabs2y {
      let mut error = dabs2y - (dabs2x / 2);

      while x != x2 {
         if error >= 0 && ((error != 0) || (ix > 0)) {
            error -= dabs2x;
            y += iy;
         }

         error += dabs2y;
         x += ix;

         frame.put_pixel(x, y, color);
      }
   } else {
      let mut error = dabs2x - (dabs2y / 2);

      while y != y2 {
         if error >= 0 && ((error != 0) || (iy > 0)) {
            error -= dabs2y;
            x += ix;
         }

         error += dabs2x;
         y += iy;

         frame.put_pixel(x, y, color);
      }
   }
}

pub fn hline(frame: &mut Frame, x1: i32, x2: i32, y: i32, color: &RGB) {
   let min_x = min(x1, x2);
   let max_x = max(x1, x2);

   for x in min_x..max_x + 1 {
      frame.put_pixel(x, y, color);
   }
}

pub fn vline(frame: &mut Frame, x: i32, y1: i32, y2: i32, color: &RGB) {
   let min_y = min(y1, y2);
   let max_y = max(y1, y2);

   for y in min_y..max_y + 1 {
      frame.put_pixel(x, y, color);
   }
}
