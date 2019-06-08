extern crate reqwest;

use std::fs::*;
use std::io::*;
use std::io::Read;

use super::uname::get_init_path;
use super::edit_til::write_markdown;

/*
  get_url function

  @param url String
*/
pub fn get_url(url: String, filepath: String) -> Result<()> {
  
  const INIT_PATH_LEN: usize = 3;
  let mut init_path: String;

  // path first elements is '/'
  if filepath.chars().nth(0) != Some('/') {
    init_path = "/".to_string() + &filepath;
  }
  else {
    init_path = filepath.to_string();
  }
  
  let init_path_vec: Vec<&str> = init_path.split("/").collect();
  if init_path_vec.len() >= INIT_PATH_LEN {
    create_dir_all(get_init_path() + &init_path.replace(init_path_vec[init_path_vec.len() - 1], ""))?;
  }
  
  init_path = get_init_path() + &init_path;

  let mut resp = reqwest::get("https://google.com").unwrap();

  let mut s = String::new();
  resp.read_to_string(&mut s)?;
  
  write_markdown(init_path, s);

  Ok(())
}