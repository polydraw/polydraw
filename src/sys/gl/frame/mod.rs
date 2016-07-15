pub mod buffer;
pub mod quad;
pub mod draw_pixels;

use error::{RuntimeError, ErrorKind};
use frame::GPUFrame;

use super::{has_buffer_functions, has_quad_functions, has_pixel_functions};

#[inline]
pub fn create_gpu_frame(
   width: u32, height: u32
) -> Result<Box<GPUFrame>, RuntimeError> {
   match create_buffer_frame(width, height) {
      Ok(result) => { return Ok(result); },
      Err(_) => {}
   }

   match create_quad_frame(width, height) {
      Ok(result) => { return Ok(result); },
      Err(_) => {}
   }

   match create_pixel_frame(width, height) {
      Ok(result) => { return Ok(result); },
      Err(_) => {}
   }

   Err(RuntimeError::new(
      ErrorKind::GL,
      "Cannot create GL frame for drawing".to_string()
   ))
}

#[inline]
fn create_buffer_frame(
   width: u32, height: u32
) -> Result<Box<GPUFrame>, RuntimeError> {
   if !has_buffer_functions() {
      return Err(RuntimeError::new(
         ErrorKind::GL,
         "Unavailable functions for instancing GL buffer frame".to_string()
      ));
   }

   match buffer::BufferFrame::new(width, height) {
      Ok(gpu_frame) => Ok(Box::new(gpu_frame)),
      Err(e) => Err(e)
   }
}

#[inline]
fn create_quad_frame(
   width: u32, height: u32
) -> Result<Box<GPUFrame>, RuntimeError> {
   if !has_quad_functions() {
      return Err(RuntimeError::new(
         ErrorKind::GL,
         "Unavailable functions for instancing GL quad frame".to_string()
      ));
   }

   match quad::QuadFrame::new(width, height) {
      Ok(gpu_frame) => Ok(Box::new(gpu_frame)),
      Err(e) => Err(e)
   }
}

#[inline]
fn create_pixel_frame(
   width: u32, height: u32
) -> Result<Box<GPUFrame>, RuntimeError> {
   if !has_pixel_functions() {
      return Err(RuntimeError::new(
         ErrorKind::GL,
         "Unavailable functions for instancing GL draw pixels frame".to_string()
      ));
   }

   match draw_pixels::DrawPixelsFrame::new(width, height) {
      Ok(gpu_frame) => Ok(Box::new(gpu_frame)),
      Err(e) => Err(e)
   }
}
