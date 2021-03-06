#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::mem;

pub use libc::{
   c_char, c_uchar, c_int, c_uint, c_float, c_double, c_void, ptrdiff_t
};

use sys::utils::fn_ptr::{FnPtr, NULL_PTR, FnPtrLoader};

pub type GLenum = c_uint;
pub type GLchar = c_char;
pub type GLubyte = c_uchar;
pub type GLint = c_int;
pub type GLuint = c_uint;
pub type GLsizei = c_int;
pub type GLvoid = c_void;
pub type GLbitfield = c_uint;
pub type GLclampf = c_float;
pub type GLintptr = ptrdiff_t;
pub type GLsizeiptr = ptrdiff_t;
pub type GLboolean = c_uchar;
pub type GLfloat = c_float;
pub type GLdouble = c_double;

pub const GL_TRUE:                           GLboolean = 1;
pub const GL_FALSE:                          GLboolean = 0;

pub const GL_BYTE:                              GLenum = 0x1400;
pub const GL_UNSIGNED_BYTE:                     GLenum = 0x1401;
pub const GL_SHORT:                             GLenum = 0x1402;
pub const GL_UNSIGNED_SHORT:                    GLenum = 0x1403;
pub const GL_INT:                               GLenum = 0x1404;
pub const GL_UNSIGNED_INT:                      GLenum = 0x1405;
pub const GL_FLOAT:                             GLenum = 0x1406;
pub const GL_2_BYTES:                           GLenum = 0x1407;
pub const GL_3_BYTES:                           GLenum = 0x1408;
pub const GL_4_BYTES:                           GLenum = 0x1409;
pub const GL_DOUBLE:                            GLenum = 0x140A;

pub const GL_NO_ERROR:                          GLenum = 0;
pub const GL_INVALID_ENUM:                      GLenum = 0x0500;
pub const GL_INVALID_VALUE:                     GLenum = 0x0501;
pub const GL_INVALID_OPERATION:                 GLenum = 0x0502;
pub const GL_STACK_OVERFLOW:                    GLenum = 0x0503;
pub const GL_STACK_UNDERFLOW:                   GLenum = 0x0504;
pub const GL_OUT_OF_MEMORY:                     GLenum = 0x0505;
pub const GL_INVALID_FRAMEBUFFER_OPERATION:     GLenum = 0x0506;

pub const GL_CURRENT_BIT:                       GLenum = 0x00000001;
pub const GL_POINT_BIT:                         GLenum = 0x00000002;
pub const GL_LINE_BIT:                          GLenum = 0x00000004;
pub const GL_POLYGON_BIT:                       GLenum = 0x00000008;
pub const GL_POLYGON_STIPPLE_BIT:               GLenum = 0x00000010;
pub const GL_PIXEL_MODE_BIT:                    GLenum = 0x00000020;
pub const GL_LIGHTING_BIT:                      GLenum = 0x00000040;
pub const GL_FOG_BIT:                           GLenum = 0x00000080;
pub const GL_DEPTH_BUFFER_BIT:                  GLenum = 0x00000100;
pub const GL_ACCUM_BUFFER_BIT:                  GLenum = 0x00000200;
pub const GL_STENCIL_BUFFER_BIT:                GLenum = 0x00000400;
pub const GL_VIEWPORT_BIT:                      GLenum = 0x00000800;
pub const GL_TRANSFORM_BIT:                     GLenum = 0x00001000;
pub const GL_ENABLE_BIT:                        GLenum = 0x00002000;
pub const GL_COLOR_BUFFER_BIT:                  GLenum = 0x00004000;
pub const GL_HINT_BIT:                          GLenum = 0x00008000;
pub const GL_EVAL_BIT:                          GLenum = 0x00010000;
pub const GL_LIST_BIT:                          GLenum = 0x00020000;
pub const GL_TEXTURE_BIT:                       GLenum = 0x00040000;
pub const GL_SCISSOR_BIT:                       GLenum = 0x00080000;
pub const GL_ALL_ATTRIB_BITS:                   GLenum = 0xFFFFFFFF;

pub const GL_POINTS:                            GLenum = 0x0000;
pub const GL_LINES:                             GLenum = 0x0001;
pub const GL_LINE_LOOP:                         GLenum = 0x0002;
pub const GL_LINE_STRIP:                        GLenum = 0x0003;
pub const GL_TRIANGLES:                         GLenum = 0x0004;
pub const GL_TRIANGLE_STRIP:                    GLenum = 0x0005;
pub const GL_TRIANGLE_FAN:                      GLenum = 0x0006;
pub const GL_QUADS:                             GLenum = 0x0007;

