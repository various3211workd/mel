extern crate walkdir;

use std::io::{BufReader, Read, BufWriter, Write};
use std::io::Result;
use std::{fs, str};
use std::env::current_dir;

use super::uname::get_inittree_path;
use super::markdown;

/*
  cat_til function
  @param arg_path :String
  return None
*/
pub fn cat_til(arg_path: String, flag_html: bool) -> Result<()> {

  let mut path = current_dir()
    .unwrap()
    .display()
    .to_string()
    .replace("\\","/");
  
  println!("{}", arg_path);

  if !path.starts_with("/") {
    path.push_str("/");
  }
  path.push_str(&arg_path);
  
  let mut fs = BufReader::new(fs::File::open(path)?);
  let mut buf = vec![];
  
  fs.read_to_end(&mut buf)?;

  if flag_html {
    markdown::parsing_html(str::from_utf8(&buf).unwrap().to_string());
  }
  else {
    markdown::parsing(str::from_utf8(&buf).unwrap().to_string());
  }

  Ok(())
}

/*
  cat_til_num function

  @param num :usize

  return None
*/
pub fn cat_til_num(num: usize, flag_html: bool) -> Result<()> {
  let mut f = fs::File::open(get_inittree_path())?;
  
  let mut contents = String::new();
  f.read_to_string(&mut contents)?;
  
  let init_path: Vec<&str> = contents.split("&").collect();
  let show_file_str_len = init_path[num].to_string().len();
  
  print!("┏━━━━━━┳");
  put_markdown_line(show_file_str_len);
  println!("┓");
  
  println!("┃ Show ┃ [ {} ] ┃",
    init_path[num].to_string());

  print!("┗━━━━━━┻");
  put_markdown_line(show_file_str_len);
  println!("┛");

  // init file path loop
  show_markdown(init_path[num].to_string(), flag_html)?;

  Ok(())
}

/*
  put_markdown_line function

  @param path_len: usize

  return None
*/
fn put_markdown_line(path_len: usize) {
  let padding = 5;
  let mut counter = 0;
  loop {
    counter += 1;
    print!("━");
    if counter > path_len + padding {
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
pub fn write_til_num(num: usize, comment: String) -> Result<()> {
  let mut f = fs::File::open(get_inittree_path())?;
  
  let mut contents = String::new();
  f.read_to_string(&mut contents)?;
  
  let init_path: Vec<&str> = contents.split("&").collect();
  
  write_markdown(init_path[num].to_string(), comment);
  
  Ok(())
}

/*
  show_markdown function

  @param path :String

  return Result<(), String>
*/
fn show_markdown(path: String, flag_html: bool) -> Result<()> {
  let mut f = BufReader::new(fs::File::open(path).unwrap()); // buffering read
  let mut buf = vec![];
  f.read_to_end(&mut buf)?;

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
pub fn write_markdown(path: String, comment: String) {
  let mut f = BufWriter::new(
    fs::OpenOptions::new()
      .create(true)
      .write(true)
      .append(true)
      .open(path)
      .expect("[Error] can't write markdown"));

  f.write(format!("{}{}", "\n\n", comment).as_bytes()).unwrap();
}