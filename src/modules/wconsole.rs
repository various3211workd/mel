extern crate wincolor;

use wincolor::{
  Console, 
  Color, 
  Intense
};

/*
  complete_str function

  windows console put Cyan color

  @param String
*/
pub fn complete_str(text :String) {
  let mut con = Console::stdout().unwrap();
  con.fg(Intense::Yes, Color::Cyan).unwrap();
  println!("{}", text);
  con.reset().unwrap();
}

/*
  err_str function

  windows console put Magenta color

  @param String
*/
pub fn err_str(text :String) {
  let mut con = Console::stdout().unwrap();
  con.fg(Intense::Yes, Color::Magenta).unwrap();
  println!("{}", text);
  con.reset().unwrap();
}