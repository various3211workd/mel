#[macro_use]
extern crate serde_derive;
extern crate wincolor;

use docopt::Docopt;
use wincolor::{
  Console, 
  Color, 
  Intense
};

use mel::modules::*;

// Write the Docopt usage string.
const USAGE: &'static str = "
Usage:
  mel init
  mel list
  mel <path>
  mel (--help | --version)

Options:
  -h, --help     Show this screen
  -v, --version  Show version
";

#[derive(Deserialize)]
struct Args {
  cmd_init: bool,
  cmd_list: bool,
  arg_path: String,
}

#[warn(unused_must_use)]
fn main() {
  let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());

  if args.cmd_init {
    let mut con = Console::stdout().unwrap();
    match init::init() {
      Ok(()) => {
        con.fg(Intense::Yes, Color::Cyan).unwrap();
        println!("[ o ] Init Complete");
        con.reset().unwrap();
      },
      Err(e) => {
        con.fg(Intense::Yes, Color::Magenta).unwrap();
        println!("{}", e.to_string());
        con.reset().unwrap();
      }
    }
  }
  else if args.cmd_list {
    showlist::show();
  }
  else {
    catfile::cat_til(args.arg_path);
  }
}