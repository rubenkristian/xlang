use std::{env, fs, char};

fn main() {
  let args: Vec<String> = env::args().collect();

  let args_len = args.len();
  println!("{args_len}");
  if args.len() >= 2 {
    let path = args[1].to_string();
    println!("{path}");
    let content = fs::read_to_string(path).expect("message");
    let chars = content.chars();
    
    let mut i = 0;
    for character in chars {
      print!("{} char {}\n", i, character);
      i += 1;
    }
  }
}