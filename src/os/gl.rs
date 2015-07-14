pub mod ffi {
   #![allow(non_camel_case_types)]

   use libc::{
      c_uint, c_float
   };

   pub type GLenum = c_uint;
   pub type GLbitfield = c_uint;
   pub type GLclampf = c_float;

   pub const GL_CURRENT_BIT:          GLenum = 0x00000001;
   pub const GL_POINT_BIT:            GLenum = 0x00000002;
   pub const GL_LINE_BIT:             GLenum = 0x00000004;
   pub const GL_POLYGON_BIT:          GLenum = 0x00000008;
   pub const GL_POLYGON_STIPPLE_BIT:  GLenum = 0x00000010;
   pub const GL_PIXEL_MODE_BIT:       GLenum = 0x00000020;
   pub const GL_LIGHTING_BIT:         GLenum = 0x00000040;
   pub const GL_FOG_BIT:              GLenum = 0x00000080;
   pub const GL_DEPTH_BUFFER_BIT:     GLenum = 0x00000100;
   pub const GL_ACCUM_BUFFER_BIT:     GLenum = 0x00000200;
   pub const GL_STENCIL_BUFFER_BIT:   GLenum = 0x00000400;
   pub const GL_VIEWPORT_BIT:         GLenum = 0x00000800;
   pub const GL_TRANSFORM_BIT:        GLenum = 0x00001000;
   pub const GL_ENABLE_BIT:           GLenum = 0x00002000;
   pub const GL_COLOR_BUFFER_BIT:     GLenum = 0x00004000;
   pub const GL_HINT_BIT:             GLenum = 0x00008000;
   pub const GL_EVAL_BIT:             GLenum = 0x00010000;
   pub const GL_LIST_BIT:             GLenum = 0x00020000;
   pub const GL_TEXTURE_BIT:          GLenum = 0x00040000;
   pub const GL_SCISSOR_BIT:          GLenum = 0x00080000;
   pub const GL_ALL_ATTRIB_BITS:      GLenum = 0xFFFFFFFF;

   #[link(name="GL")]
   extern "C" {
      pub fn glClearColor(
         red: GLclampf,
         green: GLclampf,
         blue: GLclampf,
         alpha:
         GLclampf
      ) -> ();

      pub fn glClear(
         mask: GLbitfield
      ) -> ();

      pub fn glFlush() -> ();
   }
}
