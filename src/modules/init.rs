use std::fs::*;
use std::io::*;

use serde_json::{Result, Value};

/*
  init function
*/
pub fn init() {
  putJsonFile(createJson());
}

/*
  createJson function

  create til json tree.

  return Value
*/
fn createJson() -> Value {
  let data = r#"
      {
          "name": "John Doe",
          "age": 43,
          "phones": [
              "+44 1234567",
              "+44 2345678"
          ]
      }"#;

  let jsontree: Value = serde_json::from_str(data).expect("json error");

  // debug
  println!("{:?}", jsontree);

  jsontree
}

/*
  putJsonFile function

  create json file on jsontree.

  @param jsontree Value
*/
fn putJsonFile(jsontree: Value) {

  //let filename = addr.split(":").collect::<Vec<&str>>();

  let mut f = BufWriter::new(
    OpenOptions::new()
      .write(true)
      .create(true)
      .open("initTree.json")
      .expect("[Error] can't create initTree.json file"));

  f.write(format!("{}", jsontree).as_bytes()).unwrap();
}
