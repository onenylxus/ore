use crate::reader::read_str;
use crate::types::{OreResult, OreType};

// Read function
fn read(string: &String) -> OreResult {
  read_str(string)
}

// Evaluate function
fn eval(_tokens: OreType) {}

// Print function
fn print(string: &String) {
  println!("ore< {}", String::from(string));
}

// Read-eval-print function
pub fn rep(string: String) {
  match read(&string) {
    Ok(t) => {
      eval(t);
      print(&string);
    },
    Err(e) => println!("ore< Error: {}", e),
  }
}