pub const GL_MAP_COLOR:                         GLenum = 0x0D10;
pub const GL_MAP_STENCIL:                       GLenum = 0x0D11;
pub const GL_INDEX_SHIFT:                       GLenum = 0x0D12;
pub const GL_INDEX_OFFSET:                      GLenum = 0x0D13;
pub const GL_RED_SCALE:                         GLenum = 0x0D14;
pub const GL_RED_BIAS:                          GLenum = 0x0D15;
pub const GL_GREEN_SCALE:                       GLenum = 0x0D18;
pub const GL_GREEN_BIAS:                        GLenum = 0x0D19;
pub const GL_BLUE_SCALE:                        GLenum = 0x0D1A;
pub const GL_BLUE_BIAS:                         GLenum = 0x0D1B;
pub const GL_ALPHA_SCALE:                       GLenum = 0x0D1C;
pub const GL_ALPHA_BIAS:                        GLenum = 0x0D1D;
pub const GL_DEPTH_SCALE:                       GLenum = 0x0D1E;
pub const GL_DEPTH_BIAS:                        GLenum = 0x0D1F;
pub const GL_PIXEL_MAP_S_TO_S_SIZE:             GLenum = 0x0CB1;
pub const GL_PIXEL_MAP_I_TO_I_SIZE:             GLenum = 0x0CB0;
pub const GL_PIXEL_MAP_I_TO_R_SIZE:             GLenum = 0x0CB2;
pub const GL_PIXEL_MAP_I_TO_G_SIZE:             GLenum = 0x0CB3;
pub const GL_PIXEL_MAP_I_TO_B_SIZE:             GLenum = 0x0CB4;
pub const GL_PIXEL_MAP_I_TO_A_SIZE:             GLenum = 0x0CB5;
pub const GL_PIXEL_MAP_R_TO_R_SIZE:             GLenum = 0x0CB6;
pub const GL_PIXEL_MAP_G_TO_G_SIZE:             GLenum = 0x0CB7;
pub const GL_PIXEL_MAP_B_TO_B_SIZE:             GLenum = 0x0CB8;
pub const GL_PIXEL_MAP_A_TO_A_SIZE:             GLenum = 0x0CB9;
pub const GL_PIXEL_MAP_S_TO_S:                  GLenum = 0x0C71;
pub const GL_PIXEL_MAP_I_TO_I:                  GLenum = 0x0C70;
pub const GL_PIXEL_MAP_I_TO_R:                  GLenum = 0x0C72;
pub const GL_PIXEL_MAP_I_TO_G:                  GLenum = 0x0C73;
pub const GL_PIXEL_MAP_I_TO_B:                  GLenum = 0x0C74;
pub const GL_PIXEL_MAP_I_TO_A:                  GLenum = 0x0C75;
pub const GL_PIXEL_MAP_R_TO_R:                  GLenum = 0x0C76;
pub const GL_PIXEL_MAP_G_TO_G:                  GLenum = 0x0C77;
pub const GL_PIXEL_MAP_B_TO_B:                  GLenum = 0x0C78;
pub const GL_PIXEL_MAP_A_TO_A:                  GLenum = 0x0C79;
pub const GL_PACK_ALIGNMENT:                    GLenum = 0x0D05;
pub const GL_PACK_LSB_FIRST:                    GLenum = 0x0D01;
pub const GL_PACK_ROW_LENGTH:                   GLenum = 0x0D02;
pub const GL_PACK_SKIP_PIXELS:                  GLenum = 0x0D04;
pub const GL_PACK_SKIP_ROWS:                    GLenum = 0x0D03;
pub const GL_PACK_SWAP_BYTES:                   GLenum = 0x0D00;
pub const GL_UNPACK_ALIGNMENT:                  GLenum = 0x0CF5;
pub const GL_UNPACK_LSB_FIRST:                  GLenum = 0x0CF1;
pub const GL_UNPACK_ROW_LENGTH:                 GLenum = 0x0CF2;
pub const GL_UNPACK_SKIP_PIXELS:                GLenum = 0x0CF4;
pub const GL_UNPACK_SKIP_ROWS:                  GLenum = 0x0CF3;
pub const GL_UNPACK_SWAP_BYTES:                 GLenum = 0x0CF0;
pub const GL_ZOOM_X:                            GLenum = 0x0D16;
pub const GL_ZOOM_Y:                            GLenum = 0x0D17;

pub const GL_TEXTURE_1D:                        GLenum = 0x0DE0;
pub const GL_TEXTURE_2D:                        GLenum = 0x0DE1;
pub const GL_TEXTURE_WRAP_S:                    GLenum = 0x2802;
pub const GL_TEXTURE_WRAP_T:                    GLenum = 0x2803;
pub const GL_TEXTURE_MAG_FILTER:                GLenum = 0x2800;
pub const GL_TEXTURE_MIN_FILTER:                GLenum = 0x2801;

