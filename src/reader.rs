use crate::types::{OreError, OreResult, OreType};
use crate::types::OreType::{Bool, Float, Hash, Int, List, Nil, Str, Sym, Vect};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::rc::Rc;

// Reader stores tokens and a position
struct Reader {
  tokens: Vec<String>,
  pos: usize,
}

impl Reader {
  // Next function returns the token at the current position and increments the position
  fn next(&mut self) -> Result<String, OreError> {
    self.pos += 1;
    Ok(self.tokens.get(self.pos - 1).ok_or(format!("index {} not found", self.pos - 1))?.to_string())
  }

  // Peek function just returns the token at the current position
  fn peek(&self) -> Result<String, OreError> {
    Ok(self.tokens.get(self.pos).ok_or(format!("index {} not found", self.pos))?.to_string())
  }
}

// Read string function calls tokenize, creates Reader instance with the tokens and calls read_form
pub fn read_str(string: &String) -> OreResult {
  let tokens = tokenize(string);
  let mut reader = Reader { tokens, pos: 0 };

  read_form(&mut reader)
}

// Tokenize function takes the string and returns a list of all tokens(strings) in it
fn tokenize(string: &str) -> Vec<String> {
  let re = Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap();
  let mut tokens = vec![];

  for capture in re.captures_iter(string) {
    tokens.push(String::from(&capture[1]))
  }

  tokens
}

// Read form function
fn read_form(reader: &mut Reader) -> OreResult {
  // Peek at the first token in the reader
  let token = reader.peek()?;

  // If the token is a left parenthesis, call read_list
  // if the token is a right parenthesis, return an error of unexpected symbol
  // otherwise call read_atom
  match &token[..] {
    "(" => read_list(reader, ")"),
    ")" => Err("unexpected symbol ')'".to_string()),
    "[" => read_list(reader, "]"),
    "]" => Err("unexpected symbol ']'".to_string()),
    "{" => read_list(reader, "}"),
    "}" => Err("unexpected symbol '}'".to_string()),
    _ => read_atom(reader),
  }
}

// Read list function
fn read_list(reader: &mut Reader, end: &str) -> OreResult {
  let mut tokens = vec![];

  // Skip the left parenthesis
  reader.next()?;

  // Call read_form repeatedly until the matching right parenthesis is found
  // return an error of expected symbol if it reach end of file first
  loop {
    let token = match reader.peek() {
      Ok(t) => t,
      Err(_) => return Err(format!("expected symbol '{}'", end)),
    };

    if token == end {
      break
    }
    tokens.push(read_form(reader)?)
  }

  // Skip the right parenthesis
  reader.next()?;

  // Return the results into a List type
  match end {
    ")" => Ok(List(Rc::new(tokens.into()))),
    "]" => Ok(Vect(Rc::new(tokens.into()))),
    "}" => {
      let mut hm: HashMap<String, OreType> = HashMap::default();
      for (k, v) in tokens.iter().tuples() {
        match k {
          Str(s) => {
            hm.insert(s.to_string(), v.clone());
          },
          _ => return Err("hash key is not string".to_string()),
        }
      }
      Ok(Hash(Rc::new(hm)))
    }
    _ => Err(format!("unknown end '{}'", end)),
  }
}

// Read atom function convert contents of tokens into proper data type values
fn read_atom(reader: &mut Reader) -> OreResult {
  let f_re = Regex::new(r"^-?\d+\.\d+$").unwrap();
  let i_re = Regex::new(r"^-?\d+$").unwrap();
  let s_re = Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap();

  let token = reader.next()?;
  match &token[..] {
    "nil" => Ok(Nil),
    "false" => Ok(Bool(false)),
    "true" => Ok(Bool(true)),
    _ => {
      if f_re.is_match(&token) {
        Ok(Float(token.parse().unwrap()))
      } else if i_re.is_match(&token) {
        Ok(Int(token.parse().unwrap()))
      } else if s_re.is_match(&token) {
        Ok(Str(token[1..token.len() - 1].to_string()))
      } else if token.starts_with(':') {
        Ok(Str(format!("\u{29e}{}", &token[1..])))
      } else {
        Ok(Sym(token.to_string()))
      }
    },
  }
}
