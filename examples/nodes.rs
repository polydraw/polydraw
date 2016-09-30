extern crate polydraw;

use polydraw::Application;
use polydraw::draw::RGB;
use polydraw::geom::point::Point;
use polydraw::node::{
   NodeRenderer, Data, Add, BuildPoint, BuildList, BuildPoly, BuildLayer,
   BuildArtboard, NodeBuilder, Inlet, Rotate, Nth, Multiply, Divide,
};


fn main() {
   let points = Data::PointList(
      Box::new(vec![
         Point::new(0, 0),
         Point::new(90, 1200),
         Point::new(261, 1735),
         Point::new(1443, 410)
      ])
   );

   let color = Data::Rgb(RGB::new(0, 127, 255));

   let mut builder = NodeBuilder::new();

   builder.data("center", Data::Point(Point::new(1500, 600)));

   builder.data("segment", Data::Int(120));

   builder.operator::<Nth>("center-x", vec![
      Inlet::Source("center"),
      Inlet::Data(Data::Int(0)),
   ]);

   builder.operator::<Nth>("center-y", vec![
      Inlet::Source("center"),
      Inlet::Data(Data::Int(1)),
   ]);

   // p1 = center.x, center.y - 2 * segment

   builder.operator::<Multiply>("double-segment", vec![
      Inlet::Source("segment"),
      Inlet::Data(Data::Int(2)),
   ]);

   builder.operator::<Divide>("p1-y", vec![
      Inlet::Source("center-y"),
      Inlet::Source("double-segment"),
   ]);

   builder.operator::<BuildPoint>("p1", vec![
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

   builder.operator::<BuildPoint>("translate-point", vec![
      Inlet::Source("frame"),
      Inlet::Data(Data::Int(0)),
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
