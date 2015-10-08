pub mod ffi;

use std::mem;
use std::ptr;

use super::utils::fn_ptr::FnPtrLoader;
use super::super::draw::RGB;

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
   pub fn update(&self, width: u32, height: u32, buffer: &Buffer) {
      unsafe {
         ffi::glTexSubImage2D(
            ffi::GL_TEXTURE_2D,
            0,
            0, 0, width as ffi::GLsizei, height as ffi::GLsizei,
            ffi::GL_RGBA,
            ffi::GL_UNSIGNED_BYTE,
            buffer.ptr as *const ffi::c_void
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
   pub ptr: *mut u8,
   pub width: u32,
   pub height: u32,
   pub max_width: u32,
   pub max_height: u32,
}

impl Buffer {
   pub fn new(width: u32, height: u32, screen_width: u32, screen_height: u32) -> Self {
      let mut name: ffi::GLuint = unsafe { mem::uninitialized() };

      unsafe {
         ffi::glGenBuffers(1, &mut name)
      };

      Buffer {
         name: name,
         ptr: ptr::null_mut(),
         width: width,
         height: height,
         max_width: screen_width,
         max_height: screen_height,
      }
   }

   #[inline]
   pub fn resize(&mut self, width: u32, height: u32) {
      self.width = width;
      self.height = height;
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
   pub fn data(&self) {
      unsafe {
         ffi::glBufferData(
            ffi::GL_PIXEL_UNPACK_BUFFER,
            (self.max_width * self.max_height * 32) as ffi::GLsizeiptr,
            ptr::null(),
            ffi::GL_STREAM_DRAW
         )
      };
   }

   #[inline]
   pub fn map(&mut self) {
      self.ptr = unsafe {
         ffi::glMapBuffer(ffi::GL_PIXEL_UNPACK_BUFFER, ffi::GL_WRITE_ONLY) as *mut u8
      };
   }

   #[inline]
   pub fn unmap(&self) {
      unsafe {
         ffi::glUnmapBuffer(ffi::GL_PIXEL_UNPACK_BUFFER)
      };
   }

   #[inline]
   pub fn clear(&mut self) {
      unsafe {
         ptr::write_bytes(self.ptr as *mut u8, 0, (self.max_width * self.max_height * 4) as usize)
      };
   }

   pub fn put_pixel(&mut self, x: i32, y: i32, color: &RGB) {
      if x >= self.width as i32 || y >= self.height as i32 || x < 0 || y < 0 {
         return;
      }

      let i = 4 * (x + y * self.width as i32) as isize;
      unsafe {
         *self.ptr.offset(i) = color.r;
         *self.ptr.offset(i+1) = color.g;
         *self.ptr.offset(i+2) = color.b;
      }
   }
}

impl Drop for Buffer {
   fn drop (&mut self) {
      unsafe {
         ffi::glDeleteBuffers(1, &self.name)
      };
   }
}
