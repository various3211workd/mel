use std::fs::File;
use std::io::prelude::*;

/*
  cat_til function

  @param arg_path: String

  return None
*/
pub fn cat_til(arg_path: String) {

  // user cat file
  let show_path: Vec<&str> = arg_path.rsplit("\\").collect();

  // initTree.json file
  let mut f = File::open("initTree.json")
    .expect("file not found");
  
  let mut contents = String::new();
  
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");
  
  //let a = contents.split("&");
  //let b = arg_path;

  for path in contents.split("&") {
    let p: Vec<&str> = path.rsplit("\\").collect();
    if show_path[0] == "README.md" {
      if show_path[1] == p[1] {
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
    
    /*
    if x == arg_path {
      let mut f = File::open(&arg_path).expect("file not found");
      let mut buf = String::new();
      f.read_to_string(&mut buf)
        .expect("something went wrong reading the file");
      println!("{}", buf);
    }
    */
  }
  /*
  match a {
    _arg_path => {
      println!("{}", b);
      let mut f = File::open(b).expect("file not found");
      let mut c = String::new();
      f.read_to_string(&mut c)
        .expect("something went wrong reading the file");
      println!("{}", c);
    },
    _ => {
      panic!()
    }
  }
  */
  
  /*
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
  */
}