pub const GL_MODELVIEW:                         GLenum = 0x1700;
pub const GL_PROJECTION:                        GLenum = 0x1701;
pub const GL_TEXTURE:                           GLenum = 0x1702;

pub const GL_RGB:                               GLenum = 0x1907;
pub const GL_RGBA:                              GLenum = 0x1908;

pub const GL_BGR:                               GLenum = 0x80E0;
pub const GL_BGRA:                              GLenum = 0x80E1;

pub const GL_RGB4:                              GLenum = 0x804F;
pub const GL_RGB5:                              GLenum = 0x8050;
pub const GL_RGB8:                              GLenum = 0x8051;
pub const GL_RGB10:                             GLenum = 0x8052;
pub const GL_RGB12:                             GLenum = 0x8053;
pub const GL_RGB16:                             GLenum = 0x8054;
pub const GL_RGBA2:                             GLenum = 0x8055;
pub const GL_RGBA4:                             GLenum = 0x8056;
pub const GL_RGB5_A1:                           GLenum = 0x8057;
pub const GL_RGBA8:                             GLenum = 0x8058;
pub const GL_RGB10_A2:                          GLenum = 0x8059;
pub const GL_RGBA12:                            GLenum = 0x805A;
pub const GL_RGBA16:                            GLenum = 0x805B;

pub const GL_FRAGMENT_SHADER:                   GLenum = 0x8B30;
pub const GL_VERTEX_SHADER:                     GLenum = 0x8B31;

pub const GL_DELETE_STATUS:                     GLenum = 0x8B80;
pub const GL_COMPILE_STATUS:                    GLenum = 0x8B81;
pub const GL_LINK_STATUS:                       GLenum = 0x8B82;
pub const GL_VALIDATE_STATUS:                   GLenum = 0x8B83;
pub const GL_INFO_LOG_LENGTH:                   GLenum = 0x8B84;
pub const GL_ATTACHED_SHADERS:                  GLenum = 0x8B85;
pub const GL_ACTIVE_UNIFORMS:                   GLenum = 0x8B86;
pub const GL_ACTIVE_UNIFORM_MAX_LENGTH:         GLenum = 0x8B87;
pub const GL_SHADER_SOURCE_LENGTH:              GLenum = 0x8B88;
pub const GL_ACTIVE_ATTRIBUTES:                 GLenum = 0x8B89;
pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH:       GLenum = 0x8B8A;
pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT:   GLenum = 0x8B8B;
pub const GL_SHADING_LANGUAGE_VERSION:          GLenum = 0x8B8C;
pub const GL_CURRENT_PROGRAM:                   GLenum = 0x8B8D;

pub const GL_FRAMEBUFFER:                       GLenum = 0x8D40;
pub const GL_READ_FRAMEBUFFER:                  GLenum = 0x8CA8;
pub const GL_DRAW_FRAMEBUFFER:                  GLenum = 0x8CA9;

pub const GL_COLOR_ATTACHMENT0:                 GLenum = 0x8CE0;

pub const GL_CLAMP_TO_EDGE:                     GLenum = 0x812F;
pub const GL_NEAREST:                           GLenum = 0x2600;

pub const GL_SCISSOR_TEST:                      GLenum = 0x0C11;
pub const GL_CULL_FACE:                         GLenum = 0x0B44;
pub const GL_BLEND:                             GLenum = 0x0BE2;
pub const GL_DITHER:                            GLenum = 0x0BD0;
pub const GL_STENCIL_TEST:                      GLenum = 0x0B90;
pub const GL_DEPTH_TEST:                        GLenum = 0x0B71;
pub const GL_POLYGON_OFFSET_FILL:               GLenum = 0x8037;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE:          GLenum = 0x809E;
pub const GL_SAMPLE_COVERAGE:                   GLenum = 0x80A0;

pub const GL_STREAM_DRAW:                       GLenum = 0x88E0;
pub const GL_STATIC_DRAW:                       GLenum = 0x88E4;
pub const GL_DYNAMIC_DRAW:                      GLenum = 0x88E8;

pub const GL_PIXEL_PACK_BUFFER:                 GLenum = 0x88EB;
pub const GL_PIXEL_UNPACK_BUFFER:               GLenum = 0x88EC;

pub const GL_READ_ONLY:                         GLenum = 0x88B8;
pub const GL_WRITE_ONLY:                        GLenum = 0x88B9;
pub const GL_READ_WRITE:                        GLenum = 0x88BA;

