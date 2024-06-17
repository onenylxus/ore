use std::rc::Rc;

pub enum OreType {
  Nil,
  Bool(bool),
  Int(i64),
  Float(f64),
  Str(String),
  Sym(String),
  List(Rc<Vec<OreType>>),
}
pub type OreError = String;
pub type OreResult = Result<OreType, OreError>;
