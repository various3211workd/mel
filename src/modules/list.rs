extern crate walkdir;
extern crate pbr;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use walkdir::WalkDir;
use std::fs::*;

use super::uname::get_init_path;
use super::init::put_init_file;

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
  println!("[ <3 ] TIL List\n");
  
  let mut index = 0;
  let markdown_paths = contents.split("&").collect::<Vec<&str>>();
  let init_path = markdown_paths[0];
  
  for path in markdown_paths {
    let a_path: String = path.replace(init_path, "");
    
    let p = a_path.rsplit("/").collect::<Vec<&str>>();

    // true delete flag and include delete_string
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
    index += 1;
  }
}

/*
  delete function

  @param: delete_string String

  return None
*/
pub fn delete(delete_string: String) {
  let mut f = File::open(get_init_path()).expect("file not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents).unwrap();

  let init_path: Vec<&str> = contents.split("&").collect();
  let mut inittree = "".to_string();  

  // current dir
  inittree.push_str(&init_path[0]);
  inittree.push_str("&");

  for tils in &init_path[1..] {
    if (tils.find(&delete_string) != None) {
      println!("delete {}", tils);
    }
    else {
      inittree.push_str(tils);
      inittree.push_str("&"); 
    }
  }
  
  put_init_file(inittree);
}
