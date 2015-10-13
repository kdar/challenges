use std::io::{BufReader, BufRead};
use std::env;

#[allow(dead_code)]
static TEST: &'static str = "8
10
17
67
152
1894
90765
510987
104382426112";


fn power_of_ten_to_words(n: usize, modulo: usize, word: &str) -> Vec<String> {
  let ending = if n % modulo == 0 {
    vec![]
  } else {
    num_to_words(n % modulo)
  };

  let mut answer = num_to_words(n / modulo);
  answer.extend(vec![word.to_owned()]);
  answer.extend(ending);
  answer
}

fn num_to_words(n: usize) -> Vec<String> {
  let s = match n {
    1 => "One",
    2 => "Two",
    3 => "Three",
    4 => "Four",
    5 => "Five",
    6 => "Six",
    7 => "Seven",
    8 => "Eight",
    9 => "Nine",
    10 => "Ten",
    11 => "Eleven",
    12 => "Twelve",
    13 => "Thirteen",
    14 => "Fourteen",
    15 => "Fifteen",
    16 => "Sixteen",
    17 => "Seventeen",
    18 => "Eighteen",
    19 => "Nineteen",
    20 => "Twenty",
    30 => "Thirty",
    40 => "Forty",
    50 => "Fifty",
    60 => "Sixty",
    70 => "Seventy",
    80 => "Eighty",
    90 => "Ninety",
    _ => "",
  };

  if s != "" {
    return vec![s.to_owned()];
  }

  match n {
    n if n < 100 => {
      let u = n % 10;
      let t = n - u;

      let mut answer = num_to_words(t);
      answer.extend(num_to_words(u));
      answer
    },
    n if n < 1000 => {
      power_of_ten_to_words(n, 100, "Hundred")
    },
    n if n < 1000000 => {
      power_of_ten_to_words(n, 1000, "Thousand")
    },
    n if n < 1000000000 => {
      power_of_ten_to_words(n, 1000000, "Million")
    },
    n if n < 1000000000000 => {
      power_of_ten_to_words(n, 1000000000, "Billion")
    },
    n => panic!("{}", n),
  }
}

fn solve(n: usize) -> String {
  num_to_words(n).connect(" ")
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
