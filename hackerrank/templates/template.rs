use std::io::{BufReader, BufRead};
use std::env;

#[allow(dead_code)]
static TEST: &'static str = "";

fn solve() -> {

}

fn setup(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let test_cases = line.trim().parse().unwrap();

  for _ in 0..test_cases {
    // line.clear();
    // input.read_line(&mut line).ok().unwrap();
    // let n = line.trim().parse().unwrap();
    // let whatever: Vec<usize> = line.split(' ').map(|x| x.trim().parse().unwrap()).collect();
  }
}

fn main() {
  // for testing purposes
  if let Some(arg) = env::args().nth(1) {
    if arg == "test" {
      let mut br = BufReader::new(TEST.as_bytes());
      setup(&mut br);
      return;
    }
  }

  let stdin = std::io::stdin();
  let mut br = BufReader::new(stdin);
  setup(&mut br);
}
