extern crate polydraw;

use polydraw::Application;
use polydraw::node::{
   NodeRenderer, Data, Add, Join, BuildList, BuildPoly, BuildLayer,
   BuildArtboard, NodeBuilder, Inlet, Center, Rotate, Nth, Multiply, Divide,
};


fn main() {
   let points = Data::VT2I64(
      vec![(0, 0), (90, 1200), (261, 1735), (1443, 410)]
   );

   let color = Data::T3U8(
      (0, 127, 255)
   );

   let mut builder = NodeBuilder::new();

   builder.data("center", Data::T2I64((1500, 600)));

   builder.data("segment", Data::I64(120));

   builder.operator::<Nth>("center-x", vec![
      Inlet::Source("center"),
      Inlet::Data(Data::I64(0)),
   ]);

   builder.operator::<Nth>("center-y", vec![
      Inlet::Source("center"),
      Inlet::Data(Data::I64(1)),
   ]);

   // p1 = center.x, center.y - 2 * segment

   builder.operator::<Multiply>("double-segment", vec![
      Inlet::Source("segment"),
      Inlet::Data(Data::I64(2)),
   ]);

   builder.operator::<Divide>("p1-y", vec![
      Inlet::Source("center-y"),
      Inlet::Source("double-segment"),
   ]);

   builder.operator::<Join>("p1", vec![
      Inlet::Source("center-x"),
      Inlet::Source("p1-y"),
   ]);

   builder.data("poly-points", points);

/*
   builder.operator::<Center>("poly-center", vec![
      Inlet::Source("poly-points"),
   ]);
*/

   builder.operator::<Rotate>("rotated-points", vec![
      Inlet::Source("poly-points"),
      Inlet::Source("p1"),
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
