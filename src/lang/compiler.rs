use node::{
   Data, Add, BuildPoint, BuildList, NodeBuilder, Inlet, Center, Rotate,
   Multiply, Divide, SourceOperator, Subtract, BuildRgb, BuildBBox,
};
use node::{
   eval_add, eval_divide, eval_multiply, eval_subtract, eval_rotate, eval_bbox,
   eval_center, eval_rgb,
};
use geom::point::Point;

use super::parser::{
   Ast, ListBox, List, PointBox, PointDef, FunctionBox, Function, BinaryBox,
   Binary, Assignment, BinaryType,
};


pub fn compile(builder: &mut NodeBuilder, ast_list: Vec<Ast>) {
   for ast in ast_list {
      if let Ast::Assignment(assignment) = ast {
         build_assignment(builder, *assignment);
      }
   }
}


fn build_assignment(builder: &mut NodeBuilder, assignment: Assignment) {
   let Assignment {node_id, value} = assignment;
   match value {
      Ast::Name(value) => builder.operator::<SourceOperator>(
         node_id, vec![Inlet::Source(value)]
      ),
      Ast::Int(value) => builder.data(node_id, Data::Int(value)),
      Ast::Float(value) => builder.data(node_id, Data::Float(value)),
      Ast::Point(value) => build_point(builder, node_id, value),
      Ast::Binary(value) => build_binary(builder, node_id, value),
      Ast::List(value) => build_list(builder, node_id, value),
      Ast::Function(value) => build_function(builder, node_id, value),
      _ => {},
   }
}

fn build_anon_node(builder: &mut NodeBuilder, element: Ast) -> Inlet {
   match element {
      Ast::Name(value) => Inlet::Source(value),
      Ast::Int(value) => Inlet::Data(Data::Int(value)),
      Ast::Float(value) => Inlet::Data(Data::Float(value)),
      Ast::Point(value) => build_anon_point(builder, value),
      Ast::Binary(value) => build_anon_binary(builder, value),
      Ast::List(value) => build_anon_list(builder, value),
      Ast::Function(value) => build_anon_function(builder, value),
      _ => Inlet::None,
   }
}


fn build_point(builder: &mut NodeBuilder, node_id: String, point: PointBox) {
   let PointDef {x, y} = {*point};

   match (x, y) {
      (Ast::Int(x), Ast::Int(y)) => {
         builder.data(node_id, Data::Point(Point::new(x, y)))
      },
      (x, y) => {
         let x_inlet = build_anon_node(builder, x);
         let y_inlet = build_anon_node(builder, y);

         builder.operator::<BuildPoint>(node_id, vec![x_inlet, y_inlet])
      },
   }
}

fn build_anon_point(builder: &mut NodeBuilder, point: PointBox) -> Inlet {
   let PointDef {x, y} = {*point};

   match (x, y) {
      (Ast::Int(x), Ast::Int(y)) => {
         Inlet::Data(Data::Point(Point::new(x, y)))
      },
      (x, y) => {
         let x_inlet = build_anon_node(builder, x);
         let y_inlet = build_anon_node(builder, y);

         builder.anonymous::<BuildPoint>(vec![x_inlet, y_inlet])
      },
   }
}


fn build_binary(builder: &mut NodeBuilder, node_id: String, binary: BinaryBox) {
   let Binary {operator, left, right} = {*binary};

   let left_inlet = build_anon_node(builder, left);
   let right_inlet = build_anon_node(builder, right);

   match (left_inlet, right_inlet) {
      (Inlet::Data(left), Inlet::Data(right)) => {
         builder.data(node_id, exec_binary(operator, left, right));
      },
      (left_inlet, right_inlet) => {
         let inlets = vec![left_inlet, right_inlet];
         match operator {
            BinaryType::Subtract => builder.operator::<Subtract>(node_id, inlets),
            BinaryType::Add => builder.operator::<Add>(node_id, inlets),
            BinaryType::Divide => builder.operator::<Divide>(node_id, inlets),
            BinaryType::Multiply => builder.operator::<Multiply>(node_id, inlets),
         }
      },
   }
}

fn build_anon_binary(builder: &mut NodeBuilder, binary: BinaryBox) -> Inlet {
   let Binary {operator, left, right} = {*binary};

   let left_inlet = build_anon_node(builder, left);
   let right_inlet = build_anon_node(builder, right);

   match (left_inlet, right_inlet) {
      (Inlet::Data(left), Inlet::Data(right)) => {
         Inlet::Data(exec_binary(operator, left, right))
      },
      (left_inlet, right_inlet) => {
         let inlets = vec![left_inlet, right_inlet];
         match operator {
            BinaryType::Subtract => builder.anonymous::<Subtract>(inlets),
            BinaryType::Add => builder.anonymous::<Add>(inlets),
            BinaryType::Divide => builder.anonymous::<Divide>(inlets),
            BinaryType::Multiply => builder.anonymous::<Multiply>(inlets),
         }
      },
   }
}

