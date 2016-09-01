extern crate polydraw;
extern crate toml;

use std::collections::HashMap;

use polydraw::Application;
use polydraw::node::{
   Node, NodeRenderer, Data, NODE_INDEX_OFFSET, create_state, execution_sort,
   Operator, DataOp, AddOp, JoinOp, ListOp, PolyOp, LayerOp, ArtboardOp,
};


const NODE_DEFS: &'static str = r#"

   [poly-points]
   type = "[(i64, i64)]"
   data = [ [0, 0], [90, 1200], [261, 1735], [1443, 410] ]

   [translate-point]
   type = "join"
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
      { type = "(u8, u8, u8)", data = [0, 127, 255] } ]

   [poly-list]
   type = "list"
   data = [
      { from = "poly" } ]

   [layer]
   type = "layer"
   data = [
      { from = "poly-list" } ]

   [artboard]
   type = "artboard"
   data = [
      { from = "layer" } ]

"#;



fn parse(node_defs: &str) -> (Vec<Node>, Vec<Vec<Data>>, usize) {
   let mut parser = toml::Parser::new(node_defs);

   if let Some(all_tables) = parser.parse() {
      let mut slot_map = HashMap::new();

      // Data::frame number at slot 0
      slot_map.insert("frame", 0);

      for (i, node_id) in all_tables.keys().enumerate() {
         let slot = i + NODE_INDEX_OFFSET;
         slot_map.insert(node_id.as_str(), slot);
      }

      let mut state = create_state(all_tables.len());

      let mut nodes = Vec::new();

      let mut artboard_slot = 0;

      for (i, (node_id, value)) in all_tables.iter().enumerate() {
         if let &toml::Value::Table(ref node_table) = value {
            let slot = i + NODE_INDEX_OFFSET;

            let result = process_node_table(
               node_id, slot, node_table, &slot_map, &mut state
            );

            if let Some((node, is_final)) = result {
               if is_final {
                  artboard_slot = node.slot;
               }

               nodes.push(node);
            }
         } else {
            panic!("`{}` is not a table ", node_id);
         }
      }

      let nodes = execution_sort(nodes);

      state[artboard_slot].push(Data::None);

      return (nodes, state, artboard_slot);
   }

   panic!("parse errors: {:?}", parser.errors);
}

fn process_node_table(
   node_id: &str,
   node_index: usize,
   node_table: &toml::Table,
   slot_map: &HashMap<&str, usize>,
   state: &mut Vec<Vec<Data>>,
) -> Option<(Node, bool)> {

   let node_type = extract_node_type(node_id, node_table);

   let operator = match node_type.as_ref() {
      "add" => create_operator::<AddOp>(),
      "join" => create_operator::<JoinOp>(),
      "list" => create_operator::<ListOp>(),

      "poly" => create_operator::<PolyOp>(),
      "layer" => create_operator::<LayerOp>(),
      "artboard" => create_operator::<ArtboardOp>(),

      _ => None,
   };

   if let Some(operator) = operator {
      let node = create_node(node_id, node_index, node_table, slot_map, operator, state);

      let is_final = node_type == "artboard";

      Some((node, is_final))
   } else {
      let node = create_data_node(node_id, node_index, node_table);

      Some((node, false))
   }
}


fn create_operator<T: 'static + Operator>() -> Option<Box<Operator>> {
   Some(Box::new(T::new()))
}


fn create_node(
   node_id: &str,
   node_index: usize,
   node_table: &toml::Table,
   slot_map: &HashMap<&str, usize>,
   operator: Box<Operator>,
   state: &mut Vec<Vec<Data>>,
) -> Node {

   let data_value = extract_data_value(node_id, node_table);

   let (consts, inlets) = to_defaults(node_id, data_value, slot_map, state);

   Node::new(operator, consts, inlets, node_index)
}


fn create_data_node(
   node_id: &str,
   node_index: usize,
   node_table: &toml::Table,
) -> Node {
   let data = extract_table_data(node_id, node_table);

   let operator = Box::new(DataOp::new());

   let consts = vec![data];
   let inlets = vec![None];

   Node::new(operator, consts, inlets, node_index)
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


fn to_defaults(
   node_id: &str,
   data: &toml::Value,
   slot_map: &HashMap<&str, usize>,
   state: &mut Vec<Vec<Data>>,
) -> (Vec<Data>, Vec<Option<(usize, usize)>>) {

   let array = match data {
      &toml::Value::Array(ref array) => array,
      _ => {
         panic!("data is not an array: {}", node_id);
      }
   };

   let mut consts = Vec::with_capacity(array.len());
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

            let in_id = match from {
               &toml::Value::String(ref in_id) => in_id,
               _ => {
                  panic!("From is not a string {:?}: {}", from, node_id);
               }
            };

            let slot = match slot_map.get::<str>(in_id) {
               Some(slot) => slot,
               _ => {
                  panic!("Unrecognized ID {:?}: {}", in_id, node_id);
               }
            };

            let subslot = state[*slot].len();

            state[*slot].push(Data::None);

            inlets.push(Some((*slot, subslot)));

            consts.push(Data::None);

         },
         None => {
            consts.push(
               extract_table_data(node_id, table)
            );

            inlets.push(None);
         }
      }
   }

   (consts, inlets)
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
      "(u8, u8, u8)" => toml_to_u8u8u8(node_id, data),
      "[(i64, i64)]" => toml_to_vi64i64(node_id, data),
      _ => {
         panic!("Unknown data type {}: {}", type_str, node_id);
      }
   }
}


fn toml_to_i64(node_id: &str, data: &toml::Value) -> Data {
   Data::I64(extract_i64(node_id, data))
}


fn toml_to_u8u8u8(node_id: &str, data: &toml::Value) -> Data {
   match data {
      &toml::Value::Array(ref array) => {
         if array.len() != 3 {
            panic!("Not a triple {:?}: {}", array, node_id);
         }

         let first = extract_u8(node_id, &array[0]);
         let second = extract_u8(node_id, &array[1]);
         let third = extract_u8(node_id, &array[2]);

         Data::U8U8U8((first, second, third))
      },
      _ => {
         panic!("Value not an array {:?}: {}", data, node_id);
      }
   }
}


fn toml_to_vi64i64(node_id: &str, data: &toml::Value) -> Data {
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

                  container.push((left, right));
               },
               _ => {
                  panic!("Value not an array {:?}: {}", inner_array, node_id);
               }
            }
         }

         Data::VI64I64(container)
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
   let (nodes, state, artboard_slot) = parse(NODE_DEFS);

   let mut renderer = NodeRenderer::new(nodes, state, artboard_slot);

   Application::new()
      .renderer(&mut renderer)
      .title("Nodes")
      .size(1200, 800)
      .run();
}

