use std::env;
use std::fs::*;
use std::io::Read;

/*
  get_init_path function

  return String
*/
pub fn get_inittree_path() -> String {
  let user_name = match env::home_dir() {
    Some(val) => { val.display().to_string() + "/.mel/initTree.json" },
    None => { panic!("can't get user name..."); }
  };

  user_name
}

/*
  get_folder_path function

  return String
*/
pub fn get_folder_path() -> String {
  let user_name = match env::home_dir() {
    Some(val) => { val.display().to_string() + "/.mel" },
    None => { panic!("can't get user name..."); }
  };

  user_name
}


/*
  get_init_path function

  return String
*/
pub fn get_init_path() -> String {
  let mut f = File::open(get_inittree_path()).unwrap();

  let mut contents = String::new();
  match f.read_to_string(&mut contents) {
    Ok(_) => { },
    Err(e) => { panic!("{}", e); }
  }
  
  contents.split("&").collect::<Vec<&str>>()[0].to_owned()
}