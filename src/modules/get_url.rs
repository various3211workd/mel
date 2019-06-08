extern crate reqwest;

use std::io::Read;

use super::uname::get_init_path;
use super::edit_til::write_markdown;

/*
  get_url function

  @param url String
*/
pub fn get_url(url: String, filepath: String) {
  
  let mut init_path: String;

  // path first elements is '/'
  if filepath.chars().nth(0) == Some('/') {
    init_path = get_init_path() + &filepath;
  }
  else {
    init_path = get_init_path() + "/" + &filepath;
  }
  
  //let mut resp = reqwest::get("https://google.com").unwrap();

  //let mut s = String::new();
  //resp.read_to_string(&mut s);
  
  write_markdown(init_path, s);

}