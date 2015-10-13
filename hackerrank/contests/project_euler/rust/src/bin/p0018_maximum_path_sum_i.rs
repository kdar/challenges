use std::cmp::max;

// Algorithm: brute forcing this from top to bottom would mean transversing 16384 routes.
// Instead, we start from the bottom of the triangle and merge the max of a pair of
// bottom row values, to the previous row. Then repeat this process again.

use std::io::{BufReader, BufRead};
use std::env;

#[allow(dead_code)]
static TEST: &'static str = "1
4
3
7 4
2 4 6
8 5 9 3";

fn solve(triangle: &mut Vec<Vec<usize>>) -> usize {
  for row in (1..triangle.len()).rev() {
    for col in 0..triangle[row].len()-1 {
      triangle[row-1][col] += max(triangle[row][col], triangle[row][col+1]);
    }
  }
  triangle[0][0]
}

fn setup(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let test_cases = line.trim().parse().unwrap();

  for _ in 0..test_cases {
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let n = line.trim().parse().unwrap();

    let mut triangle = vec![];
    for _ in 0..n {
      line.clear();
      input.read_line(&mut line).ok().unwrap();
      let row: Vec<usize> = line.split(' ').map(|x| x.trim().parse().unwrap()).collect();
      triangle.push(row);
    }
    println!("{}", solve(&mut triangle));
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
