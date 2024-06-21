use crate::types::OreType::{self, Bool, Float, Int, List, Nil, Str, Sym};

// Print string function return the string representation of all tokens
pub fn pr_str(token: &OreType, print_readably: bool) -> String {
  match token {
    Nil => "nil".to_string(),
    Bool(false) => "false".to_string(),
    Bool(true) => "true".to_string(),
    Int(i) => format!("{}", i),
    Float(f) => format!("{}", f),
    Str(s) => {
      let mut string = s.clone();

      // Translate special characters (doublequotes, newlines and backslashes) into their printed representations
      if print_readably {
        string = string.chars().map(
          |c| match c {
            '"' => "\\\"".to_string(),
            '\n' => "\\n".to_string(),
            '\\' => "\\\\".to_string(),
            _ => c.to_string(),
          }
        ).collect::<Vec<String>>().join("");
      }

      format!("\"{}\"", string)
    },
    Sym(s) => s.clone(),
    List(l) => {
      let strings = l.iter().map(|x| pr_str(x, print_readably)).collect::<Vec<String>>();
      format!("({})", strings.join(" "))
    },
  }
}
