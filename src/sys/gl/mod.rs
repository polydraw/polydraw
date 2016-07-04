pub mod ffi;

use std::mem;
use std::ptr;
use std::ffi::CString;

use error::{RuntimeError, ErrorKind};

use super::utils::fn_ptr::FnPtrLoader;

#[inline]
pub fn load<T: FnPtrLoader>(loader: &T) {
   unsafe {
      ffi::load_functions(loader)
   };
}

#[inline]
fn gl_error<T>(message: &str) -> Result<T, RuntimeError> {
   Err(
      RuntimeError::new(
         ErrorKind::GL,
         message.to_string()
      )
   )
}

#[inline]
fn gl_result<T>(value: T) -> Result<T, RuntimeError> {
   let result = unsafe {
      ffi::glGetError()
   };

   match result {
      ffi::GL_NO_ERROR => Ok(value),
      ffi::GL_INVALID_ENUM => gl_error(
         "An unacceptable value is specified for an enumerated argument"
      ),
      ffi::GL_INVALID_VALUE => gl_error(
         "A numeric argument is out of range"
      ),
      ffi::GL_INVALID_OPERATION => gl_error(
         "The specified operation is not allowed in the current state"
      ),
      ffi::GL_INVALID_FRAMEBUFFER_OPERATION => gl_error(
         "The framebuffer object is not complete"
      ),
      ffi::GL_OUT_OF_MEMORY => gl_error(
         "There is not enough memory left to execute the command"
      ),
      ffi::GL_STACK_UNDERFLOW => gl_error(
         "Performing an operation that would cause an internal stack to underflow"
      ),
      ffi::GL_STACK_OVERFLOW => gl_error(
         "Performing an operation that would cause an internal stack to overflow"
      ),
      _ => gl_error("Unknown OpenGL error")
   }
}

#[inline]
pub fn reset_pixelstore_alignment() -> Result<(), RuntimeError> {
   unsafe {
      ffi::glPixelStorei(ffi::GL_UNPACK_ALIGNMENT, 1);
   }

   gl_result(())
}

#[inline]
pub fn enable_framebuffer_srgb() -> Result<(), RuntimeError> {
   unsafe {
      ffi::glEnable(ffi::GL_FRAMEBUFFER_SRGB);
   }

   gl_result(())
}

pub struct Texture {
   pub name: ffi::GLuint,
}

impl Texture {
   pub fn new(width: u32, height: u32) -> Result<Self, RuntimeError> {
      let mut name: ffi::GLuint = unsafe { mem::uninitialized() };

      unsafe {
         ffi::glGenTextures(1, &mut name);

         ffi::glBindTexture(ffi::GL_TEXTURE_2D, name);

         ffi::glTexParameteri(
            ffi::GL_TEXTURE_2D,
            ffi::GL_TEXTURE_WRAP_S,
            ffi::GL_CLAMP_TO_EDGE as ffi::GLint
         );

         ffi::glTexParameteri(
            ffi::GL_TEXTURE_2D,
            ffi::GL_TEXTURE_WRAP_T,
            ffi::GL_CLAMP_TO_EDGE as ffi::GLint
         );

         ffi::glTexParameteri(
            ffi::GL_TEXTURE_2D,
            ffi::GL_TEXTURE_MIN_FILTER,
            ffi::GL_NEAREST as ffi::GLint
         );

         ffi::glTexParameteri(
            ffi::GL_TEXTURE_2D,
            ffi::GL_TEXTURE_MAG_FILTER,
            ffi::GL_NEAREST as ffi::GLint
         );

         ffi::glTexImage2D(
            ffi::GL_TEXTURE_2D,
            0,
            ffi::GL_RGBA8 as ffi::GLint,
            width as ffi::GLsizei,
            height as ffi::GLsizei,
            0,
            ffi::GL_RGBA,
            ffi::GL_UNSIGNED_BYTE,
            ptr::null()
         );
      }

      gl_result(Texture {
         name: name,
      })
   }

