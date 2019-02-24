use std::fs::File;
use std::io::prelude::*;

/*
  cat_til function

  @param arg_path: String

  return None
*/
pub fn cat_til(arg_path: String) {

  // user cat file
  let show_path: Vec<&str> = arg_path.rsplit("/").collect();

  // initTree.json file
  let mut f = File::open("initTree.json")
    .expect("file not found");
  
  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");
  
  let init_path: Vec<&str> = contents.split("&").collect();

  // init file path loop
  for path in init_path[0..init_path.len() - 1].into_iter() {
    if show_path[0] == "README.md" {
      let dest_path: Vec<&str> = path.rsplit("/").collect();
      
      if dest_path[1] == show_path[1] {
        let mut f = File::open(path).expect("file not found");
        let mut buf = String::new();
        f.read_to_string(&mut buf)
          .expect("something went wrong reading the file");

        println!("{}", buf);
      }
      else { }
    }
    else {
      println!("[DEBUG2] {}", path);
      let mut f = File::open(path).expect("file not found");
      let mut a = String::new();
      f.read_to_string(&mut a)
        .expect("something went wrong reading the file");

      println!("{}", a);
    }
  }
}