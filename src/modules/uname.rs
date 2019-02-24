use std::env::*;

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

pub fn get_init_path() -> String {
  "C:/Users/".to_owned() + &get_username() + "/.mel/initTree.json"
}

pub fn get_folder_path() -> String {
  "C:/Users/".to_owned() + &get_username() + "/.mel"
}