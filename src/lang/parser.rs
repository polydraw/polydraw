use std::fmt;

use super::tokenizer::Token;


#[derive(Clone, Debug)]
pub enum FnType {
   Builtin,
   Defined,
}


#[derive(Clone, Debug)]
pub struct FnIndex {
   pub index: usize,
   pub span: usize,
}

impl FnIndex {
   pub fn new(index: usize, span: usize) -> Self {
      FnIndex {
         index: index,
         span: span,
      }
   }
}


#[derive(Clone, Debug)]
pub struct FnRef {
   pub fn_type: FnType,
   pub fn_index: FnIndex,
}

impl FnRef {
   #[inline]
   pub fn builtin(fn_index: FnIndex) -> Self {
      FnRef {
         fn_type: FnType::Builtin,
         fn_index: fn_index,
      }
   }

   #[inline]
   pub fn defined(fn_index: FnIndex) -> Self {
      FnRef {
         fn_type: FnType::Defined,
         fn_index: fn_index,
      }
   }
}


#[derive(Clone)]
pub enum Value {
   Int(i64),
   Float(f64),
   Bool(bool),
//   String(Box<String>),
   List(Box<Vec<Value>>),
   Name(Box<String>),
   FunctionRef(Box<String>),
   Call(Box<FunctionCall>),
}

impl fmt::Debug for Value {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         &Value::Int(ref value) => write!(f, "{}", value),
         &Value::Float(ref value) => write!(f, "{}", value),
         &Value::Bool(ref value) => write!(f, "{}", value),
//         &Value::String(ref value) => write!(f, "{:?}", value),
         &Value::Call(ref value) => write!(f, "{:?}", value),
         &Value::FunctionRef(ref value) => write!(f, "@{}", value),
         &Value::Name(ref value) => write!(f, "{}", value),
         &Value::List(ref value) => write!(f, "{:?}", value),
      }
   }
}


#[derive(Clone)]
pub struct FunctionCall {
   pub name: String,
   pub arguments: Vec<Value>,
}

impl FunctionCall {
   pub fn new(name: String, arguments: Vec<Value>) -> Self {
      FunctionCall {
         name: name,
         arguments: arguments
      }
   }

   fn match_point(&self) -> bool {
      if self.arguments.len() != 2 {
         return false;
      }

      self.name == "point"
   }

   fn match_binary_operator(&self) -> Option<&'static str> {
      if self.arguments.len() != 2 {
         return None;
      }

      Some(match &self.name as &str {
         "subtract" => "-",
         "add" => "+",
         "divide" => "/",
         "multiply" => "*",
         "equal" => "==",
         "unequal" => "!=",
         "range" => "..",
         "less" => "<",
         "less-equal" => "<=",
         "greater" => ">",
         "greater-equal" => ">=",
         _ => return None,
      })
   }
}

impl fmt::Debug for FunctionCall {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      if self.match_point() {
         write!(f, "<{:?} {:?}>", self.arguments[0], self.arguments[1])
      } else if let Some(operator) = self.match_binary_operator() {
         write!(f, "({:?} {} {:?})", self.arguments[0], operator, self.arguments[1])
      } else {
         write!(f, "({}!", self.name).unwrap();

         for argument in self.arguments.iter() {
            write!(f, " {:?}", argument).unwrap();
         }

         write!(f, ")")
      }
   }
}


#[derive(Clone)]
pub struct Assignment {
   pub names: Vec<String>,
   pub value: Value,
}

impl Assignment {
   pub fn new(names: Vec<String>, value: Value) -> Self {
      Assignment {
         names: names,
         value: value
      }
   }
}

impl fmt::Debug for Assignment {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      for name in self.names.iter() {
         write!(f, "{} ", name).unwrap();
      }

      write!(f, "= {:?}", self.value)
   }
}


#[derive(Clone)]
pub enum Argument {
   Name(String),
   List(Vec<Argument>),
}

impl fmt::Debug for Argument {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         &Argument::Name(ref name) => write!(f, "{}", name),
         &Argument::List(ref list) => {
            write!(f, "[").unwrap();

            for (index, argument) in list.iter().enumerate() {
               write!(f, "{:?}", argument).unwrap();

               if index != list.len() - 1 {
                  write!(f, " ").unwrap();
               }
            }

            write!(f, "]")
         },
      }
   }
}


#[derive(Clone)]
pub struct FunctionArguments {
   pub total_len: usize,
   pub arguments: Vec<Argument>,
}

impl fmt::Debug for FunctionArguments {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      for (index, argument) in self.arguments.iter().enumerate() {
         write!(f, "{:?}", argument).unwrap();

         if index != self.arguments.len() - 1 {
            write!(f, " ").unwrap();
         }
      }

      write!(f, "")
   }
}

impl FunctionArguments {
   #[inline]
   pub fn new(arguments: Vec<Argument>) -> Self {
      let total_len = argument_len(&arguments);

      FunctionArguments {
         total_len: total_len,
         arguments: arguments,
      }
   }
}

