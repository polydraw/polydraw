pub mod buffer;
pub mod quad;

use error::RuntimeError;
use frame::GPUFrame;

use super::ffi;

#[inline]
pub fn create_gpu_frame(
   width: u32, height: u32
) -> Result<Box<GPUFrame>, RuntimeError> {

   if ffi::has_gen_buffers() {
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
