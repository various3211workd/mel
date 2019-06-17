#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use mel::modules::*;

// Write the Docopt usage string.
const USAGE: &'static str = "
mel is the coined term of 'memo' + '(TIL)Today I Learning'.
Can look back my daily notes.

Usage:
  mel init
  mel update
  mel list
  mel <filepath> [--html]
  mel -n <num> [--html]
  mel -g <url> [filepath]
  mel -d <string>
  mel -wn <num> <string>
  mel (--help | --version)

Options:
  -h, --help     Show this screen
  -v, --version  Show version
  -n, --number   Show list number
  -w, --write    Write til file
  -g, --get      Get Url file
  -d, --delete   list delete strings
  
  --html     Put html
";
const VERSION: &'static str = "v1.0.2";

#[derive(Deserialize)]
struct Args {
  cmd_init: bool,       // init option
  cmd_list: bool,       // list option
  cmd_update: bool,     // update option
  flag_number: bool,    // -n option
  flag_html: bool,      // --html option
  flag_write: bool,     // -w option
  flag_help: bool,      // -h option
  flag_version: bool,   // -v option
  flag_delete: bool,    // -d option
  flag_get: bool,       // -g option
  arg_num: usize,
  arg_string: String,
  arg_url: String,
  arg_filepath: String,
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
    options::init();
    wconsole::complete_str("[ o ] Init Complete".to_string());
  }
  // list
  else if args.cmd_list {
    options::show();
  }
  // -d option
  else if args.flag_delete {
    match options::delete(args.arg_string) {
      Ok(()) => {
        wconsole::complete_str("\n[ D ] delete Complete".to_string());
      },
      Err(_) => {
        wconsole::err_str("[ X ] Error Can't delete contents...".to_string());
      }
    }
  }
  // init file update
  else if args.cmd_update {
    options::update();
    wconsole::complete_str("[ U ] Update Complete".to_string());
  }
  // -wn option
  else if args.flag_write {
    edit_til::write_til_num(args.arg_num, args.arg_string);
    wconsole::complete_str("[ W ] Write Complete".to_string());
  }
  // -n option
  else if args.flag_number {
    edit_til::cat_til_num(args.arg_num, args.flag_html);
  }
  // -g option
  else if args.flag_get {
    match options::get_url(args.arg_url, args.arg_filepath) {
      Ok(()) => {
        wconsole::complete_str("[ G ] Get Contents".to_string());
      },
      Err(_) => {
        wconsole::err_str("[ X ] Error Can't get contents...".to_string());
      }
    }
  }
  else {
    edit_til::cat_til(args.arg_filepath, args.flag_html);
  }
}