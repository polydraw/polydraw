use node::{
   Data, Add, BuildPoint, BuildList, ProgramBuilder, Inlet, Center, Rotate,
   Multiply, Divide, SourceOperator, Subtract, BuildRgb, BBox, Equal, Unequal,
   Less, LessEqual, Greater, GreaterEqual, Gate, FunctionOperator, Polar, Each,
   EachWithLast, BuildPoly, ListType, BuildLayer,
};
use node::{
   eval_add, eval_divide, eval_multiply, eval_subtract, eval_rotate, eval_bbox,
   eval_center, eval_rgb, eval_equal, eval_unequal, eval_less, eval_less_equal,
   eval_greater, eval_greater_equal, eval_gate, eval_polar, eval_poly,
   eval_layer,
};
use data::IntPoint;

use super::parser::{
   Ast, ListBox, List, PointBox, PointDef, FunctionCallBox, FunctionCall,
   BinaryBox, Binary, Assignment, BinaryType, Function,
};


const EXEC_FUNCS: [&'static str; 12] = [
   "add",
   "divide",
   "multiply",
   "subtract",
   "polar",
   "rotate",
   "center",
   "bbox",
   "rgb",
   "gate",
   "poly",
   "layer",
];


pub fn compile(builder: &mut ProgramBuilder, ast_list: Vec<Ast>) {
   for ast in ast_list {
      if let Ast::Function(function) = ast {
         let Function {name, arguments, assignments} = {*function};

         builder.function(name, arguments);

         for ast in assignments {
            if let Ast::Assignment(assignment) = ast {
               build_assignment(builder, *assignment);
            }
         }
      }
   }
}


fn build_assignment(builder: &mut ProgramBuilder, assignment: Assignment) {
   let Assignment {node_id, value} = assignment;
   match value {
      Ast::Name(value) => builder.operator(
         SourceOperator::new(), node_id, vec![Inlet::Source(value)]
      ),
      Ast::Int(value) => builder.data(node_id, Data::Int(value)),
      Ast::Float(value) => builder.data(node_id, Data::Float(value)),
      Ast::Bool(value) => builder.data(node_id, Data::Bool(value)),
      Ast::FunctionRef(value) => builder.data(node_id, Data::FunctionRef(value)),
      Ast::Point(value) => build_point(builder, node_id, value),
      Ast::Binary(value) => build_binary(builder, node_id, value),
      Ast::List(value) => build_list(builder, node_id, value),
      Ast::FunctionCall(value) => build_function_call(builder, node_id, value),
      _ => {},
   }
}

fn build_anon_node(builder: &mut ProgramBuilder, element: Ast) -> Inlet {
   match element {
      Ast::Name(value) => Inlet::Source(value),
      Ast::Int(value) => Inlet::Data(Data::Int(value)),
      Ast::Float(value) => Inlet::Data(Data::Float(value)),
      Ast::Bool(value) => Inlet::Data(Data::Bool(value)),
      Ast::FunctionRef(value) => Inlet::Data(Data::FunctionRef(value)),
      Ast::Point(value) => build_anon_point(builder, value),
      Ast::Binary(value) => build_anon_binary(builder, value),
      Ast::List(value) => build_anon_list(builder, value),
      Ast::FunctionCall(value) => build_anon_function(builder, value),
      _ => Inlet::None,
   }
}


fn build_point(builder: &mut ProgramBuilder, node_id: String, point: PointBox) {
   let PointDef {x, y} = {*point};

   match (x, y) {
      (Ast::Int(x), Ast::Int(y)) => {
         builder.data(node_id, Data::Point(IntPoint::new(x, y)))
      },
      (x, y) => {
         let x_inlet = build_anon_node(builder, x);
         let y_inlet = build_anon_node(builder, y);

         builder.operator(BuildPoint::new(), node_id, vec![x_inlet, y_inlet])
      },
   }
}

