use std::io::{BufReader, BufRead};
use std::env;

#[allow(dead_code)]
static TEST: &'static str = "2
24
49";

fn sum_of_divisors(mut n: usize) -> usize {
  let mut prod = 1;
  let mut k = 2;
  while k*k<=n {
    let mut p=1;
    while n%k==0 {
      p=p*k+1;
      n/=k;
    }
    prod*=p;
    k += 1;
  }

  if n>1 {
    prod*=1+n;
  }
  prod
}

fn solve(n: usize, sum_of_abundants: &Vec<bool>) -> String {
  if n > 28123 || sum_of_abundants[n] {
    return "YES".to_owned();
  }

  "NO".to_owned()
}

fn precompute() -> Vec<bool> {
  let limit = 28123;
  let mut abundant: Vec<usize> = Vec::new();

  for i in 2..limit {
    if sum_of_divisors(i)>2*i {
      abundant.push(i);
    }
  }

  let mut sum_of_abundants: Vec<bool> = vec![false; limit*2+1];

  for i in 0..abundant.len() {
    for j in i..abundant.len() {
      sum_of_abundants[abundant[i]+abundant[j]]=true;
    }
  }

  sum_of_abundants
}

fn setup(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let test_cases = line.trim().parse().unwrap();

  let sum_of_abundants = precompute();
  for _ in 0..test_cases {
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let n = line.trim().parse().unwrap();

    println!("{}", solve(n, &sum_of_abundants));
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
