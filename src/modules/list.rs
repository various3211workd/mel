use std::fs::File;
use std::io::prelude::*;

use super::uname::get_init_path;
/*
  show function

  return None
*/
pub fn show() {

  let mut f = File::open(get_init_path()).expect("file not found");

  let mut contents = String::new();
  match f.read_to_string(&mut contents) {
    Ok(_) => { },
    Err(e) => { panic!("{}", e); }
  }
  println!("[ <3 ] Can Use Commands!!\n");
  
  let mut index = 0;
  for path in contents.split("&") {
    let p: Vec<&str> = path.rsplit("/").collect();
    
    if p.len() > 3 {
      println!("[ {} ] /{}/{}", index, p[1], p[0]);
    }

    index += 1;
  }
}