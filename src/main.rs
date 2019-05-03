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
  mel -n <num> [--html]
  mel -g <url>
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
  flag_html: bool,      // output html option
  flag_write: bool,     // -w option
  flag_help: bool,
  flag_version: bool,
  flag_delete: bool,    // -d option
  flag_get: bool,       // -g option
  arg_num: usize,
  arg_string: String,
  arg_url: String,
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
    match init::init() {
      Ok(()) => {
        wconsole::complete_str("[ o ] Init Complete".to_string());
      },
      Err(e) => {
        wconsole::err_str(e.to_string());
      }
    }
  }
  // list
  else if args.cmd_list {
    list::show();
  }
  // -d option
  else if args.flag_delete {
    list::delete(args.arg_string);
    wconsole::complete_str("\n[ D ] delete Complete".to_string());
  }
  // init file update
  else if args.cmd_update {
    init::update();
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
    get_url::get_url(args.arg_url);
  }
}