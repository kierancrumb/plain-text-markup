use std::fs;

#[derive(Debug)]
struct Token {
  element: Element,
  body: Vec<char>
}

#[derive(Debug, PartialEq)]
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
  println!(r#"
     |^^^^^^^^^^^^{}^^^^^^|
     |  COMPILING... {}  [?|""";__
     |_..._..._____{}__===|= __|__)
. .. " (@)'(@)""{}""*|(@)(@)***(@)
  "#,
    char_pad('^', file_path.len()),
    file_path, 
    char_pad('_',file_path.len()),
    char_pad('"',file_path.len())
  );

  let file_content: String = fs::read_to_string(file_path).expect("idk man, something didn't work");
  let chunks: Vec<Token> = tokenize(file_content);
  let compiled = asemble(chunks); 

  return compiled
}  

fn tokenize(content: String) -> Vec<Token> {// choppes it into chunks
  let mut lines: Vec<Token> = Vec::new();
  let mut new_line: bool = true;
  let mut in_pre: bool = false;
  let mut i = 0;

  for c in content.chars() {
    if new_line {

      let mut new_token = Token {
        element: match_elmnt(c),
        body: Vec::new()
      };

      if in_pre {
        if c == '`' {
          new_token.element = Element::Pre;
          in_pre = false;
        } else {
          new_token.element = Element::P;
        }
      } else if c == '`' {
        in_pre = true;
      }



      lines.push(new_token);

      if lines[i].element == Element::P { lines[i].body.push(c) }

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
  let mut output: Vec<String> = Vec::new();

  for tok in tokens.iter() {

    match tok.element {
      Element::P => output.push(String::from_iter(&tok.body)),
      Element::H => {
        output.push(String::from("header"));
      },
      Element::Bq => {
        output.push(vec![

          String::from("\t|"),
          String::from_iter(&tok.body)

        ].join(""));
      },
      Element::Ul => {
        output.push(vec![

          String::from("\t•"),
          String::from_iter(&tok.body)

        ].join(""));
      },
      Element::Hr => output.push(String::from("-----------------------------------")),
      Element::Pre => continue,
      _ => output.push(String::from("todo")),
    }
  }

  dbg!(output);

  return String::from("foo")
}

fn char_pad(chr: char, len: usize) -> String {
  let mut output: Vec<char> = Vec::new();

  while output.len() < len {
    output.push(chr);
  }

  return String::from_iter(output)
}