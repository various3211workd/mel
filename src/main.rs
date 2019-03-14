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
  mel [-w] <path> <comment>
  mel (--help | --version)

Options:
  -h, --help     Show this screen
  -v, --version  SHow version
  -n, --number   Show list number
  -o, --html     Put html
  -w, --write    Write TIL file
";
const VERSION: &'static str = "v1.0.1";

#[derive(Deserialize)]
struct Args {
  cmd_init: bool,       // init option
  cmd_list: bool,       // list option
  cmd_update: bool,     // update option
  flag_number: bool,    // -n option
  arg_num: usize,
  flag_html: bool,      // output html option
  arg_path: String,
  flag_write: bool,     // -w option
  arg_comment: String,
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
  else if args.flag_write {
    writefile::write_til(args.arg_path, args.arg_comment);
  }
  else {
    catfile::cat_til(args.arg_path, args.flag_html);
  }
}