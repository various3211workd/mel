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
mel is the coined term of 'memo' + '(TIL)Today I Learning'.
Can look back my daily notes.

Usage:
  mel init
  mel list
  mel update
  mel [-o] -n <num>
  mel [-o] <path>
  mel (--help | --version)

Options:
  -h, --help     Show this screen
  -v, --version  SHow version
  -n, --number   Show list number
  -o, --html     Put html 
";
const VERSION: &'static str = "v1.0.1";

#[derive(Deserialize)]
struct Args {
  cmd_init: bool,
  cmd_list: bool,
  cmd_update: bool,
  flag_number: bool,
  arg_num: usize,
  flag_html: bool,
  arg_path: String,
  flag_help: bool,
  flag_version: bool,
}

#[warn(unused_must_use)]
fn main() {
  let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());

  if args.flag_help {
    println!("{}", USAGE);
  }
  else if args.flag_version {
    println!("{}", VERSION);
  }
  else if args.cmd_init {
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
    list::show();
  }
  // init file update
  else if args.cmd_update {
    init::update(); 
    let mut con = Console::stdout().unwrap();
    con.fg(Intense::Yes, Color::Cyan).unwrap();
    println!("[ U ] Update Complete");
    con.reset().unwrap();
  }
  else if args.flag_number {
    catfile::cat_til_num(args.arg_num, args.flag_html);
  }
  else {
    catfile::cat_til(args.arg_path, args.flag_html);
  }
}