pub const GL_MAP_READ_BIT:                      GLenum = 0x0001;
pub const GL_MAP_WRITE_BIT:                     GLenum = 0x0002;
pub const GL_MAP_INVALIDATE_RANGE_BIT:          GLenum = 0x0004;
pub const GL_MAP_INVALIDATE_BUFFER_BIT:         GLenum = 0x0008;
pub const GL_MAP_FLUSH_EXPLICIT_BIT:            GLenum = 0x0010;
pub const GL_MAP_UNSYNCHRONIZED_BIT:            GLenum = 0x0020;
pub const GL_MAP_PERSISTENT_BIT:                GLenum = 0x0040;
pub const GL_MAP_COHERENT_BIT:                  GLenum = 0x0080;
pub const GL_DYNAMIC_STORAGE_BIT:               GLenum = 0x0100;
pub const GL_CLIENT_STORAGE_BIT:                GLenum = 0x0200;

pub const GL_FRAMEBUFFER_SRGB:                  GLenum = 0x8DB9;

pub const GL_DEBUG_OUTPUT_SYNCHRONOUS:          GLenum = 0x8242;
pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH:  GLenum = 0x8243;
pub const GL_DEBUG_CALLBACK_FUNCTION:           GLenum = 0x8244;
pub const GL_DEBUG_CALLBACK_USER_PARAM:         GLenum = 0x8245;
pub const GL_DEBUG_SOURCE_API:                  GLenum = 0x8246;
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM:        GLenum = 0x8247;
pub const GL_DEBUG_SOURCE_SHADER_COMPILER:      GLenum = 0x8248;
pub const GL_DEBUG_SOURCE_THIRD_PARTY:          GLenum = 0x8249;
pub const GL_DEBUG_SOURCE_APPLICATION:          GLenum = 0x824A;
pub const GL_DEBUG_SOURCE_OTHER:                GLenum = 0x824B;
pub const GL_DEBUG_TYPE_ERROR:                  GLenum = 0x824C;
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR:    GLenum = 0x824D;
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR:     GLenum = 0x824E;
pub const GL_DEBUG_TYPE_PORTABILITY:            GLenum = 0x824F;
pub const GL_DEBUG_TYPE_PERFORMANCE:            GLenum = 0x8250;
pub const GL_DEBUG_TYPE_OTHER:                  GLenum = 0x8251;
pub const GL_DEBUG_TYPE_MARKER:                 GLenum = 0x8268;
pub const GL_DEBUG_TYPE_PUSH_GROUP:             GLenum = 0x8269;
pub const GL_DEBUG_TYPE_POP_GROUP:              GLenum = 0x826A;
pub const GL_DEBUG_SEVERITY_NOTIFICATION:       GLenum = 0x826B;
pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH:       GLenum = 0x826C;
pub const GL_DEBUG_GROUP_STACK_DEPTH:           GLenum = 0x826D;
pub const GL_BUFFER:                            GLenum = 0x82E0;
pub const GL_SHADER:                            GLenum = 0x82E1;
pub const GL_PROGRAM:                           GLenum = 0x82E2;
pub const GL_QUERY:                             GLenum = 0x82E3;
pub const GL_PROGRAM_PIPELINE:                  GLenum = 0x82E4;
pub const GL_SAMPLER:                           GLenum = 0x82E6;
pub const GL_DISPLAY_LIST:                      GLenum = 0x82E7;
pub const GL_MAX_LABEL_LENGTH:                  GLenum = 0x82E8;
pub const GL_MAX_DEBUG_MESSAGE_LENGTH:          GLenum = 0x9143;
pub const GL_MAX_DEBUG_LOGGED_MESSAGES:         GLenum = 0x9144;
pub const GL_DEBUG_LOGGED_MESSAGES:             GLenum = 0x9145;
pub const GL_DEBUG_SEVERITY_HIGH:               GLenum = 0x9146;
pub const GL_DEBUG_SEVERITY_MEDIUM:             GLenum = 0x9147;
pub const GL_DEBUG_SEVERITY_LOW:                GLenum = 0x9148;
pub const GL_DEBUG_OUTPUT:                      GLenum = 0x92E0;
pub const GL_CONTEXT_FLAG_DEBUG_BIT:            GLenum = 0x00000002;