fn exec_binary(binary_type: BinaryType, left: Data, right: Data) -> Data {
   match binary_type {
      BinaryType::Subtract => eval_subtract(left, right),
      BinaryType::Add => eval_add(left, right),
      BinaryType::Divide => eval_divide(left, right),
      BinaryType::Multiply => eval_multiply(left, right),
   }
}


pub type Eval1ArgFn = fn(in1: Data) -> Data;
pub type Eval2ArgFn = fn(in1: Data, in2: Data) -> Data;
pub type Eval3ArgFn = fn(in1: Data, in2: Data, in3: Data) -> Data;


fn build_function(builder: &mut NodeBuilder, node_id: String, function: FunctionBox) {
   let Function {name, arguments} = {*function};

   let (inlets, data_only) = function_inlets(builder, arguments);

   if data_only {
      builder.data(node_id, exec_data_only(name, inlets));
      return;
   };

   match &name as &str {
      "add" => builder.operator::<Add>(node_id, inlets),
      "divide" => builder.operator::<Divide>(node_id, inlets),
      "multiply" => builder.operator::<Multiply>(node_id, inlets),
      "subtract" => builder.operator::<Subtract>(node_id, inlets),
      "rotate" => builder.operator::<Rotate>(node_id, inlets),
      "center" => builder.operator::<Center>(node_id, inlets),
      "bbox" => builder.operator::<BuildBBox>(node_id, inlets),
      "rgb" => builder.operator::<BuildRgb>(node_id, inlets),
      _ => panic!("Unrecognized function {}", name),
   }
}

fn build_anon_function(builder: &mut NodeBuilder, function: FunctionBox) -> Inlet {
   let Function {name, arguments} = {*function};

   let (inlets, data_only) = function_inlets(builder, arguments);

   if data_only {
      return Inlet::Data(exec_data_only(name, inlets));
   }

   match &name as &str {
      "add" => builder.anonymous::<Add>(inlets),
      "divide" => builder.anonymous::<Divide>(inlets),
      "multiply" => builder.anonymous::<Multiply>(inlets),
      "subtract" => builder.anonymous::<Subtract>(inlets),
      "rotate" => builder.anonymous::<Rotate>(inlets),
      "center" => builder.anonymous::<Center>(inlets),
      "bbox" => builder.anonymous::<BuildBBox>(inlets),
      "rgb" => builder.anonymous::<BuildRgb>(inlets),
      _ => panic!("Unrecognized function {}", name),
   }
}

fn function_inlets(builder: &mut NodeBuilder, arguments: Vec<Ast>) -> (Vec<Inlet>, bool) {
   let mut inlets = Vec::new();

   let mut data_only = true;

   for argument in arguments {
      let inlet = build_anon_node(builder, argument);

      if let Inlet::Data(_) = inlet {} else {
         data_only = false;
      }

      inlets.push(inlet);
   }

   (inlets, data_only)
}

fn exec_data_only(name: String, inlets: Vec<Inlet>) -> Data {
   match &name as &str {
      "add" => exec_2_arg_fn(eval_add, inlets),
      "divide" => exec_2_arg_fn(eval_divide, inlets),
      "multiply" => exec_2_arg_fn(eval_multiply, inlets),
      "subtract" => exec_2_arg_fn(eval_subtract, inlets),
      "rotate" => exec_3_arg_fn(eval_rotate, inlets),
      "center" => exec_1_arg_fn(eval_center, inlets),
      "bbox" => exec_1_arg_fn(eval_bbox, inlets),
      "rgb" => exec_3_arg_fn(eval_rgb, inlets),
      _ => panic!("Unrecognized function {}", name),
   }
}

fn exec_1_arg_fn(function: Eval1ArgFn, inlets: Vec<Inlet>) -> Data {
   let mut arguments = arguments_from_inlets(inlets, 1);

   let arg1 = arguments.pop().unwrap();

   function(arg1)
}

fn exec_2_arg_fn(function: Eval2ArgFn, inlets: Vec<Inlet>) -> Data {
   let mut arguments = arguments_from_inlets(inlets, 2);

   let arg2 = arguments.pop().unwrap();
   let arg1 = arguments.pop().unwrap();

   function(arg1, arg2)
}

fn exec_3_arg_fn(function: Eval3ArgFn, inlets: Vec<Inlet>) -> Data {
   let mut arguments = arguments_from_inlets(inlets, 3);

   let arg3 = arguments.pop().unwrap();
   let arg2 = arguments.pop().unwrap();
   let arg1 = arguments.pop().unwrap();

   function(arg1, arg2, arg3)
}

