use std::fs::File;
use std::io::prelude::*;

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
  let mut inittree = "".to_string();
  let mut f = File::open(get_init_path()).expect("file not found");

  let mut contents = String::new();
  match f.read_to_string(&mut contents) {
    Ok(_) => { },
    Err(e) => { panic!("{}", e); }
  }
  let markdown_paths = contents.split("&").collect::<Vec<&str>>();
  let init_path = markdown_paths[0];
  
  for path in markdown_paths {
    let a_path: String = path.replace(init_path, "");
    
    // true delete flag and include delete_string
    if !(a_path.find(&delete_string) != None) {
      inittree.push_str(&a_path);
      inittree.push_str("&");
    }
  }
  
  put_init_file(inittree);
}
