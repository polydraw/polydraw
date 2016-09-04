use devel::{DevelRenderer, Scene};
use renderer::Renderer;
use frame::Frame;

use super::data::Data;
use super::builder::{NodeBuilder, NodeScene};


pub struct NodeRenderer {
   renderer: DevelRenderer,
   frame: i64,
   node_scene: NodeScene,
}

impl NodeRenderer {
   #[inline]
   pub fn new(mut builder: NodeBuilder) -> Self {
      let node_scene = builder.compile();

      NodeRenderer {
         renderer: DevelRenderer::new(Scene::new()),
         frame: 0,
         node_scene: node_scene,
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
      for frame_state in self.node_scene.state[0].iter_mut() {
         *frame_state = Data::I64(self.frame);
      }

      for node in &self.node_scene.nodes {
         node.process(&mut self.node_scene.state);
      }

      let mut scene = Scene::new();

      if let Data::VLayer(ref artboard) = self.node_scene.state[self.node_scene.artboard_slot][0] {
         for layer in artboard {
            for poly in &layer.polys {
               scene.push(poly.clone());
            }
         }
      }

      self.renderer.set_scene(scene);

      self.renderer.render(frame);

      self.frame += 1;
   }
}

