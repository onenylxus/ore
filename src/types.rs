use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub enum OreType {
  Nil,
  Bool(bool),
  Int(i64),
  Float(f64),
  Str(String),
  Sym(String),
  List(Rc<Vec<OreType>>),
  Vect(Rc<Vec<OreType>>),
  Hash(Rc<HashMap<String, OreType>>),
}
pub type OreError = String;
pub type OreResult = Result<OreType, OreError>;