fn build_anon_point(builder: &mut ProgramBuilder, point: PointBox) -> Inlet {
   let PointDef {x, y} = {*point};

   match (x, y) {
      (Ast::Int(x), Ast::Int(y)) => {
         Inlet::Data(Data::Point(IntPoint::new(x, y)))
      },
      (x, y) => {
         let x_inlet = build_anon_node(builder, x);
         let y_inlet = build_anon_node(builder, y);

         builder.anonymous(BuildPoint::new(), vec![x_inlet, y_inlet])
      },
   }
}


fn build_binary(builder: &mut ProgramBuilder, node_id: String, binary: BinaryBox) {
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
            BinaryType::Subtract => builder.operator(Subtract::new(), node_id, inlets),
            BinaryType::Add => builder.operator(Add::new(), node_id, inlets),
            BinaryType::Divide => builder.operator(Divide::new(), node_id, inlets),
            BinaryType::Multiply => builder.operator(Multiply::new(), node_id, inlets),
            BinaryType::Equal => builder.operator(Equal::new(), node_id, inlets),
            BinaryType::Unequal => builder.operator(Unequal::new(), node_id, inlets),
            BinaryType::Less => builder.operator(Less::new(), node_id, inlets),
            BinaryType::LessEqual => builder.operator(LessEqual::new(), node_id, inlets),
            BinaryType::Greater => builder.operator(Greater::new(), node_id, inlets),
            BinaryType::GreaterEqual => builder.operator(GreaterEqual::new(), node_id, inlets),
         }
      },
   }
}

fn build_anon_binary(builder: &mut ProgramBuilder, binary: BinaryBox) -> Inlet {
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
            BinaryType::Subtract => builder.anonymous(Subtract::new(), inlets),
            BinaryType::Add => builder.anonymous(Add::new(), inlets),
            BinaryType::Divide => builder.anonymous(Divide::new(), inlets),
            BinaryType::Multiply => builder.anonymous(Multiply::new(), inlets),
            BinaryType::Equal => builder.anonymous(Equal::new(), inlets),
            BinaryType::Unequal => builder.anonymous(Unequal::new(), inlets),
            BinaryType::Less => builder.anonymous(Less::new(), inlets),
            BinaryType::LessEqual => builder.anonymous(LessEqual::new(), inlets),
            BinaryType::Greater => builder.anonymous(Greater::new(), inlets),
            BinaryType::GreaterEqual => builder.anonymous(GreaterEqual::new(), inlets),
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
      BinaryType::Equal => eval_equal(left, right),
      BinaryType::Unequal => eval_unequal(left, right),
      BinaryType::Less => eval_less(left, right),
      BinaryType::LessEqual => eval_less_equal(left, right),
      BinaryType::Greater => eval_greater(left, right),
      BinaryType::GreaterEqual => eval_greater_equal(left, right),
   }
}


pub type Eval1ArgFn = fn(in1: Data) -> Data;
pub type Eval2ArgFn = fn(in1: Data, in2: Data) -> Data;
pub type Eval3ArgFn = fn(in1: Data, in2: Data, in3: Data) -> Data;


fn build_function_call(
   builder: &mut ProgramBuilder, node_id: String, function: FunctionCallBox
) {

   let FunctionCall {name, arguments} = {*function};

   let (inlets, data_only) = function_inlets(builder, arguments);

   if data_only && EXEC_FUNCS.contains(&(&name as &str)) {
      builder.data(node_id, exec_data_only(name, inlets));
      return;
   };

   match &name as &str {
      "add" => builder.operator(Add::new(), node_id, inlets),
      "divide" => builder.operator(Divide::new(), node_id, inlets),
      "multiply" => builder.operator(Multiply::new(), node_id, inlets),
      "subtract" => builder.operator(Subtract::new(), node_id, inlets),
      "polar" => builder.operator(Polar::new(), node_id, inlets),
      "rotate" => builder.operator(Rotate::new(), node_id, inlets),
      "center" => builder.operator(Center::new(), node_id, inlets),
      "bbox" => builder.operator(BBox::new(), node_id, inlets),
      "rgb" => builder.operator(BuildRgb::new(), node_id, inlets),
      "gate" => builder.operator(Gate::new(), node_id, inlets),
      "poly" => builder.operator(BuildPoly::new(), node_id, inlets),
      "layer" => builder.operator(BuildLayer::new(), node_id, inlets),
      "each" => builder.operator(Each::new(), node_id, inlets),
      "each-with-last" => builder.operator(EachWithLast::new(), node_id, inlets),
      _ => builder.operator(FunctionOperator::new(name), node_id, inlets),
   }
}

