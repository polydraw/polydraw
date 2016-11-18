extern crate polydraw;

use polydraw::Application;
use polydraw::draw::RGB;
use polydraw::data::IntPoint;
use polydraw::node::{
   NodeRenderer, Data, Add, BuildPoint, BuildList, BuildPoly, BuildLayer,
   BuildArtboard, ProgramBuilder, Inlet, Rotate, Nth, Multiply, Divide,
};


fn main() {
   let points = Data::PointList(
      Box::new(vec![
         IntPoint::new(0, 0),
         IntPoint::new(90, 1200),
         IntPoint::new(261, 1735),
         IntPoint::new(1443, 410)
      ])
   );

   let color = Data::Rgb(RGB::new(0, 127, 255));

   let mut builder = ProgramBuilder::new();

   builder.function(String::from("main"), vec![
      String::from("frame"), String::from("width"), String::from("height")
   ]);

   builder.data(String::from("center"), Data::Point(IntPoint::new(1500, 600)));

   builder.data(String::from("segment"), Data::Int(120));

   builder.operator(Nth::new(), String::from("center-x"), vec![
      Inlet::Source(String::from("center")),
      Inlet::Data(Data::Int(0)),
   ]);

   builder.operator(Nth::new(), String::from("center-y"), vec![
      Inlet::Source(String::from("center")),
      Inlet::Data(Data::Int(1)),
   ]);

   // p1 = center.x, center.y - 2 * segment

   builder.operator(Multiply::new(), String::from("double-segment"), vec![
      Inlet::Source(String::from("segment")),
      Inlet::Data(Data::Int(2)),
   ]);

   builder.operator(Divide::new(), String::from("p1-y"), vec![
      Inlet::Source(String::from("center-y")),
      Inlet::Source(String::from("double-segment")),
   ]);

   builder.operator(BuildPoint::new(), String::from("p1"), vec![
      Inlet::Source(String::from("center-x")),
      Inlet::Source(String::from("p1-y")),
   ]);

   builder.data(String::from("poly-points"), points);

/*
   builder.operator::<Center>(String::from("poly-center"), vec![
      Inlet::Source("poly-points"),
   ]);
*/

   builder.operator(Rotate::new(), String::from("rotated-points"), vec![
      Inlet::Source(String::from("poly-points")),
      Inlet::Source(String::from("p1")),
      Inlet::Source(String::from("frame")),
   ]);

   builder.operator(BuildPoint::new(), String::from("translate-point"), vec![
      Inlet::Source(String::from("frame")),
      Inlet::Data(Data::Int(0)),
   ]);

   builder.operator(Add::new(), String::from("add-operator"), vec![
      Inlet::Source(String::from("rotated-points")),
      Inlet::Source(String::from("translate-point")),
   ]);

   builder.operator(BuildPoly::new(), String::from("poly"), vec![
      Inlet::Source(String::from("add-operator")),
      Inlet::Data(color),
   ]);

   builder.operator(BuildList::new(), String::from("poly-list"), vec![
      Inlet::Source(String::from("poly")),
   ]);

   builder.operator(BuildLayer::new(), String::from("layer"), vec![
      Inlet::Source(String::from("poly-list")),
   ]);

   builder.operator(BuildArtboard::new(), String::from("result"), vec![
      Inlet::Source(String::from("layer")),
   ]);

   let mut renderer = NodeRenderer::new(builder);


   Application::new()
      .renderer(&mut renderer)
      .title("Program")
      .size(1200, 800)
      .run();
}
