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
  mel -n <num>
  mel <path>
  mel (--help | --version)

Options:
  -h, --help     Show this screen
  -v, --version  Show version
  -n, --number   Show list number
";

#[derive(Deserialize)]
struct Args {
  cmd_init: bool,
  cmd_list: bool,
  flag_number: bool,
  arg_num: usize,
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
  else if args.flag_number {
    catfile::cat_til_num(args.arg_num);
  }
  else {
    catfile::cat_til(args.arg_path);
  }
}