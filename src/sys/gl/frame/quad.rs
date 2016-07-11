use std::iter::repeat;

use error::{RuntimeError, VoidResult};
use frame::GPUFrame;
use draw::RGB;

use super::super::ffi;
use super::super::{
   Texture, Framebuffer, Shader, Program, viewport, vertex_attrib_pointer,
   enable_vertex_attrib_array, uniform_value_1i, draw_arrays,
};


pub struct QuadFrame {
   pub texture: Texture,
   pub framebuffer: Framebuffer,
   pub program: Program,
   pub vertex_attrib: ffi::GLint,
   pub tex_coord_attrib: ffi::GLint,
   pub tex_sampler_uniform: ffi::GLint,
   pub data: Vec<u8>,
}

impl GPUFrame for QuadFrame {
   #[inline]
   fn new(width: u32, height: u32) -> Result<Self, RuntimeError> {
      let data = Self::create_data(width, height);

      let texture = try!(Texture::new(width, height));
      let framebuffer = try!(Framebuffer::new(&texture));

      try!(texture.bind());
      try!(framebuffer.bind());

      let (program, vertex_attrib, tex_coord_attrib, tex_sampler_uniform) = try!(Self::create_program());

      Ok(QuadFrame {
         texture: texture,
         framebuffer: framebuffer,
         data: data,
         program: program,
         vertex_attrib: vertex_attrib,
         tex_coord_attrib: tex_coord_attrib,
         tex_sampler_uniform: tex_sampler_uniform,
      })
   }

   #[inline]
   fn clear(&mut self) {
      for item in self.data.iter_mut() {
         *item = 0;
      }
   }

   #[inline]
   fn put_pixel(&mut self, x: i32, y: i32, color: &RGB, width: u32, height: u32) {
      if x >= width as i32 || y >= height as i32 || x < 0 || y < 0 {
         return;
      }

      let i = 4 * (x + y * width as i32) as usize;
      self.data[i    ] = color.r;
      self.data[i + 1] = color.g;
      self.data[i + 2] = color.b;
   }

   #[inline]
   fn resize(&mut self, width: u32, height: u32) -> VoidResult {
      self.data.resize((width * height * 4) as usize, 0);
      self.texture.resize(width, height)
   }

   #[inline]
   fn pre_render(&mut self) -> VoidResult {
      Ok(())
   }

   #[inline]
   fn post_render(&mut self, width: u32, height: u32) -> VoidResult {
      let quad_vertices = [
         -1.0, -1.0, 0.0,
         1.0, -1.0, 0.0,
         -1.0, 1.0, 0.0,
         1.0, 1.0, 0.0,
      ];

      let quad_tex_coords = [
         0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0,
      ];

      try!(self.texture.update(width, height, &self.data));

      try!(viewport(width, height));

      try!(self.program.use_program());

      try!(vertex_attrib_pointer(
         self.vertex_attrib as ffi::GLuint,
         3,
         &quad_vertices
      ));

      try!(vertex_attrib_pointer(
         self.tex_coord_attrib as ffi::GLuint,
         2,
         &quad_tex_coords
      ));

      try!(enable_vertex_attrib_array(self.vertex_attrib as ffi::GLuint));

      try!(enable_vertex_attrib_array(self.tex_coord_attrib as ffi::GLuint));

      try!(uniform_value_1i(self.tex_sampler_uniform, 0));

      draw_arrays(4)
   }
}

impl QuadFrame {
   #[inline]
   pub fn create_data(width: u32, height: u32) -> Vec<u8> {
      repeat(0u8)
         .take((width * height * 4) as usize)
         .collect::<Vec<_>>()
   }

   fn create_program() -> Result<(Program, ffi::GLint, ffi::GLint, ffi::GLint), RuntimeError> {
      let vertex_src = "
         attribute vec4 vertex;
         attribute vec2 texCoord;
         varying vec2 vTexCoord;
         void main()
         {
            gl_Position = vertex;
            vTexCoord = texCoord;
         }
      ";

      let fragment_src = "
         #ifdef GL_ES
            precision mediump float;
         #endif
         varying vec2 vTexCoord;
         uniform sampler2D texSampler;
         void main()
         {
            gl_FragColor = texture2D(texSampler, vTexCoord);
         }
      ";

      let vertex = try!(Shader::new(ffi::GL_VERTEX_SHADER));

      try!(vertex.shader_source(&vertex_src));

      try!(vertex.compile());

      let fragment = try!(Shader::new(ffi::GL_FRAGMENT_SHADER));

      try!(fragment.shader_source(&fragment_src));

      try!(fragment.compile());

      let program = try!(Program::new());

      try!(program.attach_shader(&vertex));
      try!(program.attach_shader(&fragment));

      try!(program.link());

      let vertex_attrib = try!(program.get_attrib_location("vertex"));

      let tex_coord_attrib = try!(program.get_attrib_location("texCoord"));

      let tex_sampler_uniform = try!(program.get_uniform_location("texSampler"));

      Ok((program, vertex_attrib, tex_coord_attrib, tex_sampler_uniform))
   }
}