static mut glGetErrorPtr:                        FnPtr = NULL_PTR;
static mut glClearPtr:                           FnPtr = NULL_PTR;
static mut glFlushPtr:                           FnPtr = NULL_PTR;
static mut glFinishPtr:                          FnPtr = NULL_PTR;
static mut glDisablePtr:                         FnPtr = NULL_PTR;
static mut glEnablePtr:                          FnPtr = NULL_PTR;
static mut glPixelStoreiPtr:                     FnPtr = NULL_PTR;
static mut glGenTexturesPtr:                     FnPtr = NULL_PTR;
static mut glDeleteTexturesPtr:                  FnPtr = NULL_PTR;
static mut glActiveTexturePtr:                   FnPtr = NULL_PTR;
static mut glBindTexturePtr:                     FnPtr = NULL_PTR;
static mut glTexParameteriPtr:                   FnPtr = NULL_PTR;
static mut glGetIntegervPtr:                     FnPtr = NULL_PTR;
static mut glClearColorPtr:                      FnPtr = NULL_PTR;
static mut glTexImage2DPtr:                      FnPtr = NULL_PTR;
static mut glTexSubImage2DPtr:                   FnPtr = NULL_PTR;
static mut glViewportPtr:                        FnPtr = NULL_PTR;
static mut glDrawElementsPtr:                    FnPtr = NULL_PTR;
static mut glDrawArraysPtr:                      FnPtr = NULL_PTR;
static mut glGenFramebuffersPtr:                 FnPtr = NULL_PTR;
static mut glDeleteFramebuffersPtr:              FnPtr = NULL_PTR;
static mut glBindFramebufferPtr:                 FnPtr = NULL_PTR;
static mut glFramebufferTexture2DPtr:            FnPtr = NULL_PTR;
static mut glBlitFramebufferPtr:                 FnPtr = NULL_PTR;
static mut glGenBuffersPtr:                      FnPtr = NULL_PTR;
static mut glDeleteBuffersPtr:                   FnPtr = NULL_PTR;
static mut glBindBufferPtr:                      FnPtr = NULL_PTR;
static mut glBufferDataPtr:                      FnPtr = NULL_PTR;
static mut glMapBufferPtr:                       FnPtr = NULL_PTR;
static mut glUnmapBufferPtr:                     FnPtr = NULL_PTR;
static mut glMapBufferRangePtr:                  FnPtr = NULL_PTR;
static mut glCreateShaderPtr:                    FnPtr = NULL_PTR;
static mut glShaderSourcePtr:                    FnPtr = NULL_PTR;
static mut glCompileShaderPtr:                   FnPtr = NULL_PTR;
static mut glGetShaderivPtr:                     FnPtr = NULL_PTR;
static mut glDeleteShaderPtr:                    FnPtr = NULL_PTR;
static mut glCreateProgramPtr:                   FnPtr = NULL_PTR;
static mut glAttachShaderPtr:                    FnPtr = NULL_PTR;
static mut glLinkProgramPtr:                     FnPtr = NULL_PTR;
static mut glUseProgramPtr:                      FnPtr = NULL_PTR;
static mut glGetProgramivPtr:                    FnPtr = NULL_PTR;
static mut glDeleteProgramPtr:                   FnPtr = NULL_PTR;
static mut glGetAttribLocationPtr:               FnPtr = NULL_PTR;
static mut glVertexAttribPointerPtr:             FnPtr = NULL_PTR;
static mut glUniform1iPtr:                       FnPtr = NULL_PTR;
static mut glEnableVertexAttribArrayPtr:         FnPtr = NULL_PTR;
static mut glGetUniformLocationPtr:              FnPtr = NULL_PTR;
static mut glDebugMessageControlPtr:             FnPtr = NULL_PTR;
static mut glGetDebugMessageLogPtr:              FnPtr = NULL_PTR;
static mut glDrawPixelsPtr:                      FnPtr = NULL_PTR;

pub static mut BUFFER_FUNCTIONS_LOADED:           bool = false;
pub static mut QUAD_FUNCTIONS_LOADED:             bool = false;
pub static mut PIXEL_FUNCTIONS_LOADED:            bool = false;
pub static mut DEBUG_FUNCTIONS_LOADED:            bool = false;

#[inline]
pub unsafe fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) {
   mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(glGenFramebuffersPtr)(n, framebuffers)
}

#[inline]
pub unsafe fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) {
   mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(glDeleteFramebuffersPtr)(n, framebuffers)
}

#[inline]
pub unsafe fn glBindFramebuffer(target: GLenum, framebuffer: GLuint) {
   mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(glBindFramebufferPtr)(target, framebuffer)
}

#[inline]
pub unsafe fn glFramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) {
   mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint) -> ()>(glFramebufferTexture2DPtr)(target, attachment, textarget, texture, level)
}

#[inline]
pub unsafe fn glBlitFramebuffer(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) {
   mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLbitfield, GLenum) -> ()>(glBlitFramebufferPtr)(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter)
}

#[inline]
pub unsafe fn glGenBuffers(n: GLsizei, buffers: *mut GLuint) {
   mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(glGenBuffersPtr)(n, buffers)
}

#[inline]
pub unsafe fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint) {
   mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(glDeleteBuffersPtr)(n, buffers)
}

#[inline]
pub unsafe fn glBindBuffer(target: GLenum, buffer: GLuint) {
   mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(glBindBufferPtr)(target, buffer)
}

#[inline]
pub unsafe fn glBufferData(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum) {
   mem::transmute::<_, extern "system" fn(GLenum, GLsizeiptr, *const c_void, GLenum) -> ()>(glBufferDataPtr)(target, size, data, usage)
}