fn build_anon_function(builder: &mut ProgramBuilder, function: FunctionCallBox) -> Inlet {
   let FunctionCall {name, arguments} = {*function};

   let (inlets, data_only) = function_inlets(builder, arguments);

   if data_only && EXEC_FUNCS.contains(&(&name as &str)) {
      return Inlet::Data(exec_data_only(name, inlets));
   }

   match &name as &str {
      "add" => builder.anonymous(Add::new(), inlets),
      "divide" => builder.anonymous(Divide::new(), inlets),
      "multiply" => builder.anonymous(Multiply::new(), inlets),
      "subtract" => builder.anonymous(Subtract::new(), inlets),
      "polar" => builder.anonymous(Polar::new(), inlets),
      "rotate" => builder.anonymous(Rotate::new(), inlets),
      "center" => builder.anonymous(Center::new(), inlets),
      "bbox" => builder.anonymous(BBox::new(), inlets),
      "rgb" => builder.anonymous(BuildRgb::new(), inlets),
      "gate" => builder.anonymous(Gate::new(), inlets),
      "poly" => builder.anonymous(BuildPoly::new(), inlets),
      "layer" => builder.anonymous(BuildLayer::new(), inlets),
      "each" => builder.anonymous(Each::new(), inlets),
      "each-with-last" => builder.anonymous(EachWithLast::new(), inlets),
      _ => builder.anonymous(FunctionOperator::new(name), inlets),
   }
}

