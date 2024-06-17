use crate::types::OreType::{self, Bool, Float, Int, List, Nil, Str, Sym};

// Print string function return the string representation of all tokens
pub fn pr_str(token: &OreType) -> String {
  match token {
    Nil => "nil".to_string(),
    Bool(false) => "false".to_string(),
    Bool(true) => "true".to_string(),
    Int(i) => format!("{}", i),
    Float(f) => format!("{}", f),
    Str(s) => format!("\"{}\"", s),
    Sym(s) => s.clone(),
    List(l) => {
      let strings: Vec<String> = l.iter().map(|x| pr_str(x)).collect();
      format!("({})", strings.join(" "))
    },
  }
}