fn argument_len(arguments: &Vec<Argument>) -> usize {
   let mut len = 0;

   for argument in arguments.iter() {
      match argument {
         &Argument::Name(_) => len += 1,
         &Argument::List(ref list) => len += argument_len(list),
      }
   }

   len
}


#[derive(Clone)]
pub struct Function {
   pub name: String,
   pub arguments: FunctionArguments,
   pub assignments: Vec<Assignment>,
   pub flat_arguments: Vec<String>,
}

impl Function {
   pub fn new(
      name: String, arguments: FunctionArguments, assignments: Vec<Assignment>
   ) -> Self {
      let mut flat = Vec::new();

      flat_arguments(&mut flat, &arguments.arguments);

      Function {
         name: name,
         arguments: arguments,
         assignments: assignments,
         flat_arguments: flat,
      }
   }
}

impl fmt::Debug for Function {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      writeln!(f, "{} >> {:?}", self.name, self.arguments).unwrap();

      for assignment in self.assignments.iter() {
         writeln!(f, "   {:?}", assignment).unwrap();
      }

      write!(f, "")
   }
}


fn flat_arguments(
   flat: &mut Vec<String>,
   arguments: &Vec<Argument>,
) {
   for argument in arguments.iter() {
      match argument {
         &Argument::Name(ref name) => flat.push(name.clone()),
         &Argument::List(ref list) => {
            flat_arguments(flat, list);
         }
      }
   }
}


pub struct FunctionArgumentStack {
   stack: Vec<Vec<Argument>>,
}

impl FunctionArgumentStack {
   #[inline]
   fn new() -> Self {
      FunctionArgumentStack {
         stack: vec![vec![]],
      }
   }

   fn push(&mut self, name: String) {
      let last = self.stack.last_mut().unwrap();

      last.push(Argument::Name(name));
   }

   fn open_list(&mut self) {
      self.stack.push(Vec::new());
   }

   fn close_list(&mut self) -> bool {
      if self.stack.len() < 2 {
         return false;
      }

      let list = self.stack.pop().unwrap();

      let last = self.stack.last_mut().unwrap();

      last.push(Argument::List(list));

      true
   }

   fn arguments(&mut self) -> Option<FunctionArguments> {
      if self.stack.len() != 1 {
         return None;
      }

      let list = self.stack.pop().unwrap();

      Some(FunctionArguments::new(list))
   }
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


pub fn parse(tokens: Vec<Token>) -> Result<Vec<Function>, String> {
   let mut functions = Vec::new();
   let mut assignments = Vec::new();

   let mut tokens = &tokens[..];
   let mut new_lines = 0;

   let mut current_function: Option<(String, FunctionArguments)> = None;

   loop {
      let taken = consume_new_line(tokens);

      new_lines += taken;
      tokens = &tokens[taken..];

      if let Some((name, arguments, taken)) = parse_function_start(tokens) {
         tokens = &tokens[taken..];
         if let Some((current_name, current_arguments)) = current_function {
            let function = Function::new(current_name, current_arguments, assignments);
            functions.push(function);
         }

         current_function = Some((name, arguments));
         assignments = Vec::new();
      } else if let Some((value, taken)) = parse_assignment(tokens) {
         tokens = &tokens[taken..];
         assignments.push(value);
      } else {
         let taken = consume_new_line(tokens);

         tokens = &tokens[taken..];

         if tokens.len() == 0 {
            if let Some((name, arguments)) = current_function {
               let function = Function::new(name, arguments, assignments);
               functions.push(function);
            }
            return Ok(functions);
         } else {
            return Err(format!("Parse error at line {}", new_lines + 1));
         }
      }
   }
}


fn parse_function_start(tokens: &[Token]) -> Option<(String, FunctionArguments, usize)> {
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

   let mut argument_stack = FunctionArgumentStack::new();

   for i in 2..tokens.len() {
      match &tokens[i] {
         &Token::Name(ref name) => argument_stack.push(name.clone()),
         &Token::BracketLeft => argument_stack.open_list(),
         &Token::BracketRight => if argument_stack.close_list() == false {
            return None;
         },
         &Token::NewLine => if let Some(arguments) = argument_stack.arguments() {
            return Some((name.clone(), arguments, i));
         } else {
            return None;
         },
         _ => return None,
      }
   }

   None
}


fn parse_assignment(tokens: &[Token]) -> Option<(Assignment, usize)> {
   if tokens.len() < 5 {
      return None;
   }

   if tokens[0] != Token::SpaceOffset {
      return None;
   }

   let mut i = 1;

   let mut names = Vec::new();

   while let &Token::Name(ref name) = &tokens[i] {
      names.push(name.clone());
      i += 1;
   }

   if i == 1 {
      return None;
   }

   if tokens[i] != Token::Assign {
      return None;
   }

   let next_new_line = find_next_new_line(tokens);

   let tokens = &tokens[i+1..next_new_line];

   if let Some(value) = match_value(tokens) {
      let assignment = Assignment::new(names, value);
      Some((assignment, next_new_line))
   } else {
      None
   }
}


fn match_value(tokens: &[Token]) -> Option<Value> {
   if let Some(value) = match_list(tokens) {
      Some(value)
   } else if let Some(value) = match_parenthesis(tokens) {
      Some(value)
   } else if let Some(value) = match_point(tokens) {
      Some(value)
   } else if let Some(value) = match_function_call(tokens) {
      Some(value)
   } else if let Some(value) = match_binary(tokens) {
      Some(value)
   } else if let Some(value) = match_function_ref(tokens) {
      Some(value)
   } else if let Some(value) = match_single(tokens) {
      Some(value)
   } else {
      None
   }
}

fn match_binary(tokens: &[Token]) -> Option<Value> {
   if tokens.len() < 3 {
      return None;
   }

   let binary_types = [
      (Token::Subtract, "subtract"),
      (Token::Add, "add"),
      (Token::Divide, "divide"),
      (Token::Multiply, "multiply"),
      (Token::Equal, "equal"),
      (Token::Unequal, "unequal"),
      (Token::Range, "range"),
      (Token::AngleBracketLeft, "less"),
      (Token::LessEqual, "less-equal"),
      (Token::AngleBracketRight, "greater"),
      (Token::GreaterEqual, "greater-equal"),
   ];

   for binary_type in &binary_types {
      for middle in 1..tokens.len() - 1 {
         if binary_type.0 != tokens[middle] {
            continue;
         }

         if let Some(left) = match_value(&tokens[..middle]) {
            if let Some(right) = match_value(&tokens[middle + 1..]) {
               return Some(
                  binary_call(binary_type.1, left, right)
               );
            }
         }
      }
   }

   None
}

fn match_function_call(tokens: &[Token]) -> Option<Value> {
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
      Some(Value::Call(Box::new(FunctionCall::new(name.clone(), contents))))
   } else {
      None
   }
}

