use std::fs;

#[derive(Debug)]
struct Token {
  element: Element,
  body: Vec<char>
}

#[derive(Debug)]
enum Element {
  H,
  Bq,
  Ul,
  Hr,
  Block,
  Pre,
  Img,
  P
}

pub fn compile(file_path: &String) -> String {
  println!("
     |^^^^^^^^^^^^^^^^^^|
     |  COMPILING...   [?|”””;__
     |_..._..._______===|= __|__)
. .. “ (@)'(@)””””*|(@)(@)***(@)
  ");
  let file_content: String = fs::read_to_string(file_path).expect("idk man, something didn't work");
  let chunks: Vec<Token> = tokenize(file_content);

  let content = String::from("dfghj");
  return content;
}  

fn tokenize(content: String) -> Vec<Token> {// choppes it into chunks
  let lines: Vec<Token> = Vec::new()
  for (i, c) in content.chars().enumerate() {
    if i == 0 {
      lines.push(Token {
        element: match_elmnt(c),
        body: Vec::new()
      })
    } else {

    }
  }

  dbg!(lines);
  return vec![Token {element: Element::P, body: String::from("hello, world")}]
}

fn match_elmnt(tag: char) -> Element {
  match tag {
    '#' => Element::H,
    '>' => Element::Bq,
    '*' => Element::Ul,
    '-' => Element::Ul,
    '=' => Element::Hr,
    '%' => Element::Block,
    '`' => Element::Pre,
    '!' => Element::Img,
    _ => Element::P
  }
}

fn line_end(c: char) {
  match c {
    '\n' => {},
    _
  }
}