   #[inline]
   pub fn bind(&self) -> Result<(), RuntimeError> {
      unsafe {
         ffi::glBindTexture(ffi::GL_TEXTURE_2D, self.name);
      }

      gl_result(())
   }

   #[inline]
   pub fn resize(&self, width: u32, height: u32) -> Result<(), RuntimeError> {
      unsafe {
         ffi::glTexImage2D(
            ffi::GL_TEXTURE_2D,
            0,
            ffi::GL_RGBA8 as ffi::GLint,
            width as ffi::GLsizei,
            height as ffi::GLsizei,
            0,
            ffi::GL_RGBA,
            ffi::GL_UNSIGNED_BYTE,
            ptr::null()
         );
      }

      gl_result(())
   }

   #[inline]
   pub fn update(&self, width: u32, height: u32) -> Result<(), RuntimeError> {
      unsafe {
         ffi::glTexSubImage2D(
            ffi::GL_TEXTURE_2D,
            0,
            0, 0, width as ffi::GLsizei, height as ffi::GLsizei,
            ffi::GL_RGBA,
            ffi::GL_UNSIGNED_BYTE,
            ptr::null()
         );
      }

      gl_result(())
   }
}

impl Drop for Texture {
   fn drop (&mut self) {
      unsafe {
         ffi::glDeleteTextures(1, &self.name)
      };
   }
}

pub struct Framebuffer {
   pub name: ffi::GLuint,
}

impl Framebuffer {
   pub fn new(texture: &Texture) -> Result<Self, RuntimeError> {
      let name = try!(Framebuffer::get_framebuffers());

      let framebuffer = Framebuffer {
         name: name,
      };

      try!(framebuffer.bind());
      try!(framebuffer.attach_texture(texture));
      try!(framebuffer.unbind());

      Ok(framebuffer)
   }

   #[inline]
   pub fn get_framebuffers() -> Result<ffi::GLuint, RuntimeError> {
      let mut name: ffi::GLuint = unsafe { mem::uninitialized() };

      unsafe {
         ffi::glGenFramebuffers(1, &mut name)
      };

      gl_result(name)
   }

   #[inline]
   pub fn bind(&self) -> Result<(), RuntimeError> {
      unsafe {
         ffi::glBindFramebuffer(ffi::GL_READ_FRAMEBUFFER, self.name)
      };

      gl_result(())
   }

   #[inline]
   pub fn unbind(&self) -> Result<(), RuntimeError> {
      unsafe {
         ffi::glBindFramebuffer(ffi::GL_READ_FRAMEBUFFER, 0)
      };

      gl_result(())
   }

   #[inline]
   pub fn attach_texture(&self, texture: &Texture) -> Result<(), RuntimeError> {
      unsafe {
         ffi::glFramebufferTexture2D(
            ffi::GL_READ_FRAMEBUFFER,
            ffi::GL_COLOR_ATTACHMENT0,
            ffi::GL_TEXTURE_2D,
            texture.name,
            0
         )
      };

      gl_result(())
   }

   #[inline]
   pub fn blit(&self, width: u32, height: u32) -> Result<(), RuntimeError> {
      unsafe {
         ffi::glBlitFramebuffer(
            0, 0, width as ffi::GLint, height as ffi::GLint,
            0, 0, width as ffi::GLint, height as ffi::GLint,
            ffi::GL_COLOR_BUFFER_BIT,
            ffi::GL_NEAREST
         )
      };

      gl_result(())
   }
}

impl Drop for Framebuffer {
   fn drop (&mut self) {
      unsafe {
         ffi::glDeleteFramebuffers(1, &self.name)
      };
   }
}

pub struct Buffer {
   pub name: ffi::GLuint,
   pub ptr: *mut ffi::c_void,
   pub size: ffi::GLsizeiptr,
}

impl Buffer {
   pub fn new() -> Self {
      let mut name: ffi::GLuint = unsafe { mem::uninitialized() };

      unsafe {
         ffi::glGenBuffers(1, &mut name)
      };

      Buffer {
         name: name,
         ptr: ptr::null_mut(),
         size: 0,
      }
   }

