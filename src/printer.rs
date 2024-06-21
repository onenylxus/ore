use crate::types::OreType::{self, Bool, Float, Hash, Int, List, Nil, Str, Sym, Vect};

// Print string function return the string representation of all tokens
pub fn pr_str(tokens: &Vec<OreType>, print_readably: bool) -> String {
  let strings = tokens.iter().map(|x| print_token(x, print_readably)).collect::<Vec<String>>();
  format!("{}", strings.join(" "))
}

// Print token function
fn print_token(token: &OreType, print_readably: bool) -> String {
  match token {
    Nil => "nil".to_string(),
    Bool(false) => "false".to_string(),
    Bool(true) => "true".to_string(),
    Int(i) => format!("{}", i),
    Float(f) => format!("{}", f),
    Str(s) => {
      if s.starts_with("\u{29e}") {
        format!(":{}", &s[2..])
      } else if print_readably {
        // Translate special characters (doublequotes, newlines and backslashes) into their printed representations
        format!("\"{}\"", s.clone().chars().map(
          |c| match c {
            '"' => "\\\"".to_string(),
            '\n' => "\\n".to_string(),
            '\\' => "\\\\".to_string(),
            _ => c.to_string(),
          }
        ).collect::<Vec<String>>().join(""))
      } else {
        s.clone()
      }
    },
    Sym(s) => s.clone(),
    List(l) => {
      format!("({})",pr_str(&**l, print_readably))
    },
    Vect(l) => {
      format!("[{}]",pr_str(&**l, print_readably))
    },
    Hash(hm) => {
      let l = hm.iter().flat_map(|(k, v)| vec![Str(k.to_string()), v.clone()]).collect();
      format!("{{{}}}", pr_str(&l, print_readably))
    },
  }
}
