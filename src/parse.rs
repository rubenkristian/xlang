use std::str::Chars;

enum Identity {
  Int,
  Float,
  String,
  Operator,
  Func,
  Data
}

#[warn(dead_code)]
struct Token {
  identity: Identity,
  value: String,
}

#[warn(dead_code)]
pub fn parsing(source: &Vec<u8>) { // Move ownership of source code, and return List of Token
  let mut keywords_2words: [&str; 10] = ["as", "id", "", "", "", "", "", "", "", ""];
  let mut keywords_3words: [&str; 10] = ["ret", "end", "fun", "", "", "", "", "", "", ""];
  let mut keywords_4words: [&str; 10] = ["else", "loop", "", "", "", "", "", "", "", ""];
  let mut keywords_5words: [&str; 10] = ["start", "id", "", "", "", "", "", "", "", ""];
  for char in source {
    if *char == b'a' {

    }
  }
}