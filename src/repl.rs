use crate::printer::pr_str;
use crate::reader::read_str;
use crate::types::{OreResult, OreType};

// Read function
fn read(string: &String) -> OreResult {
  read_str(string)
}

// Evaluate function
fn eval(tokens: OreType) -> OreType {
  tokens
}

// Print function
fn print(tokens: &Vec<OreType>) {
  println!("ore< {}", pr_str(tokens, true));
}

// Read-eval-print function
pub fn rep(string: String) {
  match read(&string) {
    Ok(t) => {
      let result = eval(t);
      print(&vec!(result));
    },
    Err(e) => println!("ore< Error: {}", e),
  }
}
