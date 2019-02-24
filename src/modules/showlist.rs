use std::fs::File;
use std::io::prelude::*;

/*
  show function

  return None
*/
pub fn show() {
  let filename = "initTree.json";

  let mut f = File::open(filename).expect("file not found");

  let mut contents = String::new();
  match f.read_to_string(&mut contents) {
    Ok(_) => { },
    Err(e) => { panic!("{}", e); }
  }
  println!("[ ! ] Can Use Commands!!\n");
  
  for path in contents.split("&") {
    let p: Vec<&str> = path.rsplit("/").collect();
    
    if p.len() > 3 {
      println!("[ * ] /{}/{}", p[1], p[0]);
    }
  }
}