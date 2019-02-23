#[macro_use]
extern crate serde_derive;
use docopt::Docopt;
use colored::*;

use mel::modules::*;

// Write the Docopt usage string.
const USAGE: &'static str = "
Usage:
  mel init
  mel -l
  mel <path>
  mel (--help | --version)

Options:
  -h, --help     Show this screen
  -l, --list     Show init til list
  -v, --version  Show version
";

#[derive(Deserialize)]
struct Args {
  cmd_init: bool,
  flag_list: bool,
  arg_path: String,
}

#[warn(unused_must_use)]
fn main() {
  let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());

  if args.cmd_init {
    match init::init() {
      Ok(()) => {
        println!("{}", "[ o ] Init Complete".green());
      },
      Err(e) => {
        panic!("{}", e.to_string().red());
      }
    }
  }
  else if args.flag_list {
    showlist::show();
  }
  else {
    catfile::cat_til(args.arg_path);
  }
}