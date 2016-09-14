extern crate polydraw;

use polydraw::Application;
use polydraw::node::{
   NodeRenderer, Data, Add, Join, BuildList, BuildPoly, BuildLayer,
   BuildArtboard, NodeBuilder, Inlet, Center, Rotate,
};


fn main() {
   let points = Data::VT2I64(
      vec![(0, 0), (90, 1200), (261, 1735), (1443, 410)]
   );

   let color = Data::T3U8(
      (0, 127, 255)
   );

   let mut builder = NodeBuilder::new();

   builder.data("poly-points", points);

   builder.operator::<Center>("poly-center", vec![
      Inlet::Source("poly-points"),
   ]);

   builder.operator::<Rotate>("rotated-points", vec![
      Inlet::Source("poly-points"),
      Inlet::Source("poly-center"),
      Inlet::Source("frame"),
   ]);

   builder.operator::<Join>("translate-point", vec![
      Inlet::Source("frame"),
      Inlet::Data(Data::I64(0)),
   ]);

   builder.operator::<Add>("add-operator", vec![
      Inlet::Source("rotated-points"),
      Inlet::Source("translate-point"),
   ]);

   builder.operator::<BuildPoly>("poly", vec![
      Inlet::Source("add-operator"),
      Inlet::Data(color),
   ]);

   builder.operator::<BuildList>("poly-list", vec![
      Inlet::Source("poly"),
   ]);

   builder.operator::<BuildLayer>("layer", vec![
      Inlet::Source("poly-list"),
   ]);

   builder.operator::<BuildArtboard>("artboard", vec![
      Inlet::Source("layer"),
   ]);

   let mut renderer = NodeRenderer::new(builder);


   Application::new()
      .renderer(&mut renderer)
      .title("Program")
      .size(1200, 800)
      .run();
}
