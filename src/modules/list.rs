use std::fs::File;
use std::io::prelude::*;

use super::uname::get_init_path;
/*
  show function

  return None
*/
pub fn show(delete_flag: bool, delete_string: String) {

  let mut f = File::open(get_init_path()).expect("file not found");

  let mut contents = String::new();
  match f.read_to_string(&mut contents) {
    Ok(_) => { },
    Err(e) => { panic!("{}", e); }
  }
  println!("[ <3 ] TIL List\n");
  
  let mut index = 0;
  let markdown_paths = contents.split("&").collect::<Vec<&str>>();
  let init_path = markdown_paths[0];
  
  for path in markdown_paths {
    let a_path: String = path.replace(init_path, "");
    
    let p = a_path.rsplit("/").collect::<Vec<&str>>();

    // true delete flag and include delete_string
    if !(delete_flag && a_path.find(&delete_string) != None) {
      if p[0] == "README.md" {
        if p.len() <= 2 {
          println!("[ {} ] {}", index, a_path);
        }
        else {
          println!("[ {} ] {}" ,index, a_path.replace("/README.md", ""));
        }
      }
      else if p[0].ends_with(".md") {
        println!("[ {} ] {}", index, a_path);
      }
    }
    index += 1;
  }
}