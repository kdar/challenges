use std::io::{BufReader, BufRead};
use std::env;

#[allow(dead_code)]
static TEST: &'static str = "1
100";

fn solve(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let p: u64 = line.trim().parse().unwrap();
  line.clear();
  input.read_line(&mut line).ok().unwrap();
  let q: u64 = line.trim().parse().unwrap();

  let mut found: Vec<u64> = vec![];
  for i in p..q+1 {
    let s = format!("{}", i.pow(2));

    let d1 = &s[0..s.len()/2];
    let d2 = &s[s.len()/2..s.len()];
    
    if i == d1.parse::<u64>().unwrap_or(0) + d2.parse::<u64>().unwrap_or(0) {
      found.push(i);
    }
  }

  if !found.is_empty() {
    println!("{}", found.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().connect(" "));
  } else {
    println!("INVALID RANGE");
  }
}

fn main() {
  // for testing purposes
  if let Some(arg) = env::args().nth(1) {
    if arg == "test" {
      let mut br = BufReader::new(TEST.as_bytes());
      solve(&mut br);
      return;
    }
  }

  let stdin = std::io::stdin();
  let mut br = BufReader::new(stdin);
  solve(&mut br);
}
