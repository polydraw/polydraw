use std::fmt;

use super::tokenizer::Token;


#[derive(PartialEq, Clone, Debug)]
pub struct Assignment {
   name: String,
   value: Ast,
}

impl Assignment {
   pub fn new(name: String, value: Ast) -> Box<Self> {
      Box::new(
         Assignment {
            name: name,
            value: value
         }
      )
   }
}

pub type AssignmentBox = Box<Assignment>;


#[derive(PartialEq, Clone, Debug)]
pub struct List {
   contents: Vec<Ast>,
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
pub struct Tuple {
   contents: Vec<Ast>,
}

impl Tuple {
   pub fn new(contents: Vec<Ast>) -> Box<Self> {
      Box::new(
         Tuple {
            contents: contents
         }
      )
   }
}

pub type TupleBox = Box<Tuple>;


#[derive(PartialEq, Clone, Debug)]
pub struct Function {
   name: String,
   arguments: Vec<Ast>,
}

impl Function {
   pub fn new(name: String, arguments: Vec<Ast>) -> Box<Self> {
      Box::new(
         Function {
            name: name,
            arguments: arguments,
         }
      )
   }
}

pub type FunctionBox = Box<Function>;


#[derive(PartialEq, Clone)]
pub enum BinaryType {
   Subtract,
   Add,
   Divide,
   Multiply,
}

impl fmt::Debug for BinaryType {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         &BinaryType::Subtract => write!(f, "-"),
         &BinaryType::Add => write!(f, "+"),
         &BinaryType::Divide => write!(f, "/"),
         &BinaryType::Multiply => write!(f, "*"),
      }
   }
}


#[derive(PartialEq, Clone, Debug)]
pub struct Binary {
   operator: BinaryType,
   left: Ast,
   right: Ast,
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
   Integer(i64),
   Float(f64),
   Assignment(AssignmentBox),
   List(ListBox),
   Tuple(TupleBox),
   Function(FunctionBox),
   Binary(BinaryBox),
}

impl fmt::Debug for Ast {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         &Ast::Name(ref value) => write!(f, "{}", value),
         &Ast::Integer(ref value) => write!(f, "{}", value),
         &Ast::Float(ref value) => write!(f, "{}", value),
         &Ast::Assignment(ref value) => write!(f, "{} = {:?}", value.name, value.value),
         &Ast::List(ref value) => write!(f, "{:?}", value.contents),
         &Ast::Tuple(ref value) => {
            let tuple_fmt = format!("{:?}", value.contents);
            write!(f, "({})", &tuple_fmt[1..tuple_fmt.len() - 1])
         },
         &Ast::Function(ref value) => write!(f, "{}!{:?}", value.name, value.arguments),
         &Ast::Binary(ref value) => write!(f, "({:?} {:?} {:?})", value.left, value.operator, value.right),
      }
   }
}

pub type AstResult = Option<(Ast, usize)>;


pub fn parse(tokens: Vec<Token>) -> Option<Vec<Ast>> {
   let mut assignments = Vec::new();

   let mut tokens = &tokens[..];

   loop {
      let taken = consume_new_line(tokens);
      tokens = &tokens[taken..];

      if let Some((ast, taken)) = parse_assignment(tokens) {
         tokens = &tokens[taken..];
         assignments.push(ast);
      } else {
         break;
      }
  }

   Some(assignments)
}


fn parse_assignment(tokens: &[Token]) -> AstResult {
   if tokens.len() < 4 {
      return None;
   }

   let name = match &tokens[0] {
      &Token::Name(ref name) => name,
      _ => return None,
   };

   if tokens[1] != Token::Assign {
      return None;
   }

   let next_new_line = find_next_new_line(tokens);

   let tokens = &tokens[2..next_new_line];

   if let Some(value) = match_value(tokens) {
      let assignment = Assignment::new(name.clone(), value);
      Some((Ast::Assignment(assignment), next_new_line + 1))
   } else {
      None
   }
}


fn match_value(tokens: &[Token]) -> Option<Ast> {
   if let Some(ast) = match_list(tokens) {
      Some(ast)
   } else if let Some(ast) = match_tuple_or_parenthesis(tokens) {
      Some(ast)
   } else if let Some(ast) = match_function(tokens) {
      Some(ast)
   } else if let Some(ast) = match_binary(tokens) {
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
      Some(Ast::Function(Function::new(name.clone(), contents)))
   } else {
      None
   }
}

fn match_single(tokens: &[Token]) -> Option<Ast> {
   if tokens.len() != 1 {
      return None;
   }

   match &tokens[0] {
      &Token::Name(ref value) => Some(Ast::Name(value.clone())),
      &Token::Integer(ref value) => Some(Ast::Integer(*value)),
      &Token::Float(ref value) => Some(Ast::Float(*value)),
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


fn match_tuple_or_parenthesis(tokens: &[Token]) -> Option<Ast> {
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
   } else if let Some(contents) = match_sequence_contents(tokens) {
      Some(Ast::Tuple(Tuple::new(contents)))
   } else {
      None
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

      if let Some(contents) = match_sequence_contents(tokens) {
         return Some((Ast::Tuple(Tuple::new(contents)), end + 1));
      }

      return None;
   }

   if let Some(ast) = match_single(&tokens[..1]) {
      Some((ast, 1))
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
