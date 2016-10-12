use std::fmt;

use super::tokenizer::Token;


#[derive(PartialEq, Clone, Debug)]
pub struct Assignment {
   pub node_id: String,
   pub value: Ast,
}

impl Assignment {
   pub fn new(node_id: String, value: Ast) -> Box<Self> {
      Box::new(
         Assignment {
            node_id: node_id,
            value: value
         }
      )
   }
}

pub type AssignmentBox = Box<Assignment>;


#[derive(PartialEq, Clone, Debug)]
pub struct List {
   pub contents: Vec<Ast>,
}

impl List {
   pub fn new(contents: Vec<Ast>) -> Box<Self> {
      Box::new(
         List {
            contents: contents
         }
      )
   }
}

pub type ListBox = Box<List>;


#[derive(PartialEq, Clone, Debug)]
pub struct PointDef {
   pub x: Ast,
   pub y: Ast,
}

impl PointDef {
   pub fn new(x: Ast, y: Ast) -> Box<Self> {
      Box::new(
         PointDef {
            x: x,
            y: y,
         }
      )
   }
}

pub type PointBox = Box<PointDef>;


#[derive(PartialEq, Clone, Debug)]
pub struct Function {
   pub name: String,
   pub arguments: Vec<String>,
   pub assignments: Vec<Ast>,
}

impl Function {
   pub fn new(
      name: String, arguments: Vec<String>, assignments: Vec<Ast>
   ) -> Box<Self> {
      Box::new(
         Function {
            name: name,
            arguments: arguments,
            assignments: assignments,
         }
      )
   }
}

pub type FunctionBox = Box<Function>;


#[derive(PartialEq, Clone, Debug)]
pub struct FunctionCall {
   pub name: String,
   pub arguments: Vec<Ast>,
}

impl FunctionCall {
   pub fn new(name: String, arguments: Vec<Ast>) -> Box<Self> {
      Box::new(
         FunctionCall {
            name: name,
            arguments: arguments,
         }
      )
   }
}

pub type FunctionCallBox = Box<FunctionCall>;


#[derive(PartialEq, Clone)]
pub enum BinaryType {
   Subtract,
   Add,
   Divide,
   Multiply,
   Equal,
   Unequal,
   Less,
   LessEqual,
   Greater,
   GreaterEqual,
}

impl fmt::Debug for BinaryType {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         &BinaryType::Subtract => write!(f, "-"),
         &BinaryType::Add => write!(f, "+"),
         &BinaryType::Divide => write!(f, "/"),
         &BinaryType::Multiply => write!(f, "*"),
         &BinaryType::Equal => write!(f, "=="),
         &BinaryType::Unequal => write!(f, "!="),
         &BinaryType::Less => write!(f, "<"),
         &BinaryType::LessEqual => write!(f, "<="),
         &BinaryType::Greater => write!(f, ">"),
         &BinaryType::GreaterEqual => write!(f, ">="),
      }
   }
}


#[derive(PartialEq, Clone, Debug)]
pub struct Binary {
   pub operator: BinaryType,
   pub left: Ast,
   pub right: Ast,
}

impl Binary {
   pub fn new(operator: BinaryType, left: Ast, right: Ast) -> Box<Self> {
      Box::new(
         Binary {
            operator: operator,
            left: left,
            right: right,
         }
      )
   }
}

pub type BinaryBox = Box<Binary>;


#[derive(PartialEq, Clone)]
pub enum Ast {
   Name(String),
   Int(i64),
   Float(f64),
   Bool(bool),
   Function(FunctionBox),
   FunctionCall(FunctionCallBox),
   FunctionRef(String),
   Assignment(AssignmentBox),
   List(ListBox),
   Point(PointBox),
   Binary(BinaryBox),
}

impl fmt::Debug for Ast {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         &Ast::Name(ref value) => write!(f, "{}", value),
         &Ast::Int(ref value) => write!(f, "{}", value),
         &Ast::Float(ref value) => write!(f, "{}", value),
         &Ast::Bool(ref value) => write!(f, "{}", value),
         &Ast::Function(ref value) => write!(f, "{} >> {:?}", value.name, value.arguments),
         &Ast::Assignment(ref value) => write!(f, "{} = {:?}", value. node_id, value.value),
         &Ast::List(ref value) => write!(f, "{:?}", value.contents),
         &Ast::FunctionCall(ref value) => write!(f, "{}!{:?}", value.name, value.arguments),
         &Ast::FunctionRef(ref value) => write!(f, "@{}", value),
         &Ast::Point(ref value) => write!(f, "<{:?} {:?}>", value.x, value.y),
         &Ast::Binary(ref value) => write!(f, "({:?} {:?} {:?})", value.left, value.operator, value.right),
      }
   }
}

pub type AstResult = Option<(Ast, usize)>;


