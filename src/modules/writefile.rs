use std::fs::File;
use std::io::prelude::*;

use super::uname::get_init_path;

pub fn write_til(path: String, comment: String) {
  let mut a_path;
  if !path.ends_with(".md") {
    a_path = path + "/README.md";
  }
  else {
    a_path = path;
  }
  let show_path: Vec<&str> = a_path.rsplit("/").collect();

  let mut f = File::open(get_init_path())
    .expect("file not found");
  
  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");
  
  let init_path: Vec<&str> = contents.split("&").collect();
  
  let markdown_path = init_path[0];
  
  // init file path loop
  for path in init_path[0..init_path.len() - 1].into_iter() {
    if show_path.len() > 2 {
      let dest_path: Vec<&str> = path.rsplit("/").collect();
      if dest_path[1] == show_path[1] && dest_path[0] == show_path[0] {
        //show_markdown(path.to_string(), flag_html).unwrap();
        break;
      }
    } else if show_path.len() == 2 {
      //show_markdown(markdown_path.to_owned() + &a_path, flag_html).unwrap();
      break;
    } else if show_path.len() == 1 {
      //show_markdown(markdown_path.to_owned() + "/" + &a_path, flag_html).unwrap();
      break;
    }
  }
}