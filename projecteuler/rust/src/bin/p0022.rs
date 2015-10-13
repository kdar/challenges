use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use std::io::{BufReader, BufRead};

fn main() {
  let mut filename = env::current_dir().unwrap();
  filename = filename.join("data").join("p0022.txt");

  let mut names: Vec<String> = Vec::new();
  let mut f = File::open(filename).unwrap();

  for bytes in BufReader::new(f).split(b',') {
    let mut s = String::from_utf8(bytes.unwrap()).unwrap();
    s = s.trim_matches('\"').to_string();
    names.push(s);
  }

  names.sort();

  let mut total = 0;
  for (i, name) in names.iter().enumerate() {
    let score = (i+1) * name.chars().fold(0, |t,x| t + x as usize - b'A' as usize + 1);
    if name == "COLIN" {
      println!("{}", score);
    }
    total += score;
  }

  println!("{}", "COLIN".chars().fold(0, |t,x| t + x as usize - b'A' as usize + 1));
  println!("871198282 {}", total);
}
