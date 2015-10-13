use std::io::{BufReader, BufRead};
use std::env;

#[allow(dead_code)]
static TEST: &'static str = "5
1
2
3
4
500";

fn esieve(limit: usize) -> Vec<usize> {
  let sieve_bound = limit - 1 / 2;
  let upper_sqrt = ((limit as f64).sqrt() as usize - 1) / 2;

  let mut primebits: Vec<bool> = vec![true; sieve_bound + 1];
  for i in 1..upper_sqrt+1 {
    if primebits[i] {
      let mut j = i * 2 * (i + 1);
      while j <= sieve_bound {
        primebits[j] = false;
        j += 2 * i + 1;
      }
    }
  }

  let mut numbers: Vec<usize> = vec![];
  numbers.push(2);
  for i in 1..sieve_bound+1 {
    if primebits[i] {
      numbers.push(2 * i + 1);
    }
  }

  numbers
}

fn solve(divisors: i64, sieve: &Vec<usize>) -> i64 {
  // the speed up trick below doesn't work for smaller divisors
  if divisors < 10 {
    let mut t = 1;
    let mut a = 1;
    let mut cnt = 0;
    while cnt <= divisors {
      cnt = 0;
      a = a+1;
      t = t+a;
      let ttx = (t as f64).sqrt() as i64;
      for i in 1..ttx+1 {
        if t % i == 0 {
          cnt = cnt + 2;
        }
      }

      if t==ttx*ttx {
        cnt -= 1; // correction for a perfect square 
      }
    }
    return t;
  }

  // optimized for big divisors

  let mut n = 1i64;
  let mut dn = 2i64;
  let mut cnt = 0i64;
  let (mut n1, mut dn1, mut exponent) = (0i64, 0i64, 0i64);

  while cnt <= divisors {
    n = n+1;
    n1 = n;
    if n1 % 2 == 0 {
      n1 = n1/2;
    }
    dn1 = 1;
    for i in 0..1000 {
      if sieve[i] as i64*sieve[i] as i64 > n1 {
        dn1=2*dn1;
        break;
      }

      exponent = 1;
      while n1 % sieve[i] as i64 == 0 {
        exponent += 1;
        n1 = n1/sieve[i] as i64;
      }

      if exponent > 1 {
        dn1=dn1*exponent;
      }

      if n1 == 1 {
        break;
      }
    }

    cnt = dn*dn1;
    dn = dn1;
  }

  n*(n-1)/2
}

fn setup(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let test_cases = line.trim().parse().unwrap();

  let sieve = esieve(1000);
  for _ in 0..test_cases {
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let n: i64 = line.trim().parse().unwrap();

    println!("{}", solve(n, &sieve));
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