#[inline]
pub unsafe fn glMapBuffer(target: GLenum, access: GLenum) -> *mut c_void {
   mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> *mut c_void>(glMapBufferPtr)(target, access)
}

#[inline]
pub unsafe fn glUnmapBuffer(target: GLenum) -> GLboolean {
   mem::transmute::<_, extern "system" fn(GLenum) -> GLboolean>(glUnmapBufferPtr)(target)
}

#[inline]
pub unsafe fn glMapBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut c_void {
   mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr, GLbitfield) -> *mut c_void>(glMapBufferRangePtr)(target, offset, length, access)
}

#[inline]
pub unsafe fn glCreateShader(shader_type: GLenum) -> GLuint {
   mem::transmute::<_, extern "system" fn(GLenum) -> GLuint>(glCreateShaderPtr)(shader_type)
}

#[inline]
pub unsafe fn glShaderSource(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint) {
   mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLint) -> ()>(glShaderSourcePtr)(shader, count, string, length)
}

#[inline]
pub unsafe fn glCompileShader(shader: GLuint) {
   mem::transmute::<_, extern "system" fn(GLuint) -> ()>(glCompileShaderPtr)(shader)
}

#[inline]
pub unsafe fn glDeleteShader(shader: GLuint) {
   mem::transmute::<_, extern "system" fn(GLuint) -> ()>(glDeleteShaderPtr)(shader)
}

#[inline]
pub unsafe fn glCreateProgram() -> GLuint {
   mem::transmute::<_, extern "system" fn() -> GLuint>(glCreateProgramPtr)()
}

#[inline]
pub unsafe fn glAttachShader(program: GLuint, shader: GLuint) {
   mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(glAttachShaderPtr)(program, shader)
}

#[inline]
pub unsafe fn glLinkProgram(program: GLuint) {
   mem::transmute::<_, extern "system" fn(GLuint) -> ()>(glLinkProgramPtr)(program)
}

#[inline]
pub unsafe fn glDeleteProgram(program: GLuint) {
   mem::transmute::<_, extern "system" fn(GLuint) -> ()>(glDeleteProgramPtr)(program)
}

#[inline]
pub unsafe fn glUseProgram(program: GLuint) {
   mem::transmute::<_, extern "system" fn(GLuint) -> ()>(glUseProgramPtr)(program)
}

#[inline]
pub unsafe fn glGetProgramiv(target: GLenum, pname: GLenum, params: *mut GLint) {
   mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(glGetProgramivPtr)(target, pname, params)
}

#[inline]
pub unsafe fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) {
   mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(glGetShaderivPtr)(shader, pname, params)
}

#[inline]
pub unsafe fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint {
   mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(glGetAttribLocationPtr)(program, name)
}

#[inline]
pub unsafe fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint {
   mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(glGetUniformLocationPtr)(program, name)
}

#[inline]
pub unsafe fn glEnableVertexAttribArray(index: GLuint) {
   mem::transmute::<_, extern "system" fn(GLuint) -> ()>(glEnableVertexAttribArrayPtr)(index)
}

#[inline]
pub unsafe fn glUniform1i(location: GLint, v0: GLint) {
   mem::transmute::<_, extern "system" fn(GLint, GLint) -> ()>(glUniform1iPtr)(location, v0)
}

#[inline]
pub unsafe fn glVertexAttribPointer(index: GLuint, size: GLint, _type: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void) {
   mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const c_void) -> ()>(glVertexAttribPointerPtr)(index, size, _type, normalized, stride, pointer)
}

pub unsafe fn glDebugMessageControl(source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean) {
   mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *const GLuint, GLboolean) -> ()>(glDebugMessageControlPtr)(source, type_, severity, count, ids, enabled)
}

pub unsafe fn glGetDebugMessageLog(count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint {
   mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLenum, *mut GLenum, *mut GLuint, *mut GLenum, *mut GLsizei, *mut GLchar) -> GLuint>(glGetDebugMessageLogPtr)(count, bufSize, sources, types, ids, severities, lengths, messageLog)
}

pub unsafe fn glDrawPixels(width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, data: *const GLvoid) {
   mem::transmute::<_, extern "system" fn(GLsizei, GLsizei, GLenum, GLenum, *const GLvoid) -> ()>(glDrawPixelsPtr)(width, height, format, type_, data)
}

#[inline]
pub unsafe fn glGetError() -> GLenum {
   mem::transmute::<_, extern "system" fn() -> GLenum>(glGetErrorPtr)()
}

#[inline]
pub unsafe fn glClear(mask: GLbitfield) {
   mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(glClearPtr)(mask)
}

#[inline]
pub unsafe fn glFlush() {
   mem::transmute::<_, extern "system" fn() -> ()>(glFlushPtr)()
}

