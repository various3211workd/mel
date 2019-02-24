extern crate walkdir;

use std::fs::*;
use std::io::*;
use std::path::PathBuf;
use walkdir::WalkDir;

use super::uname;

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

  // create .mel folder
  create_dir_all(uname::get_folder_path()).unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });
  
  // create initTree.json file
  let mut f = BufWriter::new(
    OpenOptions::new()
      .write(true)
      .truncate(true)
      .open(uname::get_init_path())
      .expect("[Error] can't create initTree.json file"));

  f.write(jsontree.as_bytes()).unwrap();
}

