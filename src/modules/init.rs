extern crate walkdir;
extern crate pbr;

use std::fs::*;
use std::io::*;
use std::env::*;
use std::path::PathBuf;
use walkdir::WalkDir;
//use pbr::ProgressBar;

use super::uname;

/*
  init function
*/
pub fn init() -> Result<()> {

  put_init_file(create_init());

  Ok(())
}

/*
  create_init function

  return Value
*/
fn create_init() -> String {
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
        Ok(aa) => {
          // create full path
          let path_vec: Vec<&str> = aa.split("\\").collect();
          let mut index = 0;
          for p in path_vec[3..path_vec.len()].into_iter() {
            inittree.push_str(&p);
            if index != path_vec.len() - 4 {
              inittree.push_str("/");
            }
            index += 1;
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
fn put_init_file(inittree: String) {

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

/*
  update function

  return None
*/
pub fn update() {

  let mut contents = String::new();
  
  let mut f = File::open(uname::get_init_path())
    .expect("file not found");
  
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");
  
  let init_path: Vec<&str> = contents.split("&").collect();
  let mut inittree = "".to_string();  

  // current dir
  inittree.push_str(&init_path[0]);
  inittree.push_str("&");
  
  // progressbar
  /*
  let mut pb = ProgressBar::new(1000);
  pb.format("|██-|");
  */

  // push full file path
  for entry in WalkDir::new(&init_path[0]).into_iter().filter_map(|e| e.ok()) {
    if entry.file_name().to_string_lossy().ends_with(".md") {
      let path = PathBuf::from(String::from(entry.path().display().to_string()));
      let cwd = canonicalize(&path).unwrap();
      match cwd.into_os_string().into_string() {
        Ok(aa) => {
          // create full path
          let path_vec: Vec<&str> = aa.split("\\").collect();
          let mut index = 0;
          for p in path_vec[3..path_vec.len()].into_iter() {
            inittree.push_str(&p);
            if index != path_vec.len() - 4 {
              inittree.push_str("/");
            }
            index += 1;
          }
          inittree.push_str("&");
        }
        Err(e) => {
          panic!("{:?}", e);
        }
      }
    }
    //pb.inc();
  }

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
  
  //pb.finish();
}