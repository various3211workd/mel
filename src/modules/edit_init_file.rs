extern crate walkdir;
extern crate pbr;

use std::fs::*;
use std::io::*;
use std::env::*;
use std::path::PathBuf;
use walkdir::WalkDir;

use super::uname;

/*
  create_init function

  return Value
*/
pub fn create_init() -> String {
  let mut inittree = "".to_string();
  
  // current dir
  inittree.push_str(&current_dir().unwrap().display().to_string().replace("\\","/"));
  inittree.push_str("&");

  // push full file path
  for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
    if entry.file_name().to_string_lossy().ends_with(".md") {
      let path = PathBuf::from(String::from(entry.path().display().to_string()));
      let cwd = canonicalize(&path).unwrap();
      match cwd.into_os_string().into_string() {
        Ok(ok_path) => {
          // create full path
          let ok_path_vec: Vec<&str> = ok_path.split("\\").collect();
          let mut index_count = 0;
          for p in ok_path_vec[3..ok_path_vec.len()].into_iter() {
            inittree.push_str(&p);
            if index_count != ok_path_vec.len() - 4 {
              inittree.push_str("/");
            }
            index_count += 1;
          }
          inittree.push_str("&");
        }
        Err(e) => {
          panic!("{:?}", e);
        }
      }
    }
  }

  let return_tree = inittree.to_string();
  return_tree
}

/*
  put_init_file function

  @param inittree Value
*/
pub fn put_init_file(inittree: String) {

  // create .mel folder
  create_dir_all(uname::get_folder_path()).unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });
  
  // create initTree.init file
  let mut f = BufWriter::new(
    OpenOptions::new()
      .write(true)
      .truncate(true)
      .open(uname::get_init_path())
      .expect("[Error] can't open file"));

  f.write(inittree.as_bytes()).unwrap();
}

