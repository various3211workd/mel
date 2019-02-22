#[macro_use]
extern crate serde_derive;
use docopt::Docopt;

use mel::modules::*;

// Write the Docopt usage string.
const USAGE: &'static str = "
Usage:
  mel
  mel init
  mel <path>...
  moni (--help | --version)

Options:
  -h, --help     Show this screen
  -v, --version  Show version
";

#[derive(Deserialize)]
struct Args {
  cmd_init: bool,
  arg_path: String,
}

#[warn(unused_must_use)]
fn main() {
  let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());

  println!("{}", args.cmd_init);

  if args.cmd_init {
    init::init();
  }
  else {

  }
}

