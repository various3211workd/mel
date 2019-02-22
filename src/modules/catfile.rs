use std::fs::File;
use std::io::prelude::*;

/*
  Cat_Til function

  cat til file.

  @param arg_path: String

  return None
*/
pub fn Cat_Til(arg_path: String) {
  
  // user cat file
  let show_path: Vec<&str> = arg_path.rsplit("\\").collect();

  // initTree.json file
  let mut f = File::open("initTree.json").expect("file not found");
  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");
  for path in contents.split("&") {
    let p: Vec<&str> = path.rsplit("\\").collect();
  
    if show_path[0] == "README.md" {
      if show_path[0] == p[0] && show_path[1] == p[1] {
        let mut f = File::open(path).expect("file not found");
        let mut a = String::new();
        f.read_to_string(&mut a)
          .expect("something went wrong reading the file");

        println!("{}", a);
      }
    }
    else {
      let mut f = File::open(path).expect("file not found");
      let mut a = String::new();
      f.read_to_string(&mut a)
        .expect("something went wrong reading the file");

      println!("{}", a);
    }
  }
}