use std::io::{BufReader, BufRead};
use std::env;

#[allow(dead_code)]
static TEST: &'static str = "5
45";

fn num_to_word(n: usize) -> String {
  match n {
    1 => "one".to_owned(),
    2 => "two".to_owned(),
    3 => "three".to_owned(),
    4 => "four".to_owned(),
    5 => "five".to_owned(),
    6 => "six".to_owned(),
    7 => "seven".to_owned(),
    8 => "eight".to_owned(),
    9 => "nine".to_owned(),
    10 => "ten".to_owned(),
    11 => "eleven".to_owned(),
    12 => "twelve".to_owned(),
    13 => "thirteen".to_owned(),
    14 => "fourteen".to_owned(),
    15 => "fifteen".to_owned(),
    16 => "sixteen".to_owned(),
    17 => "seventeen".to_owned(),
    18 => "eighteen".to_owned(),
    19 => "nineteen".to_owned(),
    20 => "twenty".to_owned(),
    30 => "thirty".to_owned(),
    40 => "forty".to_owned(),
    50 => "fifty".to_owned(),
    _ => {
      if n > 20 {
        vec![num_to_word((n / 10) * 10), num_to_word(n % 10)].connect(" ")
      } else {
        "".to_owned()
      }
    }
  }
}

fn solve(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let h = line.trim().parse().unwrap();
  line.clear();
  input.read_line(&mut line).ok().unwrap();
  let m = line.trim().parse().unwrap();

  match m {
    0 => {
      println!("{} o' clock", num_to_word(h));
    },
    1...29 => {
      if m == 15 {
        println!("quarter past {}", num_to_word(h));
      } else {
        println!("{} minute{} past {}", num_to_word(m), if m == 1 { "" } else { "s" }, num_to_word(h));
      }
    },
    30 => {
      println!("half past {}", num_to_word(h));
    },
    31...59 => {
      if m == 45 {
        println!("quarter to {}", num_to_word((h+1) % 12));
      } else {
        println!("{} minute{} to {}", num_to_word(60-m), if 60-m == 1 { "" } else { "s" }, num_to_word((h+1) % 12));
      }
    },
    _ => {},
  };
}

fn main() {
  // for testing purposes
  if let Some(arg) = env::args().nth(1) {
    if arg == "test" {
      let mut br = BufReader::new(TEST.as_bytes());
      solve(&mut br);
      return;
    }
  }

  let stdin = std::io::stdin();
  let mut br = BufReader::new(stdin);
  solve(&mut br);
}
