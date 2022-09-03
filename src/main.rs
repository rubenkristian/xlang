use std::{env, fs};

use crate::parse::parsing;

mod parse;

fn main() {
  let args: Vec<String> = env::args().collect();

  let args_len = args.len();
  println!("{args_len}");
  if args.len() >= 2 {
    let path = args[1].to_string();
    println!("{path}");
    let source = fs::read(path).expect("message");
    
    parsing(&source);
  }
}
