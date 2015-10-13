#![feature(iter_arith)]

fn solve() -> u64 {
  let n = 100;

  // https://en.wikipedia.org/wiki/1_%2B_2_%2B_3_%2B_4_%2B_%E2%8B%AF
  let sum: u64 = n * (n+1) / 2;
  let square_of_sum = sum*sum;

  // http://www.trans4mind.com/personal_development/mathematics/series/sumNaturalSquares.htm
  let sum_of_squares: u64 = n*(n+1)*(2*n+1) / 6;
  
  square_of_sum - sum_of_squares
}

fn main() {
  println!("{}", solve());
}