fn match_function_ref(tokens: &[Token]) -> Option<Value> {
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

   Some(Value::FunctionRef(Box::new(name.clone())))
}

fn match_single(tokens: &[Token]) -> Option<Value> {
   if tokens.len() != 1 {
      return None;
   }

   let value = match &tokens[0] {
      &Token::Name(ref value) => Value::Name(Box::new(value.clone())),
      &Token::Int(ref value) => Value::Int(*value),
      &Token::Float(ref value) => Value::Float(*value),
      &Token::True => Value::Bool(true),
      &Token::False => Value::Bool(false),
      _ => return None,
   };

   Some(value)
}

fn match_list(tokens: &[Token]) -> Option<Value> {
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
      Some(Value::List(Box::new(contents)))
   } else {
      None
   }
}

fn match_parenthesis(tokens: &[Token]) -> Option<Value> {
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

   if let Some(value) = match_value(tokens) {
      Some(value)
   } else {
      None
   }
}

fn match_point(tokens: &[Token]) -> Option<Value> {
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

fn match_sequence_contents(tokens: &[Token]) -> Option<Vec<Value>> {
   let mut contents = Vec::new();
   let mut tokens = tokens;

   loop {
      if tokens.len() == 0 {
         return Some(contents);
      }

      match try_list_item(tokens) {
         Some((value, end)) => {
            contents.push(value);
            tokens = &tokens[end..];
         },
         None => return None
      }
   }
}

fn try_list_item(tokens: &[Token]) -> Option<(Value, usize)> {
   if tokens.len() == 0 {
      return None;
   }

   if let Some(end) = try_sequence(tokens, Token::AngleBracketLeft, Token::AngleBracketRight) {
      let tokens = &tokens[1..end];

      if let Some(contents) = match_sequence_contents(tokens) {
         match point_def_from_contents(contents) {
            Some(value) => return Some((value, end + 1)),
            _ => return None,
         }
      }

      return None;
   }

   if let Some(end) = try_sequence(tokens, Token::BracketLeft, Token::BracketRight) {
      let tokens = &tokens[1..end];

      if let Some(contents) = match_sequence_contents(tokens) {
         return Some((Value::List(Box::new(contents)), end + 1));
      }

      return None;
   }

   if let Some(end) = try_sequence(tokens, Token::ParenLeft, Token::ParenRight) {
      let tokens = &tokens[1..end];

      if let Some(value) = match_value(tokens) {
         return Some((value, end + 1));
      }

      return None;
   }

   if let Some(value) = match_single(&tokens[..1]) {
      Some((value, 1))
   } else if let Some(value) = match_function_ref(&tokens[..2]) {
      Some((value, 2))
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

fn point_def_from_contents(mut contents: Vec<Value>) -> Option<Value> {
   if contents.len() != 2 {
      None
   } else {
      let y = contents.pop().unwrap();
      let x = contents.pop().unwrap();

      Some(binary_call("point", x, y))
   }
}

fn binary_call(name: &str, left: Value, right: Value) -> Value {
   Value::Call(Box::new(FunctionCall::new(name.to_string(), vec![left, right])))
}

