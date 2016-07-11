pub mod ffi;
pub mod frame;

use std::mem;
use std::ptr;
use std::ffi::CString;

use error::{RuntimeError, ErrorKind, VoidResult};

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
pub fn reset_pixelstore_alignment() -> VoidResult {
   unsafe {
      ffi::glPixelStorei(ffi::GL_UNPACK_ALIGNMENT, 1);
   }

   gl_result(())
}

#[inline]
pub fn enable_framebuffer_srgb() -> VoidResult {
   unsafe {
      ffi::glEnable(ffi::GL_FRAMEBUFFER_SRGB);
   }

   gl_result(())
}

#[inline]
pub fn viewport(width: u32, height: u32) -> VoidResult {
   unsafe {
      ffi::glViewport(0, 0, width as ffi::GLsizei, height as ffi::GLsizei)
   }

   gl_result(())
}

#[inline]
pub fn vertex_attrib_pointer(
   index: ffi::GLuint, size: ffi::GLint, buffer: &[ffi::GLfloat]
) -> VoidResult {

   unsafe {
      ffi::glVertexAttribPointer(
         index,
         size,
         ffi::GL_FLOAT,
         ffi::GL_FALSE,
         0,
         buffer.as_ptr() as *const ffi::GLvoid
      )
   }

   gl_result(())
}

#[inline]
pub fn enable_vertex_attrib_array(index: ffi::GLuint) -> VoidResult {
   unsafe {
      ffi::glEnableVertexAttribArray(index)
   }

   gl_result(())
}

#[inline]
pub fn uniform_value_1i(location: ffi::GLint, value: ffi::GLint) -> VoidResult {
   unsafe {
      ffi::glUniform1i(location, value)
   }

   gl_result(())
}

