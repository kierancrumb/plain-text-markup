use std::fs;

#[derive(Debug)]
struct Token {
  element: Element,
  body: Vec<char>
}

#[derive(PartialEq)]
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
  let compiled = asemble(chunks); 

  return compiled
}  

fn tokenize(content: String) -> Vec<Token> {// choppes it into chunks
  let mut lines: Vec<Token> = Vec::new();
  let mut new_line: bool = true;
  let mut i = 0;

  for c in content.chars() {
    if new_line {

      let elmnt = match_elmnt(c);

      let new_token = Token {
        element: elmnt,
        body: Vec::new()
      };

      lines.push(new_token);

      if lines[i].element == Element::P {lines[i].body.push(c)}

      i += 1;
      new_line = false;

    } else if c == '\n' {

      new_line = true;
      lines[i - 1].body.push(c);

    } else {

      lines[i - 1].body.push(c);
    }
    
  }

  return lines
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

fn asemble(tokens: Vec<Token>) -> String {
  dbg!(tokens);
  return String::from("hello, world")
}