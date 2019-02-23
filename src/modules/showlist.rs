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
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");
  
  println!("[ ! ] Can Use Commands!!\n");
  
  for path in contents.split("&") {
    let p: Vec<&str> = path.rsplit("\\").collect();
    
    if p[0] == "README.md" {
      println!("[ * ] {}/{}", p[1], p[0]);
    }
    else {
      println!("[ * ] {}", p[0]);
    }
  }
}