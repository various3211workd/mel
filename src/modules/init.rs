extern crate walkdir;

use std::fs::*;
use std::io::*;
use std::env::*;
use std::path::PathBuf;
//use std::str::Split;

//use serde_json::{Result, Value, json};
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

  return Value
*/
fn create_json() -> String {
  let mut jsontree = "".to_string();

  // push full file path
  for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
    if entry.file_name().to_string_lossy().ends_with(".md") {
      let path = PathBuf::from(String::from(entry.path().display().to_string()));
      let cwd = canonicalize(&path).unwrap();
      match cwd.into_os_string().into_string() {
        Ok(aa) => {
          // create full path
          let path_vec: Vec<&str> = aa.split("\\").collect();
          let mut index = 0;
          for p in path_vec[3..path_vec.len()].into_iter() {
            jsontree.push_str(&p);
            if index != path_vec.len() - 4 {
              jsontree.push_str("/");
            }
            index += 1;
          }
          jsontree.push_str("&");
        }
        Err(e) => {
          panic!("{:?}", e);
        }
      }
    }
  }

  let return_tree = jsontree.to_string();
  return_tree
}

/*
  put_json_file function

  @param jsontree Value
*/
fn put_json_file(jsontree: String) {
  let initfile_path = "C:/Users/".to_owned() + &get_username() + "/.mel/initTree.json";
  
  // create .mel folder
  create_dir_all("C:/Users/".to_owned() + &get_username() + "/.mel").unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });
  
  // create initTree.json file
  let mut f = BufWriter::new(
    OpenOptions::new()
      .write(true)
      .truncate(true)
      .open(initfile_path)
      .expect("[Error] can't create initTree.json file"));

  f.write(jsontree.as_bytes()).unwrap();
}

/*
  get_username function
  
  return String
*/
fn get_username() -> String {
  let user_name = match var_os("USERNAME") {
    Some(val) => {
      match val.into_string() {
        Ok(val) => { val },
        Err(e) => { panic!("{:?}", e); }
      }
    },
    None => { panic!("cat't get user name..."); }
  };

  user_name
}

/*
fn create_json() -> Value {
  let mut jsontree = json!({});
  
  for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
    if entry.file_name().to_string_lossy().ends_with(".md") {
      jsontree.push_str(&entry.path().display().to_string());
      jsontree.push_str("&");

      for x in entry.path().display().to_string().split('\\') {
        println!("{}", x);
      }
      jsontree = json!({
        f_name
      });
    }
  }

  let return_tree = jsontree.to_string();
  return_tree
}

fn Put_Json_File(jsontree: Value) {

  let mut f = BufWriter::new(
    OpenOptions::new()
      .write(true)
      .create(true)
      .open("initTree.json")
      .expect("[Error] can't create initTree.json file"));

  f.write(format!("{}", jsontree).as_bytes()).unwrap();
}
*/