#[inline]
pub unsafe fn glFinish() {
   mem::transmute::<_, extern "system" fn() -> ()>(glFinishPtr)()
}

#[inline]
pub unsafe fn glDisable(cap: GLenum) {
   mem::transmute::<_, extern "system" fn(GLenum) -> ()>(glDisablePtr)(cap)
}

#[inline]
pub unsafe fn glEnable(cap: GLenum) {
   mem::transmute::<_, extern "system" fn(GLenum) -> ()>(glEnablePtr)(cap)
}

#[inline]
pub unsafe fn glPixelStorei(pname: GLenum, param: GLint) {
   mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(glPixelStoreiPtr)(pname, param)
}

#[inline]
pub unsafe fn glGenTextures(n: GLsizei, textures: *mut GLuint) {
   mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(glGenTexturesPtr)(n, textures)
}

#[inline]
pub unsafe fn glDeleteTextures(n: GLsizei, textures: *const GLuint) {
   mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(glDeleteTexturesPtr)(n, textures)
}

#[inline]
pub unsafe fn glActiveTexture(texture: GLenum) {
   mem::transmute::<_, extern "system" fn(GLenum) -> ()>(glActiveTexturePtr)(texture)
}

#[inline]
pub unsafe fn glBindTexture(target: GLenum, texture: GLuint) {
   mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(glBindTexturePtr)(target, texture)
}

#[inline]
pub unsafe fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint) {
   mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint)>(glTexParameteriPtr)(target, pname, param)
}

#[inline]
pub unsafe fn glGetIntegerv(pname: GLenum, data: *mut GLint) {
   mem::transmute::<_, extern "system" fn(GLenum, *mut GLint)>(glGetIntegervPtr)(pname, data)
}

#[inline]
pub unsafe fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
   mem::transmute::<_, extern "system" fn(GLclampf, GLclampf, GLclampf, GLclampf) -> ()>(glClearColorPtr)(red, green, blue, alpha)
}

#[inline]
pub unsafe fn glTexImage2D(target: GLenum, level: GLint, internalFormat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, _type: GLenum, pixels: *const GLvoid) {
   mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLint, GLenum, GLenum, *const GLvoid) -> ()>(glTexImage2DPtr)(target, level, internalFormat, width, height, border, format, _type, pixels)
}

#[inline]
pub unsafe fn glTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, _type: GLenum, pixels: *const GLvoid) {
   mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *const GLvoid) -> ()>(glTexSubImage2DPtr)(target, level, xoffset, yoffset, width, height, format, _type, pixels)
}

#[inline]
pub unsafe fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
   mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> ()>(glViewportPtr)(x, y, width, height)
}

#[inline]
pub unsafe fn glDrawElements(mode: GLenum, count: GLsizei, _type: GLenum, indices: *const GLvoid) {
   mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const GLvoid) -> ()>(glDrawElementsPtr)(mode, count, _type, indices)
}

#[inline]
pub unsafe fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei) {
   mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei) -> ()>(glDrawArraysPtr)(mode, first, count)
}

