
#[derive(PartialEq, Clone, Debug)]
pub enum Token {
   Name(String),
   String(String),
   Int(i64),
   Float(f64),
   NewLine,
   Function,
   Address,
   SpaceOffset,
   Assign,
   Add,
   Subtract,
   Multiply,
   Divide,
   Not,
   ParenLeft,
   ParenRight,
   BracketLeft,
   BracketRight,
   AngleBracketLeft,
   AngleBracketRight,
   Equal,
   Unequal,
   LessEqual,
   GreaterEqual,
   True,
   False,
   Range,
}


pub type TokenResult = Option<(Token, usize)>;


pub fn tokenize(string: &str) -> Result<Vec<Token>, String> {
   let mut tokens = Vec::new();

   let chars_vec: Vec<char> = string.chars().collect();

   let mut source: &[char] = &chars_vec;

   let original = source;

   let mut consumed = 0;

   let mut after_new_line = true;

   let mut spaces_offset = 0;

   loop {
      let taken = consume_spaces(source);

      consumed += taken;
      source = &source[taken..];

      if after_new_line && taken > 0 {
         if spaces_offset == 0 {
            spaces_offset = taken;
         }

         if taken != spaces_offset {
            return Err(wrong_space_offset_error(original, consumed));
         }

         tokens.push(Token::SpaceOffset);
      }

      let taken = consume_comment(source);

      consumed += taken;
      source = &source[taken..];

      if let Some((token, taken)) = single_token(source) {
         consumed += taken;
         source = &source[taken..];

         after_new_line = Token::NewLine == token;

         tokens.push(token);
      } else {
         if consumed != original.len() {
            return Err(unrecognized_char_error(original, consumed));
         }

         break;
      }
   }

   let has_new_line = match tokens.last() {
      Some(token) => {
         match token {
            &Token::NewLine => true,
            _ => false,
         }
      },
      _ => false,
   };

   if !has_new_line {
      tokens.push(Token::NewLine);
   }

   Ok(tokens)
}


fn wrong_space_offset_error(source: &[char], consumed: usize) -> String {
   let (_, line, column) = error_position(source, consumed);

   format!(
      "Wrong space offset at line {}, col {}", line, column
   )
}


fn unrecognized_char_error(source: &[char], consumed: usize) -> String {
   let (error_ch, line, column) = error_position(source, consumed);

   format!(
      "Unrecognized character '{}' at line {}, col {}", error_ch, line, column
   )
}


fn error_position(source: &[char], consumed: usize) -> (char, usize, usize) {
   let mut line = 1;
   let mut column = 1;
   let mut error_ch = ' ';

   for (index, &ch) in source.iter().enumerate() {
      if consumed == index {
         error_ch = ch;
         break;
      }

      if ch == '\n' {
         line += 1;
         column = 1;
      } else {
         column += 1;
      }
   }

   return (error_ch, line, column)
}


fn consume_spaces(source: &[char]) -> usize {
   for (index, ch) in source.iter().enumerate() {
      if *ch != ' ' {
         return index;
      }
   }

   source.len()
}


fn consume_comment(source: &[char]) -> usize {
   let mut end = 0;

   let mut chars = source.iter();

   if let Some(ch) = chars.next() {
      if *ch == '#' {
         end += 1;

         loop {
            match chars.next() {
               Some(ch) => match *ch {
                  '\n' => break,
                  _ => end += 1,
               },
               None => break,
            }
         }
      }
   }

   end
}


fn single_token(source: &[char]) -> TokenResult {
   if let Some(result) = extract_string(source) {
      Some(result)
   } else if let Some(result) = extract_name(source) {
      Some(result)
   } else if let Some(result) = extract_number(source) {
      Some(result)
   } else if let Some(result) = extract_symbol_token(source) {
      Some(result)
   } else {
      None
   }
}


fn extract_string(source: &[char]) -> TokenResult {
   if source.len() < 2 {
      return None;
   }

   if source[0] != '"' {
      return None;
   }

   let source = &source[1..];
   let mut end = 1;

   let mut chars = source.iter();

   let mut result = String::new();

   loop {
      end += 1;

      match chars.next() {
         Some(ch) => match *ch {
            '\\' => {
               match chars.next() {
                  Some(ch) => match *ch {
                     '\\' | '"' => result.push(*ch),
                     _ => break,
                  },
                  None => break,
               }

               end += 1;
            },
            '"' => {
               let token = Token::String(result);

               return Some((token, end));
            },
            _ => result.push(*ch),
         },
         None => break,
      }
   }

   None
}


