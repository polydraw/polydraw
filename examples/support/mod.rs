use std::iter::repeat;

pub fn rand_u8(seed: &mut u64) -> u8 {
    *seed = seed.wrapping_mul(58321).wrapping_add(11113);
    (seed.wrapping_shr(16) % 256) as u8
}

#[allow(dead_code)]
pub fn create_data(width: u32, height: u32) -> Vec<u8> {
   repeat(0u8)
      .take((width * height * 3) as usize)
      .collect::<Vec<_>>()
}

pub fn update_data(data: &mut Vec<u8>, width: u32, height: u32, seed: &mut u64) {
   for y in 0..height {
      for x in 0..width {
         let i: usize = (3 * (x + y * width)) as usize;
         let r = rand_u8(seed);
         data[i] = r;
         data[i + 1] = r;
         data[i + 2] = r;
      }
   }
}
