use std::io::{BufReader, BufRead};
use std::env;

#[allow(dead_code)]
static TEST: &'static str = "1
100000
100000
100000
100000";

fn factorsum_not_including_self(n: usize, cache: &mut Vec<usize>) -> usize {
  if cache[n] != 0 {
    return cache[n];
  }

  //let mut factors: Vec<usize> = Vec::new();
  let mut sum = 1;
  //factors.push(1);
  //factors.push(n);
  let mut i = 2;
  while i*i <= n {
    if n % i == 0 {
      //factors.push(i);
      sum += i;
      if i*i != n {
        //factors.push(n/i);
        sum += n/i;
      }
    }
    i+=1;
  }
  //factors.sort();
  cache[n] = sum;
  sum
}

fn solve(n: usize, mut cache: &mut Vec<usize>) -> usize{
  let mut total = 0;
  for i in 2..n {
    let asum: usize = factorsum_not_including_self(i, &mut cache);
    let bsum: usize = factorsum_not_including_self(asum, &mut cache);

    if i == bsum && i != asum {
      total += i;
    }
  }

  total
}

fn setup(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let test_cases = line.trim().parse().unwrap();

  let mut cache: Vec<usize> = vec![0; 1000000];
  for _ in 0..test_cases {
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let n = line.trim().parse().unwrap();

    println!("{}", solve(n, &mut cache));
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
