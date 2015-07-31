#![cfg(target_os = "linux")]

pub mod ffi {
   #![allow(non_camel_case_types)]

   use libc::{
      c_int, c_uint, c_float
   };

   pub type GLenum = c_uint;
   pub type GLint = c_int;
   pub type GLbitfield = c_uint;
   pub type GLclampf = c_float;

   pub const GL_CURRENT_BIT:              GLenum = 0x00000001;
   pub const GL_POINT_BIT:                GLenum = 0x00000002;
   pub const GL_LINE_BIT:                 GLenum = 0x00000004;
   pub const GL_POLYGON_BIT:              GLenum = 0x00000008;
   pub const GL_POLYGON_STIPPLE_BIT:      GLenum = 0x00000010;
   pub const GL_PIXEL_MODE_BIT:           GLenum = 0x00000020;
   pub const GL_LIGHTING_BIT:             GLenum = 0x00000040;
   pub const GL_FOG_BIT:                  GLenum = 0x00000080;
   pub const GL_DEPTH_BUFFER_BIT:         GLenum = 0x00000100;
   pub const GL_ACCUM_BUFFER_BIT:         GLenum = 0x00000200;
   pub const GL_STENCIL_BUFFER_BIT:       GLenum = 0x00000400;
   pub const GL_VIEWPORT_BIT:             GLenum = 0x00000800;
   pub const GL_TRANSFORM_BIT:            GLenum = 0x00001000;
   pub const GL_ENABLE_BIT:               GLenum = 0x00002000;
   pub const GL_COLOR_BUFFER_BIT:         GLenum = 0x00004000;
   pub const GL_HINT_BIT:                 GLenum = 0x00008000;
   pub const GL_EVAL_BIT:                 GLenum = 0x00010000;
   pub const GL_LIST_BIT:                 GLenum = 0x00020000;
   pub const GL_TEXTURE_BIT:              GLenum = 0x00040000;
   pub const GL_SCISSOR_BIT:              GLenum = 0x00080000;
   pub const GL_ALL_ATTRIB_BITS:          GLenum = 0xFFFFFFFF;

   pub const GL_MAP_COLOR:                GLenum = 0x0D10;
   pub const GL_MAP_STENCIL:              GLenum = 0x0D11;
   pub const GL_INDEX_SHIFT:              GLenum = 0x0D12;
   pub const GL_INDEX_OFFSET:             GLenum = 0x0D13;
   pub const GL_RED_SCALE:                GLenum = 0x0D14;
   pub const GL_RED_BIAS:                 GLenum = 0x0D15;
   pub const GL_GREEN_SCALE:              GLenum = 0x0D18;
   pub const GL_GREEN_BIAS:               GLenum = 0x0D19;
   pub const GL_BLUE_SCALE:               GLenum = 0x0D1A;
   pub const GL_BLUE_BIAS:                GLenum = 0x0D1B;
   pub const GL_ALPHA_SCALE:              GLenum = 0x0D1C;
   pub const GL_ALPHA_BIAS:               GLenum = 0x0D1D;
   pub const GL_DEPTH_SCALE:              GLenum = 0x0D1E;
   pub const GL_DEPTH_BIAS:               GLenum = 0x0D1F;
   pub const GL_PIXEL_MAP_S_TO_S_SIZE:    GLenum = 0x0CB1;
   pub const GL_PIXEL_MAP_I_TO_I_SIZE:    GLenum = 0x0CB0;
   pub const GL_PIXEL_MAP_I_TO_R_SIZE:    GLenum = 0x0CB2;
   pub const GL_PIXEL_MAP_I_TO_G_SIZE:    GLenum = 0x0CB3;
   pub const GL_PIXEL_MAP_I_TO_B_SIZE:    GLenum = 0x0CB4;
   pub const GL_PIXEL_MAP_I_TO_A_SIZE:    GLenum = 0x0CB5;
   pub const GL_PIXEL_MAP_R_TO_R_SIZE:    GLenum = 0x0CB6;
   pub const GL_PIXEL_MAP_G_TO_G_SIZE:    GLenum = 0x0CB7;
   pub const GL_PIXEL_MAP_B_TO_B_SIZE:    GLenum = 0x0CB8;
   pub const GL_PIXEL_MAP_A_TO_A_SIZE:    GLenum = 0x0CB9;
   pub const GL_PIXEL_MAP_S_TO_S:         GLenum = 0x0C71;
   pub const GL_PIXEL_MAP_I_TO_I:         GLenum = 0x0C70;
   pub const GL_PIXEL_MAP_I_TO_R:         GLenum = 0x0C72;
   pub const GL_PIXEL_MAP_I_TO_G:         GLenum = 0x0C73;
   pub const GL_PIXEL_MAP_I_TO_B:         GLenum = 0x0C74;
   pub const GL_PIXEL_MAP_I_TO_A:         GLenum = 0x0C75;
   pub const GL_PIXEL_MAP_R_TO_R:         GLenum = 0x0C76;
   pub const GL_PIXEL_MAP_G_TO_G:         GLenum = 0x0C77;
   pub const GL_PIXEL_MAP_B_TO_B:         GLenum = 0x0C78;
   pub const GL_PIXEL_MAP_A_TO_A:         GLenum = 0x0C79;
   pub const GL_PACK_ALIGNMENT:           GLenum = 0x0D05;
   pub const GL_PACK_LSB_FIRST:           GLenum = 0x0D01;
   pub const GL_PACK_ROW_LENGTH:          GLenum = 0x0D02;
   pub const GL_PACK_SKIP_PIXELS:         GLenum = 0x0D04;
   pub const GL_PACK_SKIP_ROWS:           GLenum = 0x0D03;
   pub const GL_PACK_SWAP_BYTES:          GLenum = 0x0D00;
   pub const GL_UNPACK_ALIGNMENT:         GLenum = 0x0CF5;
   pub const GL_UNPACK_LSB_FIRST:         GLenum = 0x0CF1;
   pub const GL_UNPACK_ROW_LENGTH:        GLenum = 0x0CF2;
   pub const GL_UNPACK_SKIP_PIXELS:       GLenum = 0x0CF4;
   pub const GL_UNPACK_SKIP_ROWS:         GLenum = 0x0CF3;
   pub const GL_UNPACK_SWAP_BYTES:        GLenum = 0x0CF0;
   pub const GL_ZOOM_X:                   GLenum = 0x0D16;
   pub const GL_ZOOM_Y:                   GLenum = 0x0D17;

   #[link(name="GL")]
   extern "C" {
      pub fn glClearColor(
         red: GLclampf,
         green: GLclampf,
         blue: GLclampf,
         alpha: GLclampf
      ) -> ();

      pub fn glClear(
         mask: GLbitfield
      ) -> ();

      pub fn glFlush() -> ();

      pub fn glPixelStorei(pname: GLenum, param: GLint) -> ();
   }
}

#[inline]
pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
   unsafe {
      ffi::glClearColor(red, green, blue, alpha);
   }
}

#[inline]
pub fn clear() {
   unsafe {
      ffi::glClear(ffi::GL_COLOR_BUFFER_BIT);
   }
}

#[inline]
pub fn flush() {
   unsafe {
      ffi::glFlush();
   }
}

#[inline]
pub fn reset_pixelstore_unpack_alignment() {
   unsafe {
      ffi::glPixelStorei(ffi::GL_UNPACK_ALIGNMENT, 1);
   }
}
