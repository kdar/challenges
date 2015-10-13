use std::io::{BufReader, BufRead};
use std::env;

#[allow(dead_code)]
static TEST: &'static str = "if man was meant to stay on the ground god would have given us roots";

fn solve(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  line = line.trim().chars().filter_map(|x| {
    if x.is_alphanumeric() {
      x.to_lowercase().next()
    } else {
      None
    }
  }).collect();

  let cols = (line.len() as f64).sqrt().ceil() as usize;
  let mut grid: Vec<String> = vec!["".to_owned(); cols];
  for (i, c) in line.chars().enumerate() {
    grid[i%cols] = grid[i%cols].clone() + &c.to_string();
  }

  println!("{}", grid.connect(" "));
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