fn function_inlets(builder: &mut ProgramBuilder, arguments: Vec<Ast>) -> (Vec<Inlet>, bool) {
   let mut inlets = Vec::with_capacity(arguments.len());

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
      "polar" => exec_2_arg_fn(eval_polar, inlets),
      "rotate" => exec_3_arg_fn(eval_rotate, inlets),
      "center" => exec_1_arg_fn(eval_center, inlets),
      "bbox" => exec_1_arg_fn(eval_bbox, inlets),
      "rgb" => exec_3_arg_fn(eval_rgb, inlets),
      "gate" => exec_2_arg_fn(eval_gate, inlets),
      "poly" => exec_2_arg_fn(eval_poly, inlets),
      "layer" => exec_1_arg_fn(eval_layer, inlets),
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
   let mut arguments = Vec::with_capacity(inlets.len());

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

fn build_anon_list(builder: &mut ProgramBuilder, list: ListBox) -> Inlet {
   let (list_type, inlets) = list_inlets(builder, list);

   match list_type {
      ListType::Int => Inlet::Data(create_int_list(inlets)),
      ListType::Float => Inlet::Data(create_float_list(inlets)),
      ListType::Bool => Inlet::Data(create_bool_list(inlets)),
      ListType::Point => Inlet::Data(create_point_list(inlets)),
      ListType::PointList => Inlet::Data(create_point_list_list(inlets)),
      ListType::Rgb => Inlet::Data(create_rgb_list(inlets)),
      ListType::Poly => Inlet::Data(create_poly_list(inlets)),
      ListType::Layer => Inlet::Data(create_layer_list(inlets)),
      ListType::Data => Inlet::Data(create_data_list(inlets)),
      ListType::Source => builder.anonymous(BuildList::new(), inlets),
      _ => Inlet::None
   }
}

fn build_list(builder: &mut ProgramBuilder, node_id: String, list: ListBox) {
   let (list_type, inlets) = list_inlets(builder, list);

   match list_type {
      ListType::Int => builder.data(node_id, create_int_list(inlets)),
      ListType::Float => builder.data(node_id, create_float_list(inlets)),
      ListType::Bool => builder.data(node_id, create_bool_list(inlets)),
      ListType::Point => builder.data(node_id, create_point_list(inlets)),
      ListType::PointList => builder.data(node_id, create_point_list_list(inlets)),
      ListType::Rgb => builder.data(node_id, create_rgb_list(inlets)),
      ListType::Poly => builder.data(node_id, create_poly_list(inlets)),
      ListType::Layer => builder.data(node_id, create_layer_list(inlets)),
      ListType::Data => builder.data(node_id, create_data_list(inlets)),
      ListType::Source => builder.operator(BuildList::new(), node_id, inlets),
      _ => {}
   }
}

fn list_inlets(builder: &mut ProgramBuilder, list: ListBox) -> (ListType, Vec<Inlet>) {
   let List {contents} = {*list};

   let mut inlets = Vec::with_capacity(contents.len());

   let mut list_type = ListType::None;

   for element in contents {
      let inlet = build_anon_node(builder, element);

      list_type = update_inlet_list_type(list_type, &inlet);

      inlets.push(inlet);
   }

   (list_type, inlets)
}

fn inlet_list_type(inlet: &Inlet) -> ListType {
   match inlet {
      &Inlet::Data(ref data) => match data {
         &Data::Int(_) => ListType::Int,
         &Data::Float(_) => ListType::Float,
         &Data::Bool(_) => ListType::Bool,
         &Data::Point(_) => ListType::Point,
         &Data::PointList(_) => ListType::PointList,
         &Data::Rgb(_) => ListType::Rgb,
         &Data::Poly(_) => ListType::Poly,
         &Data::Layer(_) => ListType::Layer,
         &Data::None => ListType::None,
         _ => ListType::Data,
      },
      &Inlet::Source(_) => ListType::Source,
      &Inlet::None => ListType::None,
   }
}

fn update_inlet_list_type(current: ListType, inlet: &Inlet) -> ListType {
   if current == ListType::Source {
      ListType::Source
   } else {
      let new = inlet_list_type(inlet);

      if new == ListType::Source {
         ListType::Source
      } else {
         match current {
            ListType::None => new,
            _ => if new == current {
               new
            } else {
               ListType::Data
            }
         }
      }
   }
}


macro_rules! create_list {
   ($name:ident, $data_enum:path, $list_enum:path) => {
      fn $name(inlets: Vec<Inlet>) -> Data {
         let mut list = Vec::with_capacity(inlets.len());

         for inlet in inlets {
            if let Inlet::Data(data) = inlet {
               if let $data_enum(value) = data {
                  list.push(value);
               }
            }
         }

         $list_enum(Box::new(list))
      }
   }
}

macro_rules! create_list_boxed {
   ($name:ident, $data_enum:path, $list_enum:path) => {
      fn $name(inlets: Vec<Inlet>) -> Data {
         let mut list = Vec::with_capacity(inlets.len());

         for inlet in inlets {
            if let Inlet::Data(data) = inlet {
               if let $data_enum(value) = data {
                  list.push(*value);
               }
            }
         }

         $list_enum(Box::new(list))
      }
   }
}

create_list!(create_int_list, Data::Int, Data::IntList);
create_list!(create_float_list, Data::Float, Data::FloatList);
create_list!(create_bool_list, Data::Bool, Data::BoolList);
create_list!(create_point_list, Data::Point, Data::PointList);
create_list!(create_rgb_list, Data::Rgb, Data::RgbList);
create_list_boxed!(create_poly_list, Data::Poly, Data::PolyList);
create_list_boxed!(create_layer_list, Data::Layer, Data::LayerList);
create_list_boxed!(create_point_list_list, Data::PointList, Data::PointListList);


fn create_data_list(inlets: Vec<Inlet>) -> Data {
   let mut list = Vec::with_capacity(inlets.len());

   for inlet in inlets {
      if let Inlet::Data(data) = inlet {
         list.push(data);
      }
   }

   Data::DataList(Box::new(list))
}

