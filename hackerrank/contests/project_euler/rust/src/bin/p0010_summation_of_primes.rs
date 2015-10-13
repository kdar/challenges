use std::io::{BufReader, BufRead};
use std::env;

#[allow(dead_code)]
static TEST: &'static str = "2
5
10";

fn simple_sieve(limit: usize) -> Vec<usize> {
  let mut is_prime = vec![true; limit+1];
  is_prime[0] = false;
  if limit >= 1 { is_prime[1] = false }

  for num in 2..limit+1 {
    if is_prime[num] {
      let mut multiple = num*num;
      while multiple <= limit {
        is_prime[multiple] = false;
        multiple += num;
      }
    }
  }

  is_prime.iter().enumerate()
    .filter_map(|(pr, &is_pr)| if is_pr {Some(pr)} else {None})
    .collect()
}

fn solve(sieve: &Vec<usize>, nums: &Vec<usize>, max: usize, n: usize) -> usize {
  sieve.iter().filter(|x| *x <= &n).fold(0, |t,x| t+x)
}

fn setup(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let test_cases = line.trim().parse().unwrap();

  let mut nums: Vec<usize> = vec![];
  let mut max = 0;
  for _ in 0..test_cases {
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let n: usize = line.trim().parse().unwrap();

    if n > max {
      max = n;
    }

    nums.push(n);
  }

  // Precompute sieve for the max n. This is so we don't have
  // to compute the sieve for each n.
  let sieve = simple_sieve(max);

  for n in nums.iter() {
    println!("{}", solve(&sieve, &nums, max, *n));
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