fn extract_name(source: &[char]) -> TokenResult {
   let mut end = 0;

   let mut chars = source.iter();

   match chars.next() {
      Some(ch) => match *ch {
         'a' ... 'z' | 'A' ... 'Z' | '$' => end += 1,
         _ => return None,
      },
      None => return None,
   }

   loop {
      match chars.next() {
         Some(ch) => match *ch {
            'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' | '-' | '_' => end += 1,
            _ => break,
         },
         None => break,
      }
   }

   let name = as_string(&source[0..end]);

   let token = match &name as &str {
      "true" => Token::True,
      "false" => Token::False,
      _ => Token::Name(name),
   };

   Some((token, end))
}


fn as_string(source: &[char]) -> String {
   let mut result = String::with_capacity(source.len());

   for ch in source.iter() {
      result.push(*ch);
   }

   result
}


fn extract_symbol_token(source: &[char]) -> TokenResult {
   let mut chars = source.iter();

   match chars.next() {
      Some(ch) => {
         let token = match *ch {
            '\n' => Token::NewLine,
            '=' => match chars.next() {
               Some(ch) => match *ch {
                  '=' => return Some((Token::Equal, 2)),
                  _ => Token::Assign,
               },
               None => Token::Assign,
            },
            '+' => Token::Add,
            '-' => Token::Subtract,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '!' => match chars.next() {
               Some(ch) => match *ch {
                  '=' => return Some((Token::Unequal, 2)),
                  _ => Token::Not,
               },
               None => Token::Not,
            },
            '(' => Token::ParenLeft,
            ')' => Token::ParenRight,
            '[' => Token::BracketLeft,
            ']' => Token::BracketRight,
            '<' => match chars.next() {
               Some(ch) => match *ch {
                  '=' => return Some((Token::LessEqual, 2)),
                  _ => Token::AngleBracketLeft,
               },
               None => Token::AngleBracketLeft,
            },
            '>' => match chars.next() {
               Some(ch) => match *ch {
                  '=' => return Some((Token::GreaterEqual, 2)),
                  '>' => return Some((Token::Function, 2)),
                  _ => Token::AngleBracketRight,
               },
               None => Token::AngleBracketRight,
            },
            '&' => Token::Address,
            '.' => match chars.next() {
               Some(ch) => match *ch {
                  '.' => return Some((Token::Range, 2)),
                  _ => return None,
               },
               None => return None,
            },
            _ => return None,
         };

         Some((token, 1))
      },
      None => None,
   }
}


fn extract_number(source: &[char]) -> TokenResult {
   let full_len = source.len();

   let mut end = 0;

   let positive = match source.iter().next() {
      None => {
         return None;
      },
      Some(&'-') => {
         end += 1;
         false
      },
      Some(_) => {
         true
      },
   };

   let mut integral = 0;

   let source = &source[end..];
   end = 0;

   for ch in source.iter() {
      if let Some(digit) = to_digit(*ch) {
         integral = 10 * integral + digit;
         end += 1;
      } else {
         break;
      }
   }

   if end == 0 {
      return None;
   }

   let source = &source[end..];

   let len = source.len();

   let mut chars = source.iter();
   let first = chars.next();
   let second = chars.next();

   let float = match (first, second) {
      (Some(&'.'), Some(&ch)) if ch != '.' => true,
      (Some(&'.'), None) => true,
      _ => false,
   };

   if !float {
      if !positive {
         integral = -integral
      }

      return Some((
         Token::Int(integral),
         full_len - len
      ));
   }

   // Dot here

   let source = &source[1..];

   end = 0;

   let mut fractional = 0;
   let mut divisor = 1;

   for ch in source.iter() {
      if let Some(digit) = to_digit(*ch) {
         fractional = 10 * fractional + digit;
         divisor = 10 * divisor;

         end += 1;
      } else {
         break;
      }
   }

   let mut value = integral as f64 + fractional as f64 / divisor as f64;

   if !positive {
      value = -value
   }

   let source = &source[end..];

   Some((
      Token::Float(value),
      full_len - source.len()
   ))
}


fn to_digit(ch: char) -> Option<i64> {
   match ch {
      '0' ... '9' => Some((ch as u32 - '0' as u32) as i64),
      _ =>  None
   }
}


