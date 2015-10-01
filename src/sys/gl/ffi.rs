#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::mem;

use libc::{
   c_int, c_uint, c_float, c_void
};

use sys::utils::fn_ptr::{FnPtr, NULL_PTR, FnPtrLoader};

pub type GLenum = c_uint;
pub type GLint = c_int;
pub type GLuint = c_uint;
pub type GLsizei = c_int;
pub type GLvoid = c_void;
pub type GLbitfield = c_uint;
pub type GLclampf = c_float;

pub const GL_BYTE:                     GLenum = 0x1400;
pub const GL_UNSIGNED_BYTE:            GLenum = 0x1401;
pub const GL_SHORT:                    GLenum = 0x1402;
pub const GL_UNSIGNED_SHORT:           GLenum = 0x1403;
pub const GL_INT:                      GLenum = 0x1404;
pub const GL_UNSIGNED_INT:             GLenum = 0x1405;
pub const GL_FLOAT:                    GLenum = 0x1406;
pub const GL_2_BYTES:                  GLenum = 0x1407;
pub const GL_3_BYTES:                  GLenum = 0x1408;
pub const GL_4_BYTES:                  GLenum = 0x1409;
pub const GL_DOUBLE:                   GLenum = 0x140A;

pub const GL_NO_ERROR:                 GLenum = 0;
pub const GL_INVALID_ENUM:             GLenum = 0x0500;
pub const GL_INVALID_VALUE:            GLenum = 0x0501;
pub const GL_INVALID_OPERATION:        GLenum = 0x0502;
pub const GL_STACK_OVERFLOW:           GLenum = 0x0503;
pub const GL_STACK_UNDERFLOW:          GLenum = 0x0504;
pub const GL_OUT_OF_MEMORY:            GLenum = 0x0505;

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

pub const GL_TEXTURE_1D:               GLenum = 0x0DE0;
pub const GL_TEXTURE_2D:               GLenum = 0x0DE1;
pub const GL_TEXTURE_WRAP_S:           GLenum = 0x2802;
pub const GL_TEXTURE_WRAP_T:           GLenum = 0x2803;
pub const GL_TEXTURE_MAG_FILTER:       GLenum = 0x2800;
pub const GL_TEXTURE_MIN_FILTER:       GLenum = 0x2801;

pub const GL_RGB:                      GLenum = 0x1907;
pub const GL_RGBA:                     GLenum = 0x1908;

pub const GL_READ_FRAMEBUFFER:         GLenum = 0x8CA8;
pub const GL_DRAW_FRAMEBUFFER:         GLenum = 0x8CA9;

pub const GL_COLOR_ATTACHMENT0:        GLenum = 0x8CE0;

pub const GL_CLAMP_TO_EDGE:            GLenum = 0x812F;
pub const GL_NEAREST:                  GLenum = 0x2600;

pub const GL_SCISSOR_TEST:             GLenum = 0x0C11;
pub const GL_CULL_FACE:                GLenum = 0x0B44;
pub const GL_BLEND:                    GLenum = 0x0BE2;
pub const GL_DITHER:                   GLenum = 0x0BD0;
pub const GL_STENCIL_TEST:             GLenum = 0x0B90;
pub const GL_DEPTH_TEST:               GLenum = 0x0B71;
pub const GL_POLYGON_OFFSET_FILL:      GLenum = 0x8037;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E;
pub const GL_SAMPLE_COVERAGE:          GLenum = 0x80A0;

static mut glGenFramebuffersPtr: FnPtr = NULL_PTR;
static mut glDeleteFramebuffersPtr: FnPtr = NULL_PTR;
static mut glBindFramebufferPtr: FnPtr = NULL_PTR;
static mut glFramebufferTexture2DPtr: FnPtr = NULL_PTR;
static mut glBlitFramebufferPtr: FnPtr = NULL_PTR;

pub unsafe fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) {
   mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(glGenFramebuffersPtr)(n, framebuffers)
}

pub unsafe fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) {
   mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(glDeleteFramebuffersPtr)(n, framebuffers)
}

pub unsafe fn glBindFramebuffer(target: GLenum, framebuffer: GLuint) {
   mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(glBindFramebufferPtr)(target, framebuffer)
}

pub unsafe fn glFramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) {
   mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint) -> ()>(glFramebufferTexture2DPtr)(target, attachment, textarget, texture, level)
}

pub unsafe fn glBlitFramebuffer(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) {
   mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLbitfield, GLenum) -> ()>(glBlitFramebufferPtr)(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter)
}

pub unsafe fn load_functions<T: FnPtrLoader>(loader: &T) -> bool {
   glGenFramebuffersPtr = loadfn!(loader, "glGenFramebuffers");
   glDeleteFramebuffersPtr = loadfn!(loader, "glDeleteFramebuffers");
   glBindFramebufferPtr = loadfn!(loader, "glBindFramebuffer");
   glFramebufferTexture2DPtr = loadfn!(loader, "glFramebufferTexture2D");
   glBlitFramebufferPtr = loadfn!(loader, "glBlitFramebuffer");

   true
}

#[cfg_attr(target_os="linux", link(name="GL"))]
#[cfg_attr(target_os="windows", link(name="opengl32"))]
extern "C" {
   pub fn glGetError() -> GLenum;

   pub fn glClear(mask: GLbitfield) -> ();

   pub fn glFlush() -> ();

   pub fn glFinish() -> ();

   pub fn glDisable(cap: GLenum) -> ();

   pub fn glPixelStorei(pname: GLenum, param: GLint) -> ();

   pub fn glGenTextures(n: GLsizei, textures: *mut GLuint) -> ();

   pub fn glDeleteTextures(n: GLsizei, textures: *const GLuint) -> ();

   pub fn glBindTexture(target: GLenum, texture: GLuint) -> ();

   pub fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint) -> ();

   pub fn glClearColor(
      red: GLclampf,
      green: GLclampf,
      blue: GLclampf,
      alpha: GLclampf
   ) -> ();

   pub fn glTexImage2D(
      target: GLenum,
      level: GLint,
      internalFormat: GLint,
      width: GLsizei,
      height: GLsizei,
      border: GLint,
      format: GLenum,
      _type: GLenum,
      pixels: *const GLvoid
   ) -> ();

   pub fn glTexSubImage2D(
      target: GLenum,
      level: GLint,
      xoffset: GLint,
      yoffset: GLint,
      width: GLsizei,
      height: GLsizei,
      format: GLenum,
      _type: GLenum,
      pixels: *const GLvoid
   ) -> ();
}
