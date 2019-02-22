extern crate walkdir;

use std::fs::*;
use std::io::*;
//use std::str::Split;

use serde_json::{Result, Value, json};
use walkdir::WalkDir;

/*
  init function
*/
pub fn init() -> Result<()> {

  put_json_file(create_json());

  Ok(())
}

/*
  create_json function

  create til json tree.

  return Value
*/
fn create_json() -> String {
//fn create_json() -> Value {
  //let mut jsontree = json!({});
  let mut jsontree = "".to_string();

  for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
    if entry.file_name().to_string_lossy().ends_with(".md") {
      
      /*
      for x in entry.path().display().to_string().split('\\') {
        println!("{}", x);
      }
      */

      /*
      jsontree = json!({
        f_name
      });
      */

      jsontree.push_str(&entry.path().display().to_string());
      jsontree.push_str("&");
    }
  }

  let return_tree = jsontree.to_string();
  return_tree
}

/*
  put_json_file function

  create json file on jsontree.

  @param jsontree Value
*/
//fn put_json_file(jsontree: Value) {
fn put_json_file(jsontree: String) {

  let mut f = BufWriter::new(
    OpenOptions::new()
      .write(true)
      .create(true)
      .open("initTree.json")
      .expect("[Error] can't create initTree.json file"));

  //f.write(format!("{}", jsontree).as_bytes()).unwrap();
  f.write(jsontree.as_bytes()).unwrap();
}
