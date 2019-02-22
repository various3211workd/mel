extern crate walkdir;

use std::fs::*;
use std::io::*;
use std::path::PathBuf;
//use std::str::Split;

use serde_json::{Result, Value, json};
use walkdir::WalkDir;

/*
  Init function
*/
pub fn Init() -> Result<()> {

  Put_Json_File(Create_Json());

  Ok(())
}

/*
  Create_Json function

  create til json tree.

  return Value
*/
fn Create_Json() -> String {
//fn create_json() -> Value {
  //let mut jsontree = json!({});
  let mut jsontree = "".to_string();

  for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
    if entry.file_name().to_string_lossy().ends_with(".md") {
      let path = PathBuf::from(String::from(entry.path().display().to_string()));
      match canonicalize(&path) {
        Ok(aa) => {
          println!("{:?}", aa);
          //jsontree.push_str(aa);
          //jsontree.push_str("&");
        },
        Err(_) => {

        }
      }
      //jsontree.push_str(canonicalize(&path));
      //jsontree.push_str("&");


      /*
      for x in entry.path().display().to_string().split('\\') {
        println!("{}", x);
      }
      jsontree = json!({
        f_name
      });
      */
    }
  }

  let return_tree = jsontree.to_string();
  return_tree
}

/*
  Put_Json_File function

  create json file on jsontree.

  @param jsontree Value
*/
//fn Put_Json_File(jsontree: Value) {
fn Put_Json_File(jsontree: String) {

  let mut f = BufWriter::new(
    OpenOptions::new()
      .write(true)
      .create(true)
      .open("initTree.json")
      .expect("[Error] can't create initTree.json file"));

  //f.write(format!("{}", jsontree).as_bytes()).unwrap();
  f.write(jsontree.as_bytes()).unwrap();
}
