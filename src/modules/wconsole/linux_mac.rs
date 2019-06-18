extern crate termcolor;

use wincolor::{
  Console, 
  Color, 
  Intense
};
use termcolor::{
  StandardStream, 
  Color, 
  ColorChoice, 
  ColorSpec, 
  WriteColor
};

/*
  complete_str function

  Linux or Max console put Cyan color

  @param String
*/
pub fn complete_str(text :String) {
  let mut stdout = StandardStream::stdout(ColorChoice::Always);
  stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
  writeln!(&mut stdout, text).unwrap();
}

/*
  err_str function

  Linux or Max console put Magenta color

  @param String
*/
pub fn err_str(text :String) {
  let mut stdout = StandardStream::stdout(ColorChoice::Always);
  stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))?;
  writeln!(&mut stdout, text).unwrap();
}