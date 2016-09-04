extern crate polydraw;

use polydraw::Application;
use polydraw::node::{
   NodeRenderer, Data, AddOp, JoinOp, ListOp, PolyOp, LayerOp, ArtboardOp, NodeBuilder, Inlet,
};


fn main() {
   let points = Data::VI64I64(
      vec![(0, 0), (90, 1200), (261, 1735), (1443, 410)]
   );

   let color = Data::U8U8U8(
      (0, 127, 255)
   );

   let mut builder = NodeBuilder::new();

   builder.data("poly-points", points);

   builder.operator::<JoinOp>("translate-point", vec![
      Inlet::Source("frame"),
      Inlet::Data(Data::I64(0)),
   ]);

   builder.operator::<AddOp>("add-operator", vec![
      Inlet::Source("poly-points"),
      Inlet::Source("translate-point"),
   ]);

   builder.operator::<PolyOp>("poly", vec![
      Inlet::Source("add-operator"),
      Inlet::Data(color),
   ]);

   builder.operator::<ListOp>("poly-list", vec![
      Inlet::Source("poly"),
   ]);

   builder.operator::<LayerOp>("layer", vec![
      Inlet::Source("poly-list"),
   ]);

   builder.operator::<ArtboardOp>("artboard", vec![
      Inlet::Source("layer"),
   ]);

   let mut renderer = NodeRenderer::new(builder);


   Application::new()
      .renderer(&mut renderer)
      .title("Program")
      .size(1200, 800)
      .run();
}