pub fn parse(tokens: Vec<Token>) -> Result<Vec<Ast>, String> {
   let mut functions = Vec::new();
   let mut assignments = Vec::new();

   let mut tokens = &tokens[..];
   let mut new_lines = 0;

   let mut current_function: Option<(String, Vec<String>)> = None;

   loop {
      let taken = consume_new_line(tokens);

      new_lines += taken;
      tokens = &tokens[taken..];

      if let Some((name, arguments, taken)) = parse_function_start(tokens) {
         tokens = &tokens[taken..];
         if let Some((current_name, current_arguments)) = current_function {
            let function = Function::new(current_name, current_arguments, assignments);
            functions.push(Ast::Function(function));
         }

         current_function = Some((name, arguments));
         assignments = Vec::new();
      } else if let Some((ast, taken)) = parse_assignment(tokens) {
         tokens = &tokens[taken..];
         assignments.push(ast);
      } else {
         let taken = consume_new_line(tokens);

         tokens = &tokens[taken..];

         if tokens.len() == 0 {
            if let Some((name, arguments)) = current_function {
               let function = Function::new(name, arguments, assignments);
               functions.push(Ast::Function(function));
            }
            return Ok(functions);
         } else {
            return Err(format!("Parse error at line {}", new_lines + 1));
         }
      }
   }
}


fn parse_function_start(tokens: &[Token]) -> Option<(String, Vec<String>, usize)> {
   if tokens.len() < 3 {
      return None;
   }

   let name = match &tokens[0] {
      &Token::Name(ref name) => name,
      _ => return None,
   };

   if tokens[1] != Token::Function {
      return None;
   }

   let mut arguments = Vec::new();

   for i in 2..tokens.len() {
      match &tokens[i] {
         &Token::Name(ref name) => arguments.push(name.clone()),
         &Token::NewLine => return Some((name.clone(), arguments, i)),
         _ => return None,
      }
   }

   None
}


fn parse_assignment(tokens: &[Token]) -> AstResult {
   if tokens.len() < 5 {
      return None;
   }

   if tokens[0] != Token::SpaceOffset {
      return None;
   }

   let name = match &tokens[1] {
      &Token::Name(ref name) => name,
      _ => return None,
   };

   if tokens[2] != Token::Assign {
      return None;
   }

   let next_new_line = find_next_new_line(tokens);

   let tokens = &tokens[3..next_new_line];

   if let Some(value) = match_value(tokens) {
      let assignment = Assignment::new(name.clone(), value);
      Some((Ast::Assignment(assignment), next_new_line))
   } else {
      None
   }
}


fn match_value(tokens: &[Token]) -> Option<Ast> {
   if let Some(ast) = match_list(tokens) {
      Some(ast)
   } else if let Some(ast) = match_parenthesis(tokens) {
      Some(ast)
   } else if let Some(ast) = match_point(tokens) {
      Some(ast)
   } else if let Some(ast) = match_function(tokens) {
      Some(ast)
   } else if let Some(ast) = match_binary(tokens) {
      Some(ast)
   } else if let Some(ast) = match_function_ref(tokens) {
      Some(ast)
   } else if let Some(ast) = match_single(tokens) {
      Some(ast)
   } else {
      None
   }
}


fn match_binary(tokens: &[Token]) -> Option<Ast> {
   if tokens.len() < 3 {
      return None;
   }

   let binary_types = [
      (Token::Subtract, BinaryType::Subtract),
      (Token::Add, BinaryType::Add),
      (Token::Divide, BinaryType::Divide),
      (Token::Multiply, BinaryType::Multiply),
      (Token::Equal, BinaryType::Equal),
      (Token::Unequal, BinaryType::Unequal),
      (Token::AngleBracketLeft, BinaryType::Less),
      (Token::LessEqual, BinaryType::LessEqual),
      (Token::AngleBracketRight, BinaryType::GreaterEqual),
      (Token::GreaterEqual, BinaryType::GreaterEqual),
   ];

   for binary_type in &binary_types {
      for middle in 1..tokens.len() - 1 {
         if binary_type.0 != tokens[middle] {
            continue;
         }

         if let Some(left) = match_value(&tokens[..middle]) {
            if let Some(right) = match_value(&tokens[middle + 1..]) {
               return Some(Ast::Binary(Binary::new(binary_type.1.clone(), left, right)));
            }
         }
      }
   }

   None
}


fn match_function(tokens: &[Token]) -> Option<Ast> {
   if tokens.len() < 3 {
      return None;
   }

   let name = match &tokens[0] {
      &Token::Name(ref name) => name,
      _ => return None,
   };

   if tokens[1] != Token::Not {
      return None;
   }

   let tokens = &tokens[2..];

   if let Some(contents) = match_sequence_contents(tokens) {
      Some(Ast::FunctionCall(FunctionCall::new(name.clone(), contents)))
   } else {
      None
   }
}

fn match_function_ref(tokens: &[Token]) -> Option<Ast> {
   if tokens.len() < 2 {
      return None;
   }

   if tokens[0] != Token::Address {
      return None;
   }

   let name = match &tokens[1] {
      &Token::Name(ref name) => name,
      _ => return None,
   };

   Some(Ast::FunctionRef(name.clone()))
}

