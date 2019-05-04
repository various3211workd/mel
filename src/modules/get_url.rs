extern crate reqwest;
use std::io;
use std::io::Read;

use super::uname::get_init_path;

/*
  get_url function

  @param url String
*/
pub fn get_url(url :String) {
  let init_path: String = get_init_path();

  let mut resp = reqwest::get("https://github.com/various3211workd/").unwrap();

  let mut s = String::new();
  resp.read_to_string(&mut s);
  println!("{:?}", s);
}