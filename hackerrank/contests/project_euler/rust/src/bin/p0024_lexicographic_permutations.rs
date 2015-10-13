use std::io::{BufReader, BufRead};
use std::env;

#[allow(dead_code)]
static TEST: &'static str = "4
1
2
3
6227020800";

fn factorial(mut n: u64) -> u64 {
  let mut result = 1;
  while n > 0 {
    result *= n;
    n-=1;
  }
  result
}

// Solve using integer and factorials. We get the factorial
// of the length of the number set, and permutate based on
// that in a loop. The answer is a vector of integers that
// we then convert to "abcdefghikklm".
fn solve(n: usize) -> String {
  let mut idx = n - 1;
  let mut set = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
  let mut result = vec![];
  while set.len() > 0 {
    // this can be precomputed to gain even more speed. e.g. have
    // a vector from 0-13 of factorials.
    let perm = factorial(set.len() as u64 - 1) as usize;
    let rest = idx % perm;
    let rm_idx = (idx / perm) % set.len();
    idx = rest;
    result.push(set.remove(rm_idx as usize));
  }

  result.iter().map(|x| (b'a' + *x) as char).collect::<String>()
}

fn setup(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let test_cases = line.trim().parse().unwrap();

  for _ in 0..test_cases {
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let n = line.trim().parse().unwrap();

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
