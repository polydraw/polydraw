pub mod ffi;

use std::mem;
use std::ptr;

use super::utils::fn_ptr::FnPtrLoader;

#[inline]
pub fn load<T: FnPtrLoader>(loader: &T) {
   unsafe {
      ffi::load_functions(loader)
   };
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
pub fn finish() {
   unsafe {
      ffi::glFinish();
   }
}

#[inline]
pub fn reset_pixelstore_alignment() {
   unsafe {
      ffi::glPixelStorei(ffi::GL_UNPACK_ALIGNMENT, 1);
   }
}

#[inline]
pub fn enable_framebuffer_srgb() {
   unsafe {
      ffi::glEnable(ffi::GL_FRAMEBUFFER_SRGB);
   }
}

#[inline]
pub fn disable_all() {
   unsafe {
      ffi::glDisable(ffi::GL_SCISSOR_TEST);
      ffi::glDisable(ffi::GL_BLEND);
      ffi::glDisable(ffi::GL_CULL_FACE);
      ffi::glDisable(ffi::GL_DEPTH_TEST);
      ffi::glDisable(ffi::GL_DITHER);
      ffi::glDisable(ffi::GL_POLYGON_OFFSET_FILL);
      ffi::glDisable(ffi::GL_SAMPLE_ALPHA_TO_COVERAGE);
      ffi::glDisable(ffi::GL_SAMPLE_COVERAGE);
      ffi::glDisable(ffi::GL_STENCIL_TEST);
   };
}

pub struct Texture {
   pub name: ffi::GLuint,
}

impl Texture {
   pub fn new(width: u32, height: u32) -> Self {
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

      Texture {
         name: name,
      }
   }

   #[inline]
   pub fn bind(&self) {
      unsafe {
         ffi::glBindTexture(ffi::GL_TEXTURE_2D, self.name);
      }
   }

   #[inline]
   pub fn resize(&self, width: u32, height: u32) {
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
   }

   #[inline]
   pub fn update(&self, width: u32, height: u32) {
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
   pub fn new(texture: &Texture) -> Self {
      let mut name: ffi::GLuint = unsafe { mem::uninitialized() };

      unsafe {
         ffi::glGenFramebuffers(1, &mut name)
      };

      let framebuffer = Framebuffer {
         name: name,
      };

      framebuffer.bind();
      framebuffer.attach_texture(texture);
      framebuffer.unbind();

      framebuffer
   }

   #[inline]
   pub fn bind(&self) {
      unsafe {
         ffi::glBindFramebuffer(ffi::GL_READ_FRAMEBUFFER, self.name)
      };
   }

   #[inline]
   pub fn unbind(&self) {
      unsafe {
         ffi::glBindFramebuffer(ffi::GL_READ_FRAMEBUFFER, 0)
      };
   }

   #[inline]
   pub fn attach_texture(&self, texture: &Texture) {
      unsafe {
         ffi::glFramebufferTexture2D(
            ffi::GL_READ_FRAMEBUFFER,
            ffi::GL_COLOR_ATTACHMENT0,
            ffi::GL_TEXTURE_2D,
            texture.name,
            0
         )
      };
   }

   #[inline]
   pub fn blit(&self, width: u32, height: u32) {
      unsafe {
         ffi::glBlitFramebuffer(
            0, 0, width as ffi::GLint, height as ffi::GLint,
            0, 0, width as ffi::GLint, height as ffi::GLint,
            ffi::GL_COLOR_BUFFER_BIT,
            ffi::GL_NEAREST
         )
      };
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
}

impl Drop for Shader {
   fn drop (&mut self) {
      unsafe {
         ffi::glDeleteShader(self.name)
      };
   }
}