fn match_single(tokens: &[Token]) -> Option<Ast> {
   if tokens.len() != 1 {
      return None;
   }

   match &tokens[0] {
      &Token::Name(ref value) => Some(Ast::Name(value.clone())),
      &Token::Int(ref value) => Some(Ast::Int(*value)),
      &Token::Float(ref value) => Some(Ast::Float(*value)),
      &Token::True => Some(Ast::Bool(true)),
      &Token::False => Some(Ast::Bool(false)),
      _ => None
   }
}


fn match_list(tokens: &[Token]) -> Option<Ast> {
   if tokens.len() < 3 {
      return None;
   }

   if tokens[0] != Token::BracketLeft {
      return None;
   }

   if tokens[tokens.len() - 1] != Token::BracketRight {
      return None;
   }

   let tokens = &tokens[1..tokens.len() - 1];

   if let Some(contents) = match_sequence_contents(tokens) {
      Some(Ast::List(List::new(contents)))
   } else {
      None
   }
}


fn match_parenthesis(tokens: &[Token]) -> Option<Ast> {
   if tokens.len() < 3 {
      return None;
   }

   if tokens[0] != Token::ParenLeft {
      return None;
   }

   if tokens[tokens.len() - 1] != Token::ParenRight {
      return None;
   }

   let tokens = &tokens[1..tokens.len() - 1];

   if let Some(ast) = match_value(tokens) {
      Some(ast)
   } else {
      None
   }
}

fn match_point(tokens: &[Token]) -> Option<Ast> {
   if tokens.len() < 4 {
      return None;
   }

   if tokens[0] != Token::AngleBracketLeft {
      return None;
   }

   if tokens[tokens.len() - 1] != Token::AngleBracketRight {
      return None;
   }

   let tokens = &tokens[1..tokens.len() - 1];

   if let Some(contents) = match_sequence_contents(tokens) {
      point_def_from_contents(contents)
   } else {
      None
   }
}

fn point_def_from_contents(mut contents: Vec<Ast>) -> Option<Ast> {
   if contents.len() != 2 {
      None
   } else {
      let y = contents.pop().unwrap();
      let x = contents.pop().unwrap();

      Some(Ast::Point(PointDef::new(x, y)))
   }
}

fn match_sequence_contents(tokens: &[Token]) -> Option<Vec<Ast>> {
   let mut contents = Vec::new();
   let mut tokens = tokens;

   loop {
      if tokens.len() == 0 {
         return Some(contents);
      }

      match try_list_item(tokens) {
         Some((ast, end)) => {
            contents.push(ast);
            tokens = &tokens[end..];
         },
         None => return None
      }
   }
}

fn try_list_item(tokens: &[Token]) -> AstResult {
   if tokens.len() == 0 {
      return None;
   }

   if let Some(end) = try_sequence(tokens, Token::AngleBracketLeft, Token::AngleBracketRight) {
      let tokens = &tokens[1..end];

      if let Some(contents) = match_sequence_contents(tokens) {
         match point_def_from_contents(contents) {
            Some(ast) => return Some((ast, end + 1)),
            _ => return None,
         }
      }

      return None;
   }

   if let Some(end) = try_sequence(tokens, Token::BracketLeft, Token::BracketRight) {
      let tokens = &tokens[1..end];

      if let Some(contents) = match_sequence_contents(tokens) {
         return Some((Ast::List(List::new(contents)), end + 1));
      }

      return None;
   }

   if let Some(end) = try_sequence(tokens, Token::ParenLeft, Token::ParenRight) {
      let tokens = &tokens[1..end];

      if let Some(ast) = match_value(tokens) {
         return Some((ast, end + 1));
      }

      return None;
   }

   if let Some(ast) = match_single(&tokens[..1]) {
      Some((ast, 1))
   } else if let Some(ast) = match_function_ref(&tokens[..2]) {
      Some((ast, 2))
   } else {
      None
   }
}


fn try_sequence(tokens: &[Token], left: Token, right: Token) -> Option<usize> {
   if tokens.len() < 3 {
      return None;
   }

   if tokens[0] != left {
      return None;
   }

   let tokens = &tokens[1..];

   let mut start = 1;
   let mut end = 0;

   for (index, token) in tokens.iter().enumerate() {
      if *token == left {
         start += 1;
      } else if *token == right {
         end += 1;
      }

      if start == end {
         return Some(index + 1)
      }
   }

   None
}


fn find_next_new_line(tokens: &[Token]) -> usize {
   for (index, token) in tokens.iter().enumerate() {
      if let &Token::NewLine = token {
         return index;
      }
   }

   tokens.len()
}


fn consume_new_line(tokens: &[Token]) -> usize {
   for (index, token) in tokens.iter().enumerate() {
      match token {
         &Token::NewLine => {},
         _ => {
            return index;
         }
      }
   }

   tokens.len()
}
