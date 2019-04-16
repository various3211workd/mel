extern crate wincolor;

use wincolor::{
  Console, 
  Color, 
  Intense
};

use pulldown_cmark::{html, Parser};

/*
  parsing_html function

  markdonw -> put html layout

  @param String
*/
pub fn parsing_html(buf: String) {
  let parser = Parser::new(&buf);

  let mut html_buf = String::new();
  html::push_html(&mut html_buf, parser);

  println!("{}", html_buf);
}

/*
  parsing_html function

  markdonw -> put console layout
  
  @param String
*/
pub fn parsing(buf: String) {
  let mut midashi: bool = false;        // midashi flag. start '#' on true
  let mut code_syntax: bool = false;    // code syntax flag. start '`' on true
  let mut comment_flag: bool = false;   // comment flag. start '>' on true
  let mut count: i32 = 0;               // counter
 
  // windows console setup
  let mut con = Console::stdout().unwrap();
    
  // 構文解析
  for word in buf.chars() {
    
    // 見出しで始まるとき
    if word == '#' && !code_syntax {
      midashi = true;
      count += 1;
    }
    // 見出し文字 '#' が終わったとき。出力する
    else if midashi {
      match count {
        1 => {con.fg(Intense::Yes, Color::Yellow).unwrap();}
        2 => {con.fg(Intense::Yes, Color::Green).unwrap();}
        3 => {con.fg(Intense::Yes, Color::Blue).unwrap();}
        _ => {con.fg(Intense::Yes, Color::Cyan).unwrap();}
      }
      print!("{}", word);

      // 改行があった場合
      if word == '\n' {
        midashi = false;
        count = 0;
        con.reset().unwrap();
      }
    }
    // comment '>'
    else if word == '>' && !code_syntax {
      con.fg(Intense::Yes, Color::Black).unwrap();
      comment_flag = true;
    }
    // comment文字 '>' が終わったとき。出力する
    else if comment_flag {
      print!("{}", word);

      // コメント時に改行があった場合
      if word == '\n' {
        con.reset().unwrap();
        comment_flag = false;
      }
    }
    // code highlight
    else if word == '`' {
      if count < 3 && !code_syntax && word != '\n' { 
        count += 1;
      }
      else {
        count -= 1;
      }
      if count == 3 {
        code_syntax = true;
        con.fg(Intense::Yes, Color::Magenta).unwrap();
      }
      else if count == 0 {
        println!("");
        code_syntax = false;
        con.reset().unwrap();
      }
    }
    else {
      print!("{}", word);
    }
  }
}