fn arguments_from_inlets(inlets: Vec<Inlet>, count: usize) -> Vec<Data> {
   let mut arguments = Vec::new();

   for inlet in inlets {
      if let Inlet::Data(data) = inlet {
         arguments.push(data);
      }
   }

   for _ in arguments.len()..count {
      arguments.push(Data::None);
   }

   arguments
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListType {
   None,
   Int,
   Float,
   Bool,
   Point,
   PointList,
   Rgb,
   Poly,
   Source,
}

fn build_anon_list(builder: &mut NodeBuilder, list: ListBox) -> Inlet {
   let (list_type, inlets) = list_inlets(builder, list);

   match list_type {
      ListType::Int => Inlet::Data(create_int_list(inlets)),
      ListType::Float => Inlet::Data(create_float_list(inlets)),
      ListType::Bool => Inlet::Data(create_bool_list(inlets)),
      ListType::Point => Inlet::Data(create_point_list(inlets)),
      ListType::PointList => Inlet::Data(create_point_list_list(inlets)),
      ListType::Source => builder.anonymous::<BuildList>(inlets),
      _ => Inlet::None
   }
}

fn build_list(builder: &mut NodeBuilder, node_id: String, list: ListBox) {
   let (list_type, inlets) = list_inlets(builder, list);

   match list_type {
      ListType::Int => builder.data(node_id, create_int_list(inlets)),
      ListType::Float => builder.data(node_id, create_float_list(inlets)),
      ListType::Bool => builder.data(node_id, create_bool_list(inlets)),
      ListType::Point => builder.data(node_id, create_point_list(inlets)),
      ListType::PointList => builder.data(node_id, create_point_list_list(inlets)),
      ListType::Rgb => builder.data(node_id, create_rgb_list(inlets)),
      ListType::Source => builder.operator::<BuildList>(node_id, inlets),
      _ => {}
   }
}

fn list_inlets(builder: &mut NodeBuilder, list: ListBox) -> (ListType, Vec<Inlet>) {
   let List {contents} = {*list};

   let mut inlets = Vec::new();

   let mut list_type_option = None;

   for element in contents {
      let inlet = build_anon_node(builder, element);

      let element_type = element_type(&inlet);

      if element_type == ListType::None {
         panic!("Wrong list element type {:?}: {:?}", element_type, inlet);
      }

      match list_type_option {
         Some(ref mut list_type) => {
            if *list_type != ListType::Source {
               if element_type != ListType::Source && element_type != *list_type  {
                  panic!("Wrong list element {:?}: {:?}", element_type, inlet);
               }

               *list_type = element_type;
            }
         },
         None => {
            list_type_option = Some(element_type);
         }
      }

      inlets.push(inlet);
   }

   if inlets.len() == 0 {
      panic!("Empty list definition");
   }

   let list_type = list_type_option.unwrap();

   (list_type, inlets)
}

fn element_type(inlet: &Inlet) -> ListType {
   match inlet {
      &Inlet::Data(ref data) => match data {
         &Data::Int(_) => ListType::Int,
         &Data::Float(_) => ListType::Float,
         &Data::Bool(_) => ListType::Bool,
         &Data::Point(_) => ListType::Point,
         &Data::PointList(_) => ListType::PointList,
         &Data::Rgb(_) => ListType::Rgb,
         &Data::Poly(_) => ListType::Poly,
         _ => ListType::None,
      },
      &Inlet::Source(_) => ListType::Source,
      &Inlet::None => ListType::None,
   }
}

fn create_int_list(inlets: Vec<Inlet>) -> Data {
   let mut list = Vec::new();

   for inlet in inlets {
      if let Inlet::Data(data) = inlet {
         if let Data::Int(value) = data {
            list.push(value);
         }
      }
   }

   Data::IntList(Box::new(list))
}

fn create_float_list(inlets: Vec<Inlet>) -> Data {
   let mut list = Vec::new();

   for inlet in inlets {
      if let Inlet::Data(data) = inlet {
         if let Data::Float(value) = data {
            list.push(value);
         }
      }
   }

   Data::FloatList(Box::new(list))
}

fn create_bool_list(inlets: Vec<Inlet>) -> Data {
   let mut list = Vec::new();

   for inlet in inlets {
      if let Inlet::Data(data) = inlet {
         if let Data::Bool(value) = data {
            list.push(value);
         }
      }
   }

   Data::BoolList(Box::new(list))
}

fn create_point_list(inlets: Vec<Inlet>) -> Data {
   let mut list = Vec::new();

   for inlet in inlets {
      if let Inlet::Data(data) = inlet {
         if let Data::Point(value) = data {
            list.push(value);
         }
      }
   }

   Data::PointList(Box::new(list))
}

fn create_point_list_list(inlets: Vec<Inlet>) -> Data {
   let mut list = Vec::new();

   for inlet in inlets {
      if let Inlet::Data(data) = inlet {
         if let Data::PointList(value) = data {
            list.push(*value);
         }
      }
   }

   Data::PointListList(Box::new(list))
}

fn create_rgb_list(inlets: Vec<Inlet>) -> Data {
   let mut list = Vec::new();

   for inlet in inlets {
      if let Inlet::Data(data) = inlet {
         if let Data::Rgb(value) = data {
            list.push(value);
         }
      }
   }

   Data::RgbList(Box::new(list))
}
