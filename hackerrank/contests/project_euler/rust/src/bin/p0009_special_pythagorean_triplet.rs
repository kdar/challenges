use std::io::{BufReader, BufRead};
use std::env;

#[allow(dead_code)]
static TEST: &'static str = "2
12
1000";

fn solve(n: i64) -> i64 {
  // First we have two equations:
  //   a^2 + b^2 = c^2
  //   a+b+c=N
  // N is a known constant, so we will solve for b and iterate
  // on a.
  // c = N-a-b
  // a^2 + b^2 = (N-a-b)^2
  // b = N(2a-N)/2(a-N) when a != N
  let mut max = 0;
  for a in 1..n/2 {
    let b = (n * (2*a - n)) / (2 * (a - n));
    let c = n - a - b;
    if a+b+c == n && a*a+b*b == c*c {
      if a*b*c > max {
        max = a*b*c;
      }
    }
  }

  if max != 0 {
    max
  } else {
    -1
  }
}

fn setup(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let test_cases = line.trim().parse().unwrap();

  for _ in 0..test_cases {
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let n: i64 = line.trim().parse().unwrap();

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