   #[inline]
   pub fn bind(&self) {
      unsafe {
         ffi::glBindBuffer(ffi::GL_PIXEL_UNPACK_BUFFER, self.name)
      };
   }

   #[inline]
   pub fn unbind(&self) {
      unsafe {
         ffi::glBindBuffer(ffi::GL_PIXEL_UNPACK_BUFFER, 0)
      };
   }

   #[inline]
   pub fn init_data(&mut self, size: usize) {
      self.size = size as ffi::GLsizeiptr;
      unsafe {
         ffi::glBufferData(
            ffi::GL_PIXEL_UNPACK_BUFFER,
            self.size,
            ptr::null(),
            ffi::GL_STREAM_DRAW
         )
      };
   }

   #[inline]
   pub fn map(&mut self) {
      self.ptr = unsafe {
         ffi::glMapBuffer(ffi::GL_PIXEL_UNPACK_BUFFER, ffi::GL_WRITE_ONLY)
      };
   }

   #[inline]
   pub fn map_range(&mut self) {
      self.ptr = unsafe {
         ffi::glMapBufferRange(
            ffi::GL_PIXEL_UNPACK_BUFFER, 0, self.size,
            ffi::GL_MAP_WRITE_BIT// | ffi::GL_MAP_UNSYNCHRONIZED_BIT
         )
      };
   }

   #[inline]
   pub fn unmap(&self) {
      unsafe {
         ffi::glUnmapBuffer(ffi::GL_PIXEL_UNPACK_BUFFER)
      };
   }
}

impl Drop for Buffer {
   fn drop (&mut self) {
      unsafe {
         ffi::glDeleteBuffers(1, &self.name)
      };
   }
}

pub struct Shader {
   pub name: ffi::GLuint,
}

impl Shader {
   pub fn new(shader_type: ffi::GLenum) -> Self {
      let name = unsafe {
         ffi::glCreateShader(shader_type)
      };

      Shader {
         name: name,
      }
   }

   #[inline]
   pub fn shader_source(&self, string: &str) {
      let cstring = CString::new(string).unwrap().as_ptr();

      unsafe {
         ffi::glShaderSource(self.name, 1, &cstring, ptr::null())
      };
   }

   #[inline]
   pub fn compile(&self) {
      unsafe {
         ffi::glCompileShader(self.name)
      };
   }

   #[inline]
   pub fn is_compiled(&self) -> bool {
      let mut compiled: ffi::GLint = unsafe { mem::uninitialized() };

      unsafe {
         ffi::glGetShaderiv(self.name, ffi::GL_COMPILE_STATUS, &mut compiled);
      };

      compiled == 1
   }
}

impl Drop for Shader {
   fn drop (&mut self) {
      unsafe {
         ffi::glDeleteShader(self.name)
      };
   }
}

pub struct Program {
   pub name: ffi::GLuint,
}

impl Program {
   pub fn new() -> Self {
      let name = unsafe {
         ffi::glCreateProgram()
      };

      Program {
         name: name,
      }
   }

   #[inline]
   pub fn attach_shader(&self, shader: &Shader) {
      unsafe {
         ffi::glAttachShader(self.name, shader.name)
      };
   }

   #[inline]
   pub fn link(&self) {
      unsafe {
         ffi::glLinkProgram(self.name)
      };
   }

   #[inline]
   pub fn use_(&self) {
      unsafe {
         ffi::glUseProgram(self.name)
      };
   }

   #[inline]
   pub fn is_linked(&self) -> bool {
      let mut linked: ffi::GLint = unsafe { mem::uninitialized() };

      unsafe {
         ffi::glGetProgramiv(self.name, ffi::GL_LINK_STATUS, &mut linked);
      };

      linked == 1
   }
}

impl Drop for Program {
   fn drop (&mut self) {
      unsafe {
         ffi::glDeleteProgram(self.name)
      };
   }
}
