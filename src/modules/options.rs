extern crate walkdir;
extern crate pbr;

use std::fs::*;
use std::io::Result;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::PathBuf;
use walkdir::WalkDir;

use super::uname::*;
use super::markdown;
use super::edit_init_file::*;
use super::edit_til::write_markdown;

/*
  show function

  return None
*/
pub fn show() -> Result<()> {

  let mut f = File::open(get_inittree_path())?;

  let mut contents = String::new();
  match f.read_to_string(&mut contents) {
    Ok(_) => { },
    Err(e) => { panic!("{}", e); }
  }
  println!("[ <3 ] TIL List\n");
  
  let mut index = 0;
  let markdown_paths = contents.split("&").collect::<Vec<&str>>();
  let init_path = markdown_paths[0];
  
  for path in markdown_paths {
    let a_path: String = path.replace(init_path, "");
    
    let p = a_path.rsplit("/").collect::<Vec<&str>>();

    // true delete flag and include delete_string
    if p[0] == "README.md" {
      if p.len() <= 2 {
        println!("[ {} ] {}", index, a_path);
      }
      else {
        println!("[ {} ] {}" ,index, a_path.replace("/README.md", ""));
      }
    }
    else if p[0].ends_with(".md") {
      println!("[ {} ] {}", index, a_path);
    }
    index += 1;
  }

  Ok(())
}

/*
  delete function

  @param: delete_string String

  return None
*/
pub fn delete(delete_string: String) -> Result<()> {
  let mut f = File::open(get_inittree_path())?;

  let mut contents = String::new();
  f.read_to_string(&mut contents)?;

  let init_path: Vec<&str> = contents.split("&").collect();
  let mut inittree = "".to_string();  

  // current dir
  inittree.push_str(&init_path[0]);
  inittree.push_str("&");

  for tils in &init_path[1..] {
    if (tils.find(&delete_string) != None) {
      println!("delete {}", tils);
    }
    else {
      inittree.push_str(tils);
      inittree.push_str("&"); 
    }
  }
  
  put_init_file(inittree);
  
  Ok(())
}

/*
  init function

  return <Ok(),>
*/
pub fn init() -> Result<()> {
  put_init_file(create_init());
  
  Ok(())
}

/*
  update function

  return None
*/
pub fn update() -> Result<()> {
  let mut contents = String::new();
  let mut f = File::open(get_inittree_path())?;
  
  f.read_to_string(&mut contents)?;
  
  let init_path: Vec<&str> = contents.split("&").collect();
  let mut inittree = "".to_string();  

  // current dir
  inittree.push_str(&init_path[0]);
  inittree.push_str("&");
  
  // push full file path
  for entry in WalkDir::new(&init_path[0]).into_iter().filter_map(|e| e.ok()) {
    if entry.file_name().to_string_lossy().ends_with(".md") {
      let path = PathBuf::from(String::from(entry.path().display().to_string()));
      let cwd = canonicalize(&path)?;
      match cwd.into_os_string().into_string() {
        Ok(p_str) => {
          // create full path
          let path_vec: Vec<&str> = p_str.split("\\").collect();
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

  // create .mel folder
  create_dir_all(get_folder_path())?;
  
  // create initTree.init file
  let mut f = BufWriter::new(
    OpenOptions::new()
      .write(true)
      .truncate(true)
      .open(get_inittree_path())?);

  f.write(inittree.as_bytes()).unwrap();
  
  Ok(())
}

/*
  get_url function

  @param url String
*/
pub fn get_url(url: String, filepath: String) -> Result<()> {
  
  const INIT_PATH_LEN: usize = 3;
  let mut init_path: String;

  if filepath == "" {
    let mut resp = reqwest::get(&url).unwrap();

    let mut s = String::new();
    resp.read_to_string(&mut s)?;
    markdown::parsing(s);
  }
  else {
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

    let mut resp = reqwest::get(&url).unwrap();

    let mut s = String::new();
    resp.read_to_string(&mut s)?;
    
    write_markdown(init_path, s);
  }

  Ok(())
}