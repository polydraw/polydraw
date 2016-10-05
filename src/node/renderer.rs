use devel::{DevelRenderer, Scene};
use renderer::Renderer;
use frame::Frame;

use super::data::Data;
use super::builder::{NodeBuilder, Program};


pub struct NodeRenderer {
   renderer: DevelRenderer,
   frame: i64,
   frame_index: usize,
   program: Program,
}

impl NodeRenderer {
   #[inline]
   pub fn new(mut builder: NodeBuilder) -> Self {
      let frame_index = builder.input(String::from("frame"));

      let program = builder.compile();

      NodeRenderer {
         renderer: DevelRenderer::new(Scene::new()),
         frame: 0,
         frame_index: frame_index,
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
      self.program.input(self.frame_index, Data::Int(self.frame));

      let result = self.program.execute();

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

