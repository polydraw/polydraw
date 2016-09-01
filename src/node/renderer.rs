use devel::{DevelRenderer, Scene};
use renderer::Renderer;
use frame::Frame;

use super::data::Data;
use super::node::Node;


pub struct NodeRenderer {
   renderer: DevelRenderer,
   frame: i64,
   nodes: Vec<Node>,
   state: Vec<Vec<Data>>,
   artboard_slot: usize,
}

impl NodeRenderer {
   #[inline]
   pub fn new(nodes: Vec<Node>, state: Vec<Vec<Data>>, artboard_slot: usize) -> Self {
      NodeRenderer {
         renderer: DevelRenderer::new(Scene::new()),
         frame: 0,
         nodes: nodes,
         state: state,
         artboard_slot: artboard_slot,
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
      self.state[0][0] = Data::I64(self.frame);

      for node in &self.nodes {
         node.process(&mut self.state);
      }

      let mut scene = Scene::new();

      if let Data::VLayer(ref artboard) = self.state[self.artboard_slot][0] {
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