pub unsafe fn load_functions<T: FnPtrLoader>(loader: &T) -> bool {
   glGetErrorPtr = loader.load("glGetError");
   glClearPtr = loader.load("glClear");
   glFlushPtr = loader.load("glFlush");
   glFinishPtr = loader.load("glFinish");
   glDisablePtr = loader.load("glDisable");
   glEnablePtr = loader.load("glEnable");
   glPixelStoreiPtr = loader.load("glPixelStorei");
   glGenTexturesPtr = loader.load("glGenTextures");
   glDeleteTexturesPtr = loader.load("glDeleteTextures");
   glActiveTexturePtr = loader.load("glActiveTexture");
   glBindTexturePtr = loader.load("glBindTexture");
   glTexParameteriPtr = loader.load("glTexParameteri");
   glGetIntegervPtr = loader.load("glGetIntegerv");
   glClearColorPtr = loader.load("glClearColor");
   glTexImage2DPtr = loader.load("glTexImage2D");
   glTexSubImage2DPtr = loader.load("glTexSubImage2D");
   glViewportPtr = loader.load("glViewport");
   glDrawElementsPtr = loader.load("glDrawElements");
   glDrawArraysPtr = loader.load("glDrawArrays");
   glGenFramebuffersPtr = loader.load("glGenFramebuffers");
   glDeleteFramebuffersPtr = loader.load("glDeleteFramebuffers");
   glBindFramebufferPtr = loader.load("glBindFramebuffer");
   glFramebufferTexture2DPtr = loader.load("glFramebufferTexture2D");
   glBlitFramebufferPtr = loader.load("glBlitFramebuffer");
   glGenBuffersPtr = loader.load("glGenBuffers");
   glDeleteBuffersPtr = loader.load("glDeleteBuffers");
   glBindBufferPtr = loader.load("glBindBuffer");
   glBufferDataPtr = loader.load("glBufferData");
   glMapBufferPtr = loader.load("glMapBuffer");
   glUnmapBufferPtr = loader.load("glUnmapBuffer");
   glMapBufferRangePtr = loader.load("glMapBufferRange");
   glCreateShaderPtr = loader.load("glCreateShader");
   glShaderSourcePtr = loader.load("glShaderSource");
   glCompileShaderPtr = loader.load("glCompileShader");
   glGetShaderivPtr = loader.load("glGetShaderiv");
   glDeleteShaderPtr = loader.load("glDeleteShader");
   glCreateProgramPtr = loader.load("glCreateProgram");
   glAttachShaderPtr = loader.load("glAttachShader");
   glLinkProgramPtr = loader.load("glLinkProgram");
   glUseProgramPtr = loader.load("glUseProgram");
   glGetProgramivPtr = loader.load("glGetProgramiv");
   glDeleteProgramPtr = loader.load("glDeleteProgram");
   glGetAttribLocationPtr = loader.load("glGetAttribLocation");
   glVertexAttribPointerPtr = loader.load("glVertexAttribPointer");
   glUniform1iPtr = loader.load("glUniform1i");
   glEnableVertexAttribArrayPtr = loader.load("glEnableVertexAttribArray");
   glGetUniformLocationPtr = loader.load("glGetUniformLocation");
   glDrawPixelsPtr = loader.load("glDrawPixels");
   glGetDebugMessageLogPtr = loader.load_any(&[
      "glGetDebugMessageLog", "glGetDebugMessageLogARB", "glGetDebugMessageLogKHR"]);

   BUFFER_FUNCTIONS_LOADED = are_buffer_functions_loaded();

   QUAD_FUNCTIONS_LOADED = are_quad_functions_loaded();

   PIXEL_FUNCTIONS_LOADED = are_pixel_functions_loaded();

   DEBUG_FUNCTIONS_LOADED = are_debug_functions_loaded();

   true
}

unsafe fn are_buffer_functions_loaded() -> bool {
   glGenTexturesPtr != NULL_PTR &&
   glBindTexturePtr != NULL_PTR &&
   glTexParameteriPtr != NULL_PTR &&
   glTexImage2DPtr != NULL_PTR &&
   glTexSubImage2DPtr != NULL_PTR &&
   glDeleteTexturesPtr != NULL_PTR &&
   glGenFramebuffersPtr != NULL_PTR &&
   glBindFramebufferPtr != NULL_PTR &&
   glFramebufferTexture2DPtr != NULL_PTR &&
   glBlitFramebufferPtr != NULL_PTR &&
   glDeleteFramebuffersPtr != NULL_PTR &&
   glGenBuffersPtr != NULL_PTR &&
   glBindBufferPtr != NULL_PTR &&
   glBufferDataPtr != NULL_PTR &&
   glMapBufferPtr != NULL_PTR &&
   glUnmapBufferPtr != NULL_PTR &&
   glDeleteBuffersPtr != NULL_PTR
}

unsafe fn are_quad_functions_loaded() -> bool {
   glGenTexturesPtr != NULL_PTR &&
   glBindTexturePtr != NULL_PTR &&
   glTexParameteriPtr != NULL_PTR &&
   glTexImage2DPtr != NULL_PTR &&
   glTexSubImage2DPtr != NULL_PTR &&
   glDeleteTexturesPtr != NULL_PTR &&
   glVertexAttribPointerPtr != NULL_PTR &&
   glEnableVertexAttribArrayPtr != NULL_PTR &&
   glDrawArraysPtr != NULL_PTR &&
   glCreateShaderPtr != NULL_PTR &&
   glShaderSourcePtr != NULL_PTR &&
   glCompileShaderPtr != NULL_PTR &&
   glGetShaderivPtr != NULL_PTR &&
   glDeleteShaderPtr != NULL_PTR &&
   glCreateProgramPtr != NULL_PTR &&
   glAttachShaderPtr != NULL_PTR &&
   glLinkProgramPtr != NULL_PTR &&
   glUseProgramPtr != NULL_PTR &&
   glGetProgramivPtr != NULL_PTR &&
   glGetAttribLocationPtr != NULL_PTR &&
   glGetUniformLocationPtr != NULL_PTR &&
   glDeleteProgramPtr != NULL_PTR
}

unsafe fn are_pixel_functions_loaded() -> bool {
   glClearPtr != NULL_PTR &&
   glDrawPixelsPtr != NULL_PTR
}

unsafe fn are_debug_functions_loaded() -> bool {
   glGetDebugMessageLogPtr != NULL_PTR
}
