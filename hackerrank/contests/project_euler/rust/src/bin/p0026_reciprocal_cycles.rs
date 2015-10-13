use std::io::{BufReader, BufRead};
use std::env;

#[allow(dead_code)]
static TEST: &'static str = "1
14";

fn is_repeating(n: usize) -> bool {
  if n == 2 || n == 5 {
    return false;
  }

  let mut i = 2;
  while i*i <= n {
    if n % i == 0 {
      if i == 2 || i == 5 {
        return false;
      }
      if i*i != n {
        if n/i == 2 || n/i == 5 {
          return false;
        }
      }
    }
    i+=1;
  }
  true
}

fn solve(n: usize) -> usize {
  let mut answer = 0;
  let mut length = 0;

  if n <= 2 {
    return 0;
  }

  if n == 3 {
    return 2;
  }

  // a little optimization if n is greater than 100. We could
  // also just cache results instead of doing this.
  // If n is greater than 100, then start in reverse so we find
  // it quicker. This won't work for smaller n (e.g. for n=14)
  let mut range: Box<Iterator<Item=usize>> = Box::new(3..n);
  if n > 100 {
    range = Box::new((3..n).rev());
  }

  for i in range {
    if length >= i {
      break;
    }

    // if i is not divisible by 2 nor 5, then it is purely recurring.
    if !is_repeating(i) {
      continue;
    }

    let mut value = 1;
    let mut position = 0;
    let mut remainders: Vec<usize> = vec![0; i];

    // so basically we keep finding the remainder of the
    // value divided by the divisor, and we set in our list
    // of remainders the position we are at when we get that
    // answer. We then stop if the value goes to 0, or we find
    // a remainder in the 0 position (both means the number is
    // done dividing). This then gives us the length of the repeated
    // value.
    while remainders[value] == 0 && value != 0 {
      //println!("remainders[{}] = {}", value, position);
      remainders[value] = position;
      value *= 10;
      value %= i;
      position += 1;

      //println!("{} {}", remainders[value] == 0, value != 0);
    }
    //println!("");

    if position - remainders[value] > length {
      answer = i;
      length = position - remainders[value];
    }
  }

  answer
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