#[inline]
pub fn draw_arrays(count: ffi::GLsizei) -> VoidResult {
   unsafe {
      ffi::glDrawArrays(ffi::GL_TRIANGLE_STRIP, 0, count)
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
   pub fn bind(&self) -> VoidResult {
      unsafe {
         ffi::glBindTexture(ffi::GL_TEXTURE_2D, self.name);
      }

      gl_result(())
   }

   #[inline]
   pub fn resize(&self, width: u32, height: u32) -> VoidResult {
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
   pub fn null_update(&self, width: u32, height: u32) -> VoidResult {
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


   #[inline]
   pub fn update(&self, width: u32, height: u32, data: &[u8]) -> VoidResult {
      unsafe {
         ffi::glTexSubImage2D(
            ffi::GL_TEXTURE_2D,
            0,
            0, 0, width as ffi::GLsizei, height as ffi::GLsizei,
            ffi::GL_RGBA,
            ffi::GL_UNSIGNED_BYTE,
            data.as_ptr() as *const ffi::GLvoid
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
   pub fn bind(&self) -> VoidResult {
      unsafe {
         ffi::glBindFramebuffer(ffi::GL_READ_FRAMEBUFFER, self.name)
      };

      gl_result(())
   }

   #[inline]
   pub fn unbind(&self) -> VoidResult {
      unsafe {
         ffi::glBindFramebuffer(ffi::GL_READ_FRAMEBUFFER, 0)
      };

      gl_result(())
   }

   #[inline]
   pub fn attach_texture(&self, texture: &Texture) -> VoidResult {
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
   pub fn blit(&self, width: u32, height: u32) -> VoidResult {
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
   pub fn new() -> Result<Self, RuntimeError> {
      let mut name: ffi::GLuint = unsafe { mem::uninitialized() };

      unsafe {
         ffi::glGenBuffers(1, &mut name)
      };

      gl_result(Buffer {
         name: name,
         ptr: ptr::null_mut(),
         size: 0,
      })
   }

   #[inline]
   pub fn bind(&self) -> VoidResult {
      unsafe {
         ffi::glBindBuffer(ffi::GL_PIXEL_UNPACK_BUFFER, self.name)
      };

      gl_result(())
   }

   #[inline]
   pub fn unbind(&self) -> VoidResult {
      unsafe {
         ffi::glBindBuffer(ffi::GL_PIXEL_UNPACK_BUFFER, 0)
      };

      gl_result(())
   }

   #[inline]
   pub fn init_data(&mut self, size: usize) -> VoidResult {
      self.size = size as ffi::GLsizeiptr;
      unsafe {
         ffi::glBufferData(
            ffi::GL_PIXEL_UNPACK_BUFFER,
            self.size,
            ptr::null(),
            ffi::GL_STREAM_DRAW
         )
      };

      gl_result(())
   }

   #[inline]
   pub fn map(&mut self) -> VoidResult {
      self.ptr = unsafe {
         ffi::glMapBuffer(ffi::GL_PIXEL_UNPACK_BUFFER, ffi::GL_WRITE_ONLY)
      };

      gl_result(())
   }

   #[inline]
   pub fn map_range(&mut self) -> VoidResult {
      self.ptr = unsafe {
         ffi::glMapBufferRange(
            ffi::GL_PIXEL_UNPACK_BUFFER, 0, self.size,
            ffi::GL_MAP_WRITE_BIT// | ffi::GL_MAP_UNSYNCHRONIZED_BIT
         )
      };

      gl_result(())
   }

   #[inline]
   pub fn unmap(&self) -> VoidResult {
      unsafe {
         ffi::glUnmapBuffer(ffi::GL_PIXEL_UNPACK_BUFFER)
      };

      gl_result(())
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
   pub fn new(shader_type: ffi::GLenum) -> Result<Self, RuntimeError> {
      let name = unsafe {
         ffi::glCreateShader(shader_type)
      };

      gl_result(Shader {
         name: name,
      })
   }

   #[inline]
   pub fn shader_source(&self, string: &str) -> VoidResult {
      let cstring = try!(CString::new(string));

      unsafe {
         ffi::glShaderSource(self.name, 1, [cstring.as_ptr()].as_ptr(), ptr::null())
      };

      gl_result(())
   }

   #[inline]
   pub fn compile(&self) -> VoidResult {
      unsafe {
         ffi::glCompileShader(self.name)
      };

      if !try!(self.is_compiled()) {
         return gl_error("Shader not compiled")
      }

      Ok(())
   }

   #[inline]
   pub fn is_compiled(&self) -> Result<bool, RuntimeError> {
      let mut compiled: ffi::GLint = unsafe { mem::uninitialized() };

      unsafe {
         ffi::glGetShaderiv(self.name, ffi::GL_COMPILE_STATUS, &mut compiled);
      };

      gl_result(compiled == ffi::GL_TRUE as ffi::GLint)
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
   pub fn new() -> Result<Self, RuntimeError> {
      let name = unsafe {
         ffi::glCreateProgram()
      };

      gl_result(Program {
         name: name,
      })
   }

   #[inline]
   pub fn attach_shader(&self, shader: &Shader) -> VoidResult {
      unsafe {
         ffi::glAttachShader(self.name, shader.name)
      };

      gl_result(())
   }

   #[inline]
   pub fn link(&self) -> VoidResult {
      unsafe {
         ffi::glLinkProgram(self.name)
      };

      if !try!(self.is_linked()) {
         return gl_error("Program not linked")
      }

      Ok(())
   }

   #[inline]
   pub fn use_program(&self) -> VoidResult {
      unsafe {
         ffi::glUseProgram(self.name)
      };

      gl_result(())
   }

   #[inline]
   pub fn is_linked(&self) -> Result<bool, RuntimeError> {
      let mut linked: ffi::GLint = unsafe { mem::uninitialized() };

      unsafe {
         ffi::glGetProgramiv(self.name, ffi::GL_LINK_STATUS, &mut linked);
      };

      gl_result(linked == ffi::GL_TRUE as ffi::GLint)
   }

   #[inline]
   pub fn get_attrib_location(&self, attrib_name: &str) -> Result<ffi::GLint, RuntimeError> {
      let cname = try!(CString::new(attrib_name));

      let result = unsafe {
         ffi::glGetAttribLocation(self.name, cname.as_ptr())
      };

      gl_result(result)
   }

   #[inline]
   pub fn get_uniform_location(&self, variable_name: &str) -> Result<ffi::GLint, RuntimeError> {
      let cname = try!(CString::new(variable_name));

      let result = unsafe {
         ffi::glGetUniformLocation(self.name, cname.as_ptr())
      };

      gl_result(result)
   }
}

impl Drop for Program {
   fn drop (&mut self) {
      unsafe {
         ffi::glDeleteProgram(self.name)
      };
   }
}
