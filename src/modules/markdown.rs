use pulldown_cmark::{html, Parser};

pub fn parsing_html(buf: String) {
  let parser = Parser::new(&buf);

  let mut html_buf = String::new();
  html::push_html(&mut html_buf, parser);

  println!("{}", html_buf);
}

pub fn parsing(buf: String) {

  println!("{}", buf);
}