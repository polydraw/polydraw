use devel::{DevelRenderer, Scene};
use renderer::Renderer;
use frame::Frame;

use super::data::Data;
use super::builder::{ProgramBuilder, Program};


pub struct NodeRenderer {
   renderer: DevelRenderer,
   frame: i64,
   program: Program,
}

impl NodeRenderer {
   #[inline]
   pub fn new(builder: ProgramBuilder) -> Self {
      let program = builder.compile();

      NodeRenderer {
         renderer: DevelRenderer::new(Scene::new()),
         frame: 0,
         program: program,
      }
   }
}

impl Renderer for NodeRenderer {
   #[inline]
   fn init(&mut self, width: u32, height: u32) {
      self.renderer.init(width, height);
   }

   #[inline]
   fn render(&mut self, frame: &mut Frame) {
      let result = self.program.execute(vec![Data::Int(self.frame)]);

      let mut scene = Scene::new();

      if let Data::LayerList(ref artboard) = result {
         for layer in artboard.iter() {
            for poly in &layer.polys {
               scene.push(Box::new(poly.clone()));
            }
         }
      }

      self.renderer.set_scene(scene);

      self.renderer.render(frame);

      self.frame += 1;
   }
}

