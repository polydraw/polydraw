use std::iter::repeat;

pub fn rand_u8(seed: &mut u64) -> u8 {
    *seed = seed.wrapping_mul(58321).wrapping_add(11113);
    (seed.wrapping_shr(16) % 256) as u8
}

pub fn create_data(width: usize, height: usize) -> Vec<u8> {
   repeat(0u8)
      .take(width * height * 3)
      .collect::<Vec<_>>()
}

pub fn update_data(data: &mut Vec<u8>, width: usize, height: usize, seed: &mut u64) {
   for y in 0..height {
      for x in 0..width {
         let i = 3 * (x + y * width);
         let r = rand_u8(seed);
         data[i] = r;
         data[i + 1] = r;
         data[i + 2] = r;
      }
   }
}
