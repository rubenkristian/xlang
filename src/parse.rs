use std::str::Chars;

#[warn(dead_code)]
struct Token {
  identity: [u8; 4],
  value: String,
}

#[warn(dead_code)]
pub fn parsing(source: &Vec<u8>) { // Move ownership of source code, and return List of Token
  let mut keywords: [&str; 10] = ["as", "if", "end", "start", "fun", "else", "loop", "ret", "", ""];
  for char in source {
    if *char == b'a' {

    }
  }
}