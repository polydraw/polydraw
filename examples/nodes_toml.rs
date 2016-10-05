extern crate polydraw;
extern crate toml;

use polydraw::Application;
use polydraw::node::{
   NodeRenderer, Data, Add, BuildPoint, BuildList, BuildPoly, BuildLayer,
   BuildArtboard, NodeBuilder, Inlet, Operator,
};
use polydraw::draw::RGB;
use polydraw::geom::point::Point;


const NODE_DEFS: &'static str = r#"

   [poly-points]
   type = "point-list"
   data = [ [0, 0], [90, 1200], [261, 1735], [1443, 410] ]

   [translate-point]
   type = "point"
   data = [
      { from = "frame" },
      { type = "i64", data = 0 } ]

   [add-operator]
   type = "add"
   data = [
      { from = "poly-points" },
      { from = "translate-point" } ]

   [poly]
   type = "poly"
   data = [
      { from = "add-operator" },
      { type = "rgb", data = [0, 127, 255] } ]

   [poly-list]
   type = "list"
   data = [
      { from = "poly" } ]

   [layer]
   type = "layer"
   data = [
      { from = "poly-list" } ]

   [result]
   type = "artboard"
   data = [
      { from = "layer" } ]

"#;



fn parse(node_defs: &str) -> NodeBuilder {
   let mut builder = NodeBuilder::new();

   let mut parser = toml::Parser::new(node_defs);

   if let Some(all_tables) = parser.parse() {
      for (node_id, value) in all_tables.iter() {
         if let &toml::Value::Table(ref node_table) = value {

            process_node_table(&mut builder, node_id, node_table);

         } else {
            panic!("`{}` is not a table ", node_id);
         }
      }
   }

   builder
}


fn process_node_table(
   builder: &mut NodeBuilder,
   node_id: &str,
   node_table: &toml::Table,
) {

   let node_type = extract_node_type(node_id, node_table);

   match node_type.as_ref() {
      "add" => create_operator_node::<Add>(builder, node_id, node_table),
      "point" => create_operator_node::<BuildPoint>(builder, node_id, node_table),
      "list" => create_operator_node::<BuildList>(builder, node_id, node_table),

      "poly" => create_operator_node::<BuildPoly>(builder, node_id, node_table),
      "layer" => create_operator_node::<BuildLayer>(builder, node_id, node_table),
      "artboard" => create_operator_node::<BuildArtboard>(builder, node_id, node_table),

      _ => create_data_node(builder, node_id, node_table),
   }
}


fn create_operator_node<T: 'static + Operator>(
   builder: &mut NodeBuilder,
   node_id: &str,
   node_table: &toml::Table,
) {

   let data_value = extract_data_value(node_id, node_table);

   let inlets = node_inlets(node_id, data_value);

   builder.operator::<T>(String::from(node_id), inlets);
}


fn create_data_node(
   builder: &mut NodeBuilder,
   node_id: &str,
   node_table: &toml::Table,
) {
   let data = extract_table_data(node_id, node_table);

   builder.data(String::from(node_id), data);
}


fn extract_node_type<'a>(node_id: &str, node_table: &'a toml::Table) -> &'a str {
   if let Some(type_value) = node_table.get("type") {
      match type_value {
         &toml::Value::String(ref node_type) => node_type,
         _ => {
            panic!("node type not a string: {}", node_id);
         }
      }
   } else {
      panic!("node without type: {}", node_id);
   }
}


fn extract_data_value<'a>(node_id: &str, node_table: &'a toml::Table) -> &'a toml::Value {
   match node_table.get("data") {
      Some(data_value) => {
         data_value
      },
      None => {
         panic!("node without data: {}", node_id);
      }
   }
}


fn node_inlets<'a>(node_id: &'a str, data: &'a toml::Value) -> Vec<Inlet> {

   let array = match data {
      &toml::Value::Array(ref array) => array,
      _ => {
         panic!("data is not an array: {}", node_id);
      }
   };

   let mut inlets = Vec::with_capacity(array.len());

   for item in array.iter() {
      let table = match item {
         &toml::Value::Table(ref table) => table,
         _ => {
            panic!("value is not a table {:?}: {}", item, node_id);
         }
      };

      match table.get("from") {
         Some(from) => {

            let source_id = match from {
               &toml::Value::String(ref source_id) => source_id,
               _ => {
                  panic!("From is not a string {:?}: {}", from, node_id);
               }
            };

            inlets.push(
               Inlet::Source(source_id.clone()),
            );
         },
         None => {
            let data = extract_table_data(node_id, table);

            inlets.push(
               Inlet::Data(data)
            );
         }
      }
   }

   inlets
}


fn extract_table_data(node_id: &str, table: &toml::Table) -> Data {
   let type_str = match table.get("type") {
      Some(type_data) => {
         match type_data {
            &toml::Value::String(ref type_str) => type_str,
            _ => {
               panic!("Type not a string {:?}: {}", type_data, node_id);
            }
         }
      },
      None => {
         panic!("value table without a type {:?}: {}", table, node_id);
      }
   };

   let data = match table.get("data") {
      Some(data) => data,
      None => {
         panic!("value table without a data {:?}: {}", table, node_id);
      }
   };

   match type_str.as_ref() {
      "i64" => toml_to_i64(node_id, data),
      "rgb" => toml_to_rgb(node_id, data),
      "point-list" => toml_to_point_list(node_id, data),
      _ => {
         panic!("Unknown data type {}: {}", type_str, node_id);
      }
   }
}


fn toml_to_i64(node_id: &str, data: &toml::Value) -> Data {
   Data::Int(extract_i64(node_id, data))
}


fn toml_to_rgb(node_id: &str, data: &toml::Value) -> Data {
   match data {
      &toml::Value::Array(ref array) => {
         if array.len() != 3 {
            panic!("Not a triple {:?}: {}", array, node_id);
         }

         let first = extract_u8(node_id, &array[0]);
         let second = extract_u8(node_id, &array[1]);
         let third = extract_u8(node_id, &array[2]);

         Data::Rgb(RGB::new(first, second, third))
      },
      _ => {
         panic!("Value not an array {:?}: {}", data, node_id);
      }
   }
}


fn toml_to_point_list(node_id: &str, data: &toml::Value) -> Data {
   match data {
      &toml::Value::Array(ref array) => {
         let mut container = Vec::with_capacity(array.len());

         for inner_array in array {
            match inner_array {
               &toml::Value::Array(ref pair) => {
                  if pair.len() != 2 {
                     panic!("Not a pair {:?}: {}", pair, node_id);
                  }

                  let left = extract_i64(node_id, &pair[0]);
                  let right = extract_i64(node_id, &pair[1]);

                  container.push(Point::new(left, right));
               },
               _ => {
                  panic!("Value not an array {:?}: {}", inner_array, node_id);
               }
            }
         }

         Data::PointList(Box::new(container))
      },
      _ => {
         panic!("Value not an array {:?}: {}", data, node_id);
      }
   }
}


fn extract_i64(node_id: &str, data: &toml::Value) -> i64 {
   match data {
      &toml::Value::Integer(value) => value,
      _ => {
         panic!("Not an integer {:?}: {}", data, node_id);
      }
   }
}


fn extract_u8(node_id: &str, data: &toml::Value) -> u8 {
   extract_i64(node_id, data) as u8
}

fn main() {
   let builder = parse(NODE_DEFS);

   let mut renderer = NodeRenderer::new(builder);

   Application::new()
      .renderer(&mut renderer)
      .title("Nodes")
      .size(1200, 800)
      .run();
}

