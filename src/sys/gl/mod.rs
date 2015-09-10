pub mod ffi;

use std::mem;
use std::ptr;

use super::utils::fn_ptr::FnPtrLoader;

#[inline]
pub fn load<T: FnPtrLoader>(loader: T) {
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
pub fn reset_pixelstore_alignment() {
   unsafe {
      ffi::glPixelStorei(ffi::GL_UNPACK_ALIGNMENT, 1);
   }
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
            ffi::GL_RGB as ffi::GLint,
            width as ffi::GLsizei,
            height as ffi::GLsizei,
            0,
            ffi::GL_RGB,
            ffi::GL_UNSIGNED_BYTE,
            ptr::null()
         );
      }

      Texture {
         name: name,
      }
   }

   pub fn resize(&self, width: u32, height: u32) {
      unsafe {
         ffi::glBindTexture(ffi::GL_TEXTURE_2D, self.name);

         ffi::glTexImage2D(
            ffi::GL_TEXTURE_2D,
            0,
            ffi::GL_RGB as ffi::GLint,
            width as ffi::GLsizei,
            height as ffi::GLsizei,
            0,
            ffi::GL_RGB,
            ffi::GL_UNSIGNED_BYTE,
            ptr::null()
         );
      }
   }

   pub fn update(&self, width: u32, height: u32, data: &[u8]) {
      unsafe {
         ffi::glBindTexture(ffi::GL_TEXTURE_2D, self.name);

         ffi::glTexSubImage2D(
            ffi::GL_TEXTURE_2D,
            0,
            0, 0, width as ffi::GLsizei, height as ffi::GLsizei,
            ffi::GL_RGB,
            ffi::GL_UNSIGNED_BYTE,
            data.as_ptr() as *const ffi::GLvoid
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
