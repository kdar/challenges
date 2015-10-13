// Note: I think this can be optimized further.
// Maybe by generating palindromes and factoring them.

use std::io::{BufReader, BufRead};
use std::env;

#[allow(dead_code)]
static TEST: &'static str = "2
101110
800000";

fn is_palindrome(n: u64) -> bool {
  if n < 10 {
    return true;
  }

  let mut l = ((n as f64).log10()+1.0).floor() as u32;
  let mut n2 = 0u64;
  let mut nn = n;
  loop {
    if nn == 0 {
      break;
    }

    l -= 1;
    n2 += (nn % 10) * 10u64.pow(l);
    nn /= 10;
  }

  n == n2
}

fn solve(n: u64) -> u64 {
  let mut largest = 0u64;
  for x in 100..1000 {
    for y in x..1000 {
      if x*y > n {
        break;
      }

      if is_palindrome(x*y) && x*y > largest {
        largest = x*y;
      }
    }
  }
  largest
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
