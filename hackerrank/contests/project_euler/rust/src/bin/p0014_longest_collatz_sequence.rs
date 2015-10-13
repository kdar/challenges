use std::io::{BufReader, BufRead};
use std::env;

#[allow(dead_code)]
static TEST: &'static str = "5
10
15
20
1000000
5000000
";

struct Solver {
  upper_bound: usize,
  already_caculated_index: usize,
  sequences_counter: Vec<usize>,
  largest_number_indexer: Vec<usize>,
}

impl Solver {
  fn new() -> Solver {
    let upper_bound = 5000000;
    let mut s = Solver{
      upper_bound: upper_bound,
      already_caculated_index: 1,
      sequences_counter: vec![0; upper_bound+1],
      largest_number_indexer: vec![0; upper_bound+1],
    };

    s.largest_number_indexer[1] = 1;
    s
  }

  // we cannot cache all the intermediate result produced by sequences(n),
  // however caching all the input should be suffice. When the intput n exceed
  // upperBound, just compute it, it may have hundreds of intermediate results,
  // take 1001063 for example. And the input n will exceeds Int.MaxValue,
  // use Long instead.
  fn sequences(&mut self, n: usize) -> usize {
    if n == 1 {
      self.sequences_counter[1] = 0;
      return 0;
    };

    if n > 1 && n <= self.upper_bound {
      if self.sequences_counter[n] == 0 {
        self.sequences_counter[n] = if n % 2 == 0 {
          1 + self.sequences(n / 2)
        } else {
          1 + self.sequences(3 * n + 1)
        };
      }
      return self.sequences_counter[n];
    } else if n % 2 == 0 {
      return 1 + self.sequences(n / 2);
    } else {
      return 1 + self.sequences(3 * n + 1);
    }
  }

  fn solve(&mut self, n: usize) -> usize {
    let mut max_index = self.largest_number_indexer[self.already_caculated_index];
    for j in self.already_caculated_index+1..n+1 {
      let num = self.sequences(j);
      if num >= self.sequences(max_index) {
        self.largest_number_indexer[j] = j;
        max_index = j;
      } else {
        self.largest_number_indexer[j] = max_index;
      }
    }
    if n > self.already_caculated_index {
      self.already_caculated_index = n;
    }
    self.largest_number_indexer[n]
  }
}

fn setup(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let test_cases = line.trim().parse().unwrap();

  let mut solver = Solver::new();
  for _ in 0..test_cases {
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let n = line.trim().parse().unwrap();

    println!("{}", solver.solve(n));
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
