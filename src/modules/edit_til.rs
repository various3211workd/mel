use std::io::{BufReader, Read, BufWriter, Write};
use std::{fs, str};

use super::uname::get_init_path;
use super::markdown;

/*
  cat_til function

  @param arg_path :String

  return None
*/
pub fn cat_til(arg_path: String, flag_html: bool) {
  // user cat file
  let mut a_path;
  if !arg_path.ends_with(".md") {
    a_path = arg_path + "/README.md";
  }
  else {
    a_path = arg_path;
  }
  let show_path: Vec<&str> = a_path.rsplit("/").collect();

  let mut f = fs::File::open(get_init_path())
    .expect("file not found");
  
  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");
  
  let init_path: Vec<&str> = contents.split("&").collect();

  let markdown_path = init_path[0];
  
  // init file path loop
  for path in init_path[0..init_path.len() - 1].into_iter() {
    if show_path.len() > 2 {
      let dest_path: Vec<&str> = path.rsplit("/").collect();
      if dest_path[1] == show_path[1] && dest_path[0] == show_path[0] {
        show_markdown(path.to_string(), flag_html).unwrap();
        break;
      }
    } else if show_path.len() == 2 {
      show_markdown(markdown_path.to_owned() + &a_path, flag_html).unwrap();
      break;
    } else if show_path.len() == 1 {
      show_markdown(markdown_path.to_owned() + "/" + &a_path, flag_html).unwrap();
      break;
    }
  }
}

/*
  cat_til_num function

  @param num :usize

  return None
*/
pub fn cat_til_num(num: usize, flag_html: bool) {
  let mut f = fs::File::open(get_init_path())
    .expect("file not found");
  
  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");
  
  let init_path: Vec<&str> = contents.split("&").collect();
  
  // init file path loop
  show_markdown(init_path[num].to_string(), flag_html).unwrap();
}

/*
  write_til function

  @param path :String
  @param comment: String

  return None
*/
pub fn write_til(arg_path: String, comment: String) {
  // user cat file

  let mut a_path;

  if !arg_path.ends_with(".md") {
    a_path = arg_path + "/README.md";
  }
  else {
    a_path = arg_path;
  }
  let show_path: Vec<&str> = a_path.rsplit("/").collect();

  let mut f = fs::File::open(get_init_path())
    .expect("file not found");
  
  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");
  
  let init_path: Vec<&str> = contents.split("&").collect();
  
  // init file path loop
  for path in init_path[0..init_path.len() - 1].into_iter() {
    if show_path.len() > 2 {
      let dest_path: Vec<&str> = path.rsplit("/").collect();
      if dest_path[1] == show_path[1] && dest_path[0] == show_path[0] {
        write_markdown(path.to_string(), comment);
        break;
      }
    } else if show_path.len() == 2 {
      write_markdown(format!("{}{}", path.to_owned(), &a_path), comment);
      break;
    } else if show_path.len() == 1 {
      write_markdown(format!("{}{}{}", path.to_owned(), "/", &a_path), comment);
      break;
    }
  }
}

/*
  write_til_num function

  @param num: usize
  @param path :String

  return None
*/
pub fn write_til_num(num: usize, comment: String) {
  let mut f = fs::File::open(get_init_path())
    .expect("file not found");
  
  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");
  
  let init_path: Vec<&str> = contents.split("&").collect();
  
  write_markdown(init_path[num].to_string(), comment);
}

/*
  show_markdown function

  @param path :String

  return Result<(), String>
*/
fn show_markdown(path: String, flag_html: bool) -> Result<(), String> {
  let mut f = BufReader::new(fs::File::open(path).unwrap()); // buffering read
  let mut buf = vec![];
  f.read_to_end(&mut buf).unwrap();

  if flag_html {
    markdown::parsing_html(str::from_utf8(&buf).unwrap().to_string());
  }
  else {
    markdown::parsing(str::from_utf8(&buf).unwrap().to_string());
  }

  Ok(())
}

/*
  write_markdown function

  @param path :String
  @param comment: String

  return None
*/
fn write_markdown(path: String, comment: String) {
  let mut f = BufWriter::new(
    fs::OpenOptions::new()
      .write(true)
      .append(true)
      .open(path)
      .expect("[Error] can't open file"));

  f.write(format!("{}{}", "\n\n", comment).as_bytes()).unwrap();
}