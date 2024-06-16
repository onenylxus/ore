fn read(_string: &String) {}

fn eval() {}

fn print(string: &String) {
  println!("ore< {}", String::from(string));
}

pub fn rep(string: String) {
  read(&string);
  eval();
  print(&string);
}
