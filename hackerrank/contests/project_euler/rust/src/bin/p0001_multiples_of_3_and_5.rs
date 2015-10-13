use std::io::{BufReader, BufRead};
use std::env;

#[allow(dead_code)]
static TEST: &'static str = "2
10
1000000000";

fn solve(mut n: i64) -> i64 {
  n -= 1;

  let n1 = n / 3;
  let n2 = n / 5;
  let n3 = n / 15;

  let last1 = 3 + (n1 - 1) * 3;
  let last2 = 5 + (n2 - 1) * 5;
  let last3 = 15 + (n3 - 1) * 15;

  let sum1 = (n1 * (3 + last1)) / 2;
  let sum2 = (n2 * (5 + last2)) / 2;
  let sum3 = (n3 * (15 + last3)) / 2;

  sum1 + sum2 - sum3
}

fn setup(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let test_cases = line.trim().parse().unwrap();

  for _ in 0..test_cases {
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let mut n: i64 = line.trim().parse().unwrap();

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
