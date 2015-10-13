use std::env;
use std::io::Read;
use std::io::{BufReader, BufRead};

#[allow(dead_code)]
static TEST: &'static str = "5
ALEX
LUIS
JAMES
BRIAN
PAMELA
1
PAMELA";

fn solve(names: &Vec<String>, name: &str) -> usize {
  let pos = names.iter().position(|x| x == name).unwrap();
  (pos+1) * name.chars().fold(0, |t,x| t + x as usize - b'A' as usize + 1)
}

fn setup(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let names_n = line.trim().parse().unwrap();

  let mut names: Vec<String> = vec![];
  for _ in 0..names_n {
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    names.push(line.trim().to_owned());
  }

  names.sort();

  line.clear();
  input.read_line(&mut line).ok().unwrap();
  let test_cases = line.trim().parse().unwrap();
  for _ in 0..test_cases {
    line.clear();
    input.read_line(&mut line).ok().unwrap();

    println!("{}", solve(&names, line.trim()));
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
