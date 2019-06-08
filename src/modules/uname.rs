use std::env::*;
use std::fs::*;
use std::io::Read;

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
    None => { panic!("can't get user name..."); }
  };

  user_name
}

/*
  get_init_path function

  return String
*/
pub fn get_inittree_path() -> String {
  "C:/Users/".to_owned() + &get_username() + "/.mel/initTree.json"
}

/*
  get_folder_path function

  return String
*/
pub fn get_folder_path() -> String {
  "C:/Users/".to_owned() + &get_username() + "/.mel"
}


/*
  get_init_path function

  return String
*/
pub fn get_init_path() -> String {
  let mut f = File::open(get_inittree_path()).expect("file not found");

  let mut contents = String::new();
  match f.read_to_string(&mut contents) {
    Ok(_) => { },
    Err(e) => { panic!("{}", e); }
  }
  
  contents.split("&").collect::<Vec<&str>>()[0].to_owned()
}