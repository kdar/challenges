use std::io::{BufReader, BufRead};
use std::env;

#[allow(dead_code)]
static TEST: &'static str = "2
3
10";

fn gcd(mut a: u64, mut b: u64) -> u64 {
  if a < b {
    let t = b;
    b = a;
    a = t;
  }

  while b != 0 {
    let t = b;
    b = a % b;
    a = t;
  }

  a
}

fn lcm(a: u64, b: u64) -> u64 {
  a * b / gcd(a, b)
}

fn solve(n: u64) -> u64 {
  let mut a = 1u64;
  for i in 1..n+1 {
    a = lcm(a, i);
  }
  a
}

fn setup(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let test_cases = line.trim().parse().unwrap();

  for _ in 0..test_cases {
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let n: u64 = line.trim().parse().unwrap();

    println!("{}", solve(n));
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
