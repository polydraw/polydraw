
#[derive(PartialEq, Clone, Debug)]
pub enum Token {
   Name(String),
   Int(i64),
   Float(f64),
   NewLine,
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
}


pub type TokenResult = Option<(Token, usize)>;


pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
   let mut tokens = Vec::new();

   let original = source;

   let mut source = source;

   let mut consumed = 0;

   loop {
      let taken = consume_whitespace(source);
      consumed += taken;
      source = &source[taken..];

      if let Some((token, taken)) = single_token(source) {
         consumed += taken;
         source = &source[taken..];
         tokens.push(token);
      } else {
         if consumed != original.len() {
            return Err(error_string(original, consumed));
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


fn error_string(source: &str, consumed: usize) -> String {
   let mut line = 1;
   let mut column = 1;
   let mut error_ch = ' ';

   for (index, ch) in source.chars().enumerate() {
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

   format!(
      "Unrecognized character '{}' at line {}, col {}",
      error_ch, line, column
   )
}


fn consume_whitespace(source: &str) -> usize {
   for (index, ch) in source.chars().enumerate() {
      if ch != ' ' && ch != '\t' {
         return index;
      }
   }

   source.len()
}


fn single_token(source: &str) -> TokenResult {
   if let Some(result) = extract_name(source) {
      Some(result)
   } else if let Some(result) = extract_number(source) {
      Some(result)
   } else if let Some(result) = extract_char_token(source) {
      Some(result)
   } else {
      None
   }
}


fn extract_name(source: &str) -> TokenResult {
   let mut end = 0;

   let mut chars = source.chars();

   match chars.next() {
      Some(ch) => match ch {
         'a' ... 'z' | 'A' ... 'Z' => end += 1,
         _ => return None,
      },
      None => return None,
   }

   loop {
      match chars.next() {
         Some(ch) => match ch {
            'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' | '-' | '_' => end += 1,
            _ => break,
         },
         None => break,
      }
   }

   let token = Token::Name(
      String::from(&source[0..end])
   );

   Some((token, end))
}


fn extract_char_token(source: &str) -> TokenResult {
   let mut chars = source.chars();

   match chars.next() {
      Some(ch) => {
         let token = match ch {
            '\n' => Token::NewLine,
            '=' => Token::Assign,
            '+' => Token::Add,
            '-' => Token::Subtract,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '!' => Token::Not,
            '(' => Token::ParenLeft,
            ')' => Token::ParenRight,
            '[' => Token::BracketLeft,
            ']' => Token::BracketRight,
            '<' => Token::AngleBracketLeft,
            '>' => Token::AngleBracketRight,
            _ => return None,
         };

         Some((token, 1))
      },
      None => None,
   }
}


fn extract_number(source: &str) -> TokenResult {
   let full_len = source.len();

   let mut end = 0;

   let positive = match source.chars().next() {
      None => {
         return None;
      },
      Some('-') => {
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

   for ch in source.chars() {
      if let Some(digit) = to_digit(ch) {
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
   end = 0;

   match source.chars().next() {
      Some('.') => {
         end += 1;
      },
      _ => {
         if !positive {
            integral = -integral
         }

         return Some((
            Token::Int(integral),
            full_len - source.len()
         ));
      }
   }

   let source = &source[end..];

   end = 0;

   let mut fractional = 0;
   let mut divisor = 1;

   for ch in source.chars() {
      if let Some(digit) = to_digit(ch) {
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


