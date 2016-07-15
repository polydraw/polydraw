pub mod buffer;
pub mod quad;
pub mod draw_pixels;

use error::RuntimeError;
use frame::GPUFrame;

use super::{has_buffer_functions, GLES2};

#[inline]
pub fn create_gpu_frame(
   width: u32, height: u32
) -> Result<Box<GPUFrame>, RuntimeError> {

   if has_buffer_functions() && !GLES2 {
      create_buffer_frame(width, height)
   } else {
      create_quad_frame(width, height)
   }
}

#[inline]
fn create_buffer_frame(
   width: u32, height: u32
) -> Result<Box<GPUFrame>, RuntimeError> {

   match buffer::BufferFrame::new(width, height) {
      Ok(gpu_frame) => Ok(Box::new(gpu_frame)),
      Err(e) => Err(e)
   }
}

#[inline]
fn create_quad_frame(
   width: u32, height: u32
) -> Result<Box<GPUFrame>, RuntimeError> {

   match quad::QuadFrame::new(width, height) {
      Ok(gpu_frame) => Ok(Box::new(gpu_frame)),
      Err(e) => Err(e)
   }
}

#[allow(dead_code)]
#[inline]
fn create_pixel_frame(
   width: u32, height: u32
) -> Result<Box<GPUFrame>, RuntimeError> {

   match draw_pixels::DrawPixelsFrame::new(width, height) {
      Ok(gpu_frame) => Ok(Box::new(gpu_frame)),
      Err(e) => Err(e)
   